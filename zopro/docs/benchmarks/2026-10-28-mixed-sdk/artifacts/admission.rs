//! Optional mixed-flush storage. The query owns one shared peak reservation.
use std::sync::Arc;
use crate::preparation::{allocation_bytes,Reservation};
pub(crate) struct Admission {_reservation:Reservation}
pub(crate) struct Admitted {_owner:Arc<Admission>,scratch:MixedScratch}
const PENDING:usize=200;
const MATCHES:usize=32;
const QUEUE:usize=PENDING-1;
fn forecast(windows:usize)->Option<usize> {
    let arrays=[512usize.checked_mul(std::mem::size_of::<[u32;3]>())?,QUEUE,
        PENDING.checked_mul(std::mem::size_of::<usize>())?,MATCHES.checked_mul(std::mem::size_of::<[usize;2]>())?,
        PENDING.div_ceil(LANES).checked_mul(std::mem::size_of::<FqAddGroup>())?,MATCHES.div_ceil(LANES).checked_mul(std::mem::size_of::<TripleRecord>())?,
        std::mem::size_of::<Admitted>()];
    let per=arrays.into_iter().try_fold(0usize,|a,b|a.checked_add(b)?.checked_add(64))?;
    per.checked_mul(windows)?.checked_add(std::mem::size_of::<Admission>())?.checked_add(64)
}
fn reserve(size:usize,windows:usize,acquire:impl FnOnce(Option<usize>)->Option<Reservation>)->Option<Arc<Admission>> {
    if size!=PENDING || windows==0 {return None;}
    Some(Arc::new(Admission{_reservation:acquire(forecast(windows))?}))
}
pub(crate) fn admit(size:usize,windows:usize)->Option<Arc<Admission>> {
    if std::env::var("ZOPRO_CPU_MIXED_G1").as_deref()!=Ok("1"){return None;}
    reserve(size,windows,Reservation::analysis)
}
fn buffer<T>(n:usize)->Option<Vec<T>> {let mut v=Vec::new();v.try_reserve_exact(n).ok()?;Some(v)}
impl Admission {
    pub(crate) fn window(self:&Arc<Self>)->Option<Vec<Admitted>> {
        let s=MixedScratch{slots:buffer(512)?,selected:buffer(QUEUE)?,ordinary:buffer(PENDING)?,triples:buffer(MATCHES)?,
            ordinary_records:buffer(PENDING.div_ceil(LANES))?,triple_records:buffer(MATCHES.div_ceil(LANES))?};
        let mut owner:Vec<Admitted>=buffer(1)?;
        let bytes=[allocation_bytes(&s.slots)?,allocation_bytes(&s.selected)?,allocation_bytes(&s.ordinary)?,allocation_bytes(&s.triples)?,
            allocation_bytes(&s.ordinary_records)?,allocation_bytes(&s.triple_records)?,allocation_bytes(&owner)?].into_iter().try_fold(0usize,|a,b|a.checked_add(b))?;
        let per=forecast(1)?.checked_sub(std::mem::size_of::<Admission>())?.checked_sub(64)?;
        if bytes>per {return None;}
        owner.push(Admitted{_owner:self.clone(),scratch:s});Some(owner)
    }
}
impl Admitted {
    /// True consumes the pending Q layer and selected R entries exactly once.
    pub(crate) fn apply<C:CurveConfig<Base=Fq>,P:Copy+Borrow<Affine<C>>>(&mut self,b:&mut[Affine<C>],q:&[Entry<P>],queue:&mut Vec<Entry<P>>)->bool {
        if q.len()!=PENDING || queue.is_empty() || queue.len()>QUEUE {return false;}
        self.scratch.classify(q,queue);
        let m=self.scratch.triples.len();
        if m==0 || m>MATCHES {return false;}
        mixed::<C,P>(b,q,queue,&mut self.scratch);
        let left=self.scratch.compact(queue);queue.truncate(left);true
    }
}

#[cfg(test)]mod tests {
    use super::*;use zopro_bn254::{Fr,G1Config,G1Projective,G1Affine};
    fn fixture(m:usize)->(Vec<G1Affine>,Vec<(u32,G1Affine)>,Vec<(u32,G1Affine)>,Vec<G1Affine>,Vec<(u32,G1Affine)>) {
        let g=G1Projective::generator();let mut b:Vec<_>=(0..512).map(|i|(g*Fr::from(1000000+i as u64)).to_affine()).collect();
        let q:Vec<_>=(0..200).map(|i|((i as u32)|if i%3==0{1<<31}else{0},(g*Fr::from(2000000+i as u64)).to_affine())).collect();
        let mut queue=Vec::new();for i in 0..m {queue.push(((i as u32)|if i%2==0{1<<31}else{0},(g*Fr::from(3000000+i as u64)).to_affine()));queue.push((300+i as u32,(g*Fr::from(4000000+i as u64)).to_affine()));}
        let mut selected=vec![false;queue.len()];let mut seen=std::collections::BTreeSet::new();for (i,e) in queue.iter().enumerate().rev(){if q.iter().any(|p|key(p.0)==key(e.0))&&seen.insert(key(e.0)){selected[i]=true;}}
        for e in &q {let i=key(e.0)as usize;b[i]=(b[i]+signed::<G1Config,_>(e)).to_affine();}
        for (j,e) in queue.iter().enumerate(){if selected[j]{let i=key(e.0)as usize;b[i]=(b[i]+signed::<G1Config,_>(e)).to_affine();}}
        let remaining=queue.iter().enumerate().filter(|(j,_)|!selected[*j]).map(|(_,p)|*p).collect();
        let initial=(0..512).map(|i|(g*Fr::from(1000000+i as u64)).to_affine()).collect();(initial,q,queue,b,remaining)
    }
    #[test]fn scoped_admission_forecast_is_checked_and_denial_keeps_fallback() {
        assert!(forecast(usize::MAX).is_none());assert!(reserve(352,1,|_|panic!("wrong geometry must not reserve")).is_none());
        assert!(reserve(200,0,|_|panic!("empty reservation")).is_none());assert!(reserve(200,22,|bytes|{assert!(bytes.unwrap()>22*50000);None}).is_none());
        let admission=reserve(200,2,Reservation::analysis).unwrap();let state=admission.window().unwrap();assert_eq!(state.len(),1);
        assert_eq!(state[0].scratch.ordinary_records.capacity(),25);assert_eq!(state[0].scratch.triple_records.capacity(),4);
    }
    #[test]fn all_scoped_collision_counts_signed_queue_and_arithmetic() {
        let admission=reserve(200,1,Reservation::analysis).unwrap();let mut state=admission.window().unwrap();
        for m in 0..=40 {let(mut b,q,mut queue,expected,left)=fixture(m);let original=b.clone();let original_queue=queue.clone();let selected=state[0].apply::<G1Config,_>(&mut b,&q,&mut queue);
            if (1..=32).contains(&m) {assert!(selected);assert_eq!(b,expected);assert_eq!(queue,left);}
            else {assert!(!selected);assert_eq!(b,original);assert_eq!(queue,original_queue);}
        }
    }
}

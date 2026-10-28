//! Shared inversion for ordinary and sparse triple signed additions.
use super::*;
use std::borrow::Borrow;
type Entry<P>=(u32,P);
#[derive(Clone,Copy)]
struct TripleRecord {prefix:FqBatch,denominator:FqBatch,first:FqBatch,second:FqBatch,ax:FqBatch,ay:FqBatch,xsum:FqBatch,rx:FqBatch}
#[derive(Default)]
struct MixedScratch {
    slots:Vec<[u32;3]>,selected:Vec<u8>,ordinary:Vec<usize>,triples:Vec<[usize;2]>,
    ordinary_records:Vec<FqAddGroup>,triple_records:Vec<TripleRecord>,
}
fn key(encoded:u32)->u32 {encoded&0x7fffffff}
fn signed<C:CurveConfig<Base=Fq>,P:Borrow<Affine<C>>>(e:&Entry<P>)->Affine<C> {let p=*e.1.borrow();if e.0>>31!=0 {-p}else{p}}
impl MixedScratch {
    fn classify<P:Copy>(&mut self,q:&[Entry<P>],queue:&[Entry<P>]) {
        self.ordinary.clear();self.triples.clear();
        self.selected.clear();self.selected.resize(queue.len(),0);
        if q.is_empty() {return;}
        let size=(2*q.len()).next_power_of_two();self.slots.resize(size,[0;3]);self.slots.fill([0;3]);
        let mask=size-1;
        for (i,e) in q.iter().enumerate() {
            let id=key(e.0)+1;let mut slot=(id.wrapping_mul(0x9e3779b9)as usize)&mask;
            while self.slots[slot][0]!=0 {assert_ne!(self.slots[slot][0],id,"distinct pending destinations");slot=(slot+1)&mask;}
            self.slots[slot]=[id,i as u32+1,0];
        }
        for (j,e) in queue.iter().enumerate().rev() {
            let id=key(e.0)+1;let mut slot=(id.wrapping_mul(0x9e3779b9)as usize)&mask;
            while self.slots[slot][0]!=0 && self.slots[slot][0]!=id {slot=(slot+1)&mask;}
            if self.slots[slot][0]==id && self.slots[slot][2]==0 {self.slots[slot][2]=j as u32+1;self.selected[j]=1;}
        }
        // Restore pending order without sorting or a per-bucket array.
        self.ordinary.resize(q.len(),usize::MAX);
        for slot in &self.slots {if slot[0]!=0 {self.ordinary[slot[1]as usize-1]=slot[2]as usize;}}
        for i in 0..q.len() {let j=self.ordinary[i];if j>0 {if self.triples.len()==MATCHES {self.triples.clear();return;}self.triples.push([i,j-1]);}}
        let mut count=0;for i in 0..q.len() {if self.ordinary[i]==0 {self.ordinary[count]=i;count+=1;}}
        self.ordinary.truncate(count);
    }
    fn compact<P:Copy>(&self,queue:&mut[Entry<P>])->usize {
        let mut out=0;for i in 0..queue.len() {if self.selected[i]==0 {queue[out]=queue[i];out+=1;}}out
    }
    fn bytes(&self)->usize {
        self.slots.capacity()*std::mem::size_of::<[u32;3]>()+self.selected.capacity()+self.ordinary.capacity()*std::mem::size_of::<usize>()
            +self.triples.capacity()*std::mem::size_of::<[usize;2]>()+self.ordinary_records.capacity()*std::mem::size_of::<FqAddGroup>()
            +self.triple_records.capacity()*std::mem::size_of::<TripleRecord>()
    }
}
fn complete<C:CurveConfig<Base=Fq>,P:Copy+Borrow<Affine<C>>>(b:&mut[Affine<C>],q:&[Entry<P>],queue:&[Entry<P>],s:&MixedScratch) {
    for e in q {let i=key(e.0)as usize;b[i]=(b[i]+signed::<C,_>(e)).to_affine();}
    for &[_,j] in &s.triples {let e=&queue[j];let i=key(e.0)as usize;b[i]=(b[i]+signed::<C,_>(e)).to_affine();}
}
fn mixed<C:CurveConfig<Base=Fq>,P:Copy+Borrow<Affine<C>>>(b:&mut[Affine<C>],q:&[Entry<P>],queue:&[Entry<P>],s:&mut MixedScratch) {
    if q.is_empty() {return;}
    if q.iter().any(|e| b[key(e.0)as usize].is_identity() || e.1.borrow().is_identity() || b[key(e.0)as usize].x==e.1.borrow().x)
        || s.triples.iter().any(|r|queue[r[1]].1.borrow().is_identity()) {complete::<C,_>(b,q,queue,s);return;}
    s.ordinary_records.clear();s.triple_records.clear();
    s.ordinary_records.reserve(s.ordinary.len().div_ceil(LANES));s.triple_records.reserve(s.triples.len().div_ceil(LANES));
    let mut chains=FqBatch::one();
    for row in s.ordinary.chunks(LANES) {
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|if i<row.len(){b[key(q[row[i]].0)as usize]}else{Affine::<C>::IDENTITY});
        let c:[Affine<C>;LANES]=core::array::from_fn(|i|if i<row.len(){*q[row[i]].1.borrow()}else{Affine::<C>::new(Fq::ONE,Fq::ZERO)});
        let negative=(0..row.len()).fold(0u8,|mask,i|mask|(((q[row[i]].0>>31)as u8)<<i));
        // Same field spelling and write order as stream_g1_impl's ordinary records.
        unsafe {
            let slot=s.ordinary_records.as_mut_ptr().add(s.ordinary_records.len());
            let ax=FqBatch::new(&a.map(|p|p.x));core::ptr::addr_of_mut!((*slot).ax).write(ax);
            let bx=FqBatch::new(&c.map(|p|p.x));let denominator=canonical_sub::<true>(&bx,&ax);
            core::ptr::addr_of_mut!((*slot).xsum).write(canonical_add::<true>(&ax,&bx));
            core::ptr::addr_of_mut!((*slot).denominator).write(denominator);
            let ay=FqBatch::new(&a.map(|p|p.y));core::ptr::addr_of_mut!((*slot).ay).write(ay);
            let by=FqBatch::new(&c.map(|p|p.y));let numerator=scaled_signed_dy::<true,true>(&by,&ay,&chains,negative);
            core::ptr::addr_of_mut!((*slot).numerator).write(numerator);chains=chains.mul(&denominator);
            s.ordinary_records.set_len(s.ordinary_records.len()+1);
        }
    }
    for row in s.triples.chunks(LANES) {
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|if i<row.len(){b[key(q[row[i][0]].0)as usize]}else{Affine::<C>::IDENTITY});
        let c:[Affine<C>;LANES]=core::array::from_fn(|i|if i<row.len(){signed::<C,_>(&q[row[i][0]])}else{Affine::<C>::new(Fq::ONE,Fq::ZERO)});
        let r:[Affine<C>;LANES]=core::array::from_fn(|i|if i<row.len(){signed::<C,_>(&queue[row[i][1]])}else{Affine::<C>::new(Fq::ONE,Fq::ZERO)});
        let ax=FqBatch::new(&a.map(|p|p.x));let ay=FqBatch::new(&a.map(|p|p.y));
        let bx=FqBatch::new(&c.map(|p|p.x));let by=FqBatch::new(&c.map(|p|p.y));
        let rx=FqBatch::new(&r.map(|p|p.x));let ry=FqBatch::new(&r.map(|p|p.y));
        let d=canonical_sub::<true>(&bx,&ax);let n=canonical_sub::<true>(&by,&ay);let d2=d.mul(&d);
        let xsum=canonical_add::<true>(&ax,&bx);let h=d2.difference2(&xsum.add(&rx),&n,&n);
        let u=d.sum2(&ay.add(&ry),&n,&rx.sub(&ax));let denominator=d.mul(&h);
        // No output has been overwritten. Zero includes either denominator exception.
        if denominator.get().iter().any(Field::is_zero) {complete::<C,_>(b,q,queue,s);return;}
        let first=n.mul(&h);let second=d2.mul(&u);
        s.triple_records.push(TripleRecord{prefix:chains,denominator,first,second,ax,ay,xsum,rx});chains=chains.mul(&denominator);
    }
    let mut inverse=chains.inverse_nonzero();
    for (j,g) in s.triple_records.iter().enumerate().rev() {
        let factor=g.prefix.mul(&inverse);inverse=inverse.mul(&g.denominator);
        let l=g.first.mul(&factor);let w=g.second.mul(&factor);let k=w.sub(&l);
        let xs=l.mul(&l).sub(&g.xsum);let x=k.mul(&k).sub(&xs).sub(&g.rx);
        let y=w.sum2(&xs.sub(&x),&l,&x.sub(&g.ax)).add(&g.ay);let(x,y)=(x.get(),y.get());
        for i in 0..LANES.min(s.triples.len()-j*LANES) {b[key(q[s.triples[j*LANES+i][0]].0)as usize]=Affine::<C>::new(x[i],y[i]);}
    }
    for (j,g) in s.ordinary_records.iter().enumerate().rev() {
        let slope=g.numerator.mul(&inverse);inverse=inverse.mul(&g.denominator);
        let x=slope.mul(&slope).sub(&g.xsum);let y=slope.mul(&g.ax.sub(&x)).sub(&g.ay);let(x,y)=(x.get(),y.get());
        for i in 0..LANES.min(s.ordinary.len()-j*LANES) {b[key(q[s.ordinary[j*LANES+i]].0)as usize]=Affine::<C>::new(x[i],y[i]);}
    }
}
// Optional mixed-flush storage. Query owns one shared peak reservation.
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

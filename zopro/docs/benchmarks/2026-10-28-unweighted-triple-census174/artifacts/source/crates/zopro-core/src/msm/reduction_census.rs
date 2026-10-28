//! Counts only first-layer unweighted reduction inputs, with no point mutation.
use super::*;
use std::sync::atomic::{AtomicBool,AtomicU64,Ordering};
static ACTIVE:AtomicBool=AtomicBool::new(false);
static COUNTS:[[AtomicU64;32];4]=[const{[const{AtomicU64::new(0)};32]};4];

/// Caller must have joined all previous SDK work before resetting.
pub fn begin() {
    assert!(!ACTIVE.load(Ordering::Acquire));
    for cell in &COUNTS {for x in cell{x.store(0,Ordering::Relaxed);}}
    ACTIVE.store(true,Ordering::Release);
}
/// Caller must have joined the observed SDK work before collecting.
pub fn end()->[[u64;32];4] {
    assert!(ACTIVE.swap(false,Ordering::AcqRel));
    core::array::from_fn(|i|core::array::from_fn(|j|COUNTS[i][j].load(Ordering::Relaxed)))
}
pub(super) fn observe<C:CurveConfig>(points:&[LimbPoint<C::Base>],starts:&[usize],counts:&[usize],kind:usize)
where C::Base:BatchInverse {
    if !ACTIVE.load(Ordering::Acquire){return;}
    assert!(kind<2);
    let degree=core::mem::size_of::<C::Base>()/32;
    assert!((1..=2).contains(&degree));
    let cell=2*kind+degree-1;
    let mut local=[0u64;32];local[0]=1;local[1]=counts.len()as u64;
    for (&start,&n) in starts.iter().zip(counts) {
        local[2]+=n as u64;local[3]+=(n/2)as u64;local[4]+=(n/3)as u64;
        local[9]+=(n%3)as u64;local[12]+=u64::from(n>=3);local[13]=local[13].max(n as u64);
        let bin=match n{0=>0,1=>1,2=>2,3=>3,_=>(n.ilog2()as usize+2).min(13)};
        local[16+bin]+=1;
        for triple in points[start..start+n].chunks_exact(3) {
            if triple.iter().any(|p|p.is_infinity()){local[6]+=1;continue;}
            let p=affine::<C>(triple[0]);let q=affine::<C>(triple[1]);let r=affine::<C>(triple[2]);
            if p.x==q.x{local[7]+=1;continue;}
            let d=q.x-p.x;let numerator=q.y-p.y;
            let h=d.square()*(p.x+q.x+r.x)-numerator.square();
            if h==C::Base::ZERO{local[8]+=1;}else{local[5]+=1;}
        }
    }
    local[10]=u64::from(local[5]>=LANES as u64);
    local[11]=u64::from(local[4]>0&&local[5]==local[4]);
    local[14]=local[4];local[15]=u64::from(local[4]>local[5]);
    assert_eq!(local[4],local[5]+local[6]+local[7]+local[8]);
    assert_eq!(local[2],3*local[4]+local[9]);
    for (i,n) in local.into_iter().enumerate(){
        if i==13||i==14{COUNTS[cell][i].fetch_max(n,Ordering::Relaxed);}else{COUNTS[cell][i].fetch_add(n,Ordering::Relaxed);}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G1Config,G2Config};
    fn fixture<C:CurveConfig>()->Vec<LimbPoint<C::Base>> where C::Base:BatchInverse {
        let g=Projective::<C>::generator();
        let p=g.to_affine();let q=(g*Fr::from(2u64)).to_affine();let r=(g*Fr::from(4u64)).to_affine();
        [p,q,r,Affine::IDENTITY,q,r,p,-p,r,p,q,-((g*Fr::from(3u64)).to_affine())].map(limbs::<C>).to_vec()
    }
    #[test]
    fn first_layer_guards_match_curve_cases_and_disable_cleanly(){
        let g1=fixture::<G1Config>();let g2=fixture::<G2Config>();
        begin();observe::<G1Config>(&g1,&[0,3,6,9],&[3;4],0);observe::<G2Config>(&g2,&[0,3,6,9],&[3;4],0);
        let counts=end();
        for cell in &counts[..2]{assert_eq!(&cell[..10],&[1,4,12,4,4,1,1,1,1,0]);assert_eq!(cell[19],4);}
        observe::<G1Config>(&g1,&[0],&[12],1);
        assert_eq!(COUNTS[2][0].load(Ordering::Relaxed),0);
        begin();let empty=end();assert!(empty.iter().flatten().all(|&x|x==0));
    }
    #[test]
    fn concurrent_counts_preserve_slots_remainders_and_histograms(){
        let g=Projective::<G1Config>::generator();
        let points:Vec<_>=Projective::batch_to_affine(&(1..=40).map(|i|g*Fr::from(i as u64)).collect::<Vec<_>>()).into_iter().map(limbs::<G1Config>).collect();
        let starts=[0usize,0,1,3,6,10,15,23];let sizes=[0usize,1,2,3,4,5,8,17];
        begin();std::thread::scope(|scope|{for _ in 0..4{let p=&points;let a=&starts;let n=&sizes;scope.spawn(move||observe::<G1Config>(p,a,n,1));}});
        let counts=end();let cell=counts[2];
        assert_eq!(cell[0],4);assert_eq!(cell[1],32);assert_eq!(cell[2],160);assert_eq!(cell[3],72);
        assert_eq!(cell[4],40);assert_eq!(cell[9],40);assert_eq!(cell[13],17);assert_eq!(cell[14],10);
        assert_eq!(cell[4],cell[5]+cell[6]+cell[7]+cell[8]);assert_eq!(cell[16..30].iter().sum::<u64>(),32);
    }
}

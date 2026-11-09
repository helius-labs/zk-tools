//! Compact factors for the last five quad layers and the two within-quad stages.
use super::*;
use zopro_bn254::{FrConfig,ifma8::{FixedFactor,FixedFactor8}};
use crate::preparation::Reservation;
pub(super) type FixedQuad = [FixedFactor<FrConfig>;4];

pub(super) struct FixedRoots {
    first: usize,
    offsets: [usize;6],
    layers: Vec<FixedQuad>,
    pair: FixedQuad,
    single: Option<FixedQuad>,
    charged: usize,
    _reservation: Reservation,
}

impl FixedRoots {
    pub(super) fn prepare(roots:&Twiddles)->Option<Box<Self>> {
        #[cfg(zopro_fixed_fft_diagnostics)] {
            diag::ATTEMPTS.fetch_add(1,core::sync::atomic::Ordering::Relaxed);
            if diag::DENY.load(core::sync::atomic::Ordering::Acquire) {
                diag::DENIALS.fetch_add(1,core::sync::atomic::Ordering::Relaxed);
                return None;
            }
        }
        if roots.layers.is_empty() {return None;}
        let first=roots.layers.len().saturating_sub(5);
        let count=roots.layers[first..].iter().try_fold(0usize,|n,v|n.checked_add(v.len()))?;
        if count>31 {return None;}
        let bytes=core::mem::size_of::<Self>().checked_add(count.checked_mul(core::mem::size_of::<FixedQuad>())?)?;
        let reservation=Reservation::analysis(Some(bytes))?;
        let convert=|q:&Quad| q.get().map(|v|FixedFactor::<FrConfig>::from_canonical(v.to_canonical()).expect("canonical FFT factor"));
        let mut layers=Vec::with_capacity(count);
        if layers.capacity()>count {return None;}
        let mut offsets=[0;6];
        for (i,layer) in roots.layers[first..].iter().enumerate() {
            offsets[i]=layers.len();
            layers.extend(layer.iter().map(convert));
            offsets[i+1]=layers.len();
        }
        let cache=Box::new(Self {first,offsets,layers,pair:convert(&roots.pair),single:roots.single.as_ref().map(convert),charged:bytes,_reservation:reservation});
        #[cfg(zopro_fixed_fft_diagnostics)] diag::LIVE.fetch_add(bytes,core::sync::atomic::Ordering::AcqRel);
        Some(cache)
    }
    pub(super) fn layer(&self,level:usize)->Option<&[FixedQuad]> {
        let i=level.checked_sub(self.first)?;
        if i>=5 {return None;}
        Some(&self.layers[self.offsets[i]..self.offsets[i+1]])
    }
    pub(super) fn pair(&self)->&FixedQuad {&self.pair}
    pub(super) fn single(&self)->Option<&FixedQuad> {self.single.as_ref()}
    #[cfg(zopro_fixed_fft_diagnostics)]
    fn snapshot(&self)->[usize;4] {[self.layers.len(),self.layers.capacity(),self.charged,core::mem::size_of::<Self>()]}
}
#[cfg(zopro_fixed_fft_diagnostics)]
impl Drop for FixedRoots {fn drop(&mut self){diag::LIVE.fetch_sub(self.charged,core::sync::atomic::Ordering::AcqRel);}}

#[inline(always)]
pub(super) fn expand(q:&FixedQuad)->FixedFactor8<FrConfig> {
    FixedFactor8::new(&[q[0],q[1],q[2],q[3],q[0],q[1],q[2],q[3]])
}

#[cfg(zopro_fixed_fft_diagnostics)]
pub(super) mod diag {
    use super::*;
    use core::sync::atomic::{AtomicBool,AtomicUsize,Ordering};
    pub(super) static DENY:AtomicBool=AtomicBool::new(false);
    pub(super) static ATTEMPTS:AtomicUsize=AtomicUsize::new(0);
    pub(super) static DENIALS:AtomicUsize=AtomicUsize::new(0);
    pub(super) static LIVE:AtomicUsize=AtomicUsize::new(0);
    static DIF:AtomicUsize=AtomicUsize::new(0);
    static DIT:AtomicUsize=AtomicUsize::new(0);
    pub(crate) fn product(forward:bool,n:usize){(if forward{&DIT}else{&DIF}).fetch_add(n,Ordering::Relaxed);}
    pub(crate) fn counters()->[usize;5] {[DIF.load(Ordering::Acquire),DIT.load(Ordering::Acquire),ATTEMPTS.load(Ordering::Acquire),DENIALS.load(Ordering::Acquire),LIVE.load(Ordering::Acquire)]}
    pub(crate) fn snapshots(plan:&Plan)->[[usize;4];3] {
        [&plan.roots,&plan.inverse_roots,plan.twisted.as_ref().map(|(r,_)|r).unwrap_or(&plan.roots)].map(|r|r.fixed.as_ref().map(|r|r.snapshot()).unwrap_or([0;4]))
    }
    pub(crate) fn unique_charged(plan:&Plan)->usize {
        plan.roots.fixed.as_ref().map_or(0,|r|r.charged)+plan.inverse_roots.fixed.as_ref().map_or(0,|r|r.charged)+plan.twisted.as_ref().and_then(|(r,_)|r.fixed.as_ref()).map_or(0,|r|r.charged)
    }
    pub(crate) fn clear(plan:&mut Plan){plan.roots.fixed=None;plan.inverse_roots.fixed=None;if let Some((r,_))=&mut plan.twisted{r.fixed=None;}}
    pub struct DenyGuard;
    impl Drop for DenyGuard {fn drop(&mut self){DENY.store(false,Ordering::Release);}}
    pub(crate) fn force_denial()->DenyGuard {assert!(!DENY.swap(true,Ordering::AcqRel));DenyGuard}
}

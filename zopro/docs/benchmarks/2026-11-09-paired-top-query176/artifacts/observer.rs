//! Process-local research counts. Reset/snapshot require a quiescent caller.
//! The benchmark runs one complete query at a time; this is not a service API.
use std::sync::atomic::{AtomicUsize,Ordering};
static ADMITTED:AtomicUsize=AtomicUsize::new(0);
static PARTS:AtomicUsize=AtomicUsize::new(0);
static BUNDLES:AtomicUsize=AtomicUsize::new(0);
static PARENT_CHARGE:AtomicUsize=AtomicUsize::new(0);
static BUNDLE_CHARGE:AtomicUsize=AtomicUsize::new(0);
pub(crate) fn admitted(charge:usize){ADMITTED.fetch_add(1,Ordering::Relaxed);PARENT_CHARGE.fetch_max(charge,Ordering::Relaxed);}
pub(crate) fn part(){PARTS.fetch_add(1,Ordering::Relaxed);}
pub(crate) fn record(charge:usize){BUNDLES.fetch_add(1,Ordering::Relaxed);BUNDLE_CHARGE.fetch_max(charge,Ordering::Relaxed);}
pub fn snapshot()->[usize;5]{[ADMITTED.load(Ordering::Relaxed),PARTS.load(Ordering::Relaxed),BUNDLES.load(Ordering::Relaxed),PARENT_CHARGE.load(Ordering::Relaxed),BUNDLE_CHARGE.load(Ordering::Relaxed)]}
pub fn reset(){for a in [&ADMITTED,&PARTS,&BUNDLES,&PARENT_CHARGE,&BUNDLE_CHARGE]{a.store(0,Ordering::Relaxed);}}
#[cfg(test)]mod observer_tests{
    use super::*;
    #[test]fn nested_parallel_counts_maxima_and_quiescent_reuse(){
        for workers in [1,4,16,32]{
            let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
            reset();assert_eq!(snapshot(),[0;5]);
            pool.broadcast(|ctx|{let index=ctx.index();admitted(index+100);rayon::join(||{part();record(index+200)},||{part();record(index+300)});});
            assert_eq!(snapshot(),[workers,2*workers,2*workers,workers+99,workers+299]);
            reset();assert_eq!(snapshot(),[0;5]);
        }
    }
}

/// Private measurement counters; no concurrent-request attribution is claimed.
pub mod phase_search_probe {
    use std::sync::{OnceLock, atomic::{AtomicUsize, Ordering}};
    static DIVISOR: OnceLock<usize> = OnceLock::new();
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    pub(crate) fn run<F: FnOnce() -> R, R>(f: F) -> R {
        let divisor=*DIVISOR.get_or_init(|| match std::env::var("ZOPRO_DIAGNOSTIC_SEARCH_DIVISOR").as_deref() {
            Ok("3")=>3, Ok("0")|Err(_)=>0, _=>panic!("unsupported diagnostic divisor")
        });
        if divisor==0 {return f();}
        let limit=rayon::current_num_threads().div_ceil(divisor);
        CALLS.fetch_add(1,Ordering::Relaxed);
        rayon_core::diagnostic_with_search_limit(limit,f)
    }
    pub fn snapshot() -> (usize, usize, usize) {
        let (limit,active)=rayon_core::diagnostic_search_state();
        (CALLS.load(Ordering::Relaxed),limit,active)
    }
}

//! Complete one proof's two G1 jobs before starting its G2 job.
pub(super) fn g1_then_g2<A: Send, B: Send, C>(
    a: impl FnOnce() -> A + Send,
    b: impl FnOnce() -> B + Send,
    c: impl FnOnce() -> C,
) -> ((A, B), C) {
    let g1 = rayon::join(a, b);
    (g1, c())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    #[test]
    fn g2_waits_for_both_g1_jobs_in_single_and_parallel_pools() {
        for workers in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(|| {
                    let finished = AtomicUsize::new(0);
                    let result = g1_then_g2(
                        || { finished.fetch_or(1, Ordering::SeqCst); 3 },
                        || { finished.fetch_or(2, Ordering::SeqCst); 5 },
                        || { assert_eq!(finished.load(Ordering::SeqCst), 3); 8 },
                    );
                    assert_eq!(result, ((3, 5), 8));
                });
        }
    }

    #[test]
    fn a_g1_panic_does_not_start_g2() {
        let started = AtomicBool::new(false);
        let result = std::panic::catch_unwind(|| {
            g1_then_g2(
                || -> usize { panic!("G1 failure") },
                || 5usize,
                || { started.store(true, Ordering::SeqCst); 8usize },
            )
        });
        assert!(result.is_err());
        assert!(!started.load(Ordering::SeqCst));
    }
}

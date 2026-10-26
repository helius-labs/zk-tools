//! Bound simultaneous victim scans without treating deferred scans as empty.
#![allow(unexpected_cfgs)]

use crossbeam_utils::CachePadded;
#[cfg(zopro_search_loom)]
use loom::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(zopro_search_loom))]
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub(super) struct Admission {
    active: CachePadded<AtomicUsize>,
    limit: usize,
}

#[derive(Debug)]
pub(super) struct Permit<'a> {
    active: Option<&'a AtomicUsize>,
}

impl Admission {
    pub(super) fn new(workers: usize, enabled: bool) -> Self {
        Self {
            active: CachePadded::new(AtomicUsize::new(0)),
            limit: if enabled {
                workers.div_ceil(2).max(1)
            } else {
                0
            },
        }
    }

    /// A failed attempt is deferred work search, not evidence of an empty pool.
    #[inline]
    pub(super) fn try_enter(&self) -> Option<Permit<'_>> {
        if self.limit == 0 {
            return Some(Permit { active: None });
        }
        let active = self.active.load(Ordering::Relaxed);
        if active >= self.limit {
            return None;
        }
        self.active
            .compare_exchange(active, active + 1, Ordering::Acquire, Ordering::Relaxed)
            .ok()
            .map(|_| Permit {
                active: Some(&self.active),
            })
    }
}

impl Drop for Permit<'_> {
    #[inline]
    fn drop(&mut self) {
        if let Some(active) = self.active {
            let previous = active.fetch_sub(1, Ordering::Release);
            debug_assert!(previous > 0);
        }
    }
}

#[cfg(all(test, not(zopro_search_loom)))]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    #[test]
    fn exact_budget_disabled_path_and_unwind_release() {
        assert!(!std::any::type_name::<AtomicUsize>().starts_with("loom::"));
        for workers in [1usize, 2, 3, 16, 32, 64, 96] {
            let admission = Admission::new(workers, true);
            let permits = (0..workers.div_ceil(2))
                .map(|_| admission.try_enter().unwrap())
                .collect::<Vec<_>>();
            assert!(admission.try_enter().is_none());
            drop(permits);
            assert_eq!(admission.active.load(Ordering::Relaxed), 0);
            assert!(std::panic::catch_unwind(|| {
                let _permit = admission.try_enter().unwrap();
                panic!("exercise scoped permit release");
            })
            .is_err());
            assert_eq!(admission.active.load(Ordering::Relaxed), 0);
        }
        let disabled = Admission::new(8, false);
        let permits = (0..100)
            .map(|_| disabled.try_enter().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(disabled.active.load(Ordering::Relaxed), 0);
        drop(permits);
    }

    #[test]
    fn concurrent_scans_respect_budget_and_return_all_credits() {
        for workers in [2usize, 8, 32] {
            let admission = Arc::new(Admission::new(workers, true));
            let barrier = Arc::new(Barrier::new(workers));
            let inside = Arc::new(AtomicUsize::new(0));
            let completed = Arc::new(AtomicUsize::new(0));
            std::thread::scope(|scope| {
                for _ in 0..workers {
                    let (admission, barrier, inside, completed) = (
                        admission.clone(),
                        barrier.clone(),
                        inside.clone(),
                        completed.clone(),
                    );
                    scope.spawn(move || {
                        barrier.wait();
                        for _ in 0..257 {
                            loop {
                                if let Some(permit) = admission.try_enter() {
                                    let now = inside.fetch_add(1, Ordering::SeqCst) + 1;
                                    assert!(now <= workers.div_ceil(2));
                                    std::thread::yield_now();
                                    inside.fetch_sub(1, Ordering::SeqCst);
                                    drop(permit);
                                    completed.fetch_add(1, Ordering::Relaxed);
                                    break;
                                }
                                std::thread::yield_now();
                            }
                        }
                    });
                }
            });
            assert_eq!(completed.load(Ordering::Relaxed), workers * 257);
            assert_eq!(inside.load(Ordering::Relaxed), 0);
            assert_eq!(admission.active.load(Ordering::Relaxed), 0);
        }
    }
}

#[cfg(all(test, zopro_search_loom))]
mod loom_tests {
    use super::*;
    use loom::{sync::Arc, thread};

    #[test]
    fn actual_atomic_type_is_instrumented() {
        assert!(std::any::type_name::<AtomicUsize>().starts_with("loom::"));
    }

    #[test]
    fn bounded_admission_and_release_under_modelled_interleavings() {
        for workers in [2usize, 3] {
            let mut builder = loom::model::Builder::new();
            builder.preemption_bound = Some(2);
            builder.check(move || {
                let admission = Arc::new(Admission::new(workers, true));
                let inside = Arc::new(AtomicUsize::new(0));
                let mut handles = Vec::new();
                for _ in 0..3 {
                    let (admission, inside) = (admission.clone(), inside.clone());
                    handles.push(thread::spawn(move || {
                        if let Some(permit) = admission.try_enter() {
                            let now = inside.fetch_add(1, Ordering::SeqCst) + 1;
                            assert!(now <= workers.div_ceil(2));
                            thread::yield_now();
                            inside.fetch_sub(1, Ordering::SeqCst);
                            drop(permit);
                        }
                    }));
                }
                for handle in handles {
                    handle.join().unwrap();
                }
                assert_eq!(admission.active.load(Ordering::Relaxed), 0);
                assert!(admission.try_enter().is_some());
            });
        }
    }
}

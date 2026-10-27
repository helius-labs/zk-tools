//! Optional nonblocking exclusion for redundant advancement scans.
use crate::primitive::sync::atomic::{AtomicBool, Ordering};
use crossbeam_utils::CachePadded;

pub(super) struct AdvanceGate {
    pub(super) enabled: bool,
    busy: CachePadded<AtomicBool>,
}

impl AdvanceGate {
    pub(super) fn new(enabled: bool) -> Self {
        Self { enabled, busy: CachePadded::new(AtomicBool::new(false)) }
    }

    pub(super) fn configured() -> Self {
        #[cfg(feature = "std")]
        let enabled = std::env::var_os("ZOPRO_CPU_SINGLE_EPOCH_ADVANCER")
            .map_or(false, |v| v == "1");
        #[cfg(not(feature = "std"))]
        let enabled = false;
        Self::new(enabled)
    }

    pub(super) fn try_enter(&self) -> Option<AdvancePermit<'_>> {
        self.busy.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .ok().map(|_| AdvancePermit(self))
    }
}

pub(super) struct AdvancePermit<'a>(&'a AdvanceGate);
impl Drop for AdvancePermit<'_> {
    fn drop(&mut self) { self.0.busy.store(false, Ordering::Release); }
}

#[cfg(all(test, not(crossbeam_loom)))]
mod tests {
    use super::*;
    use crate::Collector;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering as StdOrdering}, Barrier};

    fn collector() -> Collector {
        let mut c = Collector::new();
        Arc::get_mut(&mut c.global).unwrap().advance_gate.enabled = true;
        c
    }

    #[test]
    fn nonblocking_permit_excludes_and_releases_on_unwind() {
        let gate = AdvanceGate::new(true);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _permit = gate.try_enter().unwrap();
            std::thread::scope(|s| { s.spawn(|| assert!(gate.try_enter().is_none())); });
            panic!("test unwind");
        }));
        assert!(result.is_err());
        assert!(gate.try_enter().is_some());
    }

    #[test]
    fn held_permit_skips_advance_and_expired_collection_still_completes() {
        let c = collector();
        let h = c.register();
        let g = h.pin();
        let before = c.global.epoch.load(Ordering::Relaxed);
        let permit = c.global.advance_gate.try_enter().unwrap();
        assert_eq!(c.global.try_advance(&g), before);
        assert_eq!(c.global.epoch.load(Ordering::Relaxed), before);
        drop(permit);
        assert_eq!(c.global.try_advance(&g), before.successor());
    }

    #[test]
    fn pinned_reader_prevents_early_reclamation_then_releases() {
        let c = collector();
        let reader = c.register();
        let g = reader.pin();
        let dropped = Arc::new(AtomicUsize::new(0));
        let d = dropped.clone();
        std::thread::scope(|s| {
            s.spawn(|| {
                let h = c.register();
                let guard = h.pin();
                guard.defer(move || { d.fetch_add(1, StdOrdering::SeqCst); });
                guard.flush();
                drop(guard);
                for _ in 0..256 { h.pin().flush(); }
            }).join().unwrap();
        });
        assert_eq!(dropped.load(StdOrdering::SeqCst), 0);
        drop(g);
        for _ in 0..256 { reader.pin().flush(); }
        assert_eq!(dropped.load(StdOrdering::SeqCst), 1);
    }

    #[test]
    fn concurrent_retirements_are_eventually_reclaimed_once() {
        let c = collector();
        let dropped = Arc::new(AtomicUsize::new(0));
        let barrier = Barrier::new(8);
        std::thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    let h = c.register();
                    barrier.wait();
                    for _ in 0..1024 {
                        let g = h.pin();
                        let d = dropped.clone();
                        g.defer(move || { d.fetch_add(1, StdOrdering::SeqCst); });
                        g.flush();
                    }
                });
            }
        });
        let h = c.register();
        for _ in 0..8192 { h.pin().flush(); }
        assert_eq!(dropped.load(StdOrdering::SeqCst), 8 * 1024);
    }
}

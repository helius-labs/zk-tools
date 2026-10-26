//! A bounded model of retention/publication/final-Arc destruction only.
//! It does not model queue slots, extraction, or the complete scheduler.
use loom::{cell::UnsafeCell, sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}}, thread};

struct Allocation { id: usize, live: AtomicBool, data: UnsafeCell<usize> }
impl Allocation {
    fn new(id: usize) -> Arc<Self> { Arc::new(Self { id, live: AtomicBool::new(true), data: UnsafeCell::new(id) }) }
    fn read(&self) {
        self.data.with(|p| unsafe {
            assert!(self.live.load(Ordering::SeqCst), "premature logical reclamation");
            assert_eq!(*p, self.id, "reclaimed payload was observed");
            thread::yield_now();
            assert!(self.live.load(Ordering::SeqCst), "premature logical reclamation");
        });
    }
    fn reclaim(&self, freed: &AtomicUsize) {
        assert!(self.live.swap(false, Ordering::SeqCst), "allocation reclaimed twice");
        self.data.with_mut(|p| unsafe { *p = usize::MAX; });
        freed.fetch_add(1, Ordering::SeqCst);
    }
}

struct Inner {
    published: AtomicUsize,
    current: UnsafeCell<Arc<Allocation>>,
    retired: UnsafeCell<Vec<Arc<Allocation>>>,
    freed: Arc<AtomicUsize>,
}
// Mirrors the single Worker owner discipline, not arbitrary concurrent writers.
unsafe impl Sync for Inner {}
impl Inner {
    fn new(first: Arc<Allocation>, freed: Arc<AtomicUsize>) -> Arc<Self> {
        Arc::new(Self { published: AtomicUsize::new(first.id), current: UnsafeCell::new(first), retired: UnsafeCell::new(Vec::new()), freed })
    }
    fn resize_from_sole_owner(&self, next: Arc<Allocation>, break_retention: bool) {
        let old_id = self.published.swap(next.id, Ordering::Release);
        let old = self.current.with_mut(|p| unsafe { std::mem::replace(&mut *p, next) });
        assert_eq!(old.id, old_id);
        // The local old handle spans the swap-to-retired-vector transition.
        if break_retention { old.reclaim(&self.freed); }
        self.retired.with_mut(|p| unsafe { (*p).push(old); });
    }
}
impl Drop for Inner {
    fn drop(&mut self) {
        self.current.with(|p| unsafe {
            let current = &*p;
            if current.live.load(Ordering::SeqCst) { current.reclaim(&self.freed); }
        });
        self.retired.with(|p| unsafe { for allocation in &*p {
            if allocation.live.load(Ordering::SeqCst) { allocation.reclaim(&self.freed); }
        }});
    }
}

fn run(break_retention: bool) {
    let mut builder = loom::model::Builder::new();
    builder.preemption_bound = Some(2); builder.max_branches = 20_000;
    builder.check(move || {
        for name in [std::any::type_name::<Arc<Inner>>(), std::any::type_name::<AtomicUsize>(), std::any::type_name::<UnsafeCell<usize>>() ] {
            assert!(name.starts_with("loom::"), "unmodelled synchronization type {name}");
        }
        // Observer handles keep test memory allocated even for the negative
        // control. Reclamation is an instrumented logical transition, so the
        // failing model never invokes real use-after-free in the test runner.
        let nodes = Arc::new(vec![Allocation::new(0), Allocation::new(1), Allocation::new(2)]);
        let freed = Arc::new(AtomicUsize::new(0));
        let owner = Inner::new(nodes[0].clone(), freed.clone());
        let mut readers = Vec::new();
        for _ in 0..2 {
            let stealer = owner.clone(); let observers = nodes.clone();
            readers.push(thread::spawn(move || {
                let id = stealer.published.load(Ordering::Acquire);
                thread::yield_now(); observers[id].read();
                // Keep the queue's shared handle through the final access.
                drop(stealer);
            }));
        }
        owner.resize_from_sole_owner(nodes[1].clone(), break_retention);
        thread::yield_now();
        owner.resize_from_sole_owner(nodes[2].clone(), break_retention);
        drop(owner);
        for reader in readers { reader.join().unwrap(); }
        assert_eq!(freed.load(Ordering::SeqCst), 3);
        assert!(nodes.iter().all(|n| !n.live.load(Ordering::SeqCst)));
    });
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn all_generations_survive_owner_drop_and_inflight_readers() { run(false); }
    #[test] fn prematurely_reclaimed_generation_is_detected() {
        assert!(std::panic::catch_unwind(|| run(true)).is_err(), "negative control did not exercise reclamation");
    }
}

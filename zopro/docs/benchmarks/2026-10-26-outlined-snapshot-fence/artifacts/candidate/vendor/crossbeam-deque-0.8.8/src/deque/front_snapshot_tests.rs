use super::*;
use std::{alloc::{GlobalAlloc, Layout, System}, sync::atomic::AtomicBool, vec, vec::Vec};

// Unit-test executable only. No allocator hook is compiled into the SDK.
struct ArrayTracker;
static WATCH_LEN: AtomicUsize = AtomicUsize::new(0);
static WATCH: [AtomicUsize; 16] = [const { AtomicUsize::new(0) }; 16];
static FREED: [AtomicBool; 16] = [const { AtomicBool::new(false) }; 16];
#[global_allocator]
static ALLOCATOR: ArrayTracker = ArrayTracker;
unsafe impl GlobalAlloc for ArrayTracker {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { unsafe { System.alloc(layout) } }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { unsafe { System.alloc_zeroed(layout) } }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 { unsafe { System.realloc(ptr, layout, size) } }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let count = WATCH_LEN.load(Ordering::Acquire);
        for i in 0..count {
            if WATCH[i].load(Ordering::Acquire) == ptr as usize { FREED[i].store(true, Ordering::Release); }
        }
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn worker<T>(fifo: bool) -> Worker<T> { if fifo { Worker::new_fifo() } else { Worker::new_lifo() } }
struct Item { id: usize, drops: Arc<Vec<AtomicUsize>> }
impl Drop for Item { fn drop(&mut self) { assert_eq!(self.drops[self.id].fetch_add(1, Ordering::SeqCst), 0, "double payload drop {}", self.id); } }
fn observe(task: Item, seen: &[AtomicUsize]) { assert_eq!(seen[task.id].fetch_add(1, Ordering::SeqCst), 0, "duplicate task {}", task.id); }

#[test]
fn tile_bounds_order_mixed_flavors_and_duplicate_references() {
    assert_eq!(Stealer::<usize>::steal_one_from_front_snapshots(&[]), Steal::Empty);
    for fifo in [false, true] {
        let a = worker(fifo); let b = worker(!fifo); let empty = worker::<usize>(fifo);
        a.push(1); a.push(2); b.push(3); b.push(4);
        let sa = a.stealer(); let sb = b.stealer(); let se = empty.stealer();
        let refs = [&se, &sa, &sa, &se, &sb, &sb, &se, &se];
        for expected in 1..=4 { assert_eq!(Stealer::steal_one_from_front_snapshots(&refs), Steal::Success(expected)); }
        assert_eq!(Stealer::steal_one_from_front_snapshots(&refs), Steal::Empty);
        let too_many = [&se; 9]; let pinned = epoch::is_pinned();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| Stealer::steal_one_from_front_snapshots(&too_many))).is_err());
        assert_eq!(epoch::is_pinned(), pinned);
    }
}

#[test]
fn stale_front_retry_is_aggregated_and_later_success_wins() {
    let a = Worker::new_lifo(); let b = Worker::new_fifo();
    a.push(11); b.push(23); let sa = a.stealer(); let sb = b.stealer();
    let guard = &epoch::pin();
    let old = [sa.inner.front.load(Ordering::Acquire), sb.inner.front.load(Ordering::Acquire)];
    assert_eq!(sa.steal(), Steal::Success(11));
    atomic::fence(Ordering::SeqCst);
    assert_eq!(Stealer::snapshot_from_fronts(&[&sa], &old[..1], guard), Steal::Retry);
    assert_eq!(Stealer::snapshot_from_fronts(&[&sa, &sb], &old, guard), Steal::Success(23));
    assert_eq!(Stealer::steal_one_from_front_snapshots(&[&sa, &sb]), Steal::Empty);
}

#[test]
fn guards_drop_on_empty_success_and_nested_calls() {
    let w = Worker::new_lifo(); let s = w.stealer(); let before = epoch::is_pinned();
    assert_eq!(Stealer::steal_one_from_front_snapshots(&[&s]), Steal::Empty);
    assert_eq!(epoch::is_pinned(), before);
    w.push(41);
    assert_eq!(Stealer::steal_one_from_front_snapshots(&[&s]), Steal::Success(41));
    assert_eq!(epoch::is_pinned(), before);
    let pinned = epoch::pin(); w.push(42);
    assert_eq!(Stealer::steal_one_from_front_snapshots(&[&s]), Steal::Success(42));
    assert!(epoch::is_pinned()); drop(pinned); assert_eq!(epoch::is_pinned(), before);
}

#[test]
fn owner_drop_and_pending_payload_destruction_remain_exactly_once() {
    for fifo in [false, true] {
        let count = 4097; let drops = Arc::new((0..count).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
        let seen = (0..count).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>();
        let w = worker(fifo); for id in 0..count { w.push(Item { id, drops: drops.clone() }); }
        let s = w.stealer(); let clone = s.clone(); drop(w);
        for _ in 0..1024 {
            match Stealer::steal_one_from_front_snapshots(&[&s, &clone]) { Steal::Success(task) => observe(task, &seen), _ => panic!("missing queued payload") }
        }
        drop(s); drop(clone);
        assert_eq!(seen.iter().map(|n| n.load(Ordering::SeqCst)).sum::<usize>(), 1024);
        assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 1));
    }
}

#[test]
fn concurrent_multiqueue_growth_owner_pop_and_tiles_are_exactly_once() {
    let per_queue = 10_003; let total = per_queue * 4;
    let drops = Arc::new((0..total).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
    let seen = Arc::new((0..total).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
    let workers = (0..4).map(|i| worker::<Item>(i % 2 == 0)).collect::<Vec<_>>();
    let stealers = workers.iter().map(Worker::stealer).collect::<Vec<_>>();
    let done = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for (index, w) in workers.into_iter().enumerate() {
            let drops = &drops; let seen = &seen; let done = &done;
            scope.spawn(move || {
                for i in 0..per_queue {
                    w.push(Item { id: index * per_queue + i, drops: drops.clone() });
                    if i % 7 == 0 { if let Some(task) = w.pop() { observe(task, seen); } }
                }
                while let Some(task) = w.pop() { observe(task, seen); }
                drop(w); done.fetch_add(1, Ordering::Release);
            });
        }
        for _ in 0..3 {
            let stealers = &stealers; let done = &done; let seen = &seen;
            scope.spawn(move || {
                let refs = stealers.iter().collect::<Vec<_>>();
                loop { match Stealer::steal_one_from_front_snapshots(&refs) {
                    Steal::Success(task) => observe(task, seen),
                    Steal::Empty if done.load(Ordering::Acquire) == 4 => break,
                    Steal::Retry | Steal::Empty => std::thread::yield_now(),
                }}
            });
        }
    });
    assert!(seen.iter().all(|n| n.load(Ordering::SeqCst) == 1));
    assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 1));
}

#[test]
fn ordinary_retired_arrays_reclaim_after_snapshot_guards_drop() {
    assert_eq!(WATCH_LEN.load(Ordering::Relaxed), 0);
    let w = Worker::<u64>::new_lifo(); let s = w.stealer();
    let held = epoch::pin();
    for i in 0..5 {
        let before = w.buffer.get();
        FREED[i].store(false, Ordering::Relaxed); WATCH[i].store(before.ptr as usize, Ordering::Release); WATCH_LEN.store(i + 1, Ordering::Release);
        w.reserve(before.cap + 1);
        assert!(w.buffer.get().cap >= 2 * before.cap);
    }
    for _ in 0..1024 { assert_eq!(Stealer::steal_one_from_front_snapshots(&[&s]), Steal::Empty); }
    assert!(FREED[..5].iter().all(|flag| !flag.load(Ordering::Acquire)));
    drop(held);
    for _ in 0..8192 {
        let guard = epoch::pin(); guard.flush(); drop(guard);
        assert_eq!(Stealer::steal_one_from_front_snapshots(&[&s]), Steal::Empty);
        if FREED[..5].iter().all(|flag| flag.load(Ordering::Acquire)) { break; }
        std::thread::yield_now();
    }
    let reclaimed = FREED[..5].iter().all(|flag| flag.load(Ordering::Acquire));
    WATCH_LEN.store(0, Ordering::Release);
    assert!(reclaimed, "normal retirement failed to reclaim all watched arrays after guards dropped");
}

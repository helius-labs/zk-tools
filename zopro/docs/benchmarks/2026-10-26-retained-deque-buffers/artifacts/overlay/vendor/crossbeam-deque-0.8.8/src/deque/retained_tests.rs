use super::*;
use std::collections::HashMap;
use std::sync::{atomic::AtomicBool, Mutex, OnceLock};

fn allocations() -> &'static Mutex<HashMap<usize, usize>> {
    static VALUES: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    VALUES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(super) fn allocation(ptr: usize, allocate: bool) {
    let mut all = allocations().lock().unwrap();
    if allocate {
        *all.entry(ptr).or_default() += 1;
    } else {
        let count = all.get_mut(&ptr).expect("deallocating an unknown payload array");
        *count -= 1;
        if *count == 0 { all.remove(&ptr); }
    }
}

fn worker<T>(fifo: bool, retained: bool) -> Worker<T> {
    let mut w = if fifo { Worker::new_fifo() } else { Worker::new_lifo() };
    // Before creating any stealer or performing an operation, choose the test
    // queue's immutable mode. This permits both modes in one mixed-batch test.
    Arc::get_mut(&mut w.inner).unwrap().retained = retained;
    w
}

fn array_pointers<T>(inner: &Inner<T>) -> Vec<usize> {
    assert!(inner.retained);
    // Call only with the sole owner idle, or after the owner has been joined.
    unsafe {
        let guard = epoch::unprotected();
        let mut pointers = (&*inner.retired_buffers.get()).iter()
            .map(|b| b.load(Ordering::Relaxed, guard).deref().ptr as usize).collect::<Vec<_>>();
        pointers.push(inner.buffer.load(Ordering::Relaxed, guard).deref().ptr as usize);
        pointers
    }
}

fn assert_alive(pointers: &[usize], alive: bool) {
    let all = allocations().lock().unwrap();
    for p in pointers { assert_eq!(all.contains_key(p), alive, "array {p:x}"); }
}

struct Item { id: usize, drops: Arc<Vec<AtomicUsize>> }
impl Drop for Item {
    fn drop(&mut self) { assert_eq!(self.drops[self.id].fetch_add(1, Ordering::SeqCst), 0, "double payload drop {}", self.id); }
}
fn observe(task: Item, seen: &mut [usize]) { seen[task.id] += 1; }

#[test]
fn constructors_sample_explicit_process_mode() {
    let expected = std::env::var("ZOPRO_CPU_RETAINED_DEQUE_BUFFERS").is_ok_and(|v| v == "1");
    assert_eq!(Worker::<usize>::new_fifo().inner.retained, expected);
    assert_eq!(Worker::<usize>::new_lifo().inner.retained, expected);
}

#[test]
fn geometric_storage_bound_and_no_shrink_across_reused_bursts() {
    for fifo in [false, true] {
        let w = worker::<usize>(fifo, true);
        for burst in 0..16 {
            for i in 0..1024 { w.push(burst * 1024 + i); }
            assert_eq!(w.buffer.get().cap, 1024);
            let retired = unsafe { &*w.inner.retired_buffers.get() };
            assert_eq!(retired.len(), 4);
            let capacities = retired.iter().map(|b| unsafe { b.load(Ordering::Relaxed, epoch::unprotected()).deref().cap }).collect::<Vec<_>>();
            assert_eq!(capacities, [64, 128, 256, 512]);
            let old: usize = capacities.iter().sum();
            assert!(old < w.buffer.get().cap);
            assert!(old + w.buffer.get().cap < 2 * w.buffer.get().cap);
            for i in 0..1024 { assert_eq!(w.pop(), Some(burst * 1024 + if fifo { i } else { 1023 - i })); }
            assert_eq!(w.pop(), None);
            assert_eq!(w.buffer.get().cap, 1024);
        }
        let pointers = array_pointers(&w.inner); assert_alive(&pointers, true);
        drop(w); assert_alive(&pointers, false);
        let ordinary = worker::<usize>(fifo, false);
        for i in 0..1024 { ordinary.push(i); }
        while ordinary.pop().is_some() {}
        assert_eq!(ordinary.buffer.get().cap, MIN_CAP);
    }
}

#[test]
fn owner_can_drop_before_stealers_without_reclaiming_any_generation() {
    for fifo in [false, true] {
        let drops = Arc::new((0..4096).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
        let w = worker(fifo, true);
        for id in 0..4096 { w.push(Item { id, drops: drops.clone() }); }
        let s = w.stealer(); let clone = s.clone();
        let pointers = array_pointers(&w.inner); assert_eq!(pointers.len(), 7);
        let mut seen = vec![0; 4096];
        for _ in 0..37 { observe(w.pop().unwrap(), &mut seen); }
        drop(w); assert_alive(&pointers, true);
        while let Steal::Success(task) = s.steal() { observe(task, &mut seen); }
        assert!(seen.iter().all(|n| *n == 1));
        drop(s); assert_alive(&pointers, true);
        std::thread::Builder::new().stack_size(128 * 1024).spawn(move || drop(clone)).unwrap().join().unwrap();
        assert_alive(&pointers, false);
        assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 1));
    }
}

#[test]
fn final_drop_drops_only_live_current_payloads_once() {
    let drops = Arc::new((0..2048).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
    let w = worker(false, true);
    for id in 0..2048 { w.push(Item { id, drops: drops.clone() }); }
    let s = w.stealer(); let pointers = array_pointers(&w.inner);
    drop(w); assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 0));
    drop(s); assert_alive(&pointers, false);
    assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 1));
}

#[test]
fn every_mixed_mode_flavor_batch_and_limit_preserves_unique_payloads() {
    for source_fifo in [false, true] { for dest_fifo in [false, true] {
        for source_retained in [false, true] { for dest_retained in [false, true] {
            for and_pop in [false, true] { for limit in [1, 3, 32] {
                let source = worker(source_fifo, source_retained); let dest = worker(dest_fifo, dest_retained);
                let drops = Arc::new((0..320).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
                for id in 0..257 { source.push(Item { id, drops: drops.clone() }); }
                for id in 257..320 { dest.push(Item { id, drops: drops.clone() }); }
                let stealer = source.stealer(); let mut seen = vec![0; 320];
                loop {
                    if and_pop {
                        match stealer.steal_batch_with_limit_and_pop(&dest, limit) {
                            Steal::Success(task) => observe(task, &mut seen), Steal::Empty => break, Steal::Retry => continue,
                        }
                    } else {
                        match stealer.steal_batch_with_limit(&dest, limit) {
                            Steal::Success(()) => {}, Steal::Empty => break, Steal::Retry => continue,
                        }
                    }
                }
                while let Some(task) = source.pop() { observe(task, &mut seen); }
                while let Some(task) = dest.pop() { observe(task, &mut seen); }
                assert!(seen.iter().all(|n| *n == 1));
                drop(stealer); drop(source); drop(dest);
                assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 1));
            }}
        }}
    }}
}

#[test]
fn concurrent_growth_owner_pop_and_steal_are_exactly_once() {
    for fifo in [false, true] {
        let count = 50_000;
        let drops = Arc::new((0..count).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
        let seen = Arc::new((0..count).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
        let w = worker(fifo, true); for id in 0..1024 { w.push(Item { id, drops: drops.clone() }); }
        let s = w.stealer(); let done = AtomicBool::new(false);
        std::thread::scope(|scope| {
            for _ in 0..3 {
                let stealer = s.clone(); let seen = &seen; let done = &done;
                scope.spawn(move || loop {
                    match stealer.steal() {
                        Steal::Success(task) => { assert_eq!(seen[task.id].fetch_add(1, Ordering::SeqCst), 0); }
                        Steal::Empty if done.load(Ordering::Acquire) => break,
                        Steal::Empty | Steal::Retry => std::thread::yield_now(),
                    }
                });
            }
            let done = &done; let seen = &seen; let drops = &drops;
            scope.spawn(move || {
                for id in 1024..count {
                    w.push(Item { id, drops: drops.clone() });
                    if id % 5 == 0 { if let Some(task) = w.pop() { assert_eq!(seen[task.id].fetch_add(1, Ordering::SeqCst), 0); } }
                }
                while let Some(task) = w.pop() { assert_eq!(seen[task.id].fetch_add(1, Ordering::SeqCst), 0); }
                drop(w); done.store(true, Ordering::Release);
            });
        });
        assert!(seen.iter().all(|n| n.load(Ordering::SeqCst) == 1));
        assert!(drops.iter().all(|n| n.load(Ordering::SeqCst) == 1));
        let pointers = array_pointers(&s.inner); assert_alive(&pointers, true);
        drop(s); assert_alive(&pointers, false);
    }
}

#[test]
fn retained_probes_preserve_nested_epoch_state_and_batch_destination_growth() {
    let w = worker(false, true); let dest = worker(true, false);
    for i in 0..257 { w.push(i); } for i in 1000..1063 { dest.push(i); }
    let s = w.stealer(); let before = epoch::is_pinned();
    assert!(matches!(s.steal(), Steal::Success(_))); assert_eq!(epoch::is_pinned(), before);
    let pinned = epoch::pin(); assert!(epoch::is_pinned());
    assert!(matches!(s.steal(), Steal::Success(_))); assert!(epoch::is_pinned());
    assert!(matches!(s.steal_batch(&dest), Steal::Success(()))); assert!(epoch::is_pinned());
    assert!(dest.buffer.get().cap > 64);
    drop(pinned); assert_eq!(epoch::is_pinned(), before);
}

use rayon_core::{join, ThreadPoolBuilder};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
struct Ticket { index: usize, dropped: Arc<Vec<AtomicUsize>> }
impl Drop for Ticket { fn drop(&mut self) { assert_eq!(self.dropped[self.index].fetch_add(1, Ordering::SeqCst), 0); } }

#[test]
fn nested_bursts_fifo_reuse_and_pool_drop_are_exactly_once() {
    for workers in [1, 2, 8, 32] {
        let pool = ThreadPoolBuilder::new().num_threads(workers).stack_size(512 * 1024).build().unwrap();
        for _ in 0..3 {
            let count = 25_003;
            let dropped = Arc::new((0..count).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
            let executed = Arc::new((0..count).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
            pool.install(|| rayon_core::scope_fifo(|scope| {
                for index in 0..count {
                    let ticket = Ticket { index, dropped: dropped.clone() }; let done = executed.clone();
                    scope.spawn_fifo(move |_| {
                        let (left, right) = join(|| index.wrapping_mul(31), || index.wrapping_mul(17));
                        assert_eq!(left + right, index * 48);
                        assert_eq!(done[index].fetch_add(1, Ordering::SeqCst), 0);
                        drop(ticket);
                    });
                }
            }));
            assert!(executed.iter().all(|n| n.load(Ordering::SeqCst) == 1));
            assert!(dropped.iter().all(|n| n.load(Ordering::SeqCst) == 1));
        }
        drop(pool);
    }
}

#[test]
fn cross_pool_entry_panic_cleanup_and_reuse_complete() {
    let a = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let b = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    assert_eq!(a.install(|| b.install(|| join(|| 19, || 23))), (19, 23));
    let dropped = Arc::new((0..4097).map(|_| AtomicUsize::new(0)).collect::<Vec<_>>());
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| a.install(|| rayon_core::scope(|scope| {
        for index in 0..4097 {
            let ticket = Ticket { index, dropped: dropped.clone() };
            scope.spawn(move |_| { if index == 2039 { panic!("deliberate scoped panic"); } drop(ticket); });
        }
    }))));
    assert!(outcome.is_err());
    assert!(dropped.iter().all(|n| n.load(Ordering::SeqCst) == 1));
    assert_eq!(a.install(|| b.install(|| join(|| 20, || 22))), (20, 22));
}

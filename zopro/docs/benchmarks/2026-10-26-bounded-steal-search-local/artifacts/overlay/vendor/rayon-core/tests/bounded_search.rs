use rayon_core::{join, ThreadPoolBuilder};
use std::sync::atomic::{AtomicUsize, Ordering};

fn nested(count: &AtomicUsize, depth: usize) {
    if depth == 0 {
        count.fetch_add(1, Ordering::Relaxed);
    } else {
        join(|| nested(count, depth - 1), || nested(count, depth - 1));
    }
}

#[test]
fn parked_pool_bursts_and_concurrent_nested_callers_complete_exactly_once() {
    for workers in [1, 2, 8, 32] {
        let pool = ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        let count = AtomicUsize::new(0);
        for _ in 0..8 {
            std::thread::sleep(std::time::Duration::from_millis(2));
            std::thread::scope(|scope| {
                for _ in 0..4 {
                    let (pool, count) = (&pool, &count);
                    scope.spawn(move || pool.install(|| nested(count, 10)));
                }
            });
        }
        assert_eq!(count.load(Ordering::Relaxed), 8 * 4 * 1024);
        let visits: Vec<_> = (0..4097).map(|_| AtomicUsize::new(0)).collect();
        pool.scope_fifo(|scope| {
            for visit in &visits {
                scope.spawn_fifo(move |_| {
                    visit.fetch_add(1, Ordering::Relaxed);
                });
            }
        });
        assert!(visits.iter().all(|n| n.load(Ordering::Relaxed) == 1));
    }
}

#[test]
fn panic_waits_for_siblings_and_cross_pool_reentry_remains_usable() {
    let first = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    let second = ThreadPoolBuilder::new().num_threads(3).build().unwrap();
    let count = AtomicUsize::new(0);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        first.scope(|scope| {
            for i in 0..257 {
                let count = &count;
                scope.spawn(move |_| {
                    count.fetch_add(1, Ordering::Relaxed);
                    if i == 17 {
                        panic!("exercise scoped sibling completion");
                    }
                });
            }
        });
    }));
    assert!(result.is_err());
    assert_eq!(count.load(Ordering::Relaxed), 257);
    first.install(|| second.install(|| first.install(|| nested(&count, 12))));
    assert_eq!(count.load(Ordering::Relaxed), 257 + 4096);
}

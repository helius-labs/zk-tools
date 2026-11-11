use rayon::ThreadPoolBuilder;
use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicUsize, Ordering};
use zopro_parallel::{for_each_chunk_mut, join};

fn expected(index: usize) -> u64 {
    (index as u64)
        .wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .rotate_left(17)
        ^ 0xa076_1d64_78bd_642f
}

#[test]
fn chunks_cover_boundaries_exactly_once_in_real_pools() {
    for workers in [1, 2, 4, 16, 32] {
        let pool = ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        pool.install(|| {
            for len in [0, 1, 3, 31, 127, 128, 129, 1023, 1024, 1025, 8193] {
                for size in [1, 3, 128, 1024, usize::MAX] {
                    let mut values = vec![(0_usize, 0_u64); len];
                    let calls = AtomicUsize::new(0);
                    for_each_chunk_mut(&mut values, size, |start, chunk| {
                        assert_eq!(start % size, 0);
                        assert_eq!(chunk.len(), size.min(len - start));
                        calls.fetch_add(1, Ordering::Relaxed);
                        for (offset, (visits, value)) in chunk.iter_mut().enumerate() {
                            *visits += 1;
                            *value = expected(start + offset);
                        }
                    });
                    assert_eq!(calls.load(Ordering::Relaxed), len.div_ceil(size));
                    let oracle: Vec<_> = (0..len).map(|i| (1, expected(i))).collect();
                    assert_eq!(values, oracle, "workers={workers}, len={len}, size={size}");
                }
            }
        });
    }
}

#[test]
fn mutable_items_need_send_but_not_sync() {
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let mut values: Vec<_> = (0..4097).map(|_| Cell::new(0_u64)).collect();
    pool.install(|| {
        for_each_chunk_mut(&mut values, 128, |start, chunk| {
            for (offset, value) in chunk.iter_mut().enumerate() {
                value.set(expected(start + offset));
            }
        });
    });
    assert!(values
        .iter()
        .enumerate()
        .all(|(i, v)| v.get() == expected(i)));
}

#[test]
fn chunks_borrow_non_static_data_without_copies() {
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let mut source: Vec<_> = (0..1025).map(|_| 0_u64).collect();
    let mut refs: Vec<_> = source.iter_mut().collect();
    pool.install(|| {
        for_each_chunk_mut(&mut refs, 64, |start, chunk| {
            for (offset, value) in chunk.iter_mut().enumerate() {
                **value = expected(start + offset);
            }
        });
    });
    assert_eq!(source, (0..1025).map(expected).collect::<Vec<_>>());
}

#[test]
fn nested_callbacks_and_independent_callers_share_one_pool() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(512 * 1024)
        .build()
        .unwrap();
    let mut a = vec![0_u64; 8193];
    let mut b = vec![0_u64; 4101];
    pool.install(|| {
        join(
            || {
                for_each_chunk_mut(&mut a, 1024, |base, outer| {
                    for_each_chunk_mut(outer, 31, |start, inner| {
                        for (offset, value) in inner.iter_mut().enumerate() {
                            *value = expected(base + start + offset);
                        }
                    });
                });
            },
            || {
                for_each_chunk_mut(&mut b, 127, |start, chunk| {
                    let split = chunk.len() / 2;
                    let (first, second) = chunk.split_at_mut(split);
                    join(
                        || {
                            for (offset, value) in first.iter_mut().enumerate() {
                                *value = expected(start + offset);
                            }
                        },
                        || {
                            for (offset, value) in second.iter_mut().enumerate() {
                                *value = expected(start + split + offset);
                            }
                        },
                    );
                });
            },
        );
    });
    assert_eq!(a, (0..8193).map(expected).collect::<Vec<_>>());
    assert_eq!(b, (0..4101).map(expected).collect::<Vec<_>>());
}

#[test]
fn callback_panic_joins_work_and_pool_remains_usable() {
    struct Active<'a>(&'a AtomicUsize);
    impl Drop for Active<'_> {
        fn drop(&mut self) {
            self.0.fetch_sub(1, Ordering::SeqCst);
        }
    }
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let active = AtomicUsize::new(0);
    let mut values = vec![0_u64; 8193];
    let result = catch_unwind(AssertUnwindSafe(|| {
        pool.install(|| {
            for_each_chunk_mut(&mut values, 127, |start, chunk| {
                active.fetch_add(1, Ordering::SeqCst);
                let _active = Active(&active);
                assert_ne!(start, 127 * 3, "intentional callback panic");
                for (offset, value) in chunk.iter_mut().enumerate() {
                    *value = expected(start + offset);
                }
            });
        });
    }));
    assert!(result.is_err());
    assert_eq!(active.load(Ordering::SeqCst), 0);
    pool.install(|| {
        for_each_chunk_mut(&mut values, 129, |start, chunk| {
            for (offset, value) in chunk.iter_mut().enumerate() {
                *value = expected(start + offset);
            }
        });
    });
    assert_eq!(values, (0..8193).map(expected).collect::<Vec<_>>());
}

#[test]
fn zero_sized_items_preserve_logical_chunk_boundaries() {
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    let mut values = [(); 1025];
    let elements = AtomicUsize::new(0);
    pool.install(|| {
        for_each_chunk_mut(&mut values, 128, |start, chunk| {
            assert_eq!(start % 128, 0);
            elements.fetch_add(chunk.len(), Ordering::Relaxed);
        });
    });
    assert_eq!(elements.load(Ordering::Relaxed), values.len());
}

#[test]
#[should_panic(expected = "chunk_size must be nonzero")]
fn zero_chunk_size_panics_even_for_empty_input() {
    for_each_chunk_mut::<u64, _>(&mut [], 0, |_, _| unreachable!());
}

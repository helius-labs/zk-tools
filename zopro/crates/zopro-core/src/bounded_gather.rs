//! Experimental indexed gather with pool-sized task grain.
use rayon::prelude::*;

pub(super) fn gather<T: Copy + Send + Sync>(values: &[T], indices: &[u32]) -> Vec<T> {
    let tasks = rayon::current_num_threads().saturating_mul(2);
    let grain = indices.len().div_ceil(tasks).max(1);
    indices
        .par_iter()
        .with_min_len(grain)
        .map(|&index| values[index as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::ThreadPoolBuilder;

    #[test]
    fn gathers_preserve_order_duplicates_and_boundaries() {
        let values: Vec<[u64; 4]> = (0..4099_u64)
            .map(|i| [i, !i, i.wrapping_mul(7919), i.rotate_left(31)])
            .collect();
        for workers in [1, 2, 4, 16, 32] {
            let pool = ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            pool.install(|| {
                for len in [0, 1, 3, 31, 32, 33, 127, 128, 129, 1025, 65535] {
                    let indices: Vec<u32> = (0..len)
                        .map(|i| {
                            if i % 7 == 0 {
                                4098
                            } else {
                                ((i * 7919) % 4099) as u32
                            }
                        })
                        .collect();
                    let expected: Vec<_> = indices.iter().map(|&i| values[i as usize]).collect();
                    let (left, right) =
                        rayon::join(|| gather(&values, &indices), || gather(&values, &indices));
                    assert_eq!(left, expected, "workers={workers}, len={len}");
                    assert_eq!(right, expected, "workers={workers}, len={len}");
                }
            });
        }
    }

    #[test]
    fn invalid_index_panics_and_pool_remains_usable() {
        let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pool.install(|| gather(&[1_u64, 2], &[0, u32::MAX, 1]));
        }));
        assert!(result.is_err());
        assert_eq!(pool.install(|| gather(&[1_u64, 2], &[1, 0, 1])), [2, 1, 2]);
    }
}

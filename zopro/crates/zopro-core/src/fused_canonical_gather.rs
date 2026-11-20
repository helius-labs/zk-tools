//! Convert disjoint wire blocks and extract sorted query indices before joining.
//! The solver validates increasing in-range indices when loading either format.
use std::mem::MaybeUninit;
use zopro_bn254::Fr;
type Words = [u64; 4];
const LEAF: usize = 1024;

fn fill(
    wires: &mut [Fr],
    base: usize,
    a_indices: &[u32],
    a: &mut [MaybeUninit<Words>],
    b_indices: &[u32],
    b: &mut [MaybeUninit<Words>],
) {
    if wires.len() <= LEAF {
        for wire in wires.iter_mut() {
            *wire = Fr::from_montgomery(wire.to_canonical());
        }
        for (&i, out) in a_indices.iter().zip(a) {
            // The stored limbs are canonical after the conversion above.
            out.write(wires[i as usize - base].0);
        }
        for (&i, out) in b_indices.iter().zip(b) {
            out.write(wires[i as usize - base].0);
        }
        return;
    }
    let middle = wires.len() / 2;
    let boundary = base + middle;
    let cut_a = a_indices.partition_point(|&i| (i as usize) < boundary);
    let cut_b = b_indices.partition_point(|&i| (i as usize) < boundary);
    let (left_w, right_w) = wires.split_at_mut(middle);
    let (left_a, right_a) = a.split_at_mut(cut_a);
    let (left_b, right_b) = b.split_at_mut(cut_b);
    rayon::join(
        || {
            fill(
                left_w,
                base,
                &a_indices[..cut_a],
                left_a,
                &b_indices[..cut_b],
                left_b,
            )
        },
        || {
            fill(
                right_w,
                boundary,
                &a_indices[cut_a..],
                right_a,
                &b_indices[cut_b..],
                right_b,
            )
        },
    );
}

pub(super) fn prepare(wires: &mut [Fr], a: &[u32], b: &[u32]) -> (Vec<Words>, Vec<Words>) {
    let mut out_a = Vec::<Words>::with_capacity(a.len());
    let mut out_b = Vec::<Words>::with_capacity(b.len());
    fill(
        wires,
        0,
        a,
        &mut out_a.spare_capacity_mut()[..a.len()],
        b,
        &mut out_b.spare_capacity_mut()[..b.len()],
    );
    // SAFETY: matching index/output slices are split at identical boundaries.
    // Every leaf writes each element exactly once. fill joins all descendants
    // before returning. On panic neither length is increased; Words has no drop.
    unsafe {
        out_a.set_len(a.len());
        out_b.set_len(b.len());
    }
    (out_a, out_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: usize, workers: usize) {
        let values: Vec<Fr> = (0..n)
            .map(|i| match i % 5 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                _ => Fr::from((i + 19) as u64).square(),
            })
            .collect();
        let all: Vec<_> = values.iter().map(Fr::to_canonical).collect();
        let a: Vec<_> = (0..n).filter(|i| i % 3 != 0).map(|i| i as u32).collect();
        let b: Vec<_> = (0..n).filter(|i| i % 7 < 2).map(|i| i as u32).collect();
        let expected_a: Vec<_> = a.iter().map(|&i| all[i as usize]).collect();
        let expected_b: Vec<_> = b.iter().map(|&i| all[i as usize]).collect();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        pool.install(|| {
            rayon::join(
                || {
                    let mut wires = values.clone();
                    let (actual_a, actual_b) = prepare(&mut wires, &a, &b);
                    assert_eq!(actual_a, expected_a);
                    assert_eq!(actual_b, expected_b);
                    assert_eq!(wires.iter().map(|wire| wire.0).collect::<Vec<_>>(), all);
                },
                || {
                    let mut wires = values.clone();
                    let (actual, empty) = prepare(&mut wires, &b, &[]);
                    assert_eq!(actual, expected_b);
                    assert!(empty.is_empty());
                },
            )
        });
    }
    #[test]
    fn sparse_dense_boundaries_and_concurrent_calls_match_serial() {
        for workers in [1, 4, 16, 32] {
            for n in [0, 1, 1023, 1024, 1025, 4097, 32769] {
                check(n, workers);
            }
        }
    }
    #[test]
    fn invalid_index_unwinds_without_publishing_uninitialized_outputs() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pool.install(|| prepare(&mut vec![Fr::ONE; 1025], &[0, 1025], &[1, 1024]));
        }));
        assert!(result.is_err());
        assert_eq!(
            pool.install(|| prepare(&mut [Fr::ONE], &[0], &[])).0,
            vec![Fr::ONE.to_canonical()]
        );
    }
}

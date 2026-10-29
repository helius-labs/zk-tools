//! Batch-local norm inversion tree in the existing two-vector records.
use super::*;

#[inline(always)]
fn get(records: &[[FqBatch; 2]], i: usize) -> FqBatch {
    records[i / 2][i % 2]
}
#[inline(always)]
fn put(records: &mut [[FqBatch; 2]], i: usize, value: FqBatch) {
    records[i / 2][i % 2] = value;
}
fn invert(records: &mut [[FqBatch; 2]], n: usize) {
    debug_assert!(n > 0 && records.len() == n);
    for i in (1..n).rev() {
        put(
            records,
            i,
            get(records, 2 * i).mul(&get(records, 2 * i + 1)),
        );
    }
    let root_inverse = get(records, 1).inverse_nonzero();
    put(records, 1, root_inverse);
    for i in 1..n {
        let left = get(records, 2 * i);
        let right = get(records, 2 * i + 1);
        let inverse = get(records, i);
        put(records, 2 * i, inverse.mul(&right));
        put(records, 2 * i + 1, inverse.mul(&left));
    }
}
pub(super) fn apply<C: CurveConfig<Base = Fq2>>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, &Affine<C>)],
    s: &mut Fq2AddScratch,
) {
    debug_assert!(!batch.is_empty() && batch.len() % LANES == 0 && s.vector_inverse);
    let n = batch.len() / LANES;
    // Retain existing capacity. Every live tree node is written before use;
    // initialization only touches newly grown records, not every flush.
    s.prefix_records.resize(n, [FqBatch::one(); 2]);
    s.tail.clear();
    for j in 0..n {
        let k = j * LANES;
        let ax = fq2_pack(core::array::from_fn(|i| {
            buckets[(batch[k + i].0 & 0x7fffffff) as usize].x
        }));
        let bx = fq2_pack(core::array::from_fn(|i| batch[k + i].1.x));
        let dx = core::array::from_fn(|i| canonical_sub::<true>(&bx[i], &ax[i]));
        put(&mut s.prefix_records, n + j, Fq2::lanes_norm(&dx));
    }
    invert(&mut s.prefix_records, n);
    for j in (0..n).rev() {
        let k = j * LANES;
        let a: [Affine<C>; LANES] =
            core::array::from_fn(|i| buckets[(batch[k + i].0 & 0x7fffffff) as usize]);
        let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = fq2_pack(a.map(|p| p.x));
        let ay = fq2_pack(a.map(|p| p.y));
        let bx = fq2_pack(b.map(|p| p.x));
        let by = fq2_pack(core::array::from_fn(|i| {
            if batch[k + i].0 >> 31 != 0 {
                -b[i].y
            } else {
                b[i].y
            }
        }));
        let dx = core::array::from_fn(|i| canonical_sub::<true>(&bx[i], &ax[i]));
        let dy = core::array::from_fn(|i| canonical_sub::<true>(&by[i], &ay[i]));
        let factor = get(&s.prefix_records, n + j);
        let slope = Fq2::lanes_scaled_numerator(&dy, &dx, &factor);
        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &Fq2::lanes_add(&ax, &bx));
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);
        let (x, y) = (fq2_unpack(x), fq2_unpack(y));
        for i in 0..LANES {
            buckets[(batch[k + i].0 & 0x7fffffff) as usize] = Affine::new(x[i], y[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{Fr, G2Config};

    #[test]
    fn arbitrary_heap_lengths_match_scalar_inverses_and_reuse() {
        let mut records = Vec::new();
        let sizes: Vec<_> = (1..=65).chain([127, 128, 129, 256, 25, 44, 1]).collect();
        for n in sizes {
            records.resize(n, [FqBatch::one(); 2]);
            let values: Vec<[Fq; LANES]> = (0..n)
                .map(|j| {
                    core::array::from_fn(|i| {
                        let x = Fq::from((j * LANES + i + 1) as u64);
                        match (i + j) % 4 {
                            0 => -x,
                            1 => x.square() + Fq::ONE,
                            _ => x,
                        }
                    })
                })
                .collect();
            for (j, v) in values.iter().enumerate() {
                assert!(v.iter().all(|x| !x.is_zero()));
                put(&mut records, n + j, FqBatch::new(v));
            }
            let capacity = records.capacity();
            invert(&mut records, n);
            assert_eq!(records.capacity(), capacity);
            for (j, v) in values.iter().enumerate() {
                let inv = get(&records, n + j).get();
                for i in 0..LANES {
                    assert_eq!(inv[i], v[i].inverse().unwrap(), "n={n},j={j},i={i}");
                    assert_eq!(inv[i] * v[i], Fq::ONE);
                }
            }
        }
    }

    #[test]
    fn signed_complete_batches_tails_disabled_and_scratch_reuse() {
        let g = Projective::<G2Config>::generator();
        let max_n = 1016usize;
        let size = max_n.next_power_of_two();
        let initial_scalars: Vec<_> = (0..size).map(|i| Fr::from((12345 + i) as u64)).collect();
        let incoming_scalars: Vec<_> = (0..max_n).map(|i| Fr::from((900001 + i) as u64)).collect();
        let initial = Projective::<G2Config>::batch_to_affine(
            &initial_scalars.iter().map(|s| g * *s).collect::<Vec<_>>(),
        );
        let incoming = Projective::<G2Config>::batch_to_affine(
            &incoming_scalars.iter().map(|s| g * *s).collect::<Vec<_>>(),
        );
        let mut scratch = Fq2AddScratch {
            canonical: true,
            vector_inverse: true,
            ..Default::default()
        };
        for n in [
            0, 1, 7, 8, 9, 16, 24, 40, 56, 72, 200, 352, 520, 1016, 24, 8,
        ] {
            for mode in 0..3 {
                let batch: Vec<_> = incoming[..n]
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        (
                            (i * 37 % size) as u32
                                | if mode == 1 || mode == 2 && i % 3 == 0 {
                                    1 << 31
                                } else {
                                    0
                                },
                            p,
                        )
                    })
                    .collect();
                let mut actual = initial.clone();
                let mut scalars = initial_scalars.clone();
                for vector_inverse in [true, false, true] {
                    scratch.vector_inverse = vector_inverse;
                    for (i, &(id, p)) in batch.iter().enumerate() {
                        let index = (id & 0x7fffffff) as usize;
                        assert_ne!(actual[index].x, p.x);
                        scalars[index] += if id >> 31 != 0 {
                            -incoming_scalars[i]
                        } else {
                            incoming_scalars[i]
                        };
                    }
                    super::super::apply::<G2Config, true>(&mut actual, &batch, &mut scratch);
                    let expected = Projective::<G2Config>::batch_to_affine(
                        &scalars.iter().map(|s| g * *s).collect::<Vec<_>>(),
                    );
                    assert_eq!(
                        actual, expected,
                        "n={n},mode={mode},vector={vector_inverse}"
                    );
                }
            }
        }
    }
}

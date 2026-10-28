//! Consume final derivative columns before packed writeback.
use super::*;

pub(super) fn enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("ZOPRO_CPU_EULER_FUSED_COLUMNS").is_ok_and(|v| v == "1"))
}

fn output_chunks(a: &mut [Fr], columns: usize) -> impl IndexedParallelIterator<Item = &mut [Fr]> {
    let padding = columns.checked_sub(a.len().div_ceil(8)).unwrap();
    a.par_chunks_mut(8)
        .chain((0..padding).into_par_iter().map(|_| &mut [][..]))
}

// Keep both 32-Pair columns outside recursive Rayon iterator frames.
#[inline(never)]
fn consume_columns(
    outputs: [&mut [Fr]; 32],
    b: &[Fr],
    values: &[PackedPair],
    roots: &Twiddles,
    q: usize,
    column: usize,
) {
    let mut left = core::array::from_fn(|row| values[row * q + column].load());
    let mut right = core::array::from_fn(|row| values[row * q + column + 1].load());
    if roots.columns.is_empty() {
        column_kernel::<false, true>(&mut left, roots, 0, q, column);
        column_kernel::<false, true>(&mut right, roots, 0, q, column + 1);
    } else {
        column_kernel::<true, true>(&mut left, roots, 0, q, column);
        column_kernel::<true, true>(&mut right, roots, 0, q, column + 1);
    }
    for (row, out) in outputs.into_iter().enumerate() {
        if out.is_empty() {
            continue;
        }
        let (da0, db0) = left[row].halves();
        let (da1, db1) = right[row].halves();
        let da = Pair::from_halves(&da0, &da1);
        let db = Pair::from_halves(&db0, &db1);
        let aa = Pair::new(&core::array::from_fn(|j| {
            out.get(j).copied().unwrap_or(Fr::ZERO)
        }));
        let bb = Pair::new(&core::array::from_fn(|j| {
            b.get(4 * (row * q + column) + j)
                .copied()
                .unwrap_or(Fr::ZERO)
        }));
        let product = da.sum2(&bb, &aa, &db).get();
        out.copy_from_slice(&product[..out.len()]);
    }
}

pub(super) fn transform_output(
    a: &mut [Fr],
    b: &[Fr],
    values: &mut [PackedPair],
    roots: &Twiddles,
) {
    assert!(values.len() > 1024 && values.len().is_power_of_two());
    assert!(a.len() <= 4 * values.len() && b.len() <= 4 * values.len());
    if a.is_empty() {
        return;
    }
    let q = values.len() / 32;
    let cutoff = leaf::cutoff(values.len(), rayon::current_num_threads());
    values
        .par_chunks_mut(q)
        .for_each(|part| stored_transform_inner::<PackedPair, true>(part, roots, 5, cutoff));
    let mut remaining = a;
    let stripes: [&mut [Fr]; 32] = core::array::from_fn(|_| {
        let rest = std::mem::take(&mut remaining);
        let len = rest.len().min(4 * q);
        let (stripe, rest) = rest.split_at_mut(len);
        remaining = rest;
        stripe
    });
    debug_assert!(remaining.is_empty());
    // The generated balanced zip below borrows every disjoint stripe once.
    let [s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15, s16, s17, s18, s19, s20, s21, s22, s23, s24, s25, s26, s27, s28, s29, s30, s31] =
        stripes;
    output_chunks(s0, q / 2)
        .zip(output_chunks(s1, q / 2))
        .zip(output_chunks(s2, q / 2).zip(output_chunks(s3, q / 2)))
        .zip(
            output_chunks(s4, q / 2)
                .zip(output_chunks(s5, q / 2))
                .zip(output_chunks(s6, q / 2).zip(output_chunks(s7, q / 2))),
        )
        .zip(
            output_chunks(s8, q / 2)
                .zip(output_chunks(s9, q / 2))
                .zip(output_chunks(s10, q / 2).zip(output_chunks(s11, q / 2)))
                .zip(
                    output_chunks(s12, q / 2)
                        .zip(output_chunks(s13, q / 2))
                        .zip(output_chunks(s14, q / 2).zip(output_chunks(s15, q / 2))),
                ),
        )
        .zip(
            output_chunks(s16, q / 2)
                .zip(output_chunks(s17, q / 2))
                .zip(output_chunks(s18, q / 2).zip(output_chunks(s19, q / 2)))
                .zip(
                    output_chunks(s20, q / 2)
                        .zip(output_chunks(s21, q / 2))
                        .zip(output_chunks(s22, q / 2).zip(output_chunks(s23, q / 2))),
                )
                .zip(
                    output_chunks(s24, q / 2)
                        .zip(output_chunks(s25, q / 2))
                        .zip(output_chunks(s26, q / 2).zip(output_chunks(s27, q / 2)))
                        .zip(
                            output_chunks(s28, q / 2)
                                .zip(output_chunks(s29, q / 2))
                                .zip(output_chunks(s30, q / 2).zip(output_chunks(s31, q / 2))),
                        ),
                ),
        )
        .enumerate()
        .for_each(
            |(
                column,
                (
                    (
                        (((o0, o1), (o2, o3)), ((o4, o5), (o6, o7))),
                        (((o8, o9), (o10, o11)), ((o12, o13), (o14, o15))),
                    ),
                    (
                        (((o16, o17), (o18, o19)), ((o20, o21), (o22, o23))),
                        (((o24, o25), (o26, o27)), ((o28, o29), (o30, o31))),
                    ),
                ),
            )| {
                consume_columns(
                    [
                        o0, o1, o2, o3, o4, o5, o6, o7, o8, o9, o10, o11, o12, o13, o14, o15, o16,
                        o17, o18, o19, o20, o21, o22, o23, o24, o25, o26, o27, o28, o29, o30, o31,
                    ],
                    b,
                    values,
                    roots,
                    q,
                    2 * column,
                )
            },
        );
}

#[cfg(test)]
mod tests {
    use super::*;
    fn random(seed: &mut u64) -> Fr {
        let mut b = [0u8; 64];
        for word in b.chunks_mut(8) {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            word.copy_from_slice(&seed.to_le_bytes());
        }
        Fr::from_random_bytes(&b)
    }
    fn roots(n: usize, cached: bool) -> Twiddles {
        let root = Fr::root_of_unity(n).unwrap().pow(&[3]);
        let mut r = Fr::ONE;
        let mut roots = Twiddles::new(
            (0..n / 2)
                .map(|_| {
                    let v = r;
                    r *= root;
                    v
                })
                .collect(),
        );
        if cached {
            roots.prepare_columns();
            assert!(!roots.columns.is_empty());
        }
        roots
    }
    fn check(n: usize, count: usize, nb: usize, roots: &Twiddles, salt: u64) {
        let mut seed = salt + 19;
        let mut values: Vec<_> = (0..n / 4)
            .map(|_| PackedPair::store(Pair::new(&core::array::from_fn(|_| random(&mut seed)))))
            .collect();
        let mut reference = values.clone();
        let mut a: Vec<_> = (0..count)
            .map(|i| {
                if i % 13 == 0 {
                    Fr::ZERO
                } else {
                    random(&mut seed)
                }
            })
            .collect();
        let b: Vec<_> = (0..nb)
            .map(|i| {
                if i % 7 == 0 {
                    -Fr::ONE
                } else {
                    random(&mut seed)
                }
            })
            .collect();
        stored_transform::<PackedPair, true>(&mut reference, roots, 0);
        let expected: Vec<_> = a
            .iter()
            .enumerate()
            .map(|(i, &a)| {
                let derivatives = reference[i / 4].load().get();
                derivatives[i % 4] * b.get(i).copied().unwrap_or(Fr::ZERO)
                    + a * derivatives[4 + i % 4]
            })
            .collect();
        let cap = a.capacity();
        transform_output(&mut a, &b, &mut values, roots);
        assert_eq!(a.len(), count);
        assert_eq!(a.capacity(), cap);
        assert_eq!(a, expected, "n={n} count={count} nb={nb} salt={salt}");
    }
    #[test]
    fn fused_columns_match_scalar_products_for_prefixes_and_cached_roots() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        pool.install(|| {
            for n in [8192usize, 16384, 32768, 131072] {
                for cached in [false, true] {
                    let roots = roots(n, cached);
                    let stripe = n / 32;
                    for count in [0, 1, 7, 8, 9, stripe - 1, stripe, stripe + 3, n - 7, n] {
                        for nb in [0, count / 3, n] {
                            check(
                                n,
                                count,
                                nb,
                                &roots,
                                0x45554c4552 + count as u64 + nb as u64,
                            );
                        }
                    }
                }
            }
        });
    }
    #[test]
    fn padded_output_iterators_preserve_each_existing_row_once() {
        for columns in [1usize, 2, 16, 128] {
            for n in 0..=columns * 8 {
                let mut values = vec![Fr::ZERO; n];
                output_chunks(&mut values, columns)
                    .enumerate()
                    .for_each(|(i, out)| {
                        for (j, value) in out.iter_mut().enumerate() {
                            *value = Fr::from((8 * i + j + 1) as u64)
                        }
                    });
                assert_eq!(
                    values,
                    (1..=n).map(|i| Fr::from(i as u64)).collect::<Vec<_>>()
                );
            }
        }
    }
    #[test]
    fn fused_columns_share_roots_on_small_worker_stacks() {
        let roots = roots(32768, true);
        for workers in [1, 2, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            std::thread::scope(|scope| {
                let a = scope.spawn(|| pool.install(|| check(32768, 30922, 30089, &roots, 11)));
                let b = scope.spawn(|| pool.install(|| check(32768, 32761, 32768, &roots, 37)));
                a.join().unwrap();
                b.join().unwrap();
            });
        }
    }
}

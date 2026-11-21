//! Prefix-local arithmetic; final coordinates and inverse-tree multiplies stay ordinary.
use super::*;

#[inline(always)]
pub(super) fn prefix_mul(a: &FqBatch, b: &FqBatch) -> FqBatch {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        a.mul_div16(b)
    }
    #[cfg(not(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    {
        a.mul(b)
    }
}
#[inline(always)]
pub(super) fn prefix_norm(dx: &[FqBatch; 2]) -> FqBatch {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        dx[0].sum2_div16(&dx[0], &dx[1], &dx[1])
    }
    #[cfg(not(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    {
        Fq2::lanes_norm(dx)
    }
}
#[inline(always)]
pub(super) fn prefix_numerator(
    dy: &[FqBatch; 2],
    dx: &[FqBatch; 2],
    factor: &FqBatch,
) -> [FqBatch; 2] {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        [
            dy[0].sum2_div16(&dx[0], &dy[1], &dx[1]).mul_div16(factor),
            dy[1]
                .difference2_div16(&dx[0], &dy[0], &dx[1])
                .mul_div16(factor),
        ]
    }
    #[cfg(not(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    {
        Fq2::lanes_scaled_numerator(dy, dx, factor)
    }
}
#[inline(always)]
pub(super) fn prefix_inverse(
    chains: &FqBatch,
    complete: bool,
    vector_inverse: bool,
) -> Option<FqBatch> {
    #[cfg(all(
        all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ),
        zopro_direct_chain_inverse
    ))]
    if complete && vector_inverse {
        return Some(chains.inverse_nonzero_scaled16());
    }
    direct_chain_inverse(chains, complete, vector_inverse)
}
#[inline(always)]
pub(super) fn prefix_tail_inverse(inverse: FqBatch, full: usize) -> FqBatch {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        if full > 0 {
            let a = inverse.add(&inverse);
            let a = a.add(&a);
            let a = a.add(&a);
            return a.add(&a);
        }
    }
    let _ = full;
    inverse
}

#[cfg(all(
    test,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod tests {
    use super::*;
    use zopro_bn254::{G1Config, G2Config};
    fn points<C: CurveConfig>(n: usize, offset: u64) -> Vec<Affine<C>> {
        let g = Projective::<C>::generator();
        Projective::<C>::batch_to_affine(
            &(0..n)
                .map(|i| g * Fr::from(offset + 113 * i as u64))
                .collect::<Vec<_>>(),
        )
    }
    #[test]
    fn uniform_prefix_g1_signed_tail_inverse_paths() {
        let initial = points::<G1Config>(64, 87123);
        let incoming = points::<G1Config>(64, 392951);
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64] {
            for vector_inverse in [false, true] {
                for canonical in [false, true] {
                    for write_groups in [false, true] {
                        for mode in 0..3 {
                            let batch = (0..n)
                                .map(|i| {
                                    (
                                        (i * 37 % 64) as u32
                                            | if mode == 1 || mode == 2 && i % 3 == 0 {
                                                1 << 31
                                            } else {
                                                0
                                            },
                                        &incoming[i],
                                    )
                                })
                                .collect::<Vec<_>>();
                            let mut actual = initial.clone();
                            let mut expected = initial.clone();
                            let mut scratch = FqAddScratch {
                                canonical,
                                vector_inverse,
                                write_groups,
                                reserve_groups: true,
                                ..Default::default()
                            };
                            for _ in 0..3 {
                                for &(encoded, b) in &batch {
                                    let id = (encoded & 0x7fffffff) as usize;
                                    let b = if encoded >> 31 != 0 { -*b } else { *b };
                                    assert_ne!(expected[id].x, b.x);
                                    expected[id] = (expected[id] + b).to_affine();
                                }
                                if canonical {
                                    stream_g1_impl::<G1Config, true, true>(
                                        &mut actual,
                                        &batch,
                                        &mut scratch,
                                    )
                                } else {
                                    stream_g1_impl::<G1Config, false, true>(
                                        &mut actual,
                                        &batch,
                                        &mut scratch,
                                    )
                                }
                                assert_eq!(actual,expected,"n={n} vi={vector_inverse} canonical={canonical} write={write_groups} mode={mode}");
                                assert_eq!(scratch.groups.len(), n / LANES);
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn uniform_prefix_borrowed_g2_tail_inverse_paths() {
        let initial = points::<G2Config>(64, 145971);
        let incoming = points::<G2Config>(64, 731259);
        let saved = incoming.clone();
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64] {
            for vector_inverse in [false, true] {
                for canonical in [false, true] {
                    for mode in 0..3 {
                        let batch = (0..n)
                            .map(|i| {
                                (
                                    (i * 37 % 64) as u32
                                        | if mode == 1 || mode == 2 && i % 3 == 0 {
                                            1 << 31
                                        } else {
                                            0
                                        },
                                    &incoming[i],
                                )
                            })
                            .collect::<Vec<_>>();
                        let mut actual = initial.clone();
                        let mut expected = initial.clone();
                        let mut scratch = Fq2AddScratch {
                            canonical,
                            vector_inverse,
                            ..Default::default()
                        };
                        for _ in 0..3 {
                            for &(encoded, b) in &batch {
                                let id = (encoded & 0x7fffffff) as usize;
                                let b = if encoded >> 31 != 0 { -*b } else { *b };
                                assert_ne!(expected[id].x, b.x);
                                expected[id] = (expected[id] + b).to_affine();
                            }
                            if canonical {
                                g2_borrowed::apply::<G2Config, true>(
                                    &mut actual,
                                    &batch,
                                    &mut scratch,
                                )
                            } else {
                                g2_borrowed::apply::<G2Config, false>(
                                    &mut actual,
                                    &batch,
                                    &mut scratch,
                                )
                            }
                            assert_eq!(
                                actual, expected,
                                "n={n} vi={vector_inverse} canonical={canonical} mode={mode}"
                            );
                            assert_eq!(scratch.prefix_records.len(), n / LANES);
                            assert_eq!(scratch.groups.capacity(), 0);
                        }
                    }
                }
            }
        }
        assert_eq!(incoming, saved);
    }
}

#[cfg(test)]
mod contained_tests {
    use super::*;
    use crate::testing::Seeded;
    use zopro_bn254::{Fr, G2Affine, G2Config, G2Projective};
    #[test]
    fn contained_indexed_buckets_match_generator_and_keep_inputs() {
        let g = G2Projective::generator();
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            pool.install(|| {
                for n in [512, 513, 1024, 1025, 4097] {
                    let weights = (0..n)
                        .map(|i| match i % 13 {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => -Fr::ONE,
                            _ => Fr::from(i as u64 + 1),
                        })
                        .collect::<Vec<_>>();
                    let points = G2Projective::batch_to_affine(
                        &weights.iter().map(|w| g * *w).collect::<Vec<_>>(),
                    );
                    let saved = points.clone();
                    for skew in [false, true] {
                        let mut bins = vec![vec![]; 128];
                        let mut expected = vec![Fr::ZERO; 128];
                        for i in 0..n {
                            let b = if skew && i < n - 7.min(n) {
                                0
                            } else {
                                (i * 19) % 128
                            };
                            bins[b].push(i as u32);
                            expected[b] += weights[i];
                        }
                        let mut order = vec![];
                        let mut starts = vec![0];
                        for b in bins {
                            order.extend(b);
                            starts.push(order.len());
                        }
                        let actual = try_reduce::<G2Config>(&points, &order, &starts)
                            .expect("contained native layout");
                        for b in 0..128 {
                            assert_eq!(actual[b], (g * expected[b]).to_affine());
                        }
                        assert_eq!(points, saved);
                    }
                }
            });
        }
    }
    #[test]
    fn contained_complete_queries_groups_boundaries_and_explicit_budget_fallback() {
        let g = G2Projective::generator();
        let n = 2051;
        let weights = (0..n)
            .map(|i| {
                if i % 17 == 0 {
                    Fr::ZERO
                } else {
                    Fr::from(i as u64 + 37)
                }
            })
            .collect::<Vec<_>>();
        let points =
            G2Projective::batch_to_affine(&weights.iter().map(|w| g * *w).collect::<Vec<_>>());
        let saved = points.clone();
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            pool.install(|| {
                let mut rng = Seeded::new(0x636f6e7461696e65);
                for kind in 0..4 {
                    let values = (0..n)
                        .map(|i| match kind {
                            0 => rng.fr(),
                            1 => Fr::ONE,
                            2 => {
                                if i % 2 == 0 {
                                    -Fr::ONE
                                } else {
                                    Fr::ZERO
                                }
                            }
                            _ => {
                                Fr::from(2u64).pow(&[[11, 63, 119, 120, 125, 126, 253][i % 7]])
                                    - Fr::ONE
                            }
                        })
                        .collect::<Vec<_>>();
                    let expected = g * weights
                        .iter()
                        .zip(&values)
                        .fold(Fr::ZERO, |s, (w, v)| s + *w * *v);
                    let original = values.iter().map(Fr::to_canonical).collect::<Vec<_>>();
                    for on in [false, true] {
                        let mut w = original.clone();
                        let point = if on {
                            crate::msm_affine::g2_query_contained_index(&points, &mut w).0
                        } else {
                            crate::msm_affine::g2_query_exclusive(&points, &mut w)
                        };
                        assert_eq!(point.to_affine(), expected.to_affine());
                        assert_eq!(points, saved);
                    }
                    let before = std::env::var_os("ZOPRO_CPU_PREPARE_MAX_BYTES");
                    std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", "0");
                    let mut w = original.clone();
                    let (point, hits) =
                        crate::msm_affine::g2_query_contained_index(&points, &mut w);
                    if let Some(s) = before {
                        std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", s);
                    } else {
                        std::env::remove_var("ZOPRO_CPU_PREPARE_MAX_BYTES");
                    }
                    assert_eq!(point.to_affine(), expected.to_affine());
                    assert_eq!(hits, 0);
                    assert_eq!(points, saved);
                }
            });
        }
    }
    #[test]
    fn contained_full_layout_fits_original_for_every_supported_half_length() {
        for half in 512..=32768 {
            assert!(
                crate::msm_affine::workspace::contained_indexed_fits::<G2Config>(half).unwrap(),
                "half={half}"
            );
        }
        for n in [0, 1, 511, 32769, usize::MAX] {
            assert!(
                !crate::msm_affine::workspace::contained_indexed_fits::<G2Config>(n)
                    .unwrap_or(false)
            );
        }
        let points = vec![G2Affine::GENERATOR; 512];
        let before = points.clone();
        assert!(try_reduce::<G2Config>(&points, &[], &[0]).is_none());
        assert!(try_reduce::<G2Config>(&points, &[0], &[0, 1]).is_none());
        assert_eq!(points, before);
    }
}

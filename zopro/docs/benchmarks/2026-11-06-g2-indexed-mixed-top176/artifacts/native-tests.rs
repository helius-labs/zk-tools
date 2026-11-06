#[cfg(test)]
mod indexed_mixed_tests {
    use super::*;
    use crate::testing::Seeded;
    use zopro_bn254::{Fr, G2Affine, G2Projective};

    fn fixture(n: usize) -> (Vec<G2Affine>, Vec<Fr>) {
        let g = G2Projective::generator();
        let weights = (0..n)
            .map(|i| match i % 19 {
                0 => Fr::ZERO,
                1 | 2 => Fr::ONE,
                3 => -Fr::ONE,
                _ => Fr::from(i as u64 + 71),
            })
            .collect::<Vec<_>>();
        let points =
            G2Projective::batch_to_affine(&weights.iter().map(|w| g * *w).collect::<Vec<_>>());
        (points, weights)
    }

    #[test]
    fn mixed_top_raw_matches_generator_for_signs_boundaries_and_tails() {
        let mut rng = Seeded::new(0x6d697865645f746f);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        pool.install(|| {
            for n in [1024, 1025, 2051, 4097] {
                let (points, weights) = fixture(n);
                let saved = points.clone();
                for kind in 0..4 {
                    let values = (0..n)
                        .map(|i| match kind {
                            0 => rng.fr(),
                            1 => {
                                if i % 3 == 0 {
                                    -Fr::ONE
                                } else {
                                    Fr::ZERO
                                }
                            }
                            2 => {
                                Fr::from(2u64).pow(&[[
                                    10, 11, 12, 63, 95, 96, 106, 107, 117, 118, 125, 126, 253,
                                ][i % 13]])
                                    - Fr::ONE
                            }
                            _ => -Fr::from(2u64).pow(&[[11, 64, 97, 108, 119, 127, 251][i % 7]]),
                        })
                        .collect::<Vec<_>>();
                    let expected = G2Projective::generator()
                        * weights
                            .iter()
                            .zip(&values)
                            .fold(Fr::ZERO, |s, (w, v)| s + *w * *v);
                    let mut words = values.iter().map(Fr::to_canonical).collect::<Vec<_>>();
                    let (actual, used) = raw_exclusive(&points, &mut words);
                    assert!(used);
                    assert_eq!(
                        actual.to_affine(),
                        expected.to_affine(),
                        "n={n} kind={kind}"
                    );
                    assert_eq!(points, saved);
                }
            }
        });
    }

    #[test]
    fn complete_query_grouping_fallbacks_and_threaded_stacks_match_generator() {
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            pool.install(|| {
                let (points, weights) = fixture(2051);
                let saved = points.clone();
                let mut rng = Seeded::new(0x636f6d706f6e656e);
                for kind in 0..3 {
                    let values = (0..points.len())
                        .map(|i| match kind {
                            0 => rng.fr(),
                            1 => Fr::ONE,
                            _ => Fr::from((i % 7) as u64),
                        })
                        .collect::<Vec<_>>();
                    let expected = G2Projective::generator()
                        * weights
                            .iter()
                            .zip(&values)
                            .fold(Fr::ZERO, |s, (w, v)| s + *w * *v);
                    let original = values.iter().map(Fr::to_canonical).collect::<Vec<_>>();
                    for on in [false, true, false, true] {
                        let mut words = original.clone();
                        let (point, used) = if on {
                            crate::msm_affine::g2_query_indexed_mixed(&points, &mut words)
                        } else {
                            (
                                crate::msm_affine::g2_query_exclusive(&points, &mut words),
                                false,
                            )
                        };
                        assert_eq!(point.to_affine(), expected.to_affine());
                        assert_eq!(used, on && kind == 0);
                        assert_eq!(points, saved);
                    }
                }
                // Unsupported sizes take the original path before mutation.
                for n in [0, 7, 1023] {
                    let mut words = vec![Fr::ONE.to_canonical(); n];
                    let (_, used) = raw_exclusive(&points[..n], &mut words);
                    assert!(!used);
                }
            });
        }
    }

    #[test]
    fn mixed_top_complete_workspace_fits_original_forecast_for_all_eligible_lengths() {
        for n in 1024..65536 {
            assert!(
                crate::msm_affine::workspace::indexed_mixed_fits(n).unwrap(),
                "n={n}"
            );
        }
    }
}

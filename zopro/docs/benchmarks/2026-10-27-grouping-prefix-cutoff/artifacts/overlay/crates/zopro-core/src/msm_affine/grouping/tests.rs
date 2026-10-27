use super::*;
use crate::msm::BatchInverse;
use crate::msm_affine::{msm_grouped, msm_raw};
use zopro_bn254::{CurveConfig, Field, G1Config, G2Config, Projective};
#[test]
fn sample_keeps_repeated_and_dense_zero_inputs() {
    let mut cases = 0;
    for n in [
        256usize, 1025, 2048, 2049, 32768, 53874, 65536, 98674, 189618,
    ] {
        for kind in 0..5 {
            let words: Vec<_> = (0..n)
                .map(|i| match kind {
                    0 => [i as u64 + 1, 0, 0, 0],
                    1 => {
                        if i % 50 == 0 {
                            [0; 4]
                        } else {
                            [i as u64 + 1, 0, 0, 0]
                        }
                    }
                    2 => {
                        if i % 2 == 0 {
                            [0; 4]
                        } else {
                            [i as u64 + 1, 0, 0, 0]
                        }
                    }
                    3 => [0; 4],
                    _ => [(i % 17) as u64 + 1, 0, 0, 0],
                })
                .collect();
            assert_eq!(
                grouping_sample(&words, true).is_some(),
                kind >= 2,
                "n={n} kind={kind}"
            );
            if kind == 1 {
                assert!(grouping_sample(&words, false).is_some());
            }
            cases += 1;
        }
    }
    // Exact full-sample boundary: 20% zeros meets the existing 80%
    // nonzero-cardinality cutoff; 19% cannot meet it.
    for zeros in [0usize, 2, 19, 190, 199, 200, 201, 500, 1000] {
        let words: Vec<_> = (0..1000)
            .map(|i| {
                if i < zeros {
                    [0; 4]
                } else {
                    [i as u64 + 1, 0, 0, 0]
                }
            })
            .collect();
        assert_eq!(
            grouping_sample(&words, true).is_some(),
            zeros >= 200,
            "zeros={zeros}"
        );
        cases += 1;
    }
    // These entirely unique, partial-stride samples used to admit an
    // expensive map that could only be rejected afterwards.
    for n in [53874usize, 98674] {
        let words: Vec<_> = (0..n).map(|i| [i as u64 + 1, 0, 0, 0]).collect();
        assert!(grouping_sample(&words, false).is_some());
        assert!(grouping_sample(&words, true).is_none());
        cases += 1;
    }
    assert_eq!(cases, 56);
    println!("ZERO_AWARE_ADMISSION_CASES={cases}");
}
#[test]
fn policies_match_independent_weighted_generator_sum() {
    fn check<C: CurveConfig>() -> usize
    where
        C::Base: BatchInverse,
    {
        let mut cases = 0;
        for n in [256usize, 1025] {
            let generator = Projective::<C>::generator();
            let points: Vec<_> = (0..n).map(|i| generator * Fr::from(i as u64 + 1)).collect();
            let bases = Projective::<C>::batch_to_affine(&points);
            for kind in 0..5 {
                let scalars: Vec<_> = (0..n)
                    .map(|i| match kind {
                        0 => Fr::ZERO,
                        1 => Fr::from((i % 17) as u64 + 1),
                        2 => {
                            if i % 2 == 0 {
                                Fr::ZERO
                            } else {
                                Fr::from(i as u64 + 13).pow(&[11])
                            }
                        }
                        3 => {
                            if i % 50 == 0 {
                                Fr::ZERO
                            } else {
                                Fr::from(i as u64 + 13).pow(&[11])
                            }
                        }
                        _ => Fr::from(i as u64 + 13).pow(&[11]),
                    })
                    .collect();
                let sum = scalars
                    .iter()
                    .enumerate()
                    .fold(Fr::ZERO, |a, (i, &v)| a + v * Fr::from(i as u64 + 1));
                let expected = (generator * sum).to_affine();
                let words: Vec<_> = scalars.iter().map(Fr::to_canonical).collect();
                for policy in [false, true] {
                    let actual = scalar_groups_policy(&words, policy)
                        .map_or_else(|| msm_raw(&bases, &words), |g| msm_grouped(&bases, &g));
                    assert_eq!(
                        actual.to_affine(),
                        expected,
                        "n={n} kind={kind} policy={policy}"
                    );
                    cases += 1;
                }
            }
        }
        cases
    }
    let cases = check::<G1Config>() + check::<G2Config>();
    assert_eq!(cases, 40);
    println!("ZERO_AWARE_MSM_IDENTITIES={cases}");
}

#[test]
fn stable_groups_match_independent_ordered_value_index() {
    use std::collections::{BTreeMap, BTreeSet};
    for n in [0usize, 1, 255] {
        assert!(scalar_groups_policy(&vec![[0; 4]; n], true).is_none());
    }
    for n in [256usize, 1025, 4097] {
        for kind in 0..5 {
            let words: Vec<_> = (0..n)
                .map(|i| {
                    let value = match kind {
                        0 => Fr::ZERO,
                        1 => Fr::ONE,
                        2 => {
                            if i % 7 == 0 {
                                Fr::ZERO
                            } else {
                                let v = Fr::from((i % 17 + 3) as u64).pow(&[91]);
                                if i % 2 == 0 {
                                    -v
                                } else {
                                    v
                                }
                            }
                        }
                        _ => {
                            let unique = n * 80 / 100 + usize::from(kind == 4);
                            Fr::from((if i < unique { i + 1 } else { 1 }) as u64)
                        }
                    };
                    value.to_canonical()
                })
                .collect();
            let mut oracle = BTreeMap::<[u64; 4], Vec<u32>>::new();
            let mut first = Vec::new();
            for (index, &value) in words.iter().enumerate() {
                if value == [0; 4] {
                    continue;
                }
                if !oracle.contains_key(&value) {
                    first.push(value);
                }
                oracle.entry(value).or_default().push(index as u32);
            }
            for policy in [false, true] {
                let actual = scalar_groups_policy(&words, policy);
                assert_eq!(
                    actual.is_none(),
                    oracle.len() * 100 > n * 80,
                    "n={n} kind={kind} policy={policy}"
                );
                if let Some(group) = actual {
                    assert_eq!(group.len, n);
                    assert_eq!(group.values, first);
                    assert_eq!(group.starts.len(), group.values.len() + 1);
                    assert_eq!(group.starts[0], 0);
                    let mut indices = BTreeSet::new();
                    for (class, value) in group.values.iter().enumerate() {
                        let actual = &group.order[group.starts[class]..group.starts[class + 1]];
                        assert_eq!(actual, &oracle[value]);
                        assert_eq!(group.counts[class], oracle[value].len());
                        assert!(actual.iter().all(|i| indices.insert(*i)));
                    }
                    assert_eq!(
                        group.order.len(),
                        oracle.values().map(Vec::len).sum::<usize>()
                    );
                    assert_eq!(*group.starts.last().unwrap(), group.order.len());
                }
            }
        }
    }
}

fn threaded_fixture<C: CurveConfig>() -> (
    Vec<zopro_bn254::Affine<C>>,
    Vec<[u64; 4]>,
    zopro_bn254::Affine<C>,
)
where
    C::Base: BatchInverse,
{
    let n = 1025;
    let generator = Projective::<C>::generator();
    let points: Vec<_> = (0..n)
        .map(|i| {
            if i % 19 == 0 {
                Projective::<C>::IDENTITY
            } else {
                generator * Fr::from(i as u64 + 1)
            }
        })
        .collect();
    let scalars: Vec<_> = (0..n)
        .map(|i| match i % 5 {
            0 => Fr::ZERO,
            1 => Fr::ONE,
            2 => -Fr::ONE,
            _ => Fr::from((i % 23 + 3) as u64).pow(&[91]),
        })
        .collect();
    let total = scalars.iter().enumerate().fold(Fr::ZERO, |sum, (i, s)| {
        if i % 19 == 0 {
            sum
        } else {
            sum + *s * Fr::from(i as u64 + 1)
        }
    });
    (
        Projective::<C>::batch_to_affine(&points),
        scalars.iter().map(Fr::to_canonical).collect(),
        (generator * total).to_affine(),
    )
}

fn grouped_result<C: CurveConfig>(
    bases: &[zopro_bn254::Affine<C>],
    words: &[[u64; 4]],
) -> zopro_bn254::Affine<C>
where
    C::Base: BatchInverse,
{
    let groups = scalar_groups_policy(words, true).expect("repeated-value fixture admits grouping");
    msm_grouped(bases, &groups).to_affine()
}

#[test]
fn scoped_pools_and_concurrent_callers_match_generator_oracles() {
    fn check<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let (bases, words, expected) = threaded_fixture::<C>();
        for workers in [1usize, 2, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 << 10)
                .build()
                .unwrap();
            std::thread::scope(|scope| {
                let jobs: Vec<_> = (0..4)
                    .map(|_| {
                        scope.spawn(|| {
                            for _ in 0..3 {
                                assert_eq!(
                                    pool.install(|| grouped_result::<C>(&bases, &words)),
                                    expected
                                );
                            }
                        })
                    })
                    .collect();
                for job in jobs {
                    job.join().unwrap();
                }
            });
        }
    }
    check::<G1Config>();
    check::<G2Config>();
}

#[test]
fn panic_waits_for_sibling_consumers_and_pool_remains_usable() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let (bases, words, expected) = threaded_fixture::<G2Config>();
    for workers in [1usize, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 << 10)
            .build()
            .unwrap();
        let completed = AtomicUsize::new(0);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            pool.install(|| {
                rayon::scope(|scope| {
                    for job in 0..4 {
                        let bases = &bases;
                        let words = &words;
                        let completed = &completed;
                        scope.spawn(move |_| {
                            let group = scalar_groups_policy(words, true).unwrap();
                            if job == 0 {
                                panic!("injected after constructing owned scalar groups");
                            }
                            assert_eq!(msm_grouped(bases, &group).to_affine(), expected);
                            completed.fetch_add(1, Ordering::Release);
                        });
                    }
                })
            })
        }));
        assert!(result.is_err());
        assert_eq!(completed.load(Ordering::Acquire), 3);
        assert_eq!(
            pool.install(|| grouped_result::<G2Config>(&bases, &words)),
            expected
        );
    }
}
#[test]
fn prefix_cutoff_matches_independent_ordered_map() {
    use std::collections::BTreeMap;
    let mut admitted_rejections = 0;
    let mut accepted = 0;
    for n in [256usize, 2049, 8193, 32768, 65536] {
        for kind in 0..10 {
            let boundary = n * if (32768..65536).contains(&n)
                && crate::simd::cpu_option("ZOPRO_CPU_SMALL_GROUPS")
            {
                95
            } else {
                80
            } / 100;
            let unique = match kind {
                0 => 1,
                1 => 17,
                2 | 5 => boundary - 1,
                3 | 6 => boundary,
                4 | 7 => boundary + 1,
                _ => n - n / 12,
            };
            let mut words: Vec<[u64; 4]> = (0..n)
                .map(|i| {
                    let index = if kind >= 5 { n - 1 - i } else { i };
                    let v = if index < unique { index + 1 } else { 1 } as u64;
                    [v, v.wrapping_mul(0x9e3779b97f4a7c15), !v, v.rotate_left(27)]
                })
                .collect();
            if kind == 8 {
                // Force duplicate samples without claiming representative sampling.
                let stride = n.div_ceil(n.min(2048));
                for i in (0..n).step_by(stride) {
                    words[i] = [1, 2, 3, 4];
                }
            }
            if kind == 9 {
                for i in (0..n).step_by(5) {
                    words[i] = [0; 4];
                }
            }
            let mut oracle = BTreeMap::<[u64; 4], Vec<u32>>::new();
            let mut first = Vec::new();
            for (i, &value) in words.iter().enumerate() {
                if value == [0; 4] {
                    continue;
                }
                if !oracle.contains_key(&value) {
                    first.push(value);
                }
                oracle.entry(value).or_default().push(i as u32);
            }
            for policy in [false, true] {
                let sample = grouping_sample(&words, policy);
                let want = sample.as_ref().is_some_and(|(_, percent)| {
                    oracle.len() as u128 * 100 <= n as u128 * *percent as u128
                });
                let actual = scalar_groups_policy(&words, policy);
                assert_eq!(actual.is_some(), want, "n={n} kind={kind} policy={policy}");
                if !want {
                    admitted_rejections += usize::from(sample.is_some());
                    continue;
                }
                accepted += 1;
                let group = actual.unwrap();
                assert_eq!(group.len, n);
                assert_eq!(group.values, first);
                assert_eq!(group.starts.len(), first.len() + 1);
                assert_eq!(group.starts[0], 0);
                for (class, value) in first.iter().enumerate() {
                    assert_eq!(group.counts[class], oracle[value].len());
                    assert_eq!(
                        &group.order[group.starts[class]..group.starts[class + 1]],
                        &oracle[value]
                    );
                }
                assert_eq!(
                    group.order.len(),
                    oracle.values().map(Vec::len).sum::<usize>()
                );
            }
        }
    }
    assert!(accepted > 0 && admitted_rejections > 0);
    println!("PREFIX_CUTOFF_ORACLE_CASES=100 accepted={accepted} admitted_rejections={admitted_rejections}");
}

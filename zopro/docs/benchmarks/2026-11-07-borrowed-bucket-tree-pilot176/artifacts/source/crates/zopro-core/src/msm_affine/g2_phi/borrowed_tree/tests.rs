use super::*;
use sha2::{Digest, Sha256};
use std::sync::atomic::Ordering;
use zopro_bn254::{Fr, G2Projective};

fn scalar(label: &str, i: usize) -> Fr {
    Fr::from_le_bytes_mod_order(&Sha256::digest(
        format!("borrowed-tree-48fa6c2d/{label}/{i}").as_bytes(),
    ))
}
fn points(n: usize) -> (Vec<G2Affine>, Vec<Fr>) {
    let g = G2Projective::generator();
    let base = scalar("point-base", 0);
    let step = scalar("point-step", 0);
    let mut p = g * base;
    let dp = g * step;
    let mut projective = Vec::with_capacity(n);
    let mut weights = Vec::with_capacity(n);
    for i in 0..n {
        let s = if i % 29 == 0 {
            Fr::ZERO
        } else if i % 31 == 0 {
            Fr::ONE
        } else {
            base + step * Fr::from(i as u64)
        };
        let value = if i % 29 == 0 {
            G2Projective::IDENTITY
        } else if i % 31 == 0 {
            g
        } else {
            p
        };
        weights.push(s);
        projective.push(value);
        p += dp;
    }
    (G2Projective::batch_to_affine(&projective), weights)
}

#[test]
fn descriptor_reuse_and_all_padding_signs() {
    let (input, weights) = points(70);
    let g = G2Projective::generator();
    // Avoid identities in this generic-only padding test.
    let indices = (0..input.len())
        .filter(|&i| !input[i].is_identity())
        .take(36)
        .collect::<Vec<_>>();
    let expected = G2Projective::batch_to_affine(
        &(0..18)
            .flat_map(|i| {
                let a = weights[indices[2 * i]];
                let b = weights[indices[2 * i + 1]];
                [g * (a + b), g * (a - b)]
            })
            .collect::<Vec<_>>(),
    );
    let original = input.clone();
    let mut adder = BatchAdd::<G2Config>::default();
    assert!(adder.borrowed_g2_enabled());
    let mut storage = PairStorage::default();
    let mut output = Vec::with_capacity(32);
    for mask in 0usize..256 {
        for n in 0..=18 {
            output.clear();
            let mut batch = storage.take();
            batch.reserve_exact(32);
            for i in 0..n {
                let negative = mask >> (i % 8) & 1 != 0;
                assert!(!append(
                    &mut output,
                    &mut batch,
                    input[indices[2 * i]],
                    Leaf {
                        point: &input[indices[2 * i + 1]],
                        negative
                    }
                ));
            }
            let address = batch.as_ptr();
            let capacity = batch.capacity();
            let (real, dummy) = execute(&mut output, &mut batch, &mut adder);
            assert_eq!(real, n);
            assert_eq!(dummy, (8 - n % 8) % 8);
            assert_eq!(output.len(), n);
            for i in 0..n {
                assert_eq!(output[i], expected[2 * i + ((mask >> (i % 8)) & 1)]);
            }
            storage.restore(batch);
            assert!(storage.0.is_empty());
            assert_eq!(storage.0.as_ptr(), address);
            assert_eq!(storage.0.capacity(), capacity);
        }
    }
    assert_eq!(input, original);
}

#[test]
fn balanced_stages_exceptions_skew_and_reuse() {
    let (input, weights) = points(257);
    let original = input.clone();
    let g = G2Projective::generator();
    let mut state = State::default();
    let lengths = [0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 127];
    for pass in 0..3 {
        for mode in 0..3 {
            let lists = (0..BAND)
                .map(|b| {
                    let n = if mode == 2 {
                        if b == 0 {
                            511
                        } else {
                            0
                        }
                    } else {
                        lengths[(b + pass) % lengths.len()]
                    };
                    (0..n)
                        .map(|i| {
                            // Fixed early patterns force identities, doubling and opposites.
                            let index = if mode == 0 && b % 4 == 0 {
                                0
                            } else if mode == 0 && b % 4 == 1 {
                                1
                            } else {
                                (b * 13 + i * 17 + pass) % input.len()
                            };
                            (index, (i + b + pass) % 3 == 0)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let counts = core::array::from_fn(|b| lists[b].len());
            let expected = G2Projective::batch_to_affine(
                &lists
                    .iter()
                    .map(|list| {
                        let value = list.iter().fold(Fr::ZERO, |a, &(i, neg)| {
                            if neg {
                                a - weights[i]
                            } else {
                                a + weights[i]
                            }
                        });
                        g * value
                    })
                    .collect::<Vec<_>>(),
            );
            let result = stage(
                &counts,
                |b, i| {
                    let (index, negative) = lists[b][i];
                    Leaf {
                        point: &input[index],
                        negative,
                    }
                },
                &mut state,
            );
            assert_eq!(&result[..], &expected);
            assert!(state.pairs.0.is_empty());
        }
    }
    assert!(
        state.stats.calls > 0
            && state.stats.real_pairs > 0
            && state.stats.dummy_pairs > 0
            && state.stats.exceptional > 0
    );
    assert_eq!(input, original);
}

#[test]
fn window_routing_on_small_stack_pools() {
    let (input, _) = points(1025);
    let original = input.clone();
    let words = (0..input.len())
        .map(|i| super::super::phi_split(scalar("window-scalars", i).to_canonical()))
        .collect::<Vec<_>>();
    // The independent control retains sequential borrowed streaming/queue order.
    let expected = [0, 4, 9].map(|w| super::super::hb_window(&input, &words, w, 12, false));
    for workers in [1, 4, 16, 32] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 * 1024)
            .build()
            .unwrap();
        pool.install(|| {
            for (i, w) in [0, 4, 9].into_iter().enumerate() {
                assert_eq!(window(&input, &words, w), expected[i]);
            }
            for n in [0, 1, 7, 8, 9, 31, 257] {
                let actual = window(&input[..n], &words[..n], 0);
                let control = super::super::hb_window(&input[..n], &words[..n], 0, 12, false);
                assert_eq!(actual, control);
            }
        });
    }
    assert_eq!(input, original);
}

#[test]
fn full_query_scalar_oracle_and_dispatch() {
    let (input, weights) = points(4097);
    let original = input.clone();
    let g = G2Projective::generator();
    for n in [1023, 1024, 1025, 4097] {
        let scalars = (0..n)
            .map(|i| scalar("query-scalars", i))
            .collect::<Vec<_>>();
        let target = g * scalars
            .iter()
            .zip(&weights)
            .fold(Fr::ZERO, |a, (s, p)| a + *s * *p);
        let mut words = scalars.iter().map(Fr::to_canonical).collect::<Vec<_>>();
        let before = WINDOW_CALLS.load(Ordering::Relaxed);
        let eligible = (1024..65536).contains(&n) && cfg!(zopro_borrowed_bucket_tree);
        if eligible {
            assert!(
                admit(n).is_some(),
                "qualification host must admit this bounded prototype"
            );
        }
        let result = super::super::raw_exclusive(&input[..n], &mut words);
        assert_eq!(result.to_affine(), target.to_affine());
        let calls = WINDOW_CALLS.load(Ordering::Relaxed) - before;
        assert_eq!(calls, if eligible { 20 } else { 0 });
        if n >= 1024 {
            for (s, w) in scalars.iter().zip(&words) {
                assert_eq!(*w, super::super::phi_split(s.to_canonical()));
            }
        }
    }
    assert_eq!(input, original);
}

#[test]
fn admission_checked_bounds_and_actual_reuse() {
    assert_eq!(extra_bytes(0, 1), None);
    assert_eq!(extra_bytes(1023, 1), None);
    assert_eq!(extra_bytes(65536, 1), None);
    assert_eq!(extra_bytes(1024, 0), None);
    assert_eq!(extra_bytes(65535, usize::MAX), None);
    for n in [1024, 30089, 65535] {
        let one = extra_bytes(n, 1).unwrap();
        let many = extra_bytes(n, 32).unwrap();
        assert!(many > one);
    }
    if !cfg!(zopro_borrowed_bucket_tree) {
        assert!(admit(1024).is_none());
    }
    let (input, _) = points(257);
    let mut state = State::default();
    let counts = core::array::from_fn(|b| if b == 0 { 32768 } else { 0 });
    let roots = stage(
        &counts,
        |_, i| Leaf {
            point: &input[i % input.len()],
            negative: i % 5 == 0,
        },
        &mut state,
    );
    assert_eq!(roots[1..], [G2Affine::IDENTITY; BAND - 1]);
    let bound = extra_bytes(65535, 1).unwrap();
    let payload = (state.current.capacity() + state.next.capacity())
        * core::mem::size_of::<G2Affine>()
        + state.pairs.0.capacity() * core::mem::size_of::<(u32, &G2Affine)>();
    assert!(payload < bound);
    assert!(state.stats.calls > 0);
}

#[cfg(test)]
mod h_only_tests {
    use super::*;
    fn points(values: &[Fr]) -> Vec<LimbPoint<Fq>> {
        let g = G1Projective::generator();
        G1Projective::batch_to_affine(&values.iter().map(|s| g * *s).collect::<Vec<_>>())
            .into_iter()
            .map(limbs::<G1Config>)
            .collect()
    }
    fn values(n: usize) -> Vec<Fr> {
        (0..n)
            .map(|i| Fr::from((i as u64 + 1) * 0x100000003 + 0x318af26))
            .collect()
    }
    fn assert_same(a: &[LimbPoint<Fq>], b: &[LimbPoint<Fq>]) {
        assert_eq!(a.len(), b.len());
        assert!(a.iter().zip(b).all(|(a, b)| a.x == b.x && a.y == b.y));
    }
    #[test]
    fn every_partial_lane_and_remainder_matches_complete_one_layer() {
        let mut t = Ternary::default();
        let mut attempts = 0;
        for triples in 8..24 {
            for tail in 0..3 {
                let counts = [triples * 3 + tail, 0, 1, 2, 3, 4, 5];
                let starts: Vec<_> = counts
                    .iter()
                    .scan(0, |at, n| {
                        let old = *at;
                        *at += n;
                        Some(old)
                    })
                    .collect();
                let input = points(&values(counts.iter().sum()));
                let mut want = input.clone();
                for (&start, &n) in starts.iter().zip(&counts) {
                    for j in 0..n.div_ceil(3) {
                        let mut sum = G1Projective::IDENTITY;
                        for k in 3 * j..(3 * j + 3).min(n) {
                            sum += affine::<G1Config>(input[start + k]);
                        }
                        want[start + j] = limbs::<G1Config>(sum.to_affine());
                    }
                }
                let mut out = input;
                let mut actual = counts;
                assert!(t.layer(&mut out, &starts, &mut actual));
                for ((&start, &n), &new) in starts.iter().zip(&counts).zip(&actual) {
                    assert_eq!(new, n.div_ceil(3));
                    assert_same(&out[start..start + new], &want[start..start + new]);
                }
                attempts += 1;
            }
        }
        eprintln!("H_NATIVE partial/remainder complete layers={attempts}");
    }
    #[test]
    fn checked_inverse_rejects_all_255_zero_masks() {
        let original = core::array::from_fn(|i| Fq::from(i as u64 + 2));
        let inv = FqBatch::new(&original).inverse_checked().unwrap().get();
        for i in 0..LANES {
            assert_eq!(original[i] * inv[i], Fq::ONE);
        }
        for mask in 1..256 {
            let a = core::array::from_fn(|i| {
                if mask & (1 << i) != 0 {
                    Fq::ZERO
                } else {
                    original[i]
                }
            });
            assert!(FqBatch::new(&a).inverse_checked().is_none());
        }
        eprintln!("H_NATIVE inverse masks=255+nonzero");
    }
    #[test]
    fn exceptional_lanes_guard_before_writes_and_opposites_are_admitted() {
        let mut checked = 0;
        let exceptions = [
            [Fr::ZERO, Fr::ONE, Fr::from(3)],
            [Fr::ONE, Fr::ZERO, Fr::from(3)],
            [Fr::ONE, Fr::from(3), Fr::ZERO],
            [Fr::ONE, Fr::ONE, Fr::from(3)],
            [Fr::ONE, Fr::from(2), Fr::from(3)],
            [Fr::ONE, Fr::from(2), -Fr::from(3)],
        ];
        for lane in 0..LANES {
            for triple in exceptions {
                let mut input_values = values(48);
                input_values[24 + lane * 3..27 + lane * 3].copy_from_slice(&triple);
                let original = points(&input_values);
                let mut out = original.clone();
                let mut counts = [48];
                assert!(!Ternary::default().layer(&mut out, &[0], &mut counts));
                assert_same(&out, &original);
                assert_eq!(counts, [48]);
                checked += 1;
            }
            let mut v = values(48);
            v[lane * 3] = Fr::ONE;
            v[lane * 3 + 1] = -Fr::ONE;
            v[lane * 3 + 2] = Fr::from(7);
            let original = points(&v);
            let mut out = original.clone();
            let mut count = [48];
            assert!(Ternary::default().layer(&mut out, &[0], &mut count));
            assert_same(&out[lane..lane + 1], &original[lane * 3 + 2..lane * 3 + 3]);
        }
        // Pair remainders are in the SAME chain, including partial lanes.
        for lane in 0..LANES {
            let counts = vec![5; LANES];
            let starts: Vec<_> = (0..LANES).map(|i| i * 5).collect();
            for pair in [[Fr::ONE, Fr::ONE], [Fr::ONE, -Fr::ONE], [Fr::ZERO, Fr::ONE]] {
                let mut v = values(5 * LANES);
                v[lane * 5 + 3..lane * 5 + 5].copy_from_slice(&pair);
                let original = points(&v);
                let mut out = original.clone();
                let mut c = counts.clone();
                assert!(!Ternary::default().layer(&mut out, &starts, &mut c));
                assert_same(&out, &original);
                assert_eq!(c, counts);
                checked += 1;
            }
        }
        eprintln!("H_NATIVE rejected-before-write={checked}; admitted opposite lanes=8");
    }
    #[test]
    fn malformed_geometry_rejected_before_mutation() {
        let original = points(&values(64));
        let cases = [
            (vec![], vec![24]),
            (vec![0], vec![]),
            (vec![0, 16], vec![24, 24]),
            (vec![30, 0], vec![24, 24]),
            (vec![60], vec![24]),
            (vec![usize::MAX], vec![24]),
            (vec![0, 65], vec![24, 0]),
        ];
        for (starts, counts) in cases {
            let mut out = original.clone();
            let mut c = counts.clone();
            let mut t = Ternary::default();
            assert!(!t.layer(&mut out, &starts, &mut c));
            assert_same(&out, &original);
            assert_eq!(c, counts);
            assert_eq!(t.bytes(), 0);
        }
        eprintln!("H_NATIVE malformed layouts=7");
    }
    #[test]
    fn reused_complete_scratch_on_small_stack_workers() {
        use std::sync::Arc;
        // Public synthetic arrays prepared once outside the workers. Each
        // worker owns scratch, reuses it through grow/shrink and fallback.
        let counts: Vec<Vec<usize>> = vec![
            vec![24],
            vec![2048],
            vec![25],
            vec![0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26],
            vec![511],
            vec![26],
            vec![3; 69],
        ];
        let data: Vec<_> = counts
            .iter()
            .map(|c| {
                let mut f = ReductionCaller::new(c);
                f.run(false, false, 1);
                f.check();
                (f.bases, f.order, f.starts, f.counts, f.expected)
            })
            .collect();
        let data = Arc::new(data);
        for workers in [1, 4, 16, 32] {
            let mut jobs = Vec::new();
            for id in 0..workers {
                let data = Arc::clone(&data);
                jobs.push(
                    std::thread::Builder::new()
                        .name(format!("h-only-{id}"))
                        .stack_size(512 * 1024)
                        .spawn(move || {
                            let mut w = Work::default();
                            for pass in 0..2 {
                                for (bases, order, starts, counts, want) in data.iter() {
                                    for selected in [false, true] {
                                        w.reduce::<true>(bases, order, starts, counts);
                                        assert_eq!(&w.output, want);
                                        // Force complete fallback through identity and
                                        // equality without reallocating the fixture.
                                        let mut alternate = bases.clone();
                                        if !alternate.is_empty() {
                                            alternate[0] = G1Affine::IDENTITY;
                                        }
                                        if alternate.len() > 3 {
                                            alternate[1] = alternate[2];
                                        }
                                        if selected {
                                            w.reduce::<true>(&alternate, order, starts, counts);
                                        } else {
                                            w.reduce::<false>(&alternate, order, starts, counts);
                                        }
                                        let expected: Vec<_> = starts
                                            .iter()
                                            .zip(counts)
                                            .map(|(&s, &n)| {
                                                let mut total = G1Projective::IDENTITY;
                                                for &i in &order[s..s + n] {
                                                    total += alternate[i as usize];
                                                }
                                                total.to_affine()
                                            })
                                            .collect();
                                        assert_eq!(w.output, expected);
                                    }
                                }
                                assert!(w.ternary.records.capacity() > 0, "pass{pass}");
                            }
                            w.bytes()
                        })
                        .unwrap(),
                );
            }
            let bytes: Vec<_> = jobs.into_iter().map(|j| j.join().unwrap()).collect();
            assert!(bytes.windows(2).all(|w| w[0] == w[1]));
            eprintln!(
                "H_NATIVE workers={workers} stack_bytes=524288 per_worker_capacity={:?}",
                bytes[0]
            );
        }
    }
}

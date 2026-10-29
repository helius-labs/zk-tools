#[cfg(test)]
mod complex_rowq_tests {
    use super::*;
    fn check<C: FpConfig, A: ark_ff::PrimeField<BigInt = ark_ff::BigInt<4>>, const M: bool>() {
        use ark_ff::{BigInteger, Field};
        let twice = crate::portable::add4(C::P, C::P).0;
        let vals = [
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(C::P, [1, 0, 0, 0]).0,
            C::P,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            twice,
        ];
        let bytes = |x: [u64; 4]| x.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();
        let inverse = A::from(2u64)
            .pow([if M { 256 } else { 260 }])
            .inverse()
            .unwrap();
        let host_inverse = A::from(2u64).pow([256]).inverse().unwrap();
        let mut seed = 0x745a134c61f1a27u64;
        let mut checks = 0;
        for group in 0..(1296 + 4096) / 8 {
            let input: [[[u64; 4]; 8]; 4] = core::array::from_fn(|slot| {
                core::array::from_fn(|lane| {
                    let idx = group * 8 + lane;
                    if idx < 1296 {
                        vals[(idx / 6usize.pow(slot as u32)) % 6]
                    } else {
                        let mut b = [0u8; 64];
                        for v in b.chunks_mut(8) {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            v.copy_from_slice(&seed.to_le_bytes());
                        }
                        let x = A::from_le_bytes_mod_order(&b).into_bigint().0;
                        if seed & 1 != 0 {
                            crate::portable::add4(x, C::P).0
                        } else {
                            x
                        }
                    }
                })
            });
            let vectors = input.map(|v| Ifma8::<C, M>::raw(&v));
            let fields = input.map(|v| v.map(|x| A::from_le_bytes_mod_order(&bytes(x)) * inverse));
            for conj in [false, true] {
                let mut state = [vectors[0], vectors[1]];
                let mut expected = [fields[0], fields[1]];
                for _ in 0..8 {
                    let old = if conj {
                        [
                            state[0].sum2(&vectors[2], &state[1], &vectors[3]),
                            state[1].difference2(&vectors[2], &state[0], &vectors[3]),
                        ]
                    } else {
                        [
                            state[0].difference2(&vectors[2], &state[1], &vectors[3]),
                            state[0].sum2(&vectors[3], &state[1], &vectors[2]),
                        ]
                    };
                    state = if conj {
                        Ifma8::complex_rowq::<true>(&state[0], &state[1], &vectors[2], &vectors[3])
                    } else {
                        Ifma8::complex_rowq::<false>(&state[0], &state[1], &vectors[2], &vectors[3])
                    };
                    let e0 = expected[0];
                    let e1 = expected[1];
                    for lane in 0..8 {
                        let (b0, b1) = (fields[2][lane], fields[3][lane]);
                        expected[0][lane] = if conj {
                            e0[lane] * b0 + e1[lane] * b1
                        } else {
                            e0[lane] * b0 - e1[lane] * b1
                        };
                        expected[1][lane] = if conj {
                            e1[lane] * b0 - e0[lane] * b1
                        } else {
                            e0[lane] * b1 + e1[lane] * b0
                        };
                    }
                    for component in 0..2 {
                        let raw: [[u64; 8]; 5] = state[component]
                            .0
                            .map(|v| unsafe { core::mem::transmute(v) });
                        let retained: [[u64; 8]; 5] =
                            old[component].0.map(|v| unsafe { core::mem::transmute(v) });
                        assert_eq!(raw, retained);
                        let host = state[component].get();
                        for lane in 0..8 {
                            assert_eq!(
                                A::from_le_bytes_mod_order(&bytes(host[lane].0)) * host_inverse,
                                expected[component][lane]
                            );
                            let w: [u64; 5] = core::array::from_fn(|j| raw[j][lane]);
                            assert!(w.iter().all(|v| *v < 1 << 52));
                            let packed = [
                                w[0] | w[1] << 52,
                                w[1] >> 12 | w[2] << 40,
                                w[2] >> 24 | w[3] << 28,
                                w[3] >> 36 | w[4] << 16,
                            ];
                            assert!(crate::portable::less(packed, twice));
                            checks += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(checks, 172544);
        eprintln!("COMPLEX_ROWQ_ARKWORKS_AND_RAW_OUTPUT_LANES_PER_CONFIG={checks}");
    }
    #[test]
    fn fq_complex_rowq_lazy_and_chains() {
        check::<crate::FqConfig, ark_ref::Fq, true>();
        check::<crate::FqConfig, ark_ref::Fq, false>();
    }
    #[test]
    fn fr_complex_rowq_lazy_and_chains() {
        check::<crate::FrConfig, ark_ref::Fr, true>();
        check::<crate::FrConfig, ark_ref::Fr, false>();
    }
}

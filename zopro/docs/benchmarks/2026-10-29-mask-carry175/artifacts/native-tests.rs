#[cfg(test)]
mod masked_tests {
    use super::*;
    #[test]
    fn masked_normalization_all_cascade_lengths() {
        type Q = Ifma8<crate::FqConfig, true>;
        const B: u64 = 1 << 52;
        let edge = [0, 1, B - 2, B - 1, B, B + 1, (1 << 61) - 2, (1 << 61) - 1];
        let mut inputs = Vec::new();
        for code in 0usize..32768 {
            inputs.push(core::array::from_fn::<_, 5, _>(|j| {
                edge[(code >> (3 * j)) & 7]
            }));
        }
        let mut seed = 0x4d41534b43415252u64;
        for _ in 0..8192 {
            inputs.push(core::array::from_fn(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed & ((1 << 61) - 1)
            }));
        }
        for chunk in inputs.chunks(8) {
            let raw: [__m512i; 5] = core::array::from_fn(|j| unsafe {
                core::mem::transmute(core::array::from_fn::<_, 8, _>(|i| chunk[i][j]))
            });
            let out = Q::normalize_masked(raw);
            let got: [[u64; 8]; 5] = out.map(|v| unsafe { core::mem::transmute(v) });
            for i in 0..8 {
                let mut want = chunk[i];
                for j in 0..4 {
                    want[j + 1] += want[j] >> 52;
                    want[j] &= B - 1;
                }
                assert_eq!(core::array::from_fn::<_, 5, _>(|j| got[j][i]), want);
            }
        }
        eprintln!("MASKED_NORMALIZATION_CASES={}", inputs.len());
    }
    fn check<C: FpConfig, A: ark_ff::PrimeField<BigInt = ark_ff::BigInt<4>>, const M: bool>() {
        use ark_ff::{BigInteger, Field};
        let twice = crate::portable::add4(C::P, C::P).0;
        let bytes = |x: [u64; 4]| x.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();
        let inverse = A::from(2u64)
            .pow([if M { 256 } else { 260 }])
            .inverse()
            .unwrap();
        let out_inverse = A::from(2u64).pow([256]).inverse().unwrap();
        let vals = [
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(C::P, [1, 0, 0, 0]).0,
            C::P,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            twice,
        ];
        let mut seed = 0x1234e29cc9e25f6au64;
        for group in 0..(1296 + 4096) / 8 {
            let input: [[[u64; 4]; 8]; 4] = core::array::from_fn(|slot| {
                core::array::from_fn(|lane| {
                    let idx = group * 8 + lane;
                    if idx < 1296 {
                        vals[(idx / 6usize.pow(slot as u32)) % 6]
                    } else {
                        let mut b = [0u8; 64];
                        for q in b.chunks_mut(8) {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            q.copy_from_slice(&seed.to_le_bytes());
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
            let v = input.map(|x| Ifma8::<C, M>::raw(&x));
            let field = input.map(|x| x.map(|a| A::from_le_bytes_mod_order(&bytes(a)) * inverse));
            let mut state = v[0];
            let mut expected = field[0];
            for step in 0..8 {
                let fused = step % 2 == 1;
                state = if fused {
                    state.sum2_masked(&v[1], &v[2], &v[3])
                } else {
                    state.mul_masked(&v[1])
                };
                let digits: [[u64; 8]; 5] = state.0.map(|x| unsafe { core::mem::transmute(x) });
                let got = state.get();
                for lane in 0..8 {
                    expected[lane] *= field[1][lane];
                    if fused {
                        expected[lane] += field[2][lane] * field[3][lane];
                    }
                    assert_eq!(
                        A::from_le_bytes_mod_order(&bytes(got[lane].0)) * out_inverse,
                        expected[lane]
                    );
                    let word = core::array::from_fn::<_, 5, _>(|j| digits[j][lane]);
                    assert!(word.iter().all(|v| *v < 1 << 52));
                    let repr = [
                        word[0] | word[1] << 52,
                        word[1] >> 12 | word[2] << 40,
                        word[2] >> 24 | word[3] << 28,
                        word[3] >> 36 | word[4] << 16,
                    ];
                    assert!(crate::portable::less(repr, twice));
                }
            }
        }
        eprintln!(
            "MASKED_CHAIN_LANES_PER_CONFIG={}",
            ((1296 + 4096) / 8) * 8 * 8
        );
    }
    #[test]
    fn masked_ordinary_fused_chains_match_arkworks() {
        check::<crate::FqConfig, ark_ref::Fq, true>();
        check::<crate::FqConfig, ark_ref::Fq, false>();
        check::<crate::FrConfig, ark_ref::Fr, true>();
        check::<crate::FrConfig, ark_ref::Fr, false>();
    }
}

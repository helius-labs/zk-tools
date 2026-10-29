#[cfg(test)]
mod rowq_tests {
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
        let host_inv = A::from(2u64).pow([256]).inverse().unwrap();
        let mut seed = 0x51424c4f434b3333u64;
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
            let v = input.map(|x| Ifma8::<C, M>::raw(&x));
            let field = input.map(|x| x.map(|a| A::from_le_bytes_mod_order(&bytes(a)) * inverse));
            for fused in [false, true] {
                let mut state = v[0];
                let mut expected = field[0];
                for _ in 0..8 {
                    let ordinary = if fused {
                        state.sum2(&v[1], &v[2], &v[3])
                    } else {
                        state.mul(&v[1])
                    };
                    state = if fused {
                        state.sum2_rowq(&v[1], &v[2], &v[3])
                    } else {
                        state.mul_rowq(&v[1])
                    };
                    let got: [[u64; 8]; 5] = state.0.map(|x| unsafe { core::mem::transmute(x) });
                    let retained: [[u64; 8]; 5] =
                        ordinary.0.map(|x| unsafe { core::mem::transmute(x) });
                    assert_eq!(got, retained);
                    let host = state.get();
                    for i in 0..8 {
                        expected[i] *= field[1][i];
                        if fused {
                            expected[i] += field[2][i] * field[3][i];
                        }
                        assert_eq!(
                            A::from_le_bytes_mod_order(&bytes(host[i].0)) * host_inv,
                            expected[i]
                        );
                        let w = core::array::from_fn::<_, 5, _>(|j| got[j][i]);
                        assert!(w.iter().all(|v| *v < 1 << 52));
                        let raw = [
                            w[0] | w[1] << 52,
                            w[1] >> 12 | w[2] << 40,
                            w[2] >> 24 | w[3] << 28,
                            w[3] >> 36 | w[4] << 16,
                        ];
                        assert!(crate::portable::less(raw, twice));
                    }
                }
            }
        }
        eprintln!(
            "ROWQ_ARKWORKS_AND_RAW_LANES_PER_CONFIG={}",
            ((1296 + 4096) / 8) * 8 * 8 * 2
        );
    }
    #[test]
    fn fq_rowq_quotients_and_chains() {
        check::<crate::FqConfig, ark_ref::Fq, true>();
        check::<crate::FqConfig, ark_ref::Fq, false>();
    }
    #[test]
    fn fr_rowq_quotients_and_chains() {
        check::<crate::FrConfig, ark_ref::Fr, true>();
        check::<crate::FrConfig, ark_ref::Fr, false>();
    }
}

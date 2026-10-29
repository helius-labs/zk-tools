#[cfg(test)]
mod product_sub_tests {
    use super::*;
    use ark_ff::{Field, PrimeField};
    fn check<C: FpConfig, A: PrimeField<BigInt = ark_ff::BigInt<4>>>() {
        let p = C::P;
        let twice = crate::portable::add4(p, p).0;
        let edges = [
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(p, [1, 0, 0, 0]).0,
            p,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            twice,
        ];
        let canonical = [
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(p, [1, 0, 0, 0]).0,
        ];
        let bytes = |x: [u64; 4]| x.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();
        let inv = A::from(2u64).pow([256]).inverse().unwrap();
        let numeric = |x: [u64; 4]| A::from_le_bytes_mod_order(&bytes(x)) * inv;
        let mut seed = 0x53554243414e4f4eu64;
        for group in 0..(1296 + 4096) / 8 {
            let input: [[[u64; 4]; 8]; 4] = core::array::from_fn(|slot| {
                core::array::from_fn(|lane| {
                    let index = group * 8 + lane;
                    if index < 1296 {
                        edges[(index / 6usize.pow(slot as u32)) % 6]
                    } else {
                        let mut b = [0u8; 64];
                        for v in b.chunks_mut(8) {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            v.copy_from_slice(&seed.to_le_bytes());
                        }
                        let x = Fp::<C>::from_random_bytes(&b).0;
                        if seed & 1 != 0 {
                            crate::portable::add4(x, p).0
                        } else {
                            x
                        }
                    }
                })
            });
            let v = input.map(|x| Ifma8::<C, true>::raw(&x));
            let scalars = input.map(|x| x.map(numeric));
            let z = core::array::from_fn(|lane| canonical[(group + lane) % 3]);
            let sub = Ifma8::<C, true>::raw(&z);
            for minus in [false, true] {
                let mut x = v[0];
                let mut expected = scalars[0];
                for _ in 0..8 {
                    x = if minus {
                        x.difference2_sub_canonical(&v[1], &v[2], &v[3], &sub)
                    } else {
                        x.sum2_sub_canonical(&v[1], &v[2], &v[3], &sub)
                    };
                    let digit: [[u64; 8]; 5] = x.0.map(|a| unsafe { core::mem::transmute(a) });
                    let actual = x.get();
                    for lane in 0..8 {
                        expected[lane] = expected[lane] * scalars[1][lane]
                            + if minus {
                                -(scalars[2][lane] * scalars[3][lane])
                            } else {
                                scalars[2][lane] * scalars[3][lane]
                            }
                            - numeric(z[lane]);
                        assert_eq!(numeric(actual[lane].0), expected[lane]);
                        let w = core::array::from_fn::<_, 5, _>(|j| digit[j][lane]);
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
            "PRODUCT_SUB_CHAIN_LANES_PER_FIELD={}",
            ((1296 + 4096) / 8) * 8 * 8 * 2
        );
    }
    #[test]
    fn fq_product_sub_chains_arkworks() {
        check::<crate::FqConfig, ark_ref::Fq>();
    }
    #[test]
    fn fr_product_sub_chains_arkworks() {
        check::<crate::FrConfig, ark_ref::Fr>();
    }
}

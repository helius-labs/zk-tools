impl<C: FpConfig, const MIXED: bool> Ifma8<C, MIXED> {
    #[inline(always)]
    fn pair_negative(&self) -> Self {
        unsafe {
            let p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let mask = Self::splat(MASK);
            let mut out = [Self::splat(0); 5];
            let mut carry = Self::splat(0);
            for j in 0..5 {
                let v = _mm512_add_epi64(_mm512_sub_epi64(p[j], self.0[j]), carry);
                carry = _mm512_srai_epi64::<52>(v);
                out[j] = _mm512_and_si512(v, mask);
            }
            Self(out, PhantomData)
        }
    }
    #[inline(always)]
    pub fn complex_rowq<const CONJ: bool>(a0: &Self, a1: &Self, b0: &Self, b1: &Self) -> [Self; 2] {
        unsafe {
            let zero = Self::splat(0);
            let inv = Self::splat(C::INV & MASK);
            let u = _mm512_madd52lo_epu64(zero, a0.0[0], inv);
            let v = _mm512_madd52lo_epu64(zero, a1.0[0], inv);
            let uv = _mm512_add_epi64(u, v);
            let two = Self::splat(2 * (1u64 << 52));
            let four = Self::splat(4 * (1u64 << 52));
            let base = Self::splat(1u64 << 52);
            let mut real = [zero; 5];
            let mut imag = [zero; 5];
            for i in 0..5 {
                let both = _mm512_add_epi64(b0.0[i], b1.0[i]);
                let twice_b1 = _mm512_slli_epi64::<1>(b1.0[i]);
                if CONJ {
                    let p = _mm512_madd52lo_epu64(zero, uv, b0.0[i]);
                    let q = _mm512_madd52lo_epu64(zero, u, both);
                    let t = _mm512_madd52lo_epu64(
                        zero,
                        v,
                        _mm512_sub_epi64(_mm512_add_epi64(base, b1.0[i]), b0.0[i]),
                    );
                    real[i] = _mm512_add_epi64(p, t);
                    imag[i] =
                        _mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(p, four), q), twice_b1);
                } else {
                    let x = _mm512_madd52lo_epu64(zero, u, b0.0[i]);
                    let y = _mm512_madd52lo_epu64(zero, v, b1.0[i]);
                    let z = _mm512_madd52lo_epu64(zero, uv, both);
                    real[i] =
                        _mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(x, four), y), twice_b1);
                    imag[i] = _mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(z, two), x), y);
                }
            }
            if CONJ {
                let neg = a0.pair_negative();
                [
                    a0.product_pairrow::<true>(b0, a1, b1, &real),
                    a1.product_pairrow::<true>(b0, &neg, b1, &imag),
                ]
            } else {
                let neg = a1.pair_negative();
                [
                    a0.product_pairrow::<true>(b0, &neg, b1, &real),
                    a0.product_pairrow::<true>(b1, a1, b0, &imag),
                ]
            }
        }
    }
    #[inline(always)]
    fn product_pairrow<const FUSED: bool>(
        &self,
        b: &Self,
        c: &Self,
        d: &Self,
        rowq: &[__m512i; 5],
    ) -> Self {
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            let mut t = [zero; 6];
            macro_rules! row {
                ($i:literal) => {{
                    let m = if $i == 0 {
                        _mm512_and_si512(rowq[$i], mask)
                    } else {
                        _mm512_and_si512(_mm512_madd52lo_epu64(rowq[$i], t[0], inv), mask)
                    };
                    macro_rules! product {
                        ($j:literal,$k:literal) => {
                            t[$j] = _mm512_madd52lo_epu64(t[$j], self.0[$j], b.0[$i]);
                            t[$k] = _mm512_madd52hi_epu64(t[$k], self.0[$j], b.0[$i]);
                        };
                    }
                    product!(0, 1);
                    product!(1, 2);
                    product!(2, 3);
                    product!(3, 4);
                    product!(4, 5);
                    if FUSED {
                        macro_rules! second {
                            ($j:literal,$k:literal) => {
                                t[$j] = _mm512_madd52lo_epu64(t[$j], c.0[$j], d.0[$i]);
                                t[$k] = _mm512_madd52hi_epu64(t[$k], c.0[$j], d.0[$i]);
                            };
                        }
                        second!(0, 1);
                        second!(1, 2);
                        second!(2, 3);
                        second!(3, 4);
                        second!(4, 5);
                    }
                    macro_rules! reduction {
                        ($j:literal,$k:literal) => {
                            t[$j] = _mm512_madd52lo_epu64(t[$j], m, p[$j]);
                            t[$k] = _mm512_madd52hi_epu64(t[$k], m, p[$j]);
                        };
                    }
                    if cfg!(zopro_cios_cancel_carry) {
                        // The cancelled low digit contributes ceil(t0/B).
                        // Compute it without waiting for m*p0's low half.
                        let carry = _mm512_srli_epi64::<52>(_mm512_add_epi64(t[0], mask));
                        let low = _mm512_madd52hi_epu64(carry, m, p[0]);
                        reduction!(1, 2);
                        reduction!(2, 3);
                        reduction!(3, 4);
                        reduction!(4, 5);
                        t[1] = _mm512_add_epi64(t[1], low);
                    } else {
                        reduction!(0, 1);
                        reduction!(1, 2);
                        reduction!(2, 3);
                        reduction!(3, 4);
                        reduction!(4, 5);
                        t[1] = _mm512_add_epi64(t[1], _mm512_srli_epi64::<52>(t[0]));
                    }
                    t = [t[1], t[2], t[3], t[4], t[5], zero];
                }};
            }
            row!(0);
            row!(1);
            row!(2);
            row!(3);
            if MIXED {
                let m = _mm512_and_si512(
                    _mm512_madd52lo_epu64(rowq[4], t[0], inv),
                    Self::splat((1u64 << 48) - 1),
                );
                // Four radix-52 cancellation steps followed by radix-48:
                // R=2^(4*52+48)=2^256, exactly the key/host representation.
                macro_rules! product {
                    ($j:literal,$k:literal) => {
                        t[$j] = _mm512_madd52lo_epu64(t[$j], self.0[$j], b.0[4]);
                        t[$k] = _mm512_madd52hi_epu64(t[$k], self.0[$j], b.0[4]);
                    };
                }
                product!(0, 1);
                product!(1, 2);
                product!(2, 3);
                product!(3, 4);
                product!(4, 5);
                if FUSED {
                    macro_rules! second {
                        ($j:literal,$k:literal) => {
                            t[$j] = _mm512_madd52lo_epu64(t[$j], c.0[$j], d.0[4]);
                            t[$k] = _mm512_madd52hi_epu64(t[$k], c.0[$j], d.0[4]);
                        };
                    }
                    second!(0, 1);
                    second!(1, 2);
                    second!(2, 3);
                    second!(3, 4);
                    second!(4, 5);
                }
                macro_rules! reduction {
                    ($j:literal,$k:literal) => {
                        t[$j] = _mm512_madd52lo_epu64(t[$j], m, p[$j]);
                        t[$k] = _mm512_madd52hi_epu64(t[$k], m, p[$j]);
                    };
                }
                if cfg!(zopro_cios_cancel_carry) {
                    let mask48 = Self::splat((1u64 << 48) - 1);
                    let carry = _mm512_srli_epi64::<48>(_mm512_add_epi64(t[0], mask48));
                    // m<2^48, so shifting by4 remains an exact IFMA input.
                    let low = _mm512_madd52hi_epu64(carry, _mm512_slli_epi64::<4>(m), p[0]);
                    reduction!(1, 2);
                    reduction!(2, 3);
                    reduction!(3, 4);
                    reduction!(4, 5);
                    // Divide the unnormalized coefficients by2^48 directly.
                    // Splitting each higher coefficient at48 bits avoids
                    // the original extra normalization before radix packing.
                    t = [
                        _mm512_add_epi64(
                            low,
                            _mm512_slli_epi64::<4>(_mm512_and_si512(t[1], mask48)),
                        ),
                        _mm512_add_epi64(
                            _mm512_srli_epi64::<48>(t[1]),
                            _mm512_slli_epi64::<4>(_mm512_and_si512(t[2], mask48)),
                        ),
                        _mm512_add_epi64(
                            _mm512_srli_epi64::<48>(t[2]),
                            _mm512_slli_epi64::<4>(_mm512_and_si512(t[3], mask48)),
                        ),
                        _mm512_add_epi64(
                            _mm512_srli_epi64::<48>(t[3]),
                            _mm512_slli_epi64::<4>(_mm512_and_si512(t[4], mask48)),
                        ),
                        _mm512_add_epi64(
                            _mm512_srli_epi64::<48>(t[4]),
                            _mm512_slli_epi64::<4>(t[5]),
                        ),
                        zero,
                    ];
                } else {
                    reduction!(0, 1);
                    reduction!(1, 2);
                    reduction!(2, 3);
                    reduction!(3, 4);
                    reduction!(4, 5);
                    for j in 0..5 {
                        t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
                        t[j] = _mm512_and_si512(t[j], mask);
                    }
                    for j in 0..5 {
                        t[j] = _mm512_or_si512(
                            _mm512_srli_epi64::<48>(t[j]),
                            _mm512_and_si512(_mm512_slli_epi64::<4>(t[j + 1]), mask),
                        );
                    }
                }
            } else {
                row!(4);
            }
            for j in 0..4 {
                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
                t[j] = _mm512_and_si512(t[j], mask);
            }
            // Inputs are below 2p, so REDC is below p + 4p²/2^260 < 2p.
            let result = [t[0], t[1], t[2], t[3], t[4]];
            if FUSED && MIXED {
                Self::reduce_fused(result)
            } else {
                Self(result, PhantomData)
            }
        }
    }
}

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

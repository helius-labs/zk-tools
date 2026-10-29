impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{
#[inline(always)]pub fn mul_masked(&self,b:&Self)->Self{self.product_masked::<false>(b,self,b)}
#[inline(always)]pub fn sum2_masked(&self,b:&Self,c:&Self,d:&Self)->Self{self.product_masked::<true>(b,c,d)}
    /// Exact five-coefficient normalization for unsigned t[j] < 2^61.
    /// The top digit retains the ordinary ripple's unrestricted high word.
    #[inline(always)]
    fn normalize_masked(t: [__m512i; 5]) -> [__m512i; 5] {
        unsafe {
            let mask = Self::splat(MASK);
            let one = Self::splat(1);
            let v1 = _mm512_add_epi64(_mm512_and_si512(t[1],mask),_mm512_srli_epi64::<52>(t[0]));
            let v2 = _mm512_add_epi64(_mm512_and_si512(t[2],mask),_mm512_srli_epi64::<52>(t[1]));
            let v3 = _mm512_add_epi64(_mm512_and_si512(t[3],mask),_mm512_srli_epi64::<52>(t[2]));
            let v4 = _mm512_add_epi64(t[4],_mm512_srli_epi64::<52>(t[3]));
            let c1 = _mm512_cmpgt_epu64_mask(v1,mask);
            let c2 = _mm512_cmpgt_epu64_mask(v2,mask) | (_mm512_cmpeq_epi64_mask(v2,mask) & c1);
            let c3 = _mm512_cmpgt_epu64_mask(v3,mask) | (_mm512_cmpeq_epi64_mask(v3,mask) & c2);
            [
                _mm512_and_si512(t[0],mask),
                _mm512_and_si512(v1,mask),
                _mm512_and_si512(_mm512_mask_add_epi64(v2,c1,v2,one),mask),
                _mm512_and_si512(_mm512_mask_add_epi64(v3,c2,v3,one),mask),
                _mm512_mask_add_epi64(v4,c3,v4,one),
            ]
        }
    }

    #[inline(always)]
    fn product_masked<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            let mut t = [zero; 6];
            macro_rules! row {
                ($i:literal) => {{
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
                    let m = _mm512_and_si512(_mm512_madd52lo_epu64(zero, t[0], inv), mask);
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
                let m = _mm512_and_si512(
                    _mm512_madd52lo_epu64(zero, t[0], inv),
                    Self::splat((1 << 48) - 1),
                );
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
            let out = Self::normalize_masked([t[0], t[1], t[2], t[3], t[4]]);
            t = [out[0],out[1],out[2],out[3],out[4],zero];
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

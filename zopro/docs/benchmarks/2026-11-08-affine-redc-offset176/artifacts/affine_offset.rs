//! Deferred normalization of canonical affine offsets; private R256 study.
use super::*;
impl<C: FpConfig> Ifma8<C, true> {
    /// Normalized product operands <=2p; x is canonical, below p.
    #[inline(always)]
    pub fn affine_mul_minus(&self, b: &Self, x: &Self) -> Self {
        self.offset_product::<false, false, false>(b, self, b, x, &Self::zero())
    }
    /// x,y are canonical raw R256 coordinates; their sum is not materialized.
    #[inline(always)]
    pub fn affine_mul_minus_sum(&self, b: &Self, x: &Self, y: &Self) -> Self {
        self.offset_product::<false, false, true>(b, self, b, x, y)
    }
    #[inline(always)]
    pub fn affine_mul_double_minus_sum(&self, b: &Self, x: &Self, y: &Self) -> Self {
        self.offset_product::<false, true, true>(b, self, b, x, y)
    }
    #[inline(always)]
    pub fn affine_sum2_minus(&self, b: &Self, c: &Self, d: &Self, x: &Self) -> Self {
        self.offset_product::<true, false, false>(b, c, d, x, &Self::zero())
    }
    #[inline(always)]
    pub fn affine_difference2_minus(&self, b: &Self, c: &Self, d: &Self, x: &Self) -> Self {
        // Exact accepted difference2 complement, including 2p at c=0.
        let negative = unsafe {
            let p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let mask = Self::splat(MASK);
            let mut t = [Self::splat(0); 5];
            let mut carry = Self::splat(0);
            for j in 0..5 {
                let v = _mm512_add_epi64(_mm512_sub_epi64(p[j], c.0[j]), carry);
                carry = _mm512_srai_epi64::<52>(v);
                t[j] = _mm512_and_si512(v, mask);
            }
            Self(t, PhantomData)
        };
        self.offset_product::<true, false, false>(b, &negative, d, x, &Self::zero())
    }

    #[inline(always)]
    fn offset_product<const FUSED: bool, const DOUBLE: bool, const SUM: bool>(&self, b: &Self, c: &Self, d: &Self, x: &Self, y: &Self) -> Self {
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
            if true {
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
            // t is the unchanged CIOS result after R256 radix conversion,
            // before its original final carry propagation. Add coordinate
            // offsets directly as signed, unnormalized radix52 coefficients.
            let twice = const { crate::portable::add4(C::P, C::P).0 };
            let bias = split(if SUM { twice } else { C::P }).map(Self::splat);
            for j in 0..5 {
                if DOUBLE { t[j] = _mm512_add_epi64(t[j], t[j]); }
                t[j] = _mm512_sub_epi64(_mm512_add_epi64(t[j], bias[j]), x.0[j]);
                if SUM { t[j] = _mm512_sub_epi64(t[j], y.0[j]); }
            }
            // Negative coefficients need arithmetic carries. Whole biased
            // values are nonnegative; the final top limb is normalized.
            for j in 0..4 {
                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srai_epi64::<52>(t[j]));
                t[j] = _mm512_and_si512(t[j], mask);
            }
            let mut result = [t[0], t[1], t[2], t[3], t[4]];
            if DOUBLE {
                let four = const { let two = crate::portable::add4(C::P, C::P).0; crate::portable::add4(two, two).0 };
                result = Self::offset_reduce_raw(result, four);
            }
            Self(Self::offset_reduce_raw(result, twice), PhantomData)
        }
    }

    /// Exact raw conditional subtraction, keeping wider transient values out
    /// of the public normalized-lazy arithmetic API. Equal high limbs are
    /// ambiguous and use the full subtraction; smaller high limbs return.
    #[inline(always)]
    fn offset_reduce_raw(mut t: [__m512i; 5], modulus: [u64; 4]) -> [__m512i; 5] {
        unsafe {
            let p = split(modulus).map(Self::splat);
            if _mm512_cmpge_epu64_mask(t[4], p[4]) == 0 { return t; }
            let mut d = [Self::splat(0); 5];
            let mut borrow = Self::splat(0);
            let mask = Self::splat(MASK);
            for j in 0..5 {
                let v = _mm512_add_epi64(_mm512_sub_epi64(t[j], p[j]), borrow);
                d[j] = _mm512_and_si512(v, mask);
                borrow = _mm512_srai_epi64::<52>(v);
            }
            let keep = _mm512_cmpneq_epi64_mask(borrow, Self::splat(0));
            for j in 0..5 { t[j] = _mm512_mask_blend_epi64(keep, d[j], t[j]); }
            t
        }
    }
    /// Diagnostics only: raw numerical R256 words, each <=2p. No Fp with
    /// a noncanonical value is constructed. Caller checks the field bounds.
    #[cfg(zopro_affine_offset_diagnostics)]
    pub unsafe fn affine_offset_from_raw(values: &[[u64; 4]; 8]) -> Self {
        let words = values.map(split);
        Self(core::array::from_fn(|j| {
            let lanes: [u64; 8] = core::array::from_fn(|i| words[i][j]);
            unsafe { _mm512_loadu_si512(lanes.as_ptr().cast()) }
        }), PhantomData)
    }
    #[cfg(zopro_affine_offset_diagnostics)]
    pub fn affine_offset_raw_digits(&self) -> [[u64; 8]; 5] {
        let mut out = [[0u64; 8]; 5];
        for j in 0..5 { unsafe { _mm512_storeu_si512(out[j].as_mut_ptr().cast(), self.0[j]); } }
        out
    }
}

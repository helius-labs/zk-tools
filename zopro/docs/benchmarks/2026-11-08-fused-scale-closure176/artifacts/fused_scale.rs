//! Narrow public outputs; wide fused REDC is private to one scaled helper.
use crate::{FpConfig,ifma8::Ifma8};
use core::{arch::x86_64::*,marker::PhantomData};
const MASK:u64=(1<<52)-1;
#[inline(always)] fn splat(x:u64)->__m512i{unsafe{_mm512_set1_epi64(x as i64)}}
const fn split(x: [u64; 4]) -> [u64; 5] {
    [
        x[0] & MASK,
        ((x[0] >> 52) | (x[1] << 12)) & MASK,
        ((x[1] >> 40) | (x[2] << 24)) & MASK,
        ((x[2] >> 28) | (x[3] << 36)) & MASK,
        x[3] >> 16,
    ]
}


impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{
    #[inline(always)]
    fn fused_scale_wide<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {
        unsafe {
            let zero = splat(0);
            let mask = splat(MASK);
            let p = split(C::P).map(splat);
            let inv = splat(C::INV & MASK);
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
                    splat((1 << 48) - 1),
                );
                macro_rules! reduction {
                    ($j:literal,$k:literal) => {
                        t[$j] = _mm512_madd52lo_epu64(t[$j], m, p[$j]);
                        t[$k] = _mm512_madd52hi_epu64(t[$k], m, p[$j]);
                    };
                }
                if cfg!(zopro_cios_cancel_carry) {
                    let mask48 = splat((1u64 << 48) - 1);
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
            Self(result, PhantomData)
        }
    }

    /// Inputs normalized <=2p. The wide intermediate does not escape;
    /// 16p^2+2pR<R^2 ensures the following REDC returns strictly below2p.
    #[inline(always)]
    pub fn fused_sum_scaled(&self,b:&Self,c:&Self,d:&Self,f:&Self)->Self{
        assert!(C::P[3]<(199u64<<54));
        self.fused_scale_wide::<true>(b,c,d).mul(f)
    }
    #[inline(always)]
    pub fn fused_difference_scaled(&self,b:&Self,c:&Self,d:&Self,f:&Self)->Self{
        let negative = unsafe {
            let p = split(crate::portable::add4(C::P, C::P).0).map(splat);
            let mask = splat(MASK);
            let mut t = [splat(0); 5];
            let mut carry = splat(0);
            for j in 0..5 {
                let v = _mm512_add_epi64(_mm512_sub_epi64(p[j], c.0[j]), carry);
                carry = _mm512_srai_epi64::<52>(v);
                t[j] = _mm512_and_si512(v, mask);
            }
            Self(t, PhantomData)
        };

        self.fused_sum_scaled(b,&negative,d,f)
    }
    #[cfg(zopro_fused_scale_diagnostics)]
    pub fn diagnostic_fused_scale_from_digits(values:[[u64;5];8])->Self{
        unsafe{Self(core::array::from_fn(|j|{let lanes:[u64;8]=core::array::from_fn(|i|values[i][j]);_mm512_loadu_si512(lanes.as_ptr().cast())}),PhantomData)}
    }
    #[cfg(zopro_fused_scale_diagnostics)]
    pub fn diagnostic_fused_scale_digits(&self)->[[u64;5];8]{
        let columns:[[u64;8];5]=self.0.map(|v|unsafe{core::mem::transmute(v)});core::array::from_fn(|i|core::array::from_fn(|j|columns[j][i]))
    }
}

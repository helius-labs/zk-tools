//! Four independent fields in radix 2^52. The mixed-radix variant uses
//! Montgomery R=2^256 to match host fields; the uniform variant uses R=2^260.
//! AVX-512 IFMA accumulates each 52x52 product in low/high halves.
//! All limbs are normalized before a result leaves an operation.
use crate::{Fp, FpConfig};
use core::{arch::x86_64::*, marker::PhantomData};

const MASK: u64 = (1 << 52) - 1;
const LANES: usize = 4;

#[derive(Clone, Copy)]
pub struct Ifma<C: FpConfig, const MIXED: bool = false>(
    pub(crate) [__m256i; 5],
    pub(crate) PhantomData<C>,
);

/// Four vector words containing the same numerical Montgomery values.
/// Storage is compact; unpacking does not change radix or reduce values.
#[derive(Clone, Copy)]
pub struct PackedIfma<C: FpConfig, const MIXED: bool = false>([__m256i; 4], PhantomData<C>);

impl<C: FpConfig, const MIXED: bool> PackedIfma<C, MIXED> {
    #[inline(always)]
    pub fn expand(&self) -> Ifma<C, MIXED> {
        // SAFETY: this module requires AVX-512 IFMA/VL. The private
        // constructor only receives normalized values below 2^256.
        unsafe {
            let [w0, w1, w2, w3] = self.0;
            let mask = Ifma::<C, MIXED>::splat(MASK);
            Ifma(
                [
                    _mm256_and_si256(w0, mask),
                    _mm256_and_si256(
                        _mm256_or_si256(_mm256_srli_epi64::<52>(w0), _mm256_slli_epi64::<12>(w1)),
                        mask,
                    ),
                    _mm256_and_si256(
                        _mm256_or_si256(_mm256_srli_epi64::<40>(w1), _mm256_slli_epi64::<24>(w2)),
                        mask,
                    ),
                    _mm256_and_si256(
                        _mm256_or_si256(_mm256_srli_epi64::<28>(w2), _mm256_slli_epi64::<36>(w3)),
                        mask,
                    ),
                    _mm256_srli_epi64::<16>(w3),
                ],
                PhantomData,
            )
        }
    }
}

#[inline(always)]
const fn split(x: [u64; 4]) -> [u64; 5] {
    [
        x[0] & MASK,
        ((x[0] >> 52) | (x[1] << 12)) & MASK,
        ((x[1] >> 40) | (x[2] << 24)) & MASK,
        ((x[2] >> 28) | (x[3] << 36)) & MASK,
        x[3] >> 16,
    ]
}

// floor(2^288/p), computed at compile time. BN254's two primes are
// in [2^253,2^254), so the reciprocal is below 2^35.
const fn small_reciprocal(p:[u64;4])->u64 {
    assert!(p[3] >= 1u64<<61 && p[3] < 1u64<<62);
    let mut rem=[1u64,0,0,0];let mut quotient=0u64;let mut i=0;
    while i<288 {
        rem=crate::portable::add4(rem,rem).0;
        let ge=!crate::portable::less(rem,p);
        if ge {rem=crate::portable::sub4(rem,p).0;}
        quotient=(quotient<<1)|ge as u64;i+=1;
    }
    assert!(quotient<1u64<<35);quotient
}

impl<C: FpConfig, const MIXED: bool> Ifma<C, MIXED> {
    /// Pack normalized values without converting their Montgomery radix.
    /// BN254 operation outputs are below 2p < 2^256, including lazy values.
    #[inline(always)]
    pub fn compact(&self) -> PackedIfma<C, MIXED> {
        // SAFETY: all five limbs are normalized radix-52 digits. Only
        // the low 48 bits of the last limb belong to a 256-bit value.
        unsafe {
            debug_assert_eq!(_mm256_cmpge_epu64_mask(self.0[4], Self::splat(1 << 48)), 0);
            let [t0, t1, t2, t3, t4] = self.0;
            PackedIfma(
                [
                    _mm256_or_si256(t0, _mm256_slli_epi64::<52>(t1)),
                    _mm256_or_si256(_mm256_srli_epi64::<12>(t1), _mm256_slli_epi64::<40>(t2)),
                    _mm256_or_si256(_mm256_srli_epi64::<24>(t2), _mm256_slli_epi64::<28>(t3)),
                    _mm256_or_si256(_mm256_srli_epi64::<36>(t3), _mm256_slli_epi64::<16>(t4)),
                ],
                PhantomData,
            )
        }
    }

    const ONE: [u64; 4] = crate::portable::power_of_two(260, C::P);
    // A downward reciprocal of an upper bound on p's leading radix limb.
    // For x < 16p this estimates floor(x/p) from below by at most one.
    const RECIPROCAL: u64 = (1 << 52) / (split(C::P)[4] + 1);
    #[inline(always)]
    fn splat(x: u64) -> __m256i {
        // SAFETY: this module is compiled only with AVX-512 IFMA/VL.
        unsafe { _mm256_set1_epi64x(x as i64) }
    }
    #[inline(always)]
    fn raw(x: &[[u64; 4]; 4]) -> Self {
        let x = x.map(split);
        Self(
            core::array::from_fn(|j| unsafe {
                _mm256_set_epi64x(
                    x[3][j] as i64,
                    x[2][j] as i64,
                    x[1][j] as i64,
                    x[0][j] as i64,
                )
            }),
            PhantomData,
        )
    }
    #[inline(always)]
    fn constant(x: [u64; 4]) -> Self {
        Self(split(x).map(Self::splat), PhantomData)
    }
    #[inline(always)]
    pub fn new(x: &[Fp<C>; 4]) -> Self {
        if MIXED {
            return Self::raw(&x.map(|v| v.0));
        }
        // Multiply the raw R=2^256 representation by sixteen, then reduce
        // with a one-digit Barrett quotient. No Montgomery product needed.
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let raw = Self::raw(&x.map(|v| v.0)).0;
            let mut t = [zero; 5];
            t[0] = _mm256_and_si256(_mm256_slli_epi64::<4>(raw[0]), mask);
            for j in 1..5 {
                t[j] = _mm256_and_si256(
                    _mm256_or_si256(
                        _mm256_slli_epi64::<4>(raw[j]),
                        _mm256_srli_epi64::<48>(raw[j - 1]),
                    ),
                    mask,
                );
            }
            let q = _mm256_madd52hi_epu64(zero, t[4], Self::splat(Self::RECIPROCAL));
            let p = split(C::P).map(Self::splat);
            let mut carry = zero;
            for j in 0..5 {
                let v =
                    _mm256_add_epi64(_mm256_sub_epi64(t[j], _mm256_mullo_epi64(q, p[j])), carry);
                carry = _mm256_srai_epi64::<52>(v);
                t[j] = _mm256_and_si256(v, mask);
            }
            Self::reduce(t, C::P)
        }
    }
    #[inline(always)]
    pub fn get(&self) -> [Fp<C>; 4] {
        // Divide the raw representation by sixteen modulo p. Choose k<16
        // making x+k*p divisible by 16, then shift; the result is below 2p.
        let x = if MIXED {
            Self::reduce(self.0, C::P)
        } else {
            unsafe {
                let zero = Self::splat(0);
                let mask = Self::splat(MASK);
                let k = _mm256_and_si256(
                    _mm256_mullo_epi64(self.0[0], Self::splat(C::INV)),
                    Self::splat(15),
                );
                let p = split(C::P).map(Self::splat);
                let mut t = [zero; 5];
                let mut carry = zero;
                for j in 0..5 {
                    let v = _mm256_add_epi64(
                        _mm256_add_epi64(self.0[j], _mm256_mullo_epi64(k, p[j])),
                        carry,
                    );
                    carry = _mm256_srli_epi64::<52>(v);
                    t[j] = _mm256_and_si256(v, mask);
                }
                for j in 0..4 {
                    t[j] = _mm256_or_si256(
                        _mm256_srli_epi64::<4>(t[j]),
                        _mm256_and_si256(_mm256_slli_epi64::<48>(t[j + 1]), mask),
                    );
                }
                t[4] = _mm256_srli_epi64::<4>(t[4]);
                Self::reduce(t, C::P)
            }
        };
        let limbs: [[u64; 4]; 5] = x.0.map(|v| unsafe { core::mem::transmute(v) });
        core::array::from_fn(|i| {
            Fp::from_montgomery([
                limbs[0][i] | (limbs[1][i] << 52),
                (limbs[1][i] >> 12) | (limbs[2][i] << 40),
                (limbs[2][i] >> 24) | (limbs[3][i] << 28),
                (limbs[3][i] >> 36) | (limbs[4][i] << 16),
            ])
        })
    }
    #[inline(always)]
    pub fn zero() -> Self {
        Self([Self::splat(0); 5], PhantomData)
    }
    #[inline(always)]
    pub fn one() -> Self {
        Self::constant(if MIXED { C::R } else { Self::ONE })
    }
    #[inline(always)]
    fn reduce(mut t: [__m256i; 5], modulus: [u64; 4]) -> Self {
        unsafe {
            let p = split(modulus).map(Self::splat);
            let mut d = [Self::splat(0); 5];
            let mut borrow = Self::splat(0);
            let mask = Self::splat(MASK);
            for j in 0..5 {
                let v = _mm256_add_epi64(_mm256_sub_epi64(t[j], p[j]), borrow);
                d[j] = _mm256_and_si256(v, mask);
                borrow = _mm256_srai_epi64::<52>(v);
            }
            let keep = _mm256_cmpneq_epi64_mask(borrow, Self::splat(0));
            for j in 0..5 {
                t[j] = _mm256_mask_blend_epi64(keep, d[j], t[j]);
            }
            Self(t, PhantomData)
        }
    }
    /// Skip the fused result's final subtraction when its high limbs
    /// prove every lane is already below 2p. Equal high limbs remain
    /// ambiguous and take the full reduction, preserving the exact result.
    #[inline(always)]
    fn reduce_fused(t: [__m256i; 5]) -> Self {
        let modulus = const { crate::portable::add4(C::P, C::P).0 };
        let high = Self::splat(split(modulus)[4]);
        let needs = unsafe { _mm256_cmpge_epu64_mask(t[4], high) };
        if needs == 0 {
            Self(t, PhantomData)
        } else {
            core::hint::cold_path();
            Self::reduce(t, modulus)
        }
    }
    /// Multiply by ordinary unsigned integers below 2^28, preserving
    /// the input's Montgomery radix. Inputs are normalized and below 2p.
    /// Let R=2^260, mu=floor(R*2^28/p), c=floor(k*mu/2^28),
    /// a4=floor(a/2^208), q=floor(a4*c/2^52). Truncating the low
    /// four limbs adds less than c/2^52 to the quotient error, so
    /// 0<=ak/p-a4*c/2^52 < 1/16+1/131072 < 1. Therefore
    /// 0<=ak-qp < (1+1/16+1/131072)*p < 2p, without correction.
    #[inline(always)]
    pub fn mul_u28(&self, factors:&[u64;LANES])->Self {
        unsafe {
            let zero=Self::splat(0);let mask=Self::splat(MASK);
            let k=_mm256_loadu_si256(factors.as_ptr().cast());
            debug_assert_eq!(_mm256_cmpge_epu64_mask(k,Self::splat(1<<28)),0);
            let mu=const {small_reciprocal(C::P)};
            // k<2^28, mu<2^35: the full product fits below2^63.
            let c=_mm256_srli_epi64::<28>(_mm256_mullo_epi64(k,Self::splat(mu)));
            let q=_mm256_madd52hi_epu64(zero,self.0[4],c);
            let p=split(C::P).map(Self::splat);
            let mut out=[zero;5];let mut carry=zero;
            for j in 0..5 {
                let lo=_mm256_sub_epi64(_mm256_madd52lo_epu64(zero,self.0[j],k),
                    _mm256_madd52lo_epu64(zero,p[j],q));
                let value=_mm256_add_epi64(lo,carry);
                out[j]=_mm256_and_si256(value,mask);
                carry=_mm256_add_epi64(
                    _mm256_sub_epi64(_mm256_madd52hi_epu64(zero,self.0[j],k),
                        _mm256_madd52hi_epu64(zero,p[j],q)),
                    _mm256_srai_epi64::<52>(value));
            }
            debug_assert_eq!(_mm256_cmpneq_epi64_mask(carry,zero),0);
            Self(out,PhantomData)
        }
    }
    #[inline(always)]
    pub fn mul(&self, b: &Self) -> Self {
        self.product::<false>(b, self, b)
    }
    #[inline(always)]
    pub fn sum2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        self.product::<true>(b, c, d)
    }
    #[inline(always)]
    pub fn difference2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        // 2p-c is congruent to -c. It may equal 2p when c=0; it is
        // consumed only by REDC, whose output is again strictly below 2p.
        let negative = unsafe {
            let p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let mask = Self::splat(MASK);
            let mut t = [Self::splat(0); 5];
            let mut carry = Self::splat(0);
            for j in 0..5 {
                let v = _mm256_add_epi64(_mm256_sub_epi64(p[j], c.0[j]), carry);
                carry = _mm256_srai_epi64::<52>(v);
                t[j] = _mm256_and_si256(v, mask);
            }
            Self(t, PhantomData)
        };
        self.product::<true>(b, &negative, d)
    }
    #[inline(always)]
    fn product<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self { #[cfg(zopro_fq_fips)] if LANES==8 && MIXED && C::P==crate::FqConfig::P {return self.product_fips::<FUSED>(b,c,d);}
        // Four-wide transforms benefit from independent product diagonals;
        // eight-wide point batches retain CIOS's smaller register footprint.
        if LANES == 8 {
            return self.product_cios::<FUSED>(b, c, d);
        }
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            // Form the full convolution first. Independent diagonals hide
            // multiplication latency before the Montgomery cancellation chain.
            let mut t = [zero; 10];
            macro_rules! product {
                ($i:literal,$j:literal,$lo:literal,$hi:literal) => {{
                    t[$lo] = _mm256_madd52lo_epu64(t[$lo], self.0[$i], b.0[$j]);
                    t[$hi] = _mm256_madd52hi_epu64(t[$hi], self.0[$i], b.0[$j]);
                    if FUSED {
                        t[$lo] = _mm256_madd52lo_epu64(t[$lo], c.0[$i], d.0[$j]);
                        t[$hi] = _mm256_madd52hi_epu64(t[$hi], c.0[$i], d.0[$j]);
                    }
                }};
            }
            product!(0, 0, 0, 1);
            product!(0, 1, 1, 2);
            product!(1, 0, 1, 2);
            product!(0, 2, 2, 3);
            product!(1, 1, 2, 3);
            product!(2, 0, 2, 3);
            product!(0, 3, 3, 4);
            product!(1, 2, 3, 4);
            product!(2, 1, 3, 4);
            product!(3, 0, 3, 4);
            product!(0, 4, 4, 5);
            product!(1, 3, 4, 5);
            product!(2, 2, 4, 5);
            product!(3, 1, 4, 5);
            product!(4, 0, 4, 5);
            product!(1, 4, 5, 6);
            product!(2, 3, 5, 6);
            product!(3, 2, 5, 6);
            product!(4, 1, 5, 6);
            product!(2, 4, 6, 7);
            product!(3, 3, 6, 7);
            product!(4, 2, 6, 7);
            product!(3, 4, 7, 8);
            product!(4, 3, 7, 8);
            product!(4, 4, 8, 9);
            macro_rules! cancel {
                ($i:literal, $bits:literal) => {{
                    let m = _mm256_and_si256(
                        _mm256_madd52lo_epu64(zero, t[$i], inv),
                        Self::splat((1u64 << $bits) - 1),
                    );
                    macro_rules! limb {
                        ($j:literal) => {{
                            t[$i + $j] = _mm256_madd52lo_epu64(t[$i + $j], m, p[$j]);
                            t[$i + $j + 1] = _mm256_madd52hi_epu64(t[$i + $j + 1], m, p[$j]);
                        }};
                    }
                    limb!(0);
                    limb!(1);
                    limb!(2);
                    limb!(3);
                    limb!(4);
                    if $bits == 52 {
                        t[$i + 1] = _mm256_add_epi64(t[$i + 1], _mm256_srli_epi64::<52>(t[$i]));
                    }
                }};
            }
            cancel!(0, 52);
            cancel!(1, 52);
            cancel!(2, 52);
            cancel!(3, 52);
            let mut out = if MIXED {
                cancel!(4, 48);
                for j in 4..9 {
                    t[j + 1] = _mm256_add_epi64(t[j + 1], _mm256_srli_epi64::<52>(t[j]));
                    t[j] = _mm256_and_si256(t[j], mask);
                }
                core::array::from_fn(|j| {
                    _mm256_or_si256(
                        _mm256_srli_epi64::<48>(t[j + 4]),
                        _mm256_and_si256(_mm256_slli_epi64::<4>(t[j + 5]), mask),
                    )
                })
            } else {
                cancel!(4, 52);
                [t[5], t[6], t[7], t[8], t[9]]
            };
            for j in 0..4 {
                out[j + 1] = _mm256_add_epi64(out[j + 1], _mm256_srli_epi64::<52>(out[j]));
                out[j] = _mm256_and_si256(out[j], mask);
            }
            if FUSED && MIXED {
                Self::reduce_fused(out)
            } else {
                Self(out, PhantomData)
            }
        }
    }
    #[inline(always)]
    fn product_cios<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {
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
                            t[$j] = _mm256_madd52lo_epu64(t[$j], self.0[$j], b.0[$i]);
                            t[$k] = _mm256_madd52hi_epu64(t[$k], self.0[$j], b.0[$i]);
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
                                t[$j] = _mm256_madd52lo_epu64(t[$j], c.0[$j], d.0[$i]);
                                t[$k] = _mm256_madd52hi_epu64(t[$k], c.0[$j], d.0[$i]);
                            };
                        }
                        second!(0, 1);
                        second!(1, 2);
                        second!(2, 3);
                        second!(3, 4);
                        second!(4, 5);
                    }
                    let m = _mm256_and_si256(_mm256_madd52lo_epu64(zero, t[0], inv), mask);
                    macro_rules! reduction {
                        ($j:literal,$k:literal) => {
                            t[$j] = _mm256_madd52lo_epu64(t[$j], m, p[$j]);
                            t[$k] = _mm256_madd52hi_epu64(t[$k], m, p[$j]);
                        };
                    }
                    if cfg!(zopro_cios_cancel_carry) {
                        // The cancelled low digit contributes ceil(t0/B).
                        // Compute it without waiting for m*p0's low half.
                        let carry = _mm256_srli_epi64::<52>(_mm256_add_epi64(t[0], mask));
                        let low = _mm256_madd52hi_epu64(carry, m, p[0]);
                        reduction!(1, 2);
                        reduction!(2, 3);
                        reduction!(3, 4);
                        reduction!(4, 5);
                        t[1] = _mm256_add_epi64(t[1], low);
                    } else {
                        reduction!(0, 1);
                        reduction!(1, 2);
                        reduction!(2, 3);
                        reduction!(3, 4);
                        reduction!(4, 5);
                        t[1] = _mm256_add_epi64(t[1], _mm256_srli_epi64::<52>(t[0]));
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
                        t[$j] = _mm256_madd52lo_epu64(t[$j], self.0[$j], b.0[4]);
                        t[$k] = _mm256_madd52hi_epu64(t[$k], self.0[$j], b.0[4]);
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
                            t[$j] = _mm256_madd52lo_epu64(t[$j], c.0[$j], d.0[4]);
                            t[$k] = _mm256_madd52hi_epu64(t[$k], c.0[$j], d.0[4]);
                        };
                    }
                    second!(0, 1);
                    second!(1, 2);
                    second!(2, 3);
                    second!(3, 4);
                    second!(4, 5);
                }
                let m = _mm256_and_si256(
                    _mm256_madd52lo_epu64(zero, t[0], inv),
                    Self::splat((1 << 48) - 1),
                );
                macro_rules! reduction {
                    ($j:literal,$k:literal) => {
                        t[$j] = _mm256_madd52lo_epu64(t[$j], m, p[$j]);
                        t[$k] = _mm256_madd52hi_epu64(t[$k], m, p[$j]);
                    };
                }
                if cfg!(zopro_cios_cancel_carry) {
                    let mask48 = Self::splat((1u64 << 48) - 1);
                    let carry = _mm256_srli_epi64::<48>(_mm256_add_epi64(t[0], mask48));
                    // m<2^48, so shifting by4 remains an exact IFMA input.
                    let low = _mm256_madd52hi_epu64(carry, _mm256_slli_epi64::<4>(m), p[0]);
                    reduction!(1, 2);
                    reduction!(2, 3);
                    reduction!(3, 4);
                    reduction!(4, 5);
                    // Divide the unnormalized coefficients by2^48 directly.
                    // Splitting each higher coefficient at48 bits avoids
                    // the original extra normalization before radix packing.
                    t = [
                        _mm256_add_epi64(low, _mm256_slli_epi64::<4>(_mm256_and_si256(t[1], mask48))),
                        _mm256_add_epi64(_mm256_srli_epi64::<48>(t[1]), _mm256_slli_epi64::<4>(_mm256_and_si256(t[2], mask48))),
                        _mm256_add_epi64(_mm256_srli_epi64::<48>(t[2]), _mm256_slli_epi64::<4>(_mm256_and_si256(t[3], mask48))),
                        _mm256_add_epi64(_mm256_srli_epi64::<48>(t[3]), _mm256_slli_epi64::<4>(_mm256_and_si256(t[4], mask48))),
                        _mm256_add_epi64(_mm256_srli_epi64::<48>(t[4]), _mm256_slli_epi64::<4>(t[5])),
                        zero,
                    ];
                } else {
                reduction!(0, 1);
                reduction!(1, 2);
                reduction!(2, 3);
                reduction!(3, 4);
                reduction!(4, 5);
                for j in 0..5 {
                    t[j + 1] = _mm256_add_epi64(t[j + 1], _mm256_srli_epi64::<52>(t[j]));
                    t[j] = _mm256_and_si256(t[j], mask);
                }
                for j in 0..5 {
                    t[j] = _mm256_or_si256(
                        _mm256_srli_epi64::<48>(t[j]),
                        _mm256_and_si256(_mm256_slli_epi64::<4>(t[j + 1]), mask),
                    );
                }
                }
            } else {
                row!(4);
            }
            for j in 0..4 {
                t[j + 1] = _mm256_add_epi64(t[j + 1], _mm256_srli_epi64::<52>(t[j]));
                t[j] = _mm256_and_si256(t[j], mask);
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
    /// Inputs must each be below p. The unreduced sum remains below 2p.
    #[inline(always)]
    pub fn add_canonical(&self, b: &Self) -> Self {
        unsafe {
            let mask = Self::splat(MASK);
            let mut t = core::array::from_fn(|j| _mm256_add_epi64(self.0[j], b.0[j]));
            for j in 0..4 {
                t[j + 1] = _mm256_add_epi64(t[j + 1], _mm256_srli_epi64::<52>(t[j]));
                t[j] = _mm256_and_si256(t[j], mask);
            }
            Self(t, PhantomData)
        }
    }
    /// Inputs must each be below p. p+a-b is nonnegative and below 2p.
    #[inline(always)]
    pub fn sub_canonical(&self, b: &Self) -> Self {
        unsafe {
            let p = split(C::P).map(Self::splat);
            let mask = Self::splat(MASK);
            let mut t = [Self::splat(0); 5];
            let mut carry = Self::splat(0);
            for j in 0..5 {
                let v = _mm256_add_epi64(
                    _mm256_sub_epi64(_mm256_add_epi64(p[j], self.0[j]), b.0[j]),
                    carry,
                );
                carry = _mm256_srai_epi64::<52>(v);
                t[j] = _mm256_and_si256(v, mask);
            }
            Self(t, PhantomData)
        }
    }
    #[inline(always)]
    pub fn add(&self, b: &Self) -> Self {
        unsafe {
            let mask = Self::splat(MASK);
            let mut t = core::array::from_fn(|j| _mm256_add_epi64(self.0[j], b.0[j]));
            for j in 0..4 {
                t[j + 1] = _mm256_add_epi64(t[j + 1], _mm256_srli_epi64::<52>(t[j]));
                t[j] = _mm256_and_si256(t[j], mask);
            }
            Self::reduce(t, crate::portable::add4(C::P, C::P).0)
        }
    }
    #[inline(always)]
    pub fn sub(&self, b: &Self) -> Self {
        unsafe {
            let p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let mask = Self::splat(MASK);
            let mut t = [Self::splat(0); 5];
            let mut borrow = Self::splat(0);
            for j in 0..5 {
                let v = _mm256_add_epi64(_mm256_sub_epi64(self.0[j], b.0[j]), borrow);
                t[j] = _mm256_and_si256(v, mask);
                borrow = _mm256_srai_epi64::<52>(v);
            }
            let negative = _mm256_cmpneq_epi64_mask(borrow, Self::splat(0));
            for j in 0..5 {
                t[j] = _mm256_add_epi64(t[j], _mm256_maskz_mov_epi64(negative, p[j]));
            }
            for j in 0..4 {
                t[j + 1] = _mm256_add_epi64(t[j + 1], _mm256_srli_epi64::<52>(t[j]));
                t[j] = _mm256_and_si256(t[j], mask);
            }
            t[4] = _mm256_and_si256(t[4], mask);
            Self(t, PhantomData)
        }
    }
    #[inline(always)]
    pub fn shuffle<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        a: &Self,
        b: &Self,
    ) -> Self {
        unsafe {
            let idx = _mm256_set_epi64x(I3 as i64, I2 as i64, I1 as i64, I0 as i64);
            Self(
                core::array::from_fn(|j| _mm256_permutex2var_epi64(a.0[j], idx, b.0[j])),
                PhantomData,
            )
        }
    }
}


impl<C:FpConfig,const MIXED:bool> Ifma<C,MIXED> {
    /// (sign*by-ay)*scale, for canonical by/ay and scale<2p, mixed R256.
    /// The transient numerator is <=2p (the upper edge is negative zero).
    /// For BN254 p<2^254, REDC(2p*2p)<p+4p^2/2^256<2p. It never escapes
    /// before multiplication; normalized radix limbs meet the same carry bounds.
    #[inline(always)]
    pub fn signed_difference_scaled(&self, ay:&Self, scale:&Self, negative:u8)->Self {
        assert!(MIXED);assert!(C::P[3]<(1u64<<62));
        unsafe {
            let p=split(C::P);let twice=split(crate::portable::add4(C::P,C::P).0);
            let flip=_mm256_maskz_mov_epi64(negative,Self::splat(MASK));
            let mut carry=_mm256_maskz_mov_epi64(negative,Self::splat(1));
            let mut t=[Self::splat(0);5];let mask=Self::splat(MASK);
            for j in 0..5 {
                let offset=_mm256_mask_blend_epi64(negative,Self::splat(p[j]),Self::splat(twice[j]));
                let signed=_mm256_xor_si256(self.0[j],flip);
                let v=_mm256_add_epi64(_mm256_sub_epi64(_mm256_add_epi64(offset,signed),ay.0[j]),carry);
                t[j]=_mm256_and_si256(v,mask);carry=_mm256_srai_epi64::<52>(v);
            }
            Self(t,PhantomData).mul(scale)
        }
    }
}

#[cfg(zopro_fq_fips)]
impl<C: FpConfig, const MIXED: bool> Ifma<C, MIXED> {
    #[inline(always)]
    fn product_fips<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let mask48 = Self::splat((1u64 << 48) - 1);
            let inv = Self::splat(C::INV & MASK);
            let p = split(C::P).map(Self::splat);
            let mut q = [zero; 5];
            let mut out = [zero; 5];
            let mut carry = zero;
            macro_rules! raw {
                ($lo:ident, $hi:ident, $i:literal, $j:literal) => {
                    $lo = _mm256_madd52lo_epu64($lo, self.0[$i], b.0[$j]);
                    $hi = _mm256_madd52hi_epu64($hi, self.0[$i], b.0[$j]);
                    if FUSED {
                        $lo = _mm256_madd52lo_epu64($lo, c.0[$i], d.0[$j]);
                        $hi = _mm256_madd52hi_epu64($hi, c.0[$i], d.0[$j]);
                    }
                };
            }
            macro_rules! reduction {
                ($lo:ident, $hi:ident, $i:literal, $j:literal) => {
                    $lo = _mm256_madd52lo_epu64($lo, q[$i], p[$j]);
                    $hi = _mm256_madd52hi_epu64($hi, q[$i], p[$j]);
                };
            }
            macro_rules! low {
                ($k:literal, $bits:literal; $(($ai:literal,$bi:literal)),*; $(($qi:literal,$pi:literal)),*) => {{
                    let mut lo = carry;
                    let mut hi = zero;
                    $(raw!(lo, hi, $ai, $bi);)*
                    $(reduction!(lo, hi, $qi, $pi);)*
                    q[$k] = _mm256_and_si256(
                        _mm256_madd52lo_epu64(zero, lo, inv),
                        if $bits == 48 { mask48 } else { mask },
                    );
                    if $bits == 48 {
                        let upper = _mm256_srli_epi64::<48>(_mm256_add_epi64(lo, mask48));
                        let quotient = _mm256_madd52hi_epu64(upper, _mm256_slli_epi64::<4>(q[$k]), p[0]);
                        carry = _mm256_add_epi64(quotient, _mm256_slli_epi64::<4>(hi));
                    } else {
                        let upper = _mm256_srli_epi64::<52>(_mm256_add_epi64(lo, mask));
                        carry = _mm256_add_epi64(_mm256_madd52hi_epu64(upper, q[$k], p[0]), hi);
                    }
                }};
            }
            low!(0,52; (0,0); );
            low!(1,52; (0,1),(1,0); (0,1));
            low!(2,52; (0,2),(1,1),(2,0); (0,2),(1,1));
            low!(3,52; (0,3),(1,2),(2,1),(3,0); (0,3),(1,2),(2,1));
            low!(4,48; (0,4),(1,3),(2,2),(3,1),(4,0); (0,4),(1,3),(2,2),(3,1));
            macro_rules! high {
                ($o:literal; $(($i:literal,$j:literal)),*) => {{
                    let mut lo = zero;
                    let mut hi = zero;
                    $(raw!(lo, hi, $i, $j); reduction!(lo, hi, $i, $j);)*
                    let wide = _mm256_add_epi64(carry, _mm256_slli_epi64::<4>(lo));
                    out[$o] = _mm256_and_si256(wide, mask);
                    carry = _mm256_add_epi64(_mm256_srli_epi64::<52>(wide), _mm256_slli_epi64::<4>(hi));
                }};
            }
            high!(0; (1,4),(2,3),(3,2),(4,1));
            high!(1; (2,4),(3,3),(4,2));
            high!(2; (3,4),(4,3));
            high!(3; (4,4));
            out[4] = carry;
            if FUSED { Self::reduce_fused(out) } else { Self(out, PhantomData) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FqConfig, FrConfig};
    fn check<C: FpConfig, const MIXED: bool>() {
        let mut seed = 0x123456789abcdefu64;
        let mut random = || {
            let mut bytes = [0; 64];
            for chunk in bytes.chunks_mut(8) {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                chunk.copy_from_slice(&seed.to_le_bytes());
            }
            Fp::<C>::from_random_bytes(&bytes)
        };
        let edges = [
            Fp::<C>::ZERO,
            Fp::<C>::ONE,
            -Fp::<C>::ONE,
            Fp::<C>::from_u64(2),
            Fp::from_montgomery(crate::portable::sub4(C::P, [1, 0, 0, 0]).0),
            Fp::from_montgomery(crate::portable::sub4(C::P, [2, 0, 0, 0]).0),
            Fp::from_montgomery([u64::MAX, u64::MAX, u64::MAX, C::P[3] - 1]),
            Fp::from_montgomery([u64::MAX, u64::MAX, u64::MAX, 0]),
        ];
        let mut a = core::array::from_fn(|i| edges[i]);
        for round in 0..512 {
            let b = core::array::from_fn(|i| {
                if round < edges.len() {
                    edges[(round + i) % edges.len()]
                } else {
                    random()
                }
            });
            let (av, bv) = (Ifma::<C, MIXED>::new(&a), Ifma::<C, MIXED>::new(&b));
            assert_eq!(av.get(), a);
            assert_eq!(av.mul(&bv).get(), core::array::from_fn(|i| a[i] * b[i]));
            assert_eq!(
                av.sum2(&bv, &bv, &av).get(),
                core::array::from_fn(|i| a[i] * b[i] + b[i] * a[i])
            );
            let v = av.sub(&bv);
            let w = bv.add(&av);
            assert_eq!(
                v.mul(&w).get(),
                core::array::from_fn(|i| (a[i] - b[i]) * (b[i] + a[i]))
            );
            assert_eq!(
                v.sum2(&w, &w, &v).get(),
                core::array::from_fn(|i| ((a[i] - b[i]) * (b[i] + a[i])).double())
            );
            assert_eq!(
                av.difference2(&av, &bv, &bv).get(),
                core::array::from_fn(|i| a[i] * a[i] - b[i] * b[i])
            );
            assert_eq!(v.difference2(&w, &w, &v).get(), [Fp::<C>::ZERO; 4]);
            assert_eq!(av.add(&bv).get(), core::array::from_fn(|i| a[i] + b[i]));
            assert_eq!(av.sub(&bv).get(), core::array::from_fn(|i| a[i] - b[i]));
            assert_eq!(
                av.add_canonical(&bv).get(),
                core::array::from_fn(|i| a[i] + b[i])
            );
            assert_eq!(
                av.sub_canonical(&bv).get(),
                core::array::from_fn(|i| a[i] - b[i])
            );
            assert_eq!(
                Ifma::shuffle::<0, 4, 2, 6>(&av, &bv).get(),
                [a[0], b[0], a[2], b[2]]
            );
            assert_eq!(av.mul(&Ifma::one()).get(), a);
            assert_eq!(av.sub(&av).get(), [Fp::<C>::ZERO; 4]);
            a = b;
        }
        let a = Ifma::<C, MIXED>::new(&[-Fp::<C>::ONE; 4]);
        assert_eq!(a.mul(&a).get(), [Fp::<C>::ONE; 4]);
        assert_eq!(Ifma::<C, MIXED>::zero().get(), [Fp::<C>::ZERO; 4]);
    }
    fn check_fused_reduction<C: FpConfig>() {
        let twice = crate::portable::add4(C::P, C::P).0;
        let four = crate::portable::add4(twice, twice).0;
        let high_boundary = [0, 0, 0, split(twice)[4] << 16];
        let boundaries = [
            [0; 4],
            C::P,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            twice,
            crate::portable::add4(twice, [1, 0, 0, 0]).0,
            crate::portable::sub4(four, [1, 0, 0, 0]).0,
            crate::portable::sub4(high_boundary, [1, 0, 0, 0]).0,
            high_boundary,
            crate::portable::add4(high_boundary, [1, 0, 0, 0]).0,
        ];
        let limit = split(twice);
        for round in 0..boundaries.len() * 2 {
            let values = core::array::from_fn(|i| {
                boundaries[if round < boundaries.len() {
                    round
                } else {
                    (round + i) % boundaries.len()
                }]
            });
            let raw = Ifma::<C, true>::raw(&values);
            let expected = Ifma::<C, true>::reduce(raw.0, twice);
            let actual = Ifma::<C, true>::reduce_fused(raw.0);
            let expected: [[u64; LANES]; 5] =
                expected.0.map(|x| unsafe { core::mem::transmute(x) });
            let actual: [[u64; LANES]; 5] = actual.0.map(|x| unsafe { core::mem::transmute(x) });
            assert_eq!(actual, expected);
            for lane in 0..LANES {
                for j in 0..5 {
                    assert!(actual[j][lane] < 1 << 52);
                }
                assert!((0..5)
                    .rev()
                    .map(|j| actual[j][lane])
                    .cmp((0..5).rev().map(|j| limit[j]))
                    .is_lt());
            }
        }
    }

    #[test]
    fn signed_difference_scaled_matches_arkworks() {
        fn check<C:FpConfig,A:ark_ff::PrimeField>() {
            use ark_ff::Field;
            let inv_r=A::from(2u64).pow([256]).inverse().unwrap();
            let convert=|v:Fp<C>|A::from_le_bytes_mod_order(&v.0.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())*inv_r;
            let p=C::P;let twice=crate::portable::add4(p,p).0;
            let canonical=[[0;4],C::R,crate::portable::sub4(p,[1,0,0,0]).0,crate::portable::sub4(p,[2,0,0,0]).0];
            let scales=[[0;4],p,crate::portable::sub4(twice,[1,0,0,0]).0,C::R];
            let mut seed=0x8123023fe913u64;
            for round in 0..512 {
                let mut random=||{let mut bytes=[0;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}Fp::<C>::from_random_bytes(&bytes)};
                let by=core::array::from_fn(|i|if round<32 {Fp::from_montgomery(canonical[(round+i)%4])}else{random()});
                let ay=core::array::from_fn(|i|if round<32 {Fp::from_montgomery(canonical[(round/4+i)%4])}else{random()});
                let scale=core::array::from_fn(|i|if round<32 {Fp::from_montgomery(scales[(round/8+i)%4])}else{random()});
                let negative=(round as u8).wrapping_mul(37);
                let got=Ifma::<C,true>::new(&by).signed_difference_scaled(&Ifma::new(&ay),&Ifma::new(&scale),negative).get();
                for i in 0..LANES {let y=convert(by[i]);let want=((if negative>>i&1!=0{-y}else{y})-convert(ay[i]))*convert(scale[i]);assert!(crate::portable::less(got[i].0,p));assert_eq!(convert(got[i]),want);}
            }
            // All lanes exercise the exact 2p transient numerator boundary.
            let zero=Ifma::<C,true>::zero();let raw=core::array::from_fn(|_|Fp::<C>::from_montgomery(crate::portable::sub4(twice,[1,0,0,0]).0));
            assert_eq!(zero.signed_difference_scaled(&zero,&Ifma::new(&raw),255).get(),[Fp::<C>::ZERO;LANES]);
        }
        check::<crate::FqConfig,ark_ref::Fq>();check::<crate::FrConfig,ark_ref::Fr>();
    }

    #[test]
    fn compact_storage_roundtrip() {
        fn check<C: FpConfig, const MIXED: bool>() {
            assert_eq!(core::mem::size_of::<PackedIfma<C, MIXED>>(), 4 * LANES * 8);
            let twice = crate::portable::add4(C::P, C::P).0;
            let mut edges = vec![
                [0; 4],
                [1, 0, 0, 0],
                crate::portable::sub4(C::P, [1, 0, 0, 0]).0,
                C::P,
                crate::portable::add4(C::P, [1, 0, 0, 0]).0,
                crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            ];
            for bit in 0..254 {
                let mut words = [0; 4];
                words[bit / 64] = 1u64 << (bit % 64);
                edges.push(words);
            }
            let mut seed = 0x82718dcfad29u64;
            for _ in 0..128 {
                let mut bytes = [0; 64];
                for chunk in bytes.chunks_mut(8) {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    chunk.copy_from_slice(&seed.to_le_bytes());
                }
                let mut words = Fp::<C>::from_random_bytes(&bytes).0;
                if seed & 1 != 0 {
                    words = crate::portable::add4(words, C::P).0;
                }
                edges.push(words);
            }
            for first in 0..edges.len() {
                let words = core::array::from_fn(|i| edges[(first + i) % edges.len()]);
                let raw = Ifma::<C, MIXED>::raw(&words);
                let packed = raw.compact();
                let columns: [[u64; LANES]; 4] =
                    packed.0.map(|v| unsafe { core::mem::transmute(v) });
                for lane in 0..LANES {
                    for j in 0..4 {
                        assert_eq!(columns[j][lane], words[lane][j]);
                    }
                }
                let expanded = packed.expand();
                let before: [[u64; LANES]; 5] = raw.0.map(|v| unsafe { core::mem::transmute(v) });
                let after: [[u64; LANES]; 5] =
                    expanded.0.map(|v| unsafe { core::mem::transmute(v) });
                assert_eq!(before, after);
                assert_eq!(expanded.get(), raw.get());
                assert_eq!(expanded.mul(&expanded).get(), raw.mul(&raw).get());
                assert_eq!(expanded.add(&expanded).get(), raw.add(&raw).get());
            }
        }
        check::<FqConfig, false>();
        check::<FrConfig, false>();
        check::<FqConfig, true>();
        check::<FrConfig, true>();
    }

    #[test]
    fn fused_reduction_boundaries() {
        check_fused_reduction::<FqConfig>();
        check_fused_reduction::<FrConfig>();
    }
    #[test]
    fn fq() {
        check::<FqConfig, false>();
    }
    #[test]
    fn fr() {
        check::<FrConfig, false>();
    }
    #[test]
    fn fq_mixed() {
        check::<FqConfig, true>();
    }
    #[test]
    fn fr_mixed() {
        check::<FrConfig, true>();
    }
    #[test]
    fn small_integer_multiply_matches_arkworks() {
        fn check<C:FpConfig,A:ark_ff::PrimeField,const MIXED:bool>() {
            use ark_ff::Field;
            let input_inverse=A::from(2u64).pow([if MIXED {256}else{260}]).inverse().unwrap();
            let output_inverse=A::from(2u64).pow([256]).inverse().unwrap();
            let numeric=|v:[u64;4]| A::from_le_bytes_mod_order(&v.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());
            let p=C::P;let twice=crate::portable::add4(p,p).0;
            let mut edges=vec![[0;4],[1,0,0,0],p,crate::portable::sub4(p,[1,0,0,0]).0,
                crate::portable::add4(p,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0];
            for bit in [51,52,103,104,155,156,207,208,252,253] {
                let mut v=[0u64;4];v[bit/64]=1u64<<(bit%64);
                edges.extend([crate::portable::sub4(v,[1,0,0,0]).0,v,crate::portable::add4(v,[1,0,0,0]).0]);
            }
            let mut factors=vec![0u64,1,2,3,(1<<28)-1];
            for bit in 1..28 {factors.extend([(1<<bit)-1,1<<bit,(1<<bit)+1]);}
            let mut pairs=Vec::new();for &a in &edges {for &k in &factors {pairs.push((a,k));}}
            let mut seed=0x6a2c382713f99u64;
            let mut next=||{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed};
            for round in 0..4096 {
                let mut bytes=[0u8;64];for chunk in bytes.chunks_mut(8) {chunk.copy_from_slice(&next().to_le_bytes());}
                let a=Fp::<C>::from_random_bytes(&bytes).0;
                pairs.push((if round%2==0 {a}else{crate::portable::add4(a,p).0},next()&((1<<28)-1)));
            }
            let limit=split(twice);let mut checked=0;
            for chunk in pairs.chunks(LANES) {
                let v=core::array::from_fn(|i|chunk[i%chunk.len()].0);
                let k:[u64;LANES]=core::array::from_fn(|i|chunk[i%chunk.len()].1);
                let mut got=Ifma::<C,MIXED>::raw(&v);
                let mut expected: [A;LANES]=v.map(|a|numeric(a)*input_inverse);
                // Reusing lazy outputs also checks closure of the <2p bound.
                for round in 0..8 {
                    let factors=core::array::from_fn(|i|(k[i]+round*1597)&((1<<28)-1));
                    got=got.mul_u28(&factors);
                    let words:[[u64;LANES];5]=got.0.map(|x|unsafe {core::mem::transmute(x)});
                    let host=got.get();
                    for lane in 0..LANES {
                        expected[lane]*=A::from(factors[lane]);
                        assert_eq!(numeric(host[lane].0)*output_inverse,expected[lane]);
                        assert!(crate::portable::less(host[lane].0,p));
                        for j in 0..5 {assert!(words[j][lane] < 1u64<<52);}
                        assert!((0..5).rev().map(|j|words[j][lane]).cmp((0..5).rev().map(|j|limit[j])).is_lt());
                    }
                    checked+=LANES;
                }
            }
            eprintln!("EULER_SMALL_FIELD_LANES={checked}");
        }
        check::<crate::FqConfig,ark_ref::Fq,false>();
        check::<crate::FqConfig,ark_ref::Fq,true>();
        check::<crate::FrConfig,ark_ref::Fr,false>();
        check::<crate::FrConfig,ark_ref::Fr,true>();
    }

    #[test]
    fn cios_carry_matches_arkworks() {
        fn check<C:FpConfig,A:ark_ff::PrimeField,const MIXED:bool>() {
            use ark_ff::Field;
            let inv=A::from(2u64).pow([if MIXED {256}else{260}]).inverse().unwrap();
            let out_inv=A::from(2u64).pow([256]).inverse().unwrap();
            let numeric=|v:[u64;4]| {
                let mut bytes=[0u8;32];
                for (chunk,word) in bytes.chunks_mut(8).zip(v) {chunk.copy_from_slice(&word.to_le_bytes());}
                A::from_le_bytes_mod_order(&bytes)
            };
            let p=C::P;let twice=crate::portable::add4(p,p).0;
            let mut edges=vec![[0;4],[1,0,0,0],[2,0,0,0],p,crate::portable::sub4(p,[1,0,0,0]).0,
                crate::portable::add4(p,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0];
            for bit in [48,52,104,156,208,253] {
                let mut v=[0u64;4];v[bit/64]=1u64<<(bit%64);
                edges.extend([crate::portable::sub4(v,[1,0,0,0]).0,v,crate::portable::add4(v,[1,0,0,0]).0]);
            }
            assert!(edges.iter().all(|v|crate::portable::less(*v,twice)));
            let mut inputs=Vec::new();
            for (i,&a) in edges.iter().enumerate() {for (j,&b) in edges.iter().enumerate() {
                inputs.push([a,b,if (i+j)%7==0 {twice}else{edges[(i*13+j)%edges.len()]},edges[(i+j*17)%edges.len()]]);
            }}
            let mut seed=0x43494f5343415252u64;
            for round in 0..4096 {
                let mut tuple=[[0u64;4];4];
                for (j,value) in tuple.iter_mut().enumerate() {
                    let mut bytes=[0u8;64];
                    for chunk in bytes.chunks_mut(8) {seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}
                    let v=Fp::<C>::from_random_bytes(&bytes).0;
                    *value=if (round+j)%2==0 {v}else{crate::portable::add4(v,p).0};
                }
                inputs.push(tuple);
            }
            let limit=split(twice);let mut checked=0usize;
            for chunk in inputs.chunks(LANES) {
                let words:[[[u64;4];LANES];4]=core::array::from_fn(|j|core::array::from_fn(|i|chunk[i%chunk.len()][j]));
                let v=words.map(|x|Ifma::<C,MIXED>::raw(&x));
                let a:[A;LANES]=words[0].map(|x|numeric(x)*inv);
                let b:[A;LANES]=words[1].map(|x|numeric(x)*inv);
                let c:[A;LANES]=words[2].map(|x|numeric(x)*inv);
                let d:[A;LANES]=words[3].map(|x|numeric(x)*inv);
                let mut verify=|got:Ifma<C,MIXED>,want:[A;LANES]| {
                    let raw:[[u64;LANES];5]=got.0.map(|x|unsafe {core::mem::transmute(x)});
                    let host=got.get();
                    for i in 0..LANES {
                        assert_eq!(numeric(host[i].0)*out_inv,want[i]);
                        assert!(crate::portable::less(host[i].0,p));
                        for j in 0..5 {assert!(raw[j][i]<1u64<<52);}
                        assert!((0..5).rev().map(|j|raw[j][i]).cmp((0..5).rev().map(|j|limit[j])).is_lt());
                    }
                    checked+=LANES;
                };
                verify(v[0].product_cios::<false>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]));
                verify(v[0].product_cios::<true>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]+c[i]*d[i]));
                verify(v[0].difference2(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]-c[i]*d[i]));
                #[cfg(zopro_fq_fips)]
                if LANES==8 && MIXED && C::P==crate::FqConfig::P {
                    verify(v[0].product_fips::<false>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]));
                    verify(v[0].product_fips::<true>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]+c[i]*d[i]));
                    let mut fips_chain=v[0];let mut fips_want=a;
                    for step in 0..16 {
                        if step%2==0 {
                            fips_chain=fips_chain.product_fips::<false>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]*=b[i];}
                        } else {
                            fips_chain=fips_chain.product_fips::<true>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]=fips_want[i]*b[i]+c[i]*d[i];}
                        }
                        verify(fips_chain,fips_want);
                    }
                }

                let mut chain=v[0];let mut expected=a;
                for step in 0..8 {
                    if step%2==0 {
                        chain=chain.product_cios::<false>(&v[1],&v[2],&v[3]);
                        for i in 0..LANES {expected[i]*=b[i];}
                    } else {
                        chain=chain.product_cios::<true>(&v[1],&v[2],&v[3]);
                        for i in 0..LANES {expected[i]=expected[i]*b[i]+c[i]*d[i];}
                    }
                    verify(chain,expected);
                }
            }
            eprintln!("CIOS_CARRY_FIELD_LANES={checked}");
        }
        check::<crate::FqConfig,ark_ref::Fq,false>();check::<crate::FqConfig,ark_ref::Fq,true>();
        check::<crate::FrConfig,ark_ref::Fr,false>();check::<crate::FrConfig,ark_ref::Fr,true>();
    }

}

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

impl<C: FpConfig, const MIXED: bool> Ifma<C, MIXED> {
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
    fn product<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {
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
                Self::reduce(out, crate::portable::add4(C::P, C::P).0)
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
                    reduction!(0, 1);
                    reduction!(1, 2);
                    reduction!(2, 3);
                    reduction!(3, 4);
                    reduction!(4, 5);
                    t[1] = _mm256_add_epi64(t[1], _mm256_srli_epi64::<52>(t[0]));
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
                Self::reduce(result, crate::portable::add4(C::P, C::P).0)
            } else {
                Self(result, PhantomData)
            }
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
}

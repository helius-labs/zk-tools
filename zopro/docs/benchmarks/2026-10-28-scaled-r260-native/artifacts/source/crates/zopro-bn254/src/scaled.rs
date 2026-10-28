//! Bounded experimental operations for coordinate-scaled R260 G2.
//! Bounds are inclusive numerical multiples of p; all digits normalized.
use super::{Ifma8, MASK};
use crate::{FpConfig, Fq, FqConfig};
use core::{arch::x86_64::*, marker::PhantomData};
type Raw = Ifma8<FqConfig, false>;
#[derive(Clone, Copy)]
pub struct Wide<const B: u64>(Raw);
#[inline(always)]
fn splat(x: u64) -> __m512i {
    unsafe { _mm512_set1_epi64(x as i64) }
}
const fn modulus(n: u64) -> [u64; 5] {
    let p = super::split(FqConfig::P);
    let mut out = [0; 5];
    let mut carry = 0u128;
    let mut j = 0;
    while j < 5 {
        let t = p[j] as u128 * n as u128 + carry;
        out[j] = t as u64 & MASK;
        carry = t >> 52;
        j += 1;
    }
    assert!(carry == 0);
    out
}
fn valid<const B: u64>(raw: &Raw) -> bool {
    let a: [[u64; 8]; 5] = unsafe { core::mem::transmute(raw.0) };
    let p = modulus(B);
    for lane in 0..8 {
        let mut cmp = core::cmp::Ordering::Equal;
        for j in (0..5).rev() {
            if a[j][lane] > MASK {
                return false;
            }
            if cmp == core::cmp::Ordering::Equal {
                cmp = a[j][lane].cmp(&p[j]);
            }
        }
        if cmp.is_gt() {
            return false;
        }
    }
    true
}
#[inline(always)]
fn bounded<const B: u64>(raw: Raw) -> Wide<B> {
    debug_assert!(valid::<B>(&raw));
    Wide(raw)
}
impl<const B: u64> Wide<B> {
    #[inline(always)]
    fn widen<const N: u64>(&self) -> Wide<N> {
        const { assert!(N >= B) };
        bounded(self.0)
    }
}
#[inline(always)]
fn shifted<const K: u32, const B: u64>(x: &[Fq; 8]) -> Wide<B> {
    const { assert!(K == 1 || K == 2) };
    unsafe {
        let a = Ifma8::<FqConfig, true>::new(x).0;
        let mask = splat(MASK);
        let mut out = [splat(0); 5];
        out[0] = _mm512_and_si512(_mm512_slli_epi64::<K>(a[0]), mask);
        for j in 1..5 {
            let carry = if K == 1 {
                _mm512_srli_epi64::<51>(a[j - 1])
            } else {
                _mm512_srli_epi64::<50>(a[j - 1])
            };
            out[j] = _mm512_and_si512(_mm512_or_si512(_mm512_slli_epi64::<K>(a[j]), carry), mask);
        }
        bounded(Ifma8(out, PhantomData))
    }
}
#[inline(always)]
pub fn import_x(x: &[Fq; 8]) -> Wide<4> {
    shifted::<2, 4>(x)
}
#[inline(always)]
pub fn import_y(y: &[Fq; 8]) -> Wide<2> {
    shifted::<1, 2>(y)
}
// K is only1/2; use explicit branches to keep stable const generics.
#[inline(always)]
fn divide<const K: u32, const B: u64>(x: &Wide<B>) -> [Fq; 8] {
    const { assert!(K == 1 || K == 2) };
    const { assert!(FqConfig::P[0] & 3 == 3) };
    unsafe {
        let mask = splat(MASK);
        let p = modulus(1).map(splat);
        let m = _mm512_and_si512(x.0 .0[0], splat((1u64 << K) - 1));
        let mut out = [splat(0); 5];
        let mut carry = splat(0);
        for j in 0..5 {
            let t = _mm512_add_epi64(
                _mm512_add_epi64(x.0 .0[j], _mm512_mullo_epi64(m, p[j])),
                carry,
            );
            out[j] = _mm512_and_si512(t, mask);
            carry = _mm512_srli_epi64::<52>(t);
        }
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry, splat(0)), 0);
        for j in 0..4 {
            out[j] = if K == 1 {
                _mm512_or_si512(
                    _mm512_srli_epi64::<1>(out[j]),
                    _mm512_and_si512(_mm512_slli_epi64::<51>(out[j + 1]), mask),
                )
            } else {
                _mm512_or_si512(
                    _mm512_srli_epi64::<2>(out[j]),
                    _mm512_and_si512(_mm512_slli_epi64::<50>(out[j + 1]), mask),
                )
            };
        }
        out[4] = _mm512_srli_epi64::<K>(out[4]);
        let reduced = Raw::reduce(out, crate::portable::add4(FqConfig::P, FqConfig::P).0);
        // After division <4p<2^256; first subtract2p, then mixed get subtracts p.
        Ifma8::<FqConfig, true>(reduced.0, PhantomData).get()
    }
}
#[inline(always)]
pub fn export_x(x: &Wide<12>) -> [Fq; 8] {
    divide::<2, 12>(x)
}
#[inline(always)]
pub fn export_y(y: &Wide<4>) -> [Fq; 8] {
    divide::<1, 4>(y)
}
#[inline(always)]
fn add<const A: u64, const B: u64, const O: u64>(a: &Wide<A>, b: &Wide<B>) -> Wide<O> {
    const { assert!(O >= A + B) };
    unsafe {
        let mut t = core::array::from_fn(|j| _mm512_add_epi64(a.0 .0[j], b.0 .0[j]));
        for j in 0..4 {
            t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
            t[j] = _mm512_and_si512(t[j], splat(MASK));
        }
        bounded(Ifma8(t, PhantomData))
    }
}
#[inline(always)]
fn sub2<const A: u64, const B: u64, const C: u64, const P: u64, const O: u64>(
    a: &Wide<A>,
    b: &Wide<B>,
    c: &Wide<C>,
) -> Wide<O> {
    const { assert!(P >= B + C && O >= P + A) };
    unsafe {
        let p = modulus(P).map(splat);
        let mut t = [splat(0); 5];
        let mut carry = splat(0);
        for j in 0..5 {
            let v = _mm512_add_epi64(
                _mm512_sub_epi64(
                    _mm512_sub_epi64(_mm512_add_epi64(p[j], a.0 .0[j]), b.0 .0[j]),
                    c.0 .0[j],
                ),
                carry,
            );
            carry = _mm512_srai_epi64::<52>(v);
            t[j] = _mm512_and_si512(v, splat(MASK));
        }
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry, splat(0)), 0);
        bounded(Ifma8(t, PhantomData))
    }
}
#[inline(always)]
fn zero() -> Wide<0> {
    Wide(Raw::zero())
}
#[inline(always)]
pub fn dx(b: &Wide<4>, a: &Wide<4>) -> Wide<8> {
    sub2::<4, 4, 0, 4, 8>(b, a, &zero())
}
#[inline(always)]
pub fn dy(b: &Wide<2>, a: &Wide<2>) -> Wide<4> {
    sub2::<2, 2, 0, 2, 4>(b, a, &zero())
}
// Private CIOS, not public mul/sum2: widened domains are proven separately.
#[inline(always)]
fn product<const A: u64, const B: u64, const O: u64>(a: &Wide<A>, b: &Wide<B>) -> Wide<O> {
    const { assert!(A * B <= 64 && O >= 2) };
    bounded(a.0.product_cios::<false>(&b.0, &a.0, &b.0))
}
#[inline(always)]
fn fused<const A: u64, const B: u64, const C: u64, const D: u64, const O: u64>(
    a: &Wide<A>,
    b: &Wide<B>,
    c: &Wide<C>,
    d: &Wide<D>,
) -> Wide<O> {
    const { assert!((A * B + C * D <= 64 && O >= 2) || (A * B + C * D <= 128 && O >= 3)) };
    bounded(a.0.product_cios::<true>(&b.0, &c.0, &d.0))
}
impl Wide<2> {
    #[inline(always)]
    pub fn one() -> Self {
        bounded(Raw::one())
    }
    #[inline(always)]
    pub fn mul(&self, b: &Self) -> Self {
        product(self, b)
    }
    #[inline(always)]
    pub fn mul_norm(&self, b: &Wide<3>) -> Self {
        product(self, b)
    }
    #[inline]
    pub fn inverse_nonzero(&self) -> Self {
        let swapped = bounded::<2>(Raw::shuffle_pair::<1, 0, 3, 2>(&self.0, &self.0));
        let pairs = self.mul(&swapped);
        let other_pair = bounded::<2>(Raw::shuffle_pair::<2, 3, 0, 1>(&pairs.0, &pairs.0));
        let quads = pairs.mul(&other_pair);
        let products = quads.0.get();
        let inverse = (products[0] * products[4]).inverse().unwrap();
        let (lo, hi) = quads.0.halves();
        let other_quad = bounded::<2>(Raw::from_halves(&hi, &lo));
        bounded::<2>(Raw::new(&[inverse; 8]))
            .mul(&other_quad)
            .mul(&other_pair)
            .mul(&swapped)
    }
}
#[inline(always)]
pub fn norm(d: &[Wide<8>; 2]) -> Wide<3> {
    fused(&d[0], &d[0], &d[1], &d[1])
}
#[inline(always)]
pub fn slope(dy: &[Wide<4>; 2], dx: &[Wide<8>; 2], factor: &Wide<2>) -> [Wide<2>; 2] {
    let neg = sub2::<0, 4, 0, 4, 4>(&zero(), &dy[0], &zero());
    [
        fused::<4, 8, 4, 8, 2>(&dy[0], &dx[0], &dy[1], &dx[1]).mul(factor),
        fused::<4, 8, 4, 8, 2>(&dy[1], &dx[0], &neg, &dx[1]).mul(factor),
    ]
}
#[inline(always)]
pub fn square(s: &[Wide<2>; 2]) -> [Wide<4>; 2] {
    let sum = add::<2, 2, 4>(&s[0], &s[1]);
    let diff = sub2::<2, 2, 0, 2, 4>(&s[0], &s[1], &zero());
    let im = product::<2, 2, 2>(&s[0], &s[1]);
    [
        product::<4, 4, 2>(&sum, &diff).widen(),
        add::<2, 2, 4>(&im, &im),
    ]
}
#[inline(always)]
pub fn xcoord(s: &Wide<4>, a: &Wide<4>, b: &Wide<4>) -> Wide<12> {
    sub2::<4, 4, 4, 8, 12>(s, a, b)
}
#[inline(always)]
pub fn difference(a: &Wide<4>, x: &Wide<12>) -> Wide<16> {
    sub2::<4, 12, 0, 12, 16>(a, x, &zero())
}
#[inline(always)]
pub fn finish(s: &[Wide<2>; 2], d: &[Wide<16>; 2], ay: &[Wide<2>; 2]) -> [Wide<4>; 2] {
    let neg = sub2::<0, 2, 0, 2, 2>(&zero(), &s[1], &zero());
    let r = fused::<2, 16, 2, 16, 2>(&s[0], &d[0], &neg, &d[1]);
    let i = fused::<2, 16, 2, 16, 2>(&s[0], &d[1], &s[1], &d[0]);
    [
        sub2::<2, 2, 0, 2, 4>(&r, &ay[0], &zero()),
        sub2::<2, 2, 0, 2, 4>(&i, &ay[1], &zero()),
    ]
}
#[cfg(zopro_scaled_caller_probe)]
pub mod diagnostics {
    use super::*;
    pub fn cios(
        a: [[u64; 5]; 8],
        b: [[u64; 5]; 8],
        c: [[u64; 5]; 8],
        d: [[u64; 5]; 8],
        fused: bool,
    ) -> [[u64; 5]; 8] {
        fn pack(a: [[u64; 5]; 8]) -> Raw {
            let cols: [[u64; 8]; 5] = core::array::from_fn(|j| core::array::from_fn(|i| a[i][j]));
            assert!(cols.iter().flatten().all(|v| *v <= MASK));
            Ifma8(unsafe { core::mem::transmute(cols) }, PhantomData)
        }
        let (a, b, c, d) = (pack(a), pack(b), pack(c), pack(d));
        let out = if fused {
            a.product_cios::<true>(&b, &c, &d)
        } else {
            a.product_cios::<false>(&b, &a, &b)
        };
        let cols: [[u64; 8]; 5] = unsafe { core::mem::transmute(out.0) };
        assert!(cols.iter().flatten().all(|v| *v <= MASK));
        core::array::from_fn(|i| core::array::from_fn(|j| cols[j][i]))
    }
    pub fn codec_x(x: [Fq; 8]) -> [Fq; 8] {
        export_x(&import_x(&x).widen())
    }
    pub fn codec_y(y: [Fq; 8]) -> [Fq; 8] {
        export_y(&import_y(&y).widen())
    }
    pub fn export_raw(a: [[u64; 5]; 8], x: bool) -> [Fq; 8] {
        let cols: [[u64; 8]; 5] = core::array::from_fn(|j| core::array::from_fn(|i| a[i][j]));
        let raw = Ifma8(unsafe { core::mem::transmute(cols) }, PhantomData);
        if x {
            assert!(valid::<12>(&raw));
            export_x(&Wide(raw))
        } else {
            assert!(valid::<4>(&raw));
            export_y(&Wide(raw))
        }
    }
}

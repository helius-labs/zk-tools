//! Fixed ordinary factors; no Montgomery-encoded multiplier is accepted.
use super::*;

/// Ten radix-52 digits of constant preparation, independent of input radix.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FixedFactor<C: FpConfig> {
    w: [u64; 5],
    q: [u64; 5],
    marker: PhantomData<C>,
}

impl<C: FpConfig> FixedFactor<C> {
    /// `w` is the ordinary canonical integer, not raw Montgomery limbs.
    /// Preparation uses fixed arrays and bit division; no heap allocation.
    pub fn from_canonical(w: [u64; 4]) -> Option<Self> {
        if !crate::portable::less(w, C::P) { return None; }
        assert!(C::P[3] >= 1u64 << 61 && C::P[3] < 1u64 << 62);
        let mut rem = w;
        let mut q = [0u64; 5];
        for _ in 0..260 {
            rem = crate::portable::add4(rem, rem).0;
            let ge = !crate::portable::less(rem, C::P);
            if ge { rem = crate::portable::sub4(rem, C::P).0; }
            let mut carry = ge as u64;
            for digit in &mut q {
                let value = (*digit << 1) | carry;
                *digit = value & MASK;
                carry = value >> 52;
            }
            debug_assert_eq!(carry, 0);
        }
        Some(Self { w: split(w), q, marker: PhantomData })
    }

    pub fn broadcast(&self) -> FixedFactor8<C> {
        FixedFactor8 {
            w: self.w.map(Ifma8::<C, false>::splat),
            q: self.q.map(Ifma8::<C, false>::splat),
            marker: PhantomData,
        }
    }

    #[cfg(zopro_fixed_factor_diagnostics)]
    pub fn fixed_digits(&self) -> ([u64; 5], [u64; 5]) { (self.w, self.q) }
}

/// Expanded factors, with eight unrelated factors allowed in the lanes.
#[derive(Clone, Copy)]
pub struct FixedFactor8<C: FpConfig> {
    w: [__m512i; 5],
    q: [__m512i; 5],
    marker: PhantomData<C>,
}

impl<C: FpConfig> FixedFactor8<C> {
    pub fn new(factors: &[FixedFactor<C>; 8]) -> Self {
        let pack = |second: bool| core::array::from_fn(|j| {
            let lanes: [u64; 8] = core::array::from_fn(|i| {
                if second { factors[i].q[j] } else { factors[i].w[j] }
            });
            unsafe { _mm512_loadu_si512(lanes.as_ptr().cast()) }
        });
        Self { w: pack(false), q: pack(true), marker: PhantomData }
    }

    /// Input digits are normalized, with numerical value at most 2p.
    /// Returns normalized x*w mod p below 2p, retaining the input radix.
    #[inline(always)]
    pub fn mul<const MIXED: bool>(&self, x: &Ifma8<C, MIXED>) -> Ifma8<C, MIXED> {
        unsafe {
            let zero = Ifma8::<C, MIXED>::splat(0);
            let mask = Ifma8::<C, MIXED>::splat(MASK);
            let mut h = [zero; 5];
            macro_rules! high {
                ($i:literal, $j:literal, $k:literal) => {{
                    h[$k] = _mm512_madd52lo_epu64(h[$k], x.0[$i], self.q[$j]);
                    h[$k+1] = _mm512_madd52hi_epu64(h[$k+1], x.0[$i], self.q[$j]);
                }};
            }
            macro_rules! high_boundary {
                ($i:literal, $j:literal) => { h[0] = _mm512_madd52hi_epu64(h[0], x.0[$i], self.q[$j]); };
            }
            // Only nonzero parts of the fixed high-product triangle.
            high_boundary!(0,4); high_boundary!(1,3); high_boundary!(2,2); high_boundary!(3,1); high_boundary!(4,0);
            high!(1,4,0); high!(2,3,0); high!(3,2,0); high!(4,1,0);
            high!(2,4,1); high!(3,3,1); high!(4,2,1);
            high!(3,4,2); high!(4,3,2); high!(4,4,3);
            let mut carry = zero;
            for digit in &mut h {
                let v = _mm512_add_epi64(*digit, carry);
                *digit = _mm512_and_si512(v, mask);
                carry = _mm512_srli_epi64::<52>(v);
            }
            debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry, zero), 0);

            let p = split(C::P).map(Ifma8::<C, MIXED>::splat);
            let mut t = [zero; 5];
            macro_rules! low {
                ($i:literal, $j:literal) => {{
                    t[$i+$j] = _mm512_madd52lo_epu64(t[$i+$j], x.0[$i], self.w[$j]);
                    t[$i+$j] = _mm512_sub_epi64(t[$i+$j], _mm512_madd52lo_epu64(zero, h[$i], p[$j]));
                    t[$i+$j+1] = _mm512_madd52hi_epu64(t[$i+$j+1], x.0[$i], self.w[$j]);
                    t[$i+$j+1] = _mm512_sub_epi64(t[$i+$j+1], _mm512_madd52hi_epu64(zero, h[$i], p[$j]));
                }};
            }
            macro_rules! low_boundary {
                ($i:literal, $j:literal) => {{
                    t[4] = _mm512_madd52lo_epu64(t[4], x.0[$i], self.w[$j]);
                    t[4] = _mm512_sub_epi64(t[4], _mm512_madd52lo_epu64(zero, h[$i], p[$j]));
                }};
            }
            low!(0,0);
            low!(0,1); low!(1,0);
            low!(0,2); low!(1,1); low!(2,0);
            low!(0,3); low!(1,2); low!(2,1); low!(3,0);
            low_boundary!(0,4); low_boundary!(1,3); low_boundary!(2,2); low_boundary!(3,1); low_boundary!(4,0);
            carry = zero;
            for digit in &mut t {
                let v = _mm512_add_epi64(*digit, carry);
                *digit = _mm512_and_si512(v, mask);
                carry = _mm512_srai_epi64::<52>(v);
            }
            // The omitted whole-R multiple can have carry in [-9,8].
            // Its normalized low digits equal the full remainder in [0,10p).
            debug_assert_eq!(_mm512_cmplt_epi64_mask(carry, Ifma8::<C, MIXED>::splat((-9i64) as u64)), 0);
            debug_assert_eq!(_mm512_cmpgt_epi64_mask(carry, Ifma8::<C, MIXED>::splat(8)), 0);
            let mu = Ifma8::<C, MIXED>::splat((1u64 << 52) / (split(C::P)[4] + 1));
            let k = _mm512_madd52hi_epu64(zero, t[4], mu);
            debug_assert_eq!(_mm512_cmpgt_epu64_mask(k, Ifma8::<C, MIXED>::splat(9)), 0);
            carry = zero;
            for j in 0..5 {
                let v = _mm512_add_epi64(_mm512_sub_epi64(t[j], _mm512_mullo_epi64(k, p[j])), carry);
                t[j] = _mm512_and_si512(v, mask);
                carry = _mm512_srai_epi64::<52>(v);
            }
            debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry, zero), 0);
            Ifma8(t, PhantomData)
        }
    }
}

#[cfg(zopro_fixed_factor_diagnostics)]
impl<C: FpConfig, const MIXED: bool> Ifma8<C, MIXED> {
    /// Diagnostic only: caller supplies normalized numerical words <=2p.
    pub unsafe fn fixed_raw(words: &[[u64;4];8]) -> Self { Self::raw(words) }
    pub fn fixed_digits(&self) -> [[u64;8];5] { unsafe { self.0.map(|v| core::mem::transmute(v)) } }
}

//! Fused symmetric denominator norm, derived from the archived square schedule.
use super::*;
use crate::FqConfig;
impl Ifma8<FqConfig, true> {
    #[inline(always)]
    pub fn sum_squares(&self, other: &Self) -> Self {
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(FqConfig::P).map(Self::splat);
            let inv = Self::splat(FqConfig::INV & MASK);
            let a = self.0;
            let b = other.0;
            let mut t = [zero; 6];
            macro_rules! cancel {
                ($bits:literal) => {{
                    let mask_d = Self::splat((1u64 << $bits) - 1);
                    let q = _mm512_and_si512(_mm512_madd52lo_epu64(zero, t[0], inv), mask_d);
                    let carry = _mm512_srli_epi64::<$bits>(_mm512_add_epi64(t[0], mask_d));
                    let shifted_q = if $bits == 48 { _mm512_slli_epi64::<4>(q) } else { q };
                    let low = _mm512_madd52hi_epu64(carry, shifted_q, p[0]);
                    macro_rules! limb {
                        ($j:literal, $k:literal) => {
                            t[$j] = _mm512_madd52lo_epu64(t[$j], q, p[$j]);
                            t[$k] = _mm512_madd52hi_epu64(t[$k], q, p[$j]);
                        };
                    }
                    limb!(1, 2); limb!(2, 3); limb!(3, 4); limb!(4, 5);
                    if $bits == 52 {
                        t = [_mm512_add_epi64(t[1], low), t[2], t[3], t[4], t[5], zero];
                    } else {
                        t = [
                            _mm512_add_epi64(low, _mm512_slli_epi64::<4>(_mm512_and_si512(t[1], mask_d))),
                            _mm512_add_epi64(_mm512_srli_epi64::<48>(t[1]), _mm512_slli_epi64::<4>(_mm512_and_si512(t[2], mask_d))),
                            _mm512_add_epi64(_mm512_srli_epi64::<48>(t[2]), _mm512_slli_epi64::<4>(_mm512_and_si512(t[3], mask_d))),
                            _mm512_add_epi64(_mm512_srli_epi64::<48>(t[3]), _mm512_slli_epi64::<4>(_mm512_and_si512(t[4], mask_d))),
                            _mm512_add_epi64(_mm512_srli_epi64::<48>(t[4]), _mm512_slli_epi64::<4>(t[5])), zero,
                        ];
                    }
                }};
            }
            // Product diagonal 0.
            t[0] = _mm512_madd52lo_epu64(t[0], a[0], a[0]);
            t[0] = _mm512_madd52lo_epu64(t[0], b[0], b[0]);
            t[1] = _mm512_madd52hi_epu64(t[1], a[0], a[0]);
            t[1] = _mm512_madd52hi_epu64(t[1], b[0], b[0]);
            cancel!(52);
            // Product diagonal 1.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[0], a[1]);
                lo = _mm512_madd52lo_epu64(lo, b[0], b[1]);
                hi = _mm512_madd52hi_epu64(hi, a[0], a[1]);
                hi = _mm512_madd52hi_epu64(hi, b[0], b[1]);
                t[0] = _mm512_add_epi64(t[0], _mm512_add_epi64(lo, lo));
                t[1] = _mm512_add_epi64(t[1], _mm512_add_epi64(hi, hi));
            }
            cancel!(52);
            // Product diagonal 2.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[0], a[2]);
                lo = _mm512_madd52lo_epu64(lo, b[0], b[2]);
                hi = _mm512_madd52hi_epu64(hi, a[0], a[2]);
                hi = _mm512_madd52hi_epu64(hi, b[0], b[2]);
                t[0] = _mm512_add_epi64(t[0], _mm512_add_epi64(lo, lo));
                t[1] = _mm512_add_epi64(t[1], _mm512_add_epi64(hi, hi));
            }
            t[0] = _mm512_madd52lo_epu64(t[0], a[1], a[1]);
            t[0] = _mm512_madd52lo_epu64(t[0], b[1], b[1]);
            t[1] = _mm512_madd52hi_epu64(t[1], a[1], a[1]);
            t[1] = _mm512_madd52hi_epu64(t[1], b[1], b[1]);
            cancel!(52);
            // Product diagonal 3.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[0], a[3]);
                lo = _mm512_madd52lo_epu64(lo, b[0], b[3]);
                hi = _mm512_madd52hi_epu64(hi, a[0], a[3]);
                hi = _mm512_madd52hi_epu64(hi, b[0], b[3]);
                lo = _mm512_madd52lo_epu64(lo, a[1], a[2]);
                lo = _mm512_madd52lo_epu64(lo, b[1], b[2]);
                hi = _mm512_madd52hi_epu64(hi, a[1], a[2]);
                hi = _mm512_madd52hi_epu64(hi, b[1], b[2]);
                t[0] = _mm512_add_epi64(t[0], _mm512_add_epi64(lo, lo));
                t[1] = _mm512_add_epi64(t[1], _mm512_add_epi64(hi, hi));
            }
            cancel!(52);
            // Product diagonal 4.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[0], a[4]);
                lo = _mm512_madd52lo_epu64(lo, b[0], b[4]);
                hi = _mm512_madd52hi_epu64(hi, a[0], a[4]);
                hi = _mm512_madd52hi_epu64(hi, b[0], b[4]);
                lo = _mm512_madd52lo_epu64(lo, a[1], a[3]);
                lo = _mm512_madd52lo_epu64(lo, b[1], b[3]);
                hi = _mm512_madd52hi_epu64(hi, a[1], a[3]);
                hi = _mm512_madd52hi_epu64(hi, b[1], b[3]);
                t[0] = _mm512_add_epi64(t[0], _mm512_add_epi64(lo, lo));
                t[1] = _mm512_add_epi64(t[1], _mm512_add_epi64(hi, hi));
            }
            t[0] = _mm512_madd52lo_epu64(t[0], a[2], a[2]);
            t[0] = _mm512_madd52lo_epu64(t[0], b[2], b[2]);
            t[1] = _mm512_madd52hi_epu64(t[1], a[2], a[2]);
            t[1] = _mm512_madd52hi_epu64(t[1], b[2], b[2]);
            // Product diagonal 5.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[1], a[4]);
                lo = _mm512_madd52lo_epu64(lo, b[1], b[4]);
                hi = _mm512_madd52hi_epu64(hi, a[1], a[4]);
                hi = _mm512_madd52hi_epu64(hi, b[1], b[4]);
                lo = _mm512_madd52lo_epu64(lo, a[2], a[3]);
                lo = _mm512_madd52lo_epu64(lo, b[2], b[3]);
                hi = _mm512_madd52hi_epu64(hi, a[2], a[3]);
                hi = _mm512_madd52hi_epu64(hi, b[2], b[3]);
                t[1] = _mm512_add_epi64(t[1], _mm512_add_epi64(lo, lo));
                t[2] = _mm512_add_epi64(t[2], _mm512_add_epi64(hi, hi));
            }
            // Product diagonal 6.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[2], a[4]);
                lo = _mm512_madd52lo_epu64(lo, b[2], b[4]);
                hi = _mm512_madd52hi_epu64(hi, a[2], a[4]);
                hi = _mm512_madd52hi_epu64(hi, b[2], b[4]);
                t[2] = _mm512_add_epi64(t[2], _mm512_add_epi64(lo, lo));
                t[3] = _mm512_add_epi64(t[3], _mm512_add_epi64(hi, hi));
            }
            t[2] = _mm512_madd52lo_epu64(t[2], a[3], a[3]);
            t[2] = _mm512_madd52lo_epu64(t[2], b[3], b[3]);
            t[3] = _mm512_madd52hi_epu64(t[3], a[3], a[3]);
            t[3] = _mm512_madd52hi_epu64(t[3], b[3], b[3]);
            // Product diagonal 7.
            {
                let mut lo = zero; let mut hi = zero;
                lo = _mm512_madd52lo_epu64(lo, a[3], a[4]);
                lo = _mm512_madd52lo_epu64(lo, b[3], b[4]);
                hi = _mm512_madd52hi_epu64(hi, a[3], a[4]);
                hi = _mm512_madd52hi_epu64(hi, b[3], b[4]);
                t[3] = _mm512_add_epi64(t[3], _mm512_add_epi64(lo, lo));
                t[4] = _mm512_add_epi64(t[4], _mm512_add_epi64(hi, hi));
            }
            // Product diagonal 8.
            t[4] = _mm512_madd52lo_epu64(t[4], a[4], a[4]);
            t[4] = _mm512_madd52lo_epu64(t[4], b[4], b[4]);
            t[5] = _mm512_madd52hi_epu64(t[5], a[4], a[4]);
            t[5] = _mm512_madd52hi_epu64(t[5], b[4], b[4]);
            cancel!(48);
            for j in 0..4 {
                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
                t[j] = _mm512_and_si512(t[j], mask);
            }
            Self::reduce_fused([t[0], t[1], t[2], t[3], t[4]])
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{Field as _, PrimeField as _};
    use crate::{Field as _, Fq};

    fn reference(x: Fq) -> ark_ref::Fq {
        let canonical = Fq::from_montgomery(crate::portable::reduce_once(x.0, FqConfig::P));
        ark_ref::Fq::new(ark_ff::BigInt(canonical.to_canonical()))
    }
    fn check(a: [Fq;8], b: [Fq;8]) {
        let result = Ifma8::new(&a).sum_squares(&Ifma8::new(&b));
        for (i,x) in result.get().into_iter().enumerate() {
            assert_eq!(reference(x), reference(a[i]).square()+reference(b[i]).square());
        }
        let digits: [[u64;8];5] = result.0.map(|v| unsafe { core::mem::transmute(v) });
        let two_p=crate::portable::add4(FqConfig::P,FqConfig::P).0;
        for i in 0..8 {
            for d in digits { assert!(d[i] <= MASK); }
            assert!(digits[4][i]<1<<48);
            let w=[digits[0][i]|digits[1][i]<<52, digits[1][i]>>12|digits[2][i]<<40,
                digits[2][i]>>24|digits[3][i]<<28,digits[3][i]>>36|digits[4][i]<<16];
            assert!(crate::portable::less(w,two_p));
        }
    }
    #[test]
    fn symmetric_norm_lazy_edges_and_full_width_arkworks() {
        let p=FqConfig::P;
        let twice=crate::portable::add4(p,p).0;
        let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,
            crate::portable::add4(p,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0];
        for x in edges { for y in edges {
            check([Fq::from_montgomery(x);8],[Fq::from_montgomery(y);8]);
        }}
        let mut state=0x8ac77ce5d51e129bu64;
        let mut random=|| {let mut w=[0;4];for x in &mut w {state^=state<<13;state^=state>>7;state^=state<<17;*x=state;}w[3]&=(1<<62)-1;Fq::from_montgomery(w)};
        for _ in 0..4096 {check(core::array::from_fn(|_|random()),core::array::from_fn(|_|random()));}
    }
    #[test]
    fn symmetric_norm_dependent_lazy_chains_match_arkworks() {
        let a=core::array::from_fn(|i|Fq::from((i*1337+99)as u64));
        let b=core::array::from_fn(|i|Fq::from((i*197+13)as u64));
        let mut x=Ifma8::new(&a);let mut y=Ifma8::new(&b);
        let mut rx=a.map(reference);let mut ry=b.map(reference);
        for _ in 0..512 {
            let next=x.sum_squares(&y);let rnext=core::array::from_fn(|i|rx[i].square()+ry[i].square());
            for (i,v) in next.get().into_iter().enumerate(){assert_eq!(reference(v),rnext[i]);}
            x=y;y=next;rx=ry;ry=rnext;
        }
    }
}

//! Canonical absolute differences for sign-insensitive G2 norms.
use crate::{ifma8::Ifma8,FpConfig};use core::{arch::x86_64::*,marker::PhantomData};
const MASK:u64=(1u64<<52)-1;#[inline(always)]fn splat(x:u64)->__m512i{unsafe{_mm512_set1_epi64(x as i64)}}
const fn split(x:[u64;4])->[u64;5]{[x[0]&MASK,((x[0]>>52)|(x[1]<<12))&MASK,((x[1]>>40)|(x[2]<<24))&MASK,((x[2]>>28)|(x[3]<<36))&MASK,x[3]>>16]}
impl<C:FpConfig> Ifma8<C,true>{
    /// Canonical operands only. Return the unsigned integer |a-b|, below p.
    /// This is useful when the next operation discards the difference's sign.
    #[inline(always)]
    pub fn abs_diff_canonical(&self, b: &Self) -> Self {
        unsafe {
            let mut negative=_mm512_cmplt_epu64_mask(self.0[4],b.0[4]);
            let mut equal=_mm512_cmpeq_epi64_mask(self.0[4],b.0[4]);
            if equal!=0 {
                core::hint::cold_path();
                for j in (0..4).rev() {
                    negative|=equal&_mm512_cmplt_epu64_mask(self.0[j],b.0[j]);
                    equal&=_mm512_cmpeq_epi64_mask(self.0[j],b.0[j]);
                }
            }
            let mask=splat(MASK);let zero=splat(0);let mut carry=zero;
            let mut result=[zero;5];
            for j in 0..5 {
                let difference=_mm512_sub_epi64(self.0[j],b.0[j]);
                let magnitude=_mm512_mask_sub_epi64(difference,negative,zero,difference);
                let v=_mm512_add_epi64(magnitude,carry);
                carry=_mm512_srai_epi64::<52>(v);
                result[j]=_mm512_and_si512(v,mask);
            }
            Self(result,PhantomData)
        }
    }

    #[inline(always)]
    pub fn norm_canonical(&self, other: &Self) -> Self {
        let b=self;let c=other;let d=other;const FUSED:bool=true;const MIXED:bool=true;
        unsafe {
            let zero = splat(0);
            let mask = splat(MASK);
            let p = split(C::P).map(Self::splat);
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
}
#[cfg(test)]mod absolute_norm_tests {
 use super::*;use crate::{portable,FqConfig,FrConfig,Fp};
 fn check<C:FpConfig,A:ark_ff::PrimeField>() {
  let p=C::P;let pm1=portable::sub4(p,[1,0,0,0]).0;
  let boundaries=[[0;4],[1,0,0,0],pm1,portable::sub4(pm1,[1,0,0,0]).0,[u64::MAX,0,0,0],[0,u64::MAX,0,0],[0,0,u64::MAX,0]];
  let mut seed=0xc851_6a21_90f1_0a37u64;let mut rng=||{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed};
  let mut cases=0;let mut tied_masks=std::collections::BTreeSet::new();
  for iteration in 0..2048usize {
   let mut a:[Fp<C>;8]=core::array::from_fn(|lane|{let mut words=if iteration<boundaries.len(){boundaries[(iteration+lane)%boundaries.len()]}else{core::array::from_fn(|_|rng())};words[3]&=(1<<62)-1;if !portable::less(words,p){words=portable::sub4(words,p).0;}Fp::from_montgomery(words)});
   let mut b:[Fp<C>;8]=core::array::from_fn(|lane|{let mut words=if iteration<boundaries.len(){boundaries[(iteration+3+lane)%boundaries.len()]}else{core::array::from_fn(|_|rng())};words[3]&=(1<<62)-1;if !portable::less(words,p){words=portable::sub4(words,p).0;}Fp::from_montgomery(words)});
   // Every mask of equal leading radix52 digits, with lower digits differing
   // across all positions. Values remain canonical after this construction.
   if iteration<256 {for lane in 0..8{let same=iteration>>lane&1!=0;let high=0x12345u64+(lane as u64)*7;a[lane].0[3]=(high<<16)|(a[lane].0[3]&65535);b[lane].0[3]=((high+if same{0}else{1})<<16)|(b[lane].0[3]&65535);}}
   if iteration==256{a=[Fp::from_montgomery(pm1);8];b=[Fp::ZERO;8];}
   if iteration==257{b=[Fp::from_montgomery(pm1);8];a=[Fp::ZERO;8];}
   let av=Ifma8::<C,true>::new(&a);let bv=Ifma8::<C,true>::new(&b);
   let mut tie=0u8;for lane in 0..8{if a[lane].0[3]>>16==b[lane].0[3]>>16{tie|=1<<lane;}}if iteration<256{assert_eq!(tie,iteration as u8);tied_masks.insert(tie);}
   let ab=av.abs_diff_canonical(&bv);let ba=bv.abs_diff_canonical(&av);let expected=a.map(|_|Fp::ZERO);let mut expected=expected;
   for lane in 0..8 {assert!(portable::less(a[lane].0,p));assert!(portable::less(b[lane].0,p));let words=if portable::less(a[lane].0,b[lane].0){portable::sub4(b[lane].0,a[lane].0).0}else{portable::sub4(a[lane].0,b[lane].0).0};expected[lane]=Fp::from_montgomery(words);assert!(portable::less(words,p));}
   for limb in 0..5{let x:[u64;8]=unsafe{core::mem::transmute(ab.0[limb])};let y:[u64;8]=unsafe{core::mem::transmute(ba.0[limb])};assert_eq!(x,y);for lane in 0..8{assert!(x[lane]<(1<<52));let w=expected[lane].0;let bit=limb*52;let word=bit/64;let shift=bit%64;let mut raw=w[word]>>shift;if shift!=0&&word+1<4{raw|=w[word+1]<<(64-shift);}assert_eq!(x[lane],raw&MASK);}}
   assert_eq!(ab.get(),expected);
   let value=ab.norm_canonical(&av);let original=ab.sum2(&ab,&av,&av);for limb in 0..5{let x:[u64;8]=unsafe{core::mem::transmute(value.0[limb])};let y:[u64;8]=unsafe{core::mem::transmute(original.0[limb])};assert_eq!(x,y);}
   let bytes=|v:Fp<C>|{let canonical=v.to_canonical();A::from_le_bytes_mod_order(&canonical.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())};let got=value.get();for lane in 0..8{let x=bytes(expected[lane]);let y=bytes(a[lane]);assert_eq!(bytes(got[lane]),x*x+y*y);}
   // The norm of independent component differences is sign-insensitive.
   let n=ab.norm_canonical(&ba).get();let ordinary=av.sub_canonical(&bv);let old=ordinary.sum2(&ordinary,&ordinary,&ordinary).get();assert_eq!(n,old);
   cases+=8;
  }
  assert_eq!(tied_masks.len(),256);println!("ABS_NORM_LANES={cases}");
 }
 #[test]fn canonical_absolute_norm_fq(){check::<FqConfig,ark_ref::Fq>();}
 #[test]fn canonical_absolute_norm_fr(){check::<FrConfig,ark_ref::Fr>();}
}

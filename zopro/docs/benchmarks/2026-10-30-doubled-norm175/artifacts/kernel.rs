// Private doubled-operand symmetric norm; original cancellation/packing retained.
impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{
    #[inline(always)]
    pub fn norm_doubled(&self, b: &Self) -> Self {
        const FUSED:bool=true;
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            let mut t = [zero; 6];
            macro_rules! squares {
                ($i:literal) => {{
                    t[$i] = _mm512_madd52lo_epu64(t[$i], self.0[$i], self.0[$i]);
                    t[$i+1] = _mm512_madd52hi_epu64(t[$i+1], self.0[$i], self.0[$i]);
                    t[$i] = _mm512_madd52lo_epu64(t[$i], b.0[$i], b.0[$i]);
                    t[$i+1] = _mm512_madd52hi_epu64(t[$i+1], b.0[$i], b.0[$i]);
                    if $i < 4 {
                        // IFMA discards bit52 of the doubled digit. Restore
                        // its exact contribution once in the next coefficient.
                        let da = _mm512_slli_epi64::<1>(self.0[$i]);
                        let db = _mm512_slli_epi64::<1>(b.0[$i]);
                        let ca = _mm512_test_epi64_mask(self.0[$i],Self::splat(1<<51));
                        let cb = _mm512_test_epi64_mask(b.0[$i],Self::splat(1<<51));
                        for j in $i+1..5 {
                            t[j] = _mm512_madd52lo_epu64(t[j],da,self.0[j]);
                            t[j+1] = _mm512_madd52hi_epu64(t[j+1],da,self.0[j]);
                            t[j+1] = _mm512_mask_add_epi64(t[j+1],ca,t[j+1],self.0[j]);
                            t[j] = _mm512_madd52lo_epu64(t[j],db,b.0[j]);
                            t[j+1] = _mm512_madd52hi_epu64(t[j+1],db,b.0[j]);
                            t[j+1] = _mm512_mask_add_epi64(t[j+1],cb,t[j+1],b.0[j]);
                        }
                    }
                }};
            }

            macro_rules! row {
                ($i:literal) => {{
                    squares!($i);
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
                squares!(4);
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
            for j in 0..4 {
                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
                t[j] = _mm512_and_si512(t[j], mask);
            }
            // Two squares: unreduced REDC < p + 8p²/R; original fused correction follows.
            let result = [t[0], t[1], t[2], t[3], t[4]];
            if FUSED && MIXED {
                Self::reduce_fused(result)
            } else {
                Self(result, PhantomData)
            }
        }
    }
}
#[cfg(test)]mod doubled_norm_tests{
 use super::*;use crate::{FqConfig,FrConfig};
 fn next(x:&mut u64)->u64{*x^=*x<<13;*x^=*x>>7;*x^=*x<<17;*x}
 fn lane_array<C:FpConfig,const M:bool>(v:&Ifma8<C,M>)->[[u64;8];5]{unsafe{v.0.map(|v|core::mem::transmute(v))}}
 fn from_words<C:FpConfig,const M:bool>(a:[[u64;4];8])->Ifma8<C,M>{let limbs=a.map(split);Ifma8(core::array::from_fn(|j|unsafe{core::mem::transmute(core::array::from_fn::<u64,8,_>(|i|limbs[i][j]))}),PhantomData)}
 fn check<C:FpConfig,const M:bool>(){
  let two=crate::portable::add4(C::P,C::P).0;
  let mut edges=vec![[0;4],[1,0,0,0],C::P,crate::portable::sub4(C::P,[1,0,0,0]).0,crate::portable::add4(C::P,[1,0,0,0]).0,two,crate::portable::sub4(two,[1,0,0,0]).0];
  for bit in 0..255{let mut v=[0;4];v[bit/64]=1u64<<(bit%64);if crate::portable::less(v,two){edges.push(v);if bit!=0{edges.push(crate::portable::sub4(v,[1,0,0,0]).0);}if crate::portable::less(crate::portable::add4(v,[1,0,0,0]).0,two){edges.push(crate::portable::add4(v,[1,0,0,0]).0);}}}
  let mut seed=0x131a90cdb4748423;let mut checks=0;
  for case in 0..edges.len()+8192{
   let mut words=|offset:usize|core::array::from_fn(|i|{if case<edges.len(){edges[(case*offset+i*71)%edges.len()]}else{let mut v=core::array::from_fn(|_|next(&mut seed));v[3]&=(1<<63)-1;while !crate::portable::less(v,two){v=crate::portable::sub4(v,two).0;}v}});
   let a=from_words::<C,M>(words(1));let b=from_words::<C,M>(words(37));let got=a.norm_doubled(&b);let want=a.sum2(&a,&b,&b);assert_eq!(lane_array(&got),lane_array(&want),"case={case} radixmixed={M}");
   for d in lane_array(&got){assert!(d.iter().all(|&x|x<=MASK));}
   for raw in got.compact().0{let _=raw;} // compact checks the normalized 256-bit bound.
   assert_eq!(got.get(),want.get());checks+=8;
  }
  // Cross-cancellation small-radix model independently reconstructs integers.
  for a in 0u64..32{for b in 0u64..32{let base=16;let da=(2*a)%base;let ca=2*a/base;let lo=da*b%base;let hi=da*b/base+ca*b;assert_eq!(lo+base*hi,2*a*b);}}
  let mut a=Ifma8::<C,M>::new(&core::array::from_fn(|i|Fp::<C>::from((i+2)as u64)));let b=Ifma8::<C,M>::new(&core::array::from_fn(|i|Fp::<C>::from((i+17)as u64)));
  for _ in 0..512{let want=a.sum2(&a,&b,&b);a=a.norm_doubled(&b);assert_eq!(lane_array(&a),lane_array(&want));checks+=8;}
  println!("doubled norm M={M} {checks} exact lazy lane comparisons");
 }
 #[test]fn doubled_norm_fq(){check::<FqConfig,true>();check::<FqConfig,false>();}
 #[test]fn doubled_norm_fr(){check::<FrConfig,true>();check::<FrConfig,false>();}
}

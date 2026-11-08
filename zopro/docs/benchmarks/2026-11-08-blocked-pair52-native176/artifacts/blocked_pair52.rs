//! Two local 2x2 Karatsuba tiles in seven-coefficient paired CIOS rows.
use super::*;
impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    #[inline(always)]
    fn blocked_product<const FUSED:bool>(&self,b:&Self,c:&Self,d:&Self)->Self{
        // SAFETY: enclosing ifma8 module requires AVX512 IFMA/DQ/VL.
        // Inputs normalized in [0,2p]; mathematical range screen bounds all
        // signed coefficients. This child is not a production dispatcher.
        unsafe{
            let zero=Self::splat(0);let mask=Self::splat(MASK);
            let p=split(C::P).map(Self::splat);let inv=Self::splat(C::INV&MASK);
            let mut t=[zero;7];
            macro_rules! raw{
                ($a:expr,$b:expr,$j:literal,$r:literal,$k:literal,$h:literal)=>{{
                    t[$k]=_mm512_madd52lo_epu64(t[$k],$a.0[$j],$b.0[$r]);
                    t[$h]=_mm512_madd52hi_epu64(t[$h],$a.0[$j],$b.0[$r]);
                }};
            }
            macro_rules! tile{
                ($a:expr,$b:expr,$i:literal,$j:literal,$k:literal,$k1:literal,$k2:literal,$k3:literal)=>{{
                    let l0=_mm512_madd52lo_epu64(zero,$a.0[$i],$b.0[$i]);
                    let h0=_mm512_madd52hi_epu64(zero,$a.0[$i],$b.0[$i]);
                    let l1=_mm512_madd52lo_epu64(zero,$a.0[$j],$b.0[$j]);
                    let h1=_mm512_madd52hi_epu64(zero,$a.0[$j],$b.0[$j]);
                    let dx=_mm512_sub_epi64($a.0[$i],$a.0[$j]);
                    let dy=_mm512_sub_epi64($b.0[$j],$b.0[$i]);
                    let flip=_mm512_cmplt_epi64_mask(dx,zero);
                    let x=_mm512_abs_epi64(dx);
                    let y=_mm512_mask_sub_epi64(dy,flip,zero,dy);
                    let negative=_mm512_cmplt_epi64_mask(y,zero);
                    let lp=_mm512_madd52lo_epu64(zero,x,y);
                    let hp=_mm512_madd52hi_epu64(zero,x,y);
                    let hp=_mm512_mask_sub_epi64(hp,negative,hp,x);
                    let cross_l=_mm512_add_epi64(_mm512_add_epi64(l0,l1),lp);
                    let cross_h=_mm512_add_epi64(_mm512_add_epi64(h0,h1),hp);
                    t[$k]=_mm512_add_epi64(t[$k],l0);
                    t[$k1]=_mm512_add_epi64(t[$k1],h0);
                    t[$k1]=_mm512_add_epi64(t[$k1],cross_l);
                    t[$k2]=_mm512_add_epi64(t[$k2],cross_h);
                    t[$k2]=_mm512_add_epi64(t[$k2],l1);
                    t[$k3]=_mm512_add_epi64(t[$k3],h1);
                }};
            }
            macro_rules! first{
                ($a:expr,$b:expr)=>{{
                    tile!($a,$b,0,1,0,1,2,3);
                    raw!($a,$b,2,0,2,3);raw!($a,$b,3,0,3,4);raw!($a,$b,4,0,4,5);
                    raw!($a,$b,2,1,3,4);raw!($a,$b,3,1,4,5);raw!($a,$b,4,1,5,6);
                }};
            }
            macro_rules! second{
                ($a:expr,$b:expr)=>{{
                    tile!($a,$b,2,3,2,3,4,5);
                    raw!($a,$b,0,2,0,1);raw!($a,$b,1,2,1,2);raw!($a,$b,4,2,4,5);
                    raw!($a,$b,0,3,1,2);raw!($a,$b,1,3,2,3);raw!($a,$b,4,3,5,6);
                }};
            }
            macro_rules! cancel{
                ()=>{{
                    let q=_mm512_madd52lo_epu64(zero,t[0],inv);
                    let ceil=_mm512_srai_epi64::<52>(_mm512_add_epi64(t[0],mask));
                    let carry=_mm512_madd52hi_epu64(ceil,q,p[0]);
                    t[1]=_mm512_madd52lo_epu64(t[1],q,p[1]);
                    t[2]=_mm512_madd52hi_epu64(t[2],q,p[1]);
                    t[2]=_mm512_madd52lo_epu64(t[2],q,p[2]);
                    t[3]=_mm512_madd52hi_epu64(t[3],q,p[2]);
                    t[3]=_mm512_madd52lo_epu64(t[3],q,p[3]);
                    t[4]=_mm512_madd52hi_epu64(t[4],q,p[3]);
                    t[4]=_mm512_madd52lo_epu64(t[4],q,p[4]);
                    t[5]=_mm512_madd52hi_epu64(t[5],q,p[4]);
                    t[1]=_mm512_add_epi64(t[1],carry);
                    t=[t[1],t[2],t[3],t[4],t[5],t[6],zero];
                }};
            }
            first!(self,b);if FUSED{first!(c,d);}cancel!();cancel!();
            second!(self,b);if FUSED{second!(c,d);}cancel!();cancel!();
            macro_rules! last{
                ($a:expr,$b:expr)=>{{
                    raw!($a,$b,0,4,0,1);raw!($a,$b,1,4,1,2);raw!($a,$b,2,4,2,3);
                    raw!($a,$b,3,4,3,4);raw!($a,$b,4,4,4,5);
                }};
            }
            last!(self,b);if FUSED{last!(c,d);}
            let mut out=if MIXED{
                let mask48=Self::splat((1u64<<48)-1);
                let q=_mm512_and_si512(_mm512_madd52lo_epu64(zero,t[0],inv),mask48);
                let ceil=_mm512_srai_epi64::<48>(_mm512_add_epi64(t[0],mask48));
                let carry=_mm512_madd52hi_epu64(ceil,_mm512_slli_epi64::<4>(q),p[0]);
                t[1]=_mm512_madd52lo_epu64(t[1],q,p[1]);
                t[2]=_mm512_madd52hi_epu64(t[2],q,p[1]);
                t[2]=_mm512_madd52lo_epu64(t[2],q,p[2]);
                t[3]=_mm512_madd52hi_epu64(t[3],q,p[2]);
                t[3]=_mm512_madd52lo_epu64(t[3],q,p[3]);
                t[4]=_mm512_madd52hi_epu64(t[4],q,p[3]);
                t[4]=_mm512_madd52lo_epu64(t[4],q,p[4]);
                t[5]=_mm512_madd52hi_epu64(t[5],q,p[4]);
                [_mm512_add_epi64(_mm512_slli_epi64::<4>(t[1]),carry),
                    _mm512_slli_epi64::<4>(t[2]),_mm512_slli_epi64::<4>(t[3]),
                    _mm512_slli_epi64::<4>(t[4]),_mm512_slli_epi64::<4>(t[5])]
            }else{cancel!();[t[0],t[1],t[2],t[3],t[4]]};
            for j in 0..4{out[j+1]=_mm512_add_epi64(out[j+1],_mm512_srai_epi64::<52>(out[j]));out[j]=_mm512_and_si512(out[j],mask);}
            if FUSED&&MIXED{Self::reduce_fused(out)}else{Self(out,PhantomData)}
        }
    }
    #[doc(hidden)]#[inline(always)]pub fn blocked_mul(&self,b:&Self)->Self{self.blocked_product::<false>(b,self,b)}
    #[doc(hidden)]#[inline(always)]pub fn blocked_sum2(&self,b:&Self,c:&Self,d:&Self)->Self{self.blocked_product::<true>(b,c,d)}
    #[doc(hidden)]#[inline(always)]pub fn blocked_difference2(&self,b:&Self,c:&Self,d:&Self)->Self{
        let negative=unsafe{let p=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);let mask=Self::splat(MASK);let mut t=[Self::splat(0);5];let mut carry=Self::splat(0);for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(p[j],c.0[j]),carry);carry=_mm512_srai_epi64::<52>(v);t[j]=_mm512_and_si512(v,mask);}Self(t,PhantomData)};
        self.blocked_product::<true>(b,&negative,d)
    }
    /// Investigation-only raw constructor; caller guarantees value <=2p.
    #[doc(hidden)]pub unsafe fn blocked_from_raw(raw:&[[u64;4];8])->Self{unsafe{let c:[[u64;8];5]=core::array::from_fn(|j|core::array::from_fn(|i|split(raw[i])[j]));Self(c.map(|v|_mm512_loadu_si512(v.as_ptr().cast())),PhantomData)}}
    #[doc(hidden)]pub fn blocked_raw_digits(&self)->[[u64;8];5]{self.0.map(|v|unsafe{core::mem::transmute(v)})}
}

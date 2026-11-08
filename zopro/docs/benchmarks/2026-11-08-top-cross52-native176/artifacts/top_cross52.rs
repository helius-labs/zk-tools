//! Four top-limb cross products shared across ordinary six-slot CIOS rows.
use super::*;
impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    #[inline(always)]
    fn top_cross_product<const FUSED:bool>(&self,b:&Self,c:&Self,d:&Self)->Self{
        // SAFETY: enclosing ifma8 module requires AVX512 IFMA/DQ/VL.
        // Inputs normalized in [0,2p]; mathematical range screen bounds all
        // signed coefficients and mixed packing. D4 is additional live state. This child is not a production dispatcher.
        unsafe{
            let zero=Self::splat(0);let mask=Self::splat(MASK);
            let p=split(C::P).map(Self::splat);let inv=Self::splat(C::INV&MASK);
            let mut t=[zero;6];
            let top=(
                _mm512_madd52lo_epu64(zero,self.0[4],b.0[4]),
                _mm512_madd52hi_epu64(zero,self.0[4],b.0[4]),
            );
            let second_top=if FUSED{(
                _mm512_madd52lo_epu64(zero,c.0[4],d.0[4]),
                _mm512_madd52hi_epu64(zero,c.0[4],d.0[4]),
            )}else{(zero,zero)};
            macro_rules! raw{
                ($a:expr,$b:expr,$j:literal,$i:literal,$k:literal)=>{{
                    t[$j]=_mm512_madd52lo_epu64(t[$j],$a.0[$j],$b.0[$i]);
                    t[$k]=_mm512_madd52hi_epu64(t[$k],$a.0[$j],$b.0[$i]);
                }};
            }
            macro_rules! row{
                ($a:expr,$b:expr,$i:literal,$next:literal,$top:expr)=>{{
                    let diagonal=(
                        _mm512_madd52lo_epu64(zero,$a.0[$i],$b.0[$i]),
                        _mm512_madd52hi_epu64(zero,$a.0[$i],$b.0[$i]),
                    );
                    t[$i]=_mm512_add_epi64(t[$i],diagonal.0);
                    t[$next]=_mm512_add_epi64(t[$next],diagonal.1);
                    if $i!=0{raw!($a,$b,0,$i,1);}
                    if $i!=1{raw!($a,$b,1,$i,2);}
                    if $i!=2{raw!($a,$b,2,$i,3);}
                    if $i!=3{raw!($a,$b,3,$i,4);}
                    let dx=_mm512_sub_epi64($a.0[$i],$a.0[4]);
                    let dy=_mm512_sub_epi64($b.0[4],$b.0[$i]);
                    let flip=_mm512_cmplt_epi64_mask(dx,zero);
                    let x=_mm512_abs_epi64(dx);
                    let y=_mm512_mask_sub_epi64(dy,flip,zero,dy);
                    let negative=_mm512_cmplt_epi64_mask(y,zero);
                    let lo=_mm512_madd52lo_epu64(zero,x,y);
                    let hi=_mm512_madd52hi_epu64(zero,x,y);
                    let hi=_mm512_mask_sub_epi64(hi,negative,hi,x);
                    let cross_lo=_mm512_add_epi64(_mm512_add_epi64(diagonal.0,$top.0),lo);
                    let cross_hi=_mm512_add_epi64(_mm512_add_epi64(diagonal.1,$top.1),hi);
                    t[4]=_mm512_add_epi64(t[4],cross_lo);
                    t[5]=_mm512_add_epi64(t[5],cross_hi);
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
                    t=[t[1],t[2],t[3],t[4],t[5],zero];
                }};
            }
            row!(self,b,0,1,top);if FUSED{row!(c,d,0,1,second_top);}cancel!();
            row!(self,b,1,2,top);if FUSED{row!(c,d,1,2,second_top);}cancel!();
            row!(self,b,2,3,top);if FUSED{row!(c,d,2,3,second_top);}cancel!();
            row!(self,b,3,4,top);if FUSED{row!(c,d,3,4,second_top);}cancel!();
            t[4]=_mm512_add_epi64(t[4],top.0);
            t[5]=_mm512_add_epi64(t[5],top.1);
            if FUSED{
                t[4]=_mm512_add_epi64(t[4],second_top.0);
                t[5]=_mm512_add_epi64(t[5],second_top.1);
            }
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
    #[doc(hidden)]#[inline(always)]pub fn top_cross_mul(&self,b:&Self)->Self{self.top_cross_product::<false>(b,self,b)}
    #[doc(hidden)]#[inline(always)]pub fn top_cross_sum2(&self,b:&Self,c:&Self,d:&Self)->Self{self.top_cross_product::<true>(b,c,d)}
    #[doc(hidden)]#[inline(always)]pub fn top_cross_difference2(&self,b:&Self,c:&Self,d:&Self)->Self{
        let negative=unsafe{let p=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);let mask=Self::splat(MASK);let mut t=[Self::splat(0);5];let mut carry=Self::splat(0);for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(p[j],c.0[j]),carry);carry=_mm512_srai_epi64::<52>(v);t[j]=_mm512_and_si512(v,mask);}Self(t,PhantomData)};
        self.top_cross_product::<true>(b,&negative,d)
    }
    /// Investigation-only raw constructor; caller guarantees value <=2p.
    #[doc(hidden)]pub unsafe fn top_cross_from_raw(raw:&[[u64;4];8])->Self{unsafe{let c:[[u64;8];5]=core::array::from_fn(|j|core::array::from_fn(|i|split(raw[i])[j]));Self(c.map(|v|_mm512_loadu_si512(v.as_ptr().cast())),PhantomData)}}
    #[doc(hidden)]pub fn top_cross_raw_digits(&self)->[[u64;8];5]{self.0.map(|v|unsafe{core::mem::transmute(v)})}
}

//! Joint coordinate endpoints. Delta3 is opaque and never a host coordinate.
use super::*;
#[derive(Clone,Copy)]
pub struct AffineDelta3<C:FpConfig>([__m512i;5],PhantomData<C>);
impl<C:FpConfig> Ifma8<C,true>{
    /// S<2p, canonical ax,bx<p. Returns X<2p and opaque delta<3p.
    #[inline(always)]
    pub fn joint_x_delta(&self,ax:&Self,bx:&Self)->(Self,AffineDelta3<C>){
        const fn twice(p:[u64;4])->[u64;4]{crate::portable::add4(p,p).0}
        assert!(C::P[3]<(1u64<<62));
        unsafe{
            let p2=split(twice(C::P));let p4=split(twice(twice(C::P)));let mask=Self::splat(MASK);
            let(mut cx,mut cd)=(Self::splat(0),Self::splat(0));let mut x=[Self::splat(0);5];let mut d=[Self::splat(0);5];
            for j in 0..5{
                let rawx=_mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(self.0[j],Self::splat(p2[j])),ax.0[j]),bx.0[j]);
                let rawd=_mm512_sub_epi64(_mm512_add_epi64(Self::splat(p4[j]),ax.0[j]),rawx);
                let vx=_mm512_add_epi64(rawx,cx);let vd=_mm512_add_epi64(rawd,cd);
                x[j]=_mm512_and_si512(vx,mask);d[j]=_mm512_and_si512(vd,mask);
                cx=_mm512_srai_epi64::<52>(vx);cd=_mm512_srai_epi64::<52>(vd);
            }
            debug_assert_eq!(_mm512_cmpneq_epi64_mask(cx,Self::splat(0)),0);debug_assert_eq!(_mm512_cmpneq_epi64_mask(cd,Self::splat(0)),0);
            let x=Self::reduce_fused(x);let d=Self::reduce_fused(d);
            (x,AffineDelta3(d.0,PhantomData))
        }
    }
}
impl<C:FpConfig> AffineDelta3<C>{
    /// factor<=2p; private wider operand, corrected before returning ordinary field.
    #[inline(always)]pub fn mul(&self,factor:&Ifma8<C,true>)->Ifma8<C,true>{
        let wide=Ifma8(self.0,PhantomData);let product=wide.product_cios::<false>(factor,&wide,factor);Ifma8::reduce_fused(product.0)
    }
    /// (d0+i*d1)*(s0+i*s1). All deltas<3p, slopes<=2p.
    #[inline(always)]pub fn complex_mul(d0:&Self,d1:&Self,s0:&Ifma8<C,true>,s1:&Ifma8<C,true>)->[Ifma8<C,true>;2]{
        let a=Ifma8(d0.0,PhantomData);let b=Ifma8(d1.0,PhantomData);
        let negative=unsafe{
            let p=split(crate::portable::add4(C::P,C::P).0);let mut out=[Ifma8::<C,true>::splat(0);5];let mut carry=Ifma8::<C,true>::splat(0);let mask=Ifma8::<C,true>::splat(MASK);
            for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(Ifma8::<C,true>::splat(p[j]),s1.0[j]),carry);out[j]=_mm512_and_si512(v,mask);carry=_mm512_srai_epi64::<52>(v);}Ifma8(out,PhantomData)
        };
        [a.product_cios::<true>(s0,&negative,&b),a.product_cios::<true>(s1,&b,s0)]
    }
}
#[cfg(zopro_joint_delta_diagnostics)]
impl<C:FpConfig> Ifma8<C,true>{
    /// Diagnostic only; caller supplies normalized numerical values below2p.
    pub unsafe fn joint_raw(words:&[[u64;4];8])->Self{Self::raw(words)}
    pub fn joint_digits(&self)->[[u64;8];5]{unsafe{self.0.map(|v|core::mem::transmute(v))}}
}
#[cfg(zopro_joint_delta_diagnostics)]
impl<C:FpConfig> AffineDelta3<C>{
    /// Diagnostic only; caller supplies numerical values below3p.
    pub unsafe fn joint_raw(words:&[[u64;4];8])->Self{let x=Ifma8::<C,true>::raw(words);Self(x.0,PhantomData)}
    pub fn joint_digits(&self)->[[u64;8];5]{unsafe{self.0.map(|v|core::mem::transmute(v))}}
}

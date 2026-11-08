//! An unnormalized linear sum that can only be consumed by subtraction.
use super::*;

/// Five unnormalized digits representing the exact sum of two canonical
/// numerical Montgomery representatives. This type offers no multiplication
/// or ordinary field export, because its digits can exceed the IFMA width.
#[derive(Clone,Copy)]
pub struct UncarriedSum8<C:FpConfig,const MIXED:bool>([__m512i;5],PhantomData<C>);
impl<C:FpConfig,const MIXED:bool> UncarriedSum8<C,MIXED>{
    /// Both numerical input representatives must be below p.
    #[inline(always)]
    pub fn from_canonical(a:&Ifma8<C,MIXED>,b:&Ifma8<C,MIXED>)->Self{
        unsafe{Self(core::array::from_fn(|j|_mm512_add_epi64(a.0[j],b.0[j])),PhantomData)}
    }
    /// minuend is normalized and below2p. Return the representative in [0,2p).
    #[inline(always)]
    pub fn subtract_from(&self,minuend:&Ifma8<C,MIXED>)->Ifma8<C,MIXED>{
        unsafe{
            let zero=Ifma8::<C,MIXED>::splat(0);
            let mask=Ifma8::<C,MIXED>::splat(MASK);
            let twice=split(crate::portable::add4(C::P,C::P).0).map(Ifma8::<C,MIXED>::splat);
            let mut t=[zero;5];let mut carry=zero;
            for j in 0..5{
                let v=_mm512_add_epi64(_mm512_sub_epi64(minuend.0[j],self.0[j]),carry);
                t[j]=_mm512_and_si512(v,mask);
                carry=_mm512_srai_epi64::<52>(v);
            }
            let negative=_mm512_cmpneq_epi64_mask(carry,zero);
            for j in 0..5{t[j]=_mm512_add_epi64(t[j],_mm512_maskz_mov_epi64(negative,twice[j]));}
            for j in 0..4{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srli_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],mask);}
            t[4]=_mm512_and_si512(t[4],mask);
            Ifma8(t,PhantomData)
        }
    }
    /// Diagnostic inspection only; never constructs a normalized field value.
    pub fn diagnostic_digits(&self)->[[u64;8];5]{unsafe{self.0.map(|v|{let mut a=[0u64;8];_mm512_storeu_si512(a.as_mut_ptr().cast(),v);a})}}
}
impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    /// Diagnostic raw representative constructor, checked before ISA use.
    pub fn linear_diagnostic_raw(words:&[[u64;4];8],canonical:bool)->Self{
        let bound=if canonical{C::P}else{crate::portable::add4(C::P,C::P).0};
        assert!(words.iter().all(|w|crate::portable::less(*w,bound)));
        Self::raw(words)
    }
    pub fn linear_diagnostic_digits(&self)->[[u64;8];5]{unsafe{self.0.map(|v|{let mut a=[0u64;8];_mm512_storeu_si512(a.as_mut_ptr().cast(),v);a})}}
}

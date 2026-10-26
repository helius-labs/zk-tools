//! Raw R256 point coordinates in the existing uniform R260 arithmetic.
//! This is a rebase of previously tested quarter-slope algebra, not a new formula.
use crate::{ifma8::Ifma8,Fp,FpConfig};
use core::{arch::x86_64::*,marker::PhantomData};
const MASK:u64=(1u64<<52)-1;
fn split(x:[u64;4])->[u64;5]{core::array::from_fn(|j|{let bit=j*52;let word=bit/64;let offset=bit%64;let mut n=if word<4{x[word]>>offset}else{0};if offset!=0&&word+1<4{n|=x[word+1]<<(64-offset);}n&MASK})}
impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{
    /// Reinterpret limbs for the point kernel's scratch only; no value conversion.
    #[inline(always)]
    pub fn point_domain_bits<const OTHER:bool>(&self)->Ifma8<C,OTHER>{Ifma8(self.0,PhantomData)}
}
impl<C:FpConfig> Ifma8<C,false>{
    /// Keep host words. Their mathematical value in R260 is divided by16.
    #[inline(always)]
    pub fn point_raw_host(a:&[Fp<C>;8])->Self{Self(Ifma8::<C,true>::new(a).0,PhantomData)}
    /// Return normalized host words without a Montgomery-domain conversion.
    #[inline(always)]
    pub fn point_raw_get(&self)->[Fp<C>;8]{Ifma8::<C,true>(self.0,PhantomData).get()}
    /// Normalize4a<8p, then use the CURRENT CIOS/cancel-carry multiplication.
    /// a,b<2p and16p<R260 imply REDC(4ab)<2p. No wide operand escapes.
    #[inline(always)]
    pub fn point_mul_times4(&self,b:&Self)->Self{
        assert!(C::P[3]<(1u64<<62));
        unsafe{let mask=_mm512_set1_epi64(MASK as i64);let mut wide=[_mm512_setzero_si512();5];wide[0]=_mm512_and_si512(_mm512_slli_epi64::<2>(self.0[0]),mask);for j in 1..5{wide[j]=_mm512_or_si512(_mm512_and_si512(_mm512_slli_epi64::<2>(self.0[j]),mask),_mm512_srli_epi64::<50>(self.0[j-1]));}Self(wide,PhantomData).mul(b)}
    }
    /// Canonical by/ay, scale<2p. Signed numerator <=2p, including negative0.
    #[inline(always)]
    pub fn point_signed_scaled(&self,ay:&Self,scale:&Self,negative:u8)->Self{
        assert!(C::P[3]<(1u64<<62));
        unsafe{let p=split(C::P);let twice=split(crate::portable::add4(C::P,C::P).0);let flip=_mm512_maskz_mov_epi64(negative,_mm512_set1_epi64(MASK as i64));let mut carry=_mm512_maskz_mov_epi64(negative,_mm512_set1_epi64(1));let mask=_mm512_set1_epi64(MASK as i64);let mut t=[_mm512_setzero_si512();5];for j in 0..5{let offset=_mm512_mask_blend_epi64(negative,_mm512_set1_epi64(p[j]as i64),_mm512_set1_epi64(twice[j]as i64));let signed=_mm512_xor_si512(self.0[j],flip);let v=_mm512_add_epi64(_mm512_sub_epi64(_mm512_add_epi64(offset,signed),ay.0[j]),carry);t[j]=_mm512_and_si512(v,mask);carry=_mm512_srai_epi64::<52>(v);}Self(t,PhantomData).mul(scale)}
    }
}
#[cfg(test)]mod tests{
    use super::*;use crate::{Fq,FqConfig};
    type V=Ifma8<FqConfig,false>;
    #[test]fn raw_point_domain_and_current_cios_products_match_scalar_oracles(){
        let inv16=Fq::from(16u64).inverse().unwrap();let mut seed=Fq::from(919283u64);let mut lanes=0;
        for case in 0..1024{let a=core::array::from_fn(|i|{seed=seed.square()+Fq::from((i+case+1)as u64);match case%7{0=>Fq::ZERO,1=>Fq::ONE,2=>-Fq::ONE,_=>seed}});let b=core::array::from_fn(|i|a[(i+3)%8]+Fq::from((i+1)as u64));let raw=V::point_raw_host(&a);assert_eq!(raw.point_raw_get(),a);assert_eq!(raw.get(),a.map(|v|v*inv16));let result=V::new(&a).point_mul_times4(&V::new(&b)).get();for i in 0..8{assert_eq!(result[i],a[i]*b[i]*Fq::from(4u64));lanes+=1;}}
        assert_eq!(lanes,8192);
    }
    #[test]fn uniform_signed_numerators_cover_every_sign_mask_and_negative_zero(){
        let by=[Fq::ZERO,Fq::ONE,-Fq::ONE,Fq::from(19u64),Fq::ZERO,-Fq::ONE,Fq::ONE,Fq::from(31u64)];let ay=[Fq::ZERO,-Fq::ONE,Fq::ZERO,Fq::from(17u64),Fq::ONE,Fq::ONE,-Fq::ONE,Fq::from(5u64)];let factor=core::array::from_fn(|i|Fq::from((i+3)as u64));
        for mask in 0u16..256{let actual=V::new(&by).point_signed_scaled(&V::new(&ay),&V::new(&factor),mask as u8).get();for i in 0..8{assert_eq!(actual[i],((if mask>>i&1!=0{-by[i]}else{by[i]})-ay[i])*factor[i]);}}
    }
    #[test]fn widened_current_cios_matches_independent_ark_at_lazy_boundaries(){
        use ark_ff::{BigInteger,Field,PrimeField};
        fn check<C:FpConfig,A:PrimeField>(){
            let sub1=|mut w:[u64;4]|{for v in &mut w{let(n,b)=v.overflowing_sub(1);*v=n;if !b{break;}}w};let add1=|mut w:[u64;4]|{for v in &mut w{let(n,b)=v.overflowing_add(1);*v=n;if !b{break;}}w};
            let twice=crate::portable::add4(C::P,C::P).0;let choices=[[0;4],[1,0,0,0],sub1(C::P),C::P,add1(C::P),sub1(twice)];let decode=|w:[u64;4]|A::from_le_bytes_mod_order(&w.iter().flat_map(|x|x.to_le_bytes()).collect::<Vec<_>>());let rinv=A::from(2u64).pow([260]).inverse().unwrap();
            assert_eq!(decode(C::P),A::ZERO);assert_eq!(A::MODULUS.to_bytes_le(),C::P.iter().flat_map(|x|x.to_le_bytes()).collect::<Vec<_>>());
            for a in choices{for b in choices{let x=Ifma8::<C,false>::point_raw_host(&[Fp::<C>::from_montgomery(a);8]);let y=Ifma8::<C,false>::point_raw_host(&[Fp::<C>::from_montgomery(b);8]);let actual=x.point_mul_times4(&y).point_raw_get();let expected=decode(a)*decode(b)*A::from(4u64)*rinv;for v in actual{assert_eq!(decode(v.0),expected);assert!(v.0.iter().rev().cmp(C::P.iter().rev()).is_lt());}}}
        }
        check::<crate::FqConfig,ark_ref::Fq>();check::<crate::FrConfig,ark_ref::Fr>();
    }
}

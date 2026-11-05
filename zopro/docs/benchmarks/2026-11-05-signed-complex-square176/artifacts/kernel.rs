//! Signed square difference in the ordinary quadratic extension basis.
use crate::{ifma8::Ifma8,FpConfig};
use core::{arch::x86_64::*,marker::PhantomData};
const MASK:u64=(1u64<<52)-1;
#[inline(always)]fn splat(x:u64)->__m512i{unsafe{_mm512_set1_epi64(x as i64)}}
const fn split(x:[u64;4])->[u64;5]{[x[0]&MASK,((x[0]>>52)|(x[1]<<12))&MASK,((x[1]>>40)|(x[2]<<24))&MASK,((x[2]>>28)|(x[3]<<36))&MASK,x[3]>>16]}
impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    /// Inputs normalized and <=2p; requires4p<R. Signed REDC lies in(-p,2p).
    /// Its only normalization is adding p to negative lanes.
    #[inline(always)]
    pub fn signed_square_difference(&self,b:&Self)->Self{unsafe{
        let zero=splat(0);let mask=splat(MASK);let p=split(C::P).map(splat);let inv=splat(C::INV&MASK);let mut t=[zero;6];
        macro_rules! row{($i:literal)=>{{
            macro_rules! pair{($j:literal,$k:literal)=>{{if $j>=$i{
                let al=_mm512_madd52lo_epu64(zero,self.0[$i],self.0[$j]);let ah=_mm512_madd52hi_epu64(zero,self.0[$i],self.0[$j]);
                let bl=_mm512_madd52lo_epu64(zero,b.0[$i],b.0[$j]);let bh=_mm512_madd52hi_epu64(zero,b.0[$i],b.0[$j]);
                let lo=_mm512_sub_epi64(al,bl);let hi=_mm512_sub_epi64(ah,bh);
                let(lo,hi)=if $i==$j{(lo,hi)}else{(_mm512_slli_epi64::<1>(lo),_mm512_slli_epi64::<1>(hi))};
                t[$j]=_mm512_add_epi64(t[$j],lo);t[$k]=_mm512_add_epi64(t[$k],hi);
            }}};}pair!(0,1);pair!(1,2);pair!(2,3);pair!(3,4);pair!(4,5);
            let m=_mm512_madd52lo_epu64(zero,t[0],inv);let m=_mm512_and_si512(m,if MIXED&&$i==4{splat((1u64<<48)-1)}else{mask});
            let low=if MIXED&&$i==4{
                let carry=_mm512_srai_epi64::<48>(_mm512_add_epi64(t[0],splat((1u64<<48)-1)));
                _mm512_madd52hi_epu64(carry,_mm512_slli_epi64::<4>(m),p[0])
            }else{let carry=_mm512_srai_epi64::<52>(_mm512_add_epi64(t[0],mask));_mm512_madd52hi_epu64(carry,m,p[0])};
            macro_rules! cancel{($j:literal,$k:literal)=>{{t[$j]=_mm512_madd52lo_epu64(t[$j],m,p[$j]);t[$k]=_mm512_madd52hi_epu64(t[$k],m,p[$j]);}};}
            cancel!(1,2);cancel!(2,3);cancel!(3,4);cancel!(4,5);
            if MIXED&&$i==4{
                let mask48=splat((1u64<<48)-1);
                t=[_mm512_add_epi64(low,_mm512_slli_epi64::<4>(_mm512_and_si512(t[1],mask48))),
                _mm512_add_epi64(_mm512_srai_epi64::<48>(t[1]),_mm512_slli_epi64::<4>(_mm512_and_si512(t[2],mask48))),
                _mm512_add_epi64(_mm512_srai_epi64::<48>(t[2]),_mm512_slli_epi64::<4>(_mm512_and_si512(t[3],mask48))),
                _mm512_add_epi64(_mm512_srai_epi64::<48>(t[3]),_mm512_slli_epi64::<4>(_mm512_and_si512(t[4],mask48))),
                _mm512_add_epi64(_mm512_srai_epi64::<48>(t[4]),_mm512_slli_epi64::<4>(t[5])),zero];
            }else{t[1]=_mm512_add_epi64(t[1],low);t=[t[1],t[2],t[3],t[4],t[5],zero];}
        }};}row!(0);row!(1);row!(2);row!(3);row!(4);
        for j in 0..4{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srai_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],mask);}
        let negative=_mm512_cmplt_epi64_mask(t[4],zero);
        for j in 0..5{t[j]=_mm512_add_epi64(t[j],_mm512_maskz_mov_epi64(negative,p[j]));}
        for j in 0..4{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srai_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],mask);}
        debug_assert_eq!(_mm512_cmplt_epi64_mask(t[4],zero),0);debug_assert_eq!(_mm512_cmpge_epu64_mask(t[4],splat(1u64<<52)),0);
        Self([t[0],t[1],t[2],t[3],t[4]],PhantomData)
    }}
}

#[cfg(test)]mod tests{
 use super::*;use crate::{Fp,FqConfig,FrConfig};use ark_ff::{Field,PrimeField};
 fn raw<C:FpConfig,const M:bool>(values:&[[u64;4];8])->Ifma8<C,M>{let digits=values.map(split);Ifma8(std::array::from_fn(|j|unsafe{core::mem::transmute::<[u64;8],__m512i>(std::array::from_fn(|i|digits[i][j]))}),PhantomData)}
 fn words<C:FpConfig,const M:bool>(v:&Ifma8<C,M>)->[[u64;4];8]{let t:[[u64;8];5]=v.0.map(|v|unsafe{core::mem::transmute(v)});let limit=split(crate::portable::add4(C::P,C::P).0);std::array::from_fn(|i|{for j in 0..5{assert!(t[j][i]<1u64<<52);}assert!((0..5).rev().map(|j|t[j][i]).cmp((0..5).rev().map(|j|limit[j])).is_lt());[t[0][i]|t[1][i]<<52,t[1][i]>>12|t[2][i]<<40,t[2][i]>>24|t[3][i]<<28,t[3][i]>>36|t[4][i]<<16]})}
 fn numeric<A:PrimeField>(w:[u64;4])->A{let bytes=w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();A::from_le_bytes_mod_order(&bytes)}
 fn one<C:FpConfig,A:PrimeField,const M:bool>()->usize{
  let p=C::P;let two=crate::portable::add4(p,p).0;let one=[1,0,0,0];let edges=[[0;4],one,[2,0,0,0],crate::portable::sub4(p,one).0,p,crate::portable::add4(p,one).0,crate::portable::sub4(two,one).0,two];let mut pairs=Vec::new();for a in edges{for b in edges{pairs.push((a,b));}}
  for bit in 0..255usize{let mut x=[0u64;4];x[bit/64]=1u64<<(bit%64);for v in[crate::portable::sub4(x,one).0,x,crate::portable::add4(x,one).0]{if !crate::portable::less(two,v){pairs.push((v,crate::portable::sub4(two,v).0));pairs.push((v,v));}}}
  let mut seed=0x13198a2e03707344u64;let mut next=||{let mut bytes=[0u8;64];for c in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;c.copy_from_slice(&seed.to_le_bytes());}Fp::<C>::from_random_bytes(&bytes).0};for i in 0..4096{let mut a=next();let mut b=next();if i%2==0{a=crate::portable::add4(a,p).0;}if i%3==0{b=crate::portable::add4(b,p).0;}pairs.push((a,b));}
  let inv=A::from(2u64).pow([if M{256}else{260}]).inverse().unwrap();let mut count=0;
  for chunk in pairs.chunks(8){let a=std::array::from_fn(|i|chunk[i%chunk.len()].0);let b=std::array::from_fn(|i|chunk[i%chunk.len()].1);let av=raw::<C,M>(&a);let bv=raw::<C,M>(&b);let got=av.signed_square_difference(&bv);let output=words(&got);let original=words(&av.add(&bv).mul(&av.sub(&bv)));for i in 0..8{let aa=numeric::<A>(a[i]);let bb=numeric::<A>(b[i]);assert_eq!(numeric::<A>(output[i]),(aa.square()-bb.square())*inv);assert_eq!(numeric::<A>(output[i]),numeric::<A>(original[i]));count+=1;}}
  let mut power=[0u64;4];let bit=if M{128}else{130};power[bit/64]=1u64<<(bit%64);for mask in 0..256u32{let a=std::array::from_fn(|i|if mask>>i&1!=0{[0;4]}else{power});let b=std::array::from_fn(|i|if mask>>i&1!=0{power}else{[0;4]});let got=words(&raw::<C,M>(&a).signed_square_difference(&raw::<C,M>(&b)));for i in 0..8{assert_eq!(numeric::<A>(got[i]),if mask>>i&1!=0{-A::ONE}else{A::ONE});count+=1;}}
  count
 }
 #[test]fn signed_square_difference_all_fields_radices_boundaries_masks_and_oracles(){let n=one::<FqConfig,ark_ref::Fq,true>()+one::<FqConfig,ark_ref::Fq,false>()+one::<FrConfig,ark_ref::Fr,true>()+one::<FrConfig,ark_ref::Fr,false>();eprintln!("SIGNED_SQUARE_RAW_OUTPUTS={n}");assert_eq!(n,30976);}
 fn chain<C:FpConfig,A:PrimeField,const M:bool>()->usize{
  let a=std::array::from_fn(|i|Fp::<C>::from_u64(37+i as u64));let b=std::array::from_fn(|i|Fp::<C>::from_u64(71+i as u64));let(mut x,mut y)=(Ifma8::<C,M>::new(&a),Ifma8::<C,M>::new(&b));let mut expected=a.map(|v|numeric::<A>(v.to_canonical()));let mut other=b.map(|v|numeric::<A>(v.to_canonical()));let inv=A::from(2u64).pow([if M{256}else{260}]).inverse().unwrap();let mut count=0;
  for round in 0..1024{let original=x.add(&y).mul(&x.sub(&y));let real=x.signed_square_difference(&y);let product=x.mul(&y);let imaginary=product.add(&product);let rr=words(&real);let ii=words(&imaginary);let oo=words(&original);for i in 0..8{let real=expected[i].square()-other[i].square();let imaginary=expected[i]*other[i]+expected[i]*other[i];assert_eq!(numeric::<A>(rr[i])*inv,real);assert_eq!(numeric::<A>(ii[i])*inv,imaginary);assert_eq!(numeric::<A>(rr[i]),numeric::<A>(oo[i]));expected[i]=imaginary;other[i]=real;count+=2;}x=imaginary;y=real;if round%17==0{x=x.add(&Ifma8::one());for e in&mut expected{*e+=A::ONE;}}}
  count
 }
 #[test]fn ordinary_complex_square_dependent_chains_match_independent_field_oracle(){let n=chain::<FqConfig,ark_ref::Fq,true>()+chain::<FqConfig,ark_ref::Fq,false>()+chain::<FrConfig,ark_ref::Fr,true>()+chain::<FrConfig,ark_ref::Fr,false>();assert_eq!(n,65536);eprintln!("SIGNED_COMPLEX_CHAIN_OUTPUTS={n}");}
}

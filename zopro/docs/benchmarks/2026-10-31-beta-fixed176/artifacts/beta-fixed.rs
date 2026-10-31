// Fixed canonical beta and quotient; no input-dependent or key-derived table.
const BETA_TEXT:&str="21888242871839275220042445260109153167277707414472061641714758635765020556616";
const fn beta_words()->[u64;4]{let mut out=[0u64;4];let bytes=BETA_TEXT.as_bytes();let mut k=0;while k<bytes.len(){let mut carry=(bytes[k]-b'0')as u128;let mut i=0;while i<4{let v=out[i]as u128*10+carry;out[i]=v as u64;carry=v>>64;i+=1;}assert!(carry==0);k+=1;}out}
const fn beta_quotient()->[u64;5]{let p=crate::FqConfig::P;let mut rem=beta_words();let mut q=[0u64;5];let mut k=0;while k<260{rem=crate::portable::add4(rem,rem).0;let ge=!crate::portable::less(rem,p);if ge{rem=crate::portable::sub4(rem,p).0;}let mut carry=ge as u64;let mut j=0;while j<5{let v=(q[j]<<1)|carry;q[j]=v&MASK;carry=v>>52;j+=1;}assert!(carry==0);k+=1;}q}
const BETA_B:[u64;5]=split(beta_words());
const BETA_Q:[u64;5]=beta_quotient();
impl Ifma8<crate::FqConfig,true>{
 /// Multiply a normalized numerical R256 operand below2p by canonical beta.
 #[inline(always)]
 pub fn mul_beta_fixed(&self)->Self{
  unsafe{
   let zero=Self::splat(0);let mask=Self::splat(MASK);
   let factor=BETA_B.map(Self::splat);let quotient=BETA_Q.map(Self::splat);
   let mut high=[zero;6];
   for i in 0..5{for j in 0..5{let k=i+j;if k==3{high[0]=_mm512_madd52hi_epu64(high[0],self.0[i],quotient[j]);}else if k>=4{high[k-4]=_mm512_madd52lo_epu64(high[k-4],self.0[i],quotient[j]);high[k-3]=_mm512_madd52hi_epu64(high[k-3],self.0[i],quotient[j]);}}}
   for j in 0..5{high[j+1]=_mm512_add_epi64(high[j+1],_mm512_srli_epi64::<52>(high[j]));high[j]=_mm512_and_si512(high[j],mask);}
   let q=[high[1],high[2],high[3],high[4],high[5]];let p=split(crate::FqConfig::P).map(Self::splat);let mut out=[zero;5];
   for i in 0..5{for j in 0..5-i{let k=i+j;out[k]=_mm512_sub_epi64(_mm512_madd52lo_epu64(out[k],self.0[i],factor[j]),_mm512_madd52lo_epu64(zero,q[i],p[j]));if k<4{out[k+1]=_mm512_sub_epi64(_mm512_madd52hi_epu64(out[k+1],self.0[i],factor[j]),_mm512_madd52hi_epu64(zero,q[i],p[j]));}}}
   let mut carry=zero;for digit in &mut out{let v=_mm512_add_epi64(*digit,carry);*digit=_mm512_and_si512(v,mask);carry=_mm512_srai_epi64::<52>(v);}
   Self(out,PhantomData)
  }
 }
}

#[cfg(test)]mod beta_fixed_tests{
 use super::*;use crate::{Fq,FqConfig,Fq2,G2Affine,G2Projective};use ark_ff::{BigInt,PrimeField};
 fn ark(a:Fq)->ark_ref::Fq{ark_ref::Fq::from_bigint(BigInt(a.to_canonical())).unwrap()}
 #[test]fn constants_lazy_values_and_changing_chains_match_arkworks(){
  let beta=Fq::from_decimal(BETA_TEXT);assert_eq!(beta.to_canonical(),beta_words());assert_eq!(beta*beta*beta,Fq::ONE);let two=crate::portable::add4(FqConfig::P,FqConfig::P).0;
  let mut values=vec![[0;4],[1,0,0,0],crate::portable::sub4(FqConfig::P,[1,0,0,0]).0,FqConfig::P,crate::portable::add4(FqConfig::P,[1,0,0,0]).0,crate::portable::sub4(two,[1,0,0,0]).0];
  for bit in 0..255{let mut w=[0u64;4];w[bit/64]=1u64<<(bit%64);if crate::portable::less(w,two){values.push(w);if w!=[0;4]{values.push(crate::portable::sub4(w,[1,0,0,0]).0);}}}
  let mut seed=0x437a94fc3321e9d7u64;for _ in 0..8192{let mut w=[0;4];for x in &mut w{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;*x=seed;}w[3]&=(1u64<<63)-1;while !crate::portable::less(w,two){w=crate::portable::sub4(w,two).0;}values.push(w);}
  let expected_beta=ark(beta);let mut lanes=0;for row in values.chunks(8){let input:[Fq;8]=core::array::from_fn(|i|{let mut w=row[i%row.len()];if !crate::portable::less(w,FqConfig::P){w=crate::portable::sub4(w,FqConfig::P).0;}Fq::from_montgomery(w)});let raw=core::array::from_fn(|i|Fq::from_montgomery(row[i%row.len()]));let mut x=Ifma8::<FqConfig,true>::new(&raw);let mut expected=input.map(ark);for _ in 0..12{let next=x.mul_beta_fixed();let packed=next.compact().expand();for limb in unsafe{core::mem::transmute::<_,[[u64;8];5]>(packed.0)}{assert!(limb.iter().all(|&v|v<=MASK));}let got=next.get();for i in 0..8{expected[i]*=expected_beta;assert_eq!(ark(got[i]),expected[i]);lanes+=1;}x=next;}}
  assert!(lanes>100000);eprintln!("BETA_FIXED_ARKWORKS_LANES {lanes}");
 }
 #[test]fn complete_point_transforms_cover_tails_and_threefold_identity(){
  let beta=Fq::from_decimal(BETA_TEXT);let base=G2Projective::batch_to_affine(&(0..73).map(|i|G2Affine::GENERATOR*crate::Fr::from(i+1)).collect::<Vec<_>>());let factor=Ifma8::<FqConfig,true>::new(&[beta;8]);
  for n in [0usize,1,2,3,4,5,7,8,9,31,63,64,65,2047,2048,2049]{let original=(0..n).map(|i|if i%17==0{G2Affine::IDENTITY}else{base[i%73]}).collect::<Vec<_>>();let mut fixed=original.clone();let mut ordinary=original.clone();for _ in 0..3{for p in &mut ordinary{if !p.is_identity(){p.x=p.x.mul_by_fp(&beta);}}let full=n/4*4;for block in fixed[..full].chunks_exact_mut(4){let x=Ifma8::<FqConfig,true>::new(&core::array::from_fn(|i|if i%2==0{block[i/2].x.c0}else{block[i/2].x.c1}));let y=x.mul_beta_fixed().get();let check=x.mul(&factor).get();assert_eq!(y,check);for j in 0..4{block[j].x=Fq2::new(y[2*j],y[2*j+1]);}}for p in &mut fixed[full..]{if !p.is_identity(){p.x=p.x.mul_by_fp(&beta);}}assert_eq!(fixed,ordinary);}assert_eq!(fixed,original);}
 }
}

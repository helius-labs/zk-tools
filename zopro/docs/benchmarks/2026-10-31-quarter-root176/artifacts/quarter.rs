// One globally fixed small fourth root and exact quotient.
const Q4_SMALL_WORDS:[u64;4]=[2364373463638985163,14648454017743979685,12953715498635827032,0];
const fn quarter_quotient()->[u64;5]{let p=crate::FrConfig::P;let mut rem=Q4_SMALL_WORDS;let mut q=[0u64;5];let mut k=0;while k<260{rem=crate::portable::add4(rem,rem).0;let ge=!crate::portable::less(rem,p);if ge{rem=crate::portable::sub4(rem,p).0;}let mut carry=ge as u64;let mut j=0;while j<5{let v=(q[j]<<1)|carry;q[j]=v&MASK;carry=v>>52;j+=1;}assert!(carry==0);k+=1;}q}
const Q4_B:[u64;5]=split(Q4_SMALL_WORDS);
const Q4_Q:[u64;5]=quarter_quotient();
impl Ifma<crate::FrConfig,false>{
 /// Multiply a normalized numerical R260 operand below2p by canonical small fourth root.
 #[inline(always)]
 pub fn mul_quarter_small(&self)->Self{
  unsafe{
   let zero=Self::splat(0);let mask=Self::splat(MASK);
   let factor=Q4_B.map(Self::splat);let quotient=Q4_Q.map(Self::splat);
   let mut high=[zero;6];
   for i in 0..5{for j in 0..5{let k=i+j;if k==3{high[0]=_mm256_madd52hi_epu64(high[0],self.0[i],quotient[j]);}else if k>=4{high[k-4]=_mm256_madd52lo_epu64(high[k-4],self.0[i],quotient[j]);high[k-3]=_mm256_madd52hi_epu64(high[k-3],self.0[i],quotient[j]);}}}
   for j in 0..5{high[j+1]=_mm256_add_epi64(high[j+1],_mm256_srli_epi64::<52>(high[j]));high[j]=_mm256_and_si256(high[j],mask);}
   let q=[high[1],high[2],high[3],high[4],high[5]];let p=split(crate::FrConfig::P).map(Self::splat);let mut out=[zero;5];
   for i in 0..5{for j in 0..5-i{let k=i+j;out[k]=_mm256_sub_epi64(_mm256_madd52lo_epu64(out[k],self.0[i],factor[j]),_mm256_madd52lo_epu64(zero,q[i],p[j]));if k<4{out[k+1]=_mm256_sub_epi64(_mm256_madd52hi_epu64(out[k+1],self.0[i],factor[j]),_mm256_madd52hi_epu64(zero,q[i],p[j]));}}}
   let mut carry=zero;for digit in &mut out{let v=_mm256_add_epi64(*digit,carry);*digit=_mm256_and_si256(v,mask);carry=_mm256_srai_epi64::<52>(v);}
   Self(out,PhantomData)
  }
 }
}

#[cfg(test)]mod quarter_root_tests{
 use super::*;use crate::{Fr,FrConfig};use ark_ff::{BigInt,PrimeField};
 fn ark(v:Fr)->ark_ref::Fr{ark_ref::Fr::from_bigint(BigInt(v.to_canonical())).unwrap()}
 #[test]fn fixed_fourth_root_matches_arkworks_with_lazy_values_and_chains(){
  let k=Fr::from_canonical(Q4_SMALL_WORDS).unwrap();let i=Fr::root_of_unity(4).unwrap();assert_eq!(k,-i);assert_eq!(k*k,-Fr::ONE);let two=crate::portable::add4(FrConfig::P,FrConfig::P).0;
  let mut values=vec![[0;4],[1,0,0,0],FrConfig::P,crate::portable::sub4(FrConfig::P,[1,0,0,0]).0,crate::portable::add4(FrConfig::P,[1,0,0,0]).0,crate::portable::sub4(two,[1,0,0,0]).0];for bit in 0..255{let mut w=[0;4];w[bit/64]=1u64<<(bit%64);if crate::portable::less(w,two){values.push(w);values.push(crate::portable::sub4(w,[1,0,0,0]).0);}}
  let mut seed=0x3762fa1045cf0d39u64;for _ in 0..8192{let mut w=[0;4];for x in &mut w{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;*x=seed;}w[3]&=(1u64<<63)-1;while !crate::portable::less(w,two){w=crate::portable::sub4(w,two).0;}values.push(w);}
  let mut count=0;for row in values.chunks(4){let canonical:[Fr;4]=std::array::from_fn(|j|{let mut w=row[j%row.len()];if !crate::portable::less(w,FrConfig::P){w=crate::portable::sub4(w,FrConfig::P).0;}Fr::from_montgomery(w)});let raw=std::array::from_fn(|j|Fr::from_montgomery(row[j%row.len()]));let mut x=Ifma::<FrConfig,false>::new(&raw);let mut expected=canonical.map(ark);for pass in 0..8{let positive=x.mul_quarter_small();let negative=Ifma::<FrConfig,false>::zero().sub(&positive);let got=if pass%2==0{positive}else{negative};let scalar=got.get();for j in 0..4{expected[j]*=if pass%2==0{ark(k)}else{ark(i)};assert_eq!(ark(scalar[j]),expected[j]);count+=1;}x=got;}}
  assert!(count>69000);eprintln!("QUARTER_ROOT_ARKWORKS_OUTPUTS {count}");
 }
 #[test]fn both_orientations_restore_values_after_four_root_applications(){let k=Fr::from_canonical(Q4_SMALL_WORDS).unwrap();for sign in[false,true]{for seed in 0..128u64{let input=std::array::from_fn(|j|Fr::from(seed+j as u64+1).pow(&[251]));let mut x=Ifma::<FrConfig,false>::new(&input);for _ in 0..4{x=x.mul_quarter_small();if sign{x=Ifma::<FrConfig,false>::zero().sub(&x);}}assert_eq!(x.get(),input);assert_eq!(k*k,-Fr::ONE);}}}
}

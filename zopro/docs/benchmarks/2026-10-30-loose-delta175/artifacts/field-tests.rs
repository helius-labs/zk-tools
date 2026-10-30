#[cfg(test)]mod loose_delta_tests{
use super::*;use ark_ff::{Field,PrimeField};
fn check<C:FpConfig,A:PrimeField,const MIXED:bool>(){
 let p=C::P;let twice=crate::portable::add4(p,p).0;let anchor_edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p];let normal_edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
 let numeric=|w:[u64;4]|A::from_le_bytes_mod_order(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());let inv=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();let hostinv=A::from(2u64).pow([256]).inverse().unwrap();let mut seed=0x4c4f4f534544454cu64;let limit=split(twice);let mut checked=0u64;
 for group in 0usize..(20736+4096)/8{
  let input:[[[u64;4];8];6]=core::array::from_fn(|slot|core::array::from_fn(|lane|{let i=group*8+lane;if i<20736{let mut divisor=1;for prev in 0..slot{divisor*=if prev==2||prev==3{4}else{6};}if slot==2||slot==3{anchor_edges[(i/divisor)%4]}else{normal_edges[(i/divisor)%6]}}else{let mut bytes=[0u8;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}let value=Fp::<C>::from_random_bytes(&bytes).0;if slot!=2&&slot!=3&&seed&1==0{crate::portable::add4(value,p).0}else{value}}}));
  let v=input.map(|w|Ifma8::<C,MIXED>::raw(&w));let scalar=input.map(|w|w.map(|a|numeric(a)*inv));let mut slope=[v[0],v[1]];let mut expected=[scalar[0],scalar[1]];
  for _ in 0..8{let out=Ifma8::complex_canonical_delta(&slope[0],&slope[1],&v[2],&v[3],&v[4],&v[5]);let next:[[_;8];2]=core::array::from_fn(|slot|core::array::from_fn(|lane|{let d0=scalar[2][lane]-scalar[4][lane];let d1=scalar[3][lane]-scalar[5][lane];if slot==0{expected[0][lane]*d0-expected[1][lane]*d1}else{expected[0][lane]*d1+expected[1][lane]*d0}}));
   for slot in 0..2{let host=out[slot].get();let digits:[[u64;8];5]=out[slot].0.map(|v|unsafe{core::mem::transmute(v)});for lane in 0..8{assert_eq!(numeric(host[lane].0)*hostinv,next[slot][lane]);assert!((0..5).all(|j|digits[j][lane]<(1u64<<52)));assert!((0..5).rev().map(|j|digits[j][lane]).cmp((0..5).rev().map(|j|limit[j])).is_lt());checked+=1;}}
   slope=out;expected=next;
  }
 }
 eprintln!("LOOSE_DELTA_CHECKED_OUTPUTS={checked};MIXED={MIXED}");
}
#[test]fn fq_canonical_delta_arkworks(){check::<crate::FqConfig,ark_ref::Fq,true>();check::<crate::FqConfig,ark_ref::Fq,false>();}
#[test]fn fr_canonical_delta_arkworks(){check::<crate::FrConfig,ark_ref::Fr,true>();check::<crate::FrConfig,ark_ref::Fr,false>();}
}

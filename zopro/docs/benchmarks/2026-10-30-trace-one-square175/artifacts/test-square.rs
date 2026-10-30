mod trace_one_square_tests {
 use super::*;use ark_ff::{Field,PrimeField};type A=ark_ref::Fq;
 fn check<const MIXED:bool>(){
  type C=crate::FqConfig;let p=C::P;let twice=crate::portable::add4(p,p).0;
  let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
  let numeric=|w:[u64;4]|A::from_le_bytes_mod_order(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());
  let ri=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();let hostri=A::from(2u64).pow([256]).inverse().unwrap();
  let limit=split(twice);let mut seed=0x746f737175617265u64;let mut checked=0;
  for group in 0..516 {
   let raw:[[[u64;4];8];2]=core::array::from_fn(|slot|core::array::from_fn(|lane|{let i=group*8+lane;if i<36{edges[(i/6usize.pow(slot as u32))%6]}else{let mut bytes=[0u8;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}let w=Fp::<C>::from_random_bytes(&bytes).0;if seed&1==0{w}else{crate::portable::add4(w,p).0}}}));
   let mut x=raw.map(|v|Ifma8::<C,MIXED>::raw(&v));let mut expected=raw.map(|v|v.map(|w|numeric(w)*ri));
   for _ in 0..8 {
    let out=Ifma8::trace_one_square(&x[0],&x[1]);let a=expected[0];let b=expected[1];expected=[core::array::from_fn(|i|a[i]*a[i]+b[i]*b[i]),core::array::from_fn(|i|b[i]*(a[i]+a[i]+b[i]))];
    for slot in 0..2 {let host=out[slot].get();let digits:[[u64;8];5]=out[slot].0.map(|v|unsafe{core::mem::transmute(v)});for lane in 0..8{assert_eq!(numeric(host[lane].0)*hostri,expected[slot][lane]);assert!((0..5).all(|j|digits[j][lane]<(1u64<<52)));assert!((0..5).rev().map(|j|digits[j][lane]).cmp((0..5).rev().map(|j|limit[j])).is_lt());checked+=1;}}
    x=out;
   }
  }assert_eq!(checked,66048);eprintln!("TRACE_ONE_SQUARE_OUTPUTS={checked};MIXED={MIXED}");
 }
 #[test]fn fq_r256_shared_symmetric_square(){check::<true>();}
 #[test]fn fq_r260_shared_symmetric_square(){check::<false>();}
}

#[cfg(test)]
mod signed_difference_tests{
use super::*;use ark_ff::{Field,PrimeField};
fn check<C:FpConfig,A:PrimeField,const MIXED:bool>(){
let p=C::P;let twice=crate::portable::add4(p,p).0;let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
let numeric=|w:[u64;4]|A::from_le_bytes_mod_order(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());
let inv=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();let hostinv=A::from(2u64).pow([256]).inverse().unwrap();let mut seed=0x434f4d504c454d45u64;let limit=split(twice);let mut checked=0;
for group in 0..(1296+4096)/8{
let input:[[[u64;4];8];4]=core::array::from_fn(|slot|core::array::from_fn(|lane|{let i=group*8+lane;if i<1296{edges[(i/6usize.pow(slot as u32))%6]}else{let mut bytes=[0u8;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}let v=Fp::<C>::from_random_bytes(&bytes).0;if seed&1==0{v}else{crate::portable::add4(v,p).0}}}));
let v=input.map(|w|Ifma8::<C,MIXED>::raw(&w));let scalar=input.map(|w|w.map(|a|numeric(a)*inv));let mut x=v[0];let mut expected=scalar[0];
for _ in 0..8{x=x.difference2_signed(&v[1],&v[2],&v[3]);let host=x.get();let digits:[[u64;8];5]=x.0.map(|v|unsafe{core::mem::transmute(v)});for lane in 0..8{expected[lane]=expected[lane]*scalar[1][lane]-scalar[2][lane]*scalar[3][lane];assert_eq!(numeric(host[lane].0)*hostinv,expected[lane]);assert!((0..5).all(|j|digits[j][lane]<(1u64<<52)));assert!((0..5).rev().map(|j|digits[j][lane]).cmp((0..5).rev().map(|j|limit[j])).is_lt());}checked+=8;}}
eprintln!("SIGNED_NATIVE_LANES={checked};MIXED={MIXED}");}
#[test]fn fq_complement_chains_arkworks(){check::<crate::FqConfig,ark_ref::Fq,true>();check::<crate::FqConfig,ark_ref::Fq,false>();}
#[test]fn fr_complement_chains_arkworks(){check::<crate::FrConfig,ark_ref::Fr,true>();check::<crate::FrConfig,ark_ref::Fr,false>();}
}

#![allow(dead_code)]
pub use zopro_bn254::{Fp,FpConfig,FqConfig,FrConfig};
#[path="/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/portable.rs"]mod portable;
#[path="candidate.rs"]mod candidate;
use num_bigint::BigUint;
use std::{cell::Cell,fs};
mod observe{use super::Cell;thread_local!{static COUNTS:Cell<[u64;4]>=const{Cell::new([0;4])};}pub fn hit(i:usize){COUNTS.with(|c|{let mut a=c.get();a[i]+=1;c.set(a)});}pub fn take()->[u64;4]{COUNTS.with(|c|c.replace([0;4]))}}
fn big(w:&[u64])->BigUint{BigUint::from_bytes_le(&w.iter().flat_map(|x|x.to_le_bytes()).collect::<Vec<_>>())}
fn words(x:&BigUint)->[u64;4]{let v=x.to_u64_digits();assert!(v.len()<=4);let mut w=[0;4];w[..v.len()].copy_from_slice(&v);w}
fn rng(s:&mut u64)->u64{*s=s.wrapping_add(0x9e3779b97f4a7c15);let mut z=*s;z=(z^(z>>30)).wrapping_mul(0xbf58476d1ce4e5b9);z=(z^(z>>27)).wrapping_mul(0x94d049bb133111eb);z^(z>>31)}
fn inverse_reference<C:FpConfig>(x:&[u64;4])->[u64;4]{let p=big(&C::P);words(&((big(x).modpow(&(&p-2u8),&p)*big(&C::R2))%p))}
fn check<C:FpConfig>(name:&str)->String{
 let p=big(&C::P);let h=BigUint::from(1u8)<<64usize;let r=BigUint::from(1u8)<<256usize;
 let seed=big(&portable::power_of_two(576,C::P));assert_eq!(seed, (&h*big(&C::R2))%&p);let correction=big(&portable::power_of_two(192,C::P));assert_eq!((&h*correction)%&p, &r%&p);
 let mut values=vec![[0;4],C::R,[1,0,0,0],words(&(&p-1u8)),words(&(&p-2u8))];
 for bit in 0..254{let power=BigUint::from(1u8)<<bit;values.push(words(&power));values.push(words(&(&p-&power)));}
 let mut seed_rng=0x57ed6b120ac941f3;for _ in 0..6144{let w:[u64;4]=std::array::from_fn(|_|rng(&mut seed_rng));values.push(words(&(big(&w)%&p)));}
 observe::take();let mut complete=0;for w in &values{let x=Fp::<C>::from_montgomery(*w);let got=candidate::inverse(&x);if *w==[0;4]{assert_eq!(got,None);continue;}let got=got.unwrap();assert!(got.is_reduced());assert_eq!(got.0,inverse_reference::<C>(w));assert_eq!(got.0,x.inverse().unwrap().0);assert_eq!(x*got,Fp::ONE);complete+=1;}
 let mut x=Fp::<C>::from(43);for k in 0..512{let got=candidate::inverse(&x).unwrap();assert_eq!(got.0,inverse_reference::<C>(&x.0));x=got+Fp::from((k+3)as u64);assert_ne!(x,Fp::ZERO);complete+=1;}let inverse_branches=observe::take();assert!(inverse_branches[1]>0);
 let wanted=Fp::<C>::from(37);let scale=Fp::<C>::from_montgomery(portable::power_of_two(320,C::P));let mut pairs=vec![(0,1),(1,1),(u64::MAX,1),(1,u64::MAX),(u64::MAX,u64::MAX-1),(7540113804746346429,12200160415121876738),(12200160415121876738,7540113804746346429)];
 for _ in 0..4096{let a=rng(&mut seed_rng);let b=rng(&mut seed_rng)|1;let(mut c,mut d)=(a,b);while d!=0{(c,d)=(d,c%d);}if c==1{pairs.push((a,b));}}
 for &(a,b)in &pairs{let u=(Fp::<C>::from(a)*wanted*scale).0;let v=(Fp::<C>::from(b)*wanted*scale).0;let got=candidate::probe_finish::<C>(a,b,u,v);assert_eq!(got.0,wanted.0);assert!(got.is_reduced());}let finish_branches=observe::take();assert!(finish_branches[1]>0&&finish_branches[2]>=3);
 println!("PASS {name}: {complete} complete nonzero inverse outputs, {} crafted finish states; complete branches {:?}, finish branches {:?}",pairs.len(),inverse_branches,finish_branches);
 format!("{{\"field\":\"{name}\",\"complete_inverse_outputs\":{complete},\"input_rows\":{},\"changing_chain_steps\":512,\"crafted_finish_states\":{},\"inverse_branches_zero_normal_b1_outer\":{:?},\"crafted_finish_branches\":{:?},\"seed_identity\":true,\"early_correction_identity\":true}}",values.len(),pairs.len(),inverse_branches,finish_branches)
}
fn main(){let fq=check::<FqConfig>("Fq");let fr=check::<FrConfig>("Fr");let dest="/home/ubuntu/src/zopro-perf-work/inverse-seed-scale176/qualification.json";assert!(!std::path::Path::new(dest).exists());fs::write(dest,format!("{{\"passed\":true,\"scope\":\"New instrumented inverse prototype with accepted arithmetic; not integrated native library, proof or timing\",\"fields\":[{fq},{fr}],\"new_proofs\":0,\"new_timings\":0}}")).unwrap();}

use num_bigint::{BigInt,BigUint,Sign};use serde_json::{json,Value};use std::fs;
use zopro_bn254::{Field,Fp,FpConfig,FqConfig,FrConfig};use rayon::prelude::*;
mod native;
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/unreduced-gcd-native176";
fn raw(w:[u64;4])->BigUint{BigUint::from_bytes_le(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())}
fn words(x:&BigUint)->[u64;4]{let w=x.to_u64_digits();assert!(w.len()<=4);std::array::from_fn(|i|w.get(i).copied().unwrap_or(0))}
fn wide(x:&BigInt)->native::Wide {let(_,bytes)=x.to_bytes_le();let mut limbs=[0u64;17];assert!(bytes.len()<=136);for(i,c)in bytes.chunks(8).enumerate(){let mut b=[0;8];b[..c.len()].copy_from_slice(c);limbs[i]=u64::from_le_bytes(b);}let len=(bytes.len().div_ceil(8)).max(1);native::Wide{limbs,len,negative:x.sign()==Sign::Minus}}
fn integer(x:&native::Wide)->BigInt{let bytes=x.limbs[..x.len].iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>();BigInt::from_bytes_le(if x.negative{Sign::Minus}else{Sign::Plus},&bytes)}
fn modulo(x:BigInt,p:&BigInt)->BigInt{let r=x%p;if r.sign()==Sign::Minus{r+p}else{r}}
fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn rows()->usize{
 let mut checked=0;
 let factors=[-(i64::MAX),-(1i64<<62),-1,0,1,1i64<<62,i64::MAX];
 for n in[1usize,2,4,8,16]{
  let bound=(BigInt::from(1u8)<<(64*n))-1u8;
  for delta in[0u64,1,1<<31,u64::MAX]{
   let a=&bound-delta;let b=(&bound>>1usize)+1u8;
   for sign in 0..4 {let a=if sign&1!=0{-&a}else{a.clone()};let b=if sign&2!=0{-&b}else{b.clone()};
    for f in factors{for g in factors{let x=native::Wide::linear(&wide(&a),&wide(&b),f,g);assert_eq!(integer(&x),&a*f+&b*g);checked+=1;}}
   }
  }
 }
 // One whole-word cancellation, negative zero normalization and zero factors.
 for f in factors {let z=native::Wide::linear(&native::Wide::small(0),&native::Wide::small(0),f,-f);assert_eq!(integer(&z),BigInt::from(0));assert!(!z.negative);checked+=1;}
 checked
}
fn field<C:FpConfig>(name:&str,seed:u64)->Value{
 let prime=raw(C::P);let p=BigInt::from(prime.clone());let r2=BigUint::from(1u8)<<512usize;
 let two_inv=BigUint::from(2u8).modpow(&(&prime-BigUint::from(2u8)),&prime);
 let mut corrections=0;
 for k in[31usize,62,93,124,248,279,496,744,992]{
  let bound=(BigInt::from(1u8)<<(k+64))-1u8;
  for a in[BigInt::from(0),BigInt::from(1),(&bound>>1usize),bound]{for neg in[false,true]{let c=if neg{-&a}else{a.clone()};
   let actual=native::correction::<C>(wide(&c),k);
   let factor=two_inv.modpow(&BigUint::from(k),&prime);
   let expected=modulo(&c*BigInt::from(factor)*BigInt::from(r2.clone()),&p).to_biguint().unwrap();assert_eq!(raw(actual.0),expected);corrections+=1;
  }}
 }
 let mut input=vec![BigUint::from(1u8),&prime-BigUint::from(1u8),&prime-BigUint::from(2u8),raw(C::R),raw(C::R2)];
 for bit in 1..254usize {let a=BigUint::from(1u8)<<bit;for x in [&a-BigUint::from(1u8),a.clone(),a+BigUint::from(1u8)]{if x<prime{input.push(x)}}}
 let mut state=seed;for _ in 0..8192 {input.push(raw([next(&mut state),next(&mut state),next(&mut state),next(&mut state)])%(&prime-BigUint::from(1u8))+BigUint::from(1u8));}
 assert_eq!(native::inverse(&Fp::<C>::ZERO),None);
 let mut checks=0;let mut xor=BigUint::from(0u8);let mut expected=Vec::new();
 for x in &input {
  let value=Fp::<C>::from_montgomery(words(x));let original=value;
  let output=native::inverse(&value).unwrap();assert_eq!(value,original);assert_eq!(value*output,Fp::<C>::ONE);
  assert!(raw(output.0)<prime);let reference=x.modpow(&(&prime-BigUint::from(2u8)),&prime)*&r2%&prime;
  assert_eq!(raw(output.0),reference);assert_eq!(output,value.inverse().unwrap());xor^=&reference;expected.push(output);checks+=1;
 }
 let mut threaded=0;
 for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
  pool.install(||{input.par_iter().zip(&expected).for_each(|(x,expected)|{let value=Fp::<C>::from_montgomery(words(x));assert_eq!(native::inverse(&value).unwrap(),*expected);});});threaded+=input.len();
 }
 json!({"field":name,"seed":seed,"complete_inverse_checks":checks,"threaded_inverse_checks":threaded,"worker_counts":[1,4,16,32],"worker_stack_bytes":512*1024,"correction_checks":corrections,"raw_zero_none":true,"canonical_and_input_preservation":true,"output_xor_hex":xor.to_str_radix(16),"limits":"Synthetic qualification of standalone new full inverse. No SDK/production integration, latency, whole-stack bound or branch-complete GCD termination proof."})
}
fn main(){
 assert!(!std::path::Path::new(&format!("{ROOT}/qualification.json")).exists());
 let row_checks=rows();let fields=[field::<FqConfig>("Fq",0x7e40b28c096ad315),field::<FrConfig>("Fr",0xc193a04d7f2b658e)];
 let result=json!({"native_arithmetic_qualification_passed":true,"wide_linear_checks":row_checks,"fields":fields,"production_unchanged":true,"new_proofs":0,"canonical_unchanged":35181,"timings_measured":false,"native_consumer_codegen_reviewed":false,"goal_complete":false});
 fs::write(format!("{ROOT}/qualification.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("Fixed-limb inverse PASS: {row_checks} widened row checks; both fields complete/correction/thread oracles; no timings");
}

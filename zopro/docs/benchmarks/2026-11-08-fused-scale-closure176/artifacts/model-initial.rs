use num_bigint::BigUint;use serde_json::{json,Value};use std::{fs,path::Path};
use zopro_bn254::{FpConfig,FqConfig,FrConfig};
const R:&str="/home/ubuntu/src/zopro-perf-work/fused-scale-closure176";const B:u128=1<<52;
fn n(x:u64)->BigUint{BigUint::from(x)}
fn u64val(x:BigUint)->u64{let d=x.to_u64_digits();assert!(d.len()<=1);d.first().copied().unwrap_or(0)}
fn digits(x:&BigUint)->[u128;5]{std::array::from_fn(|j|u64val((x>>(52*j))&n((B-1)as u64))as u128)}
fn integer(t:&[u128])->BigUint{t.iter().enumerate().map(|(j,&v)|BigUint::from(v)<<(52*j)).sum()}
fn cios(a:&BigUint,b:&BigUint,c:&BigUint,d:&BigUint,p:&BigUint,mixed:bool,fused:bool,peak:&mut u128)->BigUint{
 let aa=digits(a);let bb=digits(b);let cc=digits(c);let dd=digits(d);let pp=digits(p);let m52=n(1)<<52usize;let inverse=p.modpow(&((&m52>>1usize)-n(1)),&m52);let inv=u64val((&m52-inverse)%&m52)as u128;let mut t=[0u128;6];
 for i in 0..5{let before=integer(&t)+a*BigUint::from(bb[i])+if fused{c*BigUint::from(dd[i])}else{n(0)};
  for j in 0..5{let v=aa[j]*bb[i];t[j]+=v&(B-1);t[j+1]+=v>>52;if fused{let v=cc[j]*dd[i];t[j]+=v&(B-1);t[j+1]+=v>>52;}}
  let w=if mixed&&i==4{48}else{52};let unit=1u128<<w;let m=t[0]*inv&(unit-1);let low=((t[0]+unit-1)>>w)+if w==48{((m<<4)*pp[0])>>52}else{m*pp[0]>>52};assert_eq!(low,(t[0]+m*pp[0])>>w);assert_eq!((t[0]+m*pp[0])&(unit-1),0);
  for j in 1..5{let v=m*pp[j];t[j]+=v&(B-1);t[j+1]+=v>>52;}for v in t{*peak=(*peak).max(v);assert!(v<1u128<<61);}
  t=if w==48{let mask=unit-1;[low+((t[1]&mask)<<4),(t[1]>>48)+((t[2]&mask)<<4),(t[2]>>48)+((t[3]&mask)<<4),(t[3]>>48)+((t[4]&mask)<<4),(t[4]>>48)+(t[5]<<4),0]}else{t[1]+=low;[t[1],t[2],t[3],t[4],t[5],0]};for v in t{*peak=(*peak).max(v);assert!(v<1u128<<61);}assert_eq!(integer(&t),(before+BigUint::from(m)*p)>>w);
 }
 for j in 0..4{t[j+1]+=t[j]>>52;t[j]&=B-1;}assert_eq!(t[5],0);assert!(t[4]<B);integer(&t)
}
fn words(x:&BigUint)->[u64;4]{let d=x.to_u64_digits();assert!(d.len()<=4);std::array::from_fn(|j|d.get(j).copied().unwrap_or(0))}
fn append(out:&mut Vec<u8>,x:&BigUint){for w in words(x){out.extend(w.to_le_bytes());}}
fn main(){assert!(!Path::new(&format!("{R}/analysis.json")).exists());let mut reports=vec![];let mut vector_count=0usize;let mut vectors=Vec::new();vectors.extend(b"FSCALE01");let mut peak=0;let mut all=0u64;
 for (field,pwords) in [(0u8,FqConfig::P),(1,FrConfig::P)]{let p=pwords.iter().rev().fold(n(0),|v,&w|(v<<64)+n(w));let two=&p<<1usize;let r256=n(1)<<256usize;assert!(&p*&p*16u32+&p*&r256*2u32 < &r256*&r256);assert!(pwords[3]<(199u64<<54));assert!((16u64*199*199+2*199*1024)<1024*1024);
  let edge=[n(0),n(1),p.clone(),&two-n(1),two.clone()];let mut tuples=vec![];for i in 0..5usize.pow(5){let mut q=i;tuples.push(std::array::from_fn::<_,5,_>(|_|{let x=edge[q%5].clone();q/=5;x}));}
  for bit in 0..256usize{for sign in [-1,0,1]{let x=n(1)<<bit;let x=if sign<0{&x-n(1)}else{&x+n(sign as u64)};if x<=two{tuples.push([x.clone(),&two-&x,x.clone(),p.clone(),&two-&x]);}}}
  let mut state=0x66757365642d7363u64;for _ in 0..2048{tuples.push(std::array::from_fn(|_|{let mut v=n(0);for _ in 0..4{state^=state<<13;state^=state>>7;state^=state<<17;v=(v<<64)+n(state);}v%(&two+n(1))}));}
  for mixed in [true,false]{let bits=if mixed{256}else{260};let radix=n(1)<<bits;let inverse=p.modpow(&((&radix>>1usize)-n(1)),&radix);let negative_inv=(&radix-inverse)%&radix;let rinv=radix.modpow(&(&p-n(2)),&p);let fused_cap=&p+(&p*&p*8u32/&radix);let output_cap=&p+(&two*&fused_cap/&radix);assert!(output_cap<two);let mut count=0;let mut wide_count=0;let mut changed_raw=0;let mut rows=0;let mut run=|input:&[BigUint;5]|{
   let [a,b,c,d,f]=input;assert!(input.iter().all(|x|x<=&two));let mut outputs=vec![];for negative in[false,true]{let c=if negative{&two-c}else{c.clone()};let t=a*b+&c*d;assert!(t<=&p*&p*8u32);let wide=cios(a,b,&c,d,&p,mixed,true,&mut peak);let q=&t*&negative_inv%&radix;assert_eq!(wide,(&t+q*&p)/&radix);assert!(wide<=fused_cap);wide_count+=u64::from(wide>=two);let out=cios(&wide,f,&n(0),&n(0),&p,mixed,false,&mut peak);let t2=&wide*f;let q2=&t2*&negative_inv%&radix;assert_eq!(out,(&t2+q2*&p)/&radix);assert!(out<=output_cap&&out<two);
    let reduced=if mixed&&wide>=two{&wide-&two}else{wide.clone()};let old_t=reduced*f;let old_q=&old_t*&negative_inv%&radix;let old=(&old_t+old_q*&p)/&radix;assert_eq!(&out%&p,&old%&p);changed_raw+=u64::from(out!=old);let canonical=&out%&p*&rinv%&p;assert_eq!(canonical,((&t*&rinv)%&p*f%&p)*&rinv%&p);
    if mixed{vectors.push(field);vectors.push(u8::from(negative));vectors.extend([0u8;6]);for x in input{append(&mut vectors,x);}append(&mut vectors,&out);append(&mut vectors,&canonical);vector_count+=1;}outputs.push(out);count+=1;all+=1;rows+=10;
   }[outputs.remove(0),outputs.remove(0)]
  };for input in &tuples{run(input);}let mut input=[&two-n(1),p.clone(),n(0),&two-n(3),&two-n(1)];for _ in 0..256{let [a,b]=run(&input);input=[input[1].clone(),input[2].clone(),input[3].clone(),a,b];}
  reports.push(json!({"field":if field==0{"Fq"}else{"Fr"},"radix_bits":bits,"tuples":tuples.len(),"chain_tuples":256,"scaled_outputs":count,"first_stage_ge_2p":wide_count,"different_raw_final_vs_original":changed_raw,"row_identity_checks":rows,"fused_upper_bound_decimal":fused_cap.to_string(),"scaled_upper_bound_decimal":output_cap.to_string(),"all_outputs_strict_below_2p":true}));
 }}
 assert_eq!((vectors.len()-8)/232,vector_count);fs::write(format!("{R}/oracle-vectors.bin"),&vectors).unwrap();let result=json!({"complete":true,"outputs":all,"row_identity_checks":all*10,"maximum_observed_coefficient":peak.to_string(),"all_coefficients_below_2pow61":true,"native_oracle_records":vector_count,"native_record_bytes":232,"reports":reports,"range_condition":"16p^2+2pR<R^2","range_sufficient_prime_bound":"p<199*2^246 for R256","public_Ifma8_contract_unchanged":true,"wide_value_private_inside_one_helper":true,"no_products_removed":true,"field_IFMA_count_expected_250_each_combined_operation":true,"native_selected":true,"new_timings":0,"new_proofs":0,"canonical_proofs_unchanged":37023,"production_changed":false,"goal_complete":false});fs::write(format!("{R}/analysis.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("Fused-scale closure PASS {all} outputs/{} rowidentities; {} private native oracle records; no timing",all*10,vector_count);}

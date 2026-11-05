use num_bigint::{BigInt,BigUint,Sign};
use num_traits::{One,Zero,ToPrimitive};
use serde_json::json;
use std::fs;
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/signed-complex-square176";
const B:i128=1i128<<52;
fn limbs(x:&BigUint)->[i128;5]{let mask=BigUint::from((B-1)as u64);std::array::from_fn(|j|((x>>(52*j))& &mask).to_u64().unwrap()as i128)}
fn value(t:&[i128])->BigInt{t.iter().enumerate().map(|(j,x)|BigInt::from(*x)<<(52*j)).sum()}
fn mod_positive(x:&BigInt,m:&BigInt)->BigInt{((x%m)+m)%m}
fn reducer(a:&BigUint,b:&BigUint,p:&BigUint,mixed:bool,peak:&mut i128)->(BigUint,BigInt){
 let aa=limbs(a);let bb=limbs(b);let pp=limbs(p);let inv={let modulus=BigUint::one()<<52usize;let exponent=(&modulus>>1usize)-BigUint::one();let inverse=p.modpow(&exponent,&modulus);((&modulus-inverse)%&modulus).to_u64().unwrap()as i128};let mut t=[0i128;6];let mut removed=0usize;
 for i in 0..5{
  for j in i..5 {let factor=if i==j{1}else{2};let pa=aa[i]*aa[j];let pb=bb[i]*bb[j];t[j]+=factor*((pa&(B-1))-(pb&(B-1)));t[j+1]+=factor*((pa>>52)-(pb>>52));}
  for x in t{*peak=(*peak).max(x.abs());assert!(x.abs()<1i128<<61);}
  let w=if mixed&&i==4{48}else{52};let base=1i128<<w;let m=(t[0]*inv)&(base-1);
  // Independently compare the row's signed ceil carry with full cancellation.
  let low=(t[0]+base-1)>>w;let cancel_hi=if w==48{((m<<4)*pp[0])>>52}else{(m*pp[0])>>52};assert_eq!(low+cancel_hi,(t[0]+m*pp[0])>>w);assert_eq!((t[0]+m*pp[0])&(base-1),0);
  let before=value(&t);let expected=(before+BigInt::from(m)*BigInt::from(p.clone()))>>w;
  for j in 1..5 {let product=m*pp[j];t[j]+=product&(B-1);t[j+1]+=product>>52;}
  if w==48{let mask=(1i128<<48)-1;t=[low+cancel_hi+((t[1]&mask)<<4),(t[1]>>48)+((t[2]&mask)<<4),(t[2]>>48)+((t[3]&mask)<<4),(t[3]>>48)+((t[4]&mask)<<4),(t[4]>>48)+(t[5]<<4),0];}
  else{t[1]+=low+cancel_hi;t=[t[1],t[2],t[3],t[4],t[5],0];}
  removed+=w;assert_eq!(value(&t),expected,"row value i={i}");for x in t{*peak=(*peak).max(x.abs());assert!(x.abs()<1i128<<61);}
 }
 assert_eq!(removed,if mixed{256}else{260});for j in 0..4{let carry=t[j]>>52;t[j]&=B-1;t[j+1]+=carry;}let signed=value(&t);let modulus=BigInt::from(p.clone());assert!(signed> -&modulus&&signed<(&modulus<<1usize));let negative=t[4]<0;
 if negative{for j in 0..5{t[j]+=pp[j];}for j in 0..4{let carry=t[j]>>52;t[j]&=B-1;t[j+1]+=carry;}}
 for x in &t[..5]{assert!(*x>=0&&*x<B);}assert_eq!(t[5],0);let out=value(&t).to_biguint().unwrap();assert!(out<(p<<1usize));assert_eq!(out,if negative{(&signed+modulus).to_biguint().unwrap()}else{signed.to_biguint().unwrap()});(out,signed)
}
#[derive(Clone,Copy)]struct Range{lo:i128,hi:i128}
impl Range{fn add(self,b:Self)->Self{Self{lo:self.lo+b.lo,hi:self.hi+b.hi}}fn shifted(self,n:usize)->Self{Self{lo:self.lo>>n,hi:self.hi>>n}}fn scale(self,n:i128)->Self{assert!(n>=0);Self{lo:self.lo*n,hi:self.hi*n}}}
fn interval(mixed:bool)->i128{
 let mut t=[Range{lo:0,hi:0};6];let mut peak=0;
 for i in 0..5{for j in i..5{let k=if i==j{1}else{2};t[j]=t[j].add(Range{lo:-(B-1),hi:B-1}.scale(k));t[j+1]=t[j+1].add(Range{lo:-(B-2),hi:B-2}.scale(k));}for x in t{peak=peak.max(x.lo.abs()).max(x.hi.abs());}
 let w=if mixed&&i==4{48}else{52};let base=1i128<<w;let carry=t[0].add(Range{lo:base-1,hi:base-1}).shifted(w);let low=carry.add(Range{lo:0,hi:B-2});
 for j in 1..5{t[j]=t[j].add(Range{lo:0,hi:B-1});t[j+1]=t[j+1].add(Range{lo:0,hi:B-2});}
 if w==48{let mask=(1i128<<48)-1;t=[low.add(Range{lo:0,hi:mask<<4}),t[1].shifted(48).add(Range{lo:0,hi:mask<<4}),t[2].shifted(48).add(Range{lo:0,hi:mask<<4}),t[3].shifted(48).add(Range{lo:0,hi:mask<<4}),t[4].shifted(48).add(t[5].scale(16)),Range{lo:0,hi:0}];}
 else{t[1]=t[1].add(low);t=[t[1],t[2],t[3],t[4],t[5],Range{lo:0,hi:0}];}for x in t{peak=peak.max(x.lo.abs()).max(x.hi.abs());}}
 assert!(peak<1i128<<61);peak
}
fn main(){
 assert!(!std::path::Path::new(&format!("{ROOT}/model.json")).exists());let primes=[("Fq","21888242871839275222246405745257275088696311157297823662689037894645226208583"),("Fr","21888242871839275222246405745257275088548364400416034343698204186575808495617")];let mut reports=Vec::new();let mut total=0;let mut rows=0;let mut observed_peak=0i128;
 for (field,s) in primes{let p=BigUint::parse_bytes(s.as_bytes(),10).unwrap();let two=&p<<1usize;let four=&p<<2usize;let mut pairs=Vec::new();let edges=vec![BigUint::zero(),BigUint::one(),BigUint::from(2u32),&p-1u32,p.clone(),&p+1u32,&two-1u32,two.clone()];for a in&edges{for b in&edges{pairs.push((a.clone(),b.clone()));}}
 for bit in 0..255{let x=BigUint::one()<<bit;for d in[-1i32,0,1]{let x=if d<0{&x-1u32}else{&x+d as u32};if x<=two{pairs.push((x.clone(),&two-&x));pairs.push((x.clone(),x.clone()));}}}
 let mut seed=0x243f6a8885a308d3u64;let mut next=||{let words=(0..4).map(|_|{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed}).collect::<Vec<_>>();BigUint::new(words.into_iter().flat_map(|w|[w as u32,(w>>32)as u32]).collect())%(&two+1u32)};for _ in 0..4096{pairs.push((next(),next()));}
 for mixed in[true,false]{let bits=if mixed{256}else{260};let radix=BigUint::one()<<bits;assert!(four<radix);let pinv=p.modpow(&((&radix>>1usize)-1u32),&radix);let p_signed=BigInt::from(p.clone());let r_signed=BigInt::from(radix.clone());let inv_signed=BigInt::from(pinv);let mut negative=0;let mut zeros=0;let mut check=|a:&BigUint,b:&BigUint|{let(out,signed)=reducer(a,b,&p,mixed,&mut observed_peak);let t=BigInt::from(a*a)-BigInt::from(b*b);let k=mod_positive(&(-&t*&inv_signed),&r_signed);let direct=(&t+k*&p_signed)/&r_signed;assert_eq!(signed,direct);assert_eq!(mod_positive(&(&signed*&r_signed),&p_signed),mod_positive(&t,&p_signed));if signed.sign()==Sign::Minus{negative+=1;}if out.is_zero(){zeros+=1;}total+=1;rows+=5;out};for(a,b)in&pairs{check(a,b);}let mut a=&two-17u32;let mut b=&p+31u32;for _ in 0..1024{let out=check(&a,&b);a=b;b=out;}
 reports.push(json!({"field":field,"radix_bits":bits,"independent_pairs":pairs.len(),"additional_dependent_chain_outputs":1024,"negative_pre_normalization":negative,"zero_outputs":zeros,"conservative_signed_coefficient_absolute_bound":interval(mixed).to_string()}));}
 }
 let out=json!({"complete":true,"signed_row_model_passed":true,"closed_REDC_oracle_passed":true,"outputs":total,"row_identity_comparisons":rows,"maximum_observed_coefficient_absolute":observed_peak.to_string(),"reports":reports,"production_changed":false,"native_kernel_implemented":false,"timings_collected":false,"new_proofs":0,"proof_ledger":29953,"goal_achieved":false});fs::write(format!("{ROOT}/model.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("Signed square difference model PASS: {total} outputs/{rows} exact row comparisons; signed coefficient bounds checked; no native timing");
}

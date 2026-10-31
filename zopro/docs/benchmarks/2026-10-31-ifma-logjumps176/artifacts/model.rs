use num_bigint::BigUint as N;
use serde_json::json;
use std::fs;
fn p(s:&str)->N{N::parse_bytes(s.as_bytes(),10).unwrap()}
fn rho(bits:usize,prime:&N)->N{(N::from(1u32)<<bits).modpow(&(prime-N::from(2u32)),prime)}
fn ratio(x:&N,y:&N)->f64{((x*N::from(1_000_000_000u64)/y).to_u64_digits().first().copied().unwrap_or(0))as f64/1e9}
fn jump(t:&N,widths:&[usize],prime:&N)->N{let total:usize=widths.iter().sum();let mut out=t>>total;let mut at=0;for &w in widths{let mask=(N::from(1u32)<<w)-N::from(1u32);let digit=(t>>at)&mask;out+=digit*rho(total-at,prime);at+=w;}out}
fn model(c:&N,prime:&N)->N{let first=jump(c,&[52,52],prime);let second=jump(&first,&[52,48],prime);let base:N=N::from(1u32)<<52usize;let mask=&base-N::from(1u32);let exponent:N=(N::from(1u32)<<51usize)-N::from(1u32);let mu=(&base-prime%&base).modpow(&exponent,&base)& &mask;let m=(&second*&mu)& &mask;let z=second+m*prime;assert_eq!(&z&mask,N::from(0u32));z>>52usize}
fn main(){
 let mut rows=serde_json::Map::new();
 for(name,decimal)in[("Fq","21888242871839275222246405745257275088696311157297823662689037894645226208583"),("Fr","21888242871839275222246405745257275088548364400416034343698204186575808495617")]{
  let prime=p(decimal);let b:N=N::from(1u32)<<52usize;let b48:N=N::from(1u32)<<48usize;let r:N=N::from(1u32)<<256usize;let rinv=r.modpow(&(&prime-N::from(2u32)),&prime);let two=&prime*N::from(2u32);
  let constants=[48,52,100,104].into_iter().map(|k|(k.to_string(),json!({"decimal":rho(k,&prime).to_string(),"fraction_p":ratio(&rho(k,&prime),&prime)}))).collect::<serde_json::Map<_,_>>();
  let mut bounds=vec![];
  for products in[1u32,2]{let cmax=&two*&two*N::from(products);let first=(&cmax>>104usize)+(&b-N::from(1u32))*(rho(104,&prime)+rho(52,&prime));let second=(&first>>100usize)+(&b-N::from(1u32))*rho(100,&prime)+(&b48-N::from(1u32))*rho(48,&prime);let bound=(&second+(&b-N::from(1u32))*&prime)>>52usize;assert!(bound<&prime*N::from(4u32));bounds.push(json!({"products":products,"raw_output_upper_bound":bound.to_string(),"bound_fraction_p":ratio(&bound,&prime),"one_subtract_2p_sufficient":true,"plain_below_2p":bound<two}));}
  let corners=vec![N::from(0u32),N::from(1u32),&prime-N::from(1u32),prime.clone(),&prime+N::from(1u32),&two-N::from(1u32),two.clone()];let mut checks=0;let mut above=0;let mut seed=0x6c6f676a756d7073u64;
  for i in 0usize..4096{let mut input=vec![];for j in 0..4{if i<2401{input.push(corners[(i/7usize.pow(j))%7].clone());}else{let mut v=N::from(0u32);for k in 0..4{seed^=seed<<13usize;seed^=seed>>7usize;seed^=seed<<17usize;v+=N::from(seed)<<(64*k);}input.push(v% &two);}}
   for products in[1,2]{let mut c=&input[0]*&input[1];if products==2{c+=&input[2]*&input[3];}let raw=model(&c,&prime);assert!(raw<&prime*N::from(4u32));let out=if raw>=two{above+=1;raw-&two}else{raw};assert!(out<two);assert_eq!(&out%&prime,(&c*&rinv)%&prime);checks+=1;}
  }
  rows.insert(name.into(),json!({"constants":constants,"bounds":bounds,"integer_oracle_cases":checks,"cases_requiring_2p_correction":above}));
 }
 let out=json!({"complete":true,"method":"parallel LogJumps 104+100+52, R=2^256","fields":rows,"new_proofs":0,"new_timing_samples":0});fs::write("model.json",serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("{out}");
}

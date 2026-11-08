use num_bigint::BigUint;
use serde_json::{json,Value};
use std::{fs,path::Path};
use zopro_bn254::{FpConfig,FqConfig,FrConfig};
const R:&str="/home/ubuntu/src/zopro-perf-work/pmns-truncation-width176";
fn nat(v:u64)->BigUint{BigUint::from(v)}
fn counts(n:usize,s:usize,fused:bool)->[u64;4]{
 let mut out=[0u64;4];for _ in 0..n{for _ in 0..n{for a in 0..s{for b in 0..s{
  out[0]+=if fused{4}else{2};
  out[1]+=u64::from(a+b<s)+u64::from(a+b+1<s);
  out[2]+=2;
  out[3]+=u64::from(a+b>=s-1)+u64::from(a+b+1>=s-1);
 }}}}out
}
fn whole(words:&[u64],w:usize)->BigUint{words.iter().rev().fold(nat(0),|v,&d|(v<<w)+nat(d))}
fn trace_product(a:&[u64],b:&[u64],w:usize,keep:usize)->(BigUint,BigUint){
 let base=nat(1)<<w;let mask=&base-nat(1);let mut full=nat(0);let mut kept=nat(0);
 for(i,&a)in a.iter().enumerate(){for(j,&b)in b.iter().enumerate(){let product=nat(a)*nat(b);for(k,p)in[(i+j,&product & &mask),(i+j+1,&product>>w)]{full+=&p<<(k*w);if k>=keep{kept+=p<<(k*w);}}}}
 assert_eq!(full,whole(a,w)*whole(b,w));(full,kept)
}
fn state_next(state:&mut u64)->u64{*state^=*state<<13;*state^=*state>>7;*state^=*state<<17;*state}
fn check_truncation()->Value{
 let mut checked=0u64;let mut boundary=0u64;let mut dropped_nonzero=0u64;let mut state=0x706d6e732d776964u64;
 for w in [12usize,52]{for s in 1..=5{for terms in [1usize,3,6]{for sample in 0..160{
  let mask=(1u64<<w)-1;let mut full=nat(0);let mut kept=nat(0);
  for term in 0..terms{let mut operand=|side:usize|{(0..s).map(|i|match sample{0=>0,1=>mask,2=>u64::from((i+side+term)%2==0)*mask,3=>u64::from(i==s-1),_=>state_next(&mut state)&mask}).collect::<Vec<_>>()};let a=operand(0);let b=operand(1);let(f,k)=trace_product(&a,&b,w,s-1);let weight=nat(if term%2==0{1}else{2});full+=f*&weight;kept+=k*weight;}
  assert!(kept<=full);let phi=nat(1)<<(s*w);let low=&full%&phi;let c=if low==nat(0){nat(0)}else{&phi-low}+(&phi*nat(sample as u64));assert_eq!((&c+&full)%&phi,nat(0));let dropped=&full-&kept;assert!(dropped<phi);let partial=&c+&kept;let result=(&partial/&phi)+nat(u64::from((&partial%&phi)!=nat(0)));let expected=(&c+&full)/&phi;assert_eq!(result,expected);dropped_nonzero+=u64::from(dropped!=nat(0));checked+=1;boundary+=u64::from(sample<4);
 }}}}
 // Demonstrate why the dropped-part bound is required. These small-base
 // coefficients intentionally exceed the admissible accumulator envelope.
 let s=4;let w=3;let a=vec![(1<<w)-1;s];let(f,k)=trace_product(&a,&a,w,s-1);let full=f*nat(64);let kept=k*nat(64);let phi=nat(1)<<(s*w);assert!(&full-&kept>=phi);let low=&full%&phi;let c=if low==nat(0){nat(0)}else{&phi-low};let partial=&c+&kept;assert_ne!(&partial/&phi+nat(u64::from(&partial%&phi!=nat(0))),(&c+&full)/&phi);
 json!({"exact_product_and_reconstruction_cases":checked,"boundary_cases_included":boundary,"nonzero_discarded_part_cases":dropped_nonzero,"guarded_small_base_counterexample":true,"not_a_PMNS_parameter_or_BN254_kernel_qualification":true})
}
fn main(){assert!(!Path::new(&format!("{R}/analysis.json")).exists());
 let primes=[("Fq",FqConfig::P),("Fr",FrConfig::P)].map(|(name,p)|{let p=p.iter().rev().fold(nat(0),|v,&d|(v<<64)+nat(d));assert_eq!(p.bits(),254);assert!(p>nat(1)<<208usize);assert!(p<nat(1)<<260usize);json!({"field":name,"bits":p.bits(),"four_52bit_words_insufficient":true,"minimum_52bit_capacity_words":5})});
 let mut rows=vec![];let mut schedule_checks=0;for n in 1..=16usize{for s in 1..=16usize{for fused in [false,true]{let c=counts(n,s,fused);let full=c[0]+c[1]+c[2];let truncated=c[0]+c[1]+c[3];let k=if fused{7}else{5};assert_eq!(full,(k*n*n*s*s)as u64);assert_eq!(full-truncated,(n*n*(s-1)*(s-1))as u64);assert_eq!(truncated,((k-1)*n*n*s*s+n*n*(2*s-1))as u64);schedule_checks+=1;
  if n*s>=5{rows.push(json!({"n":n,"s":s,"fused":fused,"capacity_bits":52*n*s,"raw_product_IFMAs":c[0],"quotient_low_IFMAs":c[1],"full_QM_IFMAs":c[2],"truncated_QM_IFMAs":c[3],"full_IFMAs_per_eight_independent_fields":full,"truncated_IFMAs_per_eight_independent_fields":truncated,"saved_IFMAs":full-truncated,"parameter_and_closure_validity_not_established":true}));}
 }}}
 let best=|fused:bool|{let r=rows.iter().filter(|v|v["fused"]==fused).min_by_key(|v|v["truncated_IFMAs_per_eight_independent_fields"].as_u64().unwrap()).unwrap();r.clone()};let ordinary=best(false);let fused=best(true);assert_eq!(ordinary["truncated_IFMAs_per_eight_independent_fields"],109);assert_eq!(fused["truncated_IFMAs_per_eight_independent_fields"],159);
 // Global extension beyond the finite table: t=n*s. At t>=6, the leading
 // term alone is >=144 ordinary or >=216 fused, above 109/159. At t=5,
 // only (1,5) and (5,1) occur. The n=1 minimum is degenerate integer REDC.
 for (n,s) in [(1,5),(5,1)]{for f in[false,true]{let c=counts(n,s,f);assert!(c[0]+c[1]+c[3]>=if f{159}else{109});}}
 let reconstruction=check_truncation();let result=json!({"complete":true,"primary_source":"https://arxiv.org/html/2610.09767v1#S5","prior_paper_mentions_not_new_discovery":true,"layout":"eight independent fields; n polynomial coefficients each s 52bit words; not coefficient-parallel lane packing","primes":primes,"exact_schedule_checks":schedule_checks,"capacity_admissible_table_rows":rows.len(),"rows":rows,"best_capacity_only_ordinary":ordinary,"best_capacity_only_fused":fused,"ordinary_CIOS_reference_source_count":100,"fused_CIOS_reference_source_count":150,"best_optimistic_G2_7ordinary_5fused_IFMAs":7*109+5*159,"original_G2_group_IFMAs":1450,"six_coefficient_one_word_full_IFMAs":180,"six_coefficient_one_word_truncated_IFMAs":180,"global_minimum_beyond_table_proved_by_t_ge6":true,"reconstruction_checks":reconstruction,"selection":"close this schoolbook PMNS truncation route before native implementation; no IFMA-volume reduction case","not_general_PMNS_impossibility":true,"native_selected":false,"new_timings":0,"new_profiles":0,"new_proofs":0,"canonical_proofs_unchanged":37023,"production_changed":false,"adopted":false,"goal_complete":false});fs::write(format!("{R}/analysis.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("PMNS width screen PASS; 512 exact schedules; optimistic minima109/159 vsCIOS100/150; 4800 carry reconstructions; no native candidate");}

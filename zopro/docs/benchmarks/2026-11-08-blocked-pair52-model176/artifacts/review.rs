//! Offline evidence/identity reader; does not rerun arithmetic populations.
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/blocked-pair52-model176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){assert!(!Path::new(&format!("{R}/review.json")).exists());let pins=read(format!("{R}/pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
let plan=read(format!("{R}/plan.json"));assert_eq!(read(format!("{R}/run/result.json"))["status"],"complete");let mut receipts=0;for step in plan["steps"].as_array().unwrap(){let d=format!("{R}/run/{}",step["name"].as_str().unwrap());let command=read(format!("{d}/command.json"));let exit=read(format!("{d}/exit.json"));assert_eq!(exit["success"],true);assert_eq!(exit["returncode"],0);assert_eq!(&command["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());for stream in["stdout","stderr"]{assert_eq!(exit[format!("{stream}_sha256")],hash(format!("{d}/{stream}.log")));}receipts+=1;}
let result=read(format!("{R}/result.json"));assert_eq!(result["status"],"pass");let expected=(1..=4).map(|b|(1u64<<b).pow(4)).sum::<u64>();assert_eq!(expected,69904);assert_eq!(result["exhaustive_small_radix_tiles"],expected);
// Formal monomials ai*bj. Expand the two diagonal products and the
// difference product at every global coefficient, then compare with the
// 25-term schoolbook polynomial. This is not a numerical oracle replay.
let mut coefficients:BTreeMap<(usize,usize,usize),i32>=BTreeMap::new();
let mut add=|degree:usize,a:usize,b:usize,c:i32|{*coefficients.entry((degree,a,b)).or_default()+=c;};
for i in[0usize,2]{let j=i+1;add(2*i,i,i,1);add(2*j,j,j,1);
// Diagonal sum plus (ai-aj)*(bj-bi) cancels its two own diagonal monomials.
for(a,b,c)in[(i,i,1),(j,j,1),(i,j,1),(i,i,-1),(j,j,-1),(j,i,1)]{add(i+j,a,b,c);}
for b in i..i+2{for a in 0..5{if a!=i&&a!=j{add(a+b,a,b,1);}}}}
for a in 0..5{add(a+4,a,4,1);}coefficients.retain(|_,n|*n!=0);assert_eq!(coefficients.len(),25);for((degree,a,b),n)in &coefficients{assert_eq!(*degree,a+b);assert_eq!(*n,1);}
let raw=2*(25-2);let reduction=5*(1+1+8);assert_eq!(raw,46);assert_eq!(reduction,50);assert_eq!(result["ordinary_IFMA_proxy"],raw+reduction);assert_eq!(result["fused_IFMA_proxy"],2*raw+reduction);assert_eq!(result["array_coefficients"],7);
let mut count=0u64;let mut rows=0u64;let mut tiles=0u64;for p in result["panels"].as_array().unwrap(){assert!(["Fq","Fr"].contains(&p["field"].as_str().unwrap()));assert!([256,260].contains(&p["radix_bits"].as_u64().unwrap()));let edges=p["boundary_values"].as_u64().unwrap();assert_eq!(edges,47);assert_eq!(p["boundary_tuples"],edges*edges);let n=edges*edges+4096+1024;for op in p["operations"].as_array().unwrap(){assert_eq!(*op,n);}assert_eq!(p["products"],3*n);assert_eq!(p["rows"],15*n);assert_eq!(p["raw_IFMA_proxy"],5*n*raw);assert_eq!(p["reduction_IFMA_proxy"],3*n*reduction);assert_eq!(p["tiles"],10*n);assert!(p["negative_pair_products"].as_u64().unwrap()>0);assert!(p["inclusive_two_p_input_occurrences"].as_u64().unwrap()>0);assert_eq!(p["output_digest"].as_str().unwrap().len(),64);count+=3*n;rows+=15*n;tiles+=10*n;}
assert_eq!(count,87948);assert_eq!(rows,439740);assert_eq!(tiles,293160);
// A separate coarse envelope: all remaining schoolbook halves at one
// coefficient contribute <10B/product; two tiles contribute <8B/product.
// Fused input contributions <36B. Each of five cancellations contributes
// <3B/coefficient (ceil is <64 while the invariant <64B holds), hence <51B.
// Mixed packing multiplies by16; its extra cancelled-low carry is <2B.
// Four normalizations add only bounded integer carries, giving <820B.
let b=1i128<<52;assert!(36+5*3<64);assert!(51*16+2+1<820);assert!(820*b<i64::MAX as i128);
for panel in result["intervals"].as_array().unwrap(){assert!(panel["prepacking_abs_bound_inclusive"].as_str().unwrap().parse::<i128>().unwrap()<64*b);assert!(panel["all_intermediate_abs_bound_inclusive"].as_str().unwrap().parse::<i128>().unwrap()<820*b);assert_eq!(panel["signed64"],true);assert_eq!(panel["rows"].as_array().unwrap().len(),4);}
for flag in["native_executed","production_changed","goal_complete"]{assert_eq!(result[flag],false);}assert_eq!(result["timings"],0);assert_eq!(result["proofs"],0);
let report=json!({"review_passed":true,"source_pins":pins.as_object().unwrap().len(),"receipts":receipts,"products":count,"cancellation_rows":rows,"local_tiles":tiles,"symbolic_global_schoolbook_terms":coefficients.len(),"independent_conservative_prepacking_bound":"strict51B","independent_conservative_all_intermediates_bound":"strict820B","signed64_bound":true,"model_population_reexecuted":false,"performance_established":false,"native_required":true,"model_sha256":hash(format!("{R}/model.rs")),"result_sha256":hash(format!("{R}/result.json"))});fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&report).unwrap()).unwrap();println!("Offline review PASS: {count} products / {rows} rows / {tiles} tiles; formal25terms and conservative bounds; no replay");}

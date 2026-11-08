use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,path::Path};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/unreduced-gcd-coefficients176";
fn hash(p:&str)->String {format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:&str)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){
 assert!(!Path::new(&format!("{ROOT}/review.json")).exists());
 let pins=read(&format!("{ROOT}/model-pins.json"));
 for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let complete=read(&format!("{ROOT}/model-runs/result.json"));assert_eq!(complete["status"],"complete");
 let mut receipts=Vec::new();
 for name in ["build-unit-coefficient-model","exact-unit-coefficient-screen"]{
  let dir=format!("{ROOT}/model-runs/{name}");let result=read(&format!("{dir}/exit.json"));
  assert_eq!(result["success"],true);assert_eq!(result["returncode"],0);
  for stream in["stdout","stderr"] {assert_eq!(hash(&format!("{dir}/{stream}.log")),result[format!("{stream}_sha256")].as_str().unwrap());}
  receipts.push(result);
 }
 let model=read(&format!("{ROOT}/model.json"));assert_eq!(model["mathematical_checks_passed"],true);assert_eq!(model["toy_dyadic_inverse_cases"],2048);
 let mut fields=Vec::new();
 for f in model["fields"].as_array().unwrap(){
  let rows=f["rows"].as_array().unwrap();assert_eq!(rows.len(),2810);assert_eq!(f["inputs"],2810);
  let mut unique=BTreeSet::new();let mut cohorts=BTreeMap::<String,Vec<&Value>>::new();
  for(i,row)in rows.iter().enumerate(){
   assert_eq!(row["index"],i);unique.insert(row["input_hex"].as_str().unwrap());
   let c=&row["counts"];let n=|k:&str|c[k].as_u64().unwrap();
   assert_eq!(n("old_linear_word_products"),8*(n("old_modular_coefficient_rows")+n("old_ordinary_finish_rows")));
   assert_eq!(n("old_cancel_word_products"),5*(n("old_modular_coefficient_rows")+n("old_ordinary_finish_rows")));
   assert_eq!(n("new_cancel_word_products"),5*n("new_cancel_steps"));
   let shifts=c["new_cancel_shifts"].as_object().unwrap();
   let shift_sum=shifts.iter().map(|(s,count)|s.parse::<u64>().unwrap()*count.as_u64().unwrap()).sum::<u64>();
   assert_eq!(shift_sum,31*n("rounds"));assert_eq!(n("new_cancel_steps"),(31*n("rounds")).div_ceil(64));
   assert!(n("max_coefficient_bits")<=992);assert!(n("max_finish_bits")<=1056);assert!(n("max_cancel_intermediate_bits")<=1152);
   assert_eq!(n("new_scale_field_multiplications"),1);
   if c["finish_kind"]=="ordinary_Euclid" {assert_eq!(n("old_scale_field_multiplications"),1);assert_eq!(n("old_ordinary_finish_rows"),1);}
   else {assert_eq!(n("old_scale_field_multiplications"),0);assert_eq!(n("old_ordinary_finish_rows"),0);}
   cohorts.entry(row["cohort"].as_str().unwrap().into()).or_default().push(c);
  }
  assert_eq!(unique.len(),2808);let mut aggregates=BTreeMap::new();
  for(cohort,counts)in cohorts{
   let sum=|k:&str|counts.iter().map(|v|v[k].as_u64().unwrap()).sum::<u64>();
   let max=|k:&str|counts.iter().map(|v|v[k].as_u64().unwrap()).max().unwrap();
   let old=sum("old_linear_word_products")+sum("old_cancel_word_products");
   let new=sum("new_coefficient_linear_word_products")+sum("new_finish_linear_word_products")+sum("new_cancel_word_products");
   if cohort=="random" {assert_eq!(counts.len(),2048);}
   aggregates.insert(cohort,json!({"cases":counts.len(),"old_linear_word_products":sum("old_linear_word_products"),"old_cancel_word_products":sum("old_cancel_word_products"),"new_linear_word_products":sum("new_coefficient_linear_word_products"),"new_finish_word_products":sum("new_finish_linear_word_products"),"new_cancel_word_products":sum("new_cancel_word_products"),"old_coefficient_and_finish_word_products":old,"new_coefficient_and_finish_word_products":new,"new_over_old_word_product_ratio":new as f64/old as f64,"old_scale_field_products":sum("old_scale_field_multiplications"),"new_scale_field_products":sum("new_scale_field_multiplications"),"max_coefficient_bits":max("max_coefficient_bits"),"max_finish_bits":max("max_finish_bits"),"max_cancel_intermediate_bits":max("max_cancel_intermediate_bits"),"max_rounds":max("rounds")}));
  }
  fields.push(json!({"field":f["field"],"case_rows":rows.len(),"distinct_input_values":unique.len(),"cohorts":aggregates}));
 }
 let selected=fields.iter().all(|f|f["cohorts"]["random"]["new_over_old_word_product_ratio"].as_f64().unwrap()<1.0 && f["cohorts"]["random"]["old_scale_field_products"]==f["cohorts"]["random"]["new_scale_field_products"]);
 assert!(selected);
 let output=json!({"review_passed":true,"source_pins":pins.as_object().unwrap().len(),"successful_command_receipts":receipts.len(),"model_sha256":hash(&format!("{ROOT}/model.json")),"fields":fields,"toy_cases":2048,"selected_for_separate_fixed_limb_qualification":selected,"native_backend_qualified":false,"speedup_measured":false,"new_proofs":0,"new_SDK_requests":0,"canonical_unchanged":35181,"goal_complete":false,"limits":"Reader reconstructs counters, cohorts, widths, source/stream/receipt bindings. It does not replay inverse oracles or independently implement the GCD algebra; active-width word-product counts omit instruction/carry/normalization/length costs and are not timing."});
 fs::write(format!("{ROOT}/review.json"),serde_json::to_vec_pretty(&output).unwrap()).unwrap();
 println!("Reader PASS:185 pins/2 receipts/5620 case rows; no oracle rerun; fixed-limb qualification selected");
 for f in output["fields"].as_array().unwrap(){println!("{} random: {} -> {} coefficient/finish word products, ratio {}; max coefficient/finish {} / {} bits",f["field"],f["cohorts"]["random"]["old_coefficient_and_finish_word_products"],f["cohorts"]["random"]["new_coefficient_and_finish_word_products"],f["cohorts"]["random"]["new_over_old_word_product_ratio"],f["cohorts"]["random"]["max_coefficient_bits"],f["cohorts"]["random"]["max_finish_bits"]);}
}

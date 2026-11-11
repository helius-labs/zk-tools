use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/recursive-pair177";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn n(v:&Value,k:&str)->u64{v[k].as_u64().unwrap()}
fn main(){
 let result=read(&format!("{R}/result.json"));assert_eq!(result["passed"],true);
 let prior=read("/home/ubuntu/src/zopro-perf-work/global-window-pair-bound176/analysis.json");
 let mut checks=0;let mut rows=Vec::new();
 for c in result["cases"].as_array().unwrap(){
  let old=prior["cases"].as_array().unwrap().iter().find(|o|o["case"]==c["case"]).unwrap();
  assert_eq!(n(c,"classes")*22,n(c,"coefficient_checks"));checks+=n(c,"coefficient_checks");
  assert_eq!(n(c,"classes")+n(c,"exact_group_formations"),n(c,"nonzero_rows"));
  assert_eq!(n(c,"raw_arrivals"),n(old,"raw_nonzero_digit_arrivals"));assert_eq!(n(c,"initial_class_arrivals"),n(old,"after_exact_scalar_grouping_arrivals"));
  let(mut remaining,mut nodes,mut formations,mut net,mut recursive)=(n(c,"initial_class_arrivals"),n(c,"classes"),0,0,0);
  let generations=c["rounds"].as_array().unwrap();
  for (i,g) in generations.iter().enumerate(){
   let f=n(g,"formations");let removed=n(g,"removed_arrivals");assert_eq!(n(g,"generation"),i as u64+1);assert_eq!(n(g,"nodes_before"),nodes);assert_eq!(n(g,"net_saved"),removed-f);assert!(removed>=2*f);assert!(n(g,"recursive_formations")<=f);
   if i+1<generations.len(){assert!(f>0)}else{assert_eq!(f,0)}
   remaining-=removed;nodes+=f;formations+=f;net+=removed-f;recursive+=n(g,"recursive_formations");assert_eq!(n(g,"arrivals_after"),remaining);
  }
  assert_eq!(remaining,n(c,"final_arrivals"));assert_eq!(nodes,n(c,"all_nodes"));assert_eq!(formations,n(c,"recursive_formations"));assert_eq!(net,n(c,"net_saved_beyond_exact_groups"));
  assert_eq!(n(c,"hypothetical_new_G2_affine_bytes"),128*formations);assert!(2*formations<=n(c,"initial_class_arrivals"));assert!(n(c,"retained_nonempty_rows")<=nodes);
  assert_eq!(generations[0]["formations"],old["fixed_greedy"]["disjoint_two_point_formations"]);assert_eq!(generations[0]["net_saved"],old["fixed_greedy"]["net_arrival_minus_formation_savings"]);
  let combined=n(c,"raw_arrivals")-remaining-formations-n(c,"exact_group_formations");assert_eq!(combined,n(c,"combined_net_saved"));
  for (key,num,den) in [("fraction_beyond_exact_groups",net,n(c,"initial_class_arrivals")),("combined_fraction_of_raw",combined,n(c,"raw_arrivals"))]{let expected=num as f64/den as f64;assert!((c[key].as_f64().unwrap()-expected).abs()<=2.0*f64::EPSILON);}
  rows.push(json!({"case":c["case"],"formations_involving_a_generated_node":recursive,"additional_net_saved_after_first_round":net-n(&generations[0],"net_saved"),"productive_generations":generations.len()-1}));
 }
 let mut elapsed=Vec::new();let plan=read(&format!("{R}/plan.json"));
 for step in plan["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let dir=format!("{R}/run/{name}");let command=read(&format!("{dir}/command.json"));assert_eq!(&command["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());let exit=read(&format!("{dir}/exit.json"));assert_eq!(exit["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(&format!("{dir}/{stream}.log")),exit[format!("{stream}_sha256")]);}elapsed.push(json!({"step":name,"elapsed_ms":exit["elapsed_ms"]}));}
 let stdout=fs::read_to_string(format!("{R}/run/tests/stdout.log")).unwrap();assert!(stdout.contains("2 passed; 0 failed"));
 let printed=read(&format!("{R}/run/screen/stdout.log"));assert_eq!(printed,result);
 for (p,h) in read(&format!("{R}/pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let ledger="/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf-profile177/proof-sha256-after-profile.json";assert_eq!(read(ledger).as_array().unwrap().len(),53917);assert_eq!(hash(ledger),"a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873");
 let output=json!({"passed":true,"scope":"separate same-agent aggregate/source/receipt reader; no private-DAG replay or independent curve arithmetic","coefficient_comparisons_reported":checks,"synthetic_exhaustive_inputs":3usize.pow(9),"cases":rows,"commands":elapsed,"proof_ledger_unchanged":53917,"new_proofs":0});
 fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&output).unwrap()).unwrap();println!("{}",output);
}

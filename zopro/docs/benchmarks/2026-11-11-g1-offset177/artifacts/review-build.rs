use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g1-offset177";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 assert!(!Path::new(&format!("{R}/build-review.json")).exists());
 let mut receipts=vec![];
 for(plan,folder)in[("candidate-plan.json","native-candidate"),("control-plan.json","native-control"),("portable-plan.json","portable")]{
  assert_eq!(read(format!("{R}/{folder}/result.json"))["status"],"complete");
  for step in read(format!("{R}/{plan}"))["steps"].as_array().unwrap(){
   let name=step["name"].as_str().unwrap();let p=format!("{R}/{folder}/{name}");
   let exit=read(format!("{p}/exit.json"));assert_eq!(exit["success"],true);assert_eq!(exit["returncode"],0);
   for stream in["stdout","stderr"]{assert_eq!(hash(format!("{p}/{stream}.log")),exit[format!("{stream}_sha256")]);}
   let command=read(format!("{p}/command.json"));assert_eq!(&command["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
   for(k,v)in step["environment"].as_object().into_iter().flatten(){if !["PATH","RUSTC","RUSTUP_TOOLCHAIN"].contains(&k.as_str()){assert_eq!(command["environment"][k],*v);}}
   receipts.push(json!({"folder":folder,"step":name,"exit":exit}));
  }
 }
 let native=read(format!("{R}/native-candidate/candidate-core/tests.json"));
 assert_eq!(native["passed_tests"].as_array().unwrap().len(),130);
 for test in["both_curves_signs_tails_storage_and_reuse","owned_prefix_and_unsigned_g1_share_the_canonical_offset_contract","every_small_stack_worker_executes_complete_callers"]{assert!(native["passed_tests"].as_array().unwrap().iter().any(|n|n.as_str().unwrap().ends_with(test)));}
 let portable=read(format!("{R}/portable/portable-core/tests.json"));
 // This generic plan deliberately enables no optional native experiment cfgs.
 // Require its actual unfiltered suite, not a count from a different cfg set.
 let portable_count=portable["passed_tests"].as_array().unwrap().len();
 assert_eq!(portable_count,97);
 assert!(fs::read_to_string(format!("{R}/portable/portable-core/stdout.log")).unwrap().contains("test result: ok. 97 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
 let prover=fs::read_to_string(format!("{R}/native-candidate/candidate-prover/stdout.log")).unwrap();
 for n in[2,6]{assert!(prover.contains(&format!("test result: ok. {n} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out")));}
 let mut consumers=vec![];
 for label in["control","candidate"]{
  let frozen=read(format!("{R}/frozen-{label}.json"));assert_eq!(frozen["libraries"].as_object().unwrap().len(),8);
  assert_eq!(frozen["rustflags"].as_str().unwrap().contains("--cfg zopro_g1_affine_offset"),label=="candidate");
  for(p,h)in frozen["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
  let link=read(format!("{R}/link-{label}.json"));assert_eq!(link["success"],true);assert_eq!(link["exit_code"],0);
  let binary=format!("{R}/sdk-{label}-znver4");consumers.push(json!({"label":label,"path":binary,"sha256":hash(&binary),"frozen_metadata_sha256":hash(format!("{R}/frozen-{label}.json")),"link_receipt_sha256":hash(format!("{R}/link-{label}.json"))}));
 }
 for(p,h)in read(format!("{R}/build-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let prior=read(format!("{R}/source-review.json"));assert_eq!(prior["passed"],true);assert_eq!(hash(format!("{R}/native-helper")),prior["native_helper_sha256"]);
 let value=json!({"passed":true,"native_core_tests":130,"native_prover_unit_tests":2,"native_cubic_tests":6,"portable_core_tests":portable_count,"portable_scope":"generic x86-64, no_ifma/no_asm, new candidate cfg enabled but native selector excludes it; no optional native experiment cfgs","verified_workflow_commands":receipts.len(),"receipts":receipts,"consumers":consumers,"field_qualification_inherited":prior,"reader_repair":"Initial reader expected 102 from a different cfg set; actual unfiltered generic suite is 97. Initial source and binary retained. No test was rerun or omitted.","SDK_calls":0,"new_Gnark_exports":0,"performance":"untested","adoption":"pending full-proof qualification","goal_achieved":false});
 fs::write(format!("{R}/build-review.json"),serde_json::to_vec_pretty(&value).unwrap()).unwrap();
 println!("Reviewed native130+2+6, portable97, coherent OFF/ON SDKs; no SDK proof or latency measured.");
}

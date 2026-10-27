use std::{fs,path::Path};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let r=Path::new("/tmp/zopro-grouping-prefix-cutoff");
 let mut plan=read(r.join("native-plan.json"));
 for step in plan["steps"].as_array_mut().unwrap(){
  if step["kind"]=="link-sdk" {
   let s=step.as_object_mut().unwrap();
   assert!(s["argv"].is_null());assert!(s["environment"]["CARGO_TARGET_DIR"].is_null());
   s.remove("argv");s.remove("environment");
  }
 }
 let mut pins=read(r.join("source-pins.json"));
 for name in ["repair-plan.rs","repair-plan","native-plan.json","native.log"] {let p=r.join(name);pins[p.display().to_string()]=json!(hash(&p));}
 fs::write(r.join("source-pins-v2.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 plan["source_pins"]=json!(r.join("source-pins-v2.json"));
 fs::write(r.join("native-plan-v2.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
 fs::write(r.join("preparation-failures.json"),serde_json::to_vec_pretty(&json!({"stage":"workflow plan parse before any build/test/SDK","error":"unknown argv field on link-sdk; JSON mutable access inserted null keys","repair":"remove only null argv/environment from link steps; keep original plan/helper/log and all source bytes","new_plan":"native-plan-v2.json","successful_commands_repeated":0})).unwrap()).unwrap();
 println!("Removed only null link-step keys; original plan preserved; no build or SDK had executed");
}

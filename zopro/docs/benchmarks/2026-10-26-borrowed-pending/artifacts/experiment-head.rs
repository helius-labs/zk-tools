use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf}};
const R:&str="/tmp/zopro-borrowed-pending";
const TOOL:&str="/tmp/zopro-xtask-fast-screen-target/release/xtask";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn add(p:impl AsRef<Path>,pins:&mut BTreeMap<String,String>){let p=p.as_ref();pins.insert(p.display().to_string(),hash(p));}
fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);let n=v.len();assert!(n>0);if n%2==1{v[n/2]}else{(v[n/2-1]+v[n/2])/2.}}
fn gm(v:&[f64])->f64{(v.iter().map(|x|x.ln()).sum::<f64>()/v.len()as f64).exp()}
fn prior()->BTreeSet<String>{
 let mut set:BTreeSet<String>=serde_json::from_value(read("/tmp/zopro-g2-tree-sdk/context/manifest.json")["prior_proof_sha256"].clone()).unwrap();assert_eq!(set.len(),5958);
 for p in ["/tmp/zopro-g2-tree-sdk/proof-inventory.json","/tmp/zopro-g1-quarter-current/proof-inventory.json","/tmp/zopro-pairing-x-chain-sdk/proof-inventory.json","/tmp/zopro-control-rescheduling/all-proof-inventory.json","/tmp/zopro-pairing-clean/proof-inventory.json"]{
  for(file,h)in read(p).as_object().unwrap(){assert_eq!(hash(file),h.as_str().unwrap());if file.ends_with(".proof"){assert!(set.insert(h.as_str().unwrap().into()));}}
 }assert_eq!(set.len(),6850);set
}
fn prepare(){
 let r=Path::new(R);assert_eq!(read(r.join("native/result.json"))["status"],"complete");
 let field=read(r.join("native/field/tests.json"));assert_eq!(field["passed_tests"].as_array().unwrap().len(),33);
 let core=read(r.join("native/core/tests.json"));assert_eq!(core["passed_tests"].as_array().unwrap().len(),112);
 let mut c=read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json");
 let mut pins:BTreeMap<String,String>=serde_json::from_value(c["pins"].clone()).unwrap();
 pins.extend(serde_json::from_value::<BTreeMap<String,String>>(read(r.join("source-pins.json"))).unwrap());
 let a=read(r.join("native/on/artifacts.json"));add(r.join("native/on/artifacts.json"),&mut pins);
 for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}
 assert_eq!(hash(a["binary"].as_str().unwrap()),a["binary_sha256"].as_str().unwrap());add(a["binary"].as_str().unwrap(),&mut pins);
 for p in [r.join("experiment"),r.join("native-plan.json"),r.join("native/result.json"),r.join("implementation-audit.json"),PathBuf::from(TOOL),PathBuf::from("/tmp/zopro-pairing-clean/proof-inventory.json")]{add(p,&mut pins);}
 for(p,h)in &pins{assert_eq!(&hash(p),h,"pin {p}");}
 c["variants"]=json!([
  {"name":"accepted","binary":"/tmp/zopro-grouping-module/native/candidate-sdk","environment":{"ZOPRO_CPU_BORROWED_PENDING":"0"}},
  {"name":"off","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"0"}},
  {"name":"on","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"1"}}]);
 c["pins"]=json!(pins);c["prior_proof_sha256"]=json!(prior());save(r.join("perf-before-preflight.json"),&c);
 let mut steps=Vec::new();let mut jobs=Vec::new();
 for case in c["cases"].as_array().unwrap().iter().filter(|x|x["workers"]==32){for variant in ["off","on"]{
  let name=format!("{}-w4-{variant}",case["shape"].as_str().unwrap());
  let stem=Path::new(case["image"].as_str().unwrap()).file_stem().unwrap();let d=r.join("preflight-proofs").join(&name).join(stem);
  let mut env=c["environment"].clone();env["RAYON_NUM_THREADS"]=json!("4");env["ZOPRO_CPU_BORROWED_PENDING"]=json!(if variant=="on"{"1"}else{"0"});
  steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","0-3",a["binary"],case["image"],case["request"],"1",d],"environment":env,"timeout_seconds":600}));
  steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",d.parent().unwrap(),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
  jobs.push(json!({"name":name,"directory":d,"public_sha256":case["public_sha256"]}));
 }}
 save(r.join("preflight-jobs.json"),&json!(jobs));save(r.join("preflight-pins.json"),&json!(pins));
 save(r.join("preflight-plan.json"),&json!({"version":1,"source_pins":r.join("preflight-pins.json"),"steps":steps}));
 println!("All-nine OFF/ON preflight ready:72SDK18exports; prior6850");
}

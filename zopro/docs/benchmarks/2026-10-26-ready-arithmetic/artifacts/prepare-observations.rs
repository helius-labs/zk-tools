use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{fs,path::Path,collections::{BTreeMap,BTreeSet}};
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn add(p:impl AsRef<Path>,pins:&mut BTreeMap<String,String>){let p=p.as_ref();pins.insert(p.to_str().unwrap().into(),hash(p));}
fn main(){let r=Path::new("/tmp/zopro-ready-arithmetic");assert_eq!(read(r.join("native/result.json"))["status"],"complete");
 let artifact=read(r.join("native/on/artifacts.json"));let binary=artifact["binary"].as_str().unwrap();assert_eq!(hash(binary),artifact["binary_sha256"].as_str().unwrap());
 let old=read("/tmp/zopro-g2-owned-finish/perf-plan.json");let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r.join("source-pins.json"))).unwrap();
 for(p,h)in pins.clone(){assert_eq!(hash(&p),h);}
 for p in [r.join("profile-addendum.md"),r.join("prepare-observations.rs"),r.join("native/result.json"),r.join("native/on/artifacts.json"),r.join("native/on-core/stdout.log"),r.join("native/on-prover/stdout.log"),r.join("native-plan.json")] {add(p,&mut pins);}
 for(p,h)in artifact["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}add(binary,&mut pins);
 let a=read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");add(a["binary"].as_str().unwrap(),&mut pins);for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}
 for(p,h)in old["pins"].as_object().unwrap(){if p.contains("/registry/")||p.starts_with("/tmp/zopro-keys/")||p.starts_with("/tmp/zopro-gnark-verify/"){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}}
 let mut prior:BTreeSet<String>=serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
 let previous=read("/tmp/zopro-g2-owned-finish/proof-inventory.json");let mut newly_added=0;for(p,h)in previous.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());if p.ends_with(".proof")&&prior.insert(h.as_str().unwrap().into()){newly_added+=1;}}
 save(r.join("prior-audit.json"),&json!({"previous_prior":old["prior_proof_sha256"].as_array().unwrap().len(),"previous_exports_new_to_prior":newly_added,"prior_for_preflight":prior.len()}));add(r.join("prior-audit.json"),&mut pins);add("/tmp/zopro-g2-owned-finish/proof-inventory.json",&mut pins);
 let mut cases=Vec::new();for name in ["1x2-w32","1x2-w16","8x2-w32"]{let mut c=old["cases"].as_array().unwrap().iter().find(|c|c["name"]==name).unwrap().clone();c["rounds"]=json!(17);c["profile_rounds"]=json!(51);cases.push(c);}
 let variants=json!([{"name":"accepted","binary":a["binary"]},{"name":"off","binary":binary},{"name":"on","binary":binary,"environment":{"ZOPRO_READY_OUTPUT":"1"}}]);
 let mut steps=Vec::new();
 for c in &cases {add(c["image"].as_str().unwrap(),&mut pins);add(c["request"].as_str().unwrap(),&mut pins);
  for mode in ["off","on"]{let name=format!("{}-{mode}",c["name"].as_str().unwrap());let proofs=r.join("preflight-proofs").join(&name);let stem=Path::new(c["image"].as_str().unwrap()).file_stem().unwrap().to_str().unwrap();let proofdir=proofs.join(stem);let mut env=old["environment"].clone();env["RAYON_NUM_THREADS"]=c["workers"].clone(); // explicitly convert the integer below
   env["RAYON_NUM_THREADS"]=json!(c["workers"].as_u64().unwrap().to_string());if mode=="on"{env["ZOPRO_READY_OUTPUT"]=json!(proofdir.join("readiness.json"));}
   steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",c["cpu_affinity"],binary,c["image"],c["request"],"1",proofdir],"environment":env,"timeout_seconds":120}));
   steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",proofs,"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
  }
 }
 save(r.join("observation-input-pins.json"),&json!(pins));
 save(r.join("preflight-plan.json"),&json!({"version":1,"environment":{},"source_pins":r.join("observation-input-pins.json"),"steps":steps}));
 let mut perf=old.clone();perf["blocks"]=json!(3);perf["cases"]=json!(cases);perf["variants"]=variants;perf["checks"]=json!([]);perf["profile_cases"]=json!(["1x2-w32"]);perf["profile_blocks"]=json!(1);perf["command"]=json!(["bash","-c","if test \"$ZOPRO_READY_OUTPUT\" = 1; then export ZOPRO_READY_OUTPUT=\"$5/readiness.json\"; fi; exec \"$@\"","ready-sdk","{binary}","{image}","{request}","{rounds}","{proof_dir}/{stem}"]);perf["prior_proof_sha256"]=json!(prior);perf["pins"]=json!(pins);
 save(r.join("perf-template.json"),&perf);
 println!("Frozen six preflights, three cases x three blocks x three variants, then three cycle profiles; prior{}",prior.len());
}

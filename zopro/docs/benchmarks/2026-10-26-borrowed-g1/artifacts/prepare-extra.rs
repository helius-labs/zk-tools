use std::{fs,path::Path};use serde_json::{json,Value};use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-borrowed-g1";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){let r=Path::new(R);let mut n=0;
 for(p,h)in read(r.join("native/source-pins.json")).as_object().unwrap(){if p.contains("crates/zopro-bn254/")||p.ends_with("Cargo.toml")||p.ends_with("Cargo.lock")||p.ends_with("scripts/ifma8.py"){assert_eq!(hash(p),h.as_str().unwrap());n+=1;}}
 assert_eq!(read(r.join("native/completed.json")),json!(["compiler","generator-check","field"]));assert_eq!(read(r.join("native/field/tests.json"))["passed_tests"].as_array().unwrap().len(),33);
 save(r.join("reused-checks-audit.json"),&json!({"unchanged_field_workspace_generator_inputs_checked":n,"original_native_completed":["compiler","generator-check","field"],"field_tests":33,"repeated_passed_tests":false,"repair_only_changed_new_core_test_oracle":true,"remaining_native_root":"native-repair"}));
 let mut c=read(r.join("perf-before-preflight.json"));
 for cat in c["symbol_categories"].as_array_mut().unwrap(){if cat["name"]=="G1_arithmetic"{cat["contains"].as_array_mut().unwrap().push(json!("borrowed_pending"));}}
 for rel in ["prepare-extra.rs","prepare-extra","repair.md","link-path-repair.md","native-plan-repair.json","native-repair/protocol.json","native/source-pins.json","native/completed.json","native/field/tests.json","native/field/stdout.log","native/field/stderr.log","native/compiler/stdout.log","native/generator-check/stdout.log","reused-checks-audit.json"]{let p=r.join(rel);c["pins"][p.display().to_string()]=json!(hash(&p));}
 save(r.join("preflight-pins.json"),&c["pins"]);save(r.join("perf-before-preflight.json"),&c);println!("Reused33field tests verified, repair protocol/profile labels pinned before capture");
}

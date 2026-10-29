use std::{fs,path::Path};use serde_json::{json,Value};use sha2::{Sha256,Digest};
const R:&str="/home/ubuntu/src/zopro-perf-work/shared-wire-sdk175";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){let r=Path::new(R);
let mut audit=read(r.join("source-audit.json"));let edits=audit["edits"].as_object_mut().unwrap();let key="crates/zopro-core/src/msm_affine/grouping.rs";
let after=edits[key][0][1].as_str().unwrap().replace("pub(super) use wire_union::scalar_groups_pair","pub(crate) use wire_union::scalar_groups_pair");edits.get_mut(key).unwrap()[0][1]=json!(after);save(r.join("source-audit.json"),&audit);
let mut pins=read(r.join("build-pins.json"));for(p,h)in pins.as_object_mut().unwrap(){*h=json!(hash(p));}
for file in["repair.rs","initial-native/grouping.rs","initial-native/build-plan.json","initial-native/build-pins.json"]{pins[r.join(file).display().to_string()]=json!(hash(r.join(file)));}save(r.join("build-pins.json"),&pins);
let mut plan=read(r.join("build-plan.json"));let steps=plan["steps"].as_array_mut().unwrap();steps.drain(..5);save(r.join("build-plan.json"),&plan);
save(r.join("repair.json"),&json!({"reason":"E0364/E0603 cfg-only re-export restricted to msm_affine; widen grouping re-export to crate","runtime_algorithm_changed":false,"successful_OFF_steps_repeated":false,"candidate_tests_before_failure":0,"original_workflow":"native","remaining_workflow":"native-candidate","new_plan_steps":5}));
}

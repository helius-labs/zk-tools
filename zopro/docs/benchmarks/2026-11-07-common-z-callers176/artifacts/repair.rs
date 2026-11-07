use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/common-z-callers176";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:&str,v:&Value){assert!(!Path::new(p).exists());fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){
 let old=fs::read_to_string(format!("{R}/finish-before-test-assert-repair.rs")).unwrap();let new=fs::read_to_string(format!("{R}/finish.rs")).unwrap();
 let marker="#[cfg(test)]";assert_eq!(old.split(marker).next(),new.split(marker).next());
 let path=format!("{R}/source/crates/zopro-core/src/msm/common_z_finish.rs");assert_eq!(fs::read_to_string(&path).unwrap(),old);fs::write(&path,&new).unwrap();
 let mut pins=read(&format!("{R}/build-pins.json"));
 for p in [&path,&format!("{R}/finish.rs"),&format!("{R}/finish-before-test-assert-repair.rs"),&format!("{R}/repair.rs"),&format!("{R}/repair")]{pins[p]=json!(hash(p));}
 for arm in ["fresh","candidate"]{let p=format!("{R}/libs/{arm}/libzopro_core.rlib");pins[&p]=json!(hash(&p));}
 let mut audit=read(&format!("{R}/source-audit.json"));audit["candidate_source_hashes"]["crates/zopro-core/src/msm/common_z_finish.rs"]=json!(hash(&path));
 audit["test_only_repair"]=json!({"old":format!("{R}/finish-before-test-assert-repair.rs"),"new":format!("{R}/finish.rs"),"runtime_prefix_exact":true,"reason":"assert_eq! requires Debug on private FqLimbs; assert! equality preserves the check","successful_native_libraries_rebuilt":false});
 save(&format!("{R}/source-audit-after-test-repair.json"),&audit);
 save(&format!("{R}/continuation-pins.json"),&pins);
 let original=read(&format!("{R}/build-plan.json"));let steps=original["steps"].as_array().unwrap()[2..].to_vec();
 save(&format!("{R}/continuation-plan.json"),&json!({"version":1,"source_pins":format!("{R}/continuation-pins.json"),"steps":steps}));
 println!("Prepared continuation from failed test compile only; runtime source prefix unchanged; original fresh/candidate libraries reused");
}

use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 assert_eq!(read(format!("{R}/qualification/result.json"))["status"],"complete");assert!(!Path::new(&format!("{R}/build-plan.json")).exists());
 for arm in["OFF","ON"]{for(name,n)in[("field",39),("core",125),("prover",2),("cubic",6)]{let t=read(format!("{R}/qualification/{arm}-{name}/tests.json"));assert_eq!(t["unfiltered"],true);assert_eq!(t["passed_tests"].as_array().unwrap().len(),n);}}
 let old="/home/ubuntu/src/zopro-perf-work/weighted-sdk176";
 let freeze=fs::read_to_string(format!("{old}/native.rs")).unwrap().replace(old,R).replace("\"candidate\", \"control\"","\"ON\", \"OFF\"").replace("label == \"candidate\"","label == \"ON\"").replace("\"candidate-flags.txt\"","\"ON-flags.txt\"").replace("\"native-flags.txt\"","\"OFF-flags.txt\"").replace("native/libraries-{label}/stdout.log","build/libraries-{label}/stdout.log");
 fs::write(format!("{R}/freeze.rs"),freeze).unwrap();let status=Command::new("rustc").args(["--edition=2021",&format!("{R}/freeze.rs"),"-O","-L","dependency=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps","--extern","serde_json=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps/libserde_json-cdebb9338c2784ad.rlib","--extern","sha2=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps/libsha2-16b8f2c03bdf5faa.rlib","-o",&format!("{R}/freeze")]).status().unwrap();assert!(status.success());
 let driver=fs::read_to_string(format!("{old}/driver.rs")).unwrap();let at=driver.rfind("fn main()->").unwrap();fs::write(format!("{R}/driver.rs"),format!("{}fn main()->Result<(),Box<dyn std::error::Error>>{{original_main()}}\n",&driver[..at])).unwrap();fs::create_dir_all(format!("{R}/support")).unwrap();fs::copy(format!("{old}/support/perf.rs"),format!("{R}/support/perf.rs")).unwrap();
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(format!("{R}/qualification-pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
 for f in["prepare-sdk.rs","prepare-sdk","freeze.rs","freeze","driver.rs","support/perf.rs","qualification/result.json"]{let p=format!("{R}/{f}");pins.insert(p.clone(),hash(&p));}
 let mut steps=vec![];for arm in["OFF","ON"]{
  let mut env=read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json");env["RUSTFLAGS"]=json!(fs::read_to_string(format!("{R}/{arm}-flags.txt")).unwrap());env["CARGO_TARGET_DIR"]=json!(format!("{R}/target-{arm}"));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");let manifest=format!("{R}/source/Cargo.toml");
  steps.push(json!({"kind":"command","name":format!("libraries-{arm}"),"argv":["cargo","build","--manifest-path",manifest,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
  steps.push(json!({"kind":"command","name":format!("freeze-{arm}"),"argv":[format!("{R}/freeze"),"freeze",arm],"environment":env,"timeout_seconds":600}));
  steps.push(json!({"kind":"link-sdk","name":format!("sdk-{arm}"),"artifacts":format!("{R}/frozen-{arm}-cargo.jsonl"),"driver":format!("{R}/driver.rs"),"additional_sources":[format!("{R}/support/perf.rs")],"timeout_seconds":1800}));
 }
 fs::write(format!("{R}/build-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(format!("{R}/build-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":steps})).unwrap()).unwrap();
 println!("Prepared coherentOFF/ON libraries/freezes/SDKlinks after39field+125core+8prover perarm; no timing");
}

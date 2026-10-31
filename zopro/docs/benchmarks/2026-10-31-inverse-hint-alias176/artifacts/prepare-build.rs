use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 assert_eq!(read(&format!("{R}/qualification-v2/result.json"))["status"],"complete");
 assert_eq!(read(&format!("{R}/qualification-v2/core/tests.json"))["passed_tests"].as_array().unwrap().len(),128);
 assert!(!Path::new(&format!("{R}/build-plan.json")).exists());
 let old="/home/ubuntu/src/zopro-perf-work/weighted-sdk176";
 let code=fs::read_to_string(format!("{old}/native.rs")).unwrap().replace(old,R)
   .replace("native/libraries-{label}/stdout.log","build/libraries-{label}/stdout.log")
   .replace("target-{label}/release/.fingerprint","target-native/release/.fingerprint")
   .replace("candidate-flags.txt","native-flags.txt");
 fs::write(format!("{R}/freeze.rs"),code).unwrap();
 let status=std::process::Command::new("rustc").args(["--edition=2021",&format!("{R}/freeze.rs"),"-O","-L","dependency=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps","--extern","serde_json=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps/libserde_json-cdebb9338c2784ad.rlib","--extern","sha2=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps/libsha2-16b8f2c03bdf5faa.rlib","-o",&format!("{R}/freeze")]).status().unwrap();assert!(status.success());
 let driver=fs::read_to_string(format!("{old}/driver.rs")).unwrap();let start=driver.rfind("fn main()->").unwrap();
 let driver=format!("{}fn main()->Result<(),Box<dyn std::error::Error>>{{let result=original_main();println!(\"INVERSE_ALIAS {{}}\",serde_json::json!({{\"layout_and_counts\":zopro_prover::zopro_core::inverse_hint_alias_snapshot()}}));result}}\n",&driver[..start]);
 fs::write(format!("{R}/driver.rs"),driver).unwrap();fs::create_dir_all(format!("{R}/support")).unwrap();fs::copy(format!("{old}/support/perf.rs"),format!("{R}/support/perf.rs")).unwrap();
 let mut env=read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json");env["RUSTFLAGS"]=json!(fs::read_to_string(format!("{R}/native-flags.txt")).unwrap());env["CARGO_BUILD_JOBS"]=json!("16");env["CARGO_TARGET_DIR"]=json!(format!("{R}/target-native"));env["RAYON_NUM_THREADS"]=json!("4");env["ZOPRO_CPU_INVERSE_ALIASES"]=json!("1");
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(&format!("{R}/qualification-v2-pins.json"))).unwrap();
 for(p,h)in &pins{assert_eq!(hash(p),*h);}
 for name in ["prepare-build.rs","prepare-build","capture-scope.md","driver.rs","support/perf.rs","freeze.rs","freeze","qualification-v2/result.json","qualification-v2/core/tests.json","qualification-v2/prover-ON/tests.json"] {let p=format!("{R}/{name}");pins.insert(p.clone(),hash(&p));}
 fs::write(format!("{R}/build-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 let manifest=format!("{R}/source-v2/Cargo.toml");
 fs::write(format!("{R}/build-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":[
 {"kind":"command","name":"cubic-ON","argv":["cargo","test","--manifest-path",manifest,"--locked","--offline","--release","-p","zopro-prover","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800},
 {"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",manifest,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800},
 {"kind":"command","name":"freeze-candidate","argv":[format!("{R}/freeze"),"freeze","candidate"],"environment":env,"timeout_seconds":600},
 {"kind":"link-sdk","name":"sdk","artifacts":format!("{R}/frozen-candidate-cargo.jsonl"),"driver":format!("{R}/driver.rs"),"additional_sources":[format!("{R}/support/perf.rs")],"timeout_seconds":1800}
 ]})).unwrap()).unwrap();println!("Prepared cubic-ON/coherent-library-freeze/SDK link after128core tests; fullSDK scope declared before timing");
}

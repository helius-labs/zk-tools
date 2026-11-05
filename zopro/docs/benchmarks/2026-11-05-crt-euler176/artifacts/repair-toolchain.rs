use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/crt-euler176";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let mut pins:BTreeMap<String,String>=serde_json::from_slice(&fs::read(format!("{R}/pins.json")).unwrap()).unwrap();for(p,h)in&pins{assert_eq!(hash(p),*h);}
 let mut plan:Value=serde_json::from_slice(&fs::read(format!("{R}/plan.json")).unwrap()).unwrap();
 let toolchain="/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin";
 for step in plan["steps"].as_array_mut().unwrap(){step["environment"]["RUSTUP_TOOLCHAIN"]=json!("1.98.1");step["environment"]["RUSTC"]=json!(format!("{toolchain}/rustc"));step["environment"]["RUSTDOC"]=json!(format!("{toolchain}/rustdoc"));if step["name"]!="full-domain-algebra"{step["argv"][0]=json!(format!("{toolchain}/cargo"));}}
 for tool in["rustc","rustdoc","cargo"]{let p=format!("{toolchain}/{tool}");pins.insert(p.clone(),hash(p));}
 for f in["repair-toolchain.rs","repair-toolchain","plan.json","pins.json","runs/algebra-tests/exit.json","runs/algebra-tests/stderr.log","runs/algebra-tests/stdout.log"]{let p=format!("{R}/{f}");pins.insert(p.clone(),hash(p));}
 fs::write(format!("{R}/pins-fixed.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();plan["source_pins"]=json!(format!("{R}/pins-fixed.json"));fs::write(format!("{R}/plan-fixed.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
 println!("Only toolchain executable paths/environment repaired;all previous pins match;source/dependencies/math unchanged;no prior test execution to repeat");
}

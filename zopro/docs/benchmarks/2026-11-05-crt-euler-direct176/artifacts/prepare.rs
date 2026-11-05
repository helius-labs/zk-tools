use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/crt-euler-direct176";const P:&str="/home/ubuntu/src/zopro-perf-work/crt-euler176";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 assert!(!Path::new(&format!("{R}/plan.json")).exists());let mut pins:BTreeMap<String,String>=serde_json::from_slice(&fs::read(format!("{P}/pins-fixed.json")).unwrap()).unwrap();for(p,h)in&pins{assert_eq!(hash(p),*h);}
 let parent=fs::read_to_string(format!("{P}/src/lib.rs")).unwrap();fs::write(format!("{R}/base.rs"),format!("{parent}\n#[path=\"direct.rs\"]pub mod direct;\n")).unwrap();
 let deps=format!("{P}/target/release/deps");let serde="/home/ubuntu/src/zopro-perf-work/tool-target/release/deps/libserde_json-cdebb9338c2784ad.rlib";
 let field="/home/ubuntu/src/zopro-perf-work/finalize-drain-bundle176/frozen/candidate/libzopro_bn254-9d1a3c72c557c8816b4532053cddad31ed02058854f8eec236b60ecf0bc31869.rlib";
 let libs=[("zopro_bn254",field.to_string()),("tfhe_ntt",format!("{deps}/libtfhe_ntt-3e39e5acca52175c.rlib")),("num_bigint",format!("{deps}/libnum_bigint-be646e83c0c62d72.rlib")),("num_traits",format!("{deps}/libnum_traits-c2e07c77c90d1316.rlib")),("serde_json",serde.into())];
 let compiler="/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc";pins.insert(compiler.into(),hash(compiler));for(_,p)in&libs{pins.insert(p.clone(),hash(p));}
 for f in["design.md","direct.rs","base.rs","driver.rs","prepare.rs","prepare"]{let p=format!("{R}/{f}");pins.insert(p.clone(),hash(p));}
 for f in["review.json","review.rs","algebra.json","src/lib.rs"]{let p=format!("{P}/{f}");pins.insert(p.clone(),hash(p));}
 let compile=|test:bool|{let mut a=vec![compiler.to_string(),"--edition=2021".into(),"-O".into(),"-C".into(),"target-cpu=native".into(),"-C".into(),"lto=fat".into(),"-C".into(),"codegen-units=1".into(),format!("{R}/{}",if test{"base.rs"}else{"driver.rs"}),"-L".into(),format!("dependency={deps}"),"-L".into(),"dependency=/home/ubuntu/src/zopro-perf-work/tool-target/release/deps".into()];for(name,p)in&libs{a.extend(["--extern".into(),format!("{name}={p}")]);}if test{a.push("--test".into());}a.extend(["-o".into(),format!("{R}/{}",if test{"direct-tests"}else{"direct-full-domain"})]);a};
 let cmd=|name:&str,argv:Vec<String>|json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600,"unfiltered":false});
 let steps=vec![cmd("compile-direct-tests",compile(true)),cmd("new-direct-tests",vec![format!("{R}/direct-tests"),"direct::tests::".into(),"--test-threads=1".into()]),cmd("compile-full-domain",compile(false)),cmd("full-domain-direct",vec![format!("{R}/direct-full-domain")])];fs::write(format!("{R}/pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(format!("{R}/plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":steps})).unwrap()).unwrap();println!("Prepared4direct-CRT integer algebra steps;{}pins;exact acceptedfield linked;parenttests inherited/filterout",pins.len());
}

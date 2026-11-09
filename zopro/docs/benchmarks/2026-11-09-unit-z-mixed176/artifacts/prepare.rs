use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/unit-z-mixed176";
const OLD:&str="/home/ubuntu/src/zopro-perf-work/weighted-double-add176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
    assert!(!Path::new(&format!("{R}/plan.json")).exists());
    let old=read(format!("{OLD}/native-plan.json"));
    let mut native=old["steps"][0].clone();
    let argv=native["argv"].as_array_mut().unwrap();
    for v in argv.iter_mut(){if let Some(t)=v.as_str(){*v=json!(t.replace(OLD,R));}}
    argv.extend([json!("--cfg"),json!("unit_z_native_test")]);
    native["name"]=json!("build-native-independent-qualification");
    let mut generic=native.clone();generic["name"]=json!("build-generic-noASM-noIFMA-qualification");
    let args=generic["argv"].as_array_mut().unwrap();
    for v in args.iter_mut(){if let Some(t)=v.as_str(){*v=json!(t.replace("target-cpu=native","target-cpu=generic").replace("zopro_bn254=/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib","zopro_bn254=/home/ubuntu/src/zopro-perf-work/tree-policy-owner176/libzopro_bn254-generic.rlib").replace(&format!("{R}/qualification"),&format!("{R}/qualification-generic")));}}
    // Only the output changes; the source is the same new qualification.
    for v in args.iter_mut(){if *v==format!("{R}/qualification-generic.rs"){*v=json!(format!("{R}/qualification.rs"));}}
    let at=args.iter().position(|v|v=="unit_z_native_test").unwrap();args.drain(at-1..at+1);
    args.extend([json!("--cfg"),json!("no_ifma"),json!("--cfg"),json!("no_asm")]);
    let steps=vec![native,json!({"kind":"command","name":"native-independent-curve-and-worker-tests","argv":[format!("{R}/qualification"),"--nocapture","--test-threads=1"],"timeout_seconds":600}),generic,json!({"kind":"command","name":"generic-independent-curve-tests","argv":[format!("{R}/qualification-generic"),"--nocapture","--test-threads=1"],"timeout_seconds":600}),json!({"kind":"command","name":"decode-native-actual-entrypoints","argv":["objdump","-d","-C",format!("{R}/qualification")],"timeout_seconds":120})];
    let mut pins=BTreeMap::<String,Value>::new();
    let manifest="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json";
    let m=read(manifest);assert_eq!(m["adopted"],true);
    for(rel,h)in m["source_files_sha256"].as_object().unwrap(){let p=format!("/home/ubuntu/src/zk-tools/zopro/{rel}");assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p,h.clone());}
    pins.insert(manifest.into(),json!(hash(manifest)));
    for f in["design.md","kernel.rs","qualification.rs","prepare.rs","prepare"]{let p=format!("{R}/{f}");pins.insert(p.clone(),json!(hash(p)));}
    for s in &steps{for v in s["argv"].as_array().unwrap(){let t=v.as_str().unwrap();let p=t.split_once('=').map_or(t,|(_,p)|p);if Path::new(p).is_file(){pins.insert(p.into(),json!(hash(p)));}}}
    let plan=json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":steps});
    fs::write(format!("{R}/plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
    pins.insert(format!("{R}/plan.json"),json!(hash(format!("{R}/plan.json"))));
    fs::write(format!("{R}/pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    println!("Unit-Z qualification prepared:5commands/{}pins;unchanged accepted176/field1200;no clocks/proofs",pins.len());
}

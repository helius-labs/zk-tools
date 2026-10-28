use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,collections::BTreeMap,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(format!("{R}/{p}")).unwrap()).unwrap()}
fn save(p:&str,v:&Value){fs::write(format!("{R}/{p}"),serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){
    assert_eq!(read("native-rest/result.json")["status"],"complete");assert_eq!(read("native/failure.json")["completed"],json!(["frozen-core-tests","frozen-build"]));
    let audit=read("codegen-audit.json");let identities=read("probe-artifacts.json");let mut pins:BTreeMap<String,String>=serde_json::from_value(read("pins.json")).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
    for label in["frozen","fresh","selected"]{assert_eq!(audit["variants"][label]["binary_sha256"],identities[label]["sha256"]);for row in audit["variants"][label]["G1_stream_symbols"].as_array().unwrap(){let p=row["disassembly"].as_str().unwrap();pins.insert(p.to_string(),hash(p));}}
    for f in["codegen.rs","codegen","codegen-audit.json","seal.rs","seal","experiment-v3.rs","experiment-v3","preparation-repair.json","freezer-repair.json","native/failure.json","native-first/failure.json","native-rest/generator-check/stdout.log"]{pins.insert(format!("{R}/{f}"),hash(&format!("{R}/{f}")));}
    let o=Command::new("rustup").args(["which","rustc"]).output().unwrap();assert!(o.status.success());let actual=String::from_utf8(o.stdout).unwrap().trim().to_string();pins.insert(actual.clone(),hash(&actual));
    save("capture-pins-final.json",&json!(pins));let mut plan=read("run-plan.json");plan["source_pins"]=json!(format!("{R}/capture-pins-final.json"));save("capture-plan-final.json",&plan);save("capture-seal.json",&json!({"pins":pins.len(),"codegen_matches_actual_probes":true,"test_recovery_scope":"two initial frozen G1 successes retained, only failed G2 retried; freezer-only repair retained successful build/tests","source_kernel_changed":false,"all_prior_pins_verified":true,"actual_rustc_pinned":actual,"captures_started":false}));println!("Sealed{}pins;allqualification/codegen/identitycheckscomplete",pins.len());
}

use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-real-norm-z176";const OUT:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-07-g2-real-norm-z176/artifacts";
fn r(p:&str)->PathBuf{Path::new(R).join(p)}fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let v=read(r("generator-oracle.json"));assert_eq!(v["passed"],true);assert_eq!(v["new_complete_reductions"],160);assert_eq!(v["new_generator_oracle_arguments"],80);assert!(v["scalars_with_bit253_set"].as_u64().unwrap()>0);assert_eq!(v["source_sha256"],hash(r("model.rs")));assert_eq!(v["receipts"].as_array().unwrap().len(),80);
 for(p,h)in read(r("generator-oracle-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let mut receipts=Vec::new();for s in read(r("generator-oracle-plan.json"))["steps"].as_array().unwrap(){let name=s["name"].as_str().unwrap();let base=r(&format!("generator-oracle-runs/{name}"));let e=read(base.join("exit.json"));let cmd=read(base.join("command.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);assert_eq!(&cmd["argv"].as_array().unwrap()[4..],s["argv"].as_array().unwrap().as_slice());for stream in["stdout","stderr"]{assert_eq!(hash(base.join(format!("{stream}.log"))),e[format!("{stream}_sha256")].as_str().unwrap());}
  let dest=Path::new(OUT).join("generator-oracle-runs").join(name);fs::create_dir_all(&dest).unwrap();for f in["command.json","exit.json","stdout.log","stderr.log"]{fs::copy(base.join(f),dest.join(f)).unwrap();}receipts.push(json!({"command":cmd,"exit":e}));
 }
 let audit=json!({"passed":true,"new_reductions":160,"new_closed_form_generator_arguments":80,"source_and_pins_match":true,"commands":receipts,"old_580_cases_replayed":false,"old_analysis_and_review_replayed":false,"scope":"new argument generator oracle and command/source integrity; accepted field shared, no independent field or old arithmetic rerun"});let p=r("generator-oracle-audit.json");assert!(!p.exists());fs::write(&p,serde_json::to_vec_pretty(&audit).unwrap()).unwrap();
 for f in["generator-oracle.rs","generator-oracle.json","generator-oracle-pins.json","generator-oracle-plan.json","generator-oracle-audit.json","record-generator.rs"]{fs::copy(r(f),Path::new(OUT).join(f)).unwrap();}
 println!("Recorded160new generator-oracle reductions and2first-success receipts; primary580cases/model/analysis/review untouched");
}

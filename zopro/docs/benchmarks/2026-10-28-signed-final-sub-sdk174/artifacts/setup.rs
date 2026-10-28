use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174";
const C:&str="/home/ubuntu/src/zopro-perf-work/signed-final-sub174";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn copy_source(from:&Path,to:&Path,pins:&mut BTreeMap<String,String>){fs::create_dir_all(to).unwrap();for e in fs::read_dir(from).unwrap(){let e=e.unwrap();let p=e.path();let target=to.join(e.file_name());if e.file_type().unwrap().is_dir(){copy_source(&p,&target,pins);}else{fs::copy(&p,&target).unwrap();assert_eq!(hash(&p),hash(&target));pins.insert(target.display().to_string(),hash(target));}}}
fn main(){
 assert!(!r("build-plan.json").exists());let old=Path::new(C);assert_eq!(read(old.join("analysis.json"))["component_gate"],true);
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(old.join("pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(&hash(p),h);}
 copy_source(&old.join("source"),&r("source"),&mut pins);assert_eq!(pins.keys().filter(|p|p.starts_with(&format!("{R}/source/"))).count(),176);
 for f in ["source-audit.json","frozen-control.json","frozen-candidate.json","frozen-control-cargo.jsonl","frozen-candidate-cargo.jsonl"]{fs::copy(old.join(f),r(f)).unwrap();pins.insert(r(f).display().to_string(),hash(r(f)));}
 let native=read(old.join("build-plan.json"));let mut steps=Vec::new();
 for label in ["control","candidate"]{
  let step=native["steps"].as_array().unwrap().iter().find(|s|s["name"]==format!("zopro-core-{label}")).unwrap();let env=step["environment"].clone();
  steps.push(json!({"kind":"command","name":format!("zopro-prover-{label}"),"argv":["cargo","test","--manifest-path",old.join("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
  steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":Path::new(Z).join("crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[Path::new(Z).join("crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
 }
 for p in [r("setup.rs"),r("setup-helper"),r("design.md"),old.join("analysis.json"),old.join("history-audit.json"),old.join("bounds.md")] {pins.insert(p.display().to_string(),hash(p));}
 save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
 println!("Prepared4SDKnative steps using exact qualified frozen libs,176source snapshot;no caller rerun");
}

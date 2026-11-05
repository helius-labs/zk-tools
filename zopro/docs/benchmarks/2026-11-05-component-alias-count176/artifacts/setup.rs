use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/component-alias-count176";
fn path(s:&str)->PathBuf{Path::new(ROOT).join(s)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 assert!(!path("plan.json").exists());
 let repo="/home/ubuntu/src/zk-tools/zopro";
 let manifest=format!("{repo}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
 let m=read(&manifest);let mut pins=BTreeMap::<String,String>::new();
 for(p,h)in m["source_files_sha256"].as_object().unwrap(){let p=format!("{repo}/{p}");assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p,h.as_str().unwrap().to_string());}
 assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),176);
 let input_inventory="/home/ubuntu/src/zopro-perf-work/bounded-coalesce176/inputs.json";
 let all=read(input_inventory);let selected=json!({"1x2-B":all["1x2-B"],"2x2-B":all["2x2-B"]});
 for input in selected.as_object().unwrap().values(){let p=input["file"].as_str().unwrap();assert_eq!(hash(p),input["sha256"].as_str().unwrap());pins.insert(p.into(),hash(p));}
 fs::write(path("inputs.json"),serde_json::to_vec_pretty(&selected).unwrap()).unwrap();
 let phi=fs::read_to_string(format!("{repo}/crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
 let prefix=phi.split_once("const C1:").unwrap().1.split_once("use zopro_bn254::{Fq,").unwrap().0;
 let old="/home/ubuntu/src/zopro-perf-work/lfg-multiplier-planes176/split.rs";
 let tail=fs::read_to_string(old).unwrap();let tail=tail.split_once("fn booth_digit").unwrap().1;
 fs::write(path("split.rs"),format!("const C1:{prefix}fn booth_digit{tail}")).unwrap();
 let frozen="/home/ubuntu/src/zopro-perf-work/finalize-drain-bundle176/frozen-candidate.json";
 let field=read(frozen)["libraries"]["zopro_bn254"].as_str().unwrap().to_string();
 let deps="/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
 let serde=format!("{deps}/libserde_json-cdebb9338c2784ad.rlib");
 let compiler=String::from_utf8(std::process::Command::new("rustup").args(["which","rustc"]).output().unwrap().stdout).unwrap().trim().to_string();
 let xtask="/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
 for p in[manifest.as_str(),input_inventory,old,frozen,field.as_str(),serde.as_str(),compiler.as_str(),xtask]{pins.insert(p.into(),hash(p));}
 for f in["setup.rs","setup","study.rs","split.rs","inputs.json","design.md"]{pins.insert(path(f).display().to_string(),hash(path(f)));}
 let compile=|test:bool|{let mut args=vec![compiler.clone(),"--edition=2021".into(),"-O".into(),"-C".into(),"target-cpu=native".into(),path("study.rs").display().to_string(),"-L".into(),format!("dependency={deps}"),"--extern".into(),format!("serde_json={serde}"),"--extern".into(),format!("zopro_bn254={field}")];if test{args.push("--test".into());}args.extend(["-o".into(),path(if test{"study-tests"}else{"study"}).display().to_string()]);args};
 let step=|name:&str,args:Vec<String>,unfiltered:bool|json!({"kind":"command","name":name,"argv":args,"timeout_seconds":300,"unfiltered":unfiltered});
 let steps=vec![step("compile-tests",compile(true),false),step("alias-tests",vec![path("study-tests").display().to_string(),"--test-threads=1".into()],true),step("compile-study",compile(false),false),step("real-alias-counts",vec![path("study").display().to_string()],false)];
 fs::write(path("pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 fs::write(path("plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":path("pins.json"),"steps":steps})).unwrap()).unwrap();
 println!("Prepared4steps,{}pins,176productionfilesexact;fixedtwoBqueries;zero newproofs",pins.len());
}

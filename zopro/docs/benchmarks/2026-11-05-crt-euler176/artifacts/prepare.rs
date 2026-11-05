use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},process::Command,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/crt-euler176";
fn path(s:&str)->PathBuf{Path::new(R).join(s)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn walk(p:&Path,pins:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let t=e.file_type().unwrap();assert!(!t.is_symlink());if t.is_dir(){if e.file_name()!="target"&&e.file_name()!=".git"{walk(&e.path(),pins);}}else if t.is_file(){pins.insert(e.path().display().to_string(),hash(e.path()));}}}
fn main(){
 assert!(!path("plan.json").exists());
 let manifest="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json";
 let m=read(manifest);let mut pins=BTreeMap::new();for(p,h)in m["source_files_sha256"].as_object().unwrap(){let p=format!("/home/ubuntu/src/zk-tools/zopro/{p}");assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p,h.as_str().unwrap().to_string());}assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),176);
 let cargo=String::from_utf8(Command::new("rustup").args(["which","cargo"]).output().unwrap().stdout).unwrap().trim().to_string();
 let rustc=String::from_utf8(Command::new("rustup").args(["which","rustc"]).output().unwrap().stdout).unwrap().trim().to_string();
 let meta=Command::new(&cargo).args(["metadata","--locked","--format-version","1","--manifest-path",path("Cargo.toml").to_str().unwrap()]).output().unwrap();fs::write(path("metadata.stderr"),&meta.stderr).unwrap();assert!(meta.status.success());fs::write(path("metadata.json"),&meta.stdout).unwrap();let metadata:Value=serde_json::from_slice(&meta.stdout).unwrap();
 for pkg in metadata["packages"].as_array().unwrap(){let p=Path::new(pkg["manifest_path"].as_str().unwrap()).parent().unwrap();if p!=Path::new(R){walk(p,&mut pins);}}
 for f in["Cargo.toml","Cargo.lock","src/lib.rs","src/main.rs","design.md","prepare.rs","prepare","fetch-initial.log","fetch.log","metadata.json","metadata.stderr"]{pins.insert(path(f).display().to_string(),hash(path(f)));}
 for p in[manifest,cargo.as_str(),rustc.as_str(),"/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask"]{pins.insert(p.into(),hash(p));}
 let env=json!({"RUSTFLAGS":"-C target-cpu=native","CARGO_TARGET_DIR":path("target"),"CARGO_BUILD_JOBS":"16"});
 let command=|name:&str,args:Vec<String>,unfiltered:bool|json!({"kind":"command","name":name,"argv":args,"environment":env,"timeout_seconds":900,"unfiltered":unfiltered});
 let steps=vec![command("algebra-tests",vec![cargo.clone(),"test".into(),"--release".into(),"--locked".into(),"--manifest-path".into(),path("Cargo.toml").display().to_string(),"--lib".into(),"--".into(),"--test-threads=1".into()],true),command("build-full-domain",vec![cargo,"build".into(),"--release".into(),"--locked".into(),"--manifest-path".into(),path("Cargo.toml").display().to_string(),"--bin".into(),"zopro-crt-euler-study".into()],false),command("full-domain-algebra",vec![path("target/release/zopro-crt-euler-study").display().to_string()],false)];
 fs::write(path("pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(path("plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":path("pins.json"),"steps":steps})).unwrap()).unwrap();println!("Prepared3steps;{}pins;176originalsexact;{}packages;nativeRustAVX512dependency;no performancecapture",pins.len(),metadata["packages"].as_array().unwrap().len());
}

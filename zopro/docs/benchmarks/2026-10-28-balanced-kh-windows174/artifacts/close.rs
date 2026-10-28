use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-windows174";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-balanced-kh-windows174";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn walk(p:&Path,files:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let p=e.path();let n=e.file_name().to_string_lossy().to_string();if e.file_type().unwrap().is_dir(){if !n.starts_with("target-")&&n!="frozen"{walk(&p,files);}}else if e.file_type().unwrap().is_file(){files.push(p);}}}
fn seal(){let d=Path::new(D);let mut paths=Vec::new();walk(d,&mut paths);let mut pins=BTreeMap::new();for p in paths{let rel=p.strip_prefix(d).unwrap().display().to_string();if ["artifact-index.json","SHA256SUMS"].contains(&rel.as_str()){continue;}pins.insert(rel,hash(p));}save(d.join("artifact-index.json"),&json!(pins));pins.insert("artifact-index.json".into(),hash(d.join("artifact-index.json")));fs::write(d.join("SHA256SUMS"),pins.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();println!("Sealed{}hashes",pins.len());}
fn archive(){
 let r=Path::new(R);let d=Path::new(D);assert_eq!(read(r.join("runs/result.json"))["status"],"complete");assert_eq!(read(r.join("analysis.json"))["independent_proof_count"],15919);
 for(p,h)in read(r.join("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let mut paths=Vec::new();walk(r,&mut paths);let mut local=BTreeMap::new();
 for from in paths {let rel=from.strip_prefix(r).unwrap();let ext=from.extension().and_then(|s|s.to_str()).unwrap_or("");if ext=="data"{assert!(rel.starts_with("perf"));assert!(Command::new("sudo").args(["-n","chown","ubuntu:ubuntu"]).arg(&from).status().unwrap().success());local.insert(from.display().to_string(),hash(from));continue;}
 if !rel.starts_with("source")&&!rel.starts_with("before-test-brace-repair/source")&&!["rs","json","md","jsonl","log","txt","proof","public"].contains(&ext){continue;}
 if ["archive.log","push.log","remote.log","commit.log","strict-hashes.log","staged-whitespace.log","staged-whitespace-final.log"].contains(&from.file_name().unwrap().to_str().unwrap()){continue;}
 let to=d.join("artifacts").join(rel);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(&from,&to).unwrap();assert_eq!(hash(from),hash(to));}
 save(d.join("artifacts/local-perf-data.json"),&json!(local));let mut source=Vec::new();walk(&d.join("artifacts/source"),&mut source);assert_eq!(source.len(),175);seal();
}
fn main(){match std::env::args().nth(1).as_deref(){Some("archive")=>archive(),Some("seal")=>seal(),_=>panic!("archive/seal")}}

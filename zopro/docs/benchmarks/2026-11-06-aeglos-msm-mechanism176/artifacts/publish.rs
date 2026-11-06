use serde_json::{json, Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/aeglos-msm-mechanism176";
const P:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-aeglos-msm-mechanism176";
fn h(b:impl AsRef<[u8]>)->String{format!("{:x}",Sha256::digest(b.as_ref()))}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();if e.file_type().unwrap().is_dir(){walk(&e.path(),out)}else{assert!(e.file_type().unwrap().is_file());out.push(e.path());}}}
fn main(){
 let analysis=read(format!("{R}/analysis.json"));assert_eq!(analysis["pins_verified"],195);assert_eq!(analysis["accepted_source_files_verified"],176);assert_eq!(analysis["new_runtime_candidate"],false);
 for(p,v)in read(format!("{R}/pins.json")).as_object().unwrap(){assert_eq!(h(fs::read(p).unwrap()),v.as_str().unwrap());}
 let step=Path::new(R).join("workflow/source-mechanism-audit");let exit=read(step.join("exit.json"));assert_eq!(exit["success"],true);assert_eq!(exit["returncode"],0);
 for stream in["stdout","stderr"]{assert_eq!(h(fs::read(step.join(format!("{stream}.log"))).unwrap()),exit[format!("{stream}_sha256")].as_str().unwrap());}
 let cmd=read(step.join("command.json"));assert_eq!(cmd["argv"][4],format!("{R}/audit"));assert_eq!(cmd["argv"][5],"analyze");assert_eq!(read(format!("{R}/workflow/result.json"))["status"],"complete");
 let a=Path::new(P).join("artifacts");fs::create_dir_all(&a).unwrap();
 for f in["audit.rs","publish.rs","analysis.json","design.md","reading.md","pins.json","plan.json","source-pins.sha256"]{let from=Path::new(R).join(f);let to=a.join(f);let b=fs::read(&from).unwrap();fs::write(&to,&b).unwrap();assert_eq!(b,fs::read(to).unwrap());}
 let mut files=vec![];walk(&Path::new(R).join("workflow"),&mut files);files.sort();let mut receipts=serde_json::Map::new();
 for path in files{let rel=path.strip_prefix(R).unwrap().to_string_lossy().to_string();let b=fs::read(&path).unwrap();receipts.insert(rel,json!({"sha256":h(&b),"bytes":b.len(),"utf8":String::from_utf8(b).unwrap()}));}
 fs::write(a.join("receipts.json"),serde_json::to_vec_pretty(&receipts).unwrap()).unwrap();
 let mut source_hashes=serde_json::Map::new();for f in["msm_impl.hpp","msm.hpp","msm.cpp","pool.hpp"]{let b=fs::read(format!("{R}/{f}")).unwrap();source_hashes.insert(f.into(),json!({"sha256":h(&b),"bytes":b.len(),"url":format!("https://github.com/helius-labs/aeglos/blob/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu/{f}")}));}
 let p=json!({"scope":"source audit only; no native benchmark, new test, independent semantic reviewer or performance qualification","source_hashes":source_hashes,"audit_binary_sha256":h(fs::read(format!("{R}/audit")).unwrap()),"xtask_binary_sha256":h(fs::read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask").unwrap()),"receipt_members":receipts.len(),"paper_algorithm4_image_sha256":h(fs::read(format!("{R}/noel-algorithm4-full.png")).unwrap()),"external_source_and_images_included":false,"accepted_production_changed":false,"new_proofs":0});
 fs::write(a.join("publication.json"),serde_json::to_vec_pretty(&p).unwrap()).unwrap();
 let mut all=vec![];walk(Path::new(P),&mut all);all.retain(|p|p.file_name().unwrap()!="SHA256SUMS");all.sort();let mut sums=String::new();for path in &all{let rel=path.strip_prefix(P).unwrap().to_string_lossy();sums.push_str(&format!("{}  {rel}\n",h(fs::read(path).unwrap())));}
 fs::write(Path::new(P).join("SHA256SUMS"),sums).unwrap();
 println!("Published {} files plus SHA256SUMS; {} exact UTF8 receipts; 195 pins still match; source/images/binaries excluded",all.len(),receipts.len());
}

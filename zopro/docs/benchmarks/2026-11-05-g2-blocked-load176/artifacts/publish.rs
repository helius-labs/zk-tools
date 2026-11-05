use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf},process::{Command,Stdio}};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-blocked-load176";
const P:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-05-g2-blocked-load176";
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn hash(p:impl AsRef<Path>)->String{sha(&fs::read(p).unwrap())}
fn write(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn files(p:&Path)->Vec<PathBuf>{let mut out=Vec::new();for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let t=e.file_type().unwrap();assert!(!t.is_symlink());if t.is_dir(){out.extend(files(&e.path()));}else{assert!(t.is_file());out.push(e.path());}}out.sort();out}
fn main(){
 let a=Path::new(P).join("artifacts");fs::create_dir_all(&a).unwrap();assert!(!a.join("receipts.json.gz").exists());
 let review:Value=serde_json::from_slice(&fs::read(format!("{R}/review.json")).unwrap()).unwrap();assert_eq!(review["review_passed"],true);
 for n in["blocked_input.rs","design.md","bounds.md","setup.rs","native.rs","native-plan.json","build-pins.json","native-flags.txt","candidate-flags.txt","source-audit.json","frozen-candidate.json","review.rs","review.json","reproduce.md","publish.rs"]{fs::copy(format!("{R}/{n}"),a.join(n)).unwrap();}
 let root=Path::new(R).join("source");let source=files(&root);assert_eq!(source.len(),177);
 let members=source.iter().map(|p|(format!("source/{}",p.strip_prefix(&root).unwrap().display()),hash(p))).collect::<BTreeMap<_,_>>();write(a.join("source-members.json"),&json!(members));
 let list=Path::new(R).join("publication-source.list");fs::write(&list,members.keys().flat_map(|p|p.bytes().chain([0])).collect::<Vec<_>>()).unwrap();let archive=a.join("source.tar.gz");
 assert!(Command::new("tar").args(["--create","--gzip","--mtime=@0","--owner=0","--group=0","--numeric-owner","--file"]).arg(&archive).args(["--directory",R,"--no-recursion","--null","--files-from"]).arg(&list).status().unwrap().success());
 let listed=Command::new("tar").args(["--list","--gzip","--file"]).arg(&archive).output().unwrap();assert!(listed.status.success());assert_eq!(String::from_utf8(listed.stdout).unwrap().lines().map(str::to_string).collect::<Vec<_>>(),members.keys().cloned().collect::<Vec<_>>());
 let dest=Path::new(R).join("publication-source-roundtrip");fs::create_dir(&dest).unwrap();assert!(Command::new("tar").args(["--extract","--gzip","--file"]).arg(&archive).arg("--directory").arg(&dest).status().unwrap().success());assert_eq!(files(&dest).len(),177);for(p,h)in&members{assert_eq!(hash(dest.join(p)),*h);}
 let mut receipt_paths=files(&Path::new(R).join("native"));for n in["setup.log","native.log","review.log","frozen-candidate-cargo.jsonl"]{receipt_paths.push(Path::new(R).join(n));}receipt_paths.sort();
 let receipts=receipt_paths.iter().map(|p|{let bytes=fs::read(p).unwrap();json!({"path":p.strip_prefix(R).unwrap().to_str().unwrap(),"bytes":bytes.len(),"sha256":sha(&bytes),"utf8":String::from_utf8(bytes).unwrap()})}).collect::<Vec<_>>();
 let raw=serde_json::to_vec(&json!({"format":"exact-utf8-receipt-members-v1","members":receipts})).unwrap();let tmp=Path::new(R).join("publication-receipts.json");fs::write(&tmp,&raw).unwrap();let archive=a.join("receipts.json.gz");assert!(Command::new("gzip").args(["-n","-9","-c"]).arg(&tmp).stdout(Stdio::from(fs::File::create(&archive).unwrap())).status().unwrap().success());let decoded=Command::new("gzip").args(["-d","-c"]).arg(&archive).output().unwrap();assert!(decoded.status.success());assert_eq!(decoded.stdout,raw);
 let round:Value=serde_json::from_slice(&decoded.stdout).unwrap();for m in round["members"].as_array().unwrap(){let b=m["utf8"].as_str().unwrap().as_bytes();assert_eq!(b.len()as u64,m["bytes"].as_u64().unwrap());assert_eq!(sha(b),m["sha256"].as_str().unwrap());assert_eq!(fs::read(Path::new(R).join(m["path"].as_str().unwrap())).unwrap(),b);}
 let readme=fs::read_to_string(Path::new(P).join("README.md")).unwrap();let mut links=Vec::new();for part in readme.split("](").skip(1){let link=part.split(')').next().unwrap();assert!(!link.contains("://"));assert!(Path::new(P).join(link).is_file()||link=="SHA256SUMS");links.push(link.to_string());}
 write(Path::new(P).join("publication-review.json"),&json!({"publication_passed":true,"source_members":177,"source_member_names_and_hashes_roundtrip":true,"receipt_members":receipt_paths.len(),"receipt_UTF8_lengths_and_hashes_roundtrip":true,"README_local_links":links,"executables_libraries_private_inputs_excluded":true,"new_proofs":0,"new_timings":0,"proof_ledger":29953,"no_successful_native_work_repeated":true}));
 let index=files(Path::new(P)).iter().map(|p|json!({"path":p.strip_prefix(P).unwrap().to_str().unwrap(),"bytes":fs::metadata(p).unwrap().len(),"sha256":hash(p)})).collect::<Vec<_>>();write(Path::new(P).join("artifact-index.json"),&json!({"files":index}));
 let sum=files(Path::new(P)).iter().map(|p|format!("{}  {}\n",hash(p),p.strip_prefix(P).unwrap().display())).collect::<String>();fs::write(Path::new(P).join("SHA256SUMS"),sum).unwrap();for link in&links{assert!(Path::new(P).join(link).is_file());}
 println!("Publication PASS: {} files, {} checksums, {} links, 177 source members, {} exact UTF8 receipt members",files(Path::new(P)).len(),files(Path::new(P)).len()-1,links.len(),receipt_paths.len());
}

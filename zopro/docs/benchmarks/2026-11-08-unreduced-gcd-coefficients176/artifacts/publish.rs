use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path,process::Command};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/unreduced-gcd-coefficients176";
const DEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-08-unreduced-gcd-coefficients176";
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn gather(base:&Path,dir:&Path,out:&mut Vec<String>){for e in fs::read_dir(dir).unwrap(){let p=e.unwrap().path();if p.is_dir(){gather(base,&p,out)}else{out.push(p.strip_prefix(base).unwrap().to_str().unwrap().into());}}}
fn main(){
 assert!(!Path::new(&format!("{ROOT}/publication.json")).exists());
 let review:Value=serde_json::from_slice(&fs::read(format!("{ROOT}/review.json")).unwrap()).unwrap();assert_eq!(review["review_passed"],true);
 let mut names=vec!["design.md","model.rs","accepted-matrix.rs.inc","prepare.rs","model-pins.json","model-plan.json","model.json","review.rs","review.json","publish.rs"].into_iter().map(String::from).collect::<Vec<_>>();
 gather(Path::new(ROOT),Path::new(&format!("{ROOT}/model-runs")),&mut names);names.sort();names.dedup();
 let mut evidence=BTreeMap::<String,Value>::new();let mut index=BTreeMap::new();
 for n in names{let bytes=fs::read(format!("{ROOT}/{n}")).unwrap();let utf8=String::from_utf8(bytes.clone()).unwrap();let h=sha(&bytes);evidence.insert(n.clone(),json!({"utf8":utf8,"bytes":bytes.len(),"sha256":h}));index.insert(n,json!({"bytes":bytes.len(),"sha256":h}));}
 let raw=serde_json::to_vec(&evidence).unwrap();fs::write(format!("{ROOT}/evidence.json"),&raw).unwrap();
 let gzip=Command::new("gzip").args(["-n","-c",&format!("{ROOT}/evidence.json")]).output().unwrap();assert!(gzip.status.success());
 fs::write(format!("{ROOT}/evidence.json.gz"),&gzip.stdout).unwrap();
 let plain=Command::new("gzip").args(["-d","-c",&format!("{ROOT}/evidence.json.gz")]).output().unwrap();assert!(plain.status.success());assert_eq!(plain.stdout,raw);
 let decoded:BTreeMap<String,Value>=serde_json::from_slice(&plain.stdout).unwrap();for(n,v)in decoded{let bytes=v["utf8"].as_str().unwrap().as_bytes();assert_eq!(sha(bytes),v["sha256"]);assert_eq!(bytes.len(),v["bytes"].as_u64().unwrap() as usize);assert_eq!(sha(&fs::read(format!("{ROOT}/{n}")).unwrap()),v["sha256"]);}
 fs::create_dir_all(format!("{DEST}/artifacts")).unwrap();
 for n in["design.md","model.rs","accepted-matrix.rs.inc","review.rs","review.json","model-plan.json","model-pins.json","publish.rs"] {fs::copy(format!("{ROOT}/{n}"),format!("{DEST}/artifacts/{n}")).unwrap();}
 fs::write(format!("{DEST}/artifacts/evidence.json.gz"),&gzip.stdout).unwrap();
 fs::write(format!("{DEST}/artifacts/archive-index.json"),serde_json::to_vec_pretty(&index).unwrap()).unwrap();
 let publication=json!({"publication_passed":true,"UTF8_members":evidence.len(),"gzip_bytes":gzip.stdout.len(),"all_member_hashes_and_roundtrip_exact":true,"new_proofs":0,"canonical_unchanged":35181,"selected_native_qualification":true,"native_qualified":false,"speedup_measured":false,"goal_complete":false});
 fs::write(format!("{ROOT}/publication.json"),serde_json::to_vec_pretty(&publication).unwrap()).unwrap();fs::write(format!("{DEST}/artifacts/publication.json"),serde_json::to_vec_pretty(&publication).unwrap()).unwrap();
 let mut public=Vec::new();gather(Path::new(DEST),Path::new(DEST),&mut public);public.sort();let mut sums=String::new();for n in public{assert_ne!(n,"SHA256SUMS");sums.push_str(&format!("{}  {n}\n",sha(&fs::read(format!("{DEST}/{n}")).unwrap())));}
 fs::write(format!("{DEST}/SHA256SUMS"),sums).unwrap();println!("Publication PASS: {} UTF8 members/{} compressed bytes; exact roundtrip/hashes, model only",evidence.len(),gzip.stdout.len());
}

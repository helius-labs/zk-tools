use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-tree-store176";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-tree-store176";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn files(root:&Path,p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let t=e.file_type().unwrap();assert!(!t.is_symlink());if t.is_dir(){files(root,&e.path(),out)}else{assert!(t.is_file());out.push(e.path().strip_prefix(root).unwrap().to_path_buf());}}}
fn main(){
 let r=Path::new(R);let d=Path::new(D);let review=r.join("archive-review");assert!(!review.exists());fs::create_dir(&review).unwrap();let mut counts=vec![];
 for(name,original)in[("private-source",r.join("source")),("native-receipts",r.to_path_buf()),("caller-receipts",r.to_path_buf()),("codegen",r.to_path_buf()),("workflow",r.to_path_buf())]{
  let dest=review.join(name);fs::create_dir(&dest).unwrap();assert!(Command::new("tar").arg("-xzf").arg(d.join(format!("artifacts/{name}.tar.gz"))).arg("-C").arg(&dest).status().unwrap().success());
  let mut entries=vec![];files(&dest,&dest,&mut entries);for p in &entries{assert_eq!(hash(dest.join(p)),hash(original.join(p)));assert!(!p.to_string_lossy().ends_with("-sdk"));}
  if name=="private-source"{assert_eq!(entries.len(),178);let a:Value=serde_json::from_slice(&fs::read(d.join("artifacts/source-audit.json")).unwrap()).unwrap();for(rel,h)in a["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(dest.join(rel)),h.as_str().unwrap());}}
  if name=="caller-receipts"{assert_eq!(entries.iter().filter(|p|p.file_name().unwrap()=="stdout.log").count(),180);}
  counts.push(json!({"archive":name,"regular_files_byte_exact":entries.len()}));
 }
 let text=fs::read_to_string(d.join("README.md")).unwrap();let mut links=0;let mut rest=text.as_str();while let Some(i)=rest.find("]("){rest=&rest[i+2..];let end=rest.find(')').unwrap();let link=&rest[..end];if !link.starts_with("https://")&&!link.starts_with('#'){assert!(d.join(link).exists(),"missing link {link}");links+=1;}rest=&rest[end+1..];}
 let a:Value=serde_json::from_slice(&fs::read(d.join("artifacts/caller-analysis.json")).unwrap()).unwrap();assert_eq!(a["caller_gate_passed"],false);
 let result=json!({"passed":true,"archives":counts,"readme_local_links_present":links,"new_measurements":false,"scope":"Extracted every archive; all members compared byte for byte to retained originals; source manifest178 exact;180stdout receipts retained;README links resolve."});
 fs::write(d.join("artifacts/archive-review.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("Archive closure PASS;five archives byte exact;178source files/180process outputs;{links}local links");
}

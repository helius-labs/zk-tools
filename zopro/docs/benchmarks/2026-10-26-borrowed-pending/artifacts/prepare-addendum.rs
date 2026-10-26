use std::{fs,path::Path};use serde_json::{Value,json};use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-borrowed-pending";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){let r=Path::new(R);let mut config=read(r.join("perf-before-preflight.json"));
 for rel in ["source-review-addendum.md","prepare-addendum.rs","prepare-addendum"]{
  let p=r.join(rel);config["pins"][p.display().to_string()]=json!(hash(&p));
 }
 let pins=&config["pins"];
 fs::write(r.join("preflight-pins.json"),serde_json::to_vec_pretty(pins).unwrap()).unwrap();
 fs::write(r.join("perf-before-preflight.json"),serde_json::to_vec_pretty(&config).unwrap()).unwrap();
 println!("Pre-capture scope addendum and helper pinned");
}

use std::{fs,path::Path};
use sha2::{Digest,Sha256};
fn main(){let root=Path::new("/tmp/zopro-stage-costs");let p=root.join("source-pins.json");let x:serde_json::Value=serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();let mut pins=x["files_sha256"].as_object().unwrap().clone();for rel in ["prepare-build.rs","repair-pins.rs"]{let p=root.join(rel);pins.insert(p.display().to_string(),format!("{:x}",Sha256::digest(fs::read(p).unwrap())).into());}fs::write(p,serde_json::to_vec_pretty(&pins).unwrap()).unwrap();}

use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/bucket-round178";
const P:&str="/home/ubuntu/src/zopro-perf-work/borrowed-pair178";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn copy(a:&Path,b:&Path){fs::create_dir_all(b).unwrap();for e in fs::read_dir(a).unwrap(){let e=e.unwrap();assert!(!e.file_type().unwrap().is_symlink());if e.file_type().unwrap().is_dir(){copy(&e.path(),&b.join(e.file_name()));}else{fs::copy(e.path(),b.join(e.file_name())).unwrap();}}}
fn files(p:&Path,out:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();if e.file_type().unwrap().is_dir(){files(&e.path(),out)}else{out.insert(e.path().display().to_string(),hash(e.path()));}}}
fn main(){let r=Path::new(R);assert!(!r.join("source").exists());let parent=Path::new(P);for(p,h)in read(parent.join("build-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
copy(&parent.join("source"),&r.join("source"));copy(&parent.join("driver"),&r.join("driver"));for f in["edits.json","production-bindings.json"]{fs::copy(parent.join(f),r.join(f)).unwrap();}
let module=r.join("source/crates/zopro-core/src/msm_affine/g2_phi");let mut s=fs::read_to_string(module.join("staged_tree.rs")).unwrap();let a=s.find("fn stage<'a>(").unwrap();let b=s.find("struct Route {").unwrap();s.replace_range(a..b,&format!("{}\n\n",fs::read_to_string(r.join("round_stage.rs")).unwrap()));s.push_str("#[cfg(test)]\nmod tests;\n");fs::write(module.join("direct_tree.rs"),s).unwrap();
let old="/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176/native/source-isolated/crates/zopro-core/src/msm_affine/g2_phi/borrowed_tree/tests.rs";let s=fs::read_to_string(old).unwrap().replace(", 12, false)",", 12, 0)");fs::write(module.join("direct_tree/tests.rs"),s).unwrap();
let mut plan=read(parent.join("build-plan.json"));plan=serde_json::from_str(&serde_json::to_string(&plan).unwrap().replace(P,R)).unwrap();
let mut pins=BTreeMap::new();for d in["source","driver"]{files(&r.join(d),&mut pins);}for f in["prepare.rs","round_stage.rs","protocol.md","manage.rs","review.rs","edits.json","production-bindings.json"]{pins.insert(r.join(f).display().to_string(),hash(r.join(f)));}pins.insert(old.into(),hash(old));
fs::write(r.join("build-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(r.join("build-plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();println!("{}",json!({"prepared":true,"mode2":"destination rounds","inherited_source":P,"pins":pins.len()}));}

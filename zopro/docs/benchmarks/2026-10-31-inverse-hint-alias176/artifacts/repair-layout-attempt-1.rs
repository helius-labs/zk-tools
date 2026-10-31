use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176";
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){
 assert_eq!(read(&format!("{R}/qualification/core/exit.json"))["returncode"],101);
 let mut pins=BTreeMap::new();
 let old=read(&format!("{R}/source-pins.json"));
 for(p,h)in old.as_object().unwrap(){
  assert_eq!(hash(p),h.as_str().unwrap());let mut text=fs::read_to_string(p).unwrap();
  if p.ends_with("/solver/inverse_aliases.rs"){
   assert_eq!(text.matches("size_of::<Op>() != 76").count(),1);
   text=text.replace("size_of::<Op>() != 76","size_of::<Op>() != size_of::<OriginalOp>()");
   text=text.replace("assert_eq!((size_of::<Op>(),core::mem::align_of::<Op>()),(76,4));","assert_eq!((size_of::<Op>(),core::mem::align_of::<Op>()),(size_of::<OriginalOp>(),core::mem::align_of::<OriginalOp>()));");
   text=text.replace("use core::mem::size_of;","use core::mem::size_of;\n// Exact accepted field types with the original two-kind hint layout.\n#[allow(dead_code)]enum OriginalHintKind{InvZero,Bits}\n#[allow(dead_code)]struct OriginalHint{kind:OriginalHintKind,input:(u32,u32),outputs:(u32,u32)}\n#[allow(dead_code)]enum OriginalOp{Constraint(Step),Hint(OriginalHint)}");
  }
  let q=p.replacen("/source/","/source-v2/",1);assert!(!Path::new(&q).exists());fs::create_dir_all(Path::new(&q).parent().unwrap()).unwrap();fs::write(&q,text).unwrap();pins.insert(q.clone(),hash(&q));
 }
 fs::write(format!("{R}/source-v2-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 for f in ["repair-layout.rs","repair-layout","design.md","capture-scope.md","setup.rs","aliases.rs.in","qualification/core/exit.json","qualification/core/stdout.log","qualification/core/stderr.log"]{let p=format!("{R}/{f}");pins.insert(p.clone(),hash(&p));}
 let tool="/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";pins.insert(tool.into(),hash(tool));
 fs::write(format!("{R}/qualification-v2-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 let mut plan=read(&format!("{R}/qualification-plan.json"));plan["source_pins"]=json!(format!("{R}/qualification-v2-pins.json"));
 for s in plan["steps"].as_array_mut().unwrap(){for arg in s["argv"].as_array_mut().unwrap(){let text=arg.as_str().unwrap().replace("/source/Cargo.toml","/source-v2/Cargo.toml");*arg=json!(text);}}
 fs::write(format!("{R}/qualification-v2-plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
 println!("Preserved failed177-file snapshot; repaired2layout guards using exact original-layout types; new177-file snapshot and qualification pinned");
}

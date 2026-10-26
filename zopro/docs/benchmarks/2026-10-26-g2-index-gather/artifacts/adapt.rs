use std::{fs,path::Path};
use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-g2-index-gather";
fn main(){
 let old="/tmp/zopro-g2-prefix-tree";
 let frozen=format!("{:x}",Sha256::digest(fs::read(format!("{old}/native/on-sdk")).unwrap()));
 for file in ["run.rs","closeout.rs"]{
  let mut s=fs::read_to_string(format!("{old}/{file}")).unwrap().replace(old,R).replace("2026-10-26-g2-prefix-tree","2026-10-26-g2-index-gather").replace("117","122").replace("1881","3561").replace("5958","6562");
  if file=="run.rs"{
   s=s.replace("/tmp/zopro-g2-level-tree/native/on-sdk",&format!("{old}/native/on-sdk")).replace("1f273ced307d9946ede09bd7622c654e5c91951d6c53d3190ade1ae23bde4a67",&frozen);
   s=s.replace("vec![\"frozen\",\"stream\",\"tree\",\"prefix\"]","vec![\"frozen\",\"stream\",\"prefix\",\"gather\"]").replace("[\"frozen\",\"stream\",\"tree\",\"prefix\"]","[\"frozen\",\"stream\",\"prefix\",\"gather\"]").replace("[\"stream\",\"tree\",\"prefix\"]","[\"stream\",\"prefix\",\"gather\"]");
   s=s.replace("workers.to_string(),\"7\"","workers.to_string(),\"17\"").replace("\"rounds\":7","\"rounds\":17");
   s=s.replace("(\"prefix_stream\",\"prefix\",\"stream\"),(\"prefix_frozen\",\"prefix\",\"frozen\"),(\"prefix_tree\",\"prefix\",\"tree\")","(\"gather_stream\",\"gather\",\"stream\"),(\"gather_frozen\",\"gather\",\"frozen\"),(\"gather_prefix\",\"gather\",\"prefix\")");
   s=s.replace("if workers>=16&&[\"prefix_stream\",\"prefix_frozen\"].contains(&name)","if workers==32&&name.starts_with(\"gather_\")");
   s=s.replace("if shape==\"2x2\"&&ratio>1.02","if shape==\"2x2\"&&name!=\"gather_prefix\"&&ratio>1.02");
   s=s.replace("complete prefix-tree component study","complete index-gather component study");
  }
  fs::write(Path::new(R).join(file),s).unwrap();
 }
}

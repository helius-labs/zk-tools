use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::Path};
fn hash(p:&Path)->String {let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn main(){
 let root=Path::new("/tmp/zopro-bounded-steal-search/aws");
 let read=|p:&Path|serde_json::from_slice::<Value>(&fs::read(p).unwrap()).unwrap();
 let inventory=read(&root.join("local-review/proof-inventory.json"));
 let mut pins=BTreeMap::<String,String>::new();let mut steps=Vec::new();
 for (name,digest) in inventory.as_object().unwrap(){
   let path=root.join("remote-results/output").join(name);
   assert_eq!(hash(&path),digest.as_str().unwrap());pins.insert(path.display().to_string(),digest.as_str().unwrap().into());
   if path.extension().is_some_and(|e|e=="proof"){
     let directory=path.parent().unwrap().parent().unwrap();
     steps.push(json!({"kind":"command","name":format!("verify-{:03}",steps.len()),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",directory.to_str().unwrap(),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":180}));
   }
 }
 let old=read(Path::new("/tmp/zopro-phase-cpu/aws/local-verify-pins.json"));
 for path in ["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys/transfer_confidential_1_2.zopro","/tmp/zopro-keys/merge_54_1.zopro","/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"]{
   let digest=hash(Path::new(path));assert_eq!(digest,old[path].as_str().unwrap());pins.insert(path.into(),digest);
 }
 for p in [root.join("make-local-verify.rs"),root.join("review.rs"),root.join("local-review/result.json"),root.join("local-review/proof-inventory.json"),root.join("resource-review-result.json"),Path::new("/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask").to_owned()]{pins.insert(p.display().to_string(),hash(&p));}
 assert_eq!(steps.len(),151);assert_eq!(pins.len(),312);
 let save=|name:&str,v:Value|fs::write(root.join(name),format!("{}\n",serde_json::to_string_pretty(&v).unwrap())).unwrap();
 save("local-verify-pins.json",json!(pins));
 save("local-verify-plan.json",json!({"version":1,"source_pins":root.join("local-verify-pins.json"),"environment":{"RAYON_NUM_THREADS":"4","GOMAXPROCS":"4"},"steps":steps}));
 println!("151 independent local verification commands, 312 pins; no additional proving");
}

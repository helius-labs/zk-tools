mod trace;
use std::{collections::BTreeMap,fs,path::Path};
use serde_json::{Value,json};
use sha2::{Sha256,Digest};
fn output(d:&Path)->String{
 let bytes=fs::read(d.join("stdout.log")).unwrap();let e:Value=serde_json::from_slice(&fs::read(d.join("exit.json")).unwrap()).unwrap();
 assert_eq!(e["success"],true);assert_eq!(e["stdout_sha256"],format!("{:x}",Sha256::digest(&bytes)));String::from_utf8(bytes).unwrap()
}
fn main(){
 let root=Path::new("../g2-interior256/remote/output");let mut blocks=vec![];
 for w in [32,96]{
  for b in 0..3{
   let job=format!("main-{w}-trace-b{b}");let sdk=output(&root.join(format!("b{b}/{job}")));
   let s:Value=serde_json::from_str(sdk.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();
   let workers:BTreeMap<_,_>=s["placement_before"]["worker_rows"].as_array().unwrap().iter().map(|v|(v["tid"].as_u64().unwrap(),v["index"].as_u64().unwrap())).collect();assert_eq!(workers.len(),w);
   let vals=trace::analyze(&output(&root.join(format!("b{b}-review/decode-{job}"))),&format!("zp256b{b}:"),151,&workers);
   let data:BTreeMap<_,_>=vals.into_iter().filter(|(k,_)|k.starts_with("w0")||k.starts_with("w10")||k=="last_window_critical_half_1").map(|(k,v)|(k,v.iter().sum::<f64>()/v.len() as f64)).collect();
   blocks.push(json!({"workers":w,"block":b,"means":data}));
  }
 }
 println!("{}",serde_json::to_string_pretty(&blocks).unwrap());
}

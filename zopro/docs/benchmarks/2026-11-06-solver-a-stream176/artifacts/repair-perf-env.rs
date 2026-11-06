use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/solver-a-stream176";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(format!("{R}/{p}")).unwrap()).unwrap()}
fn hash(p:&Path)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn walk(p:&Path,pins:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let p=e.unwrap().path();if p.is_dir(){walk(&p,pins)}else{pins.insert(p.display().to_string(),hash(&p));}}}
fn main(){
 assert!(!Path::new(&format!("{R}/perf-fixed-plan.json")).exists());
 let plan=read("capture-plan.json");let all=plan["steps"].as_array().unwrap();
 for s in &all[..30]{assert!(s["name"].as_str().unwrap().starts_with("latency-"));let e=read(&format!("capture/{}/exit.json",s["name"].as_str().unwrap()));assert_eq!(e["success"],true);}
 assert_eq!(all[30]["name"],"stat-w16-off");assert_eq!(read("capture/stat-w16-off/exit.json")["returncode"],101);
 let err=fs::read_to_string(format!("{R}/capture/stat-w16-off/stderr.log")).unwrap();assert!(err.contains("eligible prepared primary"));
 assert_eq!(fs::metadata(format!("{R}/capture/stat-w16-off/stdout.log")).unwrap().len(),0);
 let env=read("environment.json");let mut steps=all[30..].to_vec();
 for s in &mut steps{let old=s["argv"].as_array().unwrap();if old[0]=="sudo"&&old[2]=="env"{let mut argv=old[..3].to_vec();for(k,v)in env.as_object().unwrap(){argv.push(json!(format!("{k}={}",v.as_str().unwrap())));}argv.extend_from_slice(&old[3..]);s["argv"]=json!(argv);}}
 let mut pins=read("capture-pins.json").as_object().unwrap().iter().map(|(p,h)|(p.clone(),h.as_str().unwrap().to_owned())).collect::<BTreeMap<_,_>>();
 walk(Path::new(&format!("{R}/capture")),&mut pins);
 for f in["capture-plan.json","capture-pins.json","repair-perf-env.rs"]{let p=format!("{R}/{f}");pins.insert(p.clone(),hash(Path::new(&p)));}
 fs::write(format!("{R}/perf-fixed-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 fs::write(format!("{R}/perf-fixed-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{R}/perf-fixed-pins.json"),"environment":env,"steps":steps})).unwrap()).unwrap();
 println!("Preserved30successful latency callers; failed perf attempt stopped before preparation/counters. Forward explicit frozen runtime environment through sudo env; only remaining28commands run.");
}

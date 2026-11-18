use std::{collections::BTreeMap, fs, path::Path};
use serde_json::{json, Value};
use sha2::{Sha256,Digest};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn median(mut x:Vec<f64>)->f64{assert_eq!(x.len(),3);x.sort_by(f64::total_cmp);x[1]}
fn main(){
 assert_eq!(read("timing-run/result.json")["status"],"complete");
 let plan=read("timing-plan.json");let mut data=BTreeMap::new();let mut rows=vec![];
 for step in plan["steps"].as_array().unwrap(){
  let dir=Path::new("timing-run").join(step["name"].as_str().unwrap());let e=read(dir.join("exit.json"));assert_eq!(e["success"],true);
  let raw=fs::read(dir.join("stdout.log")).unwrap();assert_eq!(format!("{:x}",Sha256::digest(&raw)),e["stdout_sha256"]);
  let raw=std::str::from_utf8(&raw).unwrap();let mut screens=raw.lines().filter_map(|l|l.split_once("SCREEN ").map(|(_, record)| record));
  let v:Value=serde_json::from_str(screens.next().unwrap()).unwrap();assert!(screens.next().is_none());assert!(raw.contains("1 passed; 0 failed"));
  let shape=v["shape"].as_str().unwrap();let w=v["workers"].as_u64().unwrap();let a=v["arm"].as_u64().unwrap();let name=step["name"].as_str().unwrap();let b=name.split("-b").last().unwrap().parse::<u64>().unwrap();
  assert_eq!(step["environment"]["SHIFTED_ARM"],a.to_string());assert_eq!(step["environment"]["SHIFTED_WORKERS"],w.to_string());assert_eq!(step["environment"]["SHIFTED_SHAPE"],shape);
  assert_eq!(v["route_calls"],21);let times=v["times_ns"].as_array().unwrap();assert_eq!(times.len(),17);let mean=times.iter().map(|t|t.as_u64().unwrap()as f64/1e6).sum::<f64>()/17.;
  let rss=v["peak_kib"].as_f64().unwrap()/1024.;let cpu=v["cpu_us"].as_f64().unwrap()/17000.;
  assert!(data.insert((shape.to_owned(),w,a,b),(mean,rss,cpu)).is_none());rows.push(json!({"name":name,"mean_ms":mean,"max_RSS_MiB":rss,"cpu_ms":cpu,"record":v}));
 }
 assert_eq!(data.len(),36);
 let mut cells=vec![];let mut passes=true;
 for shape in ["1x2","2x2"]{for w in [16,32]{
  let get=|a,b|data[&(shape.to_owned(),w,a,b)];
  let mut arms=vec![];for arm in 0..3{
   let means=(0..3).map(|b|get(arm,b).0).collect::<Vec<_>>();
   arms.push(json!({"arm":arm,"mean_ms":median(means.clone()),"blocks":means,"cpu_ms":median((0..3).map(|b|get(arm,b).2).collect()),"peak_RSS_MiB":(0..3).map(|b|get(arm,b).1).fold(0f64,f64::max)}));
  }
  let ratio=|ctrl|median((0..3).map(|b|get(2,b).0/get(ctrl,b).0).collect());if w==32 && ratio(1)>=1.{passes=false;}
  cells.push(json!({"shape":shape,"workers":w,"arms":arms,"joined_over_tree":ratio(1),"joined_over_stream":ratio(0)}));
 }}
 fs::write("assessment.json",serde_json::to_vec_pretty(&json!({"progress_to_high_core_screen":passes,"cells":cells,"rows":rows})).unwrap()).unwrap();
}

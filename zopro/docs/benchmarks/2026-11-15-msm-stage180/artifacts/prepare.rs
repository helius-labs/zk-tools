use serde_json::{Value,json};use sha2::{Digest,Sha256};use std::{fs,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/msm-stage180";
fn save(n:&str,v:&Value){assert!(!std::path::Path::new(&format!("{R}/{n}")).exists());fs::write(format!("{R}/{n}"),serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){if std::env::args().nth(1).as_deref()==Some("check"){
 let s=fs::read_to_string(format!("{R}/qualification/unit-observer/stderr.log")).unwrap();let rows=s.lines().map(|s|serde_json::from_str::<Value>(s.strip_prefix("MSM_PHASES ").unwrap()).unwrap()).collect::<Vec<_>>();assert_eq!(rows.len(),16);
 let out=fs::read_to_string(format!("{R}/qualification/unit-observer/stdout.log")).unwrap();assert_eq!(out.lines().filter(|s|s.starts_with("TEST ")&&s.ends_with(" PASS")).count(),10);
 let mut valid=0;for r in &rows{assert_eq!(r["version"],1);if r["complete"]==true&&r["overflow"]==false&&r["malformed"]==false{valid+=1;let mut end=0;for v in r["rows"].as_array().unwrap(){let b=v["begin_ns"].as_u64().unwrap();let e=v["end_ns"].as_u64().unwrap();assert!(b>=end&&e>=b&&e<=r["total_ns"].as_u64().unwrap());end=e;let counts=v["job_items"].as_array().unwrap();if !counts.is_empty(){let sum=counts.iter().map(|x|x.as_u64().unwrap()).try_fold(0u64,|a,b|a.checked_add(b)).unwrap();assert_eq!(sum,v["items"].as_u64().unwrap());}}}}
 assert_eq!(valid,11);assert_eq!(rows.iter().filter(|r|r["overflow"]==true).count(),2);assert_eq!(rows.iter().filter(|r|r["malformed"]==true).count(),2);assert_eq!(rows.iter().filter(|r|r["complete"]==false).count(),1);
 save("observer-review.json",&json!({"passed":true,"unit_cases":10,"decoded_records":rows.len(),"valid_records":valid,"trace_bytes":rows[0]["trace_bytes"],"serialization_stack_bytes":32768,"SDK_calls":0,"new_exports":0}));println!("10 unit cases;16 decoded records;11 valid and5 explicitly invalid as expected");return;}
 let old:Value=serde_json::from_slice(&fs::read(format!("{R}/native-source-pins.json")).unwrap()).unwrap();let mut pins=BTreeMap::new();for p in old.as_object().unwrap().keys(){pins.insert(p.clone(),format!("{:x}",Sha256::digest(fs::read(p).unwrap())));}
 for n in ["design.md","source-edits.json","instrument.rs","observer-test.cpp","observer-test","prepare.rs","prepare"]{let p=format!("{R}/{n}");pins.insert(p.clone(),format!("{:x}",Sha256::digest(fs::read(p).unwrap())));}
 save("qualification-pins.json",&json!(pins));save("qualification-plan.json",&json!({"version":1,"source_pins":format!("{R}/qualification-pins.json"),"steps":[
 {"kind":"command","name":"unit-observer","argv":[format!("{R}/observer-test")],"timeout_seconds":60},
 {"kind":"command","name":"check-observer-records","argv":[format!("{R}/prepare"),"check"],"timeout_seconds":60},
 {"kind":"command","name":"native-archive","argv":["make","-C",format!("{R}/aeglos"),"-j8","cpu","CXX=g++",format!("CPU_BUILD_DIR={R}/native")],"timeout_seconds":1200}
 ]}));}

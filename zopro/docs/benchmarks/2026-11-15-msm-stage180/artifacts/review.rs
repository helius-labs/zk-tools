use serde_json::{Value,json};use std::{fs,collections::{BTreeSet,BTreeMap}};use sha2::{Digest,Sha256};
const R:&str="/home/ubuntu/src/zopro-perf-work/msm-stage180";
fn main(){
 let read=|p:&str|serde_json::from_slice::<Value>(&fs::read(p).unwrap()).unwrap();
 let run=format!("{R}/native-test-run/msm-oracles");let exit=read(&format!("{run}/exit.json"));assert_eq!(exit["success"],true);
 let stdout=fs::read_to_string(format!("{run}/stdout.log")).unwrap();assert!(stdout.ends_with("PASS\n"));
 let expected=["TestMSMMatchesGnark","TestMSMSortedThreads","TestMSMEmpty","TestMSMSharedScalars","TestMSMWithinSmallBudget","TestMSMRepeatedPlan","TestMSMWireViews"];
 for name in expected {assert!(stdout.lines().any(|s|s.starts_with(&format!("--- PASS: {name} ("))),"missing {name}");}
 assert!(!stdout.contains("--- SKIP:")&&!stdout.contains("--- FAIL:"));
 let stderr=fs::read_to_string(format!("{run}/stderr.log")).unwrap();let mut total=0;let mut phases=BTreeMap::<String,usize>::new();let mut workers=BTreeSet::new();let mut counts=BTreeSet::new();let mut max_rows=0;
 for line in stderr.lines(){let v:Value=serde_json::from_str(line.strip_prefix("MSM_PHASES ").unwrap()).unwrap();assert_eq!(v["version"],1);assert_eq!(v["complete"],true);assert_eq!(v["overflow"],false);assert_eq!(v["malformed"],false);assert_eq!(v["trace_bytes"],10808);
 let rows=v["rows"].as_array().unwrap();assert!(rows.len()<=64);max_rows=max_rows.max(rows.len());let names=rows.iter().map(|v|v["phase"].as_str().unwrap()).collect::<Vec<_>>();assert_eq!(&names[..6],&["begin","recode","count","place","scatter","accumulate"]);
 let mut at=6;while names[at]=="merge_plan" && names[at+1]=="merge"{at+=2;}
 assert_eq!(&names[at..],&["merge_plan","sums","finish"]);
 let mut end=0;for row in rows{let b=row["begin_ns"].as_u64().unwrap();let e=row["end_ns"].as_u64().unwrap();assert!(end<=b && b<=e && e<=v["total_ns"].as_u64().unwrap());end=e;
 let name=row["phase"].as_str().unwrap();*phases.entry(name.into()).or_default()+=1;
 let js=row["job_items"].as_array().unwrap();if ["recode","count","scatter","accumulate","merge","sums"].contains(&name){assert_eq!(js.len()as u64,v["jobs"].as_u64().unwrap());let sum=js.iter().map(|v|v.as_u64().unwrap()).try_fold(0u64,|a,b|a.checked_add(b)).unwrap();assert_eq!(sum,row["items"].as_u64().unwrap());}else{assert!(js.is_empty());}
 }
 workers.insert(v["workers"].as_u64().unwrap());counts.insert(v["jobs"].as_u64().unwrap());total+=1;
 }
 assert!(total>0);let patched=fs::read_to_string(format!("{R}/aeglos/cpu/msm_impl.hpp")).unwrap();let edits=read(&format!("{R}/source-edits.json"));let mut original=patched.clone();for e in edits["edits"].as_array().unwrap().iter().rev(){let n=e["new"].as_str().unwrap();assert_eq!(original.matches(n).count(),1);original=original.replacen(n,e["old"].as_str().unwrap(),1);}let hash=format!("{:x}",Sha256::digest(original.as_bytes()));assert_eq!(hash,edits["original_sha256"].as_str().unwrap());
 let out=json!({"passed":true,"MSM_test_families":expected,"subtests_passed":stdout.lines().filter(|s|s.trim_start().starts_with("--- PASS:")).count(),"phase_records":total,"max_rows":max_rows,"observed_workers":workers,"observed_jobs":counts,"phase_record_counts":phases,"source_recovery_exact":true,"trace_bytes":10808,"serialization_stack_bytes":32768,"phase_intervals_used_as_benchmark":false,"performance_qualified":false,"SDK_calls":0,"new_exports":0,"native_test_elapsed_ms":exit["elapsed_ms"]});
 let dest=format!("{R}/native-review.json");assert!(!std::path::Path::new(&dest).exists());fs::write(dest,serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("{out}");
}

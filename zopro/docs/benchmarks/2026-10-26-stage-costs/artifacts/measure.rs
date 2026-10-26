use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,collections::{BTreeMap,BTreeSet}};
const ROOT:&str="/tmp/zopro-stage-costs";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn add(p:impl AsRef<Path>,pins:&mut BTreeMap<String,String>){let p=p.as_ref();pins.insert(p.display().to_string(),hash(p));}
fn median(mut x:Vec<f64>)->f64{assert!(!x.is_empty());x.sort_by(f64::total_cmp);let n=x.len();if n%2==1{x[n/2]}else{(x[n/2-1]+x[n/2])/2.}}
fn prior()->BTreeSet<String>{
 let c=read("/tmp/zopro-ready-c7a/remote-results/output/measure/config.json");
 let mut p:BTreeSet<String>=serde_json::from_value(c["prior_proof_sha256"].clone()).unwrap();
 for(path,h)in read("/tmp/zopro-ready-c7a/proof-inventory.json").as_object().unwrap(){assert_eq!(hash(path),h.as_str().unwrap());if path.ends_with(".proof"){p.insert(h.as_str().unwrap().into());}}
 assert_eq!(p.len(),5216);p
}
fn cases()->Vec<Value>{read("/tmp/zopro-ready-arithmetic/perf-plan.json")["cases"].as_array().unwrap().iter().filter(|c|c["workers"]==32).cloned().collect()}
fn step(c:&Value,mode:&str,name:&str,rounds:usize,pins:&mut BTreeMap<String,String>)->Vec<Value>{
 let r=Path::new(ROOT);let artifact=read(r.join("native/on/artifacts.json"));
 let binary=if mode=="accepted"{"/tmp/zopro-grouping-module/native/candidate-sdk"}else{artifact["binary"].as_str().unwrap()};
 let proofroot=r.join("proofs").join(name);let stem=Path::new(c["image"].as_str().unwrap()).file_stem().unwrap();let d=proofroot.join(stem);
 let mut env=read("/tmp/zopro-ready-arithmetic/perf-plan.json")["environment"].clone();env["RAYON_NUM_THREADS"]=json!("32");
 if mode=="on"{env["ZOPRO_STAGE_OUTPUT"]=json!(d.join("stages.json"));}
 for p in [binary,c["image"].as_str().unwrap(),c["request"].as_str().unwrap(),"/tmp/zopro-gnark-verify/verify","/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"]{add(p,pins);}
 vec![json!({"kind":"command","name":name,"argv":["taskset","-c","0-31",binary,c["image"],c["request"],rounds.to_string(),d],"environment":env,"timeout_seconds":120}),
 json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",proofroot,"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120})]
}
fn make_plan(preflight:bool){let r=Path::new(ROOT);assert_eq!(read(r.join("native/result.json"))["status"],"complete");
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r.join("source-pins.json"))).unwrap();
 let artifact=read(r.join("native/on/artifacts.json"));for(p,h)in artifact["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}
 for name in ["measure.rs","measure","native-plan.json","native/result.json","native/on/artifacts.json","native/on-core/stdout.log","native/on-prover/stdout.log"]{add(r.join(name),&mut pins);}
 for(p,h)in read("/tmp/zopro-ready-arithmetic/perf-plan.json")["pins"].as_object().unwrap(){if p.contains("/registry/")||p.starts_with("/tmp/zopro-keys/"){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}}
 let mut steps=Vec::new();let cs=cases();let mut jobs=Vec::new();
 if preflight {for c in &cs{for mode in ["off","on"]{let name=format!("preflight-{}-{mode}",c["name"].as_str().unwrap());steps.extend(step(c,mode,&name,1,&mut pins));jobs.push(json!({"case":c,"mode":mode,"name":name,"rounds":1,"block":null}));}}}
 else{assert_eq!(read(r.join("preflight-analysis.json"))["status"],"passed");add(r.join("preflight-analysis.json"),&mut pins);let modes=["accepted","off","on"];
  for block in 0..3{let mut order:Vec<_>=(0..3).map(|i|modes[(i+block)%3]).collect();let mut cs=cs.clone();if block%2==1{order.reverse();cs.reverse();}for c in &cs{for mode in &order{let name=format!("b{block}-{}-{mode}",c["name"].as_str().unwrap());steps.extend(step(c,mode,&name,17,&mut pins));jobs.push(json!({"case":c,"mode":mode,"name":name,"rounds":17,"block":block}));}}}
 }
 let which=if preflight{"preflight"}else{"performance"};
 for(p,h)in &pins{assert_eq!(hash(p),*h,"{p}");}
 save(r.join(format!("{which}-pins.json")),&json!(pins));
 save(r.join(format!("{which}-jobs.json")),&json!(jobs));
 save(r.join(format!("{which}-plan.json")),&json!({"version":1,"source_pins":r.join(format!("{which}-pins.json")),"environment":{},"steps":steps}));
 let p=prior();save(r.join("prior-identities.json"),&json!(p));
 println!("Prepared{which}: {} SDK processes and verifiers; prior{}",jobs.len(),p.len());
}
fn inspect(v:&Value)->Value{
 let events=v["events"].as_array().unwrap();assert!(!events.is_empty()&&events.len()<=512);
 let origin=events.iter().map(|e|e["wall0"].as_u64().unwrap()).min().unwrap();
 let mut groups:BTreeMap<String,Vec<Value>>=BTreeMap::new();let mut tids:BTreeMap<u64,Vec<&Value>>=BTreeMap::new();
 for(i,e)in events.iter().enumerate(){assert_eq!(e["id"],i);let w0=e["wall0"].as_u64().unwrap();let w1=e["wall1"].as_u64().unwrap();let c0=e["cpu0"].as_u64().unwrap();let c1=e["cpu1"].as_u64().unwrap();assert!(w1>=w0&&c1>=c0);assert!(c1-c0<=w1-w0+2_000_000);let tid=e["tid"].as_u64().unwrap();assert!(tid>0);tids.entry(tid).or_default().push(e);
  groups.entry(format!("{}:n{}",e["label"].as_str().unwrap(),e["n"])).or_default().push(json!({"wall_ms":(w1-w0)as f64/1e6,"thread_cpu_ms":(c1-c0)as f64/1e6,"start_offset_ms":(w0-origin)as f64/1e6,"end_offset_ms":(w1-origin)as f64/1e6}));
 }
 // Per-thread RAII spans may nest or be disjoint, but never partially cross.
 for es in tids.values_mut(){es.sort_by_key(|e|e["wall0"].as_u64().unwrap());for(i,a)in es.iter().enumerate(){for b in &es[i+1..]{let b0=b["wall0"].as_u64().unwrap();let a1=a["wall1"].as_u64().unwrap();if b0<a1{assert!(b["wall1"].as_u64().unwrap()<=a1,"crossing thread spans");}}}}
 let mut out=BTreeMap::new();for(k,es)in groups{let mut metrics=BTreeMap::new();for field in ["wall_ms","thread_cpu_ms"]{metrics.insert(field,es.iter().map(|e|e[field].as_f64().unwrap()).sum::<f64>());}metrics.insert("calls",es.len()as f64);metrics.insert("first_start_offset_ms",es.iter().map(|e|e["start_offset_ms"].as_f64().unwrap()).fold(f64::INFINITY,f64::min));metrics.insert("last_end_offset_ms",es.iter().map(|e|e["end_offset_ms"].as_f64().unwrap()).fold(0.,f64::max));out.insert(k,metrics);}
 json!({"records":events.len(),"threads":tids.len(),"stages":out})
}
fn analyze(preflight:bool){let r=Path::new(ROOT);let which=if preflight{"preflight"}else{"performance"};assert_eq!(read(r.join(format!("{which}/result.json")))["status"],"complete");
 let mut inventory=BTreeMap::new();let mut prior=prior();if !preflight{for(p,h)in read(r.join("preflight-proof-inventory.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());if p.ends_with(".proof"){assert!(prior.insert(h.as_str().unwrap().into()));}inventory.insert(p.clone(),h.as_str().unwrap().to_owned());}}
 let mut rows=Vec::new();let mut observations=Vec::new();let mut sdk=if preflight{0}else{16};
 for(p,h)in read(r.join(format!("{which}-pins.json"))).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}
 for job in read(r.join(format!("{which}-jobs.json"))).as_array().unwrap(){let name=job["name"].as_str().unwrap();let case=&job["case"];let rounds=job["rounds"].as_u64().unwrap()as usize;let log=fs::read_to_string(r.join(which).join(name).join("stdout.log")).unwrap();
  let summary:Value=serde_json::from_str(log.lines().find_map(|x|x.strip_prefix("SUMMARY ")).unwrap()).unwrap();assert_eq!(summary["rounds"],rounds);assert_eq!(summary["workers"],32);assert_eq!(summary["priming"],3);
  if job["mode"]!="accepted"{assert_eq!(summary["recorder_storage_bytes"],32768);}
  let samples:Vec<Value>=log.lines().filter_map(|x|x.strip_prefix("SAMPLE ")).map(|x|serde_json::from_str(x).unwrap()).collect();assert_eq!(samples.len(),rounds);for(i,s)in samples.iter().enumerate(){assert_eq!(s["round"],i);assert!(s["ms"].as_f64().unwrap()>0.);}
  let dir=r.join("proofs").join(name).join(Path::new(case["image"].as_str().unwrap()).file_stem().unwrap());
  assert_eq!(fs::metadata(dir.join("0.proof")).unwrap().len(),164);assert_eq!(hash(dir.join("0.public")),case["public_sha256"].as_str().unwrap());assert!(prior.insert(hash(dir.join("0.proof"))));for part in ["0.proof","0.public"]{let p=dir.join(part);inventory.insert(p.display().to_string(),hash(&p));}
  let v=fs::read_to_string(r.join(which).join(format!("verify-{name}/stdout.log"))).unwrap();assert!(v.lines().last().unwrap().contains("verified 1 distinct proofs across 1 pinned keys"));
  if job["mode"]=="on"{let captures=read(dir.join("stages.json"));assert_eq!(captures.as_array().unwrap().len(),rounds);for(i,c)in captures.as_array().unwrap().iter().enumerate(){assert_eq!(c["round"],i);observations.push(json!({"case":case["name"],"block":job["block"],"round":i,"stages":inspect(c)}));}}else{assert!(!dir.join("stages.json").exists());}
  sdk+=rounds+3;rows.push(json!({"case":case["name"],"mode":job["mode"],"block":job["block"],"samples":samples,"median_ms":median(samples.iter().map(|s|s["ms"].as_f64().unwrap()).collect()),"summary":summary}));
 }
 save(r.join(if preflight{"preflight-proof-inventory.json"}else{"proof-inventory.json"}),&json!(inventory));
 if preflight{assert_eq!(sdk,16);assert_eq!(prior.len(),5220);save(r.join("preflight-analysis.json"),&json!({"status":"passed","sdk_requests":sdk,"exports":4,"observations":observations,"rows":rows}));println!("Preflight16SDK/4Gnark passes,2snapshots");return;}
 assert_eq!(sdk,376);assert_eq!(prior.len(),5238);
 let mut cases_out=BTreeMap::new();let mut failures=Vec::new();for case in cases(){let name=case["name"].as_str().unwrap();let mut latency=BTreeMap::new();for mode in ["accepted","off","on"]{let rs:Vec<_>=rows.iter().filter(|x|x["case"]==name&&x["mode"]==mode).collect();assert_eq!(rs.len(),3);latency.insert(mode,json!({"request_ms":median(rs.iter().map(|x|x["median_ms"].as_f64().unwrap()).collect()),"whole_process_maxrss_mib":rs.iter().map(|x|x["summary"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max)}));}
  let mut ratios=BTreeMap::new();for(pair,a,b)in[("off_accepted","off","accepted"),("on_off","on","off")]{let mut ratios_vec=Vec::new();for block in 0..3{let ms=|m:&str|rows.iter().find(|x|x["case"]==name&&x["mode"]==m&&x["block"]==block).unwrap()["median_ms"].as_f64().unwrap();let ratio=ms(a)/ms(b);ratios_vec.push(ratio);if !(1./1.05..=1.05).contains(&ratio){failures.push(format!("{name} b{block} {pair}={ratio}"));}}let med=median(ratios_vec.clone());if !(1./1.03..=1.03).contains(&med){failures.push(format!("{name} paired {pair}={med}"));}ratios.insert(pair,json!({"block_ratios":ratios_vec,"paired_median":med}));}
  let mut stage_data:BTreeMap<String,Vec<Value>>=BTreeMap::new();for o in observations.iter().filter(|x|x["case"]==name){for(stage,v)in o["stages"]["stages"].as_object().unwrap(){stage_data.entry(stage.clone()).or_default().push(v.clone());}}
  let mut stages=BTreeMap::new();for(stage,vs)in stage_data{let mut metrics=BTreeMap::new();for metric in ["calls","wall_ms","thread_cpu_ms","first_start_offset_ms","last_end_offset_ms"]{metrics.insert(metric,median(vs.iter().map(|x|x[metric].as_f64().unwrap()).collect()));}stages.insert(stage,json!({"captures":vs.len(),"median":metrics}));}
  let ms=latency["accepted"]["request_ms"].as_f64().unwrap();cases_out.insert(name.to_owned(),json!({"latency":latency,"ratios":ratios,"stages":stages,"M5_CPU_speedup":case["reference"]["cpu_ms"].as_f64().unwrap()/ms,"M5_Metal_speedup":case["reference"]["metal_ms"].as_f64().unwrap()/ms}));
 }
 save(r.join("analysis.json"),&json!({"sdk_requests":sdk,"distinct_gnark_exports":22,"proof_public_members":44,"latency_processes":18,"trace_snapshots":102,"preflight_snapshots":2,"timing_qualified":failures.is_empty(),"timing_failures":failures,"cases":cases_out,"rows":rows,"observations":observations,"adopted":false,"goal_achieved":false,"new_AWS_or_Aeglos_measurement":false}));
 println!("Complete376SDK22Gnark,102latencysnapshots; timingqualified{}",failures.is_empty());
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare-preflight")=>make_plan(true),Some("prepare-performance")=>make_plan(false),Some("check-preflight")=>analyze(true),Some("analyze")=>analyze(false),_=>panic!("mode")}}
#[cfg(test)]mod tests{use super::*;fn e(id:usize,w0:u64,w1:u64)->Value{json!({"id":id,"label":"a","n":4,"tid":1,"wall0":w0,"wall1":w1,"cpu0":100,"cpu1":110})}#[test]fn nested_spans_preserve_inclusive_bounds(){let a=inspect(&json!({"events":[e(0,1000,2000),e(1,1100,1800),e(2,2100,2200)]}));assert_eq!(a["records"],3);assert_eq!(a["stages"]["a:n4"]["calls"],3.);}#[test]#[should_panic]fn crossing_spans_are_rejected(){inspect(&json!({"events":[e(0,1000,2000),e(1,1100,2100)]}));}}

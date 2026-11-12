use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/scaling-divergence180";
const OLD:&str="/home/ubuntu/src/zopro-perf-work/accepted-current-c7a176";
const STAGES:[&str;5]=["witness_ms","scalars_ms","fft_ms","msm_ms","assembly_ms"];
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){assert!(!p.as_ref().exists(),"refuse overwrite");fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn median(v:&[f64])->f64{assert!(!v.is_empty() && v.len()%2==1);let mut s=v.to_vec();s.sort_by(f64::total_cmp);s[s.len()/2]}
fn mean(v:&[f64])->f64{v.iter().sum::<f64>()/v.len()as f64}
fn close(a:f64,b:f64){assert!((a-b).abs()<1e-9*(1.+a.abs().max(b.abs())),"numeric mismatch {a} {b}")}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn selected(v:&Value)->bool{v["job"]["phase"]=="latency" && (v["job"]["case"]["shape"]=="1x2" || v["job"]["case"]["shape"]=="2x2") && (v["job"]["workers"]==32||v["job"]["workers"]==96)}
fn stream(v:&Value)->String{format!("{OLD}/remote-results{}/driver.stdout",v["job"]["directory"].as_str().unwrap())}
fn main(){
 let rows_path=format!("{OLD}/remote-results/output/measure/rows.json");
 let old=read(&rows_path);let rows=old.as_array().unwrap().iter().filter(|v|selected(v)).collect::<Vec<_>>();assert_eq!(rows.len(),120);
 if std::env::args().nth(1).as_deref()==Some("prepare"){
  let mut files=vec![rows_path,format!("{OLD}/analysis.json"),format!("{ROOT}/analyze"),format!("{ROOT}/analyze.rs"),format!("{ROOT}/design.md")];
  files.extend(rows.iter().filter(|v|v["job"]["variant"]=="aeglos").map(|v|stream(v)));
  let src="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-22-aeglos-windowed-ablation/aeglos-source";
  for p in ["driver.go","engine.go","internal/cpu/key.go","internal/prover/batch.go","internal/prover/timings.go"]{files.push(format!("{src}/{p}"));}
  let pins=files.iter().map(|p|(p.clone(),json!(hash(p)))).collect::<BTreeMap<_,_>>();save(format!("{ROOT}/pins.json"),&json!(pins));
  save(format!("{ROOT}/plan.json"),&json!({"version":1,"source_pins":format!("{ROOT}/pins.json"),"steps":[{"kind":"command","name":"retained-stage-scaling","argv":[format!("{ROOT}/analyze")],"timeout_seconds":120}]}));return;
 }
 let mut process=vec![];let mut requests=0usize;let mut checked=0usize;
 for v in &rows{
  let job=&v["job"];let block=job["block"].as_u64().unwrap();let workers=job["workers"].as_u64().unwrap();let shape=job["case"]["shape"].as_str().unwrap();let arm=job["variant"].as_str().unwrap();let original=v["sdk"]["samples_ms"].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect::<Vec<_>>();
  let expected=if shape=="1x2"{101}else{17};assert_eq!(original.len(),expected);close(median(&original),v["sdk"]["median_ms"].as_f64().unwrap());checked+=expected;
  let mut columns:BTreeMap<String,Vec<f64>>=BTreeMap::new();columns.insert("verified_ms".into(),original.clone());
  if arm=="aeglos"{
   let text=fs::read_to_string(stream(v)).unwrap();let samples=text.lines().filter_map(|s|s.strip_prefix("SAMPLE ")).map(|s|serde_json::from_str::<Value>(s).unwrap()).collect::<Vec<_>>();assert_eq!(samples.len(),expected);
   for (i,s) in samples.iter().enumerate(){assert_eq!(s["round"].as_u64(),Some(i as u64));assert_eq!(s["cache_hit"],true);close(s["verified_ms"].as_f64().unwrap(),original[i]);
    for k in STAGES.into_iter().chain(["request_ms","engine_ms"]){let x=s[k].as_f64().unwrap();assert!(x.is_finite()&&x>=0.);columns.entry(k.into()).or_default().push(x);}
    let sum=STAGES.iter().map(|k|s[k].as_f64().unwrap()).sum::<f64>();let engine=s["engine_ms"].as_f64().unwrap();let outer=original[i]-engine;let residual=engine-sum;
    assert!(residual>=-1e-6 && outer>=-1e-6,"stage containment failed for {} round{i}: {residual} {outer}",job["name"]);
    columns.entry("engine_residual_ms".into()).or_default().push(residual);
    columns.entry("outer_residual_ms".into()).or_default().push(outer);
    close(sum+residual+outer,original[i]);requests+=1;
   }
  }else{assert_eq!(arm,"accepted");}
  let stats=columns.iter().map(|(k,v)|(k.clone(),json!({"mean_ms":mean(v),"median_ms":median(v)}))).collect::<BTreeMap<_,_>>();
  process.push(json!({"shape":shape,"workers":workers,"arm":arm,"block":block,"requests":expected,"stages":stats}));
 }
 let mut summaries=vec![];let mut deltas=vec![];
 for shape in ["1x2","2x2"]{for arm in ["accepted","aeglos"]{
  let mut stagekeys=vec!["verified_ms"];
  if arm=="aeglos"{stagekeys.extend(STAGES);stagekeys.extend(["engine_ms","request_ms","engine_residual_ms","outer_residual_ms"]);}
  for stage in &stagekeys{
   let mut means=BTreeMap::new();
   for w in [32u64,96]{let mut own=process.iter().filter(|v|v["shape"]==shape&&v["arm"]==arm&&v["workers"]==w).collect::<Vec<_>>();own.sort_by_key(|v|v["block"].as_u64().unwrap());assert_eq!(own.len(),15);assert_eq!(own.iter().map(|v|v["block"].as_u64().unwrap()).collect::<Vec<_>>(),(0..15).collect::<Vec<_>>());
    let avg=own.iter().map(|v|v["stages"][stage]["mean_ms"].as_f64().unwrap()).collect::<Vec<_>>();let med=own.iter().map(|v|v["stages"][stage]["median_ms"].as_f64().unwrap()).collect::<Vec<_>>();
    summaries.push(json!({"shape":shape,"arm":arm,"workers":w,"stage":stage,"median_of_means_ms":median(&avg),"mean_of_means_ms":mean(&avg),"median_of_medians_ms":median(&med)}));means.insert(w,avg);
   }
   let diff=means[&32].iter().zip(&means[&96]).map(|(a,b)|a-b).collect::<Vec<_>>();let ratio=means[&32].iter().zip(&means[&96]).map(|(a,b)|b/a).collect::<Vec<_>>();
   deltas.push(json!({"shape":shape,"arm":arm,"stage":stage,"positive_means_96_saves_ms":true,"paired_savings_ms":diff,"average_savings_ms":mean(&diff),"median_savings_ms":median(&diff),"median_96_over_32":median(&ratio),"blocks_96_faster":diff.iter().filter(|&&x|x>0.).count()}));
  }
  if arm=="aeglos"{let savings=|s:&str|deltas.iter().find(|v|v["shape"]==shape&&v["arm"]==arm&&v["stage"]==s).unwrap()["average_savings_ms"].as_f64().unwrap();let sum=STAGES.iter().map(|s|savings(s)).sum::<f64>()+savings("engine_residual_ms")+savings("outer_residual_ms");close(sum,savings("verified_ms"));}
 }}
 let original_analysis=read(format!("{OLD}/analysis.json"));for row in original_analysis["rows"].as_array().unwrap(){for arm in ["accepted","aeglos"]{if let Some(s)=summaries.iter().find(|s|s["shape"]==row["shape"]&&s["workers"]==row["workers"]&&s["arm"]==arm&&s["stage"]=="verified_ms"){close(s["median_of_medians_ms"].as_f64().unwrap(),row["arms"][arm]["median_of_process_medians_ms"].as_f64().unwrap());}}}
 assert_eq!(requests,3540);assert_eq!(checked,7080);
 save(format!("{ROOT}/analysis.json"),&json!({"historical_campaign":"2026-11-10-accepted-current-c7a176","new_capture":false,"production_changed":false,"new_exports":0,"checked_Aeglos_requests":requests,"checked_total_samples":checked,"processes":process,"summary":summaries,"paired_stage_savings":deltas,"passed":true,"limits":"Historical named elapsed stages, not exclusive CPU or ready-work attribution; observer overhead unqualified. No causal scheduler/table claim or current Zopro phase timing."}));
 println!("Validated {requests} retained Aeglos requests and {checked} total samples; all stage sums and original medians agree. No new captures or proofs.");
}

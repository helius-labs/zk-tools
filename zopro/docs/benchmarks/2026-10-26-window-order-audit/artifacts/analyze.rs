use std::{fs,path::Path,collections::{BTreeMap,BTreeSet},fmt::Write};use serde_json::{Value,json};use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-window-order-audit";
const OLD:&str="/tmp/zopro-current-readiness";
const ARCH:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-25-current-readiness";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn summary(mut v:Vec<f64>)->Value{assert!(!v.is_empty());v.sort_by(f64::total_cmp);let n=v.len();let median=if n%2==1{v[n/2]}else{(v[n/2-1]+v[n/2])/2.};json!({"n":n,"median":median,"p10":v[((n-1)as f64*0.1).floor()as usize],"p90":v[((n-1)as f64*0.9).ceil()as usize],"min":v[0],"max":v[n-1]})}
type Event=[u64;8];
fn validate(events:&[Event],workers:u64)->Result<BTreeMap<u64,Event>,String>{
 let mut by=BTreeMap::new();for e in events {if e[0]==0||e[5]>e[6]||e[7]>workers||by.insert(e[0],*e).is_some(){return Err("invalid event or duplicate identity".into());}}
 for e in events{if e[1]!=0{let p=by.get(&e[1]).ok_or("missing parent")?;if e[5]<p[5]||e[6]>p[6]||e[0]==e[1]{return Err("interval outside parent or self-parent".into());}}}
 Ok(by)
}
fn top_metrics(events:&[Event],kernel_kind:u64,window_kind:u64,windows:u64,width:u64)->Result<Value,String>{
 let kernels:Vec<_>=events.iter().filter(|e|e[2]==kernel_kind).collect();if kernels.len()!=1{return Err("kernel count".into());}let kernel=kernels[0];
 let mut all:Vec<_>=events.iter().filter(|e|e[2]==window_kind).collect();if all.len()!=windows as usize{return Err("window count".into());}
 let mut ids=BTreeSet::new();for e in &all{if e[1]!=kernel[0]||e[3]>=windows||e[4]!=width||!ids.insert(e[3]){return Err("window parent, width or identity".into());}}
 let top=*all.iter().find(|e|e[3]+1==windows).ok_or("top absent")?;
 let first=all.iter().map(|e|e[5]).min().unwrap();let last_ordinary=all.iter().filter(|e|e[3]+1!=windows).map(|e|e[6]).max().unwrap();
 all.sort_by_key(|e|(e[5],e[3]));let start_rank=all.iter().position(|e|e[0]==top[0]).unwrap();
 all.sort_by_key(|e|(e[6],e[3]));let end_rank=all.iter().position(|e|e[0]==top[0]).unwrap();
 Ok(json!({"window_count":windows,"top_window":windows-1,"kernel_ms":(kernel[6]-kernel[5])as f64/1e6,"top_start_from_kernel_ms":(top[5]-kernel[5])as f64/1e6,"top_start_from_first_window_ms":(top[5]-first)as f64/1e6,"top_inclusive_ms":(top[6]-top[5])as f64/1e6,"top_start_rank_zero_based":start_rank,"top_end_rank_zero_based":end_rank,"top_last":end_rank+1==windows as usize,"top_gap_after_ordinary_ms":top[6].saturating_sub(last_ordinary)as f64/1e6,"top_end_minus_last_ordinary_ms":(top[6]as i64-last_ordinary as i64)as f64/1e6}))
}
fn main(){let r=Path::new(R);let sums=fs::read_to_string(Path::new(ARCH).join("SHA256SUMS")).unwrap();let mut inputs=BTreeMap::new();
 for rel in ["graphs.json","protocol.json","analysis.json","diagnostic-source/crates/zopro-core/src/readiness_trace.rs"]{
  let h=sums.lines().find_map(|line|line.split_once("  ").filter(|(_,name)|*name==rel).map(|(h,_)|h)).unwrap();let p=Path::new(ARCH).join(rel);assert_eq!(hash(&p),h);inputs.insert(p.display().to_string(),h.to_string());
  if !rel.starts_with("diagnostic-source/"){assert_eq!(hash(Path::new(OLD).join(rel)),h);inputs.insert(Path::new(OLD).join(rel).display().to_string(),h.to_string());}
 }
 let p=read(Path::new(OLD).join("protocol.json"));for(kind,label)in [(16,"g1_merge_kernel"),(17,"g1_window"),(22,"kh_kernel"),(23,"kh_window"),(31,"g2_kernel"),(33,"g2_window")]{assert_eq!(p["labels"][kind],label);}
 for file in ["design.md","analyze.rs"]{inputs.insert(r.join(file).display().to_string(),hash(r.join(file)));}save(r.join("input-pins.json"),&json!(inputs));
 let graphs=read(Path::new(OLD).join("graphs.json"));assert_eq!(graphs.as_array().unwrap().len(),103);let mut rows=Vec::new();let mut skipped=0;
 for row in graphs.as_array().unwrap(){let workers=row["workers"].as_u64().unwrap();if workers==4{assert_eq!(row["process"],"preflight/on-w4");skipped+=1;continue;}assert!([16,32].contains(&workers));
  let events:Vec<Event>=row["graph"]["threads"].as_array().unwrap().iter().flat_map(|bag|bag.as_array().unwrap()).map(|e|serde_json::from_value(e.clone()).unwrap()).collect();validate(&events,workers).unwrap();
  let mut queries=BTreeMap::new();for(name,kernel,window,count,width)in [("a",16,17,22,12),("kh",22,23,20,13),("g2",31,33,11,12)]{queries.insert(name,top_metrics(&events,kernel,window,count,width).unwrap());}
  rows.push(json!({"workers":workers,"process":row["process"],"rep":row["graph"]["rep"],"block":row["graph"]["block"],"queries":queries}));
 }assert_eq!(skipped,1);assert_eq!(rows.len(),102);
 let mut result=BTreeMap::new();for workers in [16,32]{let group:Vec<_>=rows.iter().filter(|x|x["workers"]==workers).collect();assert_eq!(group.len(),51);let mut by=BTreeMap::new();for query in ["a","kh","g2"]{let mut stats=BTreeMap::new();for metric in ["kernel_ms","top_start_from_kernel_ms","top_start_from_first_window_ms","top_inclusive_ms","top_start_rank_zero_based","top_end_rank_zero_based","top_gap_after_ordinary_ms","top_end_minus_last_ordinary_ms"]{stats.insert(metric,summary(group.iter().map(|x|x["queries"][query][metric].as_f64().unwrap()).collect()));}let last=group.iter().filter(|x|x["queries"][query]["top_last"]==true).count();by.insert(query,json!({"metrics":stats,"top_last_count":last,"requests":51}));}result.insert(workers,json!(by));}
 save(r.join("rows.json"),&json!(rows));save(r.join("analysis.json"),&json!({"worker_groups":result,"all_matrix_graphs_retained":102,"preflight_graphs_not_in_matrix":1,"new_sdk_requests":0,"new_gnark_exports":0,"next_prior_count":6928,"production_changed":false,"goal_achieved":false}));
 let mut table=String::from("|Workers|Query|Top start from kernel ms|Top start from first window ms|Start rank (0-based)|Top last /51|Top inclusive ms|Gap after ordinary callbacks ms|\n|---|---|---:|---:|---:|---:|---:|---:|\n");for workers in [32,16]{for q in ["a","kh","g2"]{let cell=&result[&workers][q];let m=&cell["metrics"];writeln!(table,"|{workers}|{q}|{:.6}|{:.6}|{:.0}|{}|{:.6}|{:.6}|",m["top_start_from_kernel_ms"]["median"].as_f64().unwrap(),m["top_start_from_first_window_ms"]["median"].as_f64().unwrap(),m["top_start_rank_zero_based"]["median"].as_f64().unwrap(),cell["top_last_count"],m["top_inclusive_ms"]["median"].as_f64().unwrap(),m["top_gap_after_ordinary_ms"]["median"].as_f64().unwrap()).unwrap();}}fs::write(r.join("table.md"),&table).unwrap();println!("{table}");
 for(p,h)in &inputs{assert_eq!(&hash(p),h);}
}
#[cfg(test)]mod tests{use super::*;
 fn events()->Vec<Event>{vec![[1,0,22,0,0,0,100,0],[2,1,23,0,13,10,70,0],[3,1,23,1,13,5,90,1]]}
 #[test]fn early_top_may_finish_last_without_late_release(){let e=events();validate(&e,1).unwrap();let m=top_metrics(&e,22,23,2,13).unwrap();assert_eq!(m["top_start_rank_zero_based"],0);assert_eq!(m["top_end_rank_zero_based"],1);assert_eq!(m["top_gap_after_ordinary_ms"],20./1e6);assert_eq!(m["top_start_from_first_window_ms"],0.);}
 #[test]fn malformed_parent_and_window_labels_are_rejected(){let mut e=events();e[2][1]=99;assert!(validate(&e,1).is_err());assert!(top_metrics(&e,22,23,2,13).is_err());let mut e=events();e[2][3]=0;assert!(top_metrics(&e,22,23,2,13).is_err());let mut e=events();e[2][6]=101;assert!(validate(&e,1).is_err());}
}

//! Post-capture structural analysis; does not change the frozen timing gates.
use serde_json::{json,Value};use std::{fs,collections::BTreeMap,path::Path};
#[path="observations.rs"]mod observations;
fn main(){let r=Path::new("/tmp/zopro-ready-arithmetic");let config:Value=serde_json::from_slice(&fs::read(r.join("perf-plan.json")).unwrap()).unwrap();let mut result=BTreeMap::new();
 for c in config["cases"].as_array().unwrap(){let name=c["name"].as_str().unwrap();let stem=Path::new(c["image"].as_str().unwrap()).file_stem().unwrap();let mut shares=Vec::new();let mut counts=BTreeMap::<String,Vec<usize>>::new();let mut fill_ends=Vec::new();
  for block in 0..3{let p=r.join(format!("performance/latency/b{block}/{name}/on/proofs")).join(stem).join("readiness.json");let captures:Value=serde_json::from_slice(&fs::read(p).unwrap()).unwrap();for cap in captures.as_array().unwrap(){let events:Vec<_>=cap["events"].as_array().unwrap().iter().filter(|e|e["state"]==2).collect();let mut durations=BTreeMap::<String,u64>::new();let mut one_counts=BTreeMap::<String,usize>::new();let mut total=0u64;let mut end=0u64;let mut fill_end=0u64;
    for e in events{let k=e["kind"].as_u64().unwrap();let elapsed=e["end"].as_u64().unwrap()-e["start"].as_u64().unwrap();total+=elapsed;end=end.max(e["end"].as_u64().unwrap());*durations.entry(k.to_string()).or_default()+=elapsed;*one_counts.entry(k.to_string()).or_default()+=1;if k<=3{fill_end=fill_end.max(e["end"].as_u64().unwrap());}}
    let fill:u64=["1","2","3"].iter().map(|k|durations.get(*k).copied().unwrap_or(0)).sum();shares.push(fill as f64/total as f64);fill_ends.push((end-fill_end)as f64/1e6);for(k,v)in one_counts{counts.entry(k).or_default().push(v);}
  }}
  let mut ranges=BTreeMap::new();for(k,v)in counts{ranges.insert(k,json!({"min":v.iter().min().unwrap(),"max":v.iter().max().unwrap(),"observations_with_kind":v.len()}));}
  result.insert(name,json!({"kind_unit_count_ranges":ranges,"streaming_fill_share_of_recorded_leaf_elapsed_median":observations::median(shares),"last_recorded_leaf_after_last_recorded_fill_ms_median":observations::median(fill_ends)}));
 }
 fs::write(r.join("geometry.json"),serde_json::to_vec_pretty(&json!({"scope":"Post-capture structural description, no new gate, speed claim or new benchmark. Eight-by-two has uninstrumented large-G2 streaming fills and must not be interpreted as complete arithmetic occupancy.","cases":result})).unwrap()).unwrap();println!("Described original recorded task geometry; frozen calibration failures unchanged");}

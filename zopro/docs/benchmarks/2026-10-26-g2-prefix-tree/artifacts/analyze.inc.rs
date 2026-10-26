fn analyze(){
 let r=Path::new(R);assert_eq!(read(r.join("measure/result.json"))["status"],"complete");
 for(p,h)in read(r.join("measure-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let mut rows=Vec::new();let mut calls=0;
 for job in read(r.join("jobs.json")).as_array().unwrap(){
  let log=fs::read_to_string(r.join("measure").join(job["name"].as_str().unwrap()).join("stdout.log")).unwrap();
  let summary:Value=serde_json::from_str(log.lines().find_map(|s|s.strip_prefix("SUMMARY ")).unwrap()).unwrap();
  let samples:Vec<Value>=log.lines().filter_map(|s|s.strip_prefix("SAMPLE ")).map(|s|serde_json::from_str(s).unwrap()).collect();
  for k in ["rounds","workers"]{assert_eq!(summary[k],job[k]);}assert_eq!(summary["priming"],2);assert_eq!(summary["reference_setup_calls"],1);
  assert_eq!(summary["mode"],if job["mode"]=="frozen"{serde_json::json!("stream")}else{job["mode"].clone()});
  assert_eq!(samples.len(),job["rounds"].as_u64().unwrap()as usize);
  for(i,s)in samples.iter().enumerate(){assert_eq!(s["round"],i);assert!(s["ms"].as_f64().unwrap()>0.);}
  let peak=samples.iter().map(|s|s["peak_live_requested_above_inputs_bytes"].as_u64().unwrap()).max().unwrap();
  assert!(peak<=summary["forecast_requested_bytes"].as_u64().unwrap()+summary["points"].as_u64().unwrap()*32);
  calls+=samples.len()+3;
  rows.push(json!({"job":job,"median_ms":med(samples.iter().map(|s|s["ms"].as_f64().unwrap()).collect()),"max_requested_above_inputs_bytes":peak,"samples":samples,"summary":summary}));
 }
 assert_eq!(calls,1881);let mut comparisons=BTreeMap::new();let mut failures=Vec::new();let mut calibration=Vec::new();
 for shape in ["1x2","2x2"]{for workers in [1,16,32]{
  let mut modes=BTreeMap::new();
  for mode in ["frozen","stream","tree","prefix"]{
   let rs:Vec<_>=rows.iter().filter(|r|r["job"]["phase"]=="latency"&&r["job"]["shape"]==shape&&r["job"]["workers"]==workers&&r["job"]["mode"]==mode).collect();assert_eq!(rs.len(),7);
   modes.insert(mode,json!({"request_ms":med(rs.iter().map(|r|r["median_ms"].as_f64().unwrap()).collect()),"max_requested_above_inputs_bytes":rs.iter().map(|r|r["max_requested_above_inputs_bytes"].as_u64().unwrap()).max().unwrap(),"max_process_RSS_mib":rs.iter().map(|r|r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max),"forecast_bytes":rs[0]["summary"]["forecast_requested_bytes"]}));
  }
  let mut pairs=BTreeMap::new();
  for (name,a,b)in [("stream_frozen","stream","frozen"),("prefix_stream","prefix","stream"),("prefix_frozen","prefix","frozen"),("prefix_tree","prefix","tree")]{
   let mut ratios=Vec::new();
   for block in 0..7{let ms=|m:&str|rows.iter().find(|r|r["job"]["phase"]=="latency"&&r["job"]["shape"]==shape&&r["job"]["workers"]==workers&&r["job"]["mode"]==m&&r["job"]["block"]==block).unwrap()["median_ms"].as_f64().unwrap();let ratio=ms(a)/ms(b);ratios.push(ratio);if name=="stream_frozen"&&!(1./1.05..=1.05).contains(&ratio){calibration.push(format!("{shape}/w{workers}/b{block} stream/frozen{ratio}"));}}
   let ratio=gm(&ratios);let wins=ratios.iter().filter(|x|**x<1.).count();let median=med(ratios.clone());
   if name=="stream_frozen"&&!(1./1.03..=1.03).contains(&median){calibration.push(format!("{shape}/w{workers} paired stream/frozen{median}"));}
   if workers>=16&&["prefix_stream","prefix_frozen"].contains(&name){if shape=="1x2"&&(ratio>=1.||wins<4){failures.push(format!("{shape}/w{workers} {name} ratio{ratio} wins{wins}/7"));}if shape=="2x2"&&ratio>1.02{failures.push(format!("{shape}/w{workers} {name} ratio{ratio}"));}}
   pairs.insert(name,json!({"paired_ratios":ratios,"paired_geomean":ratio,"paired_median":median,"wins":wins}));
  }
  comparisons.insert(format!("{shape}-w{workers}"),json!({"modes":modes,"pairs":pairs}));
 }}
 let mut profiles=BTreeMap::new();
 for mode in ["stream","tree","prefix"]{
  let script=fs::read_to_string(r.join(format!("measure/samples-{mode}/stdout.log"))).unwrap();assert!(!script.contains("PERF_RECORD_LOST"));
  let report=fs::read_to_string(r.join(format!("measure/report-{mode}/stdout.log"))).unwrap();assert!(report.contains("Total Lost Samples: 0"));
  let mut pids=std::collections::BTreeSet::new();let mut count=0;
  for l in script.lines().filter(|l|l.contains("cycles:u:")){let pieces:Vec<_>=l.split_whitespace().collect();pids.insert(pieces[1].split('/').next().unwrap().parse::<u32>().unwrap());count+=1;}
  assert!(count>0);assert_eq!(pids.len(),1);
  let out=Command::new("sudo").args(["-n","sha256sum"]).arg(r.join("profiles").join(mode).join("perf.data")).output().unwrap();assert!(out.status.success());
  profiles.insert(mode,json!({"samples":count,"sampled_pids":pids,"lost_records":0,"raw_local_only":true,"raw_sha256":String::from_utf8(out.stdout).unwrap().split_whitespace().next().unwrap()}));
 }
 save(r.join("analysis.json"),&json!({"status":"complete prefix-tree component study","raw_query_calls":1881,"latency_processes":168,"profile_processes":3,"reference_setup_calls":171,"SDK_proofs":0,"Gnark_exports":0,"prior_proof_count":5958,"calibration_passed":calibration.is_empty(),"component_gate_passed":failures.is_empty()&&calibration.is_empty(),"calibration_failures":calibration,"failures":failures,"comparisons":comparisons,"profiles":profiles,"rows":rows,"adopted":false,"goal_achieved":false}));
 println!("1881rawqueries;calibration{};speedgate{}",calibration.is_empty(),failures.is_empty());
}

fn evaluate(stage:&str){
 let r=Path::new(R);let dir=r.join(stage);let result=read(dir.join("result.json"));assert_eq!(result["status"],"complete");
 let config=read(dir.join("protocol.json"));let rows=read(dir.join("rows.json"));let rows=rows.as_array().unwrap();let screen=stage=="screen";let blocks=if screen{3}else{7};
 let mut cases=BTreeMap::new();let mut calibration=Vec::new();let mut primary=Vec::new();let mut regressions=Vec::new();let mut original:BTreeMap<&str,Vec<f64>>=[("accepted",vec![]),("owned",vec![]),("kh",vec![])].into_iter().collect();
 for case in config["cases"].as_array().unwrap(){let name=case["name"].as_str().unwrap();let mut modes=BTreeMap::new();let mut pairs=BTreeMap::new();
  for variant in ["accepted","owned","kh","all"]{
   let rs:Vec<_>=rows.iter().filter(|x|x["phase"]=="latency"&&x["case"]==name&&x["variant"]==variant).collect();assert_eq!(rs.len(),blocks);
   modes.insert(variant,json!({"ms":median(rs.iter().map(|x|x["median_ms"].as_f64().unwrap()).collect()),"maxrss_mib":rs.iter().map(|x|x["summary"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max),"cpu_ms_per_request":median(rs.iter().map(|x|(x["summary"]["user_cpu_ms"].as_f64().unwrap()+x["summary"]["system_cpu_ms"].as_f64().unwrap())/x["summary"]["rounds"].as_f64().unwrap()).collect())}));
  }
  for(a,b)in [("owned","accepted"),("all","accepted"),("all","owned"),("all","kh")]{
   let mut savings=Vec::new();let ratios:Vec<f64>=(0..blocks).map(|block|{
    let ms=|v:&str|rows.iter().find(|x|x["phase"]=="latency"&&x["case"]==name&&x["variant"]==v&&x["block"]==block).unwrap()["median_ms"].as_f64().unwrap();
    savings.push((ms(b)-ms(a))*1000.);ms(a)/ms(b)
   }).collect();let ratio=gm(&ratios);let wins=ratios.iter().filter(|x|**x<1.).count();
   if a=="owned" {
    for(block,v)in ratios.iter().enumerate(){if !(1./1.05..=1.05).contains(v){calibration.push(format!("{name} block{block} owned/accepted={v}"));}}
    let med=median(ratios.clone());if !(1./1.03..=1.03).contains(&med){calibration.push(format!("{name} median owned/accepted={med}"));}
   }else{
    if name=="1x2-w32"&&(ratio>=1.||wins<if screen{2}else{4}){primary.push(format!("{name} all/{b}={ratio},wins{wins}"));}
    if ratio>1.02{regressions.push(format!("{name} all/{b}={ratio}"));}
    if !screen&&case["workers"]==32&&case["shape"]!="merge-54x1"{original.get_mut(b).unwrap().push(ratio);}
   }
   pairs.insert(format!("{a}/{b}"),json!({"geometric_ratio":ratio,"ratios":ratios,"median_ratio":median(ratios),"wins":wins,"saving_us":median(savings)}));
  }
  cases.insert(name.to_string(),json!({"shape":case["shape"],"workers":case["workers"],"reference":case["reference"],"modes":modes,"pairs":pairs}));
 }
 let mut original_geos=BTreeMap::new();if !screen{for(control,values)in original{assert_eq!(values.len(),8);original_geos.insert(control,gm(&values));}}
 let pass=calibration.is_empty()&&primary.is_empty()&&regressions.is_empty()&&original_geos.values().all(|x|*x<=1.);
 let exports=if screen{24}else{284};assert_eq!(result["distinct_verified_exports"],exports);let sdk:u64=rows.iter().map(|x|x["summary"]["rounds"].as_u64().unwrap()+3).sum();assert_eq!(sdk,if screen{480}else{13896});
 if !screen{assert_eq!(result["profiles"],4);for row in rows.iter().filter(|x|x["phase"]=="profile"){assert_eq!(row["profile"]["lost_records"],0);assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(),1);}}
 let mut inventory:BTreeMap<String,String>=serde_json::from_value(read(r.join("preflight-proof-inventory.json"))).unwrap();
 for sub in if screen{vec!["screen"]}else{vec!["screen","performance"]}{let d=r.join(sub);for(rel,h)in read(d.join("proof-inventory.json")).as_object().unwrap(){let p=d.join(rel);assert_eq!(hash(&p),h.as_str().unwrap());assert!(inventory.insert(p.display().to_string(),h.as_str().unwrap().into()).is_none());}}
 let mut seen=prior();for(p,h)in &inventory{assert_eq!(hash(p),*h);if p.ends_with(".proof"){assert_eq!(fs::metadata(p).unwrap().len(),164);assert!(seen.insert(h.clone()));}}
 assert_eq!(seen.len(),if screen{6928}else{7212});save(r.join("proof-inventory.json"),&json!(inventory));if screen{save(r.join("screen-combined-proof-inventory.json"),&json!(inventory));}
 let a=json!({"stage":stage,"cases":cases,"calibration_failures":calibration,"primary_failures":primary,"regressions":regressions,"original8":original_geos,"screen_advances_to_full":screen&&pass,"full_gate_passed":!screen&&pass,"sdk_requests":sdk+72+if screen{0}else{480},"independent_exports":exports+18+if screen{0}else{24},"next_prior_count":seen.len(),"adopted":false,"goal_achieved":false,"candidate_retained":true});save(r.join(format!("{stage}-analysis.json")),&a);
 if screen&&pass{let mut c=read(r.join("perf-plan-screen.json"));c["prior_proof_sha256"]=json!(seen);let mut pins:BTreeMap<String,String>=serde_json::from_value(c["pins"].clone()).unwrap();for f in ["screen-analysis.json","screen/result.json","screen/freshness.json","screen-combined-proof-inventory.json"]{add(r.join(f),&mut pins);}c["pins"]=json!(pins);save(r.join("perf-plan-full.json"),&c);}
 println!("{}",serde_json::to_string_pretty(&json!({"stage":stage,"passes":pass,"calibration":a["calibration_failures"],"primary":a["primary_failures"],"regressions":a["regressions"],"next_prior":seen.len()})).unwrap());
}

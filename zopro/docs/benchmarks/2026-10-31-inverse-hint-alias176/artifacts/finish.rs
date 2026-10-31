fn interval(a:&[f64])->Value{assert_eq!(a.len(),15);let mut s=a.to_vec();s.sort_by(f64::total_cmp);json!({"median":s[7],"low":s[3],"high":s[11],"wins":a.iter().filter(|&&x|x<1.).count(),"ratios":a,"nominal_marginal_coverage":0.96484375})}
fn finish(){
 assert_eq!(read(r("runs/result.json"))["status"],"complete");for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let jobs=read(r("jobs.json"));let mut ledger:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();assert_eq!(ledger.len(),28847);
 let(mut inventory,mut rows,mut counters,mut profiles,mut symbol_rows)=(vec![],vec![],BTreeMap::new(),BTreeMap::new(),BTreeMap::new());let mut vks=BTreeMap::new();let mut count_requests=0;
 for j in jobs.as_array().unwrap(){
  let n=j["name"].as_str().unwrap();let c=&j["case"];let rounds=j["rounds"].as_u64().unwrap();count_requests+=rounds+3;
  let log=fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap();let samples=log.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();assert_eq!(samples.len(),rounds as usize);
  for(i,s)in samples.iter().enumerate(){assert_eq!(s["round"],i);assert!(s["ms"].as_f64().unwrap().is_finite()&&s["ms"].as_f64().unwrap()>0.);}
  let summary:Value=serde_json::from_str(log.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();assert_eq!(summary["rounds"],rounds);assert_eq!(summary["priming"],3);assert_eq!(summary["workers"],c["workers"]);
  let aliases=routes(j,&log);rows.push(json!({"name":n,"case":c["name"],"variant":j["variant"],"panel":j["panel"],"block":j["block"],"median_ms":median(&samples.iter().map(|s|s["ms"].as_f64().unwrap()).collect::<Vec<_>>()),"CPU_ms_per_request":(summary["user_cpu_ms"].as_f64().unwrap()+summary["system_cpu_ms"].as_f64().unwrap())/rounds as f64,"counter_RSS_MiB":summary["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"usage":summary,"aliases":aliases}));
  let dir=Path::new(j["directory"].as_str().unwrap());let ph=hash(dir.join("0.proof"));assert_eq!(fs::metadata(dir.join("0.proof")).unwrap().len(),164);assert_eq!(hash(dir.join("0.public")),c["public_sha256"]);
  let receipt=fs::read_to_string(r(format!("runs/verify-{n}/stdout.log"))).unwrap();let line=receipt.lines().filter_map(|l|l.strip_prefix("VERIFIED ")).collect::<Vec<_>>();assert_eq!(line.len(),1);let f=line[0].split_whitespace().map(|s|s.split_once('=').unwrap()).collect::<BTreeMap<_,_>>();assert_eq!(f["proof_sha256"],ph);assert_eq!(f["public_sha256"],c["public_sha256"]);assert_eq!(f["image_sha256"],c["image_sha256"]);assert_eq!(f["vk_sha256"].len(),64);
  if let Some(old)=vks.insert(c["shape"].as_str().unwrap().to_string(),f["vk_sha256"].to_string()){assert_eq!(old,f["vk_sha256"]);}
  assert!(ledger.insert(ph.clone()));inventory.push(json!({"name":n,"proof_sha256":ph,"public_sha256":c["public_sha256"],"image_sha256":c["image_sha256"],"vk_sha256":f["vk_sha256"],"verified":true}));
  if j["panel"]=="stat"{
   let text=fs::read_to_string(j["data"].as_str().unwrap()).unwrap();let events=text.lines().filter_map(event_line).collect::<BTreeMap<_,_>>();assert_eq!(events.len(),4);let per=events.iter().map(|(k,v)|(k.clone(),v["count"].as_f64().map(|x|x/rounds as f64))).collect::<BTreeMap<_,_>>();
   counters.insert(n.to_string(),json!({"events":events,"per_request":per,"qualified":events.values().all(|v|v["qualified"]==true)}));
  }
  if j["panel"]=="cycles"{
   let meta=fs::read_to_string(r(format!("runs/metadata-{n}/stdout.log"))).unwrap();let(pid,tids)=profile_owners(&meta,j["binary"].as_str().unwrap());
   let attrs=fs::read_to_string(r(format!("runs/events-{n}/stdout.log"))).unwrap();for s in["cycles:u","16000000","CPU","TIME","PERIOD"]{assert!(attrs.contains(s));}assert!(!attrs.contains("freq: 1"));assert_eq!(read(r(format!("runs/loss-{n}/stdout.log")))["zero_lost"],true);
   let own=read(r(format!("runs/own-{n}/stdout.log")));assert_eq!(own["sha256_before"],own["sha256_after"]);assert_eq!(hash(j["data"].as_str().unwrap()),own["sha256_after"]);
   let mut bins=BTreeMap::<String,(u64,u64)>::new();let mut cats=BTreeMap::<String,u64>::new();let(mut count,mut period)=(0u64,0u64);
   for(p,t,cpu,w,sym)in fs::read_to_string(r(format!("runs/decode-{n}/stdout.log"))).unwrap().lines().filter_map(sample){assert_eq!(p,pid);assert!(tids.contains(&t));assert!(u64::from(cpu)<c["workers"].as_u64().unwrap());assert_eq!(w,16000000);count+=1;period+=w;let b=bins.entry(sym.clone()).or_default();b.0+=1;b.1+=w;*cats.entry(category(&sym).into()).or_default()+=w;}
   assert!(count>0);let unknown=cats.get("unknown").copied().unwrap_or(0)as f64/period as f64;
   let shares=cats.iter().map(|(s,p)|(s.clone(),json!({"period":p,"share":*p as f64/period as f64}))).collect::<BTreeMap<_,_>>();
   symbol_rows.insert(n.to_string(),json!(bins.iter().map(|(s,(count,period))|json!({"symbol":s,"samples":count,"period":period})).collect::<Vec<_>>()));
   profiles.insert(n.to_string(),json!({"samples":count,"period":period,"categories":shares,"unknown_share":unknown,"qualified":count>=1000&&unknown<=0.05,"zero_lost":true,"absolute_period_coverage_not_validated":true}));
  }
 }
 assert_eq!(count_requests,21816);assert_eq!(inventory.len(),204);assert_eq!(ledger.len(),29051);
 save(r("proof-sha256-after.json"),&json!(ledger));save(r("proof-inventory.json"),&json!(inventory));save(r("rows.json"),&json!(rows));save(r("profile-symbols.json"),&json!(symbol_rows));
 let mut cells=BTreeMap::new();let mut pairs=BTreeMap::new();let mut failed=vec![];
 let cases=read(r("cases.json"));
 for c in cases.as_array().unwrap(){let name=c["name"].as_str().unwrap();let mut arms=BTreeMap::new();
  for arm in ARMS{let rs=rows.iter().filter(|x|x["case"]==name&&x["variant"]==arm&&x["panel"]=="latency").collect::<Vec<_>>();assert_eq!(rs.len(),15);arms.insert(arm,json!({"median_ms":median(&rs.iter().map(|x|x["median_ms"].as_f64().unwrap()).collect::<Vec<_>>()),"CPU_ms_per_request":median(&rs.iter().map(|x|x["CPU_ms_per_request"].as_f64().unwrap()).collect::<Vec<_>>()),"counter_RSS_MiB_diagnostic":median(&rs.iter().map(|x|x["counter_RSS_MiB"].as_f64().unwrap()).collect::<Vec<_>>())}));}
  cells.insert(name.to_string(),json!(arms));let get=|arm:&str,b:u64|rows.iter().find(|x|x["case"]==name&&x["variant"]==arm&&x["panel"]=="latency"&&x["block"]==b).unwrap()["median_ms"].as_f64().unwrap();
  for(a,b)in[("OFF","accepted"),("ON","accepted"),("ON","OFF")]{let stats=interval(&(0..15).map(|i|get(a,i)/get(b,i)).collect::<Vec<_>>());let key=format!("{name}/{a}/{b}");
   let pass=if a=="OFF"{stats["low"].as_f64().unwrap()>=1./1.03&&stats["high"].as_f64().unwrap()<=1.03}else{stats["high"].as_f64().unwrap()<=1.03};if !pass{failed.push(format!("{key}/{}",if a=="OFF"{"calibration"}else{"nonregression"}));}
   if a=="ON"&&c["workers"]==32&&stats["high"].as_f64().unwrap()>1.{failed.push(format!("{key}/32-worker-upper-bound"));}
   pairs.insert(key,stats);
  }
 }
 let mut aggregates=BTreeMap::new();for control in["accepted","OFF"]{let values=(0..15).map(|b|{let l=cases.as_array().unwrap().iter().map(|c|pairs[&format!("{}/ON/{control}",c["name"].as_str().unwrap())]["ratios"][b].as_f64().unwrap().ln()).sum::<f64>();(l/4.).exp()}).collect::<Vec<_>>();let stats=interval(&values);let pass=stats["high"].as_f64().unwrap()<1.&&stats["wins"].as_u64().unwrap()>=10;if !pass{failed.push(format!("four-contexts/ON/{control}/gain"));}aggregates.insert(control,json!({"statistics":stats,"passed":pass}));}
 let diagnostics=counters.values().all(|v|v["qualified"]==true)&&profiles.values().all(|v|v["qualified"]==true);
 save(r("analysis.json"),&json!({"complete":true,"gate_passed":failed.is_empty(),"failed_checks":failed,"diagnostics_qualified":diagnostics,"SDK_requests":21816,"fresh_Gnark_exports":204,"proof_ledger":29051,"new_witness_solves":21816,"production_changed":false,"adopted":false,"goal_achieved":false,"core_tests":128,"prover_tests":8,"field_source_unchanged":true,"cells":cells,"pairs":pairs,"aggregates":aggregates,"counters":counters,"profiles":profiles,"limitations":["Initial4context screen, not all8shape adoption qualification","No new AWS/mobile/TEE/pressure/huge-circuit/concurrent-throughput result","RSS counters diagnostic, no precisepeak/minRAM claim","No localCPI or scheduler/caches/DRAM causal inference from flat samples"]}));
 println!("Complete4context SDK screen;204freshGnarkexports/ledger29051;{}failed checks;diagnostics={diagnostics}",failed.len());
}

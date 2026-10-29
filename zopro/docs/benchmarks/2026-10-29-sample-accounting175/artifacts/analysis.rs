fn sdk_pids(metadata:&str)->BTreeSet<u32> {
    metadata.lines().filter(|line|line.contains(SDK)).filter_map(|line| {
        let suffix=line.split_once("PERF_RECORD_MMAP2 ")?.1;
        Some(suffix.split('/').next().unwrap().parse::<u32>().unwrap())
    }).collect()
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"],"complete");
    assert_eq!(read(r("offline/result.json"))["status"],"complete");
    for(p,h) in read(r("pins.json")).as_object().unwrap() {assert_eq!(hash(p),h.as_str().unwrap());}
    let c=read(r("case.json")); let mut prior:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap(); assert_eq!(prior.len(),22210);
    let mut panels=Vec::new(); let mut inventory=Vec::new(); let mut all_symbols=BTreeMap::new();
    for j in read(r("jobs.json")).as_array().unwrap() {
        let n=j["name"].as_str().unwrap(); let event=j["event"].as_str().unwrap(); let mode=j["mode"].as_str().unwrap(); let workers=j["workers"].as_u64().unwrap();
        let stdout=fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap();
        let samples=stdout.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).map(|s|serde_json::from_str::<Value>(s).unwrap()).collect::<Vec<_>>();
        assert_eq!(samples.len(),151); for(i,s) in samples.iter().enumerate() {assert_eq!(s["round"],i);assert!(s["ms"].as_f64().unwrap()>0.);}
        let usage:Value=serde_json::from_str(stdout.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();assert_eq!(usage["workers"],workers);assert_eq!(usage["rounds"],151);assert_eq!(usage["priming"],3);
        let control:Value=serde_json::from_str(stdout.lines().find_map(|l|l.strip_prefix("CONTROL ")).unwrap()).unwrap(); assert_eq!(control["both_acks_before_SDK"],true);assert_eq!(control["order"].as_array().unwrap().len(),4);
        let attrs=fs::read_to_string(r(format!("offline/events-{n}/stdout.log"))).unwrap();
        for needle in [format!("{event}:u"),"CPU".into(),"TIME".into(),"PERIOD".into()] {assert!(attrs.contains(&needle));}
        if mode=="freq" {assert!(attrs.contains("199")&&attrs.contains("freq: 1"));} else {assert!(attrs.contains("16000000")&&!attrs.contains("freq: 1"));}
        let meta=fs::read_to_string(r(format!("offline/metadata-{n}/stdout.log"))).unwrap(); let mapped=sdk_pids(&meta);assert_eq!(mapped.len(),1);let pid=*mapped.first().unwrap();assert!(meta.contains("PERF_RECORD_FORK"));
        let loss=read(r(format!("offline/loss-{n}/stdout.log")));let zero_loss=loss["zero_lost_or_throttle"]==true;
        let csv=fs::read_to_string(r(format!("stat/{n}.csv"))).unwrap();let counter=csv.lines().filter_map(event_line).collect::<Vec<_>>();assert_eq!(counter.len(),1);assert_eq!(counter[0].0,format!("{event}:u"));let count=&counter[0].1;
        let mut per_tid=BTreeMap::<u32,(u64,u64)>::new(); let mut foreign=BTreeMap::<u32,(u64,u64)>::new();let mut symbols=BTreeMap::<String,(u64,u64)>::new();let mut categories=BTreeMap::<String,u64>::new();let(mut period,mut nsamples)=(0u64,0u64);
        for(p,t,cpu,weight,symbol) in fs::read_to_string(r(format!("offline/decode-{n}/stdout.log"))).unwrap().lines().filter_map(sample) {
            if p!=pid {let v=foreign.entry(p).or_default();v.0+=1;v.1+=weight;continue;}
            assert!((cpu as u64)<workers);if mode=="fixed" {assert_eq!(weight,16_000_000);}
            let v=per_tid.entry(t).or_default();v.0+=1;v.1+=weight;let v=symbols.entry(symbol.clone()).or_default();v.0+=1;v.1+=weight;
            *categories.entry(category(&symbol).into()).or_default()+=weight;period+=weight;nsamples+=1;
        }
        assert!(period>0&&nsamples>0);let foreign_period=foreign.values().map(|v|v.1).sum::<u64>();let foreign_share=foreign_period as f64/(foreign_period+period) as f64;
        let unknown=categories.get("unknown").copied().unwrap_or(0) as f64/period as f64;
        let attributed=zero_loss&&nsamples>=1000&&unknown<=0.05&&foreign_share<=0.001;
        let ratio=count["count"].as_f64().filter(|c|*c>0.).map(|c|period as f64/c);
        let accounting=attributed&&count["qualified"]==true&&ratio.is_some_and(|r|(0.98..=1.02).contains(&r));
        let symbol_rows=symbols.iter().map(|(s,(samples,period))|json!({"symbol":s,"samples":samples,"period":period})).collect::<Vec<_>>();all_symbols.insert(n.to_string(),json!(symbol_rows));
        let shares=categories.iter().map(|(cat,p)|(cat.clone(),json!({"period":p,"share":*p as f64/period as f64}))).collect::<BTreeMap<_,_>>();
        panels.push(json!({"name":n,"workers":workers,"event":event,"mode":mode,"block":j["block"],"SDK_PID":pid,"samples":nsamples,"SDK_sample_period":period,"stat_event":count,"sample_period_to_count":ratio,"sample_period_per_request":period as f64/151.,"stat_count_per_request":count["count"].as_f64().map(|c|c/151.),"attribution_qualified":attributed,"accounting_qualified":accounting,"zero_lost_or_throttle":zero_loss,"unknown_period_share":unknown,"foreign_period_share":foreign_share,"foreign_PIDs":foreign,"per_TID_samples_and_periods":per_tid,"categories":shares,"diagnostic_request_median_ms":median(&samples.iter().map(|s|s["ms"].as_f64().unwrap()).collect::<Vec<_>>()),"SDK_loop_usage":usage,"control":control}));
        let proof=r(format!("proofs/{n}/0.proof"));let public=r(format!("proofs/{n}/0.public"));let sha=hash(&proof);assert_eq!(fs::metadata(&proof).unwrap().len(),164);assert_eq!(hash(&public),c["public_sha256"]);
        let receipt=fs::read_to_string(r(format!("offline/verify-{n}/stdout.log"))).unwrap(); let lines=receipt.lines().filter_map(|l|l.strip_prefix("VERIFIED ")).collect::<Vec<_>>();assert_eq!(lines.len(),1);let fields=lines[0].split_whitespace().map(|s|s.split_once('=').unwrap()).collect::<BTreeMap<_,_>>();
        assert_eq!(fields["proof_sha256"],sha);assert_eq!(fields["public_sha256"],c["public_sha256"]);assert_eq!(fields["image_sha256"],c["image_sha256"]);assert_eq!(fields["vk_sha256"].len(),64);assert!(prior.insert(sha.clone()));
        inventory.push(json!({"name":n,"proof_sha256":sha,"public_sha256":hash(public),"verified":true}));
    }
    let mut aggregates=BTreeMap::new();
    for w in [16,32] {for event in ["cycles","instructions"] {for mode in ["freq","fixed"] {
        let rows=panels.iter().filter(|p|p["workers"]==w&&p["event"]==event&&p["mode"]==mode).collect::<Vec<_>>();assert_eq!(rows.len(),3);
        let ratios=rows.iter().filter_map(|r|r["sample_period_to_count"].as_f64()).collect::<Vec<_>>();
        let counters=rows.iter().filter_map(|r|r["stat_count_per_request"].as_f64()).collect::<Vec<_>>();
        aggregates.insert(format!("w{w}/{event}/{mode}"),json!({"ratio_median":if ratios.len()==3 {Some(median(&ratios))}else{None},"ratios":ratios,"count_per_request_median":if counters.len()==3 {Some(median(&counters))}else{None},"all_attribution_qualified":rows.iter().all(|p|p["attribution_qualified"]==true),"all_accounting_qualified":rows.iter().all(|p|p["accounting_qualified"]==true),"all_counters_qualified":rows.iter().all(|p|p["stat_event"]["qualified"]==true)}));
    }}}
    assert_eq!(prior.len(),22234);save(r("proof-inventory.json"),&json!(inventory));save(r("proof-sha256-after.json"),&json!(prior));save(r("profile-symbols.json"),&json!(all_symbols));
    save(r("analysis.json"),&json!({"complete":true,"diagnostic_only":true,"production_changed":false,"adopted":false,"new_latency_qualification":false,"goal_achieved":false,"SDK_requests":3696,"new_independent_exports":24,"prior_proof_count":22210,"independent_proof_count":22234,"all_attribution_qualified":panels.iter().all(|p|p["attribution_qualified"]==true),"all_accounting_qualified":panels.iter().all(|p|p["accounting_qualified"]==true),"panels":panels,"aggregates":aggregates}));
    println!("Analyzed24 simultaneous sample/count panels; ledger22234 persisted; no source adoption");
}
#[cfg(test)] mod accounting_tests {
    use super::*;
    #[test] fn sdk_mapping_ignores_cmdline_and_monitor() {
        let meta=format!("# cmdline {SDK}\n0 100: PERF_RECORD_MMAP2 17/17: {SDK}\n0 100: PERF_RECORD_MMAP2 19/19: /usr/bin/perf\n");
        assert_eq!(sdk_pids(&meta),BTreeSet::from([17]));
    }
    #[test] fn csv_coverage_and_count() {let(name,v)=event_line("1234;;cycles:u;50000;100.00;;").unwrap();assert_eq!(name,"cycles:u");assert_eq!(v["qualified"],true);assert_eq!(v["count"],1234.);}
    #[test] fn unsupported_not_qualified() {let(_,v)=event_line("<not supported>;;cycles:u;0;100.00;;").unwrap();assert_eq!(v["qualified"],false);assert!(v["count"].is_null());}
    #[test] fn multiplex_not_qualified() {let(_,v)=event_line("1234;;cycles:u;50000;50.00;;").unwrap();assert_eq!(v["qualified"],false);}
}

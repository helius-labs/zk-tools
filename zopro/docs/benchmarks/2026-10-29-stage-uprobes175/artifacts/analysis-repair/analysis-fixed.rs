#[derive(Clone,Debug)]struct Event{pid:u64,tid:u64,cpu:u32,ns:u64,label:String,enter:bool}
#[derive(Clone,Debug)]struct Span{pid:u64,tid:u64,cpu:u32,label:String,start:u64,end:u64}
fn parse_event(line:&str,group:&str)->Option<Event>{
    if line.trim_start().starts_with('#'){return None;}
    let fields=line.split_whitespace().collect::<Vec<_>>();if fields.len()<4{return None;}
    let event=fields.iter().position(|s|s.starts_with(&format!("{group}:")))?;
    assert_eq!(event,3,"unexpected perf event layout: {line}");
    let(pid,tid)=fields[0].split_once('/').unwrap();let cpu=fields[1].trim_matches(['[',']']).parse().unwrap();
    let time=fields[2].trim_end_matches(':');let(sec,fraction)=time.split_once('.').unwrap();assert!(fraction.len()<=9);
    let ns=sec.parse::<u64>().unwrap()*1_000_000_000+fraction.parse::<u64>().unwrap()*10u64.pow(9-fraction.len()as u32);
    let name=fields[3].trim_end_matches(':').strip_prefix(&format!("{group}:")).unwrap();
    let(label,enter)=if let Some(s)=name.strip_suffix("_enter"){(s,true)}else{(name.strip_suffix("_exit").unwrap(),false)};
    Some(Event{pid:pid.parse().unwrap(),tid:tid.parse().unwrap(),cpu,ns,label:label.into(),enter})
}
fn pair_events(events:&[Event])->Vec<Span>{
    let mut stacks=BTreeMap::<(u64,String),Vec<Event>>::new();let mut spans=Vec::new();
    for e in events{let key=(e.tid,e.label.clone());let stack=stacks.entry(key).or_default();if e.enter{stack.push(e.clone());}else{let start=stack.pop().expect("unmatched return");assert_eq!(start.pid,e.pid);assert!(e.ns>=start.ns);spans.push(Span{pid:e.pid,tid:e.tid,cpu:start.cpu,label:e.label.clone(),start:start.ns,end:e.ns});}}
    assert!(stacks.values().all(Vec::is_empty),"unmatched entry");spans
}
fn median(values:&[f64])->f64{assert!(!values.is_empty());let mut s=values.to_vec();s.sort_by(f64::total_cmp);s[s.len()/2]}
fn verified_receipt(text:&str,proof:&str,public:&str,image:&str){let lines=text.lines().filter_map(|l|l.strip_prefix("VERIFIED ")).collect::<Vec<_>>();assert_eq!(lines.len(),1);let fields=lines[0].split_whitespace().map(|s|s.split_once('=').unwrap()).collect::<BTreeMap<_,_>>();assert_eq!(fields["proof_sha256"],proof);assert_eq!(fields["public_sha256"],public);assert_eq!(fields["image_sha256"],image);assert_eq!(fields["vk_sha256"].len(),64);}
fn output(base:&str,name:&str)->(Vec<f64>,Value){let s=fs::read_to_string(r(format!("{base}/{name}/stdout.log"))).unwrap();let times=s.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).map(|l|serde_json::from_str::<Value>(l).unwrap()["ms"].as_f64().unwrap()).collect::<Vec<_>>();let summary=s.lines().filter_map(|l|l.strip_prefix("SUMMARY ")).map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();assert_eq!(summary.len(),1);(times,summary[0].clone())}
fn finish(){
    for dir in["preflight","runs","offline"]{assert_eq!(read(r(format!("{dir}/result.json")))["status"],"complete");}for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let group=read(r("targets.json"))["group"].as_str().unwrap().to_string();let c=read(r("case.json"));let mut prior:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),22144);
    let mut proof_names=(vec!["pre-w1".to_string(),"pre-w16".into(),"pre-w32".into()]).into_iter().map(|n|("preflight".to_string(),n)).collect::<Vec<_>>();
    let jobs=read(r("jobs.json"));for j in jobs.as_array().unwrap(){proof_names.push(("offline".into(),j["name"].as_str().unwrap().into()));}
    let mut inventory=Vec::new();for(base,n)in&proof_names{let verify=fs::read_to_string(r(format!("{base}/verify-{n}/stdout.log"))).unwrap();let proof=r(format!("proofs/{n}/0.proof"));let public=r(format!("proofs/{n}/0.public"));assert_eq!(fs::metadata(&proof).unwrap().len(),164);assert_eq!(hash(&public),c["public_sha256"]);let sha=hash(&proof);verified_receipt(&verify,&sha,c["public_sha256"].as_str().unwrap(),c["image_sha256"].as_str().unwrap());assert!(prior.insert(sha.clone()),"duplicate proof {n}");inventory.push(json!({"name":n,"proof_sha256":sha,"public_sha256":hash(&public),"bytes":164,"Gnark_passed":true}));}assert_eq!(prior.len(),22189);
    let mut observations=Vec::new();let mut traces=BTreeMap::new();let mut stage_rows=BTreeMap::<String,Vec<f64>>::new();
    for j in jobs.as_array().unwrap(){let name=j["name"].as_str().unwrap();let(times,summary)=output("runs",name);assert_eq!(times.len(),31);assert_eq!(summary["rounds"],31);assert_eq!(summary["priming"],3);assert_eq!(summary["workers"],j["workers"]);
        observations.push(json!({"name":name,"workers":j["workers"],"mode":j["mode"],"block":j["block"],"median_ms":median(&times),"times_ms":times,"summary":summary}));
        if j["mode"]!="traced"{continue;}
        assert_eq!(read(r(format!("offline/loss-{name}/stdout.log")))["zero_lost"],true);
        let attrs=fs::read_to_string(r(format!("offline/events-{name}/stdout.log"))).unwrap();assert!(attrs.contains("CPU")&&attrs.contains("TIME")&&attrs.contains("RAW"));
        let text=fs::read_to_string(r(format!("offline/decode-{name}/stdout.log"))).unwrap();let events=text.lines().filter_map(|l|parse_event(l,&group)).collect::<Vec<_>>();let pids=events.iter().map(|e|e.pid).collect::<BTreeSet<_>>();assert_eq!(pids.len(),1);let mut spans=pair_events(&events);spans.sort_by_key(|s|(s.start,s.end));
        let sdk=spans.iter().filter(|s|s.label=="sdk").cloned().collect::<Vec<_>>();assert_eq!(sdk.len(),31,"SDK coverage {name}");for p in sdk.windows(2){assert!(p[0].end<=p[1].start);}
        let mut per_request=Vec::new();for(root_index,root)in sdk.iter().enumerate(){let inside=spans.iter().filter(|s|s.start>=root.start&&s.end<=root.end).collect::<Vec<_>>();let mut stages=BTreeMap::<String,Vec<Value>>::new();for s in inside{let ms=(s.end-s.start)as f64/1e6;stage_rows.entry(format!("w{}/{}",j["workers"],s.label)).or_default().push(ms);stages.entry(s.label.clone()).or_default().push(json!({"start_ms":(s.start-root.start)as f64/1e6,"end_ms":(s.end-root.start)as f64/1e6,"duration_ms":ms,"tid":s.tid,"entry_cpu":s.cpu}));}
            assert_eq!(stages.get("solver").unwrap().len(),1);assert_eq!(stages.get("product").unwrap().len(),1);assert_eq!(stages.get("canonical").unwrap().len(),1);
            let g2_end=stages.get("g2").map(|v|v.iter().map(|v|v["end_ms"].as_f64().unwrap()).fold(0.,f64::max));let g1_end=stages.get("g1").map(|v|v.iter().map(|v|v["end_ms"].as_f64().unwrap()).fold(0.,f64::max));let canonical_end=stages["canonical"][0]["end_ms"].as_f64().unwrap();
            let row=json!({"round":root_index,"SDK_interval_ms":(root.end-root.start)as f64/1e6,"stages":stages,"canonical_end_ms":canonical_end,"G1_last_end_ms":g1_end,"G2_last_end_ms":g2_end});per_request.push(row);
        }
        for s in &spans{assert_eq!(sdk.iter().filter(|root|s.start>=root.start&&s.end<=root.end).count(),1,"span outside SDK {name}");}
        traces.insert(name.to_string(),json!({"events":events.len(),"spans":spans.len(),"SDK_intervals":31,"pids":pids,"zero_lost":true,"entry_return_complete":true,"raw_sha256":hash(r(format!("perf/{name}.data"))),"requests":per_request}));
    }
    let mut overhead=BTreeMap::new();for w in[1,16,32]{let get=|b:usize,mode:&str|observations.iter().find(|o|o["name"]==format!("b{b}-w{w}-{mode}")).unwrap()["median_ms"].as_f64().unwrap();let ratios=(0..7).map(|b|get(b,"traced")/get(b,"plain")).collect::<Vec<_>>();let low=ratios.iter().copied().fold(f64::INFINITY,f64::min);let high=ratios.iter().copied().fold(0.,f64::max);overhead.insert(w.to_string(),json!({"paired_ratio":median(&ratios),"low":low,"high":high,"within_factor1_03":low>=1./1.03&&high<=1.03,"plain_median_ms":median(&(0..7).map(|b|get(b,"plain")).collect::<Vec<_>>()),"traced_median_ms":median(&(0..7).map(|b|get(b,"traced")).collect::<Vec<_>>()),"ratios":ratios}));}
    let stages=stage_rows.into_iter().map(|(k,v)|(k,json!({"count":v.len(),"median_inclusive_ms":median(&v)}))).collect::<BTreeMap<_,_>>();
    save(r("proof-inventory.json"),&json!(inventory));save(r("proof-sha256-after.json"),&json!(prior));
    save(r("analysis.json"),&json!({"complete":true,"diagnostic_only":true,"source_rebuilds":0,"SDK_binary_unchanged":true,"production_changed":false,"adopted":false,"goal_achieved":false,"SDK_requests":1440,"new_independent_exports":45,"prior_proof_count":22144,"independent_proof_count":22189,"overhead":overhead,"stages":stages,"traces":traces,"observations":observations,"no_new_latency_qualification":true}));println!("Qualified21trace processes/651SDK intervals;45newGnark;ledger22189;diagnosticONLY");
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn ignore_perf_header(){assert!(parse_event("# cmdline : perf record -e group:* -- program", "group").is_none());}
    #[test]fn parse_nanosecond_cpu_tid(){let e=parse_event("17/22 [003] 123.000000009: group:solver_enter:","group").unwrap();assert_eq!((e.pid,e.tid,e.cpu,e.ns),(17,22,3,123000000009));assert!(e.enter);assert_eq!(e.label,"solver");}
    #[test]fn reject_other_group(){assert!(parse_event("17/22 [003] 123.1: other:solver_enter:","group").is_none());}
    #[test]fn nested_and_parallel_returns(){let input=["1/2 [000] 1.000000001: g:a_enter:","1/3 [001] 1.000000002: g:a_enter:","1/2 [000] 1.000000003: g:a_enter:","1/2 [000] 1.000000004: g:a_exit:","1/3 [001] 1.000000005: g:a_exit:","1/2 [000] 1.000000006: g:a_exit:"];let e=input.map(|s|parse_event(s,"g").unwrap());let s=pair_events(&e);assert_eq!(s.iter().map(|s|s.end-s.start).collect::<Vec<_>>(),[1,3,5]);}
    #[test]#[should_panic]fn unmatched_entry_fails(){pair_events(&[parse_event("1/2 [000] 1.0: g:a_enter:","g").unwrap()]);}
    #[test]#[should_panic]fn unmatched_return_fails(){pair_events(&[parse_event("1/2 [000] 1.0: g:a_exit:","g").unwrap()]);}
    #[test]#[should_panic]fn misbound_verifier_receipt_fails(){verified_receipt(&format!("VERIFIED proof_sha256=wrong public_sha256=p image_sha256=i vk_sha256={}","0".repeat(64)),"right","p","i");}
}

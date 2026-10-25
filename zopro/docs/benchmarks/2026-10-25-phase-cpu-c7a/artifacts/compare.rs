//! Validate the fixed primary comparison and derive descriptive paired results.
use serde_json::{Value,json};
use std::{collections::{BTreeMap,BTreeSet},fs,path::PathBuf};
fn read(p: impl AsRef<std::path::Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn text<'a>(v:&'a Value,k:&str)->&'a str {v[k].as_str().unwrap()}
fn number(v:&Value,k:&str)->f64 {v[k].as_f64().unwrap()}
fn median(mut values:Vec<f64>)->f64 {
    assert!(!values.is_empty()&&values.iter().all(|n|n.is_finite()));
    values.sort_by(f64::total_cmp);let i=values.len()/2;
    if values.len().is_multiple_of(2){(values[i-1]+values[i])/2.}else{values[i]}
}
fn counters(input:&str)->BTreeMap<String,f64> {
    let mut result=BTreeMap::new();
    for line in input.lines().filter(|l|!l.is_empty()&&!l.starts_with('#')) {
        let fields=line.split(';').collect::<Vec<_>>();
        if fields.len()<5 {continue;}
        let event=fields[2].trim();
        if !["cycles:u","instructions:u","branches:u","branch-misses:u"].contains(&event){continue;}
        let count=fields[0].trim().parse::<f64>().unwrap();
        let running=fields[4].trim().trim_end_matches('%').parse::<f64>().unwrap();
        assert!(count>0.&&count.is_finite());
        assert!((99.0..=100.0).contains(&running),"multiplexed event {event}: {running}");
        assert!(result.insert(event.into(),count).is_none(),"duplicate event");
    }
    assert_eq!(result.len(),4);result
}
fn main(){
    let args=std::env::args_os().skip(1).map(PathBuf::from).collect::<Vec<_>>();assert_eq!(args.len(),2);
    let root=&args[0];let output=&args[1];fs::create_dir(output).unwrap();
    let data=root.join("remote-results/output");
    let jobs=read(root.join("context/comparison-jobs.json"));
    let mut rows=Vec::new();let mut names=BTreeSet::new();let mut sdk=0u64;
    for job in jobs["jobs"].as_array().unwrap(){
        let name=text(job,"name");assert!(names.insert(name.to_owned()));
        let directory=data.join("comparison").join(name);
        assert_eq!(read(directory.join("exit.json"))["success"],true);
        assert_eq!(read(data.join("comparison").join(format!("verify-{name}")).join("exit.json"))["success"],true);
        let stdout=fs::read_to_string(directory.join("stdout.log")).unwrap();
        let samples=stdout.lines().filter_map(|l|l.strip_prefix("SAMPLE "))
            .map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();
        let summaries=stdout.lines().filter_map(|l|l.strip_prefix("SUMMARY "))
            .map(|l|serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();
        assert_eq!(summaries.len(),1);let summary=&summaries[0];
        let rounds=job["rounds"].as_u64().unwrap();
        assert_eq!(samples.len()as u64,rounds);assert_eq!(summary["rounds"],job["rounds"]);
        assert_eq!(summary["priming"],3);sdk+=rounds+3;
        let engine=text(job,"engine");let workers=job["workers"].as_u64().unwrap();
        if engine=="aeglos"{
            assert_eq!(summary["threads"],workers);
            assert!(samples.iter().all(|s|s["cache_hit"]==true));
        }else{assert_eq!(engine,"accepted");assert_eq!(summary["workers"],workers);}
        let sample_ms=samples.iter().enumerate().map(|(index,s)|{
            assert_eq!(s["round"].as_u64(),Some(index as u64));
            let time=s["ms"].as_f64().or_else(||s["verified_ms"].as_f64()).unwrap();
            assert!(time>0.&&time.is_finite());time
        }).collect::<Vec<_>>();
        let user=number(summary,"user_cpu_ms");let system=number(summary,"system_cpu_ms");
        let mut row=json!({"name":name,"kind":job["kind"],"block":job["block"],"workers":workers,
            "engine":engine,"rounds":rounds,"median_ms":median(sample_ms.clone()),"samples_ms":sample_ms,
            "cpu_ms_per_proof":(user+system)/rounds as f64,"user_cpu_ms_per_proof":user/rounds as f64,
            "system_cpu_ms_per_proof":system/rounds as f64,
            "mean_occupied_cpus":(user+system)/number(summary,"loop_wall_ms"),
            "voluntary_switches_per_proof":number(summary,"voluntary_switches")/rounds as f64,
            "minor_faults_per_proof":number(summary,"minor_faults")/rounds as f64,
            "maxrss_mib":number(summary,"whole_process_maxrss_kib")/1024.});
        if text(job,"kind")=="stat"{
            let values=counters(&fs::read_to_string(data.join("counters").join(name).join("stat.csv")).unwrap());
            row["ipc"]=json!(values["instructions:u"]/values["cycles:u"]);
            row["cycles_per_proof"]=json!(values["cycles:u"]/rounds as f64);
            row["instructions_per_proof"]=json!(values["instructions:u"]/rounds as f64);
            row["branch_miss_fraction"]=json!(values["branch-misses:u"]/values["branches:u"]);
            row["estimated_user_ghz"]=json!(values["cycles:u"]/(user*1e6));
            row["hardware_counters"]=json!(values);
        }
        rows.push(row);
    }
    assert_eq!(rows.len(),36);assert_eq!(sdk,4284);
    let mut aggregate=BTreeMap::new();
    for workers in [32,64,96]{
        let mut engines=BTreeMap::new();
        for engine in ["accepted","aeglos"]{
            let selected=|kind:&str|rows.iter().filter(|r|r["workers"]==workers&&r["engine"]==engine&&r["kind"]==kind).collect::<Vec<_>>();
            let latency=selected("latency");let stat=selected("stat");assert_eq!(latency.len(),3);assert_eq!(stat.len(),3);
            let mut metrics=BTreeMap::<String,Value>::new();
            metrics.insert("latency_ms".into(),json!(median(latency.iter().map(|r|number(r,"median_ms")).collect())));
            metrics.insert("latency_maxrss_mib".into(),json!(latency.iter().map(|r|number(r,"maxrss_mib")).max_by(f64::total_cmp).unwrap()));
            for key in ["cpu_ms_per_proof","user_cpu_ms_per_proof","system_cpu_ms_per_proof","mean_occupied_cpus",
                "voluntary_switches_per_proof","minor_faults_per_proof","ipc","cycles_per_proof",
                "instructions_per_proof","branch_miss_fraction","estimated_user_ghz"]{
                metrics.insert(key.into(),json!(median(stat.iter().map(|r|number(r,key)).collect())));
            }
            engines.insert(engine.to_owned(),json!(metrics));
        }
        let ratios=(0..3).map(|block|{
            let time=|engine:&str|number(rows.iter().find(|r|r["workers"]==workers&&r["engine"]==engine&&r["kind"]=="latency"&&r["block"]==block).unwrap(),"median_ms");
            time("accepted")/time("aeglos")
        }).collect::<Vec<_>>();
        aggregate.insert(workers.to_string(),json!({"engines":engines,"paired_zopro_over_aeglos":median(ratios.clone()),"paired_ratios":ratios}));
    }
    let phase=read(data.join("perf/result.json"));
    assert_eq!(phase["status"],"complete");assert_eq!(phase["profiles"],54);assert_eq!(phase["distinct_verified_exports"],108);
    let phase_rows=read(data.join("perf/rows.json"));
    let phase_sdk=phase_rows.as_array().unwrap().iter().map(|r|r["samples_ms"].as_array().unwrap().len()as u64+3).sum::<u64>();
    assert_eq!(phase_sdk,4374);
    let profiles=phase_rows.as_array().unwrap().iter().filter(|r|r["phase"]=="profile").collect::<Vec<_>>();
    assert_eq!(profiles.len(),54);
    assert!(profiles.iter().all(|r|r["profile"]["lost_records"]==0&&r["profile"]["sampled_pids"].as_array().unwrap().len()==1));
    let samples=profiles.iter().map(|r|r["profile"]["samples"].as_u64().unwrap()).sum::<u64>();
    let result=json!({"status":"verified fixed comparison","adopted":false,"goal_achieved":false,
        "phase_matrix_sdk":phase_sdk,"comparison_sdk":sdk,"remote_preflight_sdk":28,"remote_total_sdk":phase_sdk+sdk+28,
        "distinct_remote_exports":151,"proof_public_members":302,"profiles":54,"profile_samples":samples,
        "lost_records":0,"counter_running_percent_min":99,"aggregates":aggregate,
        "phase_matrix_latencies":phase["latency"],"limits":"Descriptive current build comparison. Instrumented profiles and counters do not replace unprofiled latency. Estimated GHz derives from cycles/uCPU time. Source/build/kernel differ from older AWS reports. No current Aeglos54, phone, TEE, large-circuit or minimumRAM claim."});
    fs::write(output.join("rows.json"),serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
    fs::write(output.join("result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!("COMPARISON_VALIDATED SDK={} EXPORTS=151 SAMPLES={samples}",phase_sdk+sdk+28);
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn counter_parser_requires_all_four_events_without_multiplexing(){
        let input="100;;cycles:u;1;100.00;;\n150;;instructions:u;1;100.00;;\n20;;branches:u;1;100.00;;\n1;;branch-misses:u;1;100.00;;\n";
        assert_eq!(counters(input)["instructions:u"],150.);
        assert!(std::panic::catch_unwind(||counters(&input.replace("100.00","98.00"))).is_err());
        assert!(std::panic::catch_unwind(||counters("100;;cycles:u;1;100.00;;\n")).is_err());
    }
    #[test]fn block_medians_use_all_processes_and_reject_nonfinite_inputs(){
        assert_eq!(median(vec![12.652445,12.671325,12.659135]),12.659135);
        assert_eq!(median(vec![2.,4.]),3.);
        assert!(std::panic::catch_unwind(||median(vec![f64::NAN])).is_err());
    }
}

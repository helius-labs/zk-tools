use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};

fn median(mut values: Vec<f64>) -> f64 {
    assert_eq!(values.len(), 3);
    assert!(values.iter().all(|x| x.is_finite()));
    values.sort_by(f64::total_cmp);
    values[1]
}

fn main() {
    let input: Vec<Value> = serde_json::from_slice(&fs::read("host-rows.json").unwrap()).unwrap();
    assert_eq!(input.len(),114);
    let mut results=Vec::new();
    for shape in ["1x2","2x2","merge-54x1"] {
        for workers in [32,96] {
            let rows: Vec<_> = input.iter().filter(|r| r["shape"]==shape && r["workers"]==workers && r["phase"]=="latency").collect();
            assert_eq!(rows.len(),12);
            let values = |arm: &str, field: &str| -> Vec<f64> {
                (0..3).map(|block| rows.iter().find(|r| r["block"]==block && r["arm"]==arm).unwrap()[field].as_f64().unwrap()).collect()
            };
            let graph = |arm: &str| -> BTreeMap<String,u64> {
                let name = format!("preflight-{shape}-w{workers}-{arm}-plain-b00");
                let log = fs::read_to_string(format!("remote/output/preflight-run/{name}/stderr.log")).unwrap();
                let mut traces = log.lines().filter_map(|line|line.strip_prefix("READY_SOLVER "));
                let trace=traces.next().unwrap();
                assert!(traces.next().is_none());
                trace.split_whitespace().map(|token|{
                    let (key,value)=token.split_once('=').unwrap();
                    (key.to_owned(),value.parse().unwrap())
                }).collect()
            };
            let old=graph("OLD");
            let new=graph("ON");
            let graph_ok=["leaves","edges","roots"].iter().all(|&k| old[k]==new[k]) && new["peak_charge"] < old["peak_charge"];
            let paired=|arm,control,field| values(arm,field).into_iter().zip(values(control,field)).map(|(a,b)|a/b).collect::<Vec<_>>();
            let peak=|arm|values(arm,"lifetime_RSS_MiB").into_iter().reduce(f64::max).unwrap();
            let mut arms=BTreeMap::new();
            let mut ratios=BTreeMap::new();
            for arm in ["REF","OLD","OFF","ON"] {
                let means=values(arm,"mean_ms");
                arms.insert(arm,json!({"mean_ms":median(means.clone()),"block_means_ms":means,"CPU_ms":median(values(arm,"CPU_ms_per_request")),"peak_RSS_MiB":peak(arm)}));
            }
            for control in ["OLD","REF","OFF"] {
                let times=paired("ON",control,"mean_ms");
                let cpu=paired("ON",control,"CPU_ms_per_request");
                ratios.insert(control,json!({"latency":median(times.clone()),"latency_blocks":times,"cpu":median(cpu.clone()),"cpu_blocks":cpu}));
            }
            let cal=median(paired("OFF","REF","mean_ms"));
            let calibration_ok=(1./1.03..=1.03).contains(&cal);
            let rss_ok=peak("ON")<=peak("OLD")*1.10+8.;
            let latency_ok=ratios["OLD"]["latency"].as_f64().unwrap()<=1.03;
            let attributed=calibration_ok && ["REF","OFF"].iter().all(|&k|
                ratios[k]["latency_blocks"].as_array().unwrap().iter().all(|v|v.as_f64().unwrap()<1.) && ratios[k]["cpu"].as_f64().unwrap()<=1.);
            results.push(json!({"shape":shape,"workers":workers,"arms":arms,"ON_over_control":ratios,"old_graph":old,"new_graph":new,"graph_ok":graph_ok,"old_RSS_guard":rss_ok,"old_latency_guard":latency_ok,"construction_screen":graph_ok&&rss_ok&&latency_ok,"OFF_REF":cal,"calibration_ok":calibration_ok,"primary96_readiness":if shape=="1x2" && workers==96{Some(attributed)}else{None}}));
        }
    }
    fs::write("comparisons.json",serde_json::to_vec_pretty(&results).unwrap()).unwrap();
}

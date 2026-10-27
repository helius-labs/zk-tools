use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const R: &str = "/tmp/zopro-g1-reverse-overlap";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn median(mut x: Vec<f64>) -> f64 {
    assert!(!x.is_empty());
    x.sort_by(f64::total_cmp);
    x[x.len() / 2]
}
fn gm(x: &[f64]) -> f64 {
    assert!(!x.is_empty() && x.iter().all(|x| x.is_finite() && *x > 0.));
    (x.iter().map(|x| x.ln()).sum::<f64>() / x.len() as f64).exp()
}
fn paired(a: &[f64], b: &[f64]) -> Value {
    assert_eq!(a.len(), 7);
    assert_eq!(b.len(), 7);
    let ratios: Vec<_> = a.iter().zip(b).map(|(a, b)| a / b).collect();
    json!({"geometric_ratio":gm(&ratios),"median_ratio":median(ratios.clone()),"wins":ratios.iter().filter(|x|**x<1.).count(),"block_ratios":ratios})
}
fn values(rows: &[Value], name: &str, mode: &str, key: &str) -> Vec<f64> {
    (0..7)
        .map(|block| {
            rows.iter()
                .find(|r| {
                    r["phase"] == "latency"
                        && r["case"] == name
                        && r["stage"]["block"] == block
                        && r["stage"]["mode"] == mode
                })
                .unwrap()["stage"][key]
                .as_f64()
                .unwrap()
        })
        .collect()
}
fn main() {
    let r = Path::new(R);
    assert_eq!(read(r.join("measure/result.json"))["status"], "complete");
    for (p, h) in read(r.join("measurement-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut rows = Vec::new();
    let mut receipts = BTreeMap::new();
    let mut profile_summaries = Vec::new();
    for job in read(r.join("jobs.json")).as_array().unwrap() {
        let name = job["name"].as_str().unwrap();
        let text = fs::read_to_string(r.join("measure").join(name).join("stdout.log")).unwrap();
        let receipt: Value = serde_json::from_str(
            text.lines()
                .find_map(|s| s.strip_prefix("AFFINITY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["before"], json!([2]));
        assert_eq!(receipt["after"], json!([2]));
        assert_eq!(receipt["cpu"], 2);
        let tid = receipt["tid"].as_u64().unwrap();
        receipts.insert(name.to_string(), receipt);
        let stages: Vec<Value> = text
            .lines()
            .filter_map(|s| s.strip_prefix("STAGE "))
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        let latency = job["phase"] == "latency";
        assert_eq!(stages.len(), if latency { 21 } else { 1 });
        let mut seen = BTreeSet::new();
        for stage in stages {
            assert_eq!(stage["iterations"], if latency { 1024 } else { 131072 });
            assert_eq!(stage["timed_allocation_calls"], 0);
            assert_eq!(stage["complete_storage_and_counts_correct"], true);
            assert_eq!(
                &stage["pairs"],
                if latency {
                    &job["case"]["pairs"]
                } else {
                    &job["pairs"]
                }
            );
            assert!(seen.insert((
                stage["block"].as_u64().unwrap(),
                stage["mode"].as_str().unwrap().to_string()
            )));
            rows.push(json!({"phase":job["phase"],"case":name,"stage":stage}));
        }
        if !latency {
            let samples = fs::read_to_string(
                r.join("measure")
                    .join(format!("samples-{name}"))
                    .join("stdout.log"),
            )
            .unwrap();
            let mut periods = BTreeMap::<String, u64>::new();
            let mut pids = BTreeSet::new();
            let mut tids = BTreeSet::new();
            let mut count = 0usize;
            let mut lost = 0usize;
            for line in samples.lines() {
                if line.contains("LOST") {
                    lost += 1;
                    continue;
                }
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields.is_empty() {
                    continue;
                }
                assert!(fields.len() >= 7, "bad sample: {line}");
                let (pid, sample_tid) = fields[1].split_once('/').unwrap();
                let pid: u64 = pid.parse().unwrap();
                let sample_tid: u64 = sample_tid.parse().unwrap();
                let period: u64 = fields[3].parse().unwrap();
                assert_eq!(fields[4], "cycles:u:");
                let symbol = fields[6..].join(" ");
                pids.insert(pid);
                tids.insert(sample_tid);
                *periods.entry(symbol).or_default() += period;
                count += 1;
            }
            assert_eq!(lost, 0);
            assert_eq!(pids, BTreeSet::from([tid]));
            assert_eq!(tids, BTreeSet::from([tid]));
            assert!(count > 0);
            profile_summaries.push(json!({"case":name,"samples":count,"lost":lost,"pids":pids,"tids":tids,"period":periods.values().sum::<u64>(),"symbol_periods":periods}));
        }
    }
    assert_eq!(rows.len(), 844);
    save(r.join("rows.json"), &json!(rows));
    save(r.join("affinity.json"), &json!(receipts));
    save(r.join("profile-summary.json"), &json!(profile_summaries));
    let mut cases = BTreeMap::new();
    let mut calibration = Vec::new();
    let mut regressions = Vec::new();
    for case in read(r.join("cases.json")).as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let a = values(&rows, name, "control-a", "kernel_ns_per_layer");
        let b = values(&rows, name, "control-b", "kernel_ns_per_layer");
        let c = values(&rows, name, "overlap", "kernel_ns_per_layer");
        let cal = paired(&b, &a);
        let med = cal["median_ratio"].as_f64().unwrap();
        let geo = cal["geometric_ratio"].as_f64().unwrap();
        if !(1. / 1.03..=1.03).contains(&med) {
            calibration.push(format!("{name} B/A median{med}"));
        }
        if !(1. / 1.05..=1.05).contains(&geo) {
            calibration.push(format!("{name} B/A geometric{geo}"));
        }
        let mut pairs = BTreeMap::new();
        for (control, times) in [("control-a", &a), ("control-b", &b)] {
            let p = paired(&c, times);
            if p["geometric_ratio"].as_f64().unwrap() > 1.02 {
                regressions.push(format!(
                    "{name} overlap/{control} geometric{}",
                    p["geometric_ratio"]
                ));
            }
            pairs.insert(control, p);
        }
        let mut modes = BTreeMap::new();
        for mode in ["control-a", "control-b", "overlap"] {
            let mut resources = serde_json::Map::new();
            for key in [
                "kernel_ns_per_layer",
                "caller_ns_per_layer",
                "user_cpu_ms",
                "system_cpu_ms",
                "minor_faults",
                "voluntary_switches",
                "involuntary_switches",
                "whole_process_maxrss_kib",
                "record_capacity_bytes",
            ] {
                resources.insert(key.into(), json!(median(values(&rows, name, mode, key))));
            }
            modes.insert(mode, Value::Object(resources));
        }
        cases.insert(
            name.to_string(),
            json!({"case":case,"calibration":cal,"pairs":pairs,"resource_medians":modes}),
        );
    }
    let mut aggregates = BTreeMap::new();
    let mut gain_failures = Vec::new();
    for control in ["control-a", "control-b"] {
        let ratios: Vec<_> = (0..7)
            .map(|b| {
                gm(&cases
                    .values()
                    .filter(|c| {
                        c["case"]["signed"] == true && c["case"]["pairs"].as_u64().unwrap() >= 200
                    })
                    .map(|c| c["pairs"][control]["block_ratios"][b].as_f64().unwrap())
                    .collect::<Vec<_>>())
            })
            .collect();
        let geo = gm(&ratios);
        let wins = ratios.iter().filter(|x| **x < 1.).count();
        if geo >= 1. || wins < 4 {
            gain_failures.push(format!(
                "targeted_signed_aggregate/{control}={geo},wins{wins}"
            ));
        }
        aggregates.insert(control,json!({"geometric_ratio":geo,"median_ratio":median(ratios.clone()),"wins":wins,"block_ratios":ratios}));
    }
    let advances = calibration.is_empty() && regressions.is_empty() && gain_failures.is_empty();
    let out = json!({"component_advances":advances,"calibration_failures":calibration,"nonregression_failures":regressions,"aggregate_failures":gain_failures,"targeted_signed_aggregate":aggregates,"cases":cases,"latency_cases":40,"latency_stages":840,"profile_processes":4,"cycle_samples":profile_summaries.iter().map(|x|x["samples"].as_u64().unwrap()).sum::<u64>(),"timed_allocation_calls":0,"SDK_requests":0,"Gnark_exports":0,"next_prior_count":8310,"adopted":false,"goal_achieved":false});
    save(r.join("analysis.json"), &out);
    println!(
        "component_advances={advances}; calibration{} nonreg{} gain{};0SDK/Gnark; prior8310",
        out["calibration_failures"].as_array().unwrap().len(),
        out["nonregression_failures"].as_array().unwrap().len(),
        out["aggregate_failures"].as_array().unwrap().len()
    );
}

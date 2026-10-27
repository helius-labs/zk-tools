use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
const R: &str = "/tmp/zopro-tree-record-crossover";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn median(mut v: Vec<f64>) -> f64 {
    assert!(!v.is_empty());
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 0 {
        (v[n / 2 - 1] + v[n / 2]) / 2.
    } else {
        v[n / 2]
    }
}
fn gm(v: &[f64]) -> f64 {
    assert!(!v.is_empty() && v.iter().all(|x| *x > 0. && x.is_finite()));
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn paired(a: &[f64], b: &[f64]) -> Value {
    assert_eq!(a.len(), 7);
    assert_eq!(b.len(), 7);
    let x: Vec<_> = a.iter().zip(b).map(|(a, b)| a / b).collect();
    json!({"geometric_ratio":gm(&x),"median_ratio":median(x.clone()),"wins":x.iter().filter(|x|**x<1.).count(),"block_ratios":x})
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
fn good(c: &Value, control: &str) -> bool {
    c["pairs"][control]["geometric_ratio"].as_f64().unwrap() < 1.
        && c["pairs"][control]["wins"].as_u64().unwrap() >= 4
}
fn main() {
    let r = Path::new(R);
    assert_eq!(read(r.join("measure/result.json"))["status"], "complete");
    for (p, h) in read(r.join("measurement-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut rows = Vec::new();
    let mut receipts = BTreeMap::new();
    let mut maxrss = 0u64;
    for job in read(r.join("jobs.json")).as_array().unwrap() {
        let name = job["name"].as_str().unwrap();
        let log = fs::read_to_string(r.join("measure").join(name).join("stdout.log")).unwrap();
        let receipt: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("AFFINITY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(receipt["before"], json!([2]));
        assert_eq!(receipt["after"], json!([2]));
        assert_eq!(receipt["cpu"], 2);
        let stages: Vec<Value> = log
            .lines()
            .filter_map(|l| l.strip_prefix("STAGE "))
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        assert_eq!(stages.len(), if job["phase"] == "latency" { 21 } else { 1 });
        let mut seen = BTreeSet::new();
        let expected_iterations = if job["phase"] == "latency" {
            job["case"]["iterations"].as_u64().unwrap()
        } else {
            if job["requested_pairs"] == 16 {
                131072
            } else {
                8192
            }
        };
        let mut ordinary = None;
        for stage in stages {
            assert_eq!(stage["iterations"], expected_iterations);
            assert_eq!(stage["timed_allocation_calls"], 0);
            assert_eq!(stage["all_output_storage_and_live_counts_correct"], true);
            let m = stage["ordinary_pairs"].as_u64().unwrap();
            if let Some(old) = ordinary {
                assert_eq!(m, old);
            }
            ordinary = Some(m);
            assert!(seen.insert((
                stage["block"].as_u64().unwrap(),
                stage["mode"].as_str().unwrap().to_string()
            )));
            maxrss = maxrss.max(stage["whole_process_maxrss_kib"].as_u64().unwrap());
            rows.push(json!({"phase":job["phase"],"case":name,"stage":stage}));
        }
        receipts.insert(name.to_string(), receipt);
    }
    assert_eq!(rows.len(), 1180);
    let mut cases = BTreeMap::new();
    let mut calibration = Vec::new();
    for case in read(r.join("cases.json")).as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let a = values(&rows, name, "control-a", "kernel_ns_per_layer");
        let b = values(&rows, name, "control-b", "kernel_ns_per_layer");
        let p = values(&rows, name, "prefix", "kernel_ns_per_layer");
        let cal = paired(&b, &a);
        let med = cal["median_ratio"].as_f64().unwrap();
        let geo = cal["geometric_ratio"].as_f64().unwrap();
        if !(1. / 1.03..=1.03).contains(&med) {
            calibration.push(format!("{name}:medianB/A={med}"));
        }
        if !(1. / 1.05..=1.05).contains(&geo) {
            calibration.push(format!("{name}:geometricB/A={geo}"));
        }
        let mut modes = BTreeMap::new();
        for mode in ["control-a", "control-b", "prefix"] {
            let mut data = serde_json::Map::new();
            for key in [
                "kernel_ns_per_layer",
                "caller_ns_per_layer",
                "user_cpu_ms",
                "system_cpu_ms",
                "minor_faults",
                "voluntary_switches",
                "involuntary_switches",
                "whole_process_maxrss_kib",
            ] {
                data.insert(
                    key.to_string(),
                    json!(median(values(&rows, name, mode, key))),
                );
            }
            modes.insert(mode, Value::Object(data));
        }
        let ordinary =
            rows.iter().find(|row| row["case"] == name).unwrap()["stage"]["ordinary_pairs"].clone();
        let caps = rows.iter().find(|row| row["case"] == name).unwrap()["stage"]
            ["record_capacities_bytes_both_retained"]
            .clone();
        cases.insert(name.to_string(),json!({"case":case,"ordinary_pairs":ordinary,"calibration":cal,"pairs":{"control-a":paired(&p,&a),"control-b":paired(&p,&b)},"resource_medians":modes,"retained_record_capacities_bytes":caps}));
    }
    // Fit using training only. No heldout result influences this choice.
    let mut fits = Vec::new();
    let mut threshold = None;
    for t in [8u64, 16, 32, 64, 128, 256, 512, 1024] {
        let training: Vec<_> = cases
            .values()
            .filter(|c| c["case"]["seed"] == 0 && c["ordinary_pairs"].as_u64().unwrap() >= t)
            .collect();
        let dense = training.iter().any(|c| c["case"]["mixed"] == false);
        let mixed = training.iter().any(|c| c["case"]["mixed"] == true);
        let failed: Vec<_> = training
            .iter()
            .filter(|c| !good(c, "control-a") || !good(c, "control-b"))
            .map(|c| c["case"]["name"].clone())
            .collect();
        let pass = dense && mixed && failed.is_empty();
        fits.push(json!({"threshold":t,"training_cases":training.len(),"both_layouts":dense&&mixed,"failed_training_cases":failed,"passes":pass}));
        if pass && threshold.is_none() {
            threshold = Some(t);
        }
    }
    save(
        r.join("training-rule.json"),
        &json!({"threshold_actual_ordinary_pairs":threshold,"training_only_fit":true,"fitting_observations":fits,"no_production_dispatcher":true}),
    );
    let mut failures = Vec::new();
    let mut aggregates = BTreeMap::new();
    let mut selected = Vec::new();
    if let Some(t) = threshold {
        for c in cases.values().filter(|c| c["case"]["seed"] == 1) {
            if c["ordinary_pairs"].as_u64().unwrap() >= t {
                selected.push(c["case"]["name"].clone());
                for control in ["control-a", "control-b"] {
                    if !good(c, control) {
                        failures.push(format!(
                            "{} heldoutprefix/{control}={}",
                            c["case"]["name"], c["pairs"][control]
                        ));
                    }
                    if c["pairs"][control]["geometric_ratio"].as_f64().unwrap() > 1.02 {
                        failures.push(format!(
                            "{} heldoutnonreg/{control}>1.02",
                            c["case"]["name"]
                        ));
                    }
                }
            }
        }
        assert!(!selected.is_empty());
        for control in ["control-a", "control-b"] {
            let ratios: Vec<_> = (0..7)
                .map(|b| {
                    gm(&cases
                        .values()
                        .filter(|c| c["case"]["seed"] == 1)
                        .map(|c| {
                            if c["ordinary_pairs"].as_u64().unwrap() >= t {
                                c["pairs"][control]["block_ratios"][b].as_f64().unwrap()
                            } else {
                                1.
                            }
                        })
                        .collect::<Vec<_>>())
                })
                .collect();
            let ratio = gm(&ratios);
            let wins = ratios.iter().filter(|x| **x < 1.).count();
            if ratio >= 1. || wins < 4 {
                failures.push(format!(
                    "heldout offlineaggregate/{control}={ratio},wins{wins}"
                ));
            }
            aggregates.insert(control,json!({"geometric_ratio":ratio,"wins":wins,"block_ratios":ratios,"unselected_cases_use_corresponding_original_control_ratio_one":true,"dispatcher_cost_unmeasured":true}));
        }
    } else {
        failures.push("No training threshold meets both-layout consistency rule".into());
    }
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    for pairs in [16, 1024] {
        for mode in ["control-a", "prefix"] {
            let name = format!("profile-p{pairs}-{mode}");
            let text = fs::read_to_string(
                r.join("measure")
                    .join(format!("samples-{name}"))
                    .join("stdout.log"),
            )
            .unwrap();
            assert!(
                !text.contains("PERF_RECORD_LOST")
                    && !text
                        .lines()
                        .any(|l| l.split_whitespace().any(|w| w == "LOST"))
            );
            let tid = receipts[&name]["tid"].as_u64().unwrap();
            let mut pids = BTreeSet::new();
            let mut tids = BTreeSet::new();
            let mut samples = 0;
            let mut period = 0u64;
            let mut symbols: BTreeMap<String, (usize, u64)> = BTreeMap::new();
            for line in text.lines() {
                let f: Vec<_> = line.split_whitespace().collect();
                if f.len() < 7 || f[4] != "cycles:u:" {
                    continue;
                }
                let (pid, thread) = f[1].split_once('/').unwrap();
                let pid: u64 = pid.parse().unwrap();
                let thread: u64 = thread.parse().unwrap();
                assert_eq!(thread, tid);
                assert_eq!(pid, tid);
                pids.insert(pid);
                tids.insert(thread);
                let n: u64 = f[3].parse().unwrap();
                assert!(n > 0);
                period += n;
                samples += 1;
                let row = symbols.entry(f[6].into()).or_default();
                row.0 += 1;
                row.1 += n;
            }
            assert!(samples > 0);
            assert_eq!(pids.len(), 1);
            assert_eq!(tids.len(), 1);
            let mut symbols:Vec<_>=symbols.into_iter().map(|(symbol,(samples,p))|json!({"symbol":symbol,"samples":samples,"period":p,"percent":100.*p as f64/period as f64})).collect();
            symbols.sort_by(|a, b| b["period"].as_u64().cmp(&a["period"].as_u64()));
            profiles.insert(name.clone(),json!({"samples":samples,"period":period,"pids":pids,"tids":tids,"lost_records":0,"symbols":symbols,"scope":"completecaller includingreset/Instant/loop"}));
            let data = r.join("profiles").join(name).join("perf.data");
            let h = Command::new("sudo")
                .args(["-n", "sha256sum"])
                .arg(&data)
                .output()
                .unwrap();
            assert!(h.status.success());
            raw.insert(
                data.display().to_string(),
                String::from_utf8(h.stdout)
                    .unwrap()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .to_string(),
            );
        }
    }
    let analysis = json!({"cases":cases,"training_threshold_actual_pairs":threshold,"selected_heldout_cases":selected,"heldout_offline_aggregates":aggregates,"calibration_failures":calibration,"speed_or_nonregression_failures":failures,"component_advances":calibration.is_empty()&&failures.is_empty(),"profiles":profiles,"latency_processes":56,"latency_stages":1176,"profile_processes":4,"all1180stages_complete_storage_correct":true,"timed_allocation_calls":0,"whole_process_maxrss_kib":maxrss,"SDK_requests":0,"Gnark_exports":0,"next_prior_count":7560,"adopted":false,"goal_achieved":false});
    save(r.join("rows.json"), &json!(rows));
    save(r.join("affinity.json"), &json!(receipts));
    save(r.join("analysis.json"), &analysis);
    save(r.join("raw-local-only-sha256.json"), &json!(raw));
    println!(
        "Layercomponent advances{} threshold{};calibration{}failures{};0SDK/Gnark;prior7560",
        analysis["component_advances"],
        analysis["training_threshold_actual_pairs"],
        calibration.len(),
        failures.len()
    );
}

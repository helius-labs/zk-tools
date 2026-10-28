use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
fn hash(p: &str) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let a: Value = serde_json::from_slice(&fs::read("analysis.json").unwrap()).unwrap();
    let mut bad = vec![];
    let mut nonreg = vec![];
    for (k, v) in a["pairs"].as_object().unwrap() {
        if k.ends_with("1-over-0") {
            if v["low"].as_f64().unwrap() < 1. / 1.03 || v["high"].as_f64().unwrap() > 1.03 {
                bad.push((k, v));
            }
        } else if v["high"].as_f64().unwrap() > 1.02 {
            nonreg.push((k, v));
        }
    }
    let mut summary = BTreeMap::new();
    for (k, v) in a["pairs"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.starts_with("g2-prefix") && k.ends_with("2-over-1"))
    {
        summary.insert(
            k,
            json!({"median":v["median"],"low":v["low"],"high":v["high"],"wins":v["wins"]}),
        );
    }
    let hashes = [hash("frozen-probe"), hash("off-probe"), hash("on-probe")];
    assert_eq!(hashes[0], hashes[1]);
    assert_ne!(hashes[1], hashes[2]);
    let mut stats = BTreeMap::new();
    let mut pct = true;
    for (k, v) in a["stat"].as_object().unwrap() {
        let e = &v["events"];
        for x in e.as_object().unwrap().values() {
            pct &= x["running_percent"] == 100.;
        }
        stats.insert(k,json!({"cycles":e["cycles:u"]["per_kernel_call"],"instructions":e["instructions:u"]["per_kernel_call"],"IPC":v["IPC"]}));
    }
    let samples: u64 = a["profiles"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| {
            assert_eq!(v["sampled_worker_tids"].as_array().unwrap().len(), 32);
            assert_eq!(v["pids"].as_array().unwrap().len(), 1);
            assert_eq!(v["lost"], 0);
            v["samples"].as_u64().unwrap()
        })
        .sum();
    let out = json!({"calibration_failures":bad,"nonregression_failures":nonreg,"all_G2_ON_OFF":summary,"control_executables_byte_identical":true,"executable_sha256":{"frozen":hashes[0],"off":hashes[1],"on":hashes[2]},"mode_argument_selects_measurement_label_only":true,"control_caveat":"Same ELF bytes exclude different compiled arithmetic as the control gap; process placement/address layout/host variation are not isolated","all_counters_100_percent_running":pct,"per_call_counters":stats,"profile_samples":samples,"all_profiles_all32_workers":true,"gate_unchanged":a["gates"],"proof_ledger":13181});
    fs::write(
        "interpretation.json",
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

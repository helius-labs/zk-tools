use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/common-z-callers176";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn check(b: bool) {
    assert!(b);
}
fn interval(mut data: Vec<f64>) -> (f64, f64, f64, usize) {
    let wins = data.iter().filter(|&&x| x < 1.).count();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (data[3], data[7], data[11], wins)
}
fn prepare() {
    let mut pins = BTreeMap::new();
    for p in [
        "review.rs",
        "review",
        "analysis.json",
        "observations.json",
        "receipts.json",
        "profile-symbols.json",
        "capture-plan.json",
        "capture-pins.json",
        "declaration.json",
        "source-audit-after-test-repair.json",
    ] {
        let p = r(p);
        pins.insert(p.display().to_string(), hash(&p));
    }
    fs::write(
        r("review-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let p = json!({"version":1,"source_pins":r("review-pins.json"),"steps":[{"kind":"command","name":"independent-evidence-review","argv":[r("review"),PathBuf::from("run")],"timeout_seconds":300}]});
    assert!(!r("review-plan.json").exists());
    fs::write(
        r("review-plan.json"),
        serde_json::to_vec_pretty(&p).unwrap(),
    )
    .unwrap();
}
fn run() {
    for p in ["review-pins.json", "capture-pins.json"] {
        for (p, h) in read(r(p)).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
    }
    let a = read(r("analysis.json"));
    let observations = read(r("observations.json"));
    let values = observations.as_object().unwrap();
    assert_eq!(values.len(), 1928);
    let mut computed = BTreeMap::<String, bool>::new();
    let mut ratio_rows = BTreeMap::<String, Vec<f64>>::new();
    for row in a["rows"].as_array().unwrap() {
        let context = row["context"].as_str().unwrap();
        let mut times = BTreeMap::<&str, Vec<f64>>::new();
        let mut caps = BTreeMap::new();
        for arm in ["frozen", "fresh", "OFF", "ON"] {
            let pre = &observations[format!("preflight-{context}-{arm}")];
            let cap = pre["worker_rows"][0]["scratch_capacity_bytes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .sum::<u64>();
            caps.insert(arm, cap);
            let t = (0..15)
                .map(|b| {
                    let v = &observations[format!("latency-{context}-b{b:02}-{arm}")];
                    assert_eq!(v["output_sha256"], pre["output_sha256"]);
                    assert_eq!(v["rounds_per_worker"], row["rounds"]);
                    v["elapsed_ns"].as_f64().unwrap() / v["rounds_per_worker"].as_f64().unwrap()
                })
                .collect::<Vec<_>>();
            times.insert(arm, t);
        }
        for (num, den, kind) in [
            ("fresh", "frozen", "calibration"),
            ("OFF", "frozen", "calibration"),
            ("ON", "frozen", "nonregression"),
            ("ON", "fresh", "nonregression"),
            ("ON", "OFF", "nonregression"),
        ] {
            let data = times[num]
                .iter()
                .zip(&times[den])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let (lo, mid, hi, wins) = interval(data.clone());
            let pass = if kind == "calibration" {
                lo >= 1. / 1.03 && hi <= 1.03
            } else {
                hi <= 1.03
            };
            let name = format!("{context}/{num}/{den}/{kind}");
            computed.insert(name.clone(), pass);
            let stored = a["checks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["name"] == name)
                .unwrap();
            assert_eq!(stored["statistics"]["low"], lo);
            assert_eq!(stored["statistics"]["median"], mid);
            assert_eq!(stored["statistics"]["high"], hi);
            assert_eq!(stored["statistics"]["wins"], wins);
            ratio_rows.insert(format!("{context}/{num}/{den}"), data);
        }
        for den in ["frozen", "fresh", "OFF"] {
            computed.insert(
                format!("{context}/ON/{den}/heap_capacity"),
                caps["ON"] as f64 <= 1.05 * caps[den] as f64,
            );
        }
    }
    for control in ["frozen", "fresh", "OFF"] {
        let data = (0..15)
            .map(|b| {
                let mut product = 1.;
                let mut n = 0;
                for row in a["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|row| row["workers"] == 32)
                {
                    product *= ratio_rows
                        [&format!("{}/ON/{control}", row["context"].as_str().unwrap())][b];
                    n += 1;
                }
                assert_eq!(n, 10);
                product.powf(0.1)
            })
            .collect::<Vec<_>>();
        let (lo, mid, hi, wins) = interval(data);
        let stored = &a["all10cases32_aggregate"][control];
        for (k, v) in [("low", lo), ("median", mid), ("high", hi)] {
            check((stored[k].as_f64().unwrap() - v).abs() < 1e-14);
        }
        assert_eq!(stored["wins"], wins);
        computed.insert(
            format!("all10cases32/ON/{control}/gain"),
            hi < 1. && wins >= 11,
        );
    }
    assert_eq!(computed.len(), 243);
    assert_eq!(a["checks"].as_array().unwrap().len(), 243);
    for c in a["checks"].as_array().unwrap() {
        assert_eq!(c["passed"], computed[c["name"].as_str().unwrap()]);
    }
    let failed = computed
        .iter()
        .filter(|(_, p)| !**p)
        .map(|(n, _)| n.clone())
        .collect::<BTreeSet<_>>();
    let stored = a["failed_checks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(failed, stored);
    assert_eq!(failed.len(), 10);
    assert_eq!(a["gate_passed"], false);
    let prior =
        read("/home/ubuntu/src/zopro-perf-work/g1-tree-callers176/prior-fixture-bindings.json");
    let mut worker_rows = 0;
    for v in values.values() {
        let workers = v["workers"].as_u64().unwrap();
        let rows = v["worker_rows"].as_array().unwrap();
        assert_eq!(rows.len(), workers as usize);
        let mut ids = BTreeSet::new();
        let mut tids = BTreeSet::new();
        let mut allocations = 0;
        for w in rows {
            let id = w["id"].as_u64().unwrap();
            ids.insert(id);
            check(tids.insert(w["tid"].as_u64().unwrap()));
            assert_eq!(w["cpu"], if workers == 1 { 2 } else { id });
            assert_eq!(w["pool_workers"], workers);
            assert_eq!(w["output_sha256"], v["output_sha256"]);
            allocations += w["timed_worker_allocation_calls"].as_u64().unwrap();
            worker_rows += 1;
        }
        assert_eq!(ids, (0..workers).collect());
        assert_eq!(v["timed_allocations"], allocations);
        if v["fresh_buffers"] == false {
            assert_eq!(allocations, 0);
        }
        if let Some(h) = prior["unweighted_output_sha256"].get(v["case"].as_str().unwrap()) {
            assert_eq!(&v["output_sha256"], h);
        }
    }
    let plan = read(r("capture-plan.json"));
    let mut commands = 0;
    for step in plan["steps"].as_array().unwrap() {
        let base = r(&format!("capture/{}", step["name"].as_str().unwrap()));
        let e = read(base.join("exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let c = read(base.join("command.json"));
        assert_eq!(
            &c["argv"].as_array().unwrap()[4..],
            step["argv"].as_array().unwrap().as_slice()
        );
        for (k, v) in step["environment"].as_object().unwrap() {
            assert_eq!(&c["environment"][k], v);
        }
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(base.join(format!("{stream}.log"))),
                e[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        commands += 1;
    }
    assert_eq!(commands, 1949);
    let audit = read(r("source-audit-after-test-repair.json"));
    let mut source_files = 0;
    for (p, h) in audit["candidate_source_hashes"].as_object().unwrap() {
        assert_eq!(hash(r("source").join(p)), h.as_str().unwrap());
        source_files += 1;
    }
    for (p, h) in audit["baseline_source_hashes"].as_object().unwrap() {
        let mut s = fs::read_to_string(r("source").join(p)).ok();
        let original_bytes = fs::read(r("source").join(p)).unwrap();
        for e in audit["edits"]
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .filter(|e| e["path"] == *p)
        {
            let value = s.as_mut().unwrap();
            if let Some(append) = e["append"].as_str() {
                check(value.ends_with(append));
                value.truncate(value.len() - append.len());
            } else {
                let after = e["after"].as_str().unwrap();
                assert_eq!(value.matches(after).count(), 1);
                *value = value.replacen(after, e["before"].as_str().unwrap(), 1);
            }
        }
        let bytes = s.map(String::into_bytes).unwrap_or(original_bytes);
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), h.as_str().unwrap());
    }
    assert_eq!(source_files, 180);
    let mut profiles = 0;
    let mut samples = 0;
    for (name, profile) in a["profiles"].as_object().unwrap() {
        let caller = &observations[name];
        let pid = caller["pid"].as_u64().unwrap();
        let workers = caller["worker_rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| (w["tid"].as_u64().unwrap(), w["cpu"].as_u64().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut n = 0;
        let mut worker_n = 0;
        for line in fs::read_to_string(r(&format!("capture/decode-{name}/stdout.log")))
            .unwrap()
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        {
            let (header, tail) = line.split_once(']').unwrap();
            let (id, cpu) = header.split_once('[').unwrap();
            let (p, t) = id.trim().split_once('/').unwrap();
            assert_eq!(p.parse::<u64>().unwrap(), pid);
            let tid = t.parse::<u64>().unwrap();
            let cpu = cpu.trim().parse::<u64>().unwrap();
            let (_, period_and_ip) = tail.split_once(':').unwrap();
            assert_eq!(period_and_ip.split_whitespace().next().unwrap(), "4000000");
            if let Some(&assigned) = workers.get(&tid) {
                assert_eq!(cpu, assigned);
                worker_n += 1;
            } else {
                check(cpu < 32);
            }
            n += 1;
        }
        assert_eq!(profile["samples"], n);
        assert_eq!(profile["worker_samples"], worker_n);
        assert_eq!(profile["passed"], true);
        assert_eq!(profile["zero_lost"], true);
        let raw = r(&format!("perf/{name}.data"));
        assert_eq!(hash(&raw), profile["raw_sha256"].as_str().unwrap());
        profiles += 1;
        samples += n;
    }
    assert_eq!(profiles, 4);
    let result = json!({"passed":true,"statistical_checks_reconstructed":243,"failed_checks_reconstructed":failed,"caller_outputs_checked":values.len(),"worker_rows_checked":worker_rows,"command_receipts_checked":commands,"candidate_source_files_checked":source_files,"all178_diagnostic_originals_recovered":true,"profile_header_samples_checked":samples,"profiles_checked":profiles,"scope":"independent timing/capacity/count/receipt/source-reversal and sample PID/TID/CPU/period checks; inherited arithmetic, no oracle/capture/decode replay; not independent exact ELF/IP or LOST parser"});
    assert!(!r("independent-review.json").exists());
    fs::write(
        r("independent-review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("{result}");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("run") => run(),
        _ => panic!("prepare | run"),
    }
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-r256-truncated-caller175";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn approx(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= a.abs().max(b.abs()).max(1.) * 1e-12,
        "{a} != {b}"
    );
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn stats(v: &[f64], s: &Value) {
    assert_eq!(v.len(), 31);
    let mut sorted = v.to_vec();
    sorted.sort_by(f64::total_cmp);
    for (k, i) in [("median", 15), ("low", 9), ("high", 21)] {
        approx(s[k].as_f64().unwrap(), sorted[i]);
    }
    assert_eq!(s["wins"], v.iter().filter(|&&v| v < 1.).count());
    for (a, b) in v.iter().zip(s["ratios"].as_array().unwrap()) {
        approx(*a, b.as_f64().unwrap());
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a, read(d.join("artifacts/analysis.json")));
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(digest(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let mut cells = BTreeMap::<String, Vec<(usize, f64)>>::new();
    let mut signatures = BTreeMap::new();
    let mut calls = 0u64;
    let mut processes = 0;
    let mut profile_review = BTreeMap::new();
    let jobs = read(r.join("jobs.json"));
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let stdout = fs::read_to_string(r.join(format!("runs/{name}/stdout.log"))).unwrap();
        let values: Vec<_> = stdout
            .lines()
            .filter_map(|l| l.strip_prefix("RESULT "))
            .collect();
        assert_eq!(values.len(), 1);
        let v: Value = serde_json::from_str(values[0]).unwrap();
        assert_eq!(v["internally_verified"], true);
        assert_eq!(v["timed_allocations"], 0);
        for k in ["kind", "size", "mode", "topology"] {
            assert_eq!(v[k], j[k]);
        }
        let topology = j["topology"].as_str().unwrap();
        let workers = match topology {
            "solo" => 1,
            "physical16" => 16,
            "logical32" => 32,
            _ => panic!(),
        };
        assert_eq!(v["worker_count"], workers);
        assert_eq!(v["workers"].as_array().unwrap().len(), workers);
        let rounds = j["rounds"].as_u64().unwrap();
        assert_eq!(v["iterations_per_worker"], rounds);
        assert_eq!(v["prime_iterations_per_worker"], 1024);
        calls += workers as u64 * (rounds + 3 * 1024);
        processes += 1;
        let key = format!("{}/{}/{}/{rounds}", j["kind"], j["size"], j["topology"]);
        let signature = json!([
            v["output_sha256"],
            v["capacity_bytes_per_worker"],
            v["fixture_reference_bytes_per_worker"]
        ]);
        if let Some(old) = signatures.insert(key, signature.clone()) {
            assert_eq!(old, signature);
        }
        let wall = v["wall_ns"].as_u64().unwrap();
        approx(
            v["wall_ns_per_wave"].as_f64().unwrap(),
            wall as f64 / rounds as f64,
        );
        let known: BTreeSet<_> = v["workers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w["tid"].as_u64().unwrap())
            .collect();
        assert_eq!(known.len(), workers);
        if j["phase"] == "latency" {
            cells
                .entry(format!(
                    "{}/{}",
                    j["context"].as_str().unwrap(),
                    j["variant"].as_str().unwrap()
                ))
                .or_default()
                .push((
                    j["block"].as_u64().unwrap() as usize,
                    v["wall_ns_per_wave"].as_f64().unwrap(),
                ));
        }
        if j["phase"] == "profile" {
            let script =
                fs::read_to_string(r.join(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut pids = BTreeSet::new();
            let mut tids = BTreeSet::new();
            let mut samples = 0;
            for l in script
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            {
                let f: Vec<_> = l.split_whitespace().collect();
                let (pid, tid) = f[0].split_once('/').unwrap();
                let pid = pid.parse::<u64>().unwrap();
                let tid = tid.parse::<u64>().unwrap();
                pids.insert(pid);
                tids.insert(tid);
                assert!(
                    known.contains(&tid) || tid == pid,
                    "sampled tid outside recorded workers/caller"
                );
                let period = f[1].parse::<u64>().unwrap();
                assert_eq!(period, 16000000);
                *symbols.entry(f[3..].join(" ")).or_default() += period;
                samples += 1;
            }
            let expected = &a["profiles"][name];
            assert_eq!(expected["symbol_periods"], json!(symbols));
            assert_eq!(expected["samples"], samples);
            assert_eq!(pids.len(), 1);
            assert!(samples >= 1000);
            let unknown = symbols
                .iter()
                .filter(|(k, _)| k.contains("unknown"))
                .map(|(_, v)| v)
                .sum::<u64>() as f64
                / symbols.values().sum::<u64>() as f64;
            assert!(unknown <= 0.05);
            assert_eq!(
                read(r.join(format!("runs/loss-{name}/stdout.log")))["zero_lost"],
                true
            );
            profile_review.insert(name.to_string(),json!({"samples":samples,"unknown_period_fraction":unknown,"sampled_PIDs":pids,"sampled_TIDs":tids.len(),"all_TIDs_in_recorded_worker_or_caller_set":true,"every_period_16000000":true,"zero_lost":true}));
        }
    }
    assert_eq!(processes, 1732);
    assert_eq!(a["total_kernel_calls"], calls);
    assert_eq!(cells.len(), 54);
    let mut ratios = BTreeMap::new();
    let mut checks = BTreeMap::new();
    let contexts = read(r.join("contexts.json"));
    for c in contexts.as_array().unwrap() {
        let name = c["name"].as_str().unwrap();
        for label in ["frozen", "off", "on"] {
            let values = &mut cells.get_mut(&format!("{name}/{label}")).unwrap();
            values.sort_by_key(|v| v.0);
            assert_eq!(values.len(), 31);
            for (b, v) in values.iter().enumerate() {
                assert_eq!(v.0, b);
            }
            approx(
                a["cells"][format!("{name}/{label}")]["median_wave_ns"]
                    .as_f64()
                    .unwrap(),
                median(&values.iter().map(|v| v.1).collect::<Vec<_>>()),
            );
        }
        for (x, y) in [("off", "frozen"), ("on", "off"), ("on", "frozen")] {
            let key = format!("{name}/{x}/{y}");
            let v: Vec<_> = cells[&format!("{name}/{x}")]
                .iter()
                .zip(&cells[&format!("{name}/{y}")])
                .map(|(x, y)| x.1 / y.1)
                .collect();
            stats(&v, &a["pairs"][&key]);
            let s = &a["pairs"][&key];
            let pass = if x == "off" {
                s["low"].as_f64().unwrap() >= 1. / 1.03 && s["high"].as_f64().unwrap() <= 1.03
            } else {
                s["high"].as_f64().unwrap() <= 1.03
            };
            checks.insert(key.clone(), pass);
            ratios.insert(key, v);
        }
    }
    for group in ["all18", "logical32_6"] {
        let selected: Vec<_> = contexts
            .as_array()
            .unwrap()
            .iter()
            .filter(|c| group == "all18" || c["topology"] == "logical32")
            .collect();
        for control in ["off", "frozen"] {
            let v: Vec<_> = (0..31)
                .map(|b| {
                    (selected
                        .iter()
                        .map(|c| {
                            ratios[&format!("{}/on/{control}", c["name"].as_str().unwrap())][b].ln()
                        })
                        .sum::<f64>()
                        / selected.len() as f64)
                        .exp()
                })
                .collect();
            let key = format!("{group}/on/{control}");
            stats(&v, &a["aggregates"][&key]);
            let s = &a["aggregates"][&key];
            checks.insert(
                format!("{key}/gain"),
                s["high"].as_f64().unwrap() < 1. && s["wins"].as_u64().unwrap() >= 21,
            );
        }
    }
    assert_eq!(a["checks"], json!(checks));
    let failed: Vec<_> = checks
        .iter()
        .filter(|(_, v)| !**v)
        .map(|(k, _)| k.clone())
        .collect();
    assert_eq!(a["failed_checks"], json!(failed));
    assert_eq!(a["component_gate"], failed.is_empty());
    let mut archive_counts = BTreeMap::new();
    for (name, base) in [("source", r.join("source")), ("receipts", r.to_path_buf())] {
        let gz = Command::new("gzip")
            .args(["-d", "-c"])
            .arg(d.join(format!("artifacts/{name}.json.gz")))
            .output()
            .unwrap();
        assert!(gz.status.success());
        let archive: Value = serde_json::from_slice(&gz.stdout).unwrap();
        for (path, v) in archive.as_object().unwrap() {
            let bytes: Vec<u8> = serde_json::from_value(v["bytes"].clone()).unwrap();
            assert_eq!(digest(&bytes), v["sha256"]);
            assert_eq!(fs::read(base.join(path)).unwrap(), bytes);
        }
        archive_counts.insert(name, archive.as_object().unwrap().len());
    }
    assert_eq!(archive_counts["source"], 177);
    assert_eq!(archive_counts["receipts"], 8808);
    let rows = read(d.join("accepted-rows.json"));
    assert_eq!(rows,read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-current-bench-snapshot175/data.json")["rows"]);
    let out = json!({"review_passed":true,"pins_verified":pins.as_object().unwrap().len(),"process_results_reviewed":processes,"latency_cells_recomputed":54,"paired_intervals_recomputed":54,"aggregate_intervals_recomputed":4,"failed_checks_reproduced":failed,"total_kernel_calls_including_primes":calls,"profiles":profile_review,"archive_members_checked":archive_counts,"accepted_rows_exact":true,"new_runtime_execution_or_proofs":0});
    fs::write(
        r.join("offline-review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Offline review passed:54cells/54pairs/4aggregates,{}receipts,4profiles,{}pins; no benchmark/test/proof rerun",archive_counts["receipts"],pins.as_object().unwrap().len());
}

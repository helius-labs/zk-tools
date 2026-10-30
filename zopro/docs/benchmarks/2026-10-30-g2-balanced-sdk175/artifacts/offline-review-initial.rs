use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "docs/benchmarks/2026-10-30-g2-balanced-sdk175";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n])
    }
    format!("{:x}", h.finalize())
}
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.),
        "{a} != {b}"
    )
}
fn median(x: &[f64]) -> f64 {
    let mut x = x.to_vec();
    x.sort_by(f64::total_cmp);
    if x.len() % 2 == 1 {
        x[x.len() / 2]
    } else {
        (x[x.len() / 2 - 1] + x[x.len() / 2]) / 2.
    }
}
fn interval(x: &[f64], v: &Value) {
    assert_eq!(x.len(), 31);
    let mut sorted = x.to_vec();
    sorted.sort_by(f64::total_cmp);
    for (k, i) in [("low", 9), ("median", 15), ("high", 21)] {
        near(sorted[i], num(&v[k]))
    }
    assert_eq!(v["wins"], x.iter().filter(|&&a| a < 1.).count());
    for (a, b) in x.iter().zip(v["ratios"].as_array().unwrap()) {
        near(*a, num(b))
    }
}
fn archive(p: &Path, base: &Path) -> usize {
    let out = Command::new("gzip").arg("-dc").arg(p).output().unwrap();
    assert!(out.status.success());
    let map: Value = serde_json::from_slice(&out.stdout).unwrap();
    for (path, v) in map.as_object().unwrap() {
        let b = if let Some(s) = v["utf8"].as_str() {
            s.as_bytes().to_vec()
        } else {
            let s = v["hex"].as_str().unwrap();
            (0..s.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
                .collect()
        };
        assert_eq!(b.len() as u64, v["length"].as_u64().unwrap());
        assert_eq!(format!("{:x}", Sha256::digest(&b)), v["sha256"]);
        assert_eq!(b, fs::read(base.join(path)).unwrap(), "{path}");
    }
    map.as_object().unwrap().len()
}
fn main() {
    let r = Path::new(R);
    let z = Path::new(Z);
    let d = z.join(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a, read(d.join("artifacts/analysis.json")));
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}")
    }
    let manifest = read(z.join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    let originals = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 175);
    let audit = read(r.join("source-audit.json"));
    let mut exact = 0;
    for (p, h) in originals {
        assert_eq!(hash(z.join(p)), h.as_str().unwrap());
        let candidate = r.join("source").join(p);
        let mut bytes = fs::read(&candidate).unwrap();
        if hash(&candidate) == h.as_str().unwrap() {
            exact += 1
        } else {
            let e = &audit["edits"][p];
            let s = String::from_utf8(bytes).unwrap();
            bytes = if let Some(suffix) = e["suffix"].as_str() {
                s.strip_suffix(suffix).unwrap().as_bytes().to_vec()
            } else {
                let after = e["after"].as_str().unwrap();
                assert_eq!(s.matches(after).count(), 1);
                s.replacen(after, e["before"].as_str().unwrap(), 1)
                    .into_bytes()
            };
            assert_eq!(format!("{:x}", Sha256::digest(bytes)), h.as_str().unwrap());
        }
    }
    assert_eq!(exact, 173);
    assert_eq!(
        fs::read(r.join("source").join(audit["child"].as_str().unwrap())).unwrap(),
        fs::read(
            Path::new("/home/ubuntu/src/zopro-perf-work/g2-balanced175/source")
                .join(audit["child"].as_str().unwrap())
        )
        .unwrap()
    );
    let mut archive_counts = BTreeMap::new();
    for (file, count, base) in [
        ("source", 188, r.to_path_buf()),
        ("receipts", 9942, r.to_path_buf()),
        ("proofs", 1944, r.join("proofs")),
        ("profile-symbols", 1, r.to_path_buf()),
        ("inherited-summary", 3, r.join("inherited-summary")),
    ] {
        let n = archive(&d.join(format!("artifacts/{file}.json.gz")), &base);
        assert_eq!(n, count);
        archive_counts.insert(file, n);
    }
    let result = read(r.join("runs/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["steps"].as_array().unwrap().len(), 1974);
    let jobs = read(r.join("jobs.json"));
    let rows = read(r.join("rows.json"));
    assert_eq!(jobs.as_array().unwrap().len(), 972);
    assert_eq!(rows.as_array().unwrap().len(), 972);
    let inventory = read(r.join("proof-inventory.json"));
    let mut ledger: BTreeSet<String> =
        serde_json::from_value(read(r.join("prior-proof-sha256.json"))).unwrap();
    assert_eq!(ledger.len(), 23971);
    let mut requests = 0;
    let mut bindings = BTreeMap::new();
    for ((j, row), inv) in jobs
        .as_array()
        .unwrap()
        .iter()
        .zip(rows.as_array().unwrap())
        .zip(inventory.as_array().unwrap())
    {
        let name = j["name"].as_str().unwrap();
        assert_eq!(row["name"], name);
        assert_eq!(inv["name"], name);
        let log = fs::read_to_string(r.join(format!("runs/{name}/stdout.log"))).unwrap();
        let samples: Vec<Value> = log
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(samples.len() as u64, j["rounds"].as_u64().unwrap());
        near(
            median(&samples.iter().map(|v| num(&v["ms"])).collect::<Vec<_>>()),
            num(&row["request_median_ms"]),
        );
        requests += j["rounds"].as_u64().unwrap() + 3;
        let proof = Path::new(j["directory"].as_str().unwrap()).join("0.proof");
        let public = proof.with_file_name("0.public");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        let ph = hash(proof);
        assert_eq!(inv["proof_sha256"], ph);
        assert!(ledger.insert(ph.clone()));
        assert_eq!(hash(public), j["case"]["public_sha256"]);
        let verify = fs::read_to_string(r.join(format!("runs/verify-{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = verify
            .lines()
            .filter_map(|l| l.strip_prefix("VERIFIED "))
            .collect();
        assert_eq!(lines.len(), 1);
        let f: BTreeMap<_, _> = lines[0]
            .split_whitespace()
            .map(|s| s.split_once('=').unwrap())
            .collect();
        assert_eq!(f["proof_sha256"], ph);
        assert_eq!(f["public_sha256"], j["case"]["public_sha256"]);
        assert_eq!(f["image_sha256"], j["case"]["image_sha256"]);
        assert_eq!(f["vk_sha256"].len(), 64);
        let key = j["case"]["shape"].as_str().unwrap().to_string();
        if let Some(old) = bindings.insert(key, f["vk_sha256"].to_string()) {
            assert_eq!(old, f["vk_sha256"]);
        }
    }
    assert_eq!(requests, 55788);
    assert_eq!(ledger.len(), 24943);
    assert_eq!(
        ledger,
        serde_json::from_value::<BTreeSet<String>>(read(r.join("proof-sha256-after.json")))
            .unwrap()
    );
    let mut fails = BTreeSet::new();
    let mut time = BTreeMap::<String, Vec<f64>>::new();
    let cases = read(r.join("cases.json"));
    assert_eq!(cases.as_array().unwrap().len(), 10);
    for c in cases.as_array().unwrap() {
        let name = c["name"].as_str().unwrap();
        let mut rss = BTreeMap::new();
        for variant in ["accepted", "OFF", "ON"] {
            let mut selected: Vec<_> = rows
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| v["panel"] == "latency" && v["case"] == name && v["variant"] == variant)
                .collect();
            selected.sort_by_key(|v| v["block"].as_u64().unwrap());
            assert_eq!(selected.len(), 31);
            let mut ms = vec![];
            for (b, v) in selected.iter().enumerate() {
                assert_eq!(v["block"], b);
                ms.push(num(&v["request_median_ms"]));
            }
            let cell = &a["cells"][format!("{name}/{variant}")];
            near(median(&ms), num(&cell["median_ms"]));
            near(
                median(
                    &selected
                        .iter()
                        .map(|v| num(&v["CPU_ms_per_request"]))
                        .collect::<Vec<_>>(),
                ),
                num(&cell["CPU_ms"]),
            );
            let max = selected
                .iter()
                .map(|v| num(&v["RSS_MiB"]))
                .fold(0., f64::max);
            near(max, num(&cell["max_RSS_MiB"]));
            rss.insert(variant, max);
            time.insert(format!("{name}/{variant}"), ms);
        }
        for (n, den) in [("OFF", "accepted"), ("ON", "accepted"), ("ON", "OFF")] {
            let k = format!("{name}/{n}/{den}");
            let ratios: Vec<_> = time[&format!("{name}/{n}")]
                .iter()
                .zip(&time[&format!("{name}/{den}")])
                .map(|(a, b)| a / b)
                .collect();
            let p = &a["pairs"][&k];
            interval(&ratios, &p["statistics"]);
            let q = &p["statistics"];
            let pass = if n == "OFF" {
                num(&q["low"]) >= 1. / 1.03 && num(&q["high"]) <= 1.03
            } else if name == "1x2-w32" {
                num(&q["high"]) < 1. && q["wins"].as_u64().unwrap() >= 21
            } else {
                num(&q["high"]) <= 1.02
            };
            assert_eq!(p["passed"], pass);
            if !pass {
                fails.insert(k);
            }
        }
        for v in ["accepted", "OFF"] {
            if rss["ON"] / rss[v] > 1.05 {
                fails.insert(format!("{name}/RSS/ON/{v}"));
            }
        }
    }
    let original8: Vec<_> = cases
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32 && !c["reference"].is_null())
        .collect();
    assert_eq!(original8.len(), 8);
    for den in ["accepted", "OFF"] {
        let ratios: Vec<_> = (0..31)
            .map(|b| {
                (original8
                    .iter()
                    .map(|c| {
                        let n = c["name"].as_str().unwrap();
                        (time[&format!("{n}/ON")][b] / time[&format!("{n}/{den}")][b]).ln()
                    })
                    .sum::<f64>()
                    / 8.)
                    .exp()
            })
            .collect();
        let p = &a["original8_aggregates"][den];
        interval(&ratios, &p["statistics"]);
        let pass =
            num(&p["statistics"]["high"]) < 1. && p["statistics"]["wins"].as_u64().unwrap() >= 21;
        assert_eq!(p["passed"], pass);
        if !pass {
            fails.insert(format!("original8/ON/{den}/gain"));
        }
    }
    assert_eq!(a["counters"].as_object().unwrap().len(), 6);
    assert_eq!(a["profiles"].as_object().unwrap().len(), 6);
    assert!(a["counters"]
        .as_object()
        .unwrap()
        .values()
        .all(|v| v["qualified"] == true));
    assert!(a["profiles"]
        .as_object()
        .unwrap()
        .values()
        .all(|v| v["attribution_qualified"] == true && v["zero_lost"] == true));
    assert_eq!(
        fails,
        serde_json::from_value::<BTreeSet<String>>(a["failed_checks"].clone()).unwrap()
    );
    assert_eq!(fails.len(), 3);
    assert_eq!(a["gate_passed"], fails.is_empty());
    let inherited = read(r.join("inherited-summary/data.json"));
    let summary = read(z.join("docs/benchmarks/2026-10-29-current-cpu-summary/data.json"));
    let snapshot = read(z.join("docs/benchmarks/2026-10-30-current-bench-report/data.json"));
    assert_eq!(summary["rows"], inherited["rows"]);
    assert_eq!(snapshot["rows"], inherited["rows"]);
    assert_eq!(summary["historical_distinct_Gnark_ledger"], 24943);
    assert_eq!(snapshot["historical_distinct_Gnark_ledger"], 24943);
    let out = json!({"offline_review_passed":true,"new_captures_or_verifier_executions":0,"current_pins_verified":pins.as_object().unwrap().len(),"production_files_verified":175,"candidate_originals_exact":173,"all175_reversed":true,"archive_members_verified":archive_counts,"workflow_steps":1974,"SDK_requests":requests,"new_distinct_proofs_and_receipts_verified":972,"ledger":ledger.len(),"cases_recomputed":10,"latency_processes":930,"pair_comparisons_recomputed":30,"original8_aggregates_recomputed":2,"failed_checks":fails,"accepted_rows_preserved":9,"CPU_and_Metal_reference_rows":8,"adopted":false,"production_changed":false});
    fs::write(
        r.join("offline-review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Offline review passed: all pins/175 production files/archive members; 972 proof receipts/10 cases/30 pairs/2 aggregates; three failed checks retained");
}

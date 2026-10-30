use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/query-timeline175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
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
fn median(v: &[f64]) -> f64 {
    let mut a = v.to_vec();
    a.sort_by(f64::total_cmp);
    if a.len() % 2 == 1 {
        a[a.len() / 2]
    } else {
        (a[a.len() / 2 - 1] + a[a.len() / 2]) / 2.
    }
}
fn archive(p: &Path, base: &Path) -> usize {
    let o = Command::new("gzip").arg("-dc").arg(p).output().unwrap();
    assert!(o.status.success());
    let map: Value = serde_json::from_slice(&o.stdout).unwrap();
    for (k, v) in map.as_object().unwrap() {
        let bytes = if let Some(s) = v["utf8"].as_str() {
            s.as_bytes().to_vec()
        } else {
            let h = v["hex"].as_str().unwrap();
            (0..h.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
                .collect()
        };
        assert_eq!(v["length"], bytes.len());
        assert_eq!(v["sha256"], format!("{:x}", Sha256::digest(&bytes)));
        assert_eq!(bytes, fs::read(base.join(k)).unwrap(), "{k}");
    }
    map.as_object().unwrap().len()
}
fn main() {
    let d = Path::new(Z).join("docs/benchmarks/2026-10-30-query-timeline175");
    let a = read(r("analysis.json"));
    assert_eq!(a, read(d.join("artifacts/analysis.json")));
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read(Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    let audit = read(r("source-audit.json"));
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
        let path = r("source").join(p);
        if hash(&path) == h.as_str().unwrap() {
            exact += 1;
        } else {
            assert_eq!(p, "crates/zopro-core/src/lib.rs");
            let text = fs::read_to_string(path).unwrap();
            let mut s = text
                .strip_suffix(audit["suffix"].as_str().unwrap())
                .unwrap()
                .to_string();
            for e in audit["edits"].as_array().unwrap().iter().rev() {
                let after = e["after"].as_str().unwrap();
                assert_eq!(s.matches(after).count(), 1);
                s = s.replace(after, e["before"].as_str().unwrap());
            }
            assert_eq!(format!("{:x}", Sha256::digest(s)), h.as_str().unwrap());
        }
    }
    assert_eq!(exact, 174);
    let mut archives = BTreeMap::new();
    for (name, count, base) in [
        ("source", 176, r("source")),
        ("receipts", 2923, r("")),
        ("proofs", 576, r("")),
    ] {
        let n = archive(&d.join(format!("artifacts/{name}.json.gz")), &base);
        assert_eq!(n, count);
        archives.insert(name, n);
    }
    // Preserve the exact pre-capture summary independently of later rolling updates.
    let mut inherited = BTreeMap::new();
    for f in ["README.md", "data.json", "source-hashes.json"] {
        let p = r("inherited-summary").join(f);
        let b = fs::read(&p).unwrap();
        let len = b.len();
        inherited.insert(
            f,
            json!({"sha256":hash(&p),"length":len,"utf8":String::from_utf8(b).unwrap()}),
        );
    }
    let bytes = serde_json::to_vec(&inherited).unwrap();
    let tmp = r("inherited-archive.json");
    fs::write(&tmp, &bytes).unwrap();
    let packed = d.join("artifacts/inherited-summary.json.gz");
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&tmp)
        .stdout(Stdio::from(fs::File::create(&packed).unwrap()))
        .status()
        .unwrap()
        .success());
    assert_eq!(archive(&packed, &r("inherited-summary")), 3);
    fs::remove_file(tmp).unwrap();
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    assert_eq!(
        read(r("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        576
    );
    let jobs = read(r("jobs.json"));
    let rows = read(r("rows.json"));
    let inventory = read(r("proof-inventory.json"));
    let mut ledger: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(ledger.len(), 24943);
    let mut requests = 0;
    let mut stages = 0;
    let mut vk = None;
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
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let samples: Vec<Value> = text
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let trace: Vec<Value> = text
            .lines()
            .filter_map(|l| l.strip_prefix("TIMELINE "))
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(samples.len() as u64, j["rounds"].as_u64().unwrap());
        assert_eq!(json!(trace), row["timeline"]);
        near(
            median(&samples.iter().map(|v| num(&v["ms"])).collect::<Vec<_>>()),
            num(&row["median_ms"]),
        );
        requests += samples.len() + 3;
        stages += trace.len();
        if j["variant"] == "traced" {
            assert_eq!(trace.len(), samples.len());
        } else {
            assert!(trace.is_empty());
        }
        let dir = Path::new(j["directory"].as_str().unwrap());
        let proof = dir.join("0.proof");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        let ph = hash(proof);
        assert_eq!(inv["proof_sha256"], ph);
        assert!(ledger.insert(ph.clone()));
        assert_eq!(hash(dir.join("0.public")), j["case"]["public_sha256"]);
        let receipt = fs::read_to_string(r(format!("runs/verify-{name}/stdout.log"))).unwrap();
        let verified: Vec<_> = receipt
            .lines()
            .filter_map(|l| l.strip_prefix("VERIFIED "))
            .collect();
        assert_eq!(verified.len(), 1);
        let f: BTreeMap<_, _> = verified[0]
            .split_whitespace()
            .map(|s| s.split_once('=').unwrap())
            .collect();
        assert_eq!(f["proof_sha256"], ph);
        assert_eq!(f["public_sha256"], j["case"]["public_sha256"]);
        assert_eq!(f["image_sha256"], j["case"]["image_sha256"]);
        assert_eq!(f["vk_sha256"].len(), 64);
        if let Some(old) = vk.replace(f["vk_sha256"].to_string()) {
            assert_eq!(old, f["vk_sha256"]);
        }
    }
    assert_eq!(jobs.as_array().unwrap().len(), 288);
    assert_eq!(requests, 43002);
    assert_eq!(stages, 14046);
    assert_eq!(ledger.len(), 25231);
    assert_eq!(
        ledger,
        serde_json::from_value::<BTreeSet<String>>(read(r("proof-sha256-after.json"))).unwrap()
    );
    let mut failures = BTreeSet::new();
    for case in ["1x2-w1", "1x2-w16", "1x2-w32"] {
        let mut times = BTreeMap::new();
        for variant in ["accepted", "OFF", "traced"] {
            let mut chosen: Vec<_> = rows
                .as_array()
                .unwrap()
                .iter()
                .filter(|row| {
                    row["panel"] == "latency" && row["case"] == case && row["variant"] == variant
                })
                .collect();
            chosen.sort_by_key(|row| row["block"].as_u64().unwrap());
            assert_eq!(chosen.len(), 31);
            let ms: Vec<_> = chosen.iter().map(|v| num(&v["median_ms"])).collect();
            let c = &a["cells"][format!("{case}/{variant}")];
            near(median(&ms), num(&c["median_ms"]));
            near(
                median(&chosen.iter().map(|v| num(&v["CPU_ms"])).collect::<Vec<_>>()),
                num(&c["CPU_ms"]),
            );
            near(
                chosen.iter().map(|v| num(&v["RSS_MiB"])).fold(0., f64::max),
                num(&c["max_RSS_MiB"]),
            );
            times.insert(variant, ms);
        }
        let mut qualified = true;
        for (n, d) in [("OFF", "accepted"), ("traced", "OFF")] {
            let key = format!("{case}/{n}/{d}");
            let p = &a["pairs"][&key];
            let stats = &p["statistics"];
            let mut ratio: Vec<_> = times[n].iter().zip(&times[d]).map(|(x, y)| x / y).collect();
            for (x, v) in ratio.iter().zip(stats["ratios"].as_array().unwrap()) {
                near(*x, num(v));
            }
            let wins = ratio.iter().filter(|&&x| x < 1.).count();
            assert_eq!(stats["wins"], wins);
            ratio.sort_by(f64::total_cmp);
            for (k, i) in [("low", 9), ("median", 15), ("high", 21)] {
                near(num(&stats[k]), ratio[i]);
            }
            let pass = ratio[9] >= 1. / 1.01 && ratio[21] <= 1.01;
            assert_eq!(p["within_factor1_01"], pass);
            if !pass {
                qualified = false;
                failures.insert(key);
            }
        }
        assert_eq!(a["topology_qualified"][case], qualified);
        let traces: Vec<[u64; 13]> = rows
            .as_array()
            .unwrap()
            .iter()
            .filter(|row| {
                row["panel"] == "latency" && row["case"] == case && row["variant"] == "traced"
            })
            .flat_map(|row| row["timeline"].as_array().unwrap())
            .map(|t| serde_json::from_value(t["offsets_ns"].clone()).unwrap())
            .collect();
        assert_eq!(traces.len(), 4681);
        let mut last = [0usize; 3];
        let mut metrics = BTreeMap::<&str, Vec<f64>>::new();
        for t in traces {
            assert!(t[..7].windows(2).all(|w| w[0] <= w[1]));
            let ends = [t[8], t[10], t[12]];
            let biggest = *ends.iter().max().unwrap();
            let winners: Vec<_> = ends
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == biggest)
                .map(|(i, _)| i)
                .collect();
            assert_eq!(winners.len(), 1);
            last[winners[0]] += 1;
            for q in t[7..].chunks_exact(2) {
                assert!(t[4] <= q[0] && q[0] <= q[1] && q[1] <= t[5]);
            }
            let mut sorted = ends;
            sorted.sort();
            for (k, v) in [
                ("solver", t[0]),
                ("public_correction_and_release", t[1] - t[0]),
                ("canonical_conversion", t[2] - t[1]),
                ("scalar_extraction", t[3] - t[2]),
                ("product", t[4] - t[3]),
                ("prefix_to_product_end", t[4]),
                ("query_join", t[5] - t[4]),
                ("postjoin_output", t[6] - t[5]),
                ("canonical_span", t[6]),
                ("query_A_inclusive", t[8] - t[7]),
                ("query_KH_inclusive", t[10] - t[9]),
                ("query_B2_inclusive", t[12] - t[11]),
                ("join_after_last_query", t[5] - biggest),
                ("last_minus_second_query_end", sorted[2] - sorted[1]),
            ] {
                metrics.entry(k).or_default().push(v as f64 / 1e6);
            }
        }
        for (k, v) in metrics {
            near(
                median(&v),
                num(&a["stages"][case]["median_intervals_ms"][k]),
            );
        }
        for (k, v) in [("A", last[0]), ("KH", last[1]), ("B2", last[2])] {
            assert_eq!(a["stages"][case]["last_query"][k], v);
        }
    }
    assert_eq!(
        failures,
        serde_json::from_value::<BTreeSet<String>>(a["overhead_failed_checks"].clone()).unwrap()
    );
    let inherited = read(r("inherited-summary/data.json"));
    assert_eq!(inherited["rows"].as_array().unwrap().len(), 9);
    assert_eq!(
        inherited["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["M5_reference_and_local_ratios"].is_null())
            .count(),
        8
    );
    let out = json!({"offline_review_passed":true,"original_pins_verified":pins.as_object().unwrap().len(),"production_files_verified":175,"candidate_originals_exact":174,"all175_reversed":true,"archives_verified":archives,"inherited_summary_members":3,"SDK_requests":requests,"proof_bytes_and_Gnark_receipts":288,"distinct_proof_ledger":25231,"per_request_timeline_records":stages,"case_variant_cells_recomputed":9,"paired_intervals_recomputed":6,"all_stage_medians_and_end_counts_recomputed":true,"overhead_failed_checks":failures,"accepted_rows_preserved":9,"CPU_and_Metal_reference_rows":8,"new_capture_test_proof_verifier_executions":0,"production_changed":false,"adopted":false});
    fs::write(
        r("offline-review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Offline review passed451pins/175productionfiles/176source+2923receipts+576proofmembers;288proofreceipts/14046records;allcells,pairs,stage medians,endcounts reproduced");
}

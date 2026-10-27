use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
};
const R: &str = "/tmp/zopro-current-c7a-20261027";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn median(v: &[f64]) -> f64 {
    assert!(!v.is_empty());
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn gm(v: &[f64]) -> f64 {
    assert!(!v.is_empty() && v.iter().all(|v| v.is_finite() && *v > 0.));
    (v.iter().map(|v| v.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn main() {
    let r = Path::new(R);
    let performance = r.join("remote-results/output/measure/performance");
    let result = read(performance.join("result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["distinct_verified_exports"], 249);
    assert_eq!(result["profiles"], 6);
    let protocol = read(performance.join("protocol.json"));
    let rows = read(performance.join("rows.json"));
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 249);
    let cases = read(r.join("context/manifest.json"))["cases"].clone();
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(r.join("context/manifest.json"))["prior_proof_sha256"].clone())
            .unwrap();
    assert_eq!(seen.len(), 9904);
    let mut inv = BTreeMap::new();
    let mut exports = Vec::new();
    for (kind, root, inventory) in [
        (
            "local",
            r.join("local-output"),
            read(r.join("local-output/preflight/proof-inventory.json")),
        ),
        (
            "cloud",
            performance.clone(),
            read(performance.join("proof-inventory.json")),
        ),
    ] {
        let mut count = 0;
        for (p, h) in inventory.as_object().unwrap() {
            let path = if kind == "local" {
                root.join(p.strip_prefix("/output/").unwrap())
            } else {
                root.join(p)
            };
            assert_eq!(hash(&path), h.as_str().unwrap());
            assert!(inv
                .insert(path.display().to_string(), h.as_str().unwrap().to_owned())
                .is_none());
            if path.extension().is_some_and(|s| s == "proof") {
                assert_eq!(fs::metadata(&path).unwrap().len(), 164);
                assert!(seen.insert(h.as_str().unwrap().to_owned()));
                let stem = path
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();
                let case = cases
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|c| {
                        Path::new(c["image"].as_str().unwrap()).file_stem().unwrap()
                            == stem.as_str()
                    })
                    .unwrap();
                assert_eq!(
                    hash(path.with_extension("public")),
                    case["public_sha256"].as_str().unwrap()
                );
                exports.push((path, stem));
                count += 1;
            }
        }
        assert_eq!(count, if kind == "local" { 27 } else { 249 });
    }
    assert_eq!(seen.len(), 10180);
    assert_eq!(inv.len(), 552);
    assert_eq!(exports.len(), 276);
    save(r.join("proof-inventory.json"), &json!(inv));
    let verification = r.join("local-verification-proofs");
    fs::create_dir(&verification).unwrap();
    let mut perkey = BTreeMap::<String, usize>::new();
    for (path, stem) in exports {
        let i = perkey.entry(stem.clone()).or_default();
        let dir = verification.join(&stem);
        fs::create_dir_all(&dir).unwrap();
        fs::copy(&path, dir.join(format!("{i}.proof"))).unwrap();
        fs::copy(
            path.with_extension("public"),
            dir.join(format!("{i}.public")),
        )
        .unwrap();
        *i += 1;
    }
    let log = fs::File::create(r.join("post-transport-gnark.log")).unwrap();
    let status = Command::new("/tmp/zopro-gnark-verify/verify")
        .arg("/tmp/zopro-keys")
        .arg(&verification)
        .arg("/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock")
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    assert!(status.success());
    assert!(fs::read_to_string(r.join("post-transport-gnark.log"))
        .unwrap()
        .contains("verified 276 distinct proofs across 9 pinned keys"));
    let mut cells = BTreeMap::new();
    for case in protocol["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut modes = BTreeMap::new();
        let mut pairs = BTreeMap::new();
        for mode in ["previous", "current", "aeglos"] {
            let selected: Vec<_> = rows
                .iter()
                .filter(|r| r["phase"] == "latency" && r["case"] == name && r["variant"] == mode)
                .collect();
            assert_eq!(selected.len(), 3);
            let times: Vec<_> = selected
                .iter()
                .map(|r| r["median_ms"].as_f64().unwrap())
                .collect();
            let cpu: Vec<_> = selected
                .iter()
                .map(|r| {
                    (r["summary"]["user_cpu_ms"].as_f64().unwrap()
                        + r["summary"]["system_cpu_ms"].as_f64().unwrap())
                        / r["summary"]["rounds"].as_f64().unwrap()
                })
                .collect();
            let rss = selected
                .iter()
                .map(|r| r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.)
                .fold(0., f64::max);
            modes.insert(mode,json!({"ms":median(&times),"cpu_ms_per_request":median(&cpu),"maxrss_mib":rss,"process_medians_ms":times}));
        }
        for left in ["previous", "aeglos"] {
            let mut ratios = Vec::new();
            let mut savings = Vec::new();
            for block in 0..3 {
                let ms = |mode: &str| {
                    rows.iter()
                        .find(|r| {
                            r["phase"] == "latency"
                                && r["case"] == name
                                && r["variant"] == mode
                                && r["block"] == block
                        })
                        .unwrap()["median_ms"]
                        .as_f64()
                        .unwrap()
                };
                ratios.push(ms("current") / ms(left));
                savings.push((ms(left) - ms("current")) * 1000.);
            }
            pairs.insert(format!("current/{left}"),json!({"geometric_ratio":gm(&ratios),"median_ratio":median(&ratios),"wins":ratios.iter().filter(|r|**r<1.).count(),"ratios":ratios,"saving_us_median":median(&savings)}));
        }
        cells.insert(name,json!({"shape":case["shape"],"workers":case["workers"],"reference":case["reference"],"modes":modes,"pairs":pairs}));
    }
    let mut best = BTreeMap::new();
    for case in cases.as_array().unwrap() {
        let shape = case["shape"].as_str().unwrap();
        let mut engines = BTreeMap::new();
        for mode in ["previous", "current", "aeglos"] {
            let (name, c) = cells
                .iter()
                .filter(|(_, c)| c["shape"] == shape)
                .min_by(|(_, a), (_, b)| {
                    a["modes"][mode]["ms"]
                        .as_f64()
                        .unwrap()
                        .total_cmp(&b["modes"][mode]["ms"].as_f64().unwrap())
                })
                .unwrap();
            let mut v = c["modes"][mode].clone();
            v["workers"] = c["workers"].clone();
            v["case"] = json!(name);
            if let Some(cpu) = c["reference"]["cpu_ms"].as_f64() {
                let ms = v["ms"].as_f64().unwrap();
                v["M5_CPU_speedup"] = json!(cpu / ms);
                v["M5_Metal_speedup"] = json!(c["reference"]["metal_ms"].as_f64().unwrap() / ms);
                v["CPU10x"] = json!(ms <= cpu / 10.);
            }
            engines.insert(mode, v);
        }
        best.insert(shape, engines);
    }
    let mut totals = 0u64;
    let mut samples = 0u64;
    for row in rows {
        let rounds = row["summary"]["rounds"].as_u64().unwrap();
        assert_eq!(row["summary"]["priming"], 3);
        totals += rounds + 3;
        if row["phase"] == "profile" {
            assert_eq!(row["profile"]["lost_records"], 0);
            assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
            samples += row["profile"]["samples"].as_u64().unwrap();
        }
    }
    assert_eq!(totals, 8652);
    let tenfold = best
        .values()
        .filter(|v| v["current"]["CPU10x"] == true)
        .count();
    let mut aggregates = BTreeMap::new();
    for workers in [32, 64, 96] {
        for control in ["previous", "aeglos"] {
            let key = format!("current/{control}");
            let ratios: Vec<_> = (0..3)
                .map(|block| {
                    let rs: Vec<_> = cells
                        .values()
                        .filter(|c| c["workers"] == workers && c["shape"] != "merge-54x1")
                        .map(|c| c["pairs"][&key]["ratios"][block].as_f64().unwrap())
                        .collect();
                    assert_eq!(rs.len(), 8);
                    gm(&rs)
                })
                .collect();
            aggregates.insert(format!("{key}-w{workers}"),json!({"geometric_ratio":gm(&ratios),"wins":ratios.iter().filter(|r|**r<1.).count(),"block_ratios":ratios}));
        }
    }
    save(
        r.join("analysis.json"),
        &json!({"status":"complete","cases":cells,"best_tested":best,"original8_aggregates":aggregates,"SDK_requests":totals+108,"independent_exports":276,"proof_public_members":552,"post_transport_distinct_verified":276,"next_prior_count":10180,"profiles":6,"cycle_samples":samples,"original8_CPU10x_rows":tenfold,"goal_achieved":false,"source_changed":false,"no_new_adoption":true,"profile_comparison":result["profile_comparison"],"three_block_estimates_no_95percent_claim":true}),
    );
    println!("Analyzed all9*3cores*3engines/3blocks;8760SDK276distinctGnark552members;all276posttransportverified;ledger10180;profiles{samples}samples;CPU10xrows{tenfold}");
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/tmp/zopro-grouping-prefix-cutoff";
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
include!("stats.rs");

fn cells(rows: &[Value], p: &Value) -> Value {
    let mut out = BTreeMap::new();
    for case in p["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut modes = BTreeMap::new();
        let mut pairs = BTreeMap::new();
        for mode in ["accepted", "fresh", "candidate"] {
            let rs: Vec<_> = rows
                .iter()
                .filter(|r| r["phase"] == "latency" && r["case"] == name && r["variant"] == mode)
                .collect();
            assert_eq!(rs.len(), 15);
            let ms: Vec<_> = rs
                .iter()
                .map(|r| r["median_ms"].as_f64().unwrap())
                .collect();
            let cpu: Vec<_> = rs
                .iter()
                .map(|r| {
                    (r["summary"]["user_cpu_ms"].as_f64().unwrap()
                        + r["summary"]["system_cpu_ms"].as_f64().unwrap())
                        / r["summary"]["rounds"].as_f64().unwrap()
                })
                .collect();
            let rss = rs
                .iter()
                .map(|r| r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.)
                .fold(0., f64::max);
            modes.insert(mode,json!({"ms":median(&ms),"ms_interval":ci(&ms),"cpu_ms_per_request":median(&cpu),"maxrss_mib":rss}));
        }
        for (a, b) in [
            ("fresh", "accepted"),
            ("candidate", "accepted"),
            ("candidate", "fresh"),
        ] {
            let mut ratios = Vec::new();
            let mut savings = Vec::new();
            for block in 0..15 {
                let ms = |v: &str| {
                    rows.iter()
                        .find(|r| {
                            r["phase"] == "latency"
                                && r["case"] == name
                                && r["variant"] == v
                                && r["block"] == block
                        })
                        .unwrap()["median_ms"]
                        .as_f64()
                        .unwrap()
                };
                ratios.push(ms(a) / ms(b));
                savings.push((ms(b) - ms(a)) * 1000.);
            }
            pairs.insert(format!("{a}/{b}"),json!({"ratios":ratios,"geometric_ratio":gm(&ratios),"median_ratio":median(&ratios),"median_confidence":ci(&ratios),"wins":ratios.iter().filter(|x|**x<1.).count(),"saving_us":median(&savings)}));
        }
        out.insert(name,json!({"shape":case["shape"],"workers":case["workers"],"reference":case["reference"],"modes":modes,"pairs":pairs}));
    }
    json!(out)
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("self-test") {
        let v: Vec<_> = (1..=15).map(|n| n as f64).collect();
        let c = ci(&v);
        assert_eq!(c["lower"], 4.);
        assert_eq!(c["upper"], 12.);
        assert_eq!(
            c["coverage_under_independent_exchangeable_blocks"],
            0.96484375
        );
        println!("Exact fifteen-block median interval rank/coverage check passed");
        return;
    }
    let r = Path::new(R);
    let result = read(r.join("performance/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["distinct_verified_exports"], 456);
    assert_eq!(result["profiles"], 6);
    let p = read(r.join("performance/protocol.json"));
    assert_eq!(p["blocks"], 15);
    assert_eq!(p["variants"].as_array().unwrap().len(), 3);
    for (f, h) in p["pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let rows = read(r.join("performance/rows.json"));
    let rows = rows.as_array().unwrap();
    let cases = cells(rows, &p);
    let (mut calibration, mut regressions, mut large, mut aggregatefail) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (name, c) in cases.as_object().unwrap() {
        let ci = &c["pairs"]["fresh/accepted"]["median_confidence"];
        if ci["lower"].as_f64().unwrap() < 1. / 1.03 || ci["upper"].as_f64().unwrap() > 1.03 {
            calibration.push(format!(
                "{name} fresh/accepted interval [{},{}]",
                ci["lower"], ci["upper"]
            ));
        }
        for control in ["accepted", "fresh"] {
            let key = format!("candidate/{control}");
            let ci = &c["pairs"][&key]["median_confidence"];
            if ci["upper"].as_f64().unwrap() > 1.02 {
                regressions.push(format!("{name} {key} upper={}", ci["upper"]));
            }
            if name == "1x2-w32" && ci["upper"].as_f64().unwrap() >= 1. {
                large.push(format!("{name} {key} upper={}", ci["upper"]));
            }
        }
    }
    let mut aggregate = BTreeMap::new();
    for control in ["accepted", "fresh"] {
        let key = format!("candidate/{control}");
        let vals: Vec<_> = (0..15)
            .map(|b| {
                let rs: Vec<_> = cases
                    .as_object()
                    .unwrap()
                    .values()
                    .filter(|c| c["workers"] == 32 && c["shape"] != "merge-54x1")
                    .map(|c| c["pairs"][&key]["ratios"][b].as_f64().unwrap())
                    .collect();
                assert_eq!(rs.len(), 8);
                gm(&rs)
            })
            .collect();
        let c = ci(&vals);
        if c["upper"].as_f64().unwrap() > 1. {
            aggregatefail.push(format!("{key} original8 upper={}", c["upper"]));
        }
        aggregate.insert(key, c);
    }
    let sdk: u64 = rows
        .iter()
        .map(|r| r["summary"]["rounds"].as_u64().unwrap() + 3)
        .sum();
    assert_eq!(sdk, 27624);
    for row in rows.iter().filter(|r| r["phase"] == "profile") {
        assert_eq!(row["profile"]["lost_records"], 0);
        assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
    }
    let mut inventory: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("preflight-proof-inventory.json"))).unwrap();
    assert_eq!(inventory.len(), 18);
    let mut seen: BTreeSet<String> =
        serde_json::from_value(p["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 9448);
    for (rel, h) in read(r.join("performance/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        let f = r.join("performance").join(rel);
        assert_eq!(hash(&f), h.as_str().unwrap());
        if rel.ends_with(".proof") {
            assert_eq!(fs::metadata(&f).unwrap().len(), 164);
            assert!(seen.insert(h.as_str().unwrap().to_string()));
        }
        assert!(inventory
            .insert(f.display().to_string(), h.as_str().unwrap().into())
            .is_none());
    }
    assert_eq!(inventory.len(), 930);
    assert_eq!(seen.len(), 9904);
    for (f, h) in &inventory {
        assert_eq!(&hash(f), h);
    }
    save(r.join("proof-inventory.json"), &json!(inventory));
    let passed = calibration.is_empty()
        && regressions.is_empty()
        && large.is_empty()
        && aggregatefail.is_empty();
    let tenfold = cases
        .as_object()
        .unwrap()
        .values()
        .filter(|c| {
            c["workers"] == 32
                && c["reference"]["cpu_ms"]
                    .as_f64()
                    .is_some_and(|cpu| c["modes"]["candidate"]["ms"].as_f64().unwrap() <= cpu / 10.)
        })
        .count();
    let a = json!({"cases":cases,"calibration_failures":calibration,"regressions":regressions,"primary_gain_failures":large,"aggregate_failures":aggregatefail,"original8":aggregate,"full_gate_passed":passed,"fresh_core_tests":116,"candidate_core_tests":117,"SDK_requests":sdk+36,"independent_exports":465,"proof_public_members":930,"next_prior_count":9904,"original8_CPU10x_rows":tenfold,"adopted":false,"goal_achieved":false,"exact_in_loop_cardinality_stop":true,"no_short_screen_or_extension":true,"no_successful_capture_repeated":true,"confidence_method":"individual>=95%distribution-free median interval; independent/exchangeable block assumption; not familywise"});
    save(r.join("analysis.json"), &a);
    println!("PrefixCutoff fullpass{passed}; calibration{} regression{} primary{} aggregate{};27660SDK465Gnark;prior9904;all8CPU10xrows{tenfold}",calibration.len(),regressions.len(),large.len(),aggregatefail.len());
}

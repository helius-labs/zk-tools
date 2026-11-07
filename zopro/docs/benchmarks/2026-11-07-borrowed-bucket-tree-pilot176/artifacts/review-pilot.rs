use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176/native/primary-pilot";
fn load(s: &str) -> Value {
    serde_json::from_slice(&fs::read(s).unwrap()).unwrap()
}
fn hash(s: &str) -> String {
    let mut f = fs::File::open(s).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-11 * a.abs().max(1.));
}
fn med(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn main() {
    let protocol = load(&format!("{R}/protocol.json"));
    let analysis = load(&format!("{R}/latency-analysis.json"));
    let diag = load(&format!("{R}/diagnostic-analysis.json"));
    let jobs = load(&format!("{R}/jobs.json"));
    let mut pins = 0;
    for (path, sha) in load(&format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(path), sha.as_str().unwrap());
        pins += 1;
    }
    let mut receipt_count = 0;
    for stage in ["latency", "diagnostic"] {
        let plan = load(&format!("{R}/{stage}-plan.json"));
        assert_eq!(
            load(&format!("{R}/{stage}-runs/result.json"))["status"],
            "complete"
        );
        let names = load(&format!("{R}/{stage}-runs/completed.json"));
        assert_eq!(
            names.as_array().unwrap().len(),
            plan["steps"].as_array().unwrap().len()
        );
        for s in plan["steps"].as_array().unwrap() {
            let name = s["name"].as_str().unwrap();
            assert!(names.as_array().unwrap().iter().any(|n| n == name));
            let base = format!("{R}/{stage}-runs/{name}");
            let cmd = load(&format!("{base}/command.json"));
            let got = cmd["argv"].as_array().unwrap();
            let expected = s["argv"].as_array().unwrap();
            assert_eq!(&got[got.len() - expected.len()..], expected);
            for (k, v) in s["environment"]
                .as_object()
                .unwrap_or(&serde_json::Map::new())
            {
                assert_eq!(&cmd["environment"][k], v);
            }
            let e = load(&format!("{base}/exit.json"));
            assert_eq!(e["success"], true);
            assert_eq!(e["returncode"], 0);
            for stream in ["stdout", "stderr"] {
                assert_eq!(
                    hash(&format!("{base}/{stream}.log")),
                    e[format!("{stream}_sha256")].as_str().unwrap()
                );
            }
            receipt_count += 1;
        }
    }
    let mut ledger: BTreeSet<String> =
        serde_json::from_value(load(protocol["baseline_ledger"].as_str().unwrap())).unwrap();
    assert_eq!(ledger.len(), 31648);
    let mut means = BTreeMap::<String, BTreeMap<u64, f64>>::new();
    let mut exported = 0;
    let mut requests = 0;
    let mut profile_summaries = BTreeMap::new();
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let stage = j["stage"].as_str().unwrap();
        let arm = j["arm"].as_str().unwrap();
        let rounds = j["rounds"].as_u64().unwrap();
        let raw = fs::read_to_string(format!("{R}/{stage}-runs/{name}/stdout.log")).unwrap();
        let mut times = vec![];
        let mut summary = vec![];
        for line in raw.lines() {
            if let Some(s) = line.strip_prefix("SAMPLE ") {
                let v: Value = serde_json::from_str(s).unwrap();
                assert_eq!(v["round"], times.len());
                let t = v["ms"].as_f64().unwrap();
                assert!(t > 0. && t.is_finite());
                times.push(t);
            }
            if let Some(s) = line.strip_prefix("SUMMARY ") {
                summary.push(serde_json::from_str::<Value>(s).unwrap());
            }
        }
        assert_eq!(times.len() as u64, rounds);
        assert_eq!(summary.len(), 1);
        assert_eq!(summary[0]["rounds"], rounds);
        assert_eq!(summary[0]["workers"], 32);
        assert_eq!(summary[0]["priming"], 3);
        requests += rounds + 3;
        let avg = times.iter().sum::<f64>() / rounds as f64;
        let rows = if stage == "latency" {
            &analysis["rows"]
        } else {
            &diag["rows"]
        };
        let saved = rows
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["job"]["name"] == name)
            .unwrap();
        close(avg, saved["mean_ms"].as_f64().unwrap());
        assert_eq!(summary[0], saved["summary"]);
        if stage == "latency" {
            assert!(means
                .entry(arm.to_string())
                .or_default()
                .insert(j["block"].as_u64().unwrap(), avg)
                .is_none());
        }
        let proof = format!("{R}/proofs/{name}/0.proof");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        let ph = hash(&proof);
        let pu = hash(&format!("{R}/proofs/{name}/0.public"));
        assert_eq!(pu, protocol["binding"]["public_sha256"].as_str().unwrap());
        assert!(ledger.insert(ph.clone()));
        let verification =
            fs::read_to_string(format!("{R}/{stage}-runs/verify-{name}/stdout.log")).unwrap();
        let lines: Vec<_> = verification
            .lines()
            .filter(|l| l.starts_with("VERIFIED "))
            .collect();
        assert_eq!(lines.len(), 1);
        for (k, v) in [
            ("proof_sha256", ph.as_str()),
            ("public_sha256", pu.as_str()),
            (
                "image_sha256",
                protocol["binding"]["image_sha256"].as_str().unwrap(),
            ),
            ("vk_sha256", saved["vk_sha256"].as_str().unwrap()),
        ] {
            assert!(lines[0].split_whitespace().any(|s| s == format!("{k}={v}")));
        }
        exported += 1;
    }
    assert_eq!(requests, 7794);
    assert_eq!(exported, 51);
    assert_eq!(ledger.len(), 31699);
    let saved: BTreeSet<String> =
        serde_json::from_value(load(&format!("{R}/proof-sha256-after-diagnostic.json"))).unwrap();
    assert_eq!(ledger, saved);
    for arm in ["accepted", "OFF", "ON"] {
        assert_eq!(means[arm].len(), 15);
        let mut v: Vec<_> = means[arm].values().copied().collect();
        close(
            med(&mut v),
            analysis["arms"][arm]["median_process_mean_ms"]
                .as_f64()
                .unwrap(),
        );
    }
    let mut checks = BTreeMap::new();
    for (a, b, key) in [
        ("OFF", "accepted", "OFF_over_accepted"),
        ("ON", "OFF", "ON_over_OFF"),
        ("ON", "accepted", "ON_over_accepted"),
    ] {
        let mut ratios: Vec<_> = (0..15).map(|i| means[a][&i] / means[b][&i]).collect();
        let wins = ratios.iter().filter(|r| **r < 1.).count();
        ratios.sort_by(f64::total_cmp);
        for (index, k) in [(3, "lower"), (7, "median"), (11, "upper")] {
            close(ratios[index], analysis[key][k].as_f64().unwrap());
        }
        assert_eq!(wins as u64, analysis[key]["wins"].as_u64().unwrap());
        checks.insert(
            key,
            json!({"median":ratios[7],"lower":ratios[3],"upper":ratios[11],"wins":wins}),
        );
    }
    let calibration = checks["OFF_over_accepted"]["lower"].as_f64().unwrap() >= 1. / 1.03
        && checks["OFF_over_accepted"]["upper"].as_f64().unwrap() <= 1.03;
    let mut gate = calibration;
    for key in ["ON_over_OFF", "ON_over_accepted"] {
        gate &= checks[key]["upper"].as_f64().unwrap() < 1.
            && checks[key]["wins"].as_u64().unwrap() >= 10;
    }
    assert_eq!(analysis["calibration_passed"], calibration);
    assert_eq!(analysis["pilot_gate_passed"], gate);

    for arm in ["accepted", "OFF", "ON"] {
        let csv = fs::read_to_string(format!("{R}/stat/stat-{arm}.csv")).unwrap();
        let mut count = 0;
        for l in csv.lines().filter(|s| !s.is_empty() && !s.starts_with('#')) {
            let c: Vec<_> = l.split(';').collect();
            let v = c[0].parse::<u64>().unwrap();
            assert_eq!(diag["stats"][arm][c[2]]["value"], v);
            assert_eq!(c[4].parse::<f64>().unwrap(), 100.);
            count += 1;
        }
        assert_eq!(count, 4);
        let raw = fs::read_to_string(format!(
            "{R}/diagnostic-runs/decode-cycles-{arm}/stdout.log"
        ))
        .unwrap();
        let mut n = 0;
        let mut unknown = 0;
        let mut symbols = BTreeMap::<String, u64>::new();
        for l in raw.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
            let fields: Vec<_> = l.split_whitespace().collect();
            let (pid, tid) = fields[0].split_once('/').unwrap();
            assert!(pid.parse::<u32>().unwrap() > 0 && tid.parse::<u32>().unwrap() > 0);
            assert!(
                fields[1]
                    .trim_matches(&['[', ']'][..])
                    .parse::<u32>()
                    .unwrap()
                    < 32
            );
            assert_eq!(fields[3], "16000000");
            let symbol = fields[5..].join(" ");
            if symbol.contains("unknown") {
                unknown += 1;
            }
            *symbols
                .entry(format!("{:#}", rustc_demangle::demangle(&symbol)))
                .or_default() += 1;
            n += 1;
        }
        assert_eq!(diag["profiles"][arm]["samples"], n);
        assert_eq!(diag["profiles"][arm]["unknown_samples"], unknown);
        let loss = load(&format!("{R}/diagnostic-runs/loss-cycles-{arm}/stdout.log"));
        assert_eq!(loss["zero_lost"], true);
        let mut rank: Vec<_> = symbols.into_iter().collect();
        rank.sort_by(|a, b| b.1.cmp(&a.1));
        profile_summaries.insert(arm,json!({"samples":n,"unknown":unknown,"top_symbols_demangled":rank.into_iter().take(15).collect::<Vec<_>>()}));
    }
    let on = &diag["stats"]["ON"];
    let off = &diag["stats"]["OFF"];
    let mut ratios = BTreeMap::new();
    for event in [
        "cycles:u",
        "instructions:u",
        "cache-misses:u",
        "branch-misses:u",
    ] {
        ratios.insert(
            event,
            on[event]["value"].as_f64().unwrap() / off[event]["value"].as_f64().unwrap(),
        );
    }
    let output = json!({"review_passed":true,"source_pin_checks":pins,"capture_receipts":receipt_count,"latency_gate_independently_reconstructed":true,"calibration_passed":calibration,"pilot_gate_passed":gate,"paired_checks":checks,"SDK_requests":requests,"fresh_exports":exported,"canonical_ledger_count":31699,"profile_summaries":profile_summaries,"counter_ON_over_OFF":ratios,"limitations":"Independent offline arithmetic/parser review, not a second cryptographic verifier, instruction decoder, perf LOST decoder or exact IP-to-ELF analysis. Nominal marginal intervals assume independent exchangeable blocks. No capture, proof, verifier or successful model replayed."});
    assert!(!std::path::Path::new(&format!("{R}/independent-review.json")).exists());
    fs::write(
        format!("{R}/independent-review.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!(
        "review passed:117capture receipts;51fresh exports;7794requests;pilot result reviewed"
    );
}

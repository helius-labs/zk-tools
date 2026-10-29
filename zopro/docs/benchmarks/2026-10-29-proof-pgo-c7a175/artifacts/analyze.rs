use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-c7a175";
const P: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-qualification175";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
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
fn median(xs: &[f64]) -> f64 {
    assert!(!xs.is_empty());
    let mut v = xs.to_vec();
    assert!(v.iter().all(|x| x.is_finite()));
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 0 {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.
    } else {
        v[v.len() / 2]
    }
}
fn rank(xs: &[f64]) -> Value {
    assert_eq!(xs.len(), 15);
    assert!(xs.iter().all(|x| x.is_finite() && *x > 0.));
    let mut s = xs.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[7],"low":s[3],"high":s[11],"wins":xs.iter().filter(|&&r|r<1.).count(),"ratios":xs,"nominal_individual_coverage_if_independent_exchangeable":0.96484375})
}
fn cloud() -> PathBuf {
    r("remote-results/output/measure")
}
fn prepare() {
    assert!(!r("local-verification-proofs").exists());
    let inv = read(cloud().join("proof-inventory.json"));
    assert_eq!(inv.as_object().unwrap().len(), 2442);
    let mut copies = vec![];
    let mut i = 0;
    for (p, h) in inv.as_object().unwrap() {
        let from = r("remote-results").join(p.strip_prefix('/').unwrap());
        assert_eq!(hash(&from), h.as_str().unwrap());
        if !p.ends_with(".proof") {
            continue;
        }
        assert_eq!(fs::metadata(&from).unwrap().len(), 164);
        let stem = from.parent().unwrap().file_name().unwrap();
        let out = r("local-verification-proofs").join(stem);
        fs::create_dir_all(&out).unwrap();
        for extension in ["proof", "public"] {
            let source = from.with_extension(extension);
            let to = out.join(format!("{i}.{extension}"));
            fs::copy(&source, &to).unwrap();
            assert_eq!(hash(&source), hash(&to));
            copies.push(json!({"source":source,"copy":to,"sha256":hash(to),"new_export":false}));
        }
        i += 1;
    }
    assert_eq!(i, 1221);
    save(r("verification-copy-inventory.json"), &json!(copies));
    println!("Staged1221hashchecked existingcloudproofs;no newexports");
}
fn pair(a: &Value, b: &Value) -> Value {
    let aa = a["blocks"].as_array().unwrap();
    let bb = b["blocks"].as_array().unwrap();
    assert_eq!(aa.len(), 15);
    assert_eq!(bb.len(), 15);
    let ratios: Vec<_> = aa
        .iter()
        .zip(bb)
        .enumerate()
        .map(|(i, (a, b))| {
            assert_eq!(a["block"], i);
            assert_eq!(b["block"], i);
            a["ms"].as_f64().unwrap() / b["ms"].as_f64().unwrap()
        })
        .collect();
    let differences: Vec<_> = aa
        .iter()
        .zip(bb)
        .map(|(a, b)| b["ms"].as_f64().unwrap() - a["ms"].as_f64().unwrap())
        .collect();
    let mut p = rank(&ratios);
    p["paired_saving_ms"] = json!(median(&differences));
    p
}
fn event_map(v: &Value, key: &str) -> BTreeMap<String, u64> {
    v[key]
        .as_str()
        .unwrap()
        .lines()
        .map(|l| {
            let f: Vec<_> = l.split_whitespace().collect();
            assert_eq!(f.len(), 2);
            (f[0].into(), f[1].parse().unwrap())
        })
        .collect()
}
fn finish() {
    assert_eq!(read(r("measure-result.json"))["Status"], "Success");
    assert_eq!(read(r("post-transport-gnark-exit.json"))["success"], true);
    let post = fs::read_to_string(r("post-transport-gnark.log")).unwrap();
    assert!(post.contains("verified 1221 distinct proofs across 9 pinned keys"));
    assert_eq!(read(cloud().join("complete.json"))["sdk_requests"], 37164);
    assert_eq!(read(cloud().join("complete.json"))["proof_ledger"], 20509);
    assert_eq!(
        read(cloud().join("workflow/result.json"))["status"],
        "complete"
    );
    assert_eq!(
        read(cloud().join("workflow/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        2460
    );
    let before = read(cloud().join("before.json"));
    let after = read(cloud().join("after.json"));
    let mut events = BTreeMap::new();
    for key in [
        "/sys/fs/cgroup/cpu.stat",
        "/sys/fs/cgroup/memory.events",
        "/sys/fs/cgroup/memory.swap.events",
    ] {
        let b = event_map(&before, key);
        let a = event_map(&after, key);
        events.insert(
            key,
            a.iter()
                .map(|(k, v)| (k.clone(), v.checked_sub(b[k]).unwrap()))
                .collect::<BTreeMap<_, _>>(),
        );
    }
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(Path::new(P).join("prior-proof-sha256.json"))).unwrap();
    assert_eq!(seen.len(), 18847);
    for (p, h) in read(Path::new(P).join("proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(seen.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(seen.len(), 19261);
    let mut inventory = BTreeMap::new();
    for (base, expected, remote) in [
        (r("local-output/preflight"), 54, false),
        (cloud(), 2442, true),
    ] {
        let inv = read(base.join("proof-inventory.json"));
        assert_eq!(inv.as_object().unwrap().len(), expected);
        for (p, h) in inv.as_object().unwrap() {
            let local = if remote {
                r("remote-results").join(p.strip_prefix('/').unwrap())
            } else {
                r("local-output").join(p.strip_prefix("/output/").unwrap())
            };
            assert_eq!(hash(&local), h.as_str().unwrap());
            if p.ends_with(".proof") {
                assert!(seen.insert(h.as_str().unwrap().into()));
                assert_eq!(fs::metadata(&local).unwrap().len(), 164);
            }
            assert!(inventory
                .insert(local.display().to_string(), h.clone())
                .is_none());
        }
    }
    assert_eq!(seen.len(), 20509);
    assert_eq!(inventory.len(), 2496);
    save(r("proof-inventory.json"), &json!(inventory));
    save(r("proof-sha256-after.json"), &json!(seen));
    let rows = read(cloud().join("rows.json"));
    assert_eq!(rows.as_array().unwrap().len(), 1221);
    let mut grouped: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut sdk_total = 0;
    for row in rows.as_array().unwrap() {
        let j = &row["job"];
        let s = &row["sdk"]["summary"];
        let rounds = j["rounds"].as_u64().unwrap();
        assert_eq!(s["rounds"], rounds);
        assert_eq!(s["priming"], 3);
        assert_eq!(s["workers"], j["workers"]);
        sdk_total += rounds + 3;
        let samples = row["sdk"]["samples_ms"].as_array().unwrap();
        assert_eq!(samples.len(), rounds as usize);
        assert!(samples.iter().all(|s| s.as_f64().unwrap() > 0.));
        if j["phase"] == "profile" {
            let p = &row["profile"];
            assert_eq!(p["lost_records"], 0);
            assert_eq!(p["sampled_pids"].as_array().unwrap().len(), 1);
            profiles.insert(j["name"].as_str().unwrap().to_owned(), p.clone());
            continue;
        }
        assert_eq!(j["phase"], "latency");
        let case = format!(
            "{}-w{}",
            j["case"]["shape"].as_str().unwrap(),
            j["workers"].as_u64().unwrap()
        );
        let cpu = (s["user_cpu_ms"].as_f64().unwrap() + s["system_cpu_ms"].as_f64().unwrap())
            / rounds as f64;
        grouped.entry(format!("{case}/{}",j["variant"].as_str().unwrap())).or_default().push(json!({"block":j["block"],"ms":row["sdk"]["median_ms"],"CPU_ms":cpu,"RSS_MiB":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"summary":s,"name":j["name"]}));
    }
    assert_eq!(sdk_total, 37164);
    assert_eq!(profiles.len(), 6);
    assert_eq!(grouped.len(), 81);
    let mut cells = BTreeMap::new();
    for (k, mut xs) in grouped {
        xs.sort_by_key(|x| x["block"].as_u64().unwrap());
        assert_eq!(xs.len(), 15);
        for (i, x) in xs.iter().enumerate() {
            assert_eq!(x["block"], i);
        }
        let series = |key: &str| {
            xs.iter()
                .map(|x| x[key].as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        cells.insert(k,json!({"ms":median(&series("ms")),"CPU_ms":median(&series("CPU_ms")),"median_RSS_MiB":median(&series("RSS_MiB")),"max_RSS_MiB":series("RSS_MiB").into_iter().fold(0.,f64::max),"blocks":xs}));
    }
    let manifest = read(r("context/manifest.json"));
    let mut pairs = BTreeMap::new();
    let mut nonreg = BTreeMap::new();
    let mut aeglos_wins = BTreeMap::new();
    for c in manifest["cases"].as_array().unwrap() {
        for w in [32, 64, 96] {
            let case = format!("{}-w{w}", c["shape"].as_str().unwrap());
            for (a, b) in [("PGO", "plain"), ("plain", "aeglos"), ("PGO", "aeglos")] {
                let p = pair(
                    &cells[&format!("{case}/{a}")],
                    &cells[&format!("{case}/{b}")],
                );
                if b == "plain" {
                    nonreg.insert(case.clone(), p["high"].as_f64().unwrap() <= 1.03);
                } else {
                    aeglos_wins.insert(
                        format!("{case}/{a}"),
                        p["high"].as_f64().unwrap() < 1. && p["wins"].as_u64().unwrap() >= 10,
                    );
                }
                pairs.insert(format!("{case}/{a}/{b}"), p);
            }
        }
    }
    let mut aggregates = BTreeMap::new();
    let mut aggregate_checks = BTreeMap::new();
    for w in [32, 64, 96] {
        let mut rs = vec![];
        for b in 0..15 {
            let vals: Vec<_> = manifest["cases"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|c| !c["reference"].is_null())
                .map(|c| {
                    let name = format!("{}-w{w}/PGO/plain", c["shape"].as_str().unwrap());
                    pairs[&name]["ratios"][b].as_f64().unwrap().ln()
                })
                .collect();
            assert_eq!(vals.len(), 8);
            rs.push((vals.iter().sum::<f64>() / 8.).exp());
        }
        let p = rank(&rs);
        aggregate_checks.insert(
            format!("all8-w{w}"),
            p["high"].as_f64().unwrap() < 1. && p["wins"].as_u64().unwrap() >= 10,
        );
        aggregates.insert(format!("all8-w{w}/PGO/plain"), p);
    }
    let primary = &pairs["1x2-w32/PGO/plain"];
    let primary_gain =
        primary["high"].as_f64().unwrap() < 1. && primary["wins"].as_u64().unwrap() >= 10;
    let gate = primary_gain && nonreg.values().all(|v| *v) && aggregate_checks.values().all(|v| *v);
    let mut comparisons = BTreeMap::new();
    let mut count = BTreeMap::from([("plain", 0usize), ("PGO", 0), ("aeglos", 0)]);
    for c in manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| !c["reference"].is_null())
    {
        let shape = c["shape"].as_str().unwrap();
        let cpu = c["reference"]["cpu_ms"].as_f64().unwrap();
        let metal = c["reference"]["metal_ms"].as_f64().unwrap();
        let mut modes = BTreeMap::new();
        for v in ["plain", "PGO", "aeglos"] {
            let (w, t) = [32, 64, 96]
                .into_iter()
                .map(|w| {
                    (
                        w,
                        cells[&format!("{shape}-w{w}/{v}")]["ms"].as_f64().unwrap(),
                    )
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .unwrap();
            if t <= cpu / 10. {
                *count.get_mut(v).unwrap() += 1;
            }
            modes.insert(v,json!({"best_tested_workers_descriptive":w,"ms":t,"CPU_speedup":cpu/t,"Metal_speedup":metal/t,"CPU10x":t<=cpu/10.}));
        }
        comparisons.insert(shape,json!({"reference_CPU_ms":cpu,"reference_Metal_ms":metal,"CPU10x_target_ms":cpu/10.,"variants":modes}));
    }
    assert_eq!(comparisons.len(), 8);
    assert!(count["PGO"] < 8 && count["plain"] < 8);
    let a = json!({"complete":true,"scope":"all9shapes32/64/96workers15blocks;plain175znver4/PGOprofiletransfer/pinnedAeglos0290613Sorted","source_files_exact":175,"SDK_requests":37272,"SDK_cloud":37164,"SDK_local_preflight":108,"new_independent_exports":1248,"proof_ledger":20509,"post_transport_verified":1221,"proof_public_members":2496,"cells":cells,"pairs":pairs,"aggregates":aggregates,"primary32_gain_passed":primary_gain,"nonregression_checks":nonreg,"aggregate_gain_checks":aggregate_checks,"performance_gate_passed":gate,"same_worker_strict_Aeglos_win_checks":aeglos_wins,"profiles":profiles,"cgroup_events_whole_workflow_deltas":events,"reference_comparisons_best_tested_descriptive":comparisons,"original8_CPU10x_rows_best_tested_descriptive":count,"adopted":false,"production_changed":false,"goal_achieved":false,"goal_status":"active_unmet","no_successful_captures_repeated":true});
    save(r("analysis.json"), &a);
    println!("GATE={gate};primary32gain={primary_gain};nonregfailures={:?};aggregatechecks={:?};SDK37272/Gnark1248/ledger20509",nonreg.iter().filter(|(_,v)|!**v).map(|(k,_)|k).collect::<Vec<_>>(),aggregate_checks);
    for case in ["1x2", "merge-54x1"] {
        for w in [32, 64, 96] {
            for v in ["plain", "PGO", "aeglos"] {
                let c = &cells[&format!("{case}-w{w}/{v}")];
                println!(
                    "{case}w{w}/{v}: {:.6}ms CPU{:.3} RSSmax{:.2}",
                    c["ms"].as_f64().unwrap(),
                    c["CPU_ms"].as_f64().unwrap(),
                    c["max_RSS_MiB"].as_f64().unwrap()
                );
            }
        }
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare|finish"),
    }
}

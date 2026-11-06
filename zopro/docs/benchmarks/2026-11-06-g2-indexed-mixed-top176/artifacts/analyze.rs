use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-indexed-mixed-top176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const ARMS: [&str; 3] = ["FROZEN", "OFF", "ON"];
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn stats(v: &[f64]) -> Value {
    assert_eq!(v.len(), 15);
    let mut v2 = v.to_vec();
    v2.sort_by(f64::total_cmp);
    json!({"median":v2[7],"low":v2[3],"high":v2[11],"wins":v.iter().filter(|&&v|v<1.).count(),"values":v})
}
fn caller(j: &Value) -> Value {
    let base = j["reused_from"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| r("runs").join(j["name"].as_str().unwrap()));
    let e = read(base.join("exit.json"));
    assert_eq!(e["returncode"], 0);
    assert_eq!(e["success"], true);
    for k in ["stdout", "stderr"] {
        assert_eq!(
            hash(base.join(format!("{k}.log"))),
            e[format!("{k}_sha256")]
        );
    }
    let text = fs::read_to_string(base.join("stdout.log")).unwrap();
    let rows = text
        .lines()
        .filter_map(|l| l.strip_prefix("CALLER "))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 1);
    let v: Value = serde_json::from_str(rows[0]).unwrap();
    assert_eq!(v["query"], j["actual_query"]);
    assert_eq!(v["workers"], j["case"]["workers"]);
    assert_eq!(v["rounds"], j["rounds"]);
    for k in [
        "all_timed_outputs_checked",
        "every_output_matches_independent_full_MSM",
        "point_inputs_unchanged",
        "normalization_inside_timer",
    ] {
        assert_eq!(v[k], true);
    }
    assert_eq!(v["B_word_reset_inside_timer"], j["case"]["query"] == "B");
    assert_eq!(
        v["metadata"]["route"],
        if j["variant"] == "ON" && j["case"]["query"] == "B" {
            328
        } else {
            0
        }
    );
    assert_eq!(
        v["samples_ns"].as_array().unwrap().len(),
        j["rounds"].as_u64().unwrap() as usize
    );
    v
}
fn main() {
    assert!(!r("analysis.json").exists());
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let mut pins = 0;
    for f in ["algebra-pins.json", "build-pins.json", "capture-pins.json"] {
        for (p, h) in read(r(f)).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins += 1;
        }
    }
    let m=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let audit = read(r("source-audit.json"));
    let mut originals_exact = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
        let b = fs::read(r("source").join(p)).unwrap();
        if format!("{:x}", Sha256::digest(&b)) == h.as_str().unwrap() {
            originals_exact += 1;
        }
        let restored = if let Some(s) = audit["suffixes"].get(p) {
            String::from_utf8(b)
                .unwrap()
                .strip_suffix(s.as_str().unwrap())
                .unwrap()
                .as_bytes()
                .to_vec()
        } else {
            b
        };
        assert_eq!(
            format!("{:x}", Sha256::digest(restored)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(originals_exact, 171);
    for (_, v) in audit["clones"].as_object().unwrap() {
        let mut recovered = v["candidate"].as_str().unwrap().to_string();
        for pair in v["edits"].as_array().unwrap().iter().rev() {
            let from = pair[1].as_str().unwrap();
            assert_eq!(recovered.matches(from).count(), 1);
            recovered = recovered.replacen(from, pair[0].as_str().unwrap(), 1);
        }
        assert_eq!(recovered, v["original"]);
    }
    let mut commands = 0;
    for (plan, out) in [
        ("algebra-plan.json", "algebra-check"),
        ("build-plan.json", "native"),
        ("capture-plan.json", "runs"),
    ] {
        for s in read(r(plan))["steps"].as_array().unwrap() {
            let base = r(out).join(s["name"].as_str().unwrap());
            let e = read(base.join("exit.json"));
            assert_eq!(e["success"], true);
            assert_eq!(e["returncode"], 0);
            for f in ["stdout", "stderr"] {
                assert_eq!(
                    hash(base.join(format!("{f}.log"))),
                    e[format!("{f}_sha256")]
                );
            }
            commands += 1;
        }
    }
    let jobs = read(r("jobs.json"));
    let mut rows = vec![];
    let mut all = BTreeMap::new();
    let mut times = BTreeMap::<(String, String), Vec<f64>>::new();
    let mut maxrss = BTreeMap::<(String, String), f64>::new();
    let mut cpu = BTreeMap::<(String, String), Vec<f64>>::new();
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    let mut total = 0u64;
    let mut reused = 0u64;
    let mut samples_all = 0u64;
    let mut failed = vec![];
    for j in jobs.as_array().unwrap() {
        let n = j["name"].as_str().unwrap();
        let v = caller(j);
        let rounds = j["rounds"].as_u64().unwrap();
        total += rounds + 4;
        if j.get("reused_from").is_some() {
            reused += rounds + 4;
        }
        if j["panel"] == "latency" {
            let key = (
                j["case"]["name"].as_str().unwrap().to_string(),
                j["variant"].as_str().unwrap().to_string(),
            );
            let ns = v["samples_ns"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap() as f64)
                .collect::<Vec<_>>();
            let ts = times.entry(key.clone()).or_default();
            assert_eq!(ts.len(), j["block"].as_u64().unwrap() as usize);
            ts.push(median(&ns));
            let rss = v["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.;
            maxrss
                .entry(key.clone())
                .and_modify(|m| *m = m.max(rss))
                .or_insert(rss);
            cpu.entry(key).or_default().push(
                (v["usage"]["user_cpu_ms"].as_f64().unwrap()
                    + v["usage"]["system_cpu_ms"].as_f64().unwrap())
                    / rounds as f64,
            );
        }
        if j["panel"] == "stat" {
            let mut events = BTreeMap::new();
            for line in fs::read_to_string(j["data"].as_str().unwrap())
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = line.split(';').collect::<Vec<_>>();
                if f.len() < 5
                    || ![
                        "cycles:u",
                        "instructions:u",
                        "cache-misses:u",
                        "branch-misses:u",
                    ]
                    .contains(&f[2].trim())
                {
                    continue;
                }
                let count = f[0].trim().parse::<u64>().unwrap();
                let coverage = f[4].trim().parse::<f64>().unwrap();
                assert!(coverage >= 99.9);
                events.insert(f[2].trim().to_string(),json!({"count":count,"per_query":count as f64/rounds as f64,"scheduled_percent":coverage}));
            }
            assert_eq!(events.len(), 4);
            counters.insert(n.to_string(), json!(events));
        }
        if j["panel"] == "cycles" {
            let attrs = fs::read_to_string(r(format!("runs/events-{n}/stdout.log"))).unwrap();
            for s in ["cycles:u", "16000000", "CPU", "TIME", "PERIOD"] {
                assert!(attrs.contains(s));
            }
            assert!(!attrs.contains("freq: 1"));
            let pid = v["pid"].as_u64().unwrap();
            let worker = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| (v["tid"].as_u64().unwrap(), v["cpu"].as_u64().unwrap()))
                .collect::<BTreeMap<_, _>>();
            let tids = worker.keys().copied().chain([pid]).collect::<BTreeSet<_>>();
            let workers = v["workers"].as_u64().unwrap();
            let meta = fs::read_to_string(r(format!("runs/metadata-{n}/stdout.log"))).unwrap();
            let mappings = meta
                .lines()
                .filter(|l| {
                    l.contains("PERF_RECORD_MMAP") && l.contains(j["binary"].as_str().unwrap())
                })
                .map(|l| {
                    l.split_whitespace()
                        .next()
                        .unwrap()
                        .split('/')
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<BTreeSet<_>>();
            assert_eq!(mappings, BTreeSet::from([pid]));
            let mut forked = BTreeSet::from([pid]);
            for l in meta.lines() {
                if let Some((_, s)) = l.split_once("PERF_RECORD_FORK(") {
                    let a = s
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if a[0] == pid {
                        forked.insert(a[1]);
                    }
                }
            }
            assert!(tids.is_subset(&forked));
            let own = read(r(format!("runs/own-{n}/stdout.log")));
            assert_eq!(own["sha256_before"], own["sha256_after"]);
            let data = j["data"].as_str().unwrap();
            assert_eq!(hash(data), own["sha256_before"]);
            assert_eq!(
                read(r(format!("runs/loss-{n}/stdout.log")))["zero_lost"],
                true
            );
            let mut bins = BTreeMap::<String, u64>::new();
            let mut count = 0u64;
            let mut unknown = 0u64;
            for l in fs::read_to_string(r(format!("runs/decode-{n}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let (p, t) = f[0].split_once('/').unwrap();
                let tid = t.parse::<u64>().unwrap();
                assert_eq!(p.parse::<u64>().unwrap(), pid);
                assert!(tids.contains(&tid));
                let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                assert!(cpu < workers);
                if tid != pid {
                    assert_eq!(worker[&tid], cpu);
                }
                assert_eq!(f[3], "16000000");
                let raw = f[5..].join(" ");
                let sym = rustc_demangle::try_demangle(&raw)
                    .map(|v| format!("{v:#}"))
                    .unwrap_or(raw);
                if sym.starts_with("[unknown]") {
                    unknown += 1;
                }
                *bins.entry(sym).or_default() += 1;
                count += 1;
            }
            let qualified = count >= 1000 && (unknown as f64 / count as f64) <= 0.05;
            if !qualified {
                failed.push(format!("{n}/profile"));
            }
            samples_all += count;
            profiles.insert(n.to_string(),json!({"samples":count,"unknown":unknown,"qualified":qualified,"zero_lost":true,"pid":pid,"tids":tids,"raw_sha256":hash(data),"usage":v["usage"]}));
            symbols.insert(n.to_string(), json!(bins));
        }
        all.insert(n.to_string(), v);
    }
    let cases = read(r("cases.json"));
    let mut ratios = BTreeMap::<(String, String), Vec<f64>>::new();
    for c in cases.as_array().unwrap() {
        let name = c["name"].as_str().unwrap();
        let mut medians = BTreeMap::new();
        let mut rss = BTreeMap::new();
        let mut service = BTreeMap::new();
        for arm in ARMS {
            let key = (name.to_string(), arm.to_string());
            assert_eq!(times[&key].len(), 15);
            medians.insert(arm, median(&times[&key]) / 1e6);
            rss.insert(arm, maxrss[&key]);
            service.insert(arm, median(&cpu[&key]));
        }
        let mut pairs = BTreeMap::new();
        for (num, den) in [("OFF", "FROZEN"), ("ON", "OFF"), ("ON", "FROZEN")] {
            let values = times[&(name.into(), num.into())]
                .iter()
                .zip(&times[&(name.into(), den.into())])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let s = stats(&values);
            let pass = if num == "OFF" {
                s["low"].as_f64().unwrap() >= 1. / 1.03 && s["high"].as_f64().unwrap() <= 1.03
            } else {
                s["high"].as_f64().unwrap() <= 1.03
            };
            if !pass {
                failed.push(format!(
                    "{name}/{num}/{den}/{}",
                    if num == "OFF" {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if num == "ON"
                && name == "1x2-B-w32"
                && (s["high"].as_f64().unwrap() >= 1. || s["wins"].as_u64().unwrap() < 10)
            {
                failed.push(format!("{name}/{num}/{den}/primary-gain"));
            }
            ratios.insert((name.into(), format!("{num}/{den}")), values);
            pairs.insert(format!("{num}/{den}"), s);
        }
        rows.push(json!({"case":name,"median_ms":medians,"max_process_RSS_MiB":rss,"median_CPU_ms_per_query":service,"comparisons":pairs}));
    }
    let mut aggregates = BTreeMap::new();
    for den in ["OFF", "FROZEN"] {
        let values = (0..15)
            .map(|b| {
                let a = ratios[&("1x2-B-w32".into(), format!("ON/{den}"))][b];
                let c = ratios[&("2x2-B-w32".into(), format!("ON/{den}"))][b];
                ((a.ln() + c.ln()) / 2.).exp()
            })
            .collect::<Vec<_>>();
        let s = stats(&values);
        if s["high"].as_f64().unwrap() >= 1. || s["wins"].as_u64().unwrap() < 10 {
            failed.push(format!("B32-two-shape/ON/{den}/gain"));
        }
        aggregates.insert(format!("ON/{den}"), s);
    }
    assert_eq!(all.len(), 348);
    assert_eq!(commands, 377);
    assert_eq!(total, 14112);
    assert_eq!(reused, 35);
    save(r("symbols.json"), &json!(symbols));
    save(r("callers.json"), &json!(all));
    let summary = json!({"gate_passed":failed.is_empty(),"failed_checks":failed,"adopted":false,"production_changed":false,"goal_achieved":false,"new_proofs":0,"canonical_proof_ledger":30846,"native_tests":128,"new_native_tests":3,"prior_algebra_tests":4,"snapshot_files":178,"originals_exact":originals_exact,"all_originals_recovered":true,"six_clones_inverse_recovered":true,"pin_occurrences_checked":pins,"successful_commands":commands,"caller_rows":348,"new_caller_processes":341,"reused_preflights":7,"complete_query_calls":total,"new_complete_query_calls":total-reused,"rows":rows,"aggregates":aggregates,"counters":counters,"profiles":profiles,"profile_samples":samples_all});
    save(r("analysis.json"), &summary);
    println!(
        "{}",
        json!({"gate_passed":summary["gate_passed"],"failed_checks":summary["failed_checks"],"rows":summary["rows"],"aggregates":summary["aggregates"],"profile_samples":samples_all})
    );
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/solver-a-stream176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
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
fn caller(name: &str) -> Value {
    let out = if name.starts_with("latency-") { "capture" } else { "perf-fixed" };
    let s = fs::read_to_string(r(format!("{out}/{name}/stdout.log"))).unwrap();
    let v = s
        .lines()
        .filter_map(|l| l.strip_prefix("CALLER "))
        .collect::<Vec<_>>();
    assert_eq!(v.len(), 1);
    serde_json::from_str(v[0]).unwrap()
}
fn median(a: &[f64]) -> f64 {
    let mut a = a.to_vec();
    a.sort_by(f64::total_cmp);
    a[a.len() / 2]
}
fn main() {
    assert!(!r("analysis.json").exists());
    assert_eq!(read(r("perf-fixed/result.json"))["status"], "complete");
    let mut pin_count = 0;
    for f in [
        "build-pins.json",
        "preflight-pins.json",
        "capture-pins.json",
        "perf-fixed-pins.json",
    ] {
        for (p, h) in read(r(f)).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
            pin_count += 1;
        }
    }
    let manifest=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let audit = read(r("source-audit.json"));
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
        let b = fs::read(r("source").join(p)).unwrap();
        if format!("{:x}", Sha256::digest(&b)) == h.as_str().unwrap() {
            exact += 1;
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
    assert_eq!(exact, 171);
    let ready = fs::read_to_string(r("source/crates/zopro-core/src/solver/early_feed.rs")).unwrap();
    let start = ready.find("fn solve_with_a_feed<").unwrap();
    let mut recovered = ready[start..]
        .trim_end()
        .strip_suffix('}')
        .unwrap()
        .trim_end()
        .to_string();
    for change in audit["solver_clone_edits"].as_array().unwrap().iter().rev() {
        let a = change[0].as_str().unwrap();
        let b = change[1].as_str().unwrap();
        assert_eq!(recovered.matches(b).count(), 1);
        recovered = recovered.replacen(b, a, 1);
    }
    assert_eq!(
        recovered,
        fs::read_to_string(r("original-solver.rs")).unwrap()
    );
    let mut commands = 0;
    for (plan, out) in [
        ("build-plan.json", "native"),
        ("preflight-plan.json", "preflight"),
        ("capture-plan.json", "capture"),
        ("perf-fixed-plan.json", "perf-fixed"),
    ] {
        for s in read(r(plan))["steps"].as_array().unwrap() {
            let n = s["name"].as_str().unwrap();
            if out == "capture" && !n.starts_with("latency-") { continue; }
            let base = r(format!("{out}/{n}"));
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
    let failed_exit = read(r("capture/stat-w16-off/exit.json"));
    assert_eq!(failed_exit["success"], false);
    assert_eq!(failed_exit["returncode"], 101);
    for f in ["stdout", "stderr"] {
        assert_eq!(hash(r(format!("capture/stat-w16-off/{f}.log"))), failed_exit[format!("{f}_sha256")]);
    }
    let mut rows = vec![];
    let mut failed = vec![];
    let mut all_callers = BTreeMap::new();
    let mut calls = 0usize;
    for w in [16, 32] {
        let mut off = vec![];
        let mut on = vec![];
        let mut ratios = vec![];
        let mut rss = 0f64;
        let mut early = vec![];
        let mut jobs = vec![];
        let mut forecasts = BTreeSet::new();
        for b in 0..15 {
            let name = format!("latency-b{b:02}-w{w}");
            let v = caller(&name);
            assert_eq!(v["all_results_match_independent_A_MSM"], true);
            assert_eq!(v["rounds"], 31);
            assert_eq!(v["samples"].as_array().unwrap().len(), 62);
            assert_eq!(v["metadata"].as_array().unwrap().len(), 62);
            let mut ts = [vec![], vec![]];
            for (s, m) in v["samples"]
                .as_array()
                .unwrap()
                .iter()
                .zip(v["metadata"].as_array().unwrap())
            {
                let enabled = s["on"].as_bool().unwrap();
                let i = usize::from(enabled);
                ts[i].push(s["ns"].as_f64().unwrap());
                assert_eq!(m["streamed"], enabled);
                assert_eq!(m["admission_fallback"], false);
                assert_eq!(m["grouped_fallback"], false);
                forecasts.insert(m["forecast_bytes"].as_u64().unwrap());
                if enabled {
                    assert_eq!(m["published_chunks"], 21);
                    assert_eq!(m["early_chunks"], 19);
                    assert_eq!(m["early_values"], 18958);
                    early.push(m["feeds_before_solver_return"].as_f64().unwrap());
                    jobs.push(m["jobs"].as_f64().unwrap());
                }
            }
            assert_eq!(ts[0].len(), 31);
            assert_eq!(ts[1].len(), 31);
            let a = median(&ts[0]);
            let c = median(&ts[1]);
            off.push(a);
            on.push(c);
            ratios.push(c / a);
            rss = rss.max(v["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
            calls += 62 + 6 + 2;
            all_callers.insert(name, v);
        }
        let mut sorted = ratios.clone();
        sorted.sort_by(f64::total_cmp);
        let wins = ratios.iter().filter(|&&r| r < 1.).count();
        let pass = if w == 32 {
            sorted[11] < 1. && wins >= 10
        } else {
            sorted[11] <= 1.02
        };
        if !pass {
            failed.push(format!(
                "w{w}/{}",
                if w == 32 { "gain" } else { "nonregression" }
            ));
        }
        rows.push(json!({"workers":w,"OFF_median_ms":median(&off)/1e6,"ON_median_ms":median(&on)/1e6,"block_OFF_ns":off,"block_ON_ns":on,"paired_ON_OFF":{"median":sorted[7],"low":sorted[3],"high":sorted[11],"wins":wins,"all_ratios":ratios},"max_mixed_process_RSS_MiB":rss,"feeds_before_solver_return":{"min":early.iter().copied().fold(f64::INFINITY,f64::min),"median":median(&early),"max":early.iter().copied().fold(0.,f64::max)},"jobs":{"min":jobs.iter().copied().fold(f64::INFINITY,f64::min),"median":median(&jobs),"max":jobs.iter().copied().fold(0.,f64::max)},"forecast_bytes":forecasts,"passed":pass}));
    }
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    let mut total_samples = 0;
    for w in [16, 32] {
        for mode in ["off", "on"] {
            let name = format!("stat-w{w}-{mode}");
            let mut events = BTreeMap::new();
            for l in fs::read_to_string(r(format!("stat/{name}.csv")))
                .unwrap()
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
            {
                let f = l.split(';').collect::<Vec<_>>();
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
                let scheduled = f[4].trim().parse::<f64>().unwrap();
                assert!(scheduled >= 99.9);
                events.insert(f[2].trim().to_string(),json!({"count":count,"per_call":count as f64/4096.,"scheduled_percent":scheduled}));
            }
            assert_eq!(events.len(), 4);
            counters.insert(name.clone(), json!(events));
            let v = caller(&name);
            assert_eq!(v["all_results_match_independent_A_MSM"], true);
            assert_eq!(v["rounds"], 4096);
            calls += 4096 + 8;
            all_callers.insert(name, v);
            let name = format!("cycles-w{w}-{mode}");
            let v = caller(&name);
            assert_eq!(v["all_results_match_independent_A_MSM"], true);
            assert_eq!(v["rounds"], 4096);
            calls += 4096 + 8;
            let pid = v["pid"].as_u64().unwrap();
            let mut tids = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v["tid"].as_u64().unwrap())
                .collect::<BTreeSet<_>>();
            tids.insert(pid);
            let attrs = fs::read_to_string(r(format!("perf-fixed/events-{name}/stdout.log"))).unwrap();
            for s in ["cycles:u", "16000000", "CPU", "TIME", "PERIOD"] {
                assert!(attrs.contains(s));
            }
            assert!(!attrs.contains("freq: 1"));
            let metadata =
                fs::read_to_string(r(format!("perf-fixed/metadata-{name}/stdout.log"))).unwrap();
            let owners = metadata
                .lines()
                .filter(|l| {
                    l.contains("PERF_RECORD_MMAP") && l.contains(r("caller-bin").to_str().unwrap())
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
            assert_eq!(owners, BTreeSet::from([pid]));
            let mut forked = BTreeSet::from([pid]);
            for l in metadata.lines() {
                if let Some((_, t)) = l.split_once("PERF_RECORD_FORK(") {
                    let p = t
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if p[0] == pid {
                        forked.insert(p[1]);
                    }
                }
            }
            assert!(tids.is_subset(&forked));
            assert_eq!(
                read(r(format!("perf-fixed/loss-{name}/stdout.log")))["zero_lost"],
                true
            );
            let own = read(r(format!("perf-fixed/own-{name}/stdout.log")));
            let data = r(format!("perf/{name}.data"));
            assert_eq!(own["sha256_before"], own["sha256_after"]);
            assert_eq!(hash(&data), own["sha256_before"]);
            let mut bins = BTreeMap::<String, u64>::new();
            let mut count = 0u64;
            let mut unknown = 0u64;
            for l in fs::read_to_string(r(format!("perf-fixed/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let (p, t) = f[0].split_once('/').unwrap();
                assert_eq!(p.parse::<u64>().unwrap(), pid);
                assert!(tids.contains(&t.parse::<u64>().unwrap()));
                assert!(f[1].trim_matches(['[', ']']).parse::<usize>().unwrap() < w);
                assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
                let raw = f[5..].join(" ");
                let sym = rustc_demangle::try_demangle(&raw)
                    .map(|s| format!("{s:#}"))
                    .unwrap_or(raw);
                if sym.starts_with("[unknown]") {
                    unknown += 1;
                }
                *bins.entry(sym).or_default() += 1;
                count += 1;
            }
            let qualified = count >= 400 && (unknown as f64 / count as f64) <= 0.05;
            if !qualified {
                failed.push(format!("{name}/profile"));
            }
            total_samples += count;
            symbols.insert(name.clone(), json!(bins));
            profiles.insert(name.clone(),json!({"pid":pid,"tids":tids,"samples":count,"unknown":unknown,"qualified":qualified,"raw_sha256":hash(data),"zero_lost":true,"usage":v["usage"]}));
            all_callers.insert(name, v);
        }
    }
    // Each functional primary calls OFF/ON twice, then admission/runtime fallback;
    // invalid size returns before solve. Unsupported2x2 executes no solver→A call.
    let functional_calls = 4 * 6;
    save(r("profile-symbols.json"), &json!(symbols));
    let out = json!({"necessary_component_gate_passed":failed.is_empty(),"failed_checks":failed,"adopted":false,"production_changed":false,"goal_achieved":false,"new_proofs":0,"canonical_proof_ledger":30846,"source_files":179,"originals_exact":exact,"all_originals_recovered":true,"solver_clone_exact_after_hook_recovery":true,"native_tests":130,"commands_checked":commands,"pin_occurrences_checked":pin_count,"measured_solver_A_calls":30*62+8*4096,"capture_total_solver_A_calls":calls,"functional_solver_A_calls":functional_calls,"total_solver_A_calls":calls+functional_calls,"rows":rows,"counters":counters,"profiles":profiles,"total_profile_samples":total_samples,"caller_receipts":all_callers});
    save(r("analysis.json"), &out);
    println!(
        "{}",
        json!({"component_gate":out["necessary_component_gate_passed"],"failed_checks":out["failed_checks"],"rows":out["rows"],"profile_samples":total_samples,"total_solver_A_calls":calls+functional_calls})
    );
}

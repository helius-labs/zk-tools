use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-capture176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-callers176";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
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
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn same(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-11 * (1. + a.abs()), "{a} != {b}");
}
fn sorted(v: &[f64]) -> Vec<f64> {
    let mut out = v.to_vec();
    out.sort_by(f64::total_cmp);
    out
}
fn validate_interval(raw: &[f64], reported: &Value) {
    assert_eq!(raw.len(), 15);
    let s = sorted(raw);
    same(s[3], num(&reported["low"]));
    same(s[7], num(&reported["median"]));
    same(s[11], num(&reported["high"]));
    assert_eq!(reported["wins"], raw.iter().filter(|&&v| v < 1.).count());
    for (a, b) in raw.iter().zip(reported["ratios"].as_array().unwrap()) {
        same(*a, num(b));
    }
}
fn get_caller(name: &str) -> Value {
    let s = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let a = s
        .lines()
        .filter_map(|l| l.strip_prefix("CALLER "))
        .collect::<Vec<_>>();
    assert_eq!(a.len(), 1);
    serde_json::from_str(a[0]).unwrap()
}
fn main() {
    assert!(!r("review.json").exists());
    let plan = load(r("plan.json"));
    let analysis = load(r("analysis.json"));
    let pins = load(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(digest(p), h.as_str().unwrap());
    }
    assert_eq!(load(r("runs/result.json"))["status"], "complete");
    let ids = load(r("identities.json"));
    let native = load(Path::new(Q).join("qualification.json"));
    assert_eq!(native["status"], "pass");
    assert_eq!(native["native177_recoverable"], true);
    assert_eq!(native["source_files"], 178);
    assert_eq!(native["core_ON_unfiltered_tests"], 126);
    for arm in ["frozen", "OFF", "ON"] {
        assert_eq!(
            digest(ids[arm]["binary"].as_str().unwrap()),
            ids[arm]["binary_sha256"]
        );
    }
    let mut calls = BTreeMap::new();
    let mut commands = BTreeSet::new();
    let mut command_classes = BTreeMap::<String, usize>::new();
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        assert!(commands.insert(name.to_string()));
        let b = r(format!("runs/{name}"));
        let exit = load(b.join("exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let command = load(b.join("command.json"));
        let argv = command["argv"].as_array().unwrap();
        assert_eq!(
            &argv[..4],
            &[json!("timeout"), json!("-k"), json!("5s"), json!("600s")]
        );
        assert_eq!(&argv[4..], step["argv"].as_array().unwrap());
        let mut env = plan["environment"].as_object().unwrap().clone();
        if let Some(extra) = step["environment"].as_object() {
            env.extend(extra.clone());
        }
        assert_eq!(command["environment"], json!(env));
        for s in ["stdout", "stderr"] {
            assert_eq!(
                digest(b.join(format!("{s}.log"))),
                exit[format!("{s}_sha256")]
            );
        }
        let class = if name == "preflight-check" {
            "check"
        } else {
            name.split('-').next().unwrap()
        };
        *command_classes.entry(class.to_string()).or_default() += 1;
        if ["preflight", "latency", "stat", "cycles"].contains(&class) {
            let v = get_caller(name);
            assert_eq!(v, analysis["caller_receipts"][name]);
            assert_eq!(v["complete_generator_oracle"], true);
            assert_eq!(v["timed_allocations"], 0);
            assert_eq!(v["priming_passes"], 3);
            assert_eq!(v["priming_rounds"], 128);
            let arm = name.rsplit('-').next().unwrap();
            let bin = ids[arm]["binary"].as_str().unwrap();
            let pos = argv.iter().position(|a| a == bin).unwrap();
            assert_eq!(argv[pos - 3], "taskset");
            assert_eq!(argv[pos - 2], "-c");
            assert_eq!(v["kind"], argv[pos + 1]);
            assert_eq!(
                num(&v["size"]),
                argv[pos + 2].as_str().unwrap().parse::<f64>().unwrap()
            );
            let workers = v["workers"].as_u64().unwrap() as usize;
            assert_eq!(
                workers,
                argv[pos + 3].as_str().unwrap().parse::<usize>().unwrap()
            );
            assert_eq!(
                argv[pos - 1],
                if workers == 1 {
                    "2"
                } else if workers == 16 {
                    "0-15"
                } else {
                    "0-31"
                }
            );
            assert_eq!(
                v["rounds_per_worker"].as_u64().unwrap(),
                argv[pos + 4].as_str().unwrap().parse::<u64>().unwrap()
            );
            assert_eq!(v["arm"], argv[pos + 5]);
            assert_eq!(v["arm"], if arm == "ON" { "ON" } else { "OFF" });
            assert_eq!(v["new_proofs"], 0);
            let rows = v["worker_rows"].as_array().unwrap();
            assert_eq!(rows.len(), workers);
            let mut seen = BTreeSet::new();
            for (i, row) in rows.iter().enumerate() {
                assert_eq!(row["worker"], i);
                assert_eq!(row["cpu"], if workers == 1 { 2 } else { i });
                assert!(seen.insert(row["tid"].as_u64().unwrap()));
            }
            let cap = v["scratch_capacity_bytes_by_worker"].as_array().unwrap();
            assert_eq!(cap.len(), workers);
            assert!(cap.iter().all(|a| a == &cap[0]));
            calls.insert(name.to_string(), v);
        }
    }
    assert_eq!(commands.len(), 1279);
    assert_eq!(calls.len(), 1188);
    assert_eq!(command_classes,json!({"preflight":72,"check":1,"latency":1080,"stat":18,"cycles":18,"own":18,"decode":18,"metadata":18,"events":18,"loss":18}).as_object().unwrap().iter().map(|(k,v)|(k.clone(),v.as_u64().unwrap()as usize)).collect());
    for (name, v) in &calls {
        let (prefix, _) = name.rsplit_once('-').unwrap();
        for arm in ["frozen", "OFF", "ON"] {
            let other = &calls[&format!("{prefix}-{arm}")];
            assert_eq!(v["output_sha256"], other["output_sha256"]);
            assert_eq!(
                v["scratch_capacity_bytes_by_worker"],
                other["scratch_capacity_bytes_by_worker"]
            );
        }
    }
    let rows = analysis["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 24);
    let mut failures = BTreeSet::new();
    let mut ratios = BTreeMap::new();
    let mut timing_checks = 0;
    let mut context_rows = vec![];
    for kind in ["G1", "G2"] {
        for size in [200, 201, 352, 353] {
            for workers in [1, 16, 32] {
                let name = format!("{kind}-{size}-w{workers}");
                let row = rows.iter().find(|v| v["context"] == name).unwrap();
                let rounds = if size <= 201 { 32768 } else { 16384 };
                assert_eq!(row["rounds_per_worker"], rounds);
                let mut vectors = BTreeMap::new();
                for arm in ["frozen", "OFF", "ON"] {
                    let v = (0..15)
                        .map(|b| {
                            let p = &calls[&format!("latency-{name}-b{b:02}-{arm}")];
                            assert_eq!(p["rounds_per_worker"], rounds);
                            assert_eq!(p["workers"], workers);
                            assert_eq!(p["kind"], kind);
                            assert_eq!(p["size"], size);
                            num(&p["elapsed_ns"]) / rounds as f64
                        })
                        .collect::<Vec<_>>();
                    same(sorted(&v)[7], num(&row["median_ns_per_wave"][arm]));
                    vectors.insert(arm, v);
                }
                for (num_arm, den) in [("OFF", "frozen"), ("ON", "OFF"), ("ON", "frozen")] {
                    let v = (0..15)
                        .map(|i| vectors[num_arm][i] / vectors[den][i])
                        .collect::<Vec<_>>();
                    let s = sorted(&v);
                    validate_interval(&v, &row["pairs"][format!("{num_arm}/{den}")]);
                    timing_checks += 1;
                    let calibrated = num_arm == "OFF";
                    let ok = if calibrated {
                        s[3] >= 0.97 && s[11] <= 1.03
                    } else {
                        s[11] <= 1.01
                    };
                    if !ok {
                        failures.insert(format!(
                            "{name}/{num_arm}/{den}/{}",
                            if calibrated {
                                "calibration"
                            } else {
                                "nonregression"
                            }
                        ));
                    }
                    ratios.insert(format!("{name}/{num_arm}/{den}"), v);
                }
                context_rows.push(json!({"context":name,"pairs":row["pairs"],"scratch_capacity_bytes_first_worker":row["scratch_capacity_bytes_first_worker"],"median_busy_CPUs":row["median_busy_CPUs"],"median_minor_faults_per_wave":row["median_minor_faults_per_wave"]}));
            }
        }
    }
    for scope in ["LOGICAL32", "ALL"] {
        for den in ["frozen", "OFF"] {
            let selected = ratios
                .iter()
                .filter(|(key, _)| {
                    key.ends_with(&format!("/ON/{den}"))
                        && (scope == "ALL" || key.contains("-w32/"))
                })
                .collect::<Vec<_>>();
            assert_eq!(selected.len(), if scope == "ALL" { 24 } else { 8 });
            let values = (0..15)
                .map(|b| {
                    (selected.iter().map(|(_, v)| v[b].ln()).sum::<f64>() / selected.len() as f64)
                        .exp()
                })
                .collect::<Vec<_>>();
            validate_interval(&values, &analysis["aggregates"][format!("{scope}/{den}")]);
            let s = sorted(&values);
            if s[11] >= 1. || values.iter().filter(|&&v| v < 1.).count() < 11 {
                failures.insert(format!("{scope}-aggregate/ON/{den}/gain"));
            }
            timing_checks += 1;
        }
    }
    assert_eq!(timing_checks, 76);
    assert_eq!(analysis["timing_checks"], 76);
    let mut profile_summary = BTreeMap::new();
    let mut total_samples = 0;
    let mut event_diagnostics = BTreeMap::new();
    for kind in ["G1", "G2"] {
        for (size, workers) in [(200, 1), (200, 32), (352, 32)] {
            let context = format!("{kind}-{size}-w{workers}");
            for arm in ["frozen", "OFF", "ON"] {
                let stat = format!("stat-{context}-{arm}");
                let stat_rounds = if workers == 1 { 524288 } else { 16384 };
                assert_eq!(calls[&stat]["rounds_per_worker"], stat_rounds);
                let mut counts = BTreeMap::new();
                for line in fs::read_to_string(r(format!("stat/{stat}.csv")))
                    .unwrap()
                    .lines()
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
                    let event = f[2].trim();
                    let count = f[0].trim().parse::<f64>().unwrap();
                    let coverage = f[4].trim().parse::<f64>().unwrap();
                    assert!(coverage >= 90.);
                    let reported = &analysis["counters"][&stat][event];
                    same(count, num(&reported["count"]));
                    same(coverage, num(&reported["coverage_percent"]));
                    same(count / stat_rounds as f64, num(&reported["per_wave"]));
                    assert_eq!(reported["qualified"], true);
                    counts.insert(event.to_string(), count / stat_rounds as f64);
                }
                assert_eq!(counts.len(), 4);
                event_diagnostics.insert(stat.clone(), json!(counts));
                let name = format!("cycles-{context}-{arm}");
                let caller = &calls[&name];
                assert_eq!(caller["rounds_per_worker"], stat_rounds * 4);
                let pid = caller["pid"].as_u64().unwrap();
                let assigned = caller["worker_rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| (v["tid"].as_u64().unwrap(), v["cpu"].as_u64().unwrap()))
                    .collect::<BTreeMap<_, _>>();
                let meta =
                    fs::read_to_string(r(format!("runs/metadata-{name}/stdout.log"))).unwrap();
                let mut owned = BTreeSet::from([pid]);
                let mut mapping_pids = BTreeSet::new();
                for l in meta.lines() {
                    if l.contains("PERF_RECORD_MMAP")
                        && l.contains(ids[arm]["binary"].as_str().unwrap())
                    {
                        mapping_pids.insert(
                            l.split_whitespace()
                                .next()
                                .unwrap()
                                .split('/')
                                .next()
                                .unwrap()
                                .parse::<u64>()
                                .unwrap(),
                        );
                    }
                    if let Some((_, s)) = l.split_once("PERF_RECORD_FORK(") {
                        let f = s
                            .split(')')
                            .next()
                            .unwrap()
                            .split(':')
                            .map(|v| v.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();
                        if f[0] == pid {
                            owned.insert(f[1]);
                        }
                    }
                }
                assert_eq!(mapping_pids, BTreeSet::from([pid]));
                assert!(assigned.keys().all(|t| owned.contains(t)));
                let attrs =
                    fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();
                for word in ["cycles:u", "16000000", "CPU", "PERIOD", "TIME"] {
                    assert!(attrs.contains(word));
                }
                assert!(!attrs.contains("freq: 1"));
                assert_eq!(
                    load(r(format!("runs/loss-{name}/stdout.log")))["zero_lost"],
                    true
                );
                let owner = load(r(format!("runs/own-{name}/stdout.log")));
                assert_eq!(owner["sha256_before"], owner["sha256_after"]);
                assert_eq!(
                    digest(r(format!("perf/{name}.data"))),
                    owner["sha256_before"]
                );
                let mut n = 0u64;
                let mut unknown = 0u64;
                let mut worker = 0u64;
                let mut symbols = BTreeMap::<String, u64>::new();
                for l in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
                    .unwrap()
                    .lines()
                    .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
                {
                    let f = l.split_whitespace().collect::<Vec<_>>();
                    let (p, t) = f[0].split_once('/').unwrap();
                    assert_eq!(p.parse::<u64>().unwrap(), pid);
                    let tid = t.parse::<u64>().unwrap();
                    assert!(owned.contains(&tid));
                    let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                    assert!(if workers == 1 { cpu == 2 } else { cpu < 32 });
                    if let Some(want) = assigned.get(&tid) {
                        assert_eq!(*want, cpu);
                        worker += 1;
                    }
                    assert_eq!(f[3], "16000000");
                    let raw = f[5..].join(" ");
                    let symbol = rustc_demangle::try_demangle(&raw)
                        .map(|v| format!("{v:#}"))
                        .unwrap_or(raw);
                    if symbol.starts_with("[unknown]") {
                        unknown += 1;
                    }
                    *symbols.entry(symbol).or_default() += 1;
                    n += 1;
                }
                assert!(n >= 1000);
                assert!(unknown as f64 / n as f64 <= 0.05);
                assert!(worker as f64 / n as f64 >= 0.95);
                let target = symbols
                    .iter()
                    .filter(|(s, _)| {
                        s.contains("msm::stream_g1_impl")
                            || (s.contains("msm::g2_borrowed::apply")
                                || s.contains("msm::g2_validated::apply"))
                            || s.contains("inverse_nonzero")
                            || s.contains("zopro_bn254::inverse::inverse")
                    })
                    .map(|(_, n)| *n)
                    .sum::<u64>();
                assert!(target as f64 / n as f64 >= 0.95);
                let reported = &analysis["profiles"][&name];
                assert_eq!(reported["target_samples"], target);
                same(num(&reported["target_share"]), target as f64 / n as f64);
                assert_eq!(reported["samples"], n);
                same(num(&reported["unknown_share"]), unknown as f64 / n as f64);
                same(num(&reported["worker_share"]), worker as f64 / n as f64);
                assert_eq!(reported["qualified"], true);
                assert_eq!(reported["raw_sha256"], owner["sha256_before"]);
                assert_eq!(json!(symbols), load(r("profile-symbols.json"))[&name]);
                total_samples += n;
                let inverse_samples = symbols
                    .iter()
                    .filter(|(s, _)| s.contains("inverse_nonzero"))
                    .map(|(_, n)| *n)
                    .sum::<u64>();
                profile_summary.insert(name,json!({"samples":n,"worker_samples":worker,"unknown_samples":unknown,"inverse_nonzero_samples":inverse_samples,"inverse_nonzero_flat_share":inverse_samples as f64/n as f64,"qualified":true}));
            }
        }
    }
    let expected_failures = analysis["failed_checks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(failures, expected_failures);
    assert_eq!(analysis["gate_passed"], failures.is_empty());
    assert_eq!(analysis["new_proofs"], 0);
    assert_eq!(analysis["proof_ledger"], 30846);
    assert_eq!(analysis["adopted"], false);
    assert_eq!(analysis["production_changed"], false);
    let mut pmu_ratios = BTreeMap::new();
    for kind in ["G1", "G2"] {
        for (size, w) in [(200, 1), (200, 32), (352, 32)] {
            let context = format!("{kind}-{size}-w{w}");
            let mut events = BTreeMap::new();
            for event in [
                "cycles:u",
                "instructions:u",
                "cache-misses:u",
                "branch-misses:u",
            ] {
                let on = num(&event_diagnostics[&format!("stat-{context}-ON")][event]);
                let off = num(&event_diagnostics[&format!("stat-{context}-OFF")][event]);
                events.insert(
                    event,
                    json!({"ON_per_wave":on,"OFF_per_wave":off,"ON_OFF_ratio":on/off}),
                );
            }
            pmu_ratios.insert(context, json!(events));
        }
    }
    let out = json!({"review_passed":true,"pins_checked":pins.as_object().unwrap().len(),"commands_checked":commands.len(),"callers_checked":calls.len(),"timing_checks":timing_checks,"failed_timing_checks":failures,"profile_count":profile_summary.len(),"total_profile_samples":total_samples,"profiles":profile_summary,"PMU_ON_OFF_diagnostics":pmu_ratios,"gate_passed":failures.is_empty(),"adopted":false,"new_proofs":0,"proof_ledger":30846,"contexts":context_rows,"analysis_sha256":digest(r("analysis.json")),"plan_sha256":digest(r("plan.json")),"reviewer_source_sha256":digest(r("review.rs")),"reviewer_binary_sha256":digest(r("review"))});
    fs::write(r("review.json"), serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("Independent review PASS:1279commands/1188callers/76timing checks/18profiles/{total_samples}samples; see JSON for gate and failures; no adoption");
}

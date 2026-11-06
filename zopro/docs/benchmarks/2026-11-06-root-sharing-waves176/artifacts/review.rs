use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-waves176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const LABELS: [&str; 3] = ["control-a", "control-b", "shared"];
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
fn rank(v: &[f64]) -> Vec<f64> {
    let mut a = v.to_vec();
    a.sort_by(f64::total_cmp);
    a
}
fn stats(v: &[f64], s: &Value) {
    assert_eq!(v.len(), 15);
    let a = rank(v);
    same(a[3], num(&s["low"]));
    same(a[7], num(&s["median"]));
    same(a[11], num(&s["high"]));
    assert_eq!(s["wins"], v.iter().filter(|&&x| x < 1.).count());
    for (a, b) in v.iter().zip(s["ratios"].as_array().unwrap()) {
        same(*a, num(b));
    }
}
fn result(name: &str) -> Value {
    let s = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let rows = s
        .lines()
        .filter_map(|l| l.strip_prefix("ROOT_WAVE "))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 1);
    serde_json::from_str(rows[0]).unwrap()
}
fn main() {
    assert!(!r("review.json").exists());
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = read(r("plan.json"));
    let a = read(r("analysis.json"));
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let native = read(format!("{Q}/review.json"));
    assert_eq!(native["review_passed"], true);
    assert_eq!(
        hash(format!("{Q}/driver")),
        native["identities"]["driver"]["sha256"]
    );
    let fixture = read(format!("{Q}/oracle.json"))["fixture_hash"].clone();
    let mut calls = BTreeMap::new();
    let mut classes = BTreeMap::<String, usize>::new();
    let mut names = BTreeSet::new();
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        assert!(names.insert(name.to_string()));
        let b = r(format!("runs/{name}"));
        let e = read(b.join("exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let cmd = read(b.join("command.json"));
        let argv = cmd["argv"].as_array().unwrap();
        assert_eq!(
            &argv[..4],
            &[json!("timeout"), json!("-k"), json!("5s"), json!("600s")]
        );
        assert_eq!(&argv[4..], step["argv"].as_array().unwrap());
        let mut env = plan["environment"].as_object().unwrap().clone();
        if let Some(extra) = step["environment"].as_object() {
            env.extend(extra.clone());
        }
        assert_eq!(cmd["environment"], json!(env));
        for s in ["stdout", "stderr"] {
            assert_eq!(hash(b.join(format!("{s}.log"))), e[format!("{s}_sha256")]);
        }
        let class = if name == "preflight-check" {
            "check"
        } else {
            name.split('-').next().unwrap()
        };
        *classes.entry(class.to_string()).or_default() += 1;
        if ["preflight", "latency", "stat", "cycles"].contains(&class) {
            let v = result(name);
            assert_eq!(v, a["caller_receipts"][name]);
            let p = argv
                .iter()
                .position(|v| v == &json!(format!("{Q}/driver")))
                .unwrap();
            assert_eq!(argv[p - 3], "taskset");
            assert_eq!(argv[p - 2], "-c");
            let w = argv[p + 1].as_str().unwrap().parse::<usize>().unwrap();
            let n = argv[p + 2].as_str().unwrap().parse::<usize>().unwrap();
            let label = argv[p + 3].as_str().unwrap();
            assert!([1, 16, 32].contains(&w));
            assert!(LABELS.contains(&label));
            assert_eq!(
                argv[p - 1],
                if w == 1 {
                    "2"
                } else if w == 16 {
                    "0-15"
                } else {
                    "0-31"
                }
            );
            assert_eq!(v["workers"], w);
            assert_eq!(v["roots_per_wave"], w);
            assert_eq!(v["rounds"], n);
            assert_eq!(v["label"], label);
            assert_eq!(
                v["mode"],
                if label == "shared" {
                    "optimistic-shared"
                } else {
                    "independent"
                }
            );
            assert_eq!(v["fixture_sha256"], fixture);
            assert_eq!(v["fixture_rows"], 256);
            assert_eq!(v["fixture_bytes"], 262144);
            assert_eq!(v["timed_allocations"], 0);
            assert_eq!(v["priming_passes"], 3);
            assert_eq!(v["priming_rounds"], 256);
            assert_eq!(v["zero_request_publication_or_queue_cost"], true);
            assert_eq!(v["worker_start_end_barriers_inside_clock"], true);
            assert_eq!(v["proofs"], 0);
            assert_eq!(v["new_production_threads"], 0);
            let shared = label == "shared";
            let leader = if w == 1 { 0 } else { 2 };
            assert_eq!(v["coordinator"], leader);
            assert_eq!(v["active_workers"], if shared { 1 } else { w });
            assert_eq!(v["parked_workers"], if shared { w - 1 } else { 0 });
            let rows = v["worker_rows"].as_array().unwrap();
            assert_eq!(rows.len(), w);
            let mut tids = BTreeSet::new();
            for (i, row) in rows.iter().enumerate() {
                assert_eq!(row["worker"], i);
                assert_eq!(row["cpu"], if w == 1 { 2 } else { i });
                assert_eq!(row["active"], !shared || i == leader);
                assert!(tids.insert(row["tid"].as_u64().unwrap()));
            }
            calls.insert(name.to_string(), v);
        }
    }
    assert_eq!(names.len(), 187);
    assert_eq!(calls.len(), 156);
    let expected = json!({"preflight":9,"check":1,"latency":135,"stat":6,"cycles":6,"own":6,"decode":6,"metadata":6,"events":6,"loss":6});
    assert_eq!(json!(classes), expected);
    for w in [1, 16, 32] {
        let first = &calls[&format!("preflight-w{w}-control-a")];
        for label in LABELS {
            assert_eq!(calls[&format!("preflight-w{w}-{label}")]["rounds"], 64);
            assert_eq!(
                calls[&format!("preflight-w{w}-{label}")]["output_sha256"],
                first["output_sha256"]
            );
        }
    }
    let mut failures = BTreeSet::new();
    let mut vectors = BTreeMap::new();
    let mut checks = 0;
    let rows = a["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 3);
    let mut improvements = vec![];
    for w in [1, 16, 32] {
        let row = rows.iter().find(|v| v["workers"] == w).unwrap();
        let mut times = BTreeMap::new();
        for label in LABELS {
            let mut ns = vec![];
            let mut cpu = vec![];
            let mut busy = vec![];
            let mut faults = vec![];
            let mut rss = 0f64;
            for b in 0..15 {
                let v = &calls[&format!("latency-w{w}-b{b:02}-{label}")];
                assert_eq!(v["rounds"], 262144);
                for other in LABELS {
                    assert_eq!(
                        v["output_sha256"],
                        calls[&format!("latency-w{w}-b{b:02}-{other}")]["output_sha256"]
                    );
                }
                let elapsed = num(&v["elapsed_ns"]);
                let ms = num(&v["usage"]["user_cpu_ms"]) + num(&v["usage"]["system_cpu_ms"]);
                ns.push(elapsed / 262144.);
                cpu.push(ms / 262144.);
                busy.push(ms * 1e6 / elapsed);
                faults.push(num(&v["usage"]["minor_faults"]) / 262144.);
                rss = rss.max(num(&v["usage"]["whole_process_maxrss_kib"]) / 1024.);
            }
            same(rank(&ns)[7], num(&row["median_ns_per_wave"][label]));
            same(rank(&cpu)[7], num(&row["CPU_ms_per_wave"][label]));
            same(rank(&busy)[7], num(&row["median_busy_CPUs"][label]));
            same(
                rank(&faults)[7],
                num(&row["median_minor_faults_per_wave"][label]),
            );
            same(rss, num(&row["max_observed_process_RSS_MiB"][label]));
            times.insert(label, ns);
        }
        for (num_arm, den) in [
            ("control-b", "control-a"),
            ("shared", "control-a"),
            ("shared", "control-b"),
        ] {
            let v = (0..15)
                .map(|i| times[num_arm][i] / times[den][i])
                .collect::<Vec<_>>();
            stats(&v, &row["pairs"][format!("{num_arm}/{den}")]);
            let s = rank(&v);
            let calib = num_arm == "control-b";
            let pass = if calib {
                s[3] >= 0.97 && s[11] <= 1.03
            } else {
                s[11] <= 1.01
            };
            checks += 1;
            if !pass {
                failures.insert(format!(
                    "w{w}/{num_arm}/{den}/{}",
                    if calib {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if w > 1 && num_arm == "shared" {
                checks += 1;
                if s[11] >= 1. || v.iter().filter(|&&v| v < 1.).count() < 11 {
                    failures.insert(format!("w{w}/{num_arm}/{den}/gain"));
                }
            }
            vectors.insert(format!("w{w}/{num_arm}/{den}"), v);
        }
        improvements.push(json!({"workers":w,"shared_control_a_CPU_time_ratio":num(&row["CPU_ms_per_wave"]["shared"])/num(&row["CPU_ms_per_wave"]["control-a"]),"shared_control_a_wave_ns_ratio_of_unpaired_medians":num(&row["median_ns_per_wave"]["shared"])/num(&row["median_ns_per_wave"]["control-a"])}));
    }
    for den in ["control-a", "control-b"] {
        let v = (0..15)
            .map(|i| {
                ((vectors[&format!("w16/shared/{den}")][i].ln()
                    + vectors[&format!("w32/shared/{den}")][i].ln())
                    / 2.)
                    .exp()
            })
            .collect::<Vec<_>>();
        stats(&v, &a["threaded_aggregates"][den]);
        checks += 1;
        if rank(&v)[11] >= 1. || v.iter().filter(|&&v| v < 1.).count() < 11 {
            failures.insert(format!("threaded/shared/{den}/gain"));
        }
    }
    assert_eq!(checks, 15);
    assert_eq!(a["timing_checks"], 15);
    let reported = a["failed_checks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(failures, reported);
    assert_eq!(failures.len(), 6);
    assert_eq!(a["gate_passed"], false);
    let mut profile_rows = BTreeMap::new();
    let mut total = 0;
    let mut pmu = BTreeMap::new();
    let symbols = read(r("profile-symbols.json"));
    for w in [16, 32] {
        for label in LABELS {
            let count = if label == "shared" { 16777216 } else { 2097152 };
            let stat = format!("stat-w{w}-{label}");
            assert_eq!(calls[&stat]["rounds"], count / 4);
            let mut events = BTreeMap::new();
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
                let n = f[0].trim().parse::<f64>().unwrap();
                let coverage = f[4].trim().parse::<f64>().unwrap();
                assert!(coverage >= 90.);
                let event = f[2].trim();
                same(n, num(&a["counters"][&stat][event]["count"]));
                same(
                    coverage,
                    num(&a["counters"][&stat][event]["coverage_percent"]),
                );
                same(
                    n / (count / 4) as f64,
                    num(&a["counters"][&stat][event]["per_wave"]),
                );
                assert_eq!(a["counters"][&stat][event]["qualified"], true);
                events.insert(event.to_string(), n / (count / 4) as f64);
            }
            assert_eq!(events.len(), 4);
            pmu.insert(stat.clone(), json!(events));
            let name = format!("cycles-w{w}-{label}");
            let v = &calls[&name];
            assert_eq!(v["rounds"], count);
            if label == "control-b" {
                assert_eq!(
                    v["output_sha256"],
                    calls[&format!("cycles-w{w}-control-a")]["output_sha256"]
                );
                assert_eq!(
                    calls[&stat]["output_sha256"],
                    calls[&format!("stat-w{w}-control-a")]["output_sha256"]
                );
            }
            let pid = v["pid"].as_u64().unwrap();
            let worker = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    (
                        v["tid"].as_u64().unwrap(),
                        (v["cpu"].as_u64().unwrap(), v["active"].as_bool().unwrap()),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            let meta = fs::read_to_string(r(format!("runs/metadata-{name}/stdout.log"))).unwrap();
            let mut mapped = BTreeSet::new();
            let mut owned = BTreeSet::from([pid]);
            for l in meta.lines() {
                if l.contains("PERF_RECORD_MMAP") && l.contains(&format!("{Q}/driver")) {
                    mapped.insert(
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
                    let p = s
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if p[0] == pid {
                        owned.insert(p[1]);
                    }
                }
            }
            assert_eq!(mapped, BTreeSet::from([pid]));
            assert!(worker.keys().all(|tid| owned.contains(tid)));
            let attrs = fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();
            for token in ["cycles:u", "16000000", "CPU", "TIME", "PERIOD"] {
                assert!(attrs.contains(token));
            }
            assert!(!attrs.contains("freq: 1"));
            assert_eq!(
                read(r(format!("runs/loss-{name}/stdout.log")))["zero_lost"],
                true
            );
            let ownership = read(r(format!("runs/own-{name}/stdout.log")));
            assert_eq!(ownership["sha256_before"], ownership["sha256_after"]);
            assert_eq!(
                hash(r(format!("perf/{name}.data"))),
                ownership["sha256_before"]
            );
            let mut n = 0u64;
            let mut active = 0u64;
            let mut unknown = 0u64;
            let mut bins = BTreeMap::<String, u64>::new();
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
                assert!(cpu < w as u64);
                if let Some(&(expected, yes)) = worker.get(&tid) {
                    assert_eq!(cpu, expected);
                    if yes {
                        active += 1;
                    }
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
                n += 1;
            }
            assert!(
                n >= 1000 && unknown as f64 / n as f64 <= 0.05 && active as f64 / n as f64 >= 0.95
            );
            let p = &a["profiles"][&name];
            assert_eq!(p["samples"], n);
            assert_eq!(p["active_worker_samples"], active);
            assert_eq!(p["unknown_samples"], unknown);
            same(num(&p["active_worker_share"]), active as f64 / n as f64);
            same(num(&p["unknown_share"]), unknown as f64 / n as f64);
            assert_eq!(p["qualified"], true);
            assert_eq!(p["zero_lost"], true);
            assert_eq!(json!(bins), symbols[&name]);
            profile_rows.insert(name,json!({"samples":n,"active_worker_samples":active,"unknown_samples":unknown,"qualified":true}));
            total += n;
        }
    }
    assert_eq!(total, 49708);
    assert_eq!(a["profile_samples_total"], total);
    assert_eq!(a["new_proofs"], 0);
    assert_eq!(a["proof_ledger"], 30846);
    assert_eq!(a["adopted"], false);
    assert_eq!(a["broker_implemented"], false);
    let mut pmu_ratios = BTreeMap::new();
    for w in [16, 32] {
        let mut e = BTreeMap::new();
        for event in [
            "cycles:u",
            "instructions:u",
            "cache-misses:u",
            "branch-misses:u",
        ] {
            e.insert(event,json!({"shared_control_a":num(&pmu[&format!("stat-w{w}-shared")][event])/num(&pmu[&format!("stat-w{w}-control-a")][event]),"shared_control_b":num(&pmu[&format!("stat-w{w}-shared")][event])/num(&pmu[&format!("stat-w{w}-control-b")][event])}));
        }
        pmu_ratios.insert(w.to_string(), json!(e));
    }
    let out = json!({"review_passed":true,"pins_checked":pins.as_object().unwrap().len(),"commands_checked":names.len(),"drivers_checked":calls.len(),"classes":classes,"timing_checks":checks,"failed_timing_checks":failures,"profiles_checked":profile_rows,"profile_samples":total,"CPU_and_unpaired_wall_diagnostics":improvements,"PMU_ratios":pmu_ratios,"equal_round_hashes_compared":true,"unequal_duration_diagnostic_sums_checked_by_pinned_driver":true,"gate_passed":false,"adopted":false,"broker_implemented":false,"new_proofs":0,"proof_ledger":30846,"analysis_sha256":hash(r("analysis.json")),"reviewer_source_sha256":hash(r("review.rs")),"reviewer_binary_sha256":hash(r("review"))});
    fs::write(r("review.json"), serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("Independent review PASS187commands/156drivers/329pins/15timingchecks/6failures/6profiles/{total}samples; no broker/adoption/proof claim");
}

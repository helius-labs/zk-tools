use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/rerandomized-glv-callers";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn pin(pins: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn verify(pins: &BTreeMap<String, String>) {
    for (p, h) in pins {
        assert_eq!(&hash(p), h, "pin {p}");
    }
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native-v2/result.json"))["status"], "complete");
    assert_eq!(read(r("linked/result.json"))["status"], "complete");
    for name in ["checked", "optimized"] {
        assert!(
            fs::read_to_string(r(format!("native-v2/test-{name}/stdout.log")))
                .unwrap()
                .contains("test result: ok. 5 passed; 0 failed")
        );
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins-v3.json"))).unwrap();
    verify(&pins);
    for p in [
        "control",
        "control.rs",
        "repair",
        "repair.rs",
        "repair.json",
        "build-plan-v2.json",
        "build-pins-v2.json",
        "build-plan-v3.json",
        "build-pins-v3.json",
        "fixture-link.rs",
        "fixture-link",
        "fixture-repair.json",
        "fixture-v2/probe.rs",
        "fixture-v2/checked",
        "fixture-v2/optimized",
        "linked/result.json",
        "native-v2/result.json",
        "native-v2/test-checked/stdout.log",
        "native-v2/test-optimized/stdout.log",
        "probe",
        "checked",
        "optimized",
    ] {
        pin(&mut pins, r(p));
    }
    pin(&mut pins, X);
    for (name, argv) in [
        ("machine", vec!["lscpu"]),
        (
            "host-before",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
    ] {
        let out = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(out.status.success());
        fs::write(r(format!("{name}.txt")), out.stdout).unwrap();
        pin(&mut pins, r(format!("{name}.txt")));
    }
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut add = |name: String, phase: &str, b: usize, case: &str, mode: usize, rounds: usize| {
        let normal = vec![
            r("probe").display().to_string(),
            case.into(),
            mode.to_string(),
            rounds.to_string(),
        ];
        let mut env = json!({});
        let argv = if phase == "cycles" {
            let dir = r("profiles").join(&name);
            fs::create_dir_all(&dir).unwrap();
            for f in ["ctl", "ack"] {
                assert!(Command::new("mkfifo")
                    .arg(dir.join(f))
                    .status()
                    .unwrap()
                    .success());
            }
            env["ZOPRO_BENCH_PERF_CTL"] = json!(dir.join("ctl"));
            env["ZOPRO_BENCH_PERF_ACK"] = json!(dir.join("ack"));
            let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in env.as_object().unwrap() {
                a.push(format!("{k}={}", v.as_str().unwrap()));
            }
            a.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "-e".into(),
                "cycles:u".into(),
                "-F".into(),
                "199".into(),
                "-m".into(),
                "512".into(),
                "-o".into(),
                dir.join("perf.data").display().to_string(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    dir.join("ctl").display(),
                    dir.join("ack").display()
                ),
                "--".into(),
            ]);
            a.extend(normal);
            a
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":300}));
        jobs.push(
            json!({"name":name,"phase":phase,"block":b,"case":case,"mode":mode,"rounds":rounds}),
        );
    };
    for b in 0..31 {
        let mut contexts = vec!["g1", "g2", "w16", "w32"];
        contexts.rotate_left(b % 4);
        if b % 2 == 1 {
            contexts.reverse();
        }
        for (i, case) in contexts.iter().enumerate() {
            let modes = if (b + i) % 2 == 0 { [0, 1] } else { [1, 0] };
            for mode in modes {
                add(
                    format!("b{b}-{case}-m{mode}"),
                    "latency",
                    b,
                    case,
                    mode,
                    256,
                );
            }
        }
    }
    for case in ["g2", "w32"] {
        for mode in [0, 1] {
            let name = format!("cycles-{case}-m{mode}");
            add(name.clone(), "cycles", 0, case, mode, 16384);
            profiles.push(name);
        }
    }
    for name in profiles {
        let p = r("profiles").join(&name).join("perf.data");
        for (label, argv) in [
            (
                "owner",
                vec![
                    "sudo".into(),
                    "-n".into(),
                    "chown".into(),
                    "ubuntu:ubuntu".into(),
                    p.display().to_string(),
                ],
            ),
            (
                "decode",
                vec![
                    "perf".into(),
                    "script".into(),
                    "--header".into(),
                    "--show-lost-events".into(),
                    "-i".into(),
                    p.display().to_string(),
                    "-F".into(),
                    "pid,tid,period,ip,sym".into(),
                ],
            ),
            (
                "loss",
                vec![
                    "perf".into(),
                    "script".into(),
                    "-D".into(),
                    "-i".into(),
                    p.display().to_string(),
                ],
            ),
        ] {
            steps.push(json!({"kind":"command","name":format!("{label}-{name}"),"argv":argv,"environment":{},"timeout_seconds":120}));
        }
    }
    assert_eq!(jobs.len(), 252);
    assert_eq!(steps.len(), 264);
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared248latency/4cycles/12decode;allpinsverified");
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn bounds(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":v.iter().filter(|&&x|x<1.).count(),"ratios":v,"nominal_marginal_coverage":0.9705506265163422})
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins: BTreeMap<String, String> = serde_json::from_value(read(r("pins.json"))).unwrap();
    verify(&pins);
    let jobs = read(r("jobs.json"));
    let mut rows = BTreeMap::new();
    let mut timed = 0u64;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|s| serde_json::from_str::<Value>(s).ok())
            .collect();
        assert_eq!(lines.len(), 1);
        let v = &lines[0];
        for k in ["case", "mode", "rounds"] {
            assert_eq!(v[k], j[k]);
        }
        assert_eq!(v["all_outputs_verified"], true);
        assert_eq!(v["SDK_requests"], 0);
        timed += v["rounds"].as_u64().unwrap();
        rows.insert(name.to_string(), v.clone());
    }
    let mut cells = BTreeMap::new();
    let mut comparisons = BTreeMap::new();
    let mut failures = vec![];
    for case in ["g1", "g2", "w16", "w32"] {
        for mode in [0, 1] {
            let values: Vec<_> = (0..31)
                .map(|b| {
                    rows[&format!("b{b}-{case}-m{mode}")]["ns_per_call"]
                        .as_f64()
                        .unwrap()
                })
                .collect();
            let allocs: Vec<_> = (0..31)
                .map(|b| {
                    rows[&format!("b{b}-{case}-m{mode}")]["timed_allocation_calls"]
                        .as_f64()
                        .unwrap()
                        / 256.
                })
                .collect();
            let bytes: Vec<_> = (0..31)
                .map(|b| {
                    rows[&format!("b{b}-{case}-m{mode}")]["timed_requested_bytes"]
                        .as_f64()
                        .unwrap()
                        / 256.
                })
                .collect();
            cells.insert(format!("{case}/m{mode}"),json!({"median_ns":median(&values),"all_ns":values,"allocation_calls_per_round_median":median(&allocs),"requested_bytes_per_round_median":median(&bytes)}));
        }
        let ratio: Vec<_> = (0..31)
            .map(|b| {
                rows[&format!("b{b}-{case}-m1")]["ns_per_call"]
                    .as_f64()
                    .unwrap()
                    / rows[&format!("b{b}-{case}-m0")]["ns_per_call"]
                        .as_f64()
                        .unwrap()
            })
            .collect();
        let v = bounds(&ratio);
        let pass = if case == "w16" {
            v["high"].as_f64().unwrap() <= 1.02
        } else {
            v["high"].as_f64().unwrap() < 1. && v["wins"].as_u64().unwrap() >= 21
        };
        if !pass {
            failures.push(case);
        }
        comparisons.insert(case, v);
    }
    let mut profiles = BTreeMap::new();
    for case in ["g2", "w32"] {
        for mode in [0, 1] {
            let name = format!("cycles-{case}-m{mode}");
            let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
            assert!(!loss.contains("PERF_RECORD_LOST"));
            let decoded = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut pids = BTreeSet::new();
            let mut tids = BTreeSet::new();
            let mut n = 0;
            for line in decoded.lines().filter(|s| !s.trim_start().starts_with('#')) {
                let f: Vec<_> = line.split_whitespace().collect();
                if f.len() < 4 {
                    continue;
                }
                if let Some((p, t)) = f[0].split_once('/') {
                    if let (Ok(p), Ok(t), Ok(period)) =
                        (p.parse::<u64>(), t.parse::<u64>(), f[1].parse::<u64>())
                    {
                        pids.insert(p);
                        tids.insert(t);
                        n += 1;
                        let symbol = format!("{:#}", rustc_demangle::demangle(&f[3..].join(" ")));
                        *symbols.entry(symbol).or_default() += period;
                    }
                }
            }
            assert!(n > 0);
            assert_eq!(pids.len(), 1);
            profiles.insert(name,json!({"samples":n,"pids":pids,"tids":tids,"lost":0,"symbol_period":symbols,"raw_sha256":hash(r(format!("profiles/cycles-{case}-m{mode}/perf.data")))}));
        }
    }
    assert_eq!(timed, 129024);
    save(
        r("analysis.json"),
        &json!({"gate_passed":failures.is_empty(),"failed_checks":failures,"cells":cells,"pairs":comparisons,"profiles":profiles,"timed_assembly_calls":timed,"kernel_processes":252,"latency_processes":248,"all_outputs_verified":true,"capture_pins":pins.len(),"SDK_requests":0,"new_Gnark_exports":0,"proof_ledger":13082,"adopted":false,"production_changed":false,"goal_status":"active","CPU10x_rows":0}),
    );
    save(r("rows.json"), &json!(rows));
    println!(
        "Gate={} failures={failures:?};129024timedcallers;noSDK",
        failures.is_empty()
    );
    for (case, v) in comparisons {
        println!(
            "{case}: ratio={} bounds[{},{}] wins={}",
            v["median"], v["low"], v["high"], v["wins"]
        );
    }
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!("mode"),
    }
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut buf = [0; 65536];
    loop {
        let n = file.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(pins: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn env() -> Value {
    json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"})
}
fn repair_plan() {
    assert!(!r("repair-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for f in [
        "native-v2.rs",
        "native-helper-v2",
        "repair-freezer.rs",
        "repair-freezer",
        "freezer-repair.json",
        "native/failure.json",
    ] {
        pin(&mut pins, r(f));
    }
    save(r("repair-pins.json"), &json!(pins));
    let mut step = read(r("build-plan.json"))["steps"][2].clone();
    step["argv"] = json!([r("native-helper-v2"), "freeze"]);
    save(
        r("repair-plan.json"),
        &json!({"version":1,"source_pins":r("repair-pins.json"),"steps":[step]}),
    );
    println!("Prepared failed freezer step only; passed native124 and librarybuild retained");
}
fn link() {
    assert_eq!(read(r("native-repair/result.json"))["status"], "complete");
    assert_eq!(
        read(r("native/failure.json"))["completed"],
        json!(["core-tests", "libraries"])
    );
    let log = fs::read_to_string(r("native/core-tests/stdout.log")).unwrap();
    assert!(log.contains("test result: ok. 124 passed; 0 failed"));
    assert!(!r("link-plan.json").exists());
    let a = read(r("artifacts.json"));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("repair-pins.json"))).unwrap();
    for (p, h) in a["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for f in [
        "driver.rs",
        "driver",
        "artifacts.json",
        "native/failure.json",
        "native-repair/result.json",
        "freezer-repair.json",
        "repair-pins.json",
        "repair-plan.json",
        "native/core-tests/stdout.log",
        "build-plan.json",
        "build-pins.json",
    ] {
        pin(&mut pins, r(f));
    }
    let flags = a["rustflags"].as_str().unwrap();
    assert!(flags.contains("zopro_g1_weighted_anchor"));
    let mut argv = vec!["rustc".to_string()];
    argv.extend(flags.split_whitespace().map(str::to_owned));
    argv.extend([
        "--edition=2021".into(),
        "-O".into(),
        r("probe.rs").display().to_string(),
        "-o".into(),
        r("probe").display().to_string(),
        "-L".into(),
        format!("dependency={}/frozen", R),
    ]);
    for name in [
        "zopro_core",
        "zopro_bn254",
        "libc",
        "rayon",
        "sha2",
        "serde_json",
    ] {
        argv.extend([
            "--extern".into(),
            format!("{name}={}", a["libraries"][name].as_str().unwrap()),
        ]);
    }
    save(r("link-command.json"), &json!(argv));
    save(r("link-pins.json"), &json!(pins));
    save(
        r("link-plan.json"),
        &json!({"version":1,"source_pins":r("link-pins.json"),"steps":[{"kind":"command","name":"probe","argv":argv,"timeout_seconds":300}]}),
    );
    println!("124 native core tests passed; prepared single probe link from immutable common-image libraries");
}
fn prepare() {
    assert_eq!(read(r("linked/result.json"))["status"], "complete");
    assert!(!r("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("link-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for f in [
        "probe",
        "link-command.json",
        "link-plan.json",
        "link-pins.json",
        "linked/result.json",
    ] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, X);
    let rustc = Command::new("rustup")
        .args(["which", "rustc"])
        .output()
        .unwrap();
    assert!(rustc.status.success());
    pin(&mut pins, String::from_utf8(rustc.stdout).unwrap().trim());
    for (name, argv) in [
        ("compiler", vec!["rustc", "-vV"]),
        ("machine", vec!["lscpu"]),
        (
            "host",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
    ] {
        let out = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(out.status.success());
        let p = r(format!("{name}.txt"));
        fs::write(&p, out.stdout).unwrap();
        pin(&mut pins, p);
    }
    let nm = Command::new("nm")
        .args(["--defined-only"])
        .arg(r("probe"))
        .output()
        .unwrap();
    assert!(nm.status.success());
    fs::write(r("symbols.txt"), &nm.stdout).unwrap();
    pin(&mut pins, r("symbols.txt"));
    let text = String::from_utf8(nm.stdout).unwrap();
    let symbols: Vec<_> = text
        .lines()
        .filter_map(|l| {
            let s = l.split_whitespace().last()?;
            s.contains("stream_g1_anchor_kernel")
                .then_some(s.to_owned())
        })
        .collect();
    assert!(
        symbols.len() >= 4,
        "distinct owned/borrowed ordinary/weighted kernels required"
    );
    let mut dumps = vec![];
    for (i, s) in symbols.iter().enumerate() {
        let out = Command::new("objdump")
            .args(["-d", "--no-show-raw-insn"])
            .arg(format!("--disassemble={s}"))
            .arg(r("probe"))
            .output()
            .unwrap();
        assert!(out.status.success());
        assert!(out.stdout.len() > 1000);
        let p = r(format!("kernel-{i}.asm.txt"));
        fs::write(&p, out.stdout).unwrap();
        pin(&mut pins, &p);
        dumps.push(json!({"symbol":s,"path":p,"sha256":hash(&p)}));
    }
    save(
        r("codegen.json"),
        &json!({"probe_sha256":hash(r("probe")),"kernels":dumps,"read_only_objdump":true,"same_image_modes":["ordinary0","ordinary1","weighted2"],"independent_frozen_executable_control":false}),
    );
    pin(&mut pins, r("codegen.json"));
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   kind: &str,
                   size: usize,
                   mode: usize,
                   rounds: usize| {
        let normal = vec![
            "taskset".to_owned(),
            "-c".into(),
            "2".into(),
            r("probe").display().to_string(),
            kind.into(),
            size.to_string(),
            mode.to_string(),
            rounds.to_string(),
        ];
        let mut environment = env();
        let argv = if phase != "latency" {
            let dir = r("profiles").join(&name);
            fs::create_dir_all(&dir).unwrap();
            assert!(Command::new("mkfifo")
                .arg(dir.join("ctl"))
                .arg(dir.join("ack"))
                .status()
                .unwrap()
                .success());
            environment["ZOPRO_BENCH_PERF_CTL"] = json!(dir.join("ctl"));
            environment["ZOPRO_BENCH_PERF_ACK"] = json!(dir.join("ack"));
            let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in environment.as_object().unwrap() {
                a.push(format!("{k}={}", v.as_str().unwrap()));
            }
            a.push("perf".into());
            if phase == "stat" {
                a.extend([
                    "stat".into(),
                    "--no-big-num".into(),
                    "-x".into(),
                    ";".into(),
                    "-e".into(),
                    "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                ]);
            } else {
                a.extend([
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
                ]);
            }
            a.extend([
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
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":240}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"kind":kind,"size":size,"mode":mode,"rounds":rounds}));
    };
    for block in 0..31 {
        let mut contexts = vec![
            ("g1-owned", 200),
            ("g1-owned", 352),
            ("g1-borrowed-signed", 200),
            ("g1-borrowed-signed", 352),
            ("g2-prefix", 200),
            ("g2-prefix", 352),
        ];
        contexts.rotate_left(block % 6);
        if block % 2 != 0 {
            contexts.reverse();
        }
        for (i, (kind, size)) in contexts.into_iter().enumerate() {
            let mut modes = vec![0, 1, 2];
            modes.rotate_left((block + i) % 3);
            if block % 2 != 0 {
                modes.reverse();
            }
            for mode in modes {
                add(
                    format!("b{block}-{kind}-{size}-m{mode}"),
                    "latency",
                    block,
                    kind,
                    size,
                    mode,
                    4096,
                );
            }
        }
    }
    for phase in ["stat", "cycles"] {
        for (kind, size) in [("g1-owned", 200), ("g1-borrowed-signed", 352)] {
            for mode in [0, 2] {
                let name = format!("{phase}-{kind}-{size}-m{mode}");
                add(name.clone(), phase, 0, kind, size, mode, 262144);
                if phase == "cycles" {
                    profiles.push(name);
                }
            }
        }
    }
    assert_eq!(jobs.len(), 566);
    for name in profiles {
        let p = r("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 578);
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared558latency,4stat,4cycles,then12decode/owner/losssteps; one executable ordinary0/1 vs weighted2;{}kernelmonomorphs",symbols.len());
}
fn median(v: &[f64]) -> f64 {
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    let n = x.len();
    if n % 2 == 0 {
        (x[n / 2 - 1] + x[n / 2]) / 2.
    } else {
        x[n / 2]
    }
}
fn bounds(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    json!({"median":median(v),"low":x[9],"high":x[21],"ratios":v,"n":31,"wins":v.iter().filter(|x|**x<1.).count(),"nominal_marginal_coverage":0.9705506265163422})
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let jobs = read(r("jobs.json"));
    let mut results = BTreeMap::new();
    let mut stats = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut digests = BTreeMap::new();
    let mut capacities = BTreeMap::new();
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|l| l.strip_prefix("RESULT "))
            .collect();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(v["kind"], j["kind"]);
        assert_eq!(v["size"], j["size"]);
        assert_eq!(v["mode"], j["mode"]);
        assert_eq!(v["iterations"], j["rounds"]);
        assert_eq!(v["prime_calls"], 3);
        assert_eq!(v["prime_iterations"], 1024);
        assert_eq!(v["timed_allocations"], 0);
        assert_eq!(v["internally_verified"], true);
        let key = format!("{}-{}-{}", j["kind"], j["size"], j["rounds"]);
        let h = v["output_sha256"].as_str().unwrap();
        if let Some(old) = digests.insert(key, h.to_string()) {
            assert_eq!(old, h);
        }
        let key = format!("{}-{}", j["kind"], j["size"]);
        let cap = json!([v["capacity_bytes"], v["fixture_reference_bytes"]]);
        if let Some(old) = capacities.insert(key, cap.clone()) {
            assert_eq!(old, cap);
        }
        results.insert(name.to_owned(), v);
        if j["phase"] == "stat" {
            let text = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
            let mut events = BTreeMap::new();
            for line in text.lines() {
                let f: Vec<_> = line.trim().split(';').collect();
                if f.len() < 5 || !f[2].ends_with(":u") {
                    continue;
                }
                let count: f64 = f[0].trim().parse().expect("must have counted");
                let percent: f64 = f[4].trim().parse().unwrap();
                assert!(percent > 0.);
                events.insert(f[2].to_owned(),json!({"count":count,"running_percent":percent,"per_batch":count/j["rounds"].as_u64().unwrap()as f64}));
            }
            assert_eq!(events.len(), 4);
            for e in [
                "cycles:u",
                "instructions:u",
                "cache-misses:u",
                "branch-misses:u",
            ] {
                assert!(events.contains_key(e));
            }
            let ipc = events["instructions:u"]["count"].as_f64().unwrap()
                / events["cycles:u"]["count"].as_f64().unwrap();
            stats.insert(name.to_string(), json!({"events":events,"IPC":ipc}));
        }
        if j["phase"] == "cycles" {
            let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
            assert!(!loss.contains("PERF_RECORD_LOST"));
            let mut pids = BTreeSet::new();
            let mut symbols: BTreeMap<String, u64> = BTreeMap::new();
            let (mut samples, mut period) = (0u64, 0u64);
            for line in text.lines().filter(|l| !l.trim_start().starts_with('#')) {
                let f: Vec<_> = line.split_whitespace().collect();
                if f.len() < 4 {
                    continue;
                }
                let (p, _) = f[0].split_once('/').unwrap();
                let _: u32 = p.parse().unwrap();
                let weight: u64 = f[1].parse().unwrap();
                pids.insert(p.to_owned());
                samples += 1;
                period += weight;
                let symbol = format!("{:#}", rustc_demangle::demangle(&f[3..].join(" ")));
                *symbols.entry(symbol).or_default() += weight;
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            profiles.insert(name,json!({"samples":samples,"period":period,"pids":pids,"lost":0,"symbols":symbols,"raw_perf_sha256":hash(r("profiles").join(name).join("perf.data"))}));
        }
    }
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let (mut calibration, mut nonreg) = (true, true);
    let mut aggregates = BTreeMap::new();
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-prefix"] {
        for size in [200, 352] {
            for mode in 0..3 {
                let v: Vec<_> = (0..31)
                    .map(|b| {
                        results[&format!("b{b}-{kind}-{size}-m{mode}")]["ns_per_batch"]
                            .as_f64()
                            .unwrap()
                    })
                    .collect();
                cells.insert(
                    format!("{kind}/{size}/m{mode}"),
                    json!({"median_ns_per_batch":median(&v),"process_means":v}),
                );
            }
            for (top, base) in [(1, 0), (2, 0), (2, 1)] {
                let ratios: Vec<_> = (0..31)
                    .map(|b| {
                        results[&format!("b{b}-{kind}-{size}-m{top}")]["ns_per_batch"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{b}-{kind}-{size}-m{base}")]["ns_per_batch"]
                                .as_f64()
                                .unwrap()
                    })
                    .collect();
                let b = bounds(&ratios);
                if top == 1 {
                    calibration &= b["low"].as_f64().unwrap() >= 1. / 1.03
                        && b["high"].as_f64().unwrap() <= 1.03;
                } else {
                    nonreg &= b["high"].as_f64().unwrap() <= 1.02;
                }
                pairs.insert(format!("{kind}/{size}/{top}-over-{base}"), b);
            }
        }
    }
    let mut gain = true;
    for base in [0, 1] {
        let ratios: Vec<_> = (0..31)
            .map(|b| {
                let mut sum = 0.;
                for kind in ["g1-owned", "g1-borrowed-signed"] {
                    for size in [200, 352] {
                        sum += (results[&format!("b{b}-{kind}-{size}-m2")]["ns_per_batch"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{b}-{kind}-{size}-m{base}")]["ns_per_batch"]
                                .as_f64()
                                .unwrap())
                        .ln();
                    }
                }
                (sum / 4.).exp()
            })
            .collect();
        let v = bounds(&ratios);
        gain &= v["high"].as_f64().unwrap() < 1. && v["wins"].as_u64().unwrap() >= 21;
        aggregates.insert(format!("weighted-over-ordinary{base}"), v);
    }
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    save(
        r("analysis.json"),
        &json!({"scope":"same-executable complete point-addition caller screen only; no full proof or independent frozen-executable calibration","cells":cells,"pairs":pairs,"G1_aggregate":aggregates,"gates":{"ordinary_controls_calibrated":calibration,"all_six_contexts_nonregression":nonreg,"G1_aggregate_gain":gain,"component_gate_passed":calibration&&nonreg&&gain},"stat":stats,"profiles":profiles,"results":results,"capacity_bytes_and_fixture_references":capacities,"captures":566,"latency_processes":558,"native_core_tests":124,"final_pins_verified":pins.as_object().unwrap().len(),"production_changed":false,"adopted":false,"new_Gnark_proofs":0,"independent_proof_count":12983,"goal_active_original8_achieved":0}),
    );
    for (k, v) in cells {
        println!("{k}:{}ns", v["median_ns_per_batch"]);
    }
    for (k, v) in aggregates {
        println!("{k}:{v}");
    }
    println!(
        "calibration={calibration};nonreg={nonreg};gain={gain};eligibleforfullSDKscreen={}",
        calibration && nonreg && gain
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "repair-plan" => repair_plan(),
        "link" => link(),
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!("mode"),
    }
}

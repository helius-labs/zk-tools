use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/signed-difference175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const M:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
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
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
const PATTERNS: [&str; 4] = ["chain", "streams4", "scaled", "affine"];
const LABELS: [&str; 3] = ["frozen", "fresh", "candidate"];
fn invoke(label: &str) -> (String, &'static str) {
    if label == "frozen" {
        (r("frozen-probe").display().to_string(), "ordinary")
    } else {
        (
            r("target/release/probe").display().to_string(),
            if label == "candidate" {
                "candidate"
            } else {
                "ordinary"
            },
        )
    }
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":300})
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert!(fs::read_to_string(r("native-tests.log"))
        .unwrap()
        .contains("test result: ok. 2 passed; 0 failed"));
    let model = read(r("model.json"));
    assert_eq!(model["status"], "passed");
    assert_eq!(model["new_exact_REDC_cases"], 30084);
    let mut pins = BTreeMap::new();
    let manifest = read(M);
    pin(&mut pins, M);
    let mut fields = 0;
    for (f, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, &p);
        if let Some(tail) = f.strip_prefix("crates/zopro-bn254/") {
            let q = r("bn254").join(tail);
            let mut bytes = fs::read(&q).unwrap();
            if tail == "src/ifma8.rs" {
                let suffix =
                    b"\ninclude!(\"../../kernel.rs\");\ninclude!(\"../../test-field.rs\");\n";
                assert!(bytes.ends_with(suffix));
                bytes.truncate(bytes.len() - suffix.len());
            }
            assert_eq!(bytes, fs::read(p).unwrap());
            pin(&mut pins, q);
            fields += 1;
        }
    }
    assert_eq!(fields, 17);
    assert_eq!(
        manifest["source_files_sha256"].as_object().unwrap().len(),
        175
    );
    for f in ["setup.rs","setup","field-original-pins.json","bounds.md","design.md","kernel.rs","test-field.rs","src/bin/probe.rs","perf.rs","Cargo.toml","Cargo.lock","src/bin/model.rs","target/release/model","model.json","model.log","model.stderr.log","native-tests.log","frozen-build.log","native-build.log","frozen-probe","target/release/probe","control.rs","control","prepare-build.rs","build-helper","phase1-plan.json","phase2-plan.json","frozen-assembly.txt","candidate-assembly.txt"] { pin(&mut pins,r(f)); }
    pin(&mut pins, X);
    pin(&mut pins, "/usr/bin/perf");
    for entry in fs::read_dir(r("target/release/deps")).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().is_some_and(|v| v == "rlib") {
            pin(&mut pins, p);
        }
    }
    let mut steps = Vec::new();
    for pattern in PATTERNS {
        let (bin, mode) = invoke("candidate");
        steps.push(step(
            format!("pre-{pattern}"),
            ["taskset", "-c", "2", &bin, mode, pattern, "17"]
                .map(String::from)
                .into(),
        ));
    }
    for block in 0..31 {
        let mut cells: Vec<_> = PATTERNS
            .iter()
            .flat_map(|p| LABELS.iter().map(move |l| (*p, *l)))
            .collect();
        let length = cells.len();
        cells.rotate_left((block * 5) % length);
        if block % 2 == 1 {
            cells.reverse();
        }
        for (pattern, label) in cells {
            let (bin, mode) = invoke(label);
            steps.push(step(
                format!("lat-b{block:02}-{pattern}-{label}"),
                ["taskset", "-c", "2", &bin, mode, pattern, "100000"]
                    .map(String::from)
                    .into(),
            ));
        }
    }
    let mut profiles = Vec::new();
    for pattern in ["chain","affine"] {
        for label in ["fresh","candidate"] {
            let name=format!("{pattern}-{label}");let dir=r("profiles").join(&name);fs::create_dir_all(&dir).unwrap();
            let ctl=dir.join("ctl").display().to_string();let ack=dir.join("ack").display().to_string();
            assert!(std::process::Command::new("mkfifo").args([&ctl,&ack]).status().unwrap().success());
            let data=dir.join("perf.data").display().to_string();let(bin,mode)=invoke(label);
            let argv=vec!["sudo".into(),"-n".into(),"env".into(),format!("ZOPRO_BENCH_PERF_CTL={ctl}"),format!("ZOPRO_BENCH_PERF_ACK={ack}"),"perf".into(),"record".into(),"--sample-cpu".into(),"--timestamp".into(),"--no-buildid-cache".into(),"-e".into(),"cycles:u".into(),"-F".into(),"199".into(),"-o".into(),data.clone(),"--delay=-1".into(),format!("--control=fifo:{ctl},{ack}"),"--".into(),"taskset".into(),"-c".into(),"2".into(),bin,mode.into(),pattern.into(),"1000000".into()];
            steps.push(step(format!("profile-{name}"),argv));profiles.push((name,data));
        }
    }
    for (name, data) in profiles {
        steps.push(step(
            format!("ownership-{name}"),
            ["sudo", "-n", "chown", "ubuntu:ubuntu", &data]
                .map(String::from)
                .into(),
        ));
        steps.push(step(
            format!("decode-{name}"),
            [
                "perf",
                "script",
                "--header",
                "-i",
                &data,
                "-F",
                "pid,tid,period,ip,sym,dso",
            ]
            .map(String::from)
            .into(),
        ));
        steps.push(step(
            format!("loss-{name}"),
            ["perf", "script", "-D", "-i", &data]
                .map(String::from)
                .into(),
        ));
    }
    assert_eq!(steps.len(), 392);
    save(
        r("declaration.json"),
        &json!({"blocks":31,"patterns":PATTERNS,"labels":LABELS,"latency_processes":372,"preflight_processes":4,"profiles":4,"steps":392,"iterations":100000,"prime_calls":3,"prime_iterations":2000,"profile_iterations":1000000,"profile_scope":"FIFO-gated native arithmetic interval; setup/primes/scalar oracle excluded","intended_gain":"affine and4casegeometric aggregate upper<1 and>=21wins vs BOTH controls; all8caseupper<=1.03; allfresh/frozen cal within1.03; no minimumgainfloor","interval":"order ranks10..22of31 nominal97.055%conditionalexchangeability/no familywise claim","SDK_requests":0,"exports":0,"ledger":21259,"goal_achieved":false,"production_changed":false}),
    );
    pin(&mut pins, r("declaration.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!(
        "Prepared392steps/372latency/4preflight/4profiles;{}pins",
        pins.len()
    );
}
fn output(name: &str) -> Value {
    let text = fs::read_to_string(r("runs").join(name).join("stdout.log")).unwrap();
    let all: Vec<_> = text
        .lines()
        .filter_map(|l| l.strip_prefix("RESULT "))
        .collect();
    assert_eq!(all.len(), 1);
    let v: Value = serde_json::from_str(all[0]).unwrap();
    assert_eq!(v["internally_verified"], true);
    assert_eq!(v["checked_Fq2_elements"], 32);
    v
}
fn median(a: &[f64]) -> f64 {
    let mut a = a.to_vec();
    a.sort_by(f64::total_cmp);
    a[a.len() / 2]
}
fn stat(a: &[f64]) -> Value {
    assert_eq!(a.len(), 31);
    let mut s = a.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":a.iter().filter(|v|**v<1.).count(),"ratios":a})
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins = read(r("pins.json"));
    for (f, h) in pins.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut rows = Vec::new();
    let mut observations = Vec::new();
    let (mut calibration, mut nonregression, mut gain) = (Vec::new(), Vec::new(), Vec::new());
    let mut aggregate = [[0f64; 31]; 2];
    for pattern in PATTERNS {
        assert_eq!(output(&format!("pre-{pattern}"))["iterations"], 17);
        let mut times = BTreeMap::new();
        let mut expected = None;
        for label in LABELS {
            let mut t = Vec::new();
            for block in 0..31 {
                let name = format!("lat-b{block:02}-{pattern}-{label}");
                let v = output(&name);
                assert_eq!(v["iterations"], 100000);
                assert_eq!(v["prime_calls"], 3);
                assert_eq!(v["prime_iterations"], 2000);
                let h = v["output_sha256"].as_str().unwrap();
                if let Some(e) = &expected {
                    assert_eq!(e, h);
                } else {
                    expected = Some(h.to_owned());
                }
                t.push(v["ns_per_unit"].as_f64().unwrap());
                observations.push(
                    json!({"name":name,"block":block,"label":label,"pattern":pattern,"output":v}),
                );
            }
            times.insert(label, t);
        }
        let ratio = |a: &str, b: &str| {
            times[a]
                .iter()
                .zip(&times[b])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>()
        };
        let c = stat(&ratio("fresh", "frozen"));
        if c["low"].as_f64().unwrap() < 1. / 1.03 || c["high"].as_f64().unwrap() > 1.03 {
            calibration.push(pattern.to_string());
        }
        let mut paired = BTreeMap::new();
        for (k, label) in ["frozen", "fresh"].iter().enumerate() {
            let a = ratio("candidate", label);
            for block in 0..31 {
                aggregate[k][block] += a[block].ln() / 4.;
            }
            let s = stat(&a);
            if s["high"].as_f64().unwrap() > 1.03 {
                nonregression.push(format!("{pattern}/{label}"));
            }
            if pattern == "affine"
                && (s["high"].as_f64().unwrap() >= 1. || s["wins"].as_u64().unwrap() < 21)
            {
                gain.push(format!("affine/{label}"));
            }
            paired.insert(*label, s);
        }
        let med: BTreeMap<_, _> = times.iter().map(|(l, t)| (*l, median(t))).collect();
        rows.push(json!({"pattern":pattern,"median_ns":med,"paired":paired,"calibration":c,"raw_ns_per_element":times}));
    }
    let aggregates = aggregate.map(|a| stat(&a.map(f64::exp)));
    for (k, a) in aggregates.iter().enumerate() {
        if a["high"].as_f64().unwrap() >= 1. || a["wins"].as_u64().unwrap() < 21 {
            gain.push(format!("aggregate/{k}"));
        }
    }
    let passed = calibration.is_empty() && nonregression.is_empty() && gain.is_empty();
    let mut profiles = BTreeMap::new();
    let mut rawpins = BTreeMap::new();
    for pat in ["chain", "affine"] {
        for label in ["fresh", "candidate"] {
            let name = format!("{pat}-{label}");
            let output = output(&format!("profile-{name}"));
            assert_eq!(output["iterations"], 1000000);
            let text =
                fs::read_to_string(r("runs").join(format!("decode-{name}/stdout.log"))).unwrap();
            let loss = r("runs").join(format!("loss-{name}/stdout.log"));
            assert!(!fs::read_to_string(&loss)
                .unwrap()
                .contains("PERF_RECORD_LOST"));
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut pids = BTreeSet::new();
            let mut samples = 0;
            for line in text.lines().filter(|s| !s.starts_with('#')) {
                let w: Vec<_> = line.split_whitespace().collect();
                if w.len() < 5 {
                    continue;
                }
                let (pid, tidperiod, start) = if let Some((p, _)) = w[0].split_once('/') {
                    (p, w[1], 3)
                } else {
                    (w[0], w[2], 4)
                };
                if let (Ok(pid), Ok(period)) = (pid.parse::<u64>(), tidperiod.parse::<u64>()) {
                    pids.insert(pid);
                    samples += 1;
                    *symbols.entry(w[start..].join(" ")).or_default() += period;
                }
            }
            assert!(samples > 0);
            assert_eq!(pids.len(), 1);
            profiles.insert(name.clone(),json!({"samples":samples,"lost":0,"pids":pids,"symbols":symbols,"output":output,"scope":"FIFO-gated native arithmetic interval; scalar oracle excluded"}));
            pin(&mut rawpins, r("profiles").join(&name).join("perf.data"));
            pin(&mut rawpins, loss);
        }
    }
    save(r("observations.json"), &json!(observations));
    save(r("raw-profile-pins.json"), &json!(rawpins));
    save(
        r("analysis.json"),
        &json!({"status":"complete direct signed fused difference component","gate_passed":passed,"calibration_failures":calibration,"nonregression_failures":nonregression,"gain_failures":gain,"aggregates_control_order":["frozen","fresh"],"aggregates":aggregates,"rows":rows,"profiles":profiles,"model":read(r("model.json")),"SDK_requests":0,"exports":0,"ledger":21259,"production_changed":false,"adopted":false,"goal_achieved":false,"inherited_M5":read(M)["m5"]}),
    );
    println!("Closed372latency/4preflight/4profiles; componentgate{passed};ledger21259unchanged");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare|finish"),
    }
}

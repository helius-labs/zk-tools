use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-cancel175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";

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
fn pin(m: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    m.insert(p.display().to_string(), hash(p));
}
fn cases() -> Vec<(String, String, String, String)> {
    let mut v = Vec::new();
    for f in ["Fq", "Fr"] {
        for r in ["R256", "R260"] {
            for p in ["chain", "streams4", "fused"] {
                v.push((format!("{f}-{r}-{p}"), f.into(), r.into(), p.into()));
            }
        }
    }
    v
}
fn invoke(label: &str) -> (String, &'static str) {
    if label == "frozen" {
        (format!("{R}/frozen-probe"), "ordinary")
    } else {
        (
            format!("{R}/target/release/probe"),
            if label == "triple" {
                "triple"
            } else {
                "ordinary"
            },
        )
    }
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":180})
}
fn pins() {
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "pin {p}");
    }
}
fn prepare() {
    let r = Path::new(R);
    assert!(!r.join("run-plan.json").exists());
    assert!(fs::read_to_string(r.join("native-tests.log"))
        .unwrap()
        .contains("test result: ok. 2 passed; 0 failed"));
    assert_eq!(read(r.join("model.json"))["cases"], 60168);
    let mut p = BTreeMap::new();
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    let mut count = 0;
    let mut recovered = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let original = Path::new(Z).join(f);
        assert_eq!(hash(&original), h.as_str().unwrap());
        pin(&mut p, &original);
        count += 1;
        if let Some(tail) = f.strip_prefix("crates/zopro-bn254/") {
            let local = r.join("bn254").join(tail);
            let mut bytes = fs::read(&local).unwrap();
            if tail == "src/ifma8.rs" {
                let suffix =
                    b"\ninclude!(\"../../kernel.rs\");\ninclude!(\"../../test-field.rs\");\n";
                assert!(bytes.ends_with(suffix));
                bytes.truncate(bytes.len() - suffix.len());
            }
            assert_eq!(bytes, fs::read(&original).unwrap());
            pin(&mut p, local);
            recovered += 1;
        }
    }
    assert_eq!(count, 175);
    assert!(recovered > 10);
    for f in [
        "design.md",
        "bounds.md",
        "setup.rs",
        "setup",
        "setup.log",
        "prefix.rs",
        "test-field.rs",
        "make-control.rs",
        "clean-control.rs",
        "repair.rs",
        "prefix-initial.rs",
        "kernel-initial.rs",
        "native-tests-initial.log",
        "native-build-initial.log",
        "history-audit.md",
        "model-draft-prefix-only.rs",
        "field-original-pins.json",
        "model.rs",
        "model",
        "model.json",
        "model-build.log",
        "model.json",
        "kernel.rs",
        "src/bin/probe.rs",
        "control.rs",
        "control",
        "Cargo.toml",
        "Cargo.lock",
        "frozen-probe",
        "target/release/probe",
        "native-tests.log",
        "frozen-build.log",
        "native-build.log",
    ] {
        pin(&mut p, r.join(f));
    }
    pin(&mut p, X);
    for e in fs::read_dir(r.join("target/release/deps")).unwrap() {
        let e = e.unwrap();
        let name = e.file_name().to_string_lossy().to_string();
        if name.ends_with(".rlib") {
            pin(&mut p, e.path());
        }
    }
    save(r.join("pins.json"), &json!(p));
    let cs = cases();
    let mut steps = Vec::new();
    for (c, f, rad, pat) in &cs {
        let (bin, mode) = invoke("triple");
        steps.push(step(
            format!("preflight-{c}"),
            ["taskset", "-c", "2", &bin, mode, f, rad, pat, "17"]
                .map(String::from)
                .into(),
        ));
    }
    let labels = ["frozen", "fresh", "triple"];
    for block in 0..31 {
        let mut order: Vec<_> = cs
            .iter()
            .flat_map(|c| labels.iter().map(move |v| (c, *v)))
            .collect();
        let len = order.len();
        order.rotate_left((block * 11) % len);
        if block % 2 != 0 {
            order.reverse();
        }
        for ((c, f, rad, pat), label) in order {
            let (bin, mode) = invoke(label);
            steps.push(step(
                format!("lat-b{block:02}-{c}-{label}"),
                ["taskset", "-c", "2", &bin, mode, f, rad, pat, "100000"]
                    .map(String::from)
                    .into(),
            ));
        }
    }
    let mut profiles = Vec::new();
    for pat in ["chain", "fused"] {
        for label in ["fresh", "triple"] {
            let key = format!("{pat}-{label}");
            let dir = r.join("profiles").join(&key);
            fs::create_dir_all(&dir).unwrap();
            let file = dir.join("perf.data").display().to_string();
            let (bin, mode) = invoke(label);
            steps.push(step(
                format!("profile-{key}"),
                [
                    "sudo",
                    "-n",
                    "perf",
                    "record",
                    "--no-buildid-cache",
                    "-e",
                    "cycles:u",
                    "-F",
                    "199",
                    "-o",
                    &file,
                    "--",
                    "taskset",
                    "-c",
                    "2",
                    &bin,
                    mode,
                    "Fq",
                    "R256",
                    pat,
                    "30000000",
                ]
                .map(String::from)
                .into(),
            ));
            profiles.push((key, file));
        }
    }
    for (key, file) in profiles {
        steps.push(step(
            format!("ownership-{key}"),
            ["sudo", "-n", "chown", "ubuntu:ubuntu", &file]
                .map(String::from)
                .into(),
        ));
        steps.push(step(
            format!("decode-{key}"),
            [
                "perf",
                "script",
                "--header",
                "-i",
                &file,
                "-F",
                "pid,tid,period,ip,sym,dso",
            ]
            .map(String::from)
            .into(),
        ));
        steps.push(step(
            format!("loss-{key}"),
            ["perf", "script", "-D", "-i", &file]
                .map(String::from)
                .into(),
        ));
    }
    save(
        r.join("run-plan.json"),
        &json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps}),
    );
    save(
        r.join("declaration.json"),
        &json!({"status":"frozen before probe execution","blocks":31,"latency_processes":1116,"cases":cs,"variants":labels,"preflight_processes":12,"profile_processes":4,"latency_iterations":100000,"profile_iterations":30000000,"prime_calls":3,"prime_iterations":2000,"cpu":"2","native_flags":"-C target-cpu=native --cfg zopro_cios_cancel_carry --cfg triple_cancel_kernel","profiles":"whole process; all profiles after latency; all decoding after all captures","interval":"individual ranks10..22 of31;97.055% nominal conditional on exchangeability; no familywise claim","gate":"calibration factor1.03 eachcase; FqR2563case triple aggregate upper<1 and>=21wins vs BOTH controls; eachcase nonreg upper<=1.03; no exclusions/extensions/rerolls","source_recovery_files":recovered,"SDK_requests":0,"new_exports":0,"ledger":17811,"production_changed":false,"goal_achieved":false}),
    );
    println!("Prepared1116latency+12preflight+4profiles;{}pins", p.len());
}
fn result(name: &str) -> Value {
    let text = fs::read_to_string(format!("{R}/runs/{name}/stdout.log")).unwrap();
    let v: Vec<_> = text
        .lines()
        .filter_map(|s| s.strip_prefix("RESULT "))
        .collect();
    assert_eq!(v.len(), 1);
    let v: Value = serde_json::from_str(v[0]).unwrap();
    assert_eq!(v["internally_verified"], true);
    assert_eq!(v["checked_fields"], 32);
    v
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn stat(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":v.iter().filter(|v|**v<1.).count(),"all_ratios":v})
}
fn finish() {
    pins();
    let r = Path::new(R);
    let cs = cases();
    let mut rows = Vec::new();
    let mut failcal = Vec::new();
    let mut failnonreg = Vec::new();
    let mut agg: [[f64; 31]; 2] = [[0.; 31]; 2];
    let mut output = BTreeMap::new();
    for (c, _, _, _) in &cs {
        assert_eq!(result(&format!("preflight-{c}"))["iterations"], 17);
        let mut times = BTreeMap::<String, Vec<f64>>::new();
        let mut expected = None;
        for label in ["frozen", "fresh", "triple"] {
            let mut t = Vec::new();
            for b in 0..31 {
                let v = result(&format!("lat-b{b:02}-{c}-{label}"));
                assert_eq!(v["iterations"], 100000);
                let digest = v["output_sha256"].as_str().unwrap();
                if let Some(h) = &expected {
                    assert_eq!(h, digest)
                } else {
                    expected = Some(digest.to_owned())
                }
                t.push(v["ns_per_unit"].as_f64().unwrap());
            }
            times.insert(label.into(), t);
        }
        let ratios = |a: &str, b: &str| -> Vec<f64> {
            times[a].iter().zip(&times[b]).map(|(a, b)| a / b).collect()
        };
        let cal = stat(&ratios("fresh", "frozen"));
        if cal["low"].as_f64().unwrap() < 1. / 1.03 || cal["high"].as_f64().unwrap() > 1.03 {
            failcal.push(c.clone());
        }
        let mut triple = BTreeMap::new();
        for (k, label) in ["frozen", "fresh"].iter().enumerate() {
            let ratios = ratios("triple", label);
            if c.starts_with("Fq-R256-") {
                for b in 0..31 {
                    agg[k][b] += ratios[b].ln() / 3.;
                }
            }
            let s = stat(&ratios);
            if s["high"].as_f64().unwrap() > 1.03 {
                failnonreg.push(format!("{c}/{label}"));
            }
            triple.insert(*label, s);
        }
        let med: BTreeMap<_, _> = times.iter().map(|(k, v)| (k, median(v))).collect();
        rows.push(json!({"case":c,"median_ns":med,"calibration":cal,"triple_ratios":triple}));
        output.insert(c.clone(), expected.unwrap());
    }
    let aggregates = agg.map(|v| stat(&v.map(f64::exp)));
    let aggregate_pass = aggregates
        .iter()
        .all(|s| s["high"].as_f64().unwrap() < 1. && s["wins"].as_u64().unwrap() >= 21);
    let passed = failcal.is_empty() && failnonreg.is_empty() && aggregate_pass;
    let mut profiles = BTreeMap::new();
    let mut rawpins = BTreeMap::new();
    for pat in ["chain", "fused"] {
        for label in ["fresh", "triple"] {
            let key = format!("{pat}-{label}");
            let v = result(&format!("profile-{key}"));
            assert_eq!(v["iterations"], 30000000);
            let text = fs::read_to_string(
                r.join("runs")
                    .join(format!("decode-{key}"))
                    .join("stdout.log"),
            )
            .unwrap();
            let loss = r
                .join("runs")
                .join(format!("loss-{key}"))
                .join("stdout.log");
            assert!(!fs::read_to_string(&loss)
                .unwrap()
                .contains("PERF_RECORD_LOST"));
            let mut pids = BTreeSet::new();
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut samples = 0;
            let mut period = 0u64;
            for line in text.lines().filter(|s| !s.trim_start().starts_with('#')) {
                let w: Vec<_> = line.split_whitespace().collect();
                if w.len() < 5 {
                    continue;
                }
                let parsed = if let Some((pid, _tid)) = w[0].split_once('/') {
                    pid.parse::<u64>()
                        .ok()
                        .zip(w[1].parse::<u64>().ok())
                        .map(|(p, n)| (p, n, 3))
                } else if w.len() >= 6 {
                    w[0].parse::<u64>()
                        .ok()
                        .zip(w[2].parse::<u64>().ok())
                        .map(|(p, n)| (p, n, 4))
                } else {
                    None
                };
                if let Some((pid, n, start)) = parsed {
                    samples += 1;
                    period += n;
                    pids.insert(pid);
                    *symbols.entry(w[start..].join(" ")).or_default() += n;
                }
            }
            assert!(samples > 0, "profile {key}");
            assert_eq!(pids.len(), 1);
            profiles.insert(key.clone(),json!({"samples":samples,"period":period,"pids":pids,"lost":0,"symbols":symbols,"checked_output":v}));
            pin(
                &mut rawpins,
                r.join("profiles").join(&key).join("perf.data"),
            );
            pin(&mut rawpins, loss);
        }
    }
    let mut static_code = BTreeMap::new();
    for name in ["frozen", "candidate"] {
        let text = fs::read_to_string(r.join(format!("{name}-assembly.txt"))).unwrap();
        let mut records = Vec::new();
        let mut current = String::new();
        let mut lines = Vec::<String>::new();
        let mut emit = |name: &String, lines: &Vec<String>| {
            if ["probe::chain", "probe::streams", "probe::fused"]
                .iter()
                .any(|p| name.contains(p))
            {
                let ifmas = lines.iter().filter(|s| s.contains("vpmadd52")).count();
                let stack = lines.iter().filter(|s| s.contains("%rsp")).count();
                records.push(json!({"symbol":name,"instructions":lines.len(),"IFMA_sites":ifmas,"RSP_operand_lines":stack,"stack_subtractions":lines.iter().filter(|s|s.contains("sub")&&s.contains("%rsp")).collect::<Vec<_>>()}));
            }
        };
        for line in text.lines() {
            if line.ends_with(">:") {
                emit(&current, &lines);
                current = line.into();
                lines.clear();
            } else if line.contains(':') && !line.trim().is_empty() {
                lines.push(line.into());
            }
        }
        emit(&current, &lines);
        static_code.insert(name, records);
    }
    save(
        r.join("analysis.json"),
        &json!({"status":"complete native triple-cancellation component","gate_passed":passed,"calibration_failures":failcal,"nonregression_failures":failnonreg,"aggregate_control_order":["frozen","fresh"],"aggregates":aggregates,"rows":rows,"profiles":profiles,"static_code":static_code,"output_hashes":output,"raw_profile_pins":rawpins,"new_exact_model_cases":60168,"native_Arkworks_and_raw_lanes":345088,"new_native_tests":2,"latency_processes":1116,"preflight_processes":12,"profile_processes":4,"SDK_requests":0,"new_exports":0,"proof_ledger_unchanged":17811,"adopted":false,"goal_achieved":false}),
    );
    println!("Closedtriple:gate{passed};1116latency+12preflight+4profiles;ledger17811unchanged");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare/finish"),
    }
}

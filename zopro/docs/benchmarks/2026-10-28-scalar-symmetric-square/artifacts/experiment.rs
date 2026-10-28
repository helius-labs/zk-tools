use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scalar-symmetric-square";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const DEP: &str = "/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";
const CFG: &str = "zopro_scalar_symmetric_square";
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
fn pin(m: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    m.insert(p.display().to_string(), hash(p));
}
fn checked(args: &[String], label: &str) {
    let o = Command::new(&args[0]).args(&args[1..]).output().unwrap();
    fs::write(r(format!("{label}-stdout.log")), o.stdout).unwrap();
    fs::write(r(format!("{label}-stderr.log")), o.stderr).unwrap();
    assert!(o.status.success(), "{label}: {:?}", o.status.code());
}
fn manifest() -> Value {
    read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"))
}
fn setup() {
    assert!(!r("source").exists());
    let m = manifest();
    let mut pins = BTreeMap::new();
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 172);
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let q = r("source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        pin(&mut pins, p);
    }
    let p = r("source/crates/zopro-bn254/src/lib.rs");
    let orig = fs::read_to_string(&p).unwrap();
    let suffix = "\n#[cfg(any(test, zopro_scalar_symmetric_square))]\nmod scalar_square;\n";
    fs::write(&p, format!("{orig}{suffix}")).unwrap();
    fs::write(r("original-lib.rs"), &orig).unwrap();
    let p = r("source/crates/zopro-bn254/src/fp.rs");
    let orig = fs::read_to_string(&p).unwrap();
    let anchor = "    pub fn square(&self) -> Self {\n";
    let added="        #[cfg(all(zopro_scalar_symmetric_square, target_arch = \"x86_64\", target_feature = \"bmi2\", target_feature = \"adx\", not(no_asm)))]\n        if C::P == <crate::FrConfig as FpConfig>::P {\n            return Self(crate::scalar_square::square::<C>(self.0), PhantomData);\n        }\n";
    assert_eq!(orig.matches(anchor).count(), 1);
    let changed = orig.replace(anchor, &format!("{anchor}{added}"));
    assert_eq!(changed.replace(added, ""), orig);
    fs::write(&p, changed).unwrap();
    fs::write(r("original-fp.rs"), orig).unwrap();
    fs::copy(
        r("scalar_square.rs"),
        r("source/crates/zopro-bn254/src/scalar_square.rs"),
    )
    .unwrap();
    fs::copy("/home/ubuntu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ark-ff-0.5.0/LICENSE-MIT",r("source/crates/zopro-bn254/UPSTREAM-SQUARE-LICENSE-MIT")).unwrap();
    fs::copy(
        format!("{Z}/crates/zopro-bench/examples/support/perf.rs"),
        r("perf.rs"),
    )
    .unwrap();
    save(
        r("source-audit.json"),
        &json!({"production_originals_exact":172,"private_originals_exact":170,"modified_originals":["crates/zopro-bn254/src/lib.rs","crates/zopro-bn254/src/fp.rs"],"lib_suffix":suffix,"square_insert":added,"original_fallback_exact":true,"new_module":"crates/zopro-bn254/src/scalar_square.rs","upstream_source_sha256":hash("/home/ubuntu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ark-ff-0.5.0/src/fields/models/fp/montgomery_backend.rs")}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    fs::write(r("native-flags.txt"), flags).unwrap();
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        pin(&mut pins, r("source").join(f));
    }
    for f in [
        "design.md",
        "scalar_square.rs",
        "experiment.rs",
        "experiment",
        "probe.rs",
        "perf.rs",
        "original-lib.rs",
        "original-fp.rs",
        "source-audit.json",
        "native-flags.txt",
        "source/crates/zopro-bn254/src/scalar_square.rs",
        "source/crates/zopro-bn254/UPSTREAM-SQUARE-LICENSE-MIT",
    ] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, X);
    pin(
        &mut pins,
        Command::new("rustup")
            .args(["which", "rustc"])
            .output()
            .map(|o| String::from_utf8(o.stdout).unwrap().trim().to_string())
            .unwrap(),
    );
    save(r("build-pins.json"), &json!(pins));
    let mut steps = vec![];
    for label in ["fresh", "candidate"] {
        let full = if label == "candidate" {
            format!("{flags} --cfg {CFG}")
        } else {
            flags.into()
        };
        let env = json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16"});
        steps.push(json!({"kind":"command","name":format!("{label}-tests"),"argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":["scalar_square::tests::full_512_bit_square_matches_independent_product","scalar_square::tests::montgomery_square_and_fifth_match_independent_fields"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("{label}-library"),"argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bn254","--lib","--message-format=json"],"environment":env,"timeout_seconds":600}));
        steps.push(json!({"kind":"command","name":format!("{label}-freeze"),"argv":[r("experiment"),PathBuf::from("freeze"),PathBuf::from(label)],"timeout_seconds":120}));
    }
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    for (name, args) in [
        ("compiler", vec!["rustc", "-vV"]),
        ("machine", vec!["lscpu"]),
        (
            "host-workload",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
    ] {
        checked(
            &args.into_iter().map(str::to_owned).collect::<Vec<_>>(),
            name,
        );
    }
    println!(
        "Prepared172-source private experiment;170 originals exact; fixed design before builds"
    );
}
fn freeze(label: &str) {
    assert!(["fresh", "candidate"].contains(&label));
    let text = fs::read_to_string(r(format!("native/{label}-library/stdout.log"))).unwrap();
    let artifacts: Vec<Value> = text
        .lines()
        .filter_map(|s| serde_json::from_str::<Value>(s).ok())
        .filter(|v| v["reason"] == "compiler-artifact" && v["target"]["name"] == "zopro_bn254")
        .collect();
    assert_eq!(artifacts.len(), 1);
    let metas: Vec<&str> = artifacts[0]["filenames"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .filter(|s| s.ends_with(".rmeta") && s.contains("/deps/libzopro_bn254-"))
        .collect();
    assert_eq!(metas.len(), 1);
    let lib = Path::new(metas[0]).with_extension("rlib");
    let stem = lib
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("libzopro_bn254-")
        .unwrap();
    let fingerprint = read(r(format!(
        "target-{label}/release/.fingerprint/zopro-bn254-{stem}/lib-zopro_bn254.json"
    )));
    let flags = fs::read_to_string(r("native-flags.txt")).unwrap();
    let mut expected: Vec<String> = flags.split_whitespace().map(str::to_owned).collect();
    if label == "candidate" {
        expected.extend(["--cfg".into(), CFG.into()]);
    }
    assert_eq!(fingerprint["rustflags"], json!(expected));
    let digest = hash(&lib);
    let dest = r(format!("frozen/{label}/libzopro_bn254-{digest}.rlib"));
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    assert!(!dest.exists());
    fs::copy(&lib, &dest).unwrap();
    assert_eq!(hash(&dest), digest);
    save(
        r(format!("{label}-library.json")),
        &json!({"library":dest,"sha256":digest,"hashed_cargo_library":lib,"fingerprint":fingerprint,"top_level_mutable_library_used":false}),
    );
    println!("Froze {label} immediately from hashed Cargo artifact {digest}");
}
fn link() {
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    assert!(!r("probe-artifacts.json").exists());
    let flags = fs::read_to_string(r("native-flags.txt")).unwrap();
    let accepted =
        read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json");
    let mut receipt = BTreeMap::new();
    for label in ["accepted", "fresh", "candidate"] {
        let lib = if label == "accepted" {
            let p = accepted["libraries"]["zopro_bn254"].as_str().unwrap();
            assert_eq!(hash(p), accepted["input_pins"][p].as_str().unwrap());
            let d = r("frozen/accepted/libzopro_bn254.rlib");
            fs::create_dir_all(d.parent().unwrap()).unwrap();
            fs::copy(p, &d).unwrap();
            d
        } else {
            let v = read(r(format!("{label}-library.json")));
            let p = v["library"].as_str().unwrap();
            assert_eq!(hash(p), v["sha256"].as_str().unwrap());
            PathBuf::from(p)
        };
        let mut args = vec!["rustc".to_owned()];
        args.extend(flags.split_whitespace().map(str::to_owned));
        args.extend([
            "--edition=2021".into(),
            "-O".into(),
            r("probe.rs").display().to_string(),
            "-o".into(),
            r(format!("{label}-probe")).display().to_string(),
            "-L".into(),
            format!("dependency={DEP}"),
            "-L".into(),
            format!("dependency={}", lib.parent().unwrap().display()),
            "--extern".into(),
            format!("zopro_bn254={}", lib.display()),
            "--extern".into(),
            format!("serde_json={DEP}/libserde_json-9c41682dbe4e057f.rlib"),
            "--extern".into(),
            format!("sha2={DEP}/libsha2-b36128d04ec91f93.rlib"),
            "--extern".into(),
            format!("ark_bn254={DEP}/libark_bn254-d9ceb28748a3bfd8.rlib"),
            "--extern".into(),
            format!("ark_ff={DEP}/libark_ff-9216c709978bd3d2.rlib"),
            "--extern".into(),
            format!("libc={}", accepted["libraries"]["libc"].as_str().unwrap()),
        ]);
        save(r(format!("link-{label}.json")), &json!(args));
        checked(&args, &format!("link-{label}"));
        receipt.insert(label,json!({"binary":r(format!("{label}-probe")),"sha256":hash(r(format!("{label}-probe"))),"library":lib,"library_sha256":hash(&lib)}));
    }
    assert_ne!(
        receipt["fresh"]["library_sha256"],
        receipt["candidate"]["library_sha256"]
    );
    assert_ne!(receipt["fresh"]["sha256"], receipt["candidate"]["sha256"]);
    save(
        r("probe-artifacts.json"),
        &json!({"variants":receipt,"two_OFF_binaries_identical":receipt["fresh"]["sha256"]==receipt["accepted"]["sha256"]}),
    );
    println!("Linked3frozen variants; ON and OFF library/binary identity checks passed");
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for label in ["fresh", "candidate"] {
        let p = r(format!("native/{label}-tests/stdout.log"));
        let t = fs::read_to_string(&p).unwrap();
        assert!(t.contains("test result: ok. 35 passed; 0 failed"));
        pin(&mut pins, p);
        pin(&mut pins, r(format!("{label}-library.json")));
    }
    let receipt = read(r("probe-artifacts.json"));
    for v in receipt["variants"].as_object().unwrap().values() {
        for (k, h) in [("binary", "sha256"), ("library", "library_sha256")] {
            let p = v[k].as_str().unwrap();
            assert_eq!(hash(p), v[h].as_str().unwrap());
            pin(&mut pins, p);
        }
    }
    for p in [
        "fixture.json",
        "probe-artifacts.json",
        "native/result.json",
        "build-plan.json",
        "build-pins.json",
        "compiler-stdout.log",
        "machine-stdout.log",
        "host-workload-stdout.log",
    ] {
        pin(&mut pins, r(p));
    }
    pin(&mut pins, X);
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   field: &str,
                   kind: &str,
                   label: &str,
                   rounds: usize| {
        let normal = vec![
            "taskset".to_owned(),
            "-c".into(),
            "2".into(),
            r(format!("{label}-probe")).display().to_string(),
            "run".into(),
            r("fixture.json").display().to_string(),
            field.into(),
            kind.into(),
            rounds.to_string(),
        ];
        let mut env = json!({"MALLOC_ARENA_MAX":"1"});
        let args = if phase == "profile" {
            let dir = r("profiles").join(&name);
            fs::create_dir_all(&dir).unwrap();
            assert!(Command::new("mkfifo")
                .arg(dir.join("ctl"))
                .arg(dir.join("ack"))
                .status()
                .unwrap()
                .success());
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
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    dir.join("ctl").display(),
                    dir.join("ack").display()
                ),
                "-o".into(),
                dir.join("perf.data").display().to_string(),
                "--".into(),
            ]);
            a.extend(normal);
            a
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":args,"environment":env,"timeout_seconds":240}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"field":field,"kind":kind,"variant":label,"rounds":rounds}));
    };
    for block in 0..31 {
        let mut contexts = vec![
            ("fq", "dependent"),
            ("fq", "four"),
            ("fq", "fifth"),
            ("fr", "dependent"),
            ("fr", "four"),
            ("fr", "fifth"),
        ];
        contexts.rotate_left(block % 6);
        if block % 2 != 0 {
            contexts.reverse();
        }
        for (i, (field, kind)) in contexts.into_iter().enumerate() {
            let mut labels = vec!["accepted", "fresh", "candidate"];
            labels.rotate_left((block + i) % 3);
            if (block + i) % 2 != 0 {
                labels.reverse();
            }
            for label in labels {
                add(
                    format!("b{block:02}-{field}-{kind}-{label}"),
                    "latency",
                    block,
                    field,
                    kind,
                    label,
                    262144,
                );
            }
        }
    }
    for kind in ["dependent", "fifth"] {
        for label in ["fresh", "candidate"] {
            let name = format!("profile-fr-{kind}-{label}");
            add(name.clone(), "profile", 0, "fr", kind, label, 4194304);
            profiles.push(name);
        }
    }
    assert_eq!(jobs.len(), 562);
    for name in profiles {
        let data = r("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",data],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",data,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",data],"timeout_seconds":120}));
    }
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared558latency then4profiles then12decode/owner steps");
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn interval(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    let mut c = 1u64;
    let mut tail = 1u64;
    for k in 1..=9 {
        c = c * (32 - k) / k;
        tail += c;
    }
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":v.iter().filter(|&&x|x<1.).count(),"ratios":v,"nominal_coverage":1.-2.*tail as f64/(1u64<<31)as f64})
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let jobs = read(r("jobs.json"));
    let mut results = BTreeMap::new();
    let mut contexts = vec![];
    let mut profiles = vec![];
    let mut calibration = true;
    let mut nonregression = true;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|s| s.strip_prefix("RESULT "))
            .collect();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(v["timed_allocations"], 0);
        assert_eq!(v["verified_against_Arkworks"], true);
        results.insert(name.to_owned(), v.clone());
        if j["phase"] == "profile" {
            let decode = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
            assert!(!loss.contains("PERF_RECORD_LOST"));
            let mut pids = BTreeSet::new();
            let mut symbols: BTreeMap<String, u64> = BTreeMap::new();
            let mut samples = 0usize;
            let mut period = 0u64;
            for line in decode.lines().filter(|l| !l.starts_with('#')) {
                let words: Vec<_> = line.split_whitespace().collect();
                if words.len() < 5 {
                    continue;
                }
                if words[0].parse::<u32>().is_ok() {
                    let weight: u64 = words[2].parse().unwrap();
                    pids.insert(words[0].to_owned());
                    samples += 1;
                    period += weight;
                    *symbols.entry(words[4..].join(" ")).or_default() += weight;
                }
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            let mut sym: Vec<_> = symbols.into_iter().collect();
            sym.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            profiles.push(json!({"name":name,"samples":samples,"pids":pids,"lost_records":0,"total_period":period,"top_symbols":sym.into_iter().take(15).collect::<Vec<_>>(),"raw_perf_data_sha256":hash(r("profiles").join(name).join("perf.data"))}));
        }
    }
    for field in ["fq", "fr"] {
        for kind in ["dependent", "four", "fifth"] {
            let mut medians = BTreeMap::new();
            for label in ["accepted", "fresh", "candidate"] {
                let v: Vec<_> = (0..31)
                    .map(|b| {
                        results[&format!("b{b:02}-{field}-{kind}-{label}")]["mean_ns_per_update"]
                            .as_f64()
                            .unwrap()
                    })
                    .collect();
                medians.insert(label, median(&v));
            }
            let mut ratios = BTreeMap::new();
            for (num, den) in [
                ("fresh", "accepted"),
                ("candidate", "fresh"),
                ("candidate", "accepted"),
            ] {
                let v: Vec<_> = (0..31)
                    .map(|b| {
                        results[&format!("b{b:02}-{field}-{kind}-{num}")]["mean_ns_per_update"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{b:02}-{field}-{kind}-{den}")]
                                ["mean_ns_per_update"]
                                .as_f64()
                                .unwrap()
                    })
                    .collect();
                let i = interval(&v);
                if num == "fresh" {
                    calibration &= i["low"].as_f64().unwrap() >= 1. / 1.03
                        && i["high"].as_f64().unwrap() <= 1.03;
                } else {
                    nonregression &= i["high"].as_f64().unwrap() <= 1.02;
                }
                ratios.insert(format!("{num}/{den}"), i);
            }
            contexts.push(json!({"field":field,"kind":kind,"median_of_process_mean_ns_per_update":medians,"paired":ratios}));
        }
    }
    let mut aggregate = BTreeMap::new();
    let mut gain = true;
    for den in ["accepted", "fresh"] {
        let v: Vec<_> = (0..31)
            .map(|b| {
                let sum: f64 = ["dependent", "four", "fifth"]
                    .iter()
                    .map(|kind| {
                        (results[&format!("b{b:02}-fr-{kind}-candidate")]["mean_ns_per_update"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{b:02}-fr-{kind}-{den}")]["mean_ns_per_update"]
                                .as_f64()
                                .unwrap())
                        .ln()
                    })
                    .sum();
                (sum / 3.).exp()
            })
            .collect();
        let i = interval(&v);
        gain &= i["high"].as_f64().unwrap() < 1. && i["wins"].as_u64().unwrap() >= 21;
        aggregate.insert(format!("candidate/{den}"), i);
    }
    let pins: BTreeMap<String, String> = serde_json::from_value(read(r("pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for (f, h) in manifest()["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
    }
    save(
        r("analysis.json"),
        &json!({"contexts":contexts,"Fr_aggregate":aggregate,"calibration_passed":calibration,"all_context_nonregression_passed":nonregression,"Fr_aggregate_gain_passed":gain,"component_gate_passed":calibration&&nonregression&&gain,"adopted":false,"latency_processes":558,"profiles":profiles,"results":results,"pins_verified":pins.len(),"production172_unchanged":true,"new_proofs":0,"proof_ledger_unchanged":12884,"goal_active_unmet_original8":true}),
    );
    println!(
        "Component gate {}; calibration {}; nonregression {}; Fr gain {}",
        calibration && nonregression && gain,
        calibration,
        nonregression,
        gain
    );
    for c in contexts {
        println!(
            "{} {} {}",
            c["field"], c["kind"], c["median_of_process_mean_ns_per_update"]
        );
    }
    for (k, v) in aggregate {
        println!("{k} {v}");
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    match args[1].as_str() {
        "setup" => setup(),
        "freeze" => freeze(&args[2]),
        "link" => link(),
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!("unknown action"),
    }
}

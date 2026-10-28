use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-screen";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn root(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
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
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn setup() {
    assert!(!root("build-plan.json").exists());
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let mut pins = BTreeMap::new();
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
    }
    let deps = "/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";
    let mut libraries = BTreeMap::new();
    for (name, file) in [
        ("num_bigint", "libnum_bigint-158c502562b8bd18.rlib"),
        ("serde_json", "libserde_json-9c41682dbe4e057f.rlib"),
        ("ark_bn254", "libark_bn254-d9ceb28748a3bfd8.rlib"),
        ("ark_ff", "libark_ff-9216c709978bd3d2.rlib"),
        ("ark_ec", "libark_ec-e4a9bfa1662bf7f1.rlib"),
    ] {
        let p = format!("{deps}/{file}");
        pins.insert(p.clone(), hash(&p));
        libraries.insert(name, p);
    }
    for entry in fs::read_dir(deps).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file() && p.extension().is_some_and(|x| x == "rlib" || x == "so") {
            pins.insert(p.display().to_string(), hash(p));
        }
    }
    for p in ["design.md", "model.rs", "study.rs", "study"] {
        pins.insert(root(p).display().to_string(), hash(root(p)));
    }
    for (name, args) in [
        ("compiler", vec!["rustc", "-vV"]),
        ("machine", vec!["lscpu"]),
    ] {
        let out = Command::new(args[0]).args(&args[1..]).output().unwrap();
        assert!(out.status.success());
        fs::write(root(format!("{name}.txt")), out.stdout).unwrap();
        pins.insert(
            root(format!("{name}.txt")).display().to_string(),
            hash(root(format!("{name}.txt"))),
        );
    }
    let mut steps = vec![];
    for name in ["model-tests", "model"] {
        let mut args = vec![
            "rustc".into(),
            "--edition=2021".into(),
            "-O".into(),
            "-C".into(),
            "overflow-checks=yes".into(),
            "-C".into(),
            "debug-assertions=yes".into(),
            root("model.rs").display().to_string(),
            "-L".into(),
            format!("dependency={deps}"),
            "-o".into(),
            root(name).display().to_string(),
        ];
        if name == "model-tests" {
            args.push("--test".into());
        }
        for (name, path) in &libraries {
            args.extend(["--extern".into(), format!("{name}={path}")]);
        }
        steps.push(json!({"kind":"command","name":format!("build-{name}"),"argv":args,"environment":{},"timeout_seconds":300}));
        if name == "model-tests" {
            steps.push(json!({"kind":"command","name":"tests","argv":[root(name),PathBuf::from("--test-threads=1"),PathBuf::from("--nocapture")],"environment":{},"required_tests":["tests::exact_bounds_and_scaled_coordinate_codec","tests::toy_curve_golden_addition_and_early_guard","tests::small_CIOS_matches_complete_REDC"],"unfiltered":true,"timeout_seconds":300}));
        }
    }
    save(root("build-pins.json"), &json!(pins));
    save(
        root("build-plan.json"),
        &json!({"version":1,"source_pins":root("build-pins.json"),"steps":steps}),
    );
    save(root("libraries.json"), &json!(libraries));
    println!("Preparedexactmodel3tests+modelbuild;172productionpinschecked;nonative/prooftiming");
}
fn prepare() {
    assert_eq!(read(root("native/result.json"))["status"], "complete");
    assert!(fs::read_to_string(root("native/tests/stdout.log"))
        .unwrap()
        .contains("test result: ok. 3 passed; 0 failed"));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for p in [
        "model",
        "model-tests",
        "native/result.json",
        "native/tests/stdout.log",
        "build-plan.json",
        "build-pins.json",
        "libraries.json",
    ] {
        pins.insert(root(p).display().to_string(), hash(root(p)));
    }
    save(root("pins.json"), &json!(pins));
    save(
        root("run-plan.json"),
        &json!({"version":1,"source_pins":root("pins.json"),"steps":[{"kind":"command","name":"exact-model","argv":[root("model")],"environment":{},"timeout_seconds":600}]}),
    );
    println!("Preparedfixedexactstudy;noCPUlatency/PMU/proofs");
}
fn finish() {
    assert_eq!(read(root("runs/result.json"))["status"], "complete");
    let pins = read(root("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let a = read(root("analysis.json"));
    assert_eq!(a["status"], "exact model passed");
    assert_eq!(a["codec_alias_checks"], 20810);
    assert_eq!(a["BN254_G2"]["ordinary_outputs"], 2560);
    save(
        root("closeout.json"),
        &json!({"all_pins_verified":pins.as_object().unwrap().len(),"three_new_tests_passed":true,"exact_model_passed":true,"native_speedup_proven":false,"proof_ledger":13181,"production172_unchanged":true,"goal_status":"active","CPU10x_rows":0}),
    );
    println!("Exactmodelpassed/pinsverified;nativeprototypeonlyafterworkledger;goalactive0/8");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "setup" => setup(),
        "prepare" => prepare(),
        "finish" => finish(),
        _ => panic!("mode"),
    }
}

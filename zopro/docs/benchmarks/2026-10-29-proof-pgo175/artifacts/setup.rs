const TRIPLE: &str = "x86_64-unknown-linux-gnu";
const LLVM:&str="/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata";
fn environment(m: &Value, label: &str) -> Value {
    let mut env = m["environment"].clone();
    env["RUSTFLAGS"] = json!(fs::read_to_string(r(format!("{label}-flags.txt"))).unwrap());
    env["CARGO_TARGET_DIR"] = json!(r(format!("target-{label}")));
    env["CARGO_BUILD_JOBS"] = json!("16");
    env["RAYON_NUM_THREADS"] = json!("4");
    env["LLVM_PROFILE_FILE"] = json!(r("build-raw/%p-%m.profraw"));
    env
}
fn steps(m: &Value, label: &str, tests: bool) -> Vec<Value> {
    let env = environment(m, label);
    let cargo = r("source/Cargo.toml");
    let mut steps = vec![];
    if tests {
        for (name, package) in [("field", "zopro-bn254"), ("core", "zopro-core")] {
            steps.push(json!({"kind":"command","name":format!("{label}-{name}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","--target",TRIPLE,"-p",package,"--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","--target",TRIPLE,"-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
    }
    steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","--target",TRIPLE,"-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
    let mut flags: Vec<String> = ["-O", "-C", "lto=fat", "-C", "codegen-units=1"]
        .iter()
        .map(|x| (*x).into())
        .collect();
    flags.extend(
        fs::read_to_string(r(format!("{label}-flags.txt")))
            .unwrap()
            .split_whitespace()
            .map(String::from),
    );
    let driver = if label == "generate" {
        r("train-driver.rs")
    } else {
        Path::new(Z).join("crates/zopro-bench/examples/cpu-sdk.rs")
    };
    steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":driver,"additional_sources":if label=="generate"{vec![]}else{vec![Path::new(Z).join("crates/zopro-bench/examples/support/perf.rs")]},"rustc_flags":flags,"timeout_seconds":900}));
    steps
}
fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 175);
    assert_eq!(m["full_performance_gate_passed"], false);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    pin(&mut pins, LLVM);
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        pin(&mut pins, to);
    }
    fs::create_dir(r("build-raw")).unwrap();
    fs::create_dir(r("raw")).unwrap();
    let plain = m["rustflags"].as_str().unwrap();
    let generate = format!(
        "{plain} -C profile-generate={} -C llvm-args=-instrprof-atomic-counter-update-all",
        r("raw").display()
    );
    let candidate = format!(
        "{plain} -C profile-use={} -C llvm-args=-pgo-warn-missing-function",
        r("merged.profdata").display()
    );
    for (label, flags) in [
        ("control", plain),
        ("generate", generate.as_str()),
        ("candidate", candidate.as_str()),
    ] {
        fs::write(r(format!("{label}-flags.txt")), flags).unwrap();
        pin(&mut pins, r(format!("{label}-flags.txt")));
    }
    save(
        r("source-audit.json"),
        &json!({"source_files":175,"originals_exact":175,"all_field_sources_exact":true,"production_changed":false,"compiler_only_candidate":true,"no_PGO_runtime_buffers_in_USE":true,"training_driver_outside_prover_snapshot":true}),
    );
    let mut plan = steps(&m, "control", true);
    plan.extend(steps(&m, "generate", false));
    assert_eq!(plan.len(), 9);
    for f in [
        "bootstrap.rs",
        "setup.rs",
        "native.rs",
        "native-helper",
        "design.md",
        "history-audit.md",
        "train-driver.rs",
        "source-audit.json",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        pin(&mut pins, Path::new(Z).join(f));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":plan}),
    );
    println!(
        "Prepared9phaseAsteps;175source175exact;plain33field124core8prover+GEN;no training yet"
    );
}
fn prepare_use() {
    assert!(!r("use-plan.json").exists());
    let a = read(r("training-audit.json"));
    assert_eq!(a["passed"], true);
    assert_eq!(a["proof_ledger"], 18769);
    assert_eq!(a["SDK_prove_count"], 99);
    assert_eq!(a["SDK_load_count"], 0);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    pin(&mut pins, r("merged.profdata"));
    pin(&mut pins, r("training-audit.json"));
    pin(&mut pins, r("training/result.json"));
    for (p, h) in a["pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    let m = read(Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    let plan = steps(&m, "candidate", true);
    assert_eq!(plan.len(), 6);
    save(r("use-pins.json"), &json!(pins));
    save(
        r("use-plan.json"),
        &json!({"version":1,"source_pins":r("use-pins.json"),"steps":plan}),
    );
    println!("Prepared6USEsteps pinned to99proof-only SDK profile and175source;33field124core8prover thenactualSDKlink");
}

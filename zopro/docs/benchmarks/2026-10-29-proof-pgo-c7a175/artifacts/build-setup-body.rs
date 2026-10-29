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
        fs::copy(from, &to).unwrap();
        pin(&mut pins, to);
    }
    let profile = Path::new("/home/ubuntu/src/zopro-perf-work/proof-pgo175/merged.profdata");
    assert_eq!(
        hash(profile),
        "0f9dbf4b871db206b18bacdd343e6fbb4ea92bbeae1942404fdc7d83839a8d08"
    );
    pin(&mut pins, profile);
    let training = "/home/ubuntu/src/zopro-perf-work/proof-pgo175/training-audit.json";
    let a = read(training);
    assert_eq!(a["passed"], true);
    assert_eq!(a["SDK_prove_count"], 99);
    assert_eq!(a["SDK_load_count"], 0);
    pin(&mut pins, training);
    let plain = m["rustflags"]
        .as_str()
        .unwrap()
        .replace("target-cpu=native", "target-cpu=znver4");
    assert_ne!(plain, m["rustflags"].as_str().unwrap());
    let candidate = format!(
        "{plain} -C profile-use={} -C llvm-args=-pgo-warn-missing-function",
        profile.display()
    );
    for (label, flags) in [("control", &plain), ("candidate", &candidate)] {
        fs::write(r(format!("{label}-flags.txt")), flags).unwrap();
        pin(&mut pins, r(format!("{label}-flags.txt")));
    }
    fs::create_dir(r("build-raw")).unwrap();
    for f in [
        "design.md",
        "native.rs",
        "native-helper",
        "build-setup.rs",
        "build-setup-body.rs",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        pin(&mut pins, Path::new(Z).join(f));
    }
    save(
        r("source-audit.json"),
        &json!({"exact_source_files":175,"source_changed":false,"production_changed":false,"parent_profile_architecture":"Ryzen9950X_native","consumer_target_cpu":"znver4","profile_retrained":false,"plain_rustflags":plain,"PGO_rustflags":candidate,"local_calibration_failure_retained":true}),
    );
    pin(&mut pins, r("source-audit.json"));
    save(r("build-pins.json"), &json!(pins));
    let control = steps(&m, "control", true);
    let candidate = steps(&m, "candidate", true);
    assert_eq!(control.len(), 6);
    assert_eq!(candidate.len(), 6);
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":control}),
    );
    save(
        r("use-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":candidate}),
    );
    println!("Prepared175exactsource znver4 plain/PGO;fixed existing proofprofile;6+6native steps;no cloud resources");
}
fn prepare_use() {
    panic!("Both plans frozen together by setup");
}

fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    assert_eq!(m["full_performance_gate_passed"], false);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 175);
    for (file, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(file);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
    }
    let old = Path::new("/home/ubuntu/src/zopro-perf-work/inverse-direct-finish");
    let file = "crates/zopro-bn254/src/inverse.rs";
    assert_eq!(
        fs::read(old.join("original-inverse.rs")).unwrap(),
        fs::read(r("source").join(file)).unwrap()
    );
    let changed = fs::read_to_string(old.join("source").join(file)).unwrap();
    let before = "    let (f0, g0) = (f1, g1);";
    let after="    let (f0, g0) = (f1, g1);\n    #[cfg(zopro_direct_finish_reduction)]\n    if (C::P[3] as u128)*3 > 1u128<<63 && C::P[3] < 1u64<<63 {\n        return Fp::from_montgomery(direct_finish_reduce::<C>(&u,&v,f0,g0));\n    }";
    let suffix="\n#[cfg(any(test,zopro_direct_finish_reduction))]\ninclude!(\"inverse/direct_finish.rs\");\n";
    assert_eq!(changed.matches(after).count(), 1);
    let back = changed.strip_suffix(suffix).unwrap().replace(after, before);
    assert_eq!(back.as_bytes(), fs::read(r("source").join(file)).unwrap());
    fs::write(r("source").join(file), &changed).unwrap();
    let helper = "crates/zopro-bn254/src/inverse/direct_finish.rs";
    fs::create_dir_all(r("source").join(helper).parent().unwrap()).unwrap();
    for target in [r("direct_finish.rs"), r("source").join(helper)] {
        fs::copy(old.join("direct_finish.rs"), target).unwrap();
    }
    fs::copy(old.join("bounds.md"), r("bounds.md")).unwrap();
    let mut exact = 0;
    for (file, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = r("source").join(file);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 174);
    pin(&mut pins, r("source").join(helper));
    for f in [
        "original-inverse.rs",
        "direct_finish.rs",
        "bounds.md",
        "control-identity-audit.json",
        "analysis.json",
    ] {
        pin(&mut pins, old.join(f));
    }
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"original_files":175,"originals_exact":174,"changed_file":file,"new_helper":helper,"before":before,"after":after,"suffix":suffix,"reverse175_exact":true,"exact_old_original_inverse":true,"exact_old_candidate_inverse":true,"exact_old_direct_helper":true,"core_solver_key_scheduler_forecast_exact":true,"heap_tables_key_copies_added":0,"production_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_direct_finish_reduction"));
    let candidate = format!("{flags} --cfg zopro_direct_finish_reduction");
    fs::write(r("native-flags.txt"), flags).unwrap();
    fs::write(r("candidate-flags.txt"), &candidate).unwrap();
    let mut steps = vec![];
    for label in ["control", "candidate"] {
        let mut env = m["environment"].clone();
        env["RUSTFLAGS"] = json!(if label == "candidate" {
            candidate.as_str()
        } else {
            flags
        });
        env["CARGO_TARGET_DIR"] = json!(r(format!("target-{label}")));
        env["CARGO_BUILD_JOBS"] = json!("16");
        env["RAYON_NUM_THREADS"] = json!("4");
        let cargo = r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("{label}-field"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["inverse::direct_finish_tests::direct_signed_finish_matches_independent_fullwidth_fields"],"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));
    assert_eq!(steps.len(), 13);
    for f in [
        "design.md",
        "history-audit.md",
        "bounds.md",
        "direct_finish.rs",
        "bootstrap.rs",
        "setup.rs",
        "code-audit.rs",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "native-flags.txt",
        "candidate-flags.txt",
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
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared 13 native steps; 176 private files, 174 exact originals / 175 reversible; separate immutable OFF/ON artifacts");
}

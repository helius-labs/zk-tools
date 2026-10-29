fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["full_performance_gate_passed"], false);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    for (p, h) in read(m["native_build_source_pins"].as_str().unwrap())
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    pin(&mut pins, m["native_build_source_pins"].as_str().unwrap());
    let original = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(original.len(), 175);
    for (file, h) in original {
        let p = Path::new(Z).join(file);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, &p);
        let to = r("source").join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(p, to).unwrap();
    }
    assert!(std::process::Command::new(r("patch-helper"))
        .status()
        .unwrap()
        .success());
    fs::copy(
        r("service_trace.rs"),
        r("source/crates/zopro-core/src/service_trace.rs"),
    )
    .unwrap();
    let changes = read(r("insertions.json"));
    let mut changed = vec![];
    let mut field = 0;
    for (file, h) in original {
        let p = r("source").join(file);
        if hash(&p) != h.as_str().unwrap() {
            changed.push(file);
            let mut s = fs::read_to_string(&p).unwrap();
            let short = file.strip_prefix("crates/zopro-core/src/").unwrap();
            for c in changes
                .as_array()
                .unwrap()
                .iter()
                .rev()
                .filter(|c| c["file"] == short)
            {
                if let Some(old) = c["old"].as_str() {
                    let new = c["new"].as_str().unwrap();
                    assert!(s.contains(new));
                    s = s.replacen(new, old, 1);
                } else {
                    let t = c["inserted"].as_str().unwrap();
                    assert!(s.contains(t));
                    s = s.replacen(t, "", 1);
                }
            }
            assert_eq!(s, fs::read_to_string(Path::new(Z).join(file)).unwrap());
        }
        if file.starts_with("crates/zopro-bn254/") {
            assert_eq!(hash(&p), h.as_str().unwrap());
            field += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(changed.len(), 8);
    assert_eq!(field, 17);
    pin(
        &mut pins,
        r("source/crates/zopro-core/src/service_trace.rs"),
    );
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"original_files":175,"originals_exact":167,"reverse175_exact":true,"changed":changed,"field_source_files_exact":17,"native_field_tests_inherited":33,"arithmetic_and_scheduler_unchanged":true,"explicit_cross_task_parent_ids":true,"serial_nonforking_only":true,"new_module":"service_trace.rs","production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    let flags = format!("{flags} --cfg zopro_service_trace");
    fs::write(r("candidate-flags.txt"), &flags).unwrap();
    let mut env = m["environment"].clone();
    env["RUSTFLAGS"] = json!(flags);
    env["CARGO_TARGET_DIR"] = json!(r("target-candidate"));
    env["CARGO_BUILD_JOBS"] = json!("16");
    env["RAYON_NUM_THREADS"] = json!("4");
    let cargo = r("source/Cargo.toml");
    let steps = json!([
        {"kind":"command","name":"candidate-core","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["service_trace::tests::explicit_parent_cpu_disabled_and_unwind","service_trace::tests::bounded_capacity_preserves_execution_and_reports_drops","service_trace::fold_tests::folds_count_items_and_restore_parent_before_snapshot","service_trace::fold_tests::map_init_preserves_indexed_producer_bounds_and_fixed_splits"],"unfiltered":true,"timeout_seconds":1800},
        {"kind":"command","name":"candidate-prover","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800},
        {"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800},
        {"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180},
        {"kind":"link-sdk","name":"candidate","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":r("driver.rs"),"additional_sources":[r("support/perf.rs")],"timeout_seconds":600},
        {"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}
    ]);
    for f in [
        "design.md",
        "bootstrap.rs",
        "setup.rs",
        "patch.rs",
        "patch-helper",
        "service_trace.rs",
        "driver.rs",
        "support/perf.rs",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "insertions.json",
        "candidate-flags.txt",
        "fold-additions.rs.txt",
        "rayon-reference/map_with.rs",
        "rayon-reference/for_each.rs",
        "rayon-reference/plumbing/mod.rs",
    ] {
        pin(&mut pins, r(f));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared6steps/176source167originalexact/reverse175;128core8prover;17fieldexact33inherited;1024eventsperthread");
}

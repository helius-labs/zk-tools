fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 175);
    assert_eq!(m["full_performance_gate_passed"], false);
    let mut field = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
        if p.starts_with("crates/zopro-bn254/") {
            field += 1;
        }
    }
    assert_eq!(field, 17);
    let file = "crates/zopro-core/src/msm_affine/grouping.rs";
    let original = fs::read_to_string(r("source").join(file)).unwrap();
    let before = "    let reservation = grouping_cost::reserve(scalars.len(), percent)?;";
    let after=format!("{before}\n    #[cfg(zopro_radix_groups)]\n    if radix::fits(scalars.len(), percent) {{\n        return radix::build(scalars, percent, reservation);\n    }}");
    let suffix = "\n#[cfg(any(test,zopro_radix_groups))]\nmod radix;\n";
    assert_eq!(original.matches(before).count(), 1);
    let changed = format!("{}{suffix}", original.replacen(before, &after, 1));
    assert_eq!(
        changed
            .strip_suffix(suffix)
            .unwrap()
            .replacen(&after, before, 1),
        original
    );
    fs::write(r("source").join(file), changed).unwrap();
    let helper = "crates/zopro-core/src/msm_affine/grouping/radix.rs";
    fs::copy(r("radix.rs"), r("source").join(helper)).unwrap();
    let mut exact = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let path = r("source").join(p);
        if hash(&path) == h.as_str().unwrap() {
            exact += 1;
        }
        pin(&mut pins, path);
    }
    assert_eq!(exact, 174);
    pin(&mut pins, r("source").join(helper));
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"original_files":175,"originals_exact":174,"changed_file":file,"new_helper":helper,"before":before,"after":after,"suffix":suffix,"reverse175_exact":true,"all17field_source_files_exact":true,"field33_tests_inherited":true,"sample_cutoffs_reservation_canonicalization_unchanged":true,"sorted_fingerprints_not_equality":true,"stable_first_input_and_membership_order_preserved":true,"new_key_tables_point_copies":0,"production_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_radix_groups"));
    let candidate = format!("{flags} --cfg zopro_radix_groups");
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
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["msm_affine::grouping::radix::tests::compiled_policy_dispatch_matches_cfg","msm_affine::grouping::radix::tests::exact_groups_match_ordered_oracle_with_adversarial_collisions","msm_affine::grouping::radix::tests::radix_is_stable_and_conserves_indices","msm_affine::grouping::radix::tests::checked_workspace_fits_existing_reservation"],"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));
    assert_eq!(steps.len(), 11);
    for f in [
        "design.md",
        "memory-review.md",
        "history-audit.md",
        "radix.rs",
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
    println!("Prepared11native steps;176source174originals exact/reverse175;17field files unchanged;128core8prover each mode");
}

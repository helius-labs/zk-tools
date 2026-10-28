fn setup() {
    assert!(!r("build-plan.json").exists());
    let m = read(MANIFEST);
    let mut pins = BTreeMap::new();
    let files = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 174);
    for (f, h) in files {
        let from = Path::new(Z).join(f);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let to = r("source").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        pin(&mut pins, from);
        pin(&mut pins, to);
    }
    let module = r("source/crates/zopro-core/src/msm/reduction_probe.rs");
    fs::copy(r("reduction_probe.rs"), &module).unwrap();
    pin(&mut pins, &module);
    let suffixes = [
        (
            "crates/zopro-core/src/msm.rs",
            "\n#[cfg(zopro_unweighted_probe)]\npub mod reduction_probe;\n",
        ),
        (
            "crates/zopro-core/src/lib.rs",
            "\n#[cfg(zopro_unweighted_probe)]\npub use msm::reduction_probe::ReductionCaller;\n",
        ),
    ];
    for (f, suffix) in suffixes {
        let p = r("source").join(f);
        let original = fs::read_to_string(&p).unwrap();
        fs::write(&p, format!("{original}{suffix}")).unwrap();
        pin(&mut pins, &p);
        assert_eq!(
            format!("{:x}", Sha256::digest(original.as_bytes())),
            files[f]
        );
    }
    let exact = files
        .iter()
        .filter(|(f, h)| hash(r("source").join(f)) == h.as_str().unwrap())
        .count();
    assert_eq!(exact, 172);
    save(
        r("source-audit.json"),
        &json!({"original_files":174,"candidate_files":175,"originals_exact":172,
        "reverse174_exact":true,"suffixes":suffixes,"new_module":"crates/zopro-core/src/msm/reduction_probe.rs",
        "production_changed":false,"field_key_solver_weighted_streaming_forecast_source_exact":true}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    fs::write(r("native-flags.txt"), flags).unwrap();
    let env = json!({"RUSTFLAGS":format!("{flags} --cfg zopro_unweighted_probe"),"CARGO_TARGET_DIR":r("target-candidate"),
        "CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"1","ZOPRO_CPU_DEDUP":"1","ZOPRO_CPU_ZERO_AWARE_GROUPS":"1",
        "ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1",
        "ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","ZOPRO_CPU_TREE_VECTOR_INVERSE":"1"});
    let cargo = r("source/Cargo.toml");
    let steps = json!([
        {"kind":"command","name":"core","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["msm::reduction_probe::tests::complete_reduction_matches_projective_for_tails_and_exceptions","msm::reduction_probe::tests::guards_leave_inputs_and_counts_unchanged_before_binary_fallback"],"timeout_seconds":1800},
        {"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800},
        {"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180},
        {"kind":"link-sdk","name":"probe","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":r("probe.rs"),"timeout_seconds":600},
        {"kind":"command","name":"oracle","argv":[r("native-helper"),"oracle"],"required_tests":["tests::complete_reductions_match_independent_arkworks"],"unfiltered":true,"timeout_seconds":600}
    ]);
    for f in [
        "native.rs",
        "native-helper",
        "setup.rs",
        "reduction_probe.rs",
        "probe.rs",
        "design.md",
        "native-flags.txt",
        "source-audit.json",
    ] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, MANIFEST);
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared source175;172exact/reverse174;4 native steps");
}

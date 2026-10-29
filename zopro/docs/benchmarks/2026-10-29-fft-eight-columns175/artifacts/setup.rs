fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["proof_ledger"], 17005);
    assert_eq!(m["full_performance_gate_passed"], false);
    assert_eq!(
        m["performance_failures_retained"],
        json!(["1x2-w16/OFF/accepted"])
    );
    let prior_pins = read(m["native_build_source_pins"].as_str().unwrap());
    for (p, h) in prior_pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "current production input {p}");
        pin(&mut pins, p);
    }
    pin(&mut pins, m["native_build_source_pins"].as_str().unwrap());
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 175);
    for (file, h) in originals {
        let from = Path::new(Z).join(file);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
    }
    let parent = "crates/zopro-core/src/fft/pair/leaf.rs";
    let child = "crates/zopro-core/src/fft/pair/leaf/eight.rs";
    let old = "    let mut gap=values.len()/2;";
    let new=format!("    #[cfg(zopro_fft_eight_columns)] if values.len()>=8 && roots.single.is_none() {{return eight::dif_serial(values,roots,level);}}\n{old}");
    let suffix = "\n#[cfg(zopro_fft_eight_columns)] mod eight;\n";
    let path = r("source").join(parent);
    let mut text = fs::read_to_string(&path).unwrap();
    once(&mut text, old, &new);
    text.push_str(suffix);
    fs::write(&path, &text).unwrap();
    let mut back = text.strip_suffix(suffix).unwrap().to_string();
    once(&mut back, &new, old);
    assert_eq!(back, fs::read_to_string(Path::new(Z).join(parent)).unwrap());
    let to = r("source").join(child);
    fs::create_dir_all(to.parent().unwrap()).unwrap();
    fs::copy(r("eight.rs"), &to).unwrap();
    assert_eq!(hash(&to), hash(r("eight.rs")));
    pin(&mut pins, &to);
    let mut exact = 0;
    let mut field_files = 0;
    for (file, h) in originals {
        let p = r("source").join(file);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        }
        if file.starts_with("crates/zopro-bn254/") {
            assert_eq!(hash(&p), h.as_str().unwrap());
            field_files += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 174);
    assert_eq!(field_files, 17);
    let accepted_balanced = "crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs";
    assert_eq!(
        hash(r("source").join(accepted_balanced)),
        originals[accepted_balanced].as_str().unwrap()
    );
    assert_eq!(m["environment"]["ZOPRO_CPU_DEFER_SIGN_WIDTH"], "12");
    assert_eq!(m["environment"]["ZOPRO_CPU_DEFER_SIGN_PARTS"], "1");
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"original_files":175,"originals_exact":exact,"reverse175_exact":true,"parent":parent,"child":child,"old":old,"new":new,"suffix":suffix,"accepted_balanced13_byte_exact":true,"point_formulas_unchanged":true,"no_new_heap_capacity":true,"fixed_tile_stack_extra":true,"field_source_files_exact":field_files,"field_tests_inherited":33,"new_field_tests":0,"existing_forecast_conservative":true,"production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_fft_eight_columns"));
    fs::write(r("native-flags.txt"), flags).unwrap();
    let candidate = format!("{flags} --cfg zopro_fft_eight_columns");
    fs::write(r("candidate-flags.txt"), &candidate).unwrap();
    let mut steps = vec![
        json!({"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180}),
    ];
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
        let mut core = json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800});
        if label == "candidate" {
            core["required_tests"]=json!(["fft::pair::leaf::eight::tests::tile_basis_boundaries_and_transpose_match_scalar_dft","fft::pair::leaf::eight::tests::selected_full_dif_leaves_match_direct_transforms_and_short_fallbacks"]);
        }
        steps.push(core);
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    assert_eq!(steps.len(), 11);
    for file in [
        "design.md",
        "history-audit.md",
        "eight.rs",
        "source-audit.json",
        "setup.rs",
        "native.rs",
        "native-helper",
        "native-flags.txt",
        "candidate-flags.txt",
    ] {
        pin(&mut pins, r(file));
    }
    for file in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        pin(&mut pins, Path::new(Z).join(file));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared11native steps;176source/174exact/reverse175;FFT8columns;balanced13exact;OFF124/ON126core+8prover required;field33inherited;production175unchanged");
}

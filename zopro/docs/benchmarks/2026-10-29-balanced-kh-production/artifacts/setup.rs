fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    let full = read(format!("{Q}/analysis.json"));
    assert_eq!(full["gate_passed"], false);
    assert_eq!(full["failed_checks"], json!(["1x2-w16/OFF/accepted"]));
    assert_eq!(
        full["native_field_tests"],
        "field source exact;inherited accepted33test qualification;not rerun"
    );
    let compat = read(format!("{C}/analysis.json"));
    assert_eq!(compat["qualified_additional_scope"], true);
    assert_eq!(compat["independent_proof_count"], 16500);
    assert_eq!(compat["adopted"], false);
    for root in [Q, C] {
        for name in ["analysis.json", "proof-inventory.json", "design.md"] {
            pin(&mut pins, Path::new(root).join(name));
        }
    }
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 174);
    for (file, h) in originals {
        let from = Path::new(Z).join(file);
        assert_eq!(hash(&from), h.as_str().unwrap(), "{file}");
        pin(&mut pins, &from);
        for label in ["source", "control-source"] {
            let to = r(label).join(file);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(&from, to).unwrap();
        }
    }
    let parent = "crates/zopro-core/src/msm_affine/borrowed_pending.rs";
    let child = "crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs";
    let p = r("source").join(parent);
    let original = fs::read_to_string(&p).unwrap();
    let declaration_before = "use zopro_bn254::{Affine, CurveConfig, Field, Fq, Fr, Projective};\n";
    let declaration_after = format!("{declaration_before}\nmod balanced;\n");
    let dispatch_before = "    assert!(k.len().min(h.len()) >= 1024);\n";
    let dispatch_after = format!("{dispatch_before}    if bits == 13 {{\n        return balanced::fused(k, sk, h, sh);\n    }}\n");
    let mut text = original.clone();
    once(&mut text, declaration_before, &declaration_after);
    once(&mut text, dispatch_before, &dispatch_after);
    fs::write(&p, text).unwrap();
    let from = Path::new(P).join("source").join(child);
    for root in [Q, C] {
        let same = Path::new(root).join("source").join(child);
        assert_eq!(hash(&from), hash(&same));
        pin(&mut pins, same);
    }
    pin(&mut pins, &from);
    let measured = fs::read_to_string(&from).unwrap();
    let trace = "    if std::env::var_os(\"ZOPRO_BALANCED_KH_TRACE\").is_some(){eprintln!(\"BALANCED_KH windows=20 bits=255 short=5 long=15 k={} h={}\",k.len(),h.len());}\n";
    let mut clean = measured.clone();
    once(&mut clean, trace, "");
    let to = r("source").join(child);
    fs::create_dir_all(to.parent().unwrap()).unwrap();
    fs::write(&to, &clean).unwrap();
    let normalized = r("measured-without-trace.rs");
    fs::write(&normalized, &clean).unwrap();
    let status = std::process::Command::new("rustfmt")
        .args(["--edition", "2021"])
        .arg(&to)
        .arg(&normalized)
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(fs::read(&to).unwrap(), fs::read(&normalized).unwrap());
    let mut reverse = fs::read_to_string(&p).unwrap();
    once(&mut reverse, &dispatch_after, dispatch_before);
    once(&mut reverse, &declaration_after, declaration_before);
    assert_eq!(reverse, original);
    let mut exact = 0;
    for (file, h) in originals {
        if hash(r("source").join(file)) == h.as_str().unwrap() {
            exact += 1;
        }
        for label in ["source", "control-source"] {
            pin(&mut pins, r(label).join(file));
        }
    }
    assert_eq!(exact, 173);
    pin(&mut pins, &to);
    pin(&mut pins, &normalized);
    let old_field_receipt = read(format!("{C}/inherited-field-tests.json"));
    assert_eq!(old_field_receipt["portable_field_tests"], 11);
    assert_eq!(old_field_receipt["same_arithmetic_source"], true);
    let receipt_path = old_field_receipt["receipt"].as_str().unwrap();
    let receipt = read(receipt_path);
    assert_eq!(receipt["passed_tests"].as_array().unwrap().len(), 11);
    pin(&mut pins, receipt_path);
    let mut field_files = 0;
    for file in originals
        .keys()
        .filter(|p| p.starts_with("crates/zopro-bn254/"))
    {
        let qualified = Path::new(C).join("source").join(file);
        assert_eq!(hash(&qualified), hash(r("source").join(file)));
        pin(&mut pins, qualified);
        field_files += 1;
    }
    assert_eq!(field_files, 17);
    save(
        r("field-reuse.json"),
        &json!({"native_field_tests":33,"portable_field_tests":11,"new_field_tests":0,"source_files_compared":field_files,"portable_receipt":receipt_path,"source_exact":true}),
    );
    save(
        r("source-audit.json"),
        &json!({"source_files":175,"original_files":174,"originals_exact":exact,"reverse174_exact":true,"new_modules":1,"parent":parent,"child":child,"declaration_before":declaration_before,"declaration_after":declaration_after,"dispatch_before":dispatch_before,"dispatch_after":dispatch_after,"measured_child_sha256":hash(&from),"removed_trace":trace,"formatted_measured_without_trace_sha256":hash(&normalized),"clean_child_sha256":hash(&to),"clean_child_exact_to_formatted_measured_without_trace":true,"runtime_algorithm_unchanged":true,"no_experimental_cfg":true,"no_diagnostic_trace":true,"field_unchanged":true,"forecasts_unchanged":true,"production_worktree_changed":false,"previous_performance_gate_passed":false,"previous_failed_checks":full["failed_checks"]}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_balanced_kh"));
    let portable_flags = format!(
        "{} --cfg no_ifma --cfg no_asm",
        flags.replace("target-cpu=native", "target-cpu=x86-64")
    );
    fs::write(r("native-flags.txt"), flags).unwrap();
    fs::write(r("portable-flags.txt"), &portable_flags).unwrap();
    let mut steps = vec![];
    for label in ["control", "candidate", "portable"] {
        let cargo = r(if label == "control" {
            "control-source/Cargo.toml"
        } else {
            "source/Cargo.toml"
        });
        let selected = if label == "portable" {
            portable_flags.as_str()
        } else {
            flags
        };
        let mut env = read(format!("{Q}/run-plan.json"))["steps"][0]["environment"].clone();
        env["RUSTFLAGS"] = json!(selected);
        env["CARGO_TARGET_DIR"] = json!(r(format!("target-{label}")));
        env["CARGO_BUILD_JOBS"] = json!("16");
        env["RAYON_NUM_THREADS"] = json!("4");
        if label != "control" {
            steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["msm_affine::borrowed_pending::balanced::tests::variable_booth_digits_reconstruct_exact_integers","msm_affine::borrowed_pending::balanced::tests::balanced_complete_generator_oracle_and_uniform_control"],"unfiltered":true,"timeout_seconds":1800}));
            steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        let mut link = json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600});
        if label == "portable" {
            link["rustc_flags"] = json!([
                "-O",
                "-C",
                "target-cpu=x86-64",
                "-C",
                "lto=fat",
                "-C",
                "codegen-units=1"
            ]);
        }
        steps.push(link);
    }
    assert_eq!(steps.len(), 13);
    for file in [
        "design.md",
        "setup.rs",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "field-reuse.json",
        "native-flags.txt",
        "portable-flags.txt",
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
    println!("Prepared13clean build steps;173exact/reverse174;175source; no private cfg/trace; native124/8 portable101/8 required; field33/11 reused; production unchanged");
}

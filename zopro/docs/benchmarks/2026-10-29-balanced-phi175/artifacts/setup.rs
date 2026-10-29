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
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
    }
    let parent = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let before = "    hb_raw(bases, words, 12, 2)";
    let after=format!("    #[cfg(zopro_balanced_phi)]\n    if let Some(value) = super::g2_borrowed::balanced_raw(bases, words) {{ return value; }}\n{before}");
    let path = r("source").join(parent);
    let mut text = fs::read_to_string(&path).unwrap();
    once(&mut text, before, &after);
    fs::write(path, text).unwrap();
    let parent2 = "crates/zopro-core/src/msm_affine/g2_borrowed.rs";
    let suffix = r#"
#[cfg(any(test,zopro_balanced_phi))]
mod balanced;
#[cfg(test)]
thread_local! {static BALANCED_CALLS: std::cell::Cell<usize> = const {std::cell::Cell::new(0)};}
#[cfg(any(test,zopro_balanced_phi))]
pub(super) fn balanced_raw(bases:&[G2Affine],words:&mut [[u64;4]])->Option<G2Projective> {
    if !BatchAdd::<G2Config>::default().borrowed_g2_enabled() {return None;}
    #[cfg(test)] BALANCED_CALLS.with(|c|c.set(c.get()+1));
    Some(balanced::raw(bases,words,2))
}
"#;
    let path = r("source").join(parent2);
    let original = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{original}{suffix}")).unwrap();
    let helper = "crates/zopro-core/src/msm_affine/g2_borrowed/balanced.rs";
    let to = r("source").join(helper);
    fs::create_dir_all(to.parent().unwrap()).unwrap();
    fs::copy(r("balanced.rs"), &to).unwrap();
    let mut exact = 0;
    let mut field = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let path = r("source").join(p);
        let bytes = fs::read(&path).unwrap();
        if hash(&path) == h.as_str().unwrap() {
            exact += 1;
        }
        let restored = if p == parent {
            let text = String::from_utf8(bytes).unwrap();
            assert_eq!(text.matches(&after).count(), 1);
            text.replacen(&after, before, 1).into_bytes()
        } else if p == parent2 {
            String::from_utf8(bytes)
                .unwrap()
                .strip_suffix(suffix)
                .unwrap()
                .as_bytes()
                .to_vec()
        } else {
            bytes
        };
        assert_eq!(
            format!("{:x}", Sha256::digest(&restored)),
            h.as_str().unwrap()
        );
        if p.starts_with("crates/zopro-bn254/") {
            assert_eq!(hash(&path), h.as_str().unwrap());
            field += 1;
        }
        pin(&mut pins, path);
    }
    assert_eq!(exact, 173);
    assert_eq!(field, 17);
    pin(&mut pins, to);
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"original_files":175,"originals_exact":173,"reverse175_exact":true,"changed_file":parent,"before":before,"after":after,"second_parent":parent2,"second_parent_suffix":suffix,"new_helper":helper,"all17field_source_files_exact":true,"field33_tests_inherited":true,"accepted_KH_and_A_exact":true,"workspace_forecast_exact":true,"production_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    let candidate = format!("{flags} --cfg zopro_balanced_phi");
    assert!(!flags.contains("zopro_balanced_phi"));
    fs::write(r("native-flags.txt"), flags).unwrap();
    fs::write(r("candidate-flags.txt"), &candidate).unwrap();
    let required=["msm_affine::g2_borrowed::balanced::tests::exact_signed_component_reconstruction_and_geometry","msm_affine::g2_borrowed::balanced::tests::balanced_phi_matches_independent_generator_sums","msm_affine::g2_borrowed::balanced::tests::compiled_dispatch_matches_policy"];
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
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":required,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    assert_eq!(steps.len(), 10);
    for f in [
        "design.md",
        "history-audit.md",
        "balanced.rs",
        "bootstrap.rs",
        "setup.rs",
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
    println!("Prepared10native steps;176private173exact/reverse175;127core8prover required each;field33inherited");
}

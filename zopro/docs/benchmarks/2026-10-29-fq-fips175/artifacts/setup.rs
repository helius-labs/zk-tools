fn setup() {
    use std::process::Command;
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["full_performance_gate_passed"], false);
    for (p, h) in read(m["native_build_source_pins"].as_str().unwrap())
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
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
    let file = "crates/zopro-bn254/src/ifma.rs";
    let p = r("source").join(file);
    let original = fs::read_to_string(&p).unwrap();
    let old = "    fn product<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {";
    let new=format!("{old} #[cfg(zopro_fq_fips)] if LANES==8 && MIXED && C::P==crate::FqConfig::P {{return self.product_fips::<FUSED>(b,c,d);}}");
    let helper=fs::read_to_string(r("kernel.txt")).unwrap();
    let marker = "#[cfg(test)]\nmod tests {";
    let mut text = original.clone();
    once(&mut text, old, &new);
    once(&mut text, marker, &format!("{helper}{marker}"));
    let test_marker="                verify(v[0].difference2(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]-c[i]*d[i]));";
    let extra=r#"                #[cfg(zopro_fq_fips)]
                if LANES==8 && MIXED && C::P==crate::FqConfig::P {
                    verify(v[0].product_fips::<false>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]));
                    verify(v[0].product_fips::<true>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]+c[i]*d[i]));
                    let mut fips_chain=v[0];let mut fips_want=a;
                    for step in 0..16 {
                        if step%2==0 {
                            fips_chain=fips_chain.product_fips::<false>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]*=b[i];}
                        } else {
                            fips_chain=fips_chain.product_fips::<true>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]=fips_want[i]*b[i]+c[i]*d[i];}
                        }
                        verify(fips_chain,fips_want);
                    }
                }
"#;
    once(&mut text,test_marker,&format!("{test_marker}\n{extra}"));
    fs::write(&p, &text).unwrap();
    let mut back = text;
    once(&mut back, &new, old);
    once(&mut back, &helper, "");
    once(&mut back,&format!("{test_marker}\n{extra}"),test_marker);
    assert_eq!(back, original);
    let generator = "scripts/ifma8.py";
    assert!(originals.contains_key(generator));
    let generated = "crates/zopro-bn254/src/ifma8.rs";
    let command = |script: &Path| {
        let o = Command::new("python3").arg(script).output().unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        o
    };
    let output = command(&r("source").join(generator));
    fs::write(r("generator.log"), output.stdout).unwrap();
    fs::write(r("generator.stderr.log"), output.stderr).unwrap();
    for f in [file, generator] {
        let to = r("reverse-check").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(Path::new(Z).join(f), to).unwrap();
    }
    let reverse = command(&r("reverse-check").join(generator));
    fs::write(r("reverse-generator.log"), reverse.stdout).unwrap();
    fs::write(r("reverse-generator.stderr.log"), reverse.stderr).unwrap();
    assert_eq!(
        hash(r("reverse-check").join(generated)),
        originals[generated].as_str().unwrap()
    );
    assert_eq!(
        hash(r("reverse-check").join(file)),
        originals[file].as_str().unwrap()
    );
    let mut exact = 0;
    for (file, h) in originals {
        let p = r("source").join(file);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 173);
    save(
        r("source-audit.json"),
        &json!({"source_files":175,"original_files":175,"originals_exact":exact,"reverse175_by_unedit_and_official_generator":true,"template_file":file,"generated_file":generated,"old":old,"new":new,"helper":helper,"candidate_test_extra":extra,"marker":marker,"generator_unchanged":true,"product_cios_body_exact":true,"new_diagonal_arithmetic":true,"representation_unchanged":true,"core_solver_key_scheduler_forecast_exact":true,"production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_fq_fips"));
    let candidate = format!("{flags} --cfg zopro_fq_fips");
    fs::write(r("native-flags.txt"), flags).unwrap();
    fs::write(r("candidate-flags.txt"), &candidate).unwrap();
    let mut steps = vec![
        json!({"kind":"command","name":"generator-check","argv":["python3",r("source").join(generator),"--check"],"timeout_seconds":180}),
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
        steps.push(json!({"kind":"command","name":format!("{label}-field"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":["ifma8::tests::cios_carry_matches_arkworks","ifma8::tests::fused_reduction_boundaries"],"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));
    assert_eq!(steps.len(), 14);
    for f in [
        "design.md",
        "algebra.md",
        "kernel.txt",
        "model.rs",
        "model.json",
        "build-model.sh",
        "bootstrap.rs",
        "history-audit.md",
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
    println!("Prepared14steps;175source/173exact/reverse175;officialgenerator;field33core124prover8eachrequired;ordinaryandfusedFq8MIXEDdiagonal;production175unchanged");
}

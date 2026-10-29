fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 175);
    assert_eq!(read(r("model.json"))["status"], "passed");
    assert_eq!(read(r("model.json"))["quotient_checks"], 601680);
    let old = Path::new("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175");
    let mut pins = BTreeMap::new();
    let mut add = |p: PathBuf| {
        pins.insert(p.display().to_string(), hash(&p));
    };
    add(manifest.clone());
    let msm_suffix="\n#[cfg(zopro_gauss_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_gauss_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    let lib_suffix="\n#[cfg(zopro_gauss_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(f);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let dest = r("source").join(f);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(&from, &dest).unwrap();
        add(from);
        let frozen = old.join("source-frozen").join(f);
        let mut bytes = fs::read(&frozen).unwrap();
        for (file, suffix) in [
            ("crates/zopro-core/src/msm.rs", msm_suffix),
            ("crates/zopro-core/src/lib.rs", lib_suffix),
        ] {
            if f == file {
                assert!(bytes.ends_with(suffix.as_bytes()));
                bytes.truncate(bytes.len() - suffix.len());
            }
        }
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), h.as_str().unwrap());
        add(frozen);
    }
    for file in ["caller_probe.rs", "probe.rs", "perf.rs"] {
        fs::copy(old.join(file), r(file)).unwrap();
        add(old.join(file));
        add(r(file));
    }
    let flags = format!(
        "{} --cfg zopro_gauss_caller_probe",
        m["rustflags"].as_str().unwrap()
    );
    let frozen = read(old.join("artifacts-frozen.json"));
    assert_eq!(frozen["rustflags"], flags);
    for (p, h) in frozen["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        add(PathBuf::from(p));
    }
    let oldpins = read(old.join("pins.json"));
    let oldbinary = old.join("frozen-probe");
    assert_eq!(hash(&oldbinary), oldpins[oldbinary.to_str().unwrap()]);
    fs::copy(&oldbinary, r("frozen-probe")).unwrap();
    add(oldbinary);
    add(r("frozen-probe"));
    add(old.join("artifacts-frozen.json"));
    save(r("artifacts-frozen.json"), &frozen);
    let frozen_tests = read(old.join("native/core-frozen/tests.json"));
    assert_eq!(frozen_tests["passed_tests"].as_array().unwrap().len(), 128);
    assert_eq!(frozen_tests["unfiltered"], true);
    save(r("inherited-frozen-tests.json"), &frozen_tests);
    for file in [
        "native/core-frozen/tests.json",
        "native/core-frozen/command.json",
        "native/core-frozen/exit.json",
        "native/core-frozen/stdout.log",
    ] {
        add(old.join(file));
    }
    drop(add);
    let field_suffix = "\n#[cfg(zopro_complex_rowq)]\ninclude!(\"complex_rowq.rs\");\n";
    let f = "crates/zopro-bn254/src/ifma8.rs";
    let s = fs::read_to_string(r("source").join(f)).unwrap();
    fs::write(r("source").join(f), format!("{s}{field_suffix}")).unwrap();
    let fieldhelper = "crates/zopro-bn254/src/complex_rowq.rs";
    fs::copy(r("kernel.rs"), r("source").join(fieldhelper)).unwrap();
    let cfg="all(zopro_complex_rowq,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let simd_suffix=format!("\n#[cfg({cfg})]\nimpl FqBatch{{\n#[inline(always)]pub(crate)fn complex_rowq<const CONJ:bool>(a:&[Self;2],b:&[Self;2])->[Self;2]{{let[x,y]=zopro_bn254::ifma8::Ifma8::complex_rowq::<CONJ>(&a[0].0,&a[1].0,&b[0].0,&b[1].0);[Self(x),Self(y)]}}\n}}\n");
    let f = "crates/zopro-core/src/simd.rs";
    let s = fs::read_to_string(r("source").join(f)).unwrap();
    fs::write(r("source").join(f), format!("{s}{simd_suffix}")).unwrap();
    let f = "crates/zopro-core/src/msm/g2_borrowed.rs";
    let original = fs::read_to_string(r("source").join(f)).unwrap();
    let mut s = original.clone();
    let edits=[("        let slope = Fq2::lanes_scaled_numerator(&dy, &denominator, &factor);",format!("        #[cfg({cfg})]\n        let slope = Fq2::lanes_scale(&FqBatch::complex_rowq::<true>(&dy,&denominator),&factor);\n        #[cfg(not({cfg}))]\n        let slope = Fq2::lanes_scaled_numerator(&dy, &denominator, &factor);")),("        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);",format!("        #[cfg({cfg})]\n        let y = Fq2::lanes_sub(&FqBatch::complex_rowq::<false>(&slope,&Fq2::lanes_sub(&ax,&x)),&ay);\n        #[cfg(not({cfg}))]\n        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);"))];
    for (a, b) in &edits {
        assert_eq!(s.matches(a).count(), 1);
        s = s.replace(a, b);
    }
    fs::write(r("source").join(f), &s).unwrap();
    for (a, b) in edits.iter().rev() {
        s = s.replace(b, *a);
    }
    assert_eq!(s, original);
    fs::copy(
        r("caller_probe.rs"),
        r("source/crates/zopro-core/src/msm/caller_probe.rs"),
    )
    .unwrap();
    for (f, suffix) in [
        ("crates/zopro-core/src/msm.rs", msm_suffix),
        ("crates/zopro-core/src/lib.rs", lib_suffix),
    ] {
        let s = fs::read_to_string(r("source").join(f)).unwrap();
        fs::write(r("source").join(f), format!("{s}{suffix}")).unwrap();
    }
    let suffixes = json!({"crates/zopro-bn254/src/ifma8.rs":field_suffix,"crates/zopro-core/src/simd.rs":simd_suffix,"crates/zopro-core/src/msm.rs":msm_suffix,"crates/zopro-core/src/lib.rs":lib_suffix});
    let mut exact = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = r("source").join(f);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        } else {
            let mut s = fs::read_to_string(&p).unwrap();
            if f == "crates/zopro-core/src/msm/g2_borrowed.rs" {
                for (a, b) in edits.iter().rev() {
                    s = s.replace(b, *a);
                }
            }
            if let Some(suffix) = suffixes.get(f).and_then(Value::as_str) {
                s = s.strip_suffix(suffix).unwrap().into();
            }
            assert_eq!(s, fs::read_to_string(Path::new(Z).join(f)).unwrap());
        }
        pins.insert(p.display().to_string(), hash(p));
    }
    assert_eq!(exact, 170);
    for f in [fieldhelper, "crates/zopro-core/src/msm/caller_probe.rs"] {
        pins.insert(
            r("source").join(f).display().to_string(),
            hash(r("source").join(f)),
        );
    }
    save(
        r("source-audit.json"),
        &json!({"original_files":175,"source_files":177,"originals_exact":170,"reverse175_exact":true,"frozen_baseline_reused_byte_exact":true,"frozen_source_reverse175_exact":true,"inherited_frozen_core_tests":128,"field_files_original":17,"field_originals_exact":16,"field_recovery_exact":true,"new_field_helper":fieldhelper,"suffixes":suffixes,"borrowed_body_edits":edits,"only_runtime_core_change":"two borrowed G2 complex products","extra_input_conjugation":false,"scratch_allocations_and_geometry_unchanged":true,"new_heap_tables":0}),
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    let mut steps = vec![];
    for label in ["off", "on"] {
        let full = if label == "on" {
            format!("{flags} --cfg zopro_complex_rowq")
        } else {
            flags.clone()
        };
        let mut env = m["environment"].clone();
        env["RUSTFLAGS"] = json!(full);
        env["CARGO_TARGET_DIR"] = json!(r(format!("target-{label}")));
        env["CARGO_BUILD_JOBS"] = json!("16");
        env["RAYON_NUM_THREADS"] = json!("4");
        let cargo = r("source/Cargo.toml");
        if label == "on" {
            steps.push(json!({"kind":"command","name":"field-on","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","complex_rowq_tests","--","--nocapture","--test-threads=1"],"environment":env,"required_tests":["ifma8::complex_rowq_tests::fq_complex_rowq_lazy_and_chains","ifma8::complex_rowq_tests::fr_complex_rowq_lazy_and_chains"],"timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["msm::caller_probe::tests::borrowed_signed_g2_generator_oracle_tails_and_reuse"],"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
    }
    for f in [
        "design.md",
        "bounds.md",
        "history-audit.md",
        "setup.rs",
        "native.rs",
        "native-helper",
        "make-native.rs",
        "make-kernel.rs",
        "pair-prefix.rs",
        "field-tests.rs",
        "kernel.rs",
        "model.json",
        "model/src/main.rs",
        "model/Cargo.toml",
        "model/Cargo.lock",
        "model-build.log",
        "model/target/release/complex-rowq-exact-model",
        "model.stderr.log",
        "source-audit.json",
        "native-flags.txt",
        "artifacts-frozen.json",
        "inherited-frozen-tests.json",
        "caller_probe.rs",
        "probe.rs",
        "perf.rs",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared7freshbuild/test/freeze steps; frozen128core/current175sources/binary/flags inherited exact; private177/170exact/reverse175");
}

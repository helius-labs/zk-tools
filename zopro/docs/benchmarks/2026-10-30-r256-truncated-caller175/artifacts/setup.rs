fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let m=read(&manifest);assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),175);
    let old=Path::new("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175");
    let reuse=Path::new("/home/ubuntu/src/zopro-perf-work/early-x175");
    let mut pins=BTreeMap::new();pins.insert(manifest.display().to_string(),hash(&manifest));
    let msm_suffix="\n#[cfg(zopro_gauss_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_gauss_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    let lib_suffix="\n#[cfg(zopro_gauss_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    for(f,h)in m["source_files_sha256"].as_object().unwrap() {
        let from=Path::new(Z).join(f);assert_eq!(hash(&from),h.as_str().unwrap());
        let to=r("source").join(f);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(&from,&to).unwrap();
        pins.insert(from.display().to_string(),hash(from));
        let p=old.join("source-frozen").join(f);let mut bytes=fs::read(&p).unwrap();
        for(file,suffix)in[("crates/zopro-core/src/msm.rs",msm_suffix),("crates/zopro-core/src/lib.rs",lib_suffix)] { if f==file {assert!(bytes.ends_with(suffix.as_bytes()));bytes.truncate(bytes.len()-suffix.len());} }
        assert_eq!(format!("{:x}",Sha256::digest(bytes)),h.as_str().unwrap());pins.insert(p.display().to_string(),hash(p));
    }
    assert_eq!(hash(r("caller_probe.rs")),hash(old.join("source-frozen/crates/zopro-core/src/msm/caller_probe.rs")));
    let field_suffix="\n#[cfg(zopro_r256_truncated)]\ninclude!(\"r256_truncated.rs\");\n";
    let field_before="    fn product<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {\n";
    let field_after=format!("{field_before}        #[cfg(zopro_r256_truncated)]\n        if LANES == 8 && MIXED {{\n            #[cfg(test)] r256_truncated_tests::HITS.fetch_add(1,core::sync::atomic::Ordering::Relaxed);\n            return self.product_truncated::<FUSED>(b,c,d);\n        }}\n");
    let suffixes=BTreeMap::from([("crates/zopro-bn254/src/ifma8.rs",field_suffix),("crates/zopro-core/src/msm.rs",msm_suffix),("crates/zopro-core/src/lib.rs",lib_suffix)]);
    for(f,suffix)in&suffixes {
        let p=r("source").join(f);let mut text=fs::read_to_string(&p).unwrap();
        if *f=="crates/zopro-bn254/src/ifma8.rs" {assert_eq!(text.matches(field_before).count(),1);text=text.replace(field_before,&field_after);}
        text.push_str(suffix);fs::write(p,text).unwrap();
    }
    for(from,to)in[("field.rs","crates/zopro-bn254/src/r256_truncated.rs"),("caller_probe.rs","crates/zopro-core/src/msm/caller_probe.rs")] {fs::copy(r(from),r("source").join(to)).unwrap();pins.insert(r("source").join(to).display().to_string(),hash(r("source").join(to)));}
    let mut exact=0;
    for(f,h)in m["source_files_sha256"].as_object().unwrap() {
        let p=r("source").join(f);
        if hash(&p)==h.as_str().unwrap(){exact+=1;}else{
            let text=fs::read_to_string(&p).unwrap();let mut text=text.strip_suffix(suffixes[f.as_str()]).unwrap().to_string();
            if f=="crates/zopro-bn254/src/ifma8.rs" {assert_eq!(text.matches(&field_after).count(),1);text=text.replace(&field_after,field_before);}
            assert_eq!(format!("{:x}",Sha256::digest(text)),h.as_str().unwrap());
        }
        pins.insert(p.display().to_string(),hash(p));
    }
    assert_eq!(exact,172);
    let patch=Path::new(Z).join("docs/benchmarks/2026-10-24-truncated-redc-component/evidence/source.patch");
    let raw=fs::read_to_string(&patch).unwrap();let body=fs::read_to_string(r("inherited-helper.txt")).unwrap();
    let oldpart=raw.split("+++ candidate/crates/zopro-bn254/src/ifma8.rs").nth(1).unwrap();
    let at=oldpart.find("+    // -p^-1 modulo the full Montgomery radix").unwrap();
    let expected:String=oldpart[at..].lines().take_while(|l|l.starts_with('+')).map(|l|format!("{}\n",&l[1..])).collect();assert_eq!(body,expected);
    assert!(fs::read_to_string(r("field.rs")).unwrap().contains(&body));pins.insert(patch.display().to_string(),hash(patch));
    let flags=format!("{} --cfg zopro_gauss_caller_probe",m["rustflags"].as_str().unwrap());fs::write(r("native-flags.txt"),&flags).unwrap();
    let frozen=read(reuse.join("artifacts-frozen.json"));assert_eq!(frozen["rustflags"],flags);
    for(p,h)in frozen["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.into(),h.as_str().unwrap().into());}
    save(r("artifacts-frozen.json"),&frozen);
    let probe=read(reuse.join("probe-frozen.json"));assert_eq!(hash(probe["binary"].as_str().unwrap()),probe["sha256"]);
    for(p,h)in probe["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.into(),h.as_str().unwrap().into());}
    assert_eq!(hash(r("probe.rs")),hash(reuse.join("probe.rs")));assert_eq!(hash(r("perf.rs")),hash(reuse.join("perf.rs")));save(r("probe-frozen.json"),&probe);
    let mut steps=vec![];
    for label in["off","on"] {
        let full=if label=="on"{format!("{flags} --cfg zopro_r256_truncated")}else{flags.clone()};
        let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(full);env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
        for(pkg,suite)in[("zopro-bn254","field"),("zopro-core","core")] {steps.push(json!({"kind":"command","name":format!("{suite}-{label}"),"argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p",pkg,"--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));}
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("link-{label}"),"argv":[r("native-helper"),"link",label],"timeout_seconds":600}));
    }
    save(r("source-audit.json"),&json!({"original_files":175,"snapshot_files":177,"originals_exact":172,"reverse175_exact":true,"field_originals_exact":16,"suffixes":suffixes,"field_before":field_before,"field_after":field_after,"inherited_arithmetic_body_exact":true,"production_changed":false,"frozen_source_reverse175_exact":true,"frozen_probe_and_driver_exact":true}));
    for f in["bootstrap.rs","setup.rs","native.rs","native-helper","field-tests.rs","field.rs","inherited-helper.txt","caller_probe.rs","probe.rs","perf.rs","design.md","history-audit.md","source-audit.json","native-flags.txt"] {pins.insert(r(f).display().to_string(),hash(r(f)));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared10 native steps,172 originals exact/reverse175; inherited arithmetic exact; frozen source/libraries/probe reused");
}

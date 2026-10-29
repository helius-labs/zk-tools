fn setup(){
    assert!(!r("build-plan.json").exists());
    let parent=Path::new(PARENT);let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    let a=read(parent.join("source-audit.json"));assert_eq!(a["source_files"],177);assert_eq!(a["field_source_files_exact"],17);
    let oldpins=read(parent.join("build-pins.json"));let mut pins=BTreeMap::new();
    for p in [parent.join("source-audit.json"),parent.join("build-pins.json"),parent.join("analysis.json")]{pin(&mut pins,p);}
    assert_eq!(read(parent.join("analysis.json"))["independent_proof_count"],17551);
    let mut names:Vec<String>=m["source_files_sha256"].as_object().unwrap().keys().cloned().collect();
    for file in a["new_files"].as_array().unwrap(){names.push(file.as_str().unwrap().into());}assert_eq!(names.len(),177);
    for file in &names {
        let from=parent.join("source").join(file);assert_eq!(hash(&from),oldpins[from.display().to_string()].as_str().unwrap());pin(&mut pins,&from);
        let to=r("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();
    }
    let file="crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs";let p=r("source").join(file);let original=fs::read_to_string(&p).unwrap();let mut text=original.clone();let mut edits=vec![];
    let hook="        size: usize,\n    ) {\n        for (p, s) in bases.iter().zip(short) {";
    let hooked="        size: usize,\n    ) {\n        #[cfg(zopro_gls4_census)]\n        if census::enabled(){return census::run_feed(self,bases,short,component,w,size);}\n        for (p, s) in bases.iter().zip(short) {";
    let chunk="    let chunk = bases.len().div_ceil(PARTS);";
    let chunked="    let chunk = bases.len().div_ceil(PARTS);\n    #[cfg(zopro_gls4_census)]\n    let query_id=census::query();";
    let window="                    window(&bases[start..end], &short[start..end], w, &x_factor)";
    let tagged="                    #[cfg(zopro_gls4_census)]\n                    let _context=census::context(query_id,w,part);\n                    window(&bases[start..end], &short[start..end], w, &x_factor)";
    for (old,new) in [(hook,hooked),(chunk,chunked),(window,tagged)]{once(&mut text,old,new);edits.push(json!({"old":old,"new":new}));}
    let suffix="\n#[cfg(zopro_gls4_census)]\nmod census;\n";text.push_str(suffix);
    let mut reverse=text.clone();reverse.truncate(reverse.len()-suffix.len());for e in edits.iter().rev(){once(&mut reverse,e["new"].as_str().unwrap(),e["old"].as_str().unwrap());}assert_eq!(reverse,original);fs::write(&p,text).unwrap();
    let child="crates/zopro-core/src/msm_affine/g2_borrowed/gls4/census.rs";fs::copy(r("census.rs"),r("source").join(child)).unwrap();
    let mut exact=0;let mut field=0;for name in &names {
        let p=r("source").join(name);let same=hash(&p)==hash(parent.join("source").join(name));exact+=usize::from(same);
        if name.starts_with("crates/zopro-bn254/"){assert!(same);field+=1;}pin(&mut pins,p);
    }assert_eq!(exact,176);assert_eq!(field,17);pin(&mut pins,r("source").join(child));
    save(r("source-audit.json"),&json!({"source_files":178,"parent_source_files":177,"parent_files_exact":176,"parent_recovered_by_inverse_edits":true,"changed":file,"edits":edits,"suffix":suffix,"new_file":child,"field_source_files_exact":17,"original_Window_methods_exact":true,"arithmetic_and_queue_order_unchanged":true,"no_production_edits":true,"native128core8prover33field_inherited":true}));
    let oldflags=fs::read_to_string(parent.join("candidate-flags.txt")).unwrap();let flags=format!("{oldflags} --cfg zopro_gls4_census");fs::write(r("native-flags.txt"),oldflags).unwrap();fs::write(r("candidate-flags.txt"),&flags).unwrap();
    let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(flags);env["CARGO_TARGET_DIR"]=json!(r("target-candidate"));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");env["ZOPRO_GLS4_CENSUS"]=json!("0");
    let cargo=r("source/Cargo.toml");let mut steps=vec![];
    steps.push(json!({"kind":"command","name":"census-tests","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","msm_affine::g2_borrowed::gls4::census::tests","--","--test-threads=1"],"environment":env,"required_tests":["msm_affine::g2_borrowed::gls4::census::tests::observed_state_matches_original_horner_feeds","msm_affine::g2_borrowed::gls4::census::tests::explicit_exception_and_queue_counts"],"unfiltered":false,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180}));
    steps.push(json!({"kind":"link-sdk","name":"candidate","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));assert_eq!(steps.len(),5);
    for name in ["design.md","census.rs","setup.rs","bootstrap.rs","code-audit.rs","native.rs","native-helper","native-flags.txt","candidate-flags.txt","source-audit.json"]{pin(&mut pins,r(name));}
    for name in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{pin(&mut pins,Path::new(Z).join(name));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared5steps;178source/176parentexact/reverse177;only2newobserver tests;no benchmark claim");
}

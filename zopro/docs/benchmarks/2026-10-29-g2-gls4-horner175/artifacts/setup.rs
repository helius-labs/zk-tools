fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let m=read(&manifest);assert_eq!(m["adopted"],true);assert_eq!(m["full_performance_gate_passed"],false);
    let mut pins=BTreeMap::new();pin(&mut pins,&manifest);
    for (p,h) in read(m["native_build_source_pins"].as_str().unwrap()).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    pin(&mut pins,m["native_build_source_pins"].as_str().unwrap());
    let originals=m["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),175);
    for (file,h) in originals {
        let from=Path::new(Z).join(file);assert_eq!(hash(&from),h.as_str().unwrap());pin(&mut pins,&from);
        let to=r("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();
    }
    let hook_old="    hb_raw(bases, words, 12, 2)";
    let hook_new="    #[cfg(zopro_gls4_horner)]\n    if let Some(result)=super::g2_borrowed::gls4::try_raw(bases,words){return result;}\n    hb_raw(bases, words, 12, 2)";
    let module="\n#[cfg(zopro_gls4_horner)]\npub(super) mod gls4;\n";
    let cost=r#"
#[cfg(zopro_gls4_horner)]
pub(crate) fn gls4_horner_peak(n:usize,workers:usize)->Option<usize> {
    let p=shape::<G2Config>(1<<10,6,4,batch_size(11).max(32))?;
    add(add(vector::<[u64;5]>(n,false)?,p.held)?,mul(p.leaf,workers)?)
}
"#;
    let mut edits=vec![];
    for (file,old,new) in [
        ("crates/zopro-core/src/msm_affine/g2_phi.rs",hook_old,hook_new),
        ("crates/zopro-core/src/msm_affine/g2_borrowed.rs","",module),
        ("crates/zopro-core/src/msm_affine/workspace.rs","",cost),
    ] {
        let p=r("source").join(file);let original=fs::read_to_string(&p).unwrap();let mut text=original.clone();
        if old.is_empty(){text.push_str(new);}else{once(&mut text,old,new);}
        let mut back=text.clone();if old.is_empty(){assert!(back.ends_with(new));back.truncate(back.len()-new.len());}else{once(&mut back,new,old);}
        assert_eq!(back,original);fs::write(&p,&text).unwrap();edits.push(json!({"file":file,"old":old,"new":new,"append":old.is_empty(),"reverse_exact":true}));
    }
    let child="crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs";
    let lattice="crates/zopro-core/src/msm_affine/g2_borrowed/gls4/lattice.rs";
    for (name,to) in [("gls4.rs",child),("lattice.rs",lattice)] {
        let p=r("source").join(to);fs::create_dir_all(p.parent().unwrap()).unwrap();fs::copy(r(name),p).unwrap();
    }
    let mut exact=0;let mut field=0;
    for (file,h) in originals {
        let p=r("source").join(file);let unchanged=hash(&p)==h.as_str().unwrap();exact+=usize::from(unchanged);
        if file.starts_with("crates/zopro-bn254/"){assert!(unchanged);field+=1;}pin(&mut pins,p);
    }
    assert_eq!(exact,172);assert_eq!(field,17);
    for p in [child,lattice]{pin(&mut pins,r("source").join(p));}
    save(r("source-audit.json"),&json!({"source_files":177,"original_files":175,"originals_exact":172,"reverse175_exact":true,"edits":edits,"new_files":[child,lattice],"field_source_files_exact":17,"native_field_tests_inherited":33,"solver_key_field_fft_exact":true,"original_borrowed_Window_methods_exact":true,"old_lattice_body_reused":true,"new_costs_charged_via_resource_analysis":true,"production_worktree_changed":false}));
    let flags=m["rustflags"].as_str().unwrap();let candidate=format!("{flags} --cfg zopro_gls4_horner");fs::write(r("native-flags.txt"),flags).unwrap();fs::write(r("candidate-flags.txt"),&candidate).unwrap();
    let mut steps=vec![json!({"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180})];
    for label in ["control","candidate"] {
        let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(if label=="candidate"{candidate.as_str()}else{flags});env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
        let cargo=r("source/Cargo.toml");
        let required=if label=="candidate"{json!(["msm_affine::g2_borrowed::gls4::tests::decomposition_and_digits_reconstruct_fullwidth","msm_affine::g2_borrowed::gls4::tests::affine_and_projective_horner_maps_match_scalars","msm_affine::g2_borrowed::gls4::tests::full_msm_matches_independent_generator_sums","msm_affine::g2_borrowed::gls4::tests::checked_admission_and_fallback_preserve_words"])}else{json!(["msm_affine::exclusive_g2_tests::exclusive_queries_match_generator_oracle"])};
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));assert_eq!(steps.len(),12);
    for f in ["design.md","history-audit.md","gls4.rs","lattice.rs","historical-gls4.rs","historical-integer-oracle.json","bootstrap.rs","setup.rs","code-audit.rs","native.rs","native-helper","source-audit.json","native-flags.txt","candidate-flags.txt"]{pin(&mut pins,r(f));}
    for f in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{pin(&mut pins,Path::new(Z).join(f));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared12steps;177source/172originalexact/reverse175;field17exact/33inherited;OFF124/ON128core+8provereach;newonebucketGLS4");
}

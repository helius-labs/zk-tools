fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let m=read(&manifest);assert_eq!(m["adopted"],true);assert_eq!(m["full_performance_gate_passed"],false);
    let mut pins=BTreeMap::new();pin(&mut pins,&manifest);
    for(p,h)in read(m["native_build_source_pins"].as_str().unwrap()).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    pin(&mut pins,m["native_build_source_pins"].as_str().unwrap());
    let originals=m["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),175);
    for(file,h)in originals{let from=Path::new(Z).join(file);assert_eq!(hash(&from),h.as_str().unwrap());pin(&mut pins,&from);let to=r("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
    let guard="#[cfg(all(zopro_index_pack,zopro_resource_schedule,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
    let old="    let tile = 2048 / (core::mem::size_of::<C::Base>() / 32);\n    let mut tasks = Vec::new();";
    let new=format!("    {guard}\n    if let Some(value)=packed_index::try_reduce(bases,order,starts){{return value;}}\n{old}");
    let suffix=format!("\n{guard}\nmod packed_index;\n");
    let file="crates/zopro-core/src/msm.rs";let p=r("source").join(file);let original=fs::read_to_string(&p).unwrap();let mut text=original.clone();once(&mut text,old,&new);text.push_str(&suffix);
    let mut back=text.clone();back.truncate(back.len()-suffix.len());once(&mut back,&new,old);assert_eq!(back,original);fs::write(&p,&text).unwrap();
    let child="crates/zopro-core/src/msm/packed_index.rs";let p=r("source").join(child);fs::create_dir_all(p.parent().unwrap()).unwrap();fs::copy(r("packed_index.rs"),p).unwrap();
    let mut exact=0;let mut field=0;for(file,h)in originals{let p=r("source").join(file);let same=hash(&p)==h.as_str().unwrap();exact+=usize::from(same);if file.starts_with("crates/zopro-bn254/"){assert!(same);field+=1;}pin(&mut pins,p);}
    assert_eq!(exact,174);assert_eq!(field,17);pin(&mut pins,r("source").join(child));
    save(r("source-audit.json"),&json!({"source_files":176,"original_files":175,"originals_exact":174,"reverse175_exact":true,"edits":[{"file":file,"old":old,"new":new,"append":suffix}],"new_files":[child],"field_source_files_exact":17,"native_field_tests_inherited":33,"solver_key_field_fft_exact":true,"original_indexed_and_layer_bodies_exact":true,"new_costs_charged_via_resource_analysis":true,"production_worktree_changed":false}));
    let flags=m["rustflags"].as_str().unwrap();let candidate=format!("{flags} --cfg zopro_index_pack");fs::write(r("native-flags.txt"),flags).unwrap();fs::write(r("candidate-flags.txt"),&candidate).unwrap();
    let mut steps=vec![json!({"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180})];
    for label in ["control","candidate"] {
        let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(if label=="candidate"{candidate.as_str()}else{flags});env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
        let cargo=r("source/Cargo.toml");
        let required=if label=="candidate"{json!(["msm::packed_index::tests::geometry_covers_each_input_and_bucket_once","msm::packed_index::tests::grouped_tiles_match_independent_bucket_generator_sums","msm::packed_index::tests::checked_forecast_and_denied_admission_preserve_inputs"])}else{json!(["msm_affine::exclusive_g2_tests::exclusive_queries_match_generator_oracle"])};
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));assert_eq!(steps.len(),12);
    for f in ["design.md","history-audit.md","packed_index.rs","bootstrap.rs","setup.rs","code-audit.rs","native.rs","native-helper","source-audit.json","native-flags.txt","candidate-flags.txt"]{pin(&mut pins,r(f));}
    for f in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{pin(&mut pins,Path::new(Z).join(f));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared12steps;176source174originalexact/reverse175/field17exact33inherited;OFF124ON127core8provereach");
}

fn setup(){
    assert!(!r("build-plan.json").exists());let mp=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");let m=read(&mp);let mut pins=BTreeMap::new();pin(&mut pins,&mp);assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),175);
    for(p,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(p);assert_eq!(hash(&from),h.as_str().unwrap());pin(&mut pins,&from);let to=r("source").join(p);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}

    let mut changed=BTreeMap::new();let mut children=Vec::new();
    for (parent,files,new) in [
      ("/home/ubuntu/src/zopro-perf-work/cached-hash-sdk175",vec!["crates/zopro-core/src/msm_affine/grouping.rs","crates/zopro-core/Cargo.toml","Cargo.lock"],"crates/zopro-core/src/msm_affine/grouping/representative.rs"),
      ("/home/ubuntu/src/zopro-perf-work/queue-drain175",vec!["crates/zopro-core/src/msm_affine.rs","crates/zopro-core/src/msm_affine/borrowed_pending.rs","crates/zopro-core/src/msm_affine/g2_borrowed.rs"],"crates/zopro-core/src/msm_affine/queue_drain.rs")
    ]{
      let parent=Path::new(parent);let oldpins=read(parent.join("build-pins.json"));
      for p in files.into_iter().chain([new]){let from=parent.join("source").join(p);assert_eq!(hash(&from),oldpins[from.to_str().unwrap()].as_str().unwrap());pin(&mut pins,&from);fs::create_dir_all(r("source").join(p).parent().unwrap()).unwrap();fs::copy(&from,r("source").join(p)).unwrap();if m["source_files_sha256"].get(p).is_some(){assert!(changed.insert(p.to_string(),json!({"qualified_parent_source":from,"parent_hash":hash(&from)})).is_none());}else{children.push(p.to_string());}}
      for file in["source-audit.json","analysis.json"]{pin(&mut pins,parent.join(file));}
    }
    let(mut exact,mut fields)=(0,0);for(p,h)in m["source_files_sha256"].as_object().unwrap(){let file=r("source").join(p);if hash(&file)==h.as_str().unwrap(){exact+=1;}else{assert!(changed.contains_key(p));}if p.starts_with("crates/zopro-bn254/"){fields+=1;assert_eq!(hash(&file),h.as_str().unwrap());}pin(&mut pins,file);}assert_eq!(exact,169);assert_eq!(fields,17);assert_eq!(changed.len(),6);assert_eq!(children.len(),2);for p in &children{pin(&mut pins,r("source").join(p));}
    save(r("source-audit.json"),&json!({"source_files":177,"original_files":175,"originals_exact":169,"all17field_exact":true,"workspace_and_Window_fields_unchanged":true,"copied_parent_edits":changed,"new_children":children,"composition_only":true,"new_heap_or_key_state_vs_components":false,"parent_gates_remain_failed":true}));
    let flags=m["rustflags"].as_str().unwrap();let candidate=format!("{flags} --cfg zopro_queue_drain --cfg zopro_cached_hash_groups");fs::write(r("native-flags.txt"),flags).unwrap();fs::write(r("candidate-flags.txt"),&candidate).unwrap();
    let required=["msm_affine::queue_drain::tests::simd_width_pending_and_signed_destination_guards","msm_affine::borrowed_pending::queue_drain_tests::drain_and_fallback_match_independent_buckets","msm_affine::g2_borrowed::queue_drain_tests::drain_and_fallback_match_independent_buckets"];
    let mut steps=Vec::new();for label in["control","candidate"]{let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(if label=="candidate"{candidate.as_str()}else{flags});env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");let cargo=r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":required,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    for p in["design.md","history-audit.md","bootstrap.rs","setup.rs","native.rs","native-helper","source-audit.json","native-flags.txt","candidate-flags.txt"]{pin(&mut pins,r(p));}for p in["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{pin(&mut pins,Path::new(Z).join(p));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared10native steps;177private/169exact;127OFF/131ON core tests;exact composition;field/workspace unchanged");
}

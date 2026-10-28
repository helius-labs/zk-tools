fn setup(){
 assert!(!r("build-plan.json").exists());let m=read(MANIFEST);let mut pins=BTreeMap::new();
 for(f,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(f);assert_eq!(hash(&from),h.as_str().unwrap());let to=r("source").join(f);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(&from,&to).unwrap();pin(&mut pins,from);pin(&mut pins,to);}
 assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),174);
 let module=r("source/crates/zopro-core/src/msm/reduction_census.rs");fs::copy(r("reduction_census.rs"),&module).unwrap();pin(&mut pins,&module);
 let edits=[
  ("crates/zopro-core/src/msm.rs","                while local_counts.iter().any(|&count| count > 1) {","                #[cfg(zopro_reduction_census)] reduction_census::observe::<C>(points,local_starts,local_counts,0);\n                while local_counts.iter().any(|&count| count > 1) {"),
  ("crates/zopro-core/src/msm.rs","            while count[0] > 1 {","            #[cfg(zopro_reduction_census)] reduction_census::observe::<C>(&points,&[0],&count,1);\n            while count[0] > 1 {")];
 for(f,b,a)in edits{let p=r("source").join(f);let t=fs::read_to_string(&p).unwrap();assert_eq!(t.matches(b).count(),1);fs::write(&p,t.replace(b,a)).unwrap();pin(&mut pins,&p);}
 let suffixes=[("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_reduction_census)]\npub mod reduction_census;\n"),("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_reduction_census)]\npub use msm::reduction_census;\n")];
 for(f,s)in suffixes{let p=r("source").join(f);let t=fs::read_to_string(&p).unwrap();fs::write(&p,format!("{t}{s}")).unwrap();pin(&mut pins,&p);}
 let mut exact=0;for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=r("source").join(f);if hash(&p)==h.as_str().unwrap(){exact+=1}else{let mut text=fs::read_to_string(&p).unwrap();for(ff,s)in suffixes{if ff==f{text=text.strip_suffix(s).unwrap().to_string();}}for(ff,b,a)in edits.into_iter().rev(){if ff==f{text=text.replace(a,b);}}assert_eq!(format!("{:x}",Sha256::digest(text.as_bytes())),h.as_str().unwrap());}}
 assert_eq!(exact,172);save(r("source-audit.json"),&json!({"original_files":174,"candidate_files":175,"originals_exact":172,"reverse174_exact":true,"hooks":edits,"suffixes":suffixes,"new_module":"crates/zopro-core/src/msm/reduction_census.rs","arithmetic_and_point_mutation_unchanged":true,"observer_computes_extra_guard_arithmetic":true,"weighted_and_streaming_excluded":true,"first_layer_only":true,"field_key_solver_forecast_source_exact":true,"production_changed":false}));
 let flags=m["rustflags"].as_str().unwrap();fs::write(r("native-flags.txt"),flags).unwrap();
 let env=json!({"RUSTFLAGS":format!("{flags} --cfg zopro_reduction_census"),"CARGO_TARGET_DIR":r("target-candidate"),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_DEDUP":"1","ZOPRO_CPU_ZERO_AWARE_GROUPS":"1","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
 let cargo=r("source/Cargo.toml");let steps=json!([
  {"kind":"command","name":"core","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["msm::reduction_census::tests::first_layer_guards_match_curve_cases_and_disable_cleanly","msm::reduction_census::tests::concurrent_counts_preserve_slots_remainders_and_histograms"],"timeout_seconds":1800},
  {"kind":"command","name":"prover","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800},
  {"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800},
  {"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180},
  {"kind":"link-sdk","name":"candidate","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":r("diagnostic-sdk.rs"),"timeout_seconds":600}
 ]);
 for f in ["native.rs","native-helper","setup.rs","reduction_census.rs","diagnostic-sdk.rs","design.md","native-flags.txt","source-audit.json"]{pin(&mut pins,r(f));}pin(&mut pins,MANIFEST);
 save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared5native diagnostic steps;172exact/reverse174/source175;no optimization");
}

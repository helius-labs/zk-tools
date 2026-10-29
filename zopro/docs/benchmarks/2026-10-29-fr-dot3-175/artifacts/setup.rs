fn setup() {
    assert!(!r("build-plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let m=read(&manifest);assert_eq!(m["adopted"],true);assert_eq!(m["full_performance_gate_passed"],false);
    let mut pins=BTreeMap::new();pin(&mut pins,&manifest);
    for (p,h) in read(m["native_build_source_pins"].as_str().unwrap()).as_object().unwrap() {assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    pin(&mut pins,m["native_build_source_pins"].as_str().unwrap());
    let originals=m["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),175);
    for (file,h) in originals {
        let from=Path::new(Z).join(file);assert_eq!(hash(&from),h.as_str().unwrap());pin(&mut pins,&from);
        let to=r("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();
    }
    let fp_old="    /// self^e, e little-endian limbs.";
    let fp_new=r#"    /// Three canonical-input products with one reduction (private experiment).
    #[cfg(zopro_fr_dot3)]
    #[inline(always)]
    pub fn sum_of_three_products(a:&Self,b:&Self,c:&Self,d:&Self,e:&Self,f:&Self)->Self {
        Self(crate::sop3::sum3([a.0,c.0,e.0],[b.0,d.0,f.0],C::P,C::INV),PhantomData)
    }
    /// self^e, e little-endian limbs."#;
    let lib_old="pub mod fp;";
    let lib_new="pub mod fp;\n#[cfg(zopro_fr_dot3)]\nmod sop3;";
    let dense_old="                    next[i] = Fr::sum_of_products(&a[row], &s[0], &a[row + 1], &s[1])\n                        + a[row + 2] * s[2];";
    let dense_new=format!("                    #[cfg(zopro_fr_dot3)]\n                    {{ next[i]=Fr::sum_of_three_products(&a[row],&s[0],&a[row+1],&s[1],&a[row+2],&s[2]); }}\n                    #[cfg(not(zopro_fr_dot3))]\n                    {{\n{dense_old}\n                    }}");
    let mut changes=vec![];
    for (file,old,new) in [
        ("crates/zopro-bn254/src/fp.rs",fp_old,fp_new),
        ("crates/zopro-bn254/src/lib.rs",lib_old,lib_new),
        ("crates/zopro-core/src/solver/sparse_poseidon.rs",dense_old,dense_new.as_str()),
    ] {
        let p=r("source").join(file);let original=fs::read_to_string(&p).unwrap();let mut text=original.clone();
        once(&mut text,old,new);let mut reverse=text.clone();once(&mut reverse,new,old);assert_eq!(reverse,original);
        fs::write(&p,text).unwrap();pin(&mut pins,p);changes.push(json!({"file":file,"old":old,"new":new,"inverse_exact":true}));
    }
    let child="crates/zopro-bn254/src/sop3.rs";assert!(!originals.contains_key(child));fs::copy(r("sop3.rs"),r("source").join(child)).unwrap();pin(&mut pins,r("source").join(child));
    let mut exact=0;
    for (file,h) in originals {let p=r("source").join(file);if hash(&p)==h.as_str().unwrap(){exact+=1;}pin(&mut pins,p);}
    assert_eq!(exact,172);
    save(r("source-audit.json"),&json!({"source_files":176,"original_files":175,"originals_exact":172,"reverse175_by_inverse_edits":true,"changes":changes,"new_module":child,"key_scheduler_forecast_exact":true,"solver_dense_dot_only":true,"partial_rounds_exact":true,"IFMA_sources_exact":true,"production_worktree_changed":false}));
    let flags=m["rustflags"].as_str().unwrap();let candidate=format!("{flags} --cfg zopro_fr_dot3");
    fs::write(r("native-flags.txt"),flags).unwrap();fs::write(r("candidate-flags.txt"),&candidate).unwrap();
    let mut steps=vec![json!({"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180})];
    for label in ["control","candidate"] {
        let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(if label=="candidate"{candidate.as_str()}else{flags});env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
        let cargo=r("source/Cargo.toml");
        let required=if label=="candidate" {json!(["sop3::tests::small_radix_exact_cancellation","sop3::tests::canonical_sum3_and_chains_match_arkworks"])}else{json!(["ifma8::tests::cios_carry_matches_arkworks"])};
        steps.push(json!({"kind":"command","name":format!("{label}-field"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));assert_eq!(steps.len(),14);
    for f in ["design.md","history-audit.md","bounds.md","sop3.rs","bootstrap.rs","setup.rs","code-audit.rs","native.rs","native-helper","source-audit.json","native-flags.txt","candidate-flags.txt"] {pin(&mut pins,r(f));}
    for f in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"] {pin(&mut pins,Path::new(Z).join(f));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared14steps/176source172originalexact/reverse175;control33/candidate35field124core8provereach;densewidth3sum3only");
}

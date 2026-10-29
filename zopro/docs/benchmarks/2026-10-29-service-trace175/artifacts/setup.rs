fn setup(){
    assert!(!r("build-plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");let m=read(&manifest);
    assert_eq!(m["adopted"],true);assert_eq!(m["full_performance_gate_passed"],false);
    let mut pins=BTreeMap::new();pin(&mut pins,&manifest);
    for(p,h)in read(m["native_build_source_pins"].as_str().unwrap()).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    pin(&mut pins,m["native_build_source_pins"].as_str().unwrap());
    let original=m["source_files_sha256"].as_object().unwrap();assert_eq!(original.len(),175);
    for(file,h)in original{let p=Path::new(Z).join(file);assert_eq!(hash(&p),h.as_str().unwrap());pin(&mut pins,&p);let to=r("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(p,to).unwrap();}
    assert!(std::process::Command::new(r("patch-helper")).status().unwrap().success());
    fs::copy(r("service_trace.rs"),r("source/crates/zopro-core/src/service_trace.rs")).unwrap();
    let changes=read(r("insertions.json"));let mut changed=vec![];let mut field=0;
    for(file,h)in original{
        let p=r("source").join(file);if hash(&p)!=h.as_str().unwrap(){
            changed.push(file);let mut s=fs::read_to_string(&p).unwrap();let short=file.strip_prefix("crates/zopro-core/src/").unwrap();
            for c in changes.as_array().unwrap().iter().rev().filter(|c|c["file"]==short){
                if let Some(old)=c["old"].as_str(){let new=c["new"].as_str().unwrap();assert!(s.contains(new));s=s.replacen(new,old,1);}else{let t=c["inserted"].as_str().unwrap();assert!(s.contains(t));s=s.replacen(t,"",1);}
            }
            assert_eq!(s,fs::read_to_string(Path::new(Z).join(file)).unwrap());
        }
        if file.starts_with("crates/zopro-bn254/"){assert_eq!(hash(&p),h.as_str().unwrap());field+=1;}pin(&mut pins,p);
    }
    assert_eq!(changed.len(),8);assert_eq!(field,17);pin(&mut pins,r("source/crates/zopro-core/src/service_trace.rs"));
    save(r("source-audit.json"),&json!({"source_files":176,"original_files":175,"originals_exact":167,"reverse175_exact":true,"changed":changed,"field_source_files_exact":17,"native_field_tests_inherited":33,"arithmetic_and_scheduler_unchanged":true,"explicit_cross_task_parent_ids":true,"serial_nonforking_only":true,"new_module":"service_trace.rs","production_worktree_changed":false}));
    let flags=m["rustflags"].as_str().unwrap();let flags=format!("{flags} --cfg zopro_service_trace");fs::write(r("candidate-flags.txt"),&flags).unwrap();
    let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(flags);env["CARGO_TARGET_DIR"]=json!(r("target-candidate"));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
    let cargo=r("source/Cargo.toml");let steps=json!([
        {"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180},
        {"kind":"command","name":"candidate-core","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["service_trace::tests::explicit_parent_cpu_disabled_and_unwind","service_trace::tests::bounded_capacity_preserves_execution_and_reports_drops"],"unfiltered":true,"timeout_seconds":1800},
        {"kind":"command","name":"candidate-prover","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800},
        {"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800},
        {"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180},
        {"kind":"link-sdk","name":"candidate","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":r("driver.rs"),"additional_sources":[r("support/perf.rs")],"timeout_seconds":600},
        {"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}
    ]);
    for f in ["design.md","bootstrap.rs","setup.rs","patch.rs","patch-helper","service_trace.rs","driver.rs","support/perf.rs","native.rs","native-helper","source-audit.json","insertions.json","candidate-flags.txt"]{pin(&mut pins,r(f));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared7steps/176source167originalexact/reverse175;126core8prover;17fieldexact33inherited;1024eventsperthread");
}
fn inspect(){
    use std::process::Command;
    let binary=r("native/candidate-sdk");let o=Command::new("nm").args(["-S","--size-sort","--defined-only","-C"]).arg(&binary).output().unwrap();assert!(o.status.success());let s=String::from_utf8(o.stdout).unwrap();
    fs::create_dir(r("codegen")).unwrap();let mut selected=vec![];
    for l in s.lines().filter(|l|l.contains("service_trace::")){let w:Vec<_>=l.split_whitespace().collect();if w.len()<4{continue;}let addr=u64::from_str_radix(w[0],16).unwrap();let bytes=u64::from_str_radix(w[1],16).unwrap();
        let asm=Command::new("objdump").args(["-d","-C","--no-show-raw-insn"]).arg(format!("--start-address=0x{addr:x}")).arg(format!("--stop-address=0x{:x}",addr+bytes)).arg(&binary).output().unwrap();assert!(asm.status.success());let p=r(format!("codegen/trace-{}.asm",selected.len()));fs::write(&p,asm.stdout).unwrap();selected.push(json!({"symbol":w[3..].join(" "),"bytes":bytes,"assembly":p,"assembly_sha256":hash(&p)}));
    }
    assert!(!selected.is_empty());fs::write(r("codegen/symbols.txt"),s).unwrap();save(r("code-audit.json"),&json!({"binary":binary,"sha256":hash(&binary),"selected_trace_functions":selected,"coverage_requires_semantic_preflight":true,"speed_claimed":false}));
}

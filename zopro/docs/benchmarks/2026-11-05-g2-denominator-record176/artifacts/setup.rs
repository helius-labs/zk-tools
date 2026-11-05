fn setup() {
    let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let m=read(&manifest);let original=m["source_files_sha256"].as_object().unwrap();assert_eq!(original.len(),176);assert!(!r("source").exists());
    let previous=Path::new("/home/ubuntu/src/zopro-perf-work/paired-fq2-pack176");
    for name in ["caller_probe.rs","probe.rs","perf.rs"] { fs::copy(previous.join(name),r(name)).unwrap(); }
    let mut adapter=fs::read_to_string(r("caller_probe.rs")).unwrap();
    for old in ["std::mem::size_of::<[FqBatch; 2]>()","std::mem::size_of::<[FqBatch;2]>()"] { assert_eq!(adapter.matches(old).count(),1);adapter=adapter.replace(old,"std::mem::size_of::<G2PrefixRecord>()"); }
    fs::write(r("caller_probe.rs"),&adapter).unwrap();
    let core_suffix="\n#[cfg(zopro_gauss_caller_probe)]pub use msm::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    let msm_adapter_suffix="\n#[cfg(zopro_gauss_caller_probe)]mod caller_probe;\n#[cfg(zopro_gauss_caller_probe)]pub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    let parents=BTreeMap::from([
        ("crates/zopro-core/src/lib.rs",core_suffix.to_string()),
        ("crates/zopro-core/src/msm.rs",format!("{msm_adapter_suffix}\n#[cfg(zopro_g2_cached_denominator)]type G2PrefixRecord=[FqBatch;4];\n#[cfg(not(zopro_g2_cached_denominator))]type G2PrefixRecord=[FqBatch;2];\n")),
        ("crates/zopro-core/src/msm/g2_borrowed.rs",fs::read_to_string(r("cache-test.rs.in")).unwrap())
    ]);
    let borrowed="crates/zopro-core/src/msm/g2_borrowed.rs";
    let mut edits=vec![json!({"path":"crates/zopro-core/src/msm.rs","old":"prefix_records:Vec<[FqBatch;2]>","new":"prefix_records:Vec<G2PrefixRecord>"})];
    let push="        s.prefix_records.push([chains, norm]);";
    edits.push(json!({"path":borrowed,"old":push,"new":format!("        #[cfg(zopro_g2_cached_denominator)]s.prefix_records.push([chains,norm,denominator[0],denominator[1]]);\n        #[cfg(not(zopro_g2_cached_denominator))]\n{push}")}));
    let rev_bx="        let ay = fq2_pack(a.map(|p| p.y));\n        let bx = fq2_pack(b.map(|p| p.x));";
    edits.push(json!({"path":borrowed,"old":rev_bx,"new":"        let ay = fq2_pack(a.map(|p| p.y));\n        #[cfg(zopro_g2_cached_denominator)]let bx=if FAST{[FqBatch::zero();2]}else{fq2_pack(b.map(|p|p.x))};\n        #[cfg(not(zopro_g2_cached_denominator))]\n        let bx = fq2_pack(b.map(|p| p.x));"}));
    let den="        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));";
    edits.push(json!({"path":borrowed,"old":den,"new":format!("        #[cfg(zopro_g2_cached_denominator)]let denominator=if FAST{{[s.prefix_records[j][2],s.prefix_records[j][3]]}}else{{core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]))}};\n        #[cfg(not(zopro_g2_cached_denominator))]\n{den}")}));
    let x="        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &Fq2::lanes_add(&ax, &bx));";
    edits.push(json!({"path":borrowed,"old":x,"new":format!("        #[cfg(zopro_g2_cached_denominator)]let x={{let sum=if FAST{{Fq2::lanes_add(&Fq2::lanes_add(&ax,&ax),&denominator)}}else{{Fq2::lanes_add(&ax,&bx)}};Fq2::lanes_sub(&Fq2::lanes_square(&slope),&sum)}};\n        #[cfg(not(zopro_g2_cached_denominator))]\n{x}")}));
    let owned="        s.prefix_records.push([chains,norm]);chains=chains.mul(&norm);";
    edits.push(json!({"path":"crates/zopro-core/src/msm/g2_prefix.rs","old":owned,"new":format!("        #[cfg(zopro_g2_cached_denominator)]s.prefix_records.push([chains,norm,FqBatch::zero(),FqBatch::zero()]);\n        #[cfg(not(zopro_g2_cached_denominator))]s.prefix_records.push([chains,norm]);\n        chains=chains.mul(&norm);")}));
    let mut pins=BTreeMap::new();let mut exact=0;
    for (p,h) in original {
        assert_eq!(hash(Path::new(Z).join(p)),h.as_str().unwrap());
        let mut bytes=fs::read(Path::new(Z).join(p)).unwrap();
        let local=edits.iter().filter(|e|e["path"]==*p).collect::<Vec<_>>();
        if !local.is_empty(){let mut text=String::from_utf8(bytes).unwrap();for e in local {let old=e["old"].as_str().unwrap();assert_eq!(text.matches(old).count(),1,"{p}:{old}");text=text.replace(old,e["new"].as_str().unwrap());}bytes=text.into_bytes();}
        if let Some(suffix)=parents.get(p.as_str()){bytes.extend_from_slice(suffix.as_bytes());}
        if format!("{:x}",Sha256::digest(&bytes))==h.as_str().unwrap(){exact+=1;}
        let to=r("source").join(p);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::write(&to,bytes).unwrap();pins.insert(to.display().to_string(),hash(to));
    }
    assert_eq!(exact,172);
    let child=r("source/crates/zopro-core/src/msm/caller_probe.rs");fs::copy(r("caller_probe.rs"),&child).unwrap();pins.insert(child.display().to_string(),hash(child));assert_eq!(pins.len(),177);
    for (p,h) in original {
        let mut bytes=fs::read(r("source").join(p)).unwrap();
        if let Some(suffix)=parents.get(p.as_str()){assert!(bytes.ends_with(suffix.as_bytes()));bytes.truncate(bytes.len()-suffix.len());}
        let local=edits.iter().filter(|e|e["path"]==*p).collect::<Vec<_>>();
        if !local.is_empty(){let mut text=String::from_utf8(bytes).unwrap();for e in local.into_iter().rev(){let new=e["new"].as_str().unwrap();assert_eq!(text.matches(new).count(),1);text=text.replace(new,e["old"].as_str().unwrap());}bytes=text.into_bytes();}
        assert_eq!(format!("{:x}",Sha256::digest(bytes)),h.as_str().unwrap(),"recover {p}");
        let from=previous.join("frozen-source").join(p);let mut bytes=fs::read(&from).unwrap();
        let old_suffix=if p=="crates/zopro-core/src/lib.rs"{Some(core_suffix)}else if p=="crates/zopro-core/src/msm.rs"{Some(msm_adapter_suffix)}else{None};
        if let Some(suffix)=old_suffix{assert!(bytes.ends_with(suffix.as_bytes()));bytes.truncate(bytes.len()-suffix.len());}
        assert_eq!(format!("{:x}",Sha256::digest(bytes)),h.as_str().unwrap());pins.insert(from.display().to_string(),hash(from));
    }
    let flags=format!("{} --cfg zopro_gauss_caller_probe",m["rustflags"].as_str().unwrap());fs::write(r("native-flags.txt"),&flags).unwrap();
    for name in ["artifacts-frozen.json","probe-frozen.json"] {
        let v=read(previous.join(name));let f=if name=="artifacts-frozen.json"{&v["rustflags"]}else{&v["native_flags"]};assert_eq!(*f,json!(flags));
        for (p,h) in v["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.clone(),h.as_str().unwrap().into());}
        if name=="probe-frozen.json"{assert_eq!(hash(v["binary"].as_str().unwrap()),v["sha256"].as_str().unwrap());}save(r(name),&v);
    }
    let toolchain="/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin";let mut steps=Vec::new();
    for label in ["off","on"] {
        let env=json!({"RUSTC":format!("{toolchain}/rustc"),"RUSTUP_TOOLCHAIN":"1.98.1-x86_64-unknown-linux-gnu","RUSTFLAGS":format!("{flags}{}",if label=="on"{" --cfg zopro_g2_cached_denominator"}else{""}),"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":[format!("{toolchain}/cargo"),"test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":[format!("{toolchain}/cargo"),"build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
    }
    save(r("edits.json"),&json!({"parents":parents,"replacements":edits,"adapter_original":previous.join("caller_probe.rs"),"adapter_capacity_type":"G2PrefixRecord","original_OFF_expressions_verbatim":true}));
    save(r("source-audit.json"),&json!({"originals":176,"snapshot":177,"originals_exact":172,"all176recover":true,"all17field_sources_exact":true,"prefix_fields_off":2,"prefix_fields_on":4,"no_new_heap_vector":true,"production_changed":false}));
    for name in ["setup.rs","native.rs","native-helper","design.md","history.md","cache-test.rs.in","edits.json","source-audit.json","caller_probe.rs","probe.rs","perf.rs","native-flags.txt","artifacts-frozen.json","probe-frozen.json"] {pins.insert(r(name).display().to_string(),hash(r(name)));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared177files/172originalexact/all176recover; field unchanged; four-vector denominator record vs six-vector X cache;6native steps; no timing");
}

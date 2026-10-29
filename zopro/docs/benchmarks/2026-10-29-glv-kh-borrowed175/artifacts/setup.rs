fn setup(){
    assert!(!r("build-plan.json").exists());let m=read(MANIFEST);let mut pins=BTreeMap::new();let mut edits=BTreeMap::new();
    for(file,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(file);assert_eq!(hash(&from),h.as_str().unwrap());let to=r("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
    let mut edit=|file:&str,changes:Vec<(String,String)>|{
        let p=r("source").join(file);let old=fs::read_to_string(&p).unwrap();let mut new=old.clone();
        for(before,after)in &changes{assert_eq!(new.matches(before).count(),1,"{file}: {before}");new=new.replacen(before,after,1);}
        let mut inverse=new.clone();for(before,after)in changes.iter().rev(){inverse=inverse.replacen(after,before,1);}assert_eq!(inverse,old);fs::write(p,new).unwrap();edits.insert(file.to_string(),json!(changes));
    };
    let guard="all(zopro_glv_fused_kh, zopro_resource_schedule, zopro_g2_horner_buckets, feature=\"ifma8\", target_arch=\"x86_64\", target_feature=\"avx512ifma\", target_feature=\"avx512vl\", target_feature=\"avx512dq\", not(no_ifma))";
    edit("crates/zopro-core/src/msm_affine/borrowed_pending.rs",vec![
        ("mod balanced;".into(),format!("mod balanced;\n#[cfg({guard})]\nmod glv_kh;\n#[cfg({guard})]\npub(super) fn glv_fused(k:&[zopro_bn254::G1Affine],sk:&[[u64;4]],h:&[zopro_bn254::G1Affine],sh:&[[u64;4]])->Option<zopro_bn254::G1Projective>{{glv_kh::try_fused(k,sk,h,sh)}}")),
    ]);
    let old=" if k.len().min(h.len())>=1024 && super::borrowed_pending::enabled() {return super::borrowed_pending::fused(k,sk,h,sh,bits);}";
    let new=format!(" if k.len().min(h.len())>=1024 && super::borrowed_pending::enabled() {{\n  #[cfg({guard})]\n  if bits==13 {{if let Some(value)=super::borrowed_pending::glv_fused(k,sk,h,sh){{return value;}}}}\n  return super::borrowed_pending::fused(k,sk,h,sh,bits);\n }}");
    edit("crates/zopro-core/src/msm_affine/fused_kh.rs",vec![(old.into(),new)]);
    edit("crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs",vec![
        ("struct Span {\n    start: usize,\n    width: usize,\n}".into(),"pub(super) struct Span {\n    pub(super) start: usize,\n    pub(super) width: usize,\n}".into()),
        ("fn digit_at(s: &[u64; 4], span: Span) -> i16 {".into(),"pub(super) fn digit_at(s: &[u64; 4], span: Span) -> i16 {".into())
    ]);
    let costs=r#"#[cfg(zopro_glv_fused_kh)]
pub(crate) fn glv_kh_peak(k:usize,h:usize,workers:usize)->Option<usize> {
    let n=add(k,h)?;
    let profile=shape::<G1Config>(1<<12,10,2,batch_size(13).max(32))?;
    add(vector::<[u64;4]>(n,false)?,add(profile.held,mul(profile.leaf,workers.max(1))?)?)
}

"#;
    edit("crates/zopro-core/src/msm_affine/workspace.rs",vec![("#[cfg(test)]\nmod tests {".into(),format!("{costs}#[cfg(test)]\nmod tests {{"))]);
    let child="crates/zopro-core/src/msm_affine/borrowed_pending/glv_kh.rs";fs::copy(r("glv_kh.rs"),r("source").join(child)).unwrap();
    let mut exact=0;let mut field=0;for(file,h)in m["source_files_sha256"].as_object().unwrap(){let p=r("source").join(file);if hash(&p)==h.as_str().unwrap(){exact+=1;}else{assert!(edits.contains_key(file));}if file.starts_with("crates/zopro-bn254/"){field+=1;assert_eq!(hash(&p),h.as_str().unwrap());}pin(&mut pins,p);}
    assert_eq!(exact,171);assert_eq!(field,17);pin(&mut pins,r("source").join(child));
    let old="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-22-universal-g1-bucket-horner/README.md";pin(&mut pins,old);
    save(r("source-audit.json"),&json!({"originals":175,"originals_exact":exact,"private_files":176,"modified_originals":edits.keys().collect::<Vec<_>>(),"edits":edits,"new_child":child,"field_originals_exact":17,"accepted_phi_split_source_exact":true,"production_changed":false,"historical_GLVMsm_negative_retained":old,"new_mechanism":"signed decomposition + borrowed fused K/H + ten balanced windows/two partitions + complete extra reservation"}));
    let mut steps=vec![];for(label,on)in[("control",false),("candidate",true)]{
        let flags=if on{format!("{} --cfg zopro_glv_fused_kh",m["rustflags"].as_str().unwrap())}else{m["rustflags"].as_str().unwrap().to_string()};fs::write(r(format!("native-flags-{label}.txt")),&flags).unwrap();
        let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(flags);env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
        for(name,args)in[
            ("core",vec!["cargo","test","--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"]),
            ("prover",vec!["cargo","test","--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"]),
            ("libraries",vec!["cargo","build","--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"]),
        ] {let mut args:Vec<String>=args.into_iter().map(String::from).collect();args.splice(2..2,["--manifest-path".into(),r("source/Cargo.toml").display().to_string()]);steps.push(json!({"kind":"command","name":format!("{name}-{label}"),"argv":args,"environment":env,"unfiltered":name!="libraries","timeout_seconds":1800}));}
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    for file in["setup.rs","native.rs","native-helper","glv_kh.rs","design.md","source-audit.json","native-flags-control.txt","native-flags-candidate.txt"]{pin(&mut pins,r(file));}pin(&mut pins,MANIFEST);
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared10steps176privatefiles172originalsexact; native borrowedGLV fusedK/H");
}

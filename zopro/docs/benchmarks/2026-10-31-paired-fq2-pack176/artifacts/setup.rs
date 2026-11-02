fn setup() {
    let m=read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let original=m["source_files_sha256"].as_object().unwrap();assert_eq!(original.len(),176);
    assert!(!r("source").exists());
    let old=Path::new("/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175");
    for f in["caller_probe.rs","probe.rs","perf.rs"] {fs::copy(old.join(f),r(f)).unwrap();}
    let mut parents=BTreeMap::new();
    parents.insert("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_gauss_caller_probe)]mod caller_probe;\n#[cfg(zopro_gauss_caller_probe)]pub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n".to_owned());
    parents.insert("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_gauss_caller_probe)]pub use msm::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n".to_owned());
    let guard="all(zopro_fq2_pair_pack,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let field_guard="all(any(test,zopro_fq2_pair_pack),target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let field_suffix=format!("\n#[cfg({field_guard})]mod paired_fq2_input;\n");
    let bridge=format!("\n#[cfg({guard})]impl FqBatch{{#[inline(always)]pub(crate)fn from_fq2_refs(values:[&zopro_bn254::Fq2;8])->[Self;2]{{zopro_bn254::ifma8::Ifma8::from_fq2_refs(values).map(Self)}}}}\n");
    let mut edits=BTreeMap::new();let mut all_pins=BTreeMap::new();
    for label in["frozen-source","source"] {
        for(p,h)in original {
            let from=Path::new(Z).join(p);assert_eq!(hash(&from),h.as_str().unwrap());let mut bytes=fs::read(from).unwrap();
            if let Some(suffix)=parents.get(p.as_str()){bytes.extend_from_slice(suffix.as_bytes());}
            if label=="source" {
                if p=="crates/zopro-bn254/src/lib.rs"{bytes.extend_from_slice(field_suffix.as_bytes());}
                if p=="crates/zopro-core/src/simd.rs"{bytes.extend_from_slice(bridge.as_bytes());}
                if p=="crates/zopro-core/src/msm/g2_borrowed.rs" {
                    let before=String::from_utf8(bytes).unwrap();let mut after=before.clone();
                    let begin=after.find("    let full =").unwrap();
                    after.insert_str(begin,"    #[cfg(zopro_fq2_pair_pack)]return super::paired_g2_borrowed::apply::<C,FAST>(buckets,batch,s);\n");
                    edits.insert(p.clone(),json!({"before":before,"after":after}));bytes=after.into_bytes();
                }
            }
            let to=r(label).join(p);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::write(&to,&bytes).unwrap();all_pins.insert(to.display().to_string(),hash(to));
        }
        let p=r(label).join("crates/zopro-core/src/msm/caller_probe.rs");fs::copy(r("caller_probe.rs"),&p).unwrap();all_pins.insert(p.display().to_string(),hash(p));
    }
    let p=r("source/crates/zopro-bn254/src/paired_fq2_input.rs");fs::copy(r("field.rs"),&p).unwrap();all_pins.insert(p.display().to_string(),hash(p));
    let original_body=fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm/g2_borrowed.rs")).unwrap();
    let mut body=original_body[..original_body.find("\nimpl<C: CurveConfig<Base = Fq2>> BatchAdd<C>").unwrap()].to_owned();
    let replacements=[
        ("let ax = fq2_pack(core::array::from_fn(|i| {\n            buckets[(batch[k + i].0 & 0x7fffffff) as usize].x\n        }));","let ax = FqBatch::from_fq2_refs(core::array::from_fn(|i| &buckets[(batch[k+i].0&0x7fffffff)as usize].x));"),
        ("let bx = fq2_pack(core::array::from_fn(|i| batch[k + i].1.x));","let bx = FqBatch::from_fq2_refs(core::array::from_fn(|i| &batch[k+i].1.x));"),
        ("let a: [Affine<C>; LANES] =\n            core::array::from_fn(|i| buckets[(batch[k + i].0 & 0x7fffffff) as usize]);","let a: [&Affine<C>; LANES] = core::array::from_fn(|i| &buckets[(batch[k+i].0&0x7fffffff)as usize]);"),
        ("fq2_pack(a.map(|p| p.x))","FqBatch::from_fq2_refs(a.map(|p| &p.x))"),
        ("fq2_pack(a.map(|p| p.y))","FqBatch::from_fq2_refs(a.map(|p| &p.y))"),
        ("fq2_pack(b.map(|p| p.x))","FqBatch::from_fq2_refs(b.map(|p| &p.x))"),
        ("fq2_pack(b.map(|p|p.y))","FqBatch::from_fq2_refs(b.map(|p|&p.y))"),
    ];
    for(a,b)in replacements{assert_eq!(body.matches(a).count(),1,"missing {a}");body=body.replace(a,b);}
    body=body.replace("//! Signed borrowed G2 prefix, retaining accepted arithmetic and scratch.","//! Original signed borrowed G2 prefix with paired reference constructors.");
    let p=r("source/crates/zopro-core/src/msm/paired_g2_borrowed.rs");fs::write(&p,&body).unwrap();all_pins.insert(p.display().to_string(),hash(p));
    let p=r("source/crates/zopro-core/src/msm.rs");let mut text=fs::read_to_string(&p).unwrap();let paired_suffix=format!("\n#[cfg({guard})]mod paired_g2_borrowed;\n");text.push_str(&paired_suffix);fs::write(&p,text).unwrap();all_pins.insert(p.display().to_string(),hash(p));
    save(r("edits.json"),&json!({"parents":parents,"field_suffix":field_suffix,"bridge":bridge,"paired_suffix":paired_suffix,"replacements":replacements,"inline_edits":edits}));
    // Verify all original bytes recover, including binary fixtures.
    let mut exact=BTreeMap::new();for label in["frozen-source","source"]{let mut same=0;for(p,h)in original{
        let mut bytes=fs::read(r(label).join(p)).unwrap();if format!("{:x}",Sha256::digest(&bytes))==h.as_str().unwrap(){same+=1;continue;}
        let mut text=String::from_utf8(bytes).unwrap();
        if label=="source"{if p=="crates/zopro-bn254/src/lib.rs"{text=text.strip_suffix(&field_suffix).unwrap().to_owned();}if p=="crates/zopro-core/src/simd.rs"{text=text.strip_suffix(&bridge).unwrap().to_owned();}if p=="crates/zopro-core/src/msm.rs"{text=text.strip_suffix(&paired_suffix).unwrap().to_owned();}if let Some(e)=edits.get(p){assert_eq!(text,e["after"]);text=e["before"].as_str().unwrap().to_owned();}}
        if let Some(suffix)=parents.get(p.as_str()){text=text.strip_suffix(suffix).unwrap().to_owned();}bytes=text.into_bytes();assert_eq!(format!("{:x}",Sha256::digest(bytes)),h.as_str().unwrap());
    }exact.insert(label,same);}
    assert_eq!(exact["frozen-source"],174);assert_eq!(exact["source"],171);
    let flags=format!("{} --cfg zopro_gauss_caller_probe",m["rustflags"].as_str().unwrap());fs::write(r("native-flags.txt"),&flags).unwrap();
    let mut steps=vec![];
    for label in["frozen","off","on"] {
        let mut e=read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json");let full=format!("{flags}{}",if label=="on"{" --cfg zopro_fq2_pair_pack"}else{""});e["RUSTFLAGS"]=json!(full);e["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));e["CARGO_BUILD_JOBS"]=json!("16");e["RAYON_NUM_THREADS"]=json!("4");let mf=r(if label=="frozen"{"frozen-source/Cargo.toml"}else{"source/Cargo.toml"});
        for(pkg,suite)in[("zopro-bn254","field"),("zopro-core","core")] {if label=="frozen"&&suite=="field"{continue;}steps.push(json!({"kind":"command","name":format!("{suite}-{label}"),"argv":["cargo","test","--manifest-path",mf,"--locked","--offline","--release","-p",pkg,"--lib","--","--test-threads=1"],"environment":e,"unfiltered":true,"timeout_seconds":1800}));}
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",mf,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":e,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":e,"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("link-{label}"),"argv":[r("native-helper"),"link",label],"timeout_seconds":600}));
    }
    save(r("source-audit.json"),&json!({"originals":176,"frozen_snapshot_files":177,"candidate_snapshot_files":179,"originals_exact":exact,"all_original_bytes_recovered":true,"field_arithmetic_and_generator_exact":true,"new_heap_buffers":0,"production_changed":false}));
    for f in["native.rs","native-helper","setup.rs","field.rs","caller_probe.rs","probe.rs","perf.rs","design.md","bootstrap.rs","source-audit.json","native-flags.txt","edits.json"] {all_pins.insert(r(f).display().to_string(),hash(r(f)));}
    save(r("build-pins.json"),&json!(all_pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));
    println!("Prepared14steps, all176originalsrecovered inbothsnapshots;fieldmath/generatorexact;pairconstructor/signedG2only");
}

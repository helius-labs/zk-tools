const OLD_SQUARE:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-24-integrated-square-component/source/crates/zopro-bn254/src/ifma.rs";
fn setup() {
    assert!(!r("build-plan.json").exists());
    let m=read(MANIFEST);let mut pins=BTreeMap::new();
    for(f,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(f);assert_eq!(hash(&from),h.as_str().unwrap());let to=r("source").join(f);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(&from,&to).unwrap();pin(&mut pins,from);pin(&mut pins,to);}
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),174);
    let old=fs::read_to_string(OLD_SQUARE).unwrap();
    let start=old.find("    pub fn square(&self) -> Self {").unwrap();
    let end=start+old[start..].find("    #[inline(always)]\n    pub fn mul").unwrap();
    let code=old[start..end].replace("    pub fn square(&self) -> Self {","    pub fn sum_squares(&self, other: &Self) -> Self {")
        .replace("        if !cfg!(zopro_integrated_square) { return self.mul(self); }\n","")
        .replace("_mm256_","_mm512_").replace("_si256","_si512")
        .replace("split(C::P)","split(FqConfig::P)").replace("C::INV","FqConfig::INV")
        .replace("            let a = self.0;","            let a = self.0;\n            let b = other.0;")
        .replace("if MIXED { cancel!(48); } else { cancel!(52); }","cancel!(48);")
        .replace("            // REDC(a^2) < p + 4p^2/R < 2p for both supported radices.\n            Self([t[0], t[1], t[2], t[3], t[4]], PhantomData)","            Self::reduce_fused([t[0], t[1], t[2], t[3], t[4]])");
    let mut generated=String::from("//! Fused symmetric denominator norm, derived from the archived square schedule.\nuse super::*;\nuse crate::FqConfig;\nimpl Ifma8<FqConfig, true> {\n    #[inline(always)]\n");
    let mut duplicated=0;
    for line in code.lines(){generated.push_str(line);generated.push('\n');if line.contains("_mm512_madd52")&&line.contains("a["){generated.push_str(&line.replace("a[","b["));generated.push('\n');duplicated+=1;}}
    assert_eq!(duplicated,30);assert!(!generated.contains("C::")&&!generated.contains("MIXED")&&!generated.contains("zopro_integrated_square"));
    generated.push_str("}\n");generated.push_str(&fs::read_to_string(r("norm-tests.rs")).unwrap());
    fs::create_dir_all(r("source/crates/zopro-bn254/src/ifma8")).unwrap();
    let module=r("source/crates/zopro-bn254/src/ifma8/symmetric_norm.rs");fs::write(&module,generated).unwrap();pin(&mut pins,&module);
    let mut edits=vec![];
    let field="crates/zopro-bn254/src/ifma8.rs";
    let suffix="\n#[cfg(zopro_symmetric_norm)]\nmod symmetric_norm;\n";
    let p=r("source").join(field);let before=fs::read_to_string(&p).unwrap();fs::write(&p,format!("{before}{suffix}")).unwrap();pin(&mut pins,&p);edits.push(json!({"file":field,"suffix":suffix}));
    let wrapper="crates/zopro-core/src/simd.rs";
    let suffix="\n#[cfg(all(zopro_symmetric_norm,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nimpl FqBatch { #[inline(always)] pub(crate) fn sum_squares(&self, b:&Self)->Self {Self(self.0.sum_squares(&b.0))} }\n";
    let p=r("source").join(wrapper);let before=fs::read_to_string(&p).unwrap();fs::write(&p,format!("{before}{suffix}")).unwrap();pin(&mut pins,&p);edits.push(json!({"file":wrapper,"suffix":suffix}));
    let core="crates/zopro-core/src/msm.rs";let p=r("source").join(core);let before=fs::read_to_string(&p).unwrap();
    let needle="            value[0].sum2(&value[0], &value[1], &value[1])";
    let replacement="            #[cfg(all(zopro_symmetric_norm,feature=\"ifma8\"))] return value[0].sum_squares(&value[1]); value[0].sum2(&value[0], &value[1], &value[1])";
    assert_eq!(before.matches(needle).count(),1);let after=before.replace(needle,replacement);assert_eq!(after.replace(replacement,needle),before);fs::write(&p,after).unwrap();pin(&mut pins,&p);edits.push(json!({"file":core,"before":needle,"after":replacement}));
    let mut exact=0;for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=r("source").join(f);let mut bytes=fs::read(&p).unwrap();if hash(&p)==h.as_str().unwrap(){exact+=1}else{let edit=edits.iter().find(|x|x["file"]==*f).unwrap();let text=String::from_utf8(bytes).unwrap();let reversed=if let Some(s)=edit["suffix"].as_str(){text.strip_suffix(s).unwrap().to_string()}else{text.replace(edit["after"].as_str().unwrap(),edit["before"].as_str().unwrap())};bytes=reversed.into_bytes();assert_eq!(format!("{:x}",Sha256::digest(&bytes)),h.as_str().unwrap());}}
    assert_eq!(exact,171);save(r("source-audit.json"),&json!({"original_files":174,"candidate_files":175,"originals_exact":171,"reverse174_exact":true,"edits":edits,"new_module":"crates/zopro-bn254/src/ifma8/symmetric_norm.rs","source_square_template":OLD_SQUARE,"duplicated_product_IFMA_lines":duplicated,"OFF_expression_retained_verbatim":true,"forecast_key_solver_G1_unchanged":true,"production_changed":false}));
    let flags=m["rustflags"].as_str().unwrap();fs::write(r("native-flags.txt"),flags).unwrap();let cargo=r("source/Cargo.toml");let mut steps=vec![];
    for label in ["control","candidate"]{let flag=if label=="control"{flags.to_string()}else{format!("{flags} --cfg zopro_symmetric_norm")};
        let env=json!({"RUSTFLAGS":flag,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_DEDUP":"1","ZOPRO_CPU_ZERO_AWARE_GROUPS":"1","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        for package in ["zopro-bn254","zopro-core","zopro-prover"]{let mut argv=vec!["cargo".into(),"test".into(),"--manifest-path".into(),cargo.display().to_string(),"--locked".into(),"--offline".into(),"--release".into(),"-p".into(),package.into(),"--lib".into()];if package=="zopro-prover"{argv.extend(["--test".into(),"cubic".into()]);}argv.extend(["--".into(),"--test-threads=1".into()]);let required=if label=="candidate"&&package=="zopro-bn254"{vec!["ifma8::symmetric_norm::tests::symmetric_norm_lazy_edges_and_full_width_arkworks","ifma8::symmetric_norm::tests::symmetric_norm_dependent_lazy_chains_match_arkworks"]}else{vec![]};steps.push(json!({"kind":"command","name":format!("{package}-{label}"),"argv":argv,"environment":env,"unfiltered":true,"required_tests":required,"timeout_seconds":1800}));}
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    assert_eq!(steps.len(),12);for f in ["native.rs","native-helper","setup.rs","norm-tests.rs","design.md","source-audit.json","native-flags.txt"]{pin(&mut pins,r(f));}pin(&mut pins,MANIFEST);pin(&mut pins,OLD_SQUARE);save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared12 native steps;171exact/reverse174/candidate175; fused symmetricnorm only");
}

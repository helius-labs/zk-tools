fn setup() {
    assert!(!r("build-plan.json").exists());
    let m = read(MANIFEST);
    let mut pins = BTreeMap::new();
    let files = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 174);
    for (f, h) in files {
        let from = Path::new(Z).join(f);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let to = r("source").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        pin(&mut pins, from);
        pin(&mut pins, to);
    }
    let original = fs::read_to_string(r("source/crates/zopro-bn254/src/ifma8.rs")).unwrap();
    let start = original
        .find(
            "    fn product_cios<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {",
        )
        .unwrap();
    let end = start
        + original[start..]
            .find("    /// Inputs must each be below p.")
            .unwrap();
    let function = &original[start..end];
    let split = function.rfind("            for j in 0..4 {").unwrap();
    let prefix=function[..split].replace("    fn product_cios<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {","    #[inline(always)]\n    pub fn mul_sub_final(&self, b:&Self, subtrahend:&Self)->Self {\n        const FUSED:bool=false;let c=self;let d=b;");
    assert!(prefix.ends_with("            }\n"));
    let generated=format!("//! Post-REDC signed final-carry subtraction. Private experiment.\nuse super::*;\nimpl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED> {{\n{prefix}            Self::finish_sub(t,subtrahend)\n        }}\n    }}\n{}\n}}\n{}",fs::read_to_string(r("finalize.rs")).unwrap(),fs::read_to_string(r("field-tests.rs")).unwrap());
    let field_module = r("source/crates/zopro-bn254/src/ifma8/signed_final.rs");
    fs::create_dir_all(field_module.parent().unwrap()).unwrap();
    fs::write(&field_module, &generated).unwrap();
    pin(&mut pins, &field_module);
    let caller_module = r("source/crates/zopro-core/src/msm/caller_probe.rs");
    fs::copy(r("caller_probe.rs"), &caller_module).unwrap();
    pin(&mut pins, &caller_module);
    let suffixes=[
        ("crates/zopro-bn254/src/ifma8.rs","\n#[cfg(zopro_signed_final_sub)]\nmod signed_final;\n"),
        ("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_finish_caller)]\npub mod caller_probe;\n"),
        ("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_finish_caller)]\npub use msm::caller_probe::{Caller,CallerG1,CallerG2};\n"),
        ("crates/zopro-core/src/simd.rs","\nimpl FqBatch {\n #[inline(always)] pub(crate) fn g1_mul_sub(&self,b:&Self,c:&Self)->Self {\n  #[cfg(all(zopro_signed_final_sub,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))] return Self(self.0.mul_sub_final(&b.0,&c.0));\n  #[allow(unreachable_code)] self.mul(b).sub(c)\n }\n}\n")];
    for (f, suffix) in suffixes {
        let p = r("source").join(f);
        let s = fs::read_to_string(&p).unwrap();
        fs::write(&p, format!("{s}{suffix}")).unwrap();
        pin(&mut pins, &p);
    }
    let replacements = [
        (
            "        let x = slope.mul(&slope).sub(&g.xsum);",
            "        let x = slope.g1_mul_sub(&slope,&g.xsum);",
        ),
        (
            "        let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);",
            "        let y = slope.g1_mul_sub(&g.ax.sub(&x),&g.ay);",
        ),
    ];
    let p = r("source/crates/zopro-core/src/msm.rs");
    let mut text = fs::read_to_string(&p).unwrap();
    for (before, after) in replacements {
        assert_eq!(text.matches(before).count(), 1);
        text = text.replace(before, after);
    }
    fs::write(&p, text).unwrap();
    pin(&mut pins, &p);
    let mut exact = 0;
    for (f, h) in files {
        let p = r("source").join(f);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        } else {
            let mut text = fs::read_to_string(&p).unwrap();
            for (ff, suffix) in suffixes {
                if ff == f {
                    text = text.strip_suffix(suffix).unwrap().to_string();
                }
            }
            if f == "crates/zopro-core/src/msm.rs" {
                for (before, after) in replacements {
                    text = text.replace(after, before);
                }
            }
            assert_eq!(
                format!("{:x}", Sha256::digest(text.as_bytes())),
                h.as_str().unwrap()
            );
        }
    }
    assert_eq!(exact, 170);
    save(
        r("source-audit.json"),
        &json!({"original_files":174,"private_files":176,"originals_exact":170,"reverse174_exact":true,"suffixes":suffixes,"msm_replacements":replacements,
        "new_modules":["crates/zopro-bn254/src/ifma8/signed_final.rs","crates/zopro-core/src/msm/caller_probe.rs"],"original_CIOS_body_unchanged":true,
        "new_method_prefix_derived_from_original_CIOS":true,"field_generation_parent_unchanged":true,"new_heap_storage":false,"production_changed":false,
        "caller_adapter_byte_exact_to":"/home/ubuntu/src/zopro-perf-work/inverse-finish-callers/caller_probe.rs"}),
    );
    let history = [
        "docs/benchmarks/2026-10-22-universal-mul-sub-sdk/README.md",
        "docs/benchmarks/2026-10-22-universal-mul-sub-sdk/source/crates/zopro-bn254/src/ifma8.rs",
        "docs/benchmarks/2026-10-24-canonical-y-output/README.md",
    ];
    let mut history_pins = BTreeMap::new();
    for f in history {
        pin(&mut history_pins, Path::new(Z).join(f));
        pin(&mut pins, Path::new(Z).join(f));
    }
    save(
        r("history-audit.json"),
        &json!({"generic_fusion_is_NOT_new":true,"22Oct":"normalized delta injection before final cancellation; failed original3percent SDK gate","24Oct":"canonical Y subtraction after normalized multiplication; failed SDK gate","current_distinction":"no injected delta; subtraction directly in post-REDC coefficient carry; old multiply body retained","history_pins":history_pins,"novelty_claim":false}),
    );
    let flags = format!(
        "{} --cfg zopro_finish_caller",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    let cargo = r("source/Cargo.toml");
    let mut steps = Vec::new();
    for label in ["control", "candidate"] {
        let flag = if label == "control" {
            flags.clone()
        } else {
            format!("{flags} --cfg zopro_signed_final_sub")
        };
        let env = json!({"RUSTFLAGS":flag,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"1","ZOPRO_CPU_DEDUP":"1","ZOPRO_CPU_ZERO_AWARE_GROUPS":"1","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        for package in ["zopro-bn254", "zopro-core"] {
            let required = if package == "zopro-bn254" && label == "candidate" {
                vec!["ifma8::signed_final::signed_final_tests::signed_coefficients_match_separate_normalization_exactly","ifma8::signed_final::signed_final_tests::complete_mul_sub_lazy_edges_chains_match_arkworks"]
            } else if package == "zopro-core" {
                vec![
                    "msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse",
                    "msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse",
                    "msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse",
                ]
            } else {
                vec![]
            };
            steps.push(json!({"kind":"command","name":format!("{package}-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p",package,"--lib","--","--test-threads=1","--nocapture"],"environment":env,"unfiltered":true,"required_tests":required,"timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":r("probe.rs"),"additional_sources":[r("perf.rs")],"timeout_seconds":600}));
    }
    for f in [
        "native.rs",
        "native-helper",
        "setup.rs",
        "finalize.rs",
        "field-tests.rs",
        "caller_probe.rs",
        "probe.rs",
        "perf.rs",
        "design.md",
        "bounds.md",
        "history-audit.json",
        "source-audit.json",
        "native-flags.txt",
    ] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, MANIFEST);
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared10 native steps;176privatefiles/170exact/reverse174;no production edit");
}

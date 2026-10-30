fn setup() {
    assert!(!r("build-plan.json").exists());
    let mp = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&mp);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &mp);
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
    }
    let file = r("source/crates/zopro-core/src/lib.rs");
    let mut s = fs::read_to_string(&file).unwrap();
    let mut edits = vec![];
    let mut edit = |s: &mut String, before: &str, after: String| {
        once(s, before, &after);
        edits.push(json!({"before":before,"after":after}));
    };
    let start="        use msm_affine::{canonical,msm_canonical as msm,msm_grouped,msm_raw,scalar_groups};\n        let solver::Solution";
    edit(&mut s,start,start.replace("\n        let solver::Solution","\n        #[cfg(zopro_query_timeline)] let timeline=query_timeline::Start::new();\n        let solver::Solution"));
    for(before,name)in[("        let solver::Solution{mut w,a,b,c}=plan.solve_in_with_slots(witness,self.domain,&folded.slots,materialize_c)?;","solver_end"),("        let correction=folded.public_term(&w[..plan.n_public]);","correction_end"),("        let words=canonical(&mut w);","canonical_end"),("        let (sa,sb,sk)=(&sa_vec[..],&mut sb_vec[..],&words[plan.n_public..]);","extraction_end"),("        let count=product.len();let (h,z,k)=(&mut product[..],&queries.product[..count],&folded.private[..]);","product_end")]{edit(&mut s,before,format!("{before}\n        #[cfg(zopro_query_timeline)] let {name}=timeline.stamp();"));}
    let before="            let ((ar,kh),bs2)=rayon::join(\n                || rayon::join(|| msm(&self.a[..],sa),\n                    || msm_affine::fused_kh::fused(k,sk,z,canonical(h),13)),\n                || {\n                    if (1024..65536).contains(&self.b2.len()) {\n                        msm_affine::g2_query_exclusive(&self.b2,sb)\n                    } else {\n                        let groups=crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\")\n                            .then(||scalar_groups(sb)).flatten();\n                        groups.as_ref().map_or_else(||msm_raw_b2(&self.b2,sb),\n                            |g|msm_grouped_b2(&self.b2,g))\n                    }\n                });";
    let trace = before
        .replace(
            "let ((ar,kh),bs2)",
            "let (((ar,a_span),(kh,kh_span)),(bs2,b_span))",
        )
        .replace(
            "|| msm(&self.a[..],sa)",
            "|| timeline.measure(|| msm(&self.a[..],sa))",
        )
        .replace(
            "|| msm_affine::fused_kh::fused(k,sk,z,canonical(h),13)",
            "|| timeline.measure(|| msm_affine::fused_kh::fused(k,sk,z,canonical(h),13))",
        )
        .replace(
            "                || {",
            "                || timeline.measure(|| {",
        )
        .replace("                });", "                }));");
    edit(&mut s,before,format!("            #[cfg(not(zopro_query_timeline))]\n{before}\n            #[cfg(zopro_query_timeline)]\n{trace}\n            #[cfg(zopro_query_timeline)] let join_end=timeline.stamp();"));
    let before = "            output_g2(&mut out,bs2);\n            return Ok(out);";
    edit(&mut s,before,before.replace("            return Ok(out);","            #[cfg(zopro_query_timeline)] timeline.finish([solver_end,correction_end,canonical_end,extraction_end,product_end,join_end,timeline.stamp()],[a_span,kh_span,b_span]);\n            return Ok(out);"));
    let suffix = "\n#[cfg(zopro_query_timeline)]\n#[doc(hidden)]\npub mod query_timeline;\n";
    s.push_str(suffix);
    fs::write(&file, &s).unwrap();
    let mut reverse = s.strip_suffix(suffix).unwrap().to_string();
    for e in edits.iter().rev() {
        once(
            &mut reverse,
            e["after"].as_str().unwrap(),
            e["before"].as_str().unwrap(),
        );
    }
    assert_eq!(
        reverse,
        fs::read_to_string(Path::new(Z).join("crates/zopro-core/src/lib.rs")).unwrap()
    );
    fs::copy(
        r("query_timeline.rs"),
        r("source/crates/zopro-core/src/query_timeline.rs"),
    )
    .unwrap();
    let mut exact = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let path = r("source").join(p);
        if hash(&path) == h.as_str().unwrap() {
            exact += 1;
        } else {
            assert_eq!(p, "crates/zopro-core/src/lib.rs");
        }
        pin(&mut pins, path);
    }
    assert_eq!(exact, 174);
    pin(
        &mut pins,
        r("source/crates/zopro-core/src/query_timeline.rs"),
    );
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"originals_exact":exact,"reverse175_exact":true,"edits":edits,"suffix":suffix,"field_files_exact":17,"production_changed":false,"algorithm_changed":false,"new_timeline_payload_bytes":256*13*8}),
    );
    let flags = format!(
        "{} --cfg zopro_query_timeline",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    let old = "/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175";
    let p = read(Path::new(old).join("build-plan.json"));
    let mut steps = vec![];
    for step in p["steps"].as_array().unwrap().iter().take(3) {
        let mut step: Value =
            serde_json::from_str(&serde_json::to_string(step).unwrap().replace(old, R)).unwrap();
        step["environment"]["RUSTFLAGS"] = json!(flags);
        steps.push(step);
    }
    steps.push(json!({"kind":"command","name":"freeze-control","argv":[r("native-helper"),"freeze","control"],"environment":{"RUSTFLAGS":flags},"timeout_seconds":120}));
    for f in [
        "native.rs",
        "native-helper",
        "setup.rs",
        "query_timeline.rs",
        "design.md",
        "source-audit.json",
        "native-flags.txt",
    ] {
        pin(&mut pins, r(f));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared timeline native4steps;176source/174exact/reverse175,17field exact; no timings yet");
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-contained-index176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn function(s: &str, sig: &str) -> String {
    let a = s.find(sig).unwrap();
    let open = a + s[a..].find('{').unwrap();
    let mut end = open + 1;
    let mut depth = 1;
    while depth > 0 {
        match s.as_bytes()[end] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        end += 1;
    }
    s[a..end].into()
}
fn edit(s: &mut String, e: &mut Vec<(String, String)>, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replacen(a, b, 1);
    e.push((a.into(), b.into()));
}
fn audit(s: &str, old: &str, e: &[(String, String)]) {
    let mut v = s.to_owned();
    for (a, b) in e.iter().rev() {
        assert_eq!(v.matches(b).count(), 1);
        v = v.replacen(b, a, 1);
    }
    assert_eq!(v, old);
}
fn main() {
    assert!(!r("source").exists());
    let m=load(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let b = fs::read(Path::new(Z).join(p)).unwrap();
        assert_eq!(hash(&b), h.as_str().unwrap());
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::write(to, b).unwrap();
    }
    let prior=fs::read_to_string(Path::new(Z).join("docs/benchmarks/2026-10-29-g2-packed-index175/artifacts/source/crates/zopro-core/src/msm/packed_index.rs")).unwrap();
    let old = prior[..prior.find("#[cfg(test)]").unwrap()].to_owned();
    let mut packed = old.clone();
    let mut e = vec![];
    edit(
        &mut packed,
        &mut e,
        "pub(super) fn try_reduce<",
        "pub(crate) fn try_reduce<",
    );
    edit(&mut packed,&mut e,"    let _reservation = crate::preparation::Reservation::analysis(peak::<C>(\n        order.len(),\n        buckets,\n        rayon::current_num_threads(),\n    ))?;", "    // Explicit optional budgets retain the original reducer. This route\n    // acquires no additional charge and relies on checked full-layout\n    // containment within the original G2 workspace envelope.\n    if buckets != 128 || order.is_empty() || order.len() > bases.len()\n        || std::env::var_os(\"ZOPRO_CPU_PREPARE_MAX_BYTES\").is_some()\n        || !crate::msm_affine::workspace::contained_indexed_fits::<C>(bases.len())? {\n        return None;\n    }");
    audit(&packed, &old, &e);
    let mut clones = BTreeMap::new();
    clones.insert("old_global_packing",json!({"original":old,"candidate":packed,"edits":e,"original_path":"docs/benchmarks/2026-10-29-g2-packed-index175/artifacts/source/crates/zopro-core/src/msm/packed_index.rs"}));
    let packed_child = format!(
        "{packed}\n{}",
        fs::read_to_string(r("native-tests.rs")).unwrap()
    );
    let phi = fs::read_to_string(r("source/crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
    let msm = fs::read_to_string(r("source/crates/zopro-core/src/msm_affine.rs")).unwrap();
    let original = function(&phi, "fn hb_top_component(");
    let mut top = original.clone();
    let mut e = vec![];
    edit(
        &mut top,
        &mut e,
        "    top: usize,",
        "    top: usize,\n    hits: &AtomicUsize,",
    );
    edit(&mut top,&mut e,"    let mut reduced = crate::msm::reduce_indexed_buckets(bases, &order, &starts);","    let mut reduced = if let Some(value) = crate::msm::contained_index::try_reduce::<G2Config>(bases, &order, &starts) {\n        hits.fetch_add(1, Ordering::Relaxed);\n        value\n    } else { crate::msm::reduce_indexed_buckets(bases, &order, &starts) };");
    audit(&top, &original, &e);
    clones.insert(
        "top",
        json!({"original":original,"candidate":top,"edits":e}),
    );
    let original = function(
        &phi,
        "if w + 1 == 127usize.div_ceil(bits) && 126 - w * bits <= 8 {",
    );
    let mut branch = original.clone();
    let mut e = vec![];
    edit(
        &mut branch,
        &mut e,
        "hb_top_component(bases, short, true, w, bits, top)",
        "hb_top_component(bases, short, true, w, bits, top, hits)",
    );
    edit(
        &mut branch,
        &mut e,
        "hb_top_component(bases, short, false, w, bits, top)",
        "hb_top_component(bases, short, false, w, bits, top, hits)",
    );
    audit(&branch, &original, &e);
    clones.insert(
        "top_branch",
        json!({"original":original,"candidate":branch,"edits":e}),
    );
    let window=format!("fn window(bases:&[G2Affine],short:&[[u64;4]],w:usize,bits:usize,hits:&AtomicUsize)->Vec<G2Affine>{{\n{branch}\nsuper::hb_window(bases,short,w,bits)\n}}\n");
    let original = function(&phi, "fn hb_raw(");
    let mut raw = original.clone();
    let mut e = vec![];
    edit(&mut raw,&mut e,"fn hb_raw(bases: &[G2Affine], short: &mut [[u64; 4]], bits: usize, parts: usize) -> G2Projective {","pub(in crate::msm_affine) fn raw_exclusive(bases: &[G2Affine], short: &mut [[u64; 4]]) -> (G2Projective, usize) {\n    if !(1024..65536).contains(&bases.len()) { return (super::raw_exclusive(bases, short), 0); }\n    let bits = 12;\n    let parts = 2;\n    let hits = AtomicUsize::new(0);");
    edit(
        &mut raw,
        &mut e,
        "hb_window(&bases[start..end], &short[start..end], w, bits)",
        "window(&bases[start..end], &short[start..end], w, bits, &hits)",
    );
    edit(
        &mut raw,
        &mut e,
        "    result\n}",
        "    (result, hits.load(Ordering::Relaxed))\n}",
    );
    audit(&raw, &original, &e);
    clones.insert(
        "raw",
        json!({"original":original,"candidate":raw,"edits":e}),
    );
    let phi_child=format!("//! Original uniform12/top6, only indexed packing admission differs.\nuse super::*;\nuse std::sync::atomic::{{AtomicUsize, Ordering}};\n{top}\n{window}\n{raw}\n");
    let original = function(&msm, "pub(super) fn g2_query_exclusive(");
    let mut query = original.clone();
    let mut e = vec![];
    edit(
        &mut query,
        &mut e,
        "pub(super) fn g2_query_exclusive(",
        "pub(super) fn g2_query_contained_index(",
    );
    edit(
        &mut query,
        &mut e,
        ") -> zopro_bn254::G2Projective {",
        ") -> (zopro_bn254::G2Projective, usize) {",
    );
    edit(
        &mut query,
        &mut e,
        "return crate::msm_grouped_b2(bases, &g);",
        "return (crate::msm_grouped_b2(bases, &g), 0);",
    );
    edit(
        &mut query,
        &mut e,
        "g2_phi::raw_exclusive(&points, &mut g.values)",
        "g2_phi::contained_indexed::raw_exclusive(&points, &mut g.values)",
    );
    edit(
        &mut query,
        &mut e,
        "g2_phi::raw_exclusive(bases, words)",
        "g2_phi::contained_indexed::raw_exclusive(bases, words)",
    );
    edit(&mut query,&mut e,"    match groups {\n        Some(g) => crate::msm_grouped_b2(bases, &g),\n        None => crate::msm_raw_b2(bases, words),\n    }","    (match groups {\n        Some(g) => crate::msm_grouped_b2(bases, &g),\n        None => crate::msm_raw_b2(bases, words),\n    }, 0)");
    audit(&query, &original, &e);
    clones.insert(
        "query",
        json!({"original":original,"candidate":query,"edits":e}),
    );
    let cfg="#[cfg(all(zopro_contained_index, zopro_resource_schedule, zopro_g2_horner_buckets, feature=\"ifma8\", target_arch=\"x86_64\", target_feature=\"avx512ifma\", target_feature=\"avx512vl\", target_feature=\"avx512dq\", not(no_ifma)))]";
    let forecast = r#"
pub(crate) fn contained_indexed_fits<C: CurveConfig>(half: usize) -> Option<bool>
where C::Base: BatchInverse {
    if size_of::<C::Base>() != 64 || !(512..=32768).contains(&half) { return Some(false); }
    // The slice can be either floor(N/2) or ceil(N/2); cover every possible N.
    let minimum_n = half.checked_mul(2)?.checked_sub(1)?.max(1024);
    let maximum_n = half.checked_mul(2)?.checked_add(1)?.min(65535);
    let tile = 2048 / (size_of::<C::Base>() / 32);
    let segments = half.min(add(half.div_ceil(tile), 128)?);
    let mut indexed = top::<C>(half, 7)?;
    // Count all new layout descriptors AGAIN above the old top envelope;
    // the original ten ordinary + one narrow parent release sufficient slack.
    indexed.held = add(indexed.held, add(vector::<(usize,usize)>(segments, false)?,
        vector::<usize>(add(half.div_ceil(tile), 1)?, false)?)?)?;
    let local = mul(vector::<usize>(128.min(tile), false)?, 2)?;
    indexed.leaf = indexed.leaf.max(add(crate::msm::workspace_grouping::<C>(half.min(tile), 1)?, local)?);
    let ordinary = shape::<C>(2048, 10, 2, batch_size(12).max(32))?;
    let narrow = shape::<C>(64, 1, 2, batch_size(12).max(32))?;
    let extra = mul(vector::<Affine<C>>(64, false)?, 3)?;
    let held = add(add(ordinary.held, narrow.held)?,
        add(mul(add(indexed.held, extra)?, 2)?, vector::<[u64;4]>(maximum_n, false)?)?)?;
    let leaf = ordinary.leaf.max(narrow.leaf).max(indexed.leaf);
    let original = hb::<C>(minimum_n)?;
    Some(held <= original.held && leaf <= original.leaf)
}
"#;
    let mut export =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/query-wrapper176/probe-export.rs")
            .unwrap();
    export.truncate(export.find("#[cfg(all(test,").unwrap());
    let end = export.rfind('}').unwrap();
    export.insert_str(end,"\n pub fn b_reference(bases:&[G2Affine],words:&mut[[u64;4]])->(G2Projective,Metadata){(crate::msm_affine::g2_query_exclusive(bases,words),Metadata::default())}\n pub fn b_contained(bases:&[G2Affine],words:&mut[[u64;4]])->(G2Projective,Metadata){let(p,hits)=crate::msm_affine::g2_query_contained_index(bases,words);(p,Metadata{route:match hits{0=>0,4=>336,_=>332},..Metadata::default()})}\n");
    let suffixes = BTreeMap::from([
        (
            "crates/zopro-core/src/msm.rs",
            format!("\n{cfg}\npub(crate) mod contained_index;\n"),
        ),
        (
            "crates/zopro-core/src/msm_affine/g2_phi.rs",
            format!("\n{cfg}\npub(super) mod contained_indexed;\n"),
        ),
        (
            "crates/zopro-core/src/msm_affine.rs",
            format!("\n{cfg}\n{query}\n"),
        ),
        (
            "crates/zopro-core/src/msm_affine/workspace.rs",
            format!("\n{cfg}\n{forecast}\n"),
        ),
        ("crates/zopro-core/src/lib.rs", export),
    ]);
    for (p, s) in &suffixes {
        let p = r("source").join(p);
        let old = fs::read_to_string(&p).unwrap();
        fs::write(p, format!("{old}{s}")).unwrap();
    }
    for (p, s) in [
        ("crates/zopro-core/src/msm/contained_index.rs", packed_child),
        (
            "crates/zopro-core/src/msm_affine/g2_phi/contained_indexed.rs",
            phi_child,
        ),
    ] {
        let p = r("source").join(p);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, s).unwrap();
    }
    let mut exact = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let b = fs::read(r("source").join(p)).unwrap();
        if hash(&b) == h.as_str().unwrap() {
            exact += 1;
        }
        let restored = if let Some(s) = suffixes.get(p.as_str()) {
            String::from_utf8(b)
                .unwrap()
                .strip_suffix(s)
                .unwrap()
                .as_bytes()
                .to_vec()
        } else {
            b
        };
        assert_eq!(hash(&restored), h.as_str().unwrap());
    }
    assert_eq!(exact, 171);
    fs::write(r("source-audit.json"),serde_json::to_vec_pretty(&json!({"snapshot_files":178,"originals_exact":171,"all176_originals_recovered":true,"original_functions_exact":true,"old_global_packing_execution_unchanged":true,"old_admission_removed_only_after_layout_containment":true,"suffixes":suffixes,"clones":clones,"new_native_tests":3,"production_changed":false})).unwrap()).unwrap();
    let old_driver =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/driver.rs").unwrap();
    let mut driver = old_driver.clone();
    let mut driver_edits = vec![];
    edit(
        &mut driver,
        &mut driver_edits,
        "[\"A\",\"B\"].contains",
        "[\"A\",\"B\",\"M\"].contains",
    );
    edit(&mut driver,&mut driver_edits,"zopro_core::coalesce_query_probe::b(std::hint::black_box(&g2),std::hint::black_box(&mut words))","if query==\"M\"{zopro_core::coalesce_query_probe::b_contained(std::hint::black_box(&g2),std::hint::black_box(&mut words))}else{zopro_core::coalesce_query_probe::b_reference(std::hint::black_box(&g2),std::hint::black_box(&mut words))}");
    edit(
        &mut driver,
        &mut driver_edits,
        "\"B_word_reset_inside_timer\":query==\"B\"",
        "\"B_word_reset_inside_timer\":query!=\"A\"",
    );
    audit(&driver, &old_driver, &driver_edits);
    fs::write(r("driver-audit.json"),serde_json::to_vec_pretty(&json!({"original_driver":"/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/driver.rs","edits":driver_edits,"inverse_recovery_exact":true,"same_candidate_executable_OFF_ON":true,"OFF_direct_wrapper_avoids_TLS_versus_frozen_context_only":true})).unwrap()).unwrap();
    fs::create_dir_all(r("driver/src")).unwrap();
    fs::write(r("driver/src/main.rs"), &driver).unwrap();
    fs::write(r("driver.rs"), driver).unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/perf_support.rs",
        r("driver/src/perf_support.rs"),
    )
    .unwrap();
    fs::write(r("driver/Cargo.toml"),"[package]\nname=\"zopro-contained-index\"\nversion=\"0.0.0\"\nedition=\"2021\"\npublish=false\n[workspace]\n[dependencies]\nzopro-core={path=\"../source/crates/zopro-core\"}\nzopro-bn254={path=\"../source/crates/zopro-bn254\"}\nserde_json=\"=1.0.149\"\nsha2=\"=0.10.9\"\nrayon=\"1.10\"\nlibc=\"0.2\"\n[profile.release]\nopt-level=3\nlto=\"fat\"\ncodegen-units=1\n").unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/query-wrapper176/fixtures.json",
        r("fixtures.json"),
    )
    .unwrap();
    let mut env = load("/home/ubuntu/src/zopro-perf-work/solver-a-stream176/environment.json");
    env["RUSTFLAGS"] = json!(format!(
        "{} --cfg zopro_contained_index --cfg zopro_coalesce_probe",
        m["rustflags"].as_str().unwrap()
    ));
    env["CARGO_TARGET_DIR"] = json!(r("target").display().to_string());
    fs::write(
        r("environment.json"),
        serde_json::to_vec_pretty(&env).unwrap(),
    )
    .unwrap();
    let cargo = "/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/cargo";
    let steps = vec![
        json!({"kind":"command","name":"core-tests","argv":[cargo,"test","--locked","--offline","--release","--manifest-path",r("source/Cargo.toml"),"-p","zopro-core","--features","ifma8","--lib","--","--test-threads=1"],"environment":env,"timeout_seconds":1800,"unfiltered":true}),
        json!({"kind":"command","name":"driver-lock","argv":[cargo,"generate-lockfile","--offline","--manifest-path",r("driver/Cargo.toml")],"environment":env,"timeout_seconds":120}),
        json!({"kind":"command","name":"driver-build","argv":[cargo,"build","--locked","--offline","--release","-vv","--message-format=json","--manifest-path",r("driver/Cargo.toml")],"environment":env,"timeout_seconds":1800}),
    ];
    fn walk(p: &Path, pins: &mut BTreeMap<String, String>) {
        for e in fs::read_dir(p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, pins)
            } else {
                pins.insert(p.display().to_string(), hash(&fs::read(p).unwrap()));
            }
        }
    }
    let mut pins = BTreeMap::new();
    walk(&r("source"), &mut pins);
    walk(&r("driver"), &mut pins);
    for f in [
        "setup.rs",
        "design.md",
        "native-tests.rs",
        "source-audit.json",
        "driver-audit.json",
        "environment.json",
        "fixtures.json",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(&fs::read(p).unwrap()));
    }
    for p in["/home/ubuntu/src/zopro-perf-work/g2-bucket-preserving-pack176/census.json","/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-g2-packed-index175/artifacts/source/crates/zopro-core/src/msm/packed_index.rs"]{pins.insert(p.into(),hash(&fs::read(p).unwrap()));}
    fs::write(
        r("build-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        r("build-plan.json"),
        serde_json::to_vec_pretty(
            &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("Prepared178files/171originalsexact/all176recover; originalglobalpacking math exact; prospectiveadmission-onlychange plus nativeuniform12adapter;3newnative tests.");
}

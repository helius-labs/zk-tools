use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-indexed-mixed-top176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn function(text: &str, sig: &str) -> String {
    let a = text.find(sig).unwrap();
    let open = a + text[a..].find('{').unwrap();
    let mut end = open + 1;
    let mut depth = 1;
    while depth > 0 {
        match text.as_bytes()[end] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        end += 1;
    }
    text[a..end].to_string()
}
fn edit(s: &mut String, edits: &mut Vec<(String, String)>, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replacen(a, b, 1);
    edits.push((a.into(), b.into()));
}
fn recover(s: &str, original: &str, edits: &[(String, String)]) {
    let mut s = s.to_owned();
    for (a, b) in edits.iter().rev() {
        assert_eq!(s.matches(b).count(), 1, "{b}");
        s = s.replacen(b, a, 1);
    }
    assert_eq!(s, original);
}
fn main() {
    assert!(!r("source").exists());
    assert_eq!(read(r("algebra-check/result.json"))["status"], "complete");
    let manifest=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let bytes = fs::read(Path::new(Z).join(p)).unwrap();
        assert_eq!(hash(&bytes), h.as_str().unwrap());
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::write(to, bytes).unwrap();
    }
    let phi = fs::read_to_string(r("source/crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
    let borrowed =
        fs::read_to_string(r("source/crates/zopro-core/src/msm_affine/g2_borrowed.rs")).unwrap();
    let msm = fs::read_to_string(r("source/crates/zopro-core/src/msm_affine.rs")).unwrap();
    let mut clones = BTreeMap::new();
    let mut algebra = fs::read_to_string(r("algebra.rs")).unwrap();
    algebra.truncate(algebra.find("#[cfg(test)]").unwrap());
    let native_digit = function(
        &fs::read_to_string(r(
            "source/crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs",
        ))
        .unwrap(),
        "fn digit_at(",
    );
    // The accepted reader has inline(always); token-equivalent arithmetic is
    // inherited by this earlier independently checked scalar model.
    assert!(native_digit.contains("((value + 1) >> 1)"));
    let component_suffix =
        "\nuse super::*;\nuse super::super::g2_phi::indexed_mixed::{Span, component};\n";
    let original = function(&borrowed, "    fn feed(");
    let mut feed = original.clone();
    let mut edits = vec![];
    edit(&mut feed, &mut edits, "    fn feed(", "fn feed(");
    edit(
        &mut feed,
        &mut edits,
        "        &mut self,",
        "        state: &mut Window<'_>,",
    );
    edit(
        &mut feed,
        &mut edits,
        "        w: usize,\n        bits: usize,",
        "        span: Span,",
    );
    edit(
        &mut feed,
        &mut edits,
        "component(s, high, w, bits)",
        "component(s, high, span)",
    );
    // Replace each self receiver separately, preserving the full loop body.
    let count = feed.matches("self.").count();
    let replacement = feed.replace("self.", "state.");
    edits.push((feed.clone(), replacement.clone()));
    feed = replacement;
    recover(&feed, &original, &edits);
    // The feed method's key lifetime must remain shared with Window's references.
    let old = feed.clone();
    feed = feed
        .replace("fn feed(", "fn feed<'a>(")
        .replace("state: &mut Window<'_>", "state: &mut Window<'a>")
        .replace("bases: &'a [G2Affine]", "bases: &'a [G2Affine]");
    edits.push((old, feed.clone()));
    recover(&feed, &original, &edits);
    clones.insert("borrowed_feed",json!({"original":original,"candidate":feed,"edits":edits,"self_receiver_substitutions":count}));
    let original = function(&borrowed, "pub(super) fn try_window(");
    let mut window = original.clone();
    let mut edits = vec![];
    edit(
        &mut window,
        &mut edits,
        "pub(super) fn try_window(",
        "pub(in crate::msm_affine) fn try_span(",
    );
    edit(
        &mut window,
        &mut edits,
        "    w: usize,\n    bits: usize,",
        "    span: Span,",
    );
    edit(
        &mut window,
        &mut edits,
        "    let size = batch_size(bits).max(32);",
        "    let bits = span.width;\n    let size = batch_size(bits).max(32);",
    );
    edit(
        &mut window,
        &mut edits,
        "s.feed(bases, short, true, w, bits, size);",
        "feed(&mut s, bases, short, true, span, size);",
    );
    edit(
        &mut window,
        &mut edits,
        "s.feed(bases, short, false, w, bits, size);",
        "feed(&mut s, bases, short, false, span, size);",
    );
    recover(&window, &original, &edits);
    clones.insert(
        "borrowed_window",
        json!({"original":original,"candidate":window,"edits":edits}),
    );
    let borrowed_child=format!("//! Offset spans using original borrowed Window state.\n{component_suffix}\n{feed}\n{window}\n");
    let original = function(&phi, "fn hb_top_component(");
    let mut top = original.clone();
    let mut edits = vec![];
    edit(
        &mut top,
        &mut edits,
        "    w: usize,\n    bits: usize,",
        "    span: Span,",
    );
    let count = top.matches("hb_digit(s, high, w, bits)").count();
    assert_eq!(count, 2);
    let replacement = top.replace("hb_digit(s, high, w, bits)", "component(s, high, span)");
    edits.push((top.clone(), replacement.clone()));
    top = replacement;
    recover(&top, &original, &edits);
    clones.insert(
        "top",
        json!({"original":original,"candidate":top,"edits":edits}),
    );
    let original = function(&phi, "fn hb_window(");
    let mut window = original.clone();
    let mut edits = vec![];
    edit(
        &mut window,
        &mut edits,
        "w: usize, bits: usize)",
        "span: Span)",
    );
    edit(&mut window,&mut edits,"    if w + 1 == 127usize.div_ceil(bits) && 126 - w * bits <= 8 {\n        let top = 126 - w * bits;","    let bits = span.width;\n    if span.start == 118 {\n        let top = 8;");
    edit(
        &mut window,
        &mut edits,
        "hb_top_component(bases, short, true, w, bits, top)",
        "hb_top_component(bases, short, true, span, top)",
    );
    edit(
        &mut window,
        &mut edits,
        "hb_top_component(bases, short, false, w, bits, top)",
        "hb_top_component(bases, short, false, span, top)",
    );
    edit(
        &mut window,
        &mut edits,
        "super::g2_borrowed::try_window(bases, short, w, bits)",
        "super::super::g2_borrowed::indexed_mixed::try_span(bases, short, span)",
    );
    edit(
        &mut window,
        &mut edits,
        "hb_digit(s, true, w, bits)",
        "component(s, true, span)",
    );
    edit(
        &mut window,
        &mut edits,
        "hb_digit(s, false, w, bits)",
        "component(s, false, span)",
    );
    recover(&window, &original, &edits);
    clones.insert(
        "phi_window",
        json!({"original":original,"candidate":window,"edits":edits}),
    );
    let original = function(&phi, "fn hb_raw(");
    let mut raw = original.clone();
    let mut edits = vec![];
    edit(&mut raw,&mut edits,"fn hb_raw(bases: &[G2Affine], short: &mut [[u64; 4]], bits: usize, parts: usize) -> G2Projective {","pub(in crate::msm_affine) fn raw_exclusive(bases: &[G2Affine], short: &mut [[u64; 4]]) -> (G2Projective, bool) {\n    if !(1024..65536).contains(&bases.len()) || !super::super::workspace::indexed_mixed_fits(bases.len()).unwrap_or(false) {\n        return (super::raw_exclusive(bases, short), false);\n    }\n    let parts = 2;");
    edit(
        &mut raw,
        &mut edits,
        "    let windows = 127usize.div_ceil(bits);",
        "    let cover = spans();\n    let windows = cover.len();",
    );
    edit(
        &mut raw,
        &mut edits,
        "hb_window(&bases[start..end], &short[start..end], w, bits)",
        "hb_window(&bases[start..end], &short[start..end], cover[w])",
    );
    edit(&mut raw,&mut edits,"    for sum in sums.iter().rev() {\n        for _ in 0..bits {","    for (i, sum) in sums.iter().enumerate().rev() {\n        let shifts = if i == windows - 1 { 0 } else { cover[i].width };\n        for _ in 0..shifts {");
    edit(
        &mut raw,
        &mut edits,
        "    result\n}",
        "    (result, true)\n}",
    );
    recover(&raw, &original, &edits);
    clones.insert(
        "phi_raw",
        json!({"original":original,"candidate":raw,"edits":edits}),
    );
    let phi_child = format!(
        "{algebra}\nuse super::*;\n{top}\n{window}\n{raw}\n{}",
        fs::read_to_string(r("native-tests.rs")).unwrap()
    );
    let original = function(&msm, "pub(super) fn g2_query_exclusive(");
    let mut query = original.clone();
    let mut edits = vec![];
    edit(
        &mut query,
        &mut edits,
        "pub(super) fn g2_query_exclusive(",
        "pub(super) fn g2_query_indexed_mixed(",
    );
    edit(
        &mut query,
        &mut edits,
        ") -> zopro_bn254::G2Projective {",
        ") -> (zopro_bn254::G2Projective, bool) {",
    );
    edit(
        &mut query,
        &mut edits,
        "return crate::msm_grouped_b2(bases, &g);",
        "return (crate::msm_grouped_b2(bases, &g), false);",
    );
    edit(
        &mut query,
        &mut edits,
        "g2_phi::raw_exclusive(&points, &mut g.values)",
        "g2_phi::indexed_mixed::raw_exclusive(&points, &mut g.values)",
    );
    edit(
        &mut query,
        &mut edits,
        "g2_phi::raw_exclusive(bases, words)",
        "g2_phi::indexed_mixed::raw_exclusive(bases, words)",
    );
    edit(&mut query,&mut edits,"    match groups {\n        Some(g) => crate::msm_grouped_b2(bases, &g),\n        None => crate::msm_raw_b2(bases, words),\n    }","    (match groups {\n        Some(g) => crate::msm_grouped_b2(bases, &g),\n        None => crate::msm_raw_b2(bases, words),\n    }, false)");
    recover(&query, &original, &edits);
    clones.insert(
        "query",
        json!({"original":original,"candidate":query,"edits":edits}),
    );
    let cfg="#[cfg(all(zopro_indexed_mixed_top, zopro_g2_horner_buckets, feature=\"ifma8\", target_arch=\"x86_64\", target_feature=\"avx512ifma\", target_feature=\"avx512vl\", target_feature=\"avx512dq\", not(no_ifma)))]";
    let forecast = r#"
pub(crate) fn indexed_mixed_fits(n: usize) -> Option<bool> {
    use zopro_bn254::G2Config;
    if !(1024..65536).contains(&n) { return Some(false); }
    let ordinary12 = shape::<G2Config>(2048, 8, 2, batch_size(12).max(32))?;
    let ordinary11 = shape::<G2Config>(1024, 2, 2, batch_size(11).max(32))?;
    // Conservative generic parent/leaf for the final indexed output as well.
    let leading = shape::<G2Config>(256, 1, 2, batch_size(12).max(32))?;
    let sign_top = top::<G2Config>(n.div_ceil(2), 9)?;
    let extra = mul(vector::<zopro_bn254::G2Affine>(256, false)?, 3)?;
    let held = add(add(add(ordinary12.held, ordinary11.held)?, leading.held)?,
        add(mul(add(sign_top.held, extra)?, 2)?, vector::<[u64;4]>(n, false)?)?)?;
    let leaf = ordinary12.leaf.max(ordinary11.leaf).max(leading.leaf).max(sign_top.leaf);
    let original = hb::<G2Config>(n)?;
    Some(held <= original.held && leaf <= original.leaf)
}
"#;
    let mut export =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/query-wrapper176/probe-export.rs")
            .unwrap();
    export.truncate(export.find("#[cfg(all(test,").unwrap());
    let end = export.rfind('}').unwrap();
    export.insert_str(end,"\n pub fn b_mixed(bases:&[G2Affine],words:&mut[[u64;4]])->(G2Projective,Metadata){let(p,used)=crate::msm_affine::g2_query_indexed_mixed(bases,words);(p,Metadata{route:if used{328}else{0},..Metadata::default()})}\n");
    let suffixes = BTreeMap::from([
        (
            "crates/zopro-core/src/msm_affine/g2_phi.rs",
            format!("\n{cfg}\npub(super) mod indexed_mixed;\n"),
        ),
        (
            "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
            format!("\n{cfg}\npub(super) mod indexed_mixed;\n"),
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
        let original = fs::read_to_string(&p).unwrap();
        fs::write(p, format!("{original}{s}")).unwrap();
    }
    for (p, s) in [
        (
            "crates/zopro-core/src/msm_affine/g2_phi/indexed_mixed.rs",
            phi_child,
        ),
        (
            "crates/zopro-core/src/msm_affine/g2_borrowed/indexed_mixed.rs",
            borrowed_child,
        ),
    ] {
        let p = r("source").join(p);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, s).unwrap();
    }
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let bytes = fs::read(r("source").join(p)).unwrap();
        if hash(&bytes) == h.as_str().unwrap() {
            exact += 1;
        }
        let restored = if let Some(s) = suffixes.get(p.as_str()) {
            String::from_utf8(bytes)
                .unwrap()
                .strip_suffix(s)
                .unwrap()
                .as_bytes()
                .to_vec()
        } else {
            bytes
        };
        assert_eq!(hash(&restored), h.as_str().unwrap());
    }
    assert_eq!(exact, 171);
    fs::write(r("source-audit.json"),serde_json::to_vec_pretty(&json!({"snapshot_files":178,"originals_exact":171,"all176_originals_recovered":true,"original_functions_exact":true,"suffixes":suffixes,"clones":clones,"production_changed":false,"new_native_tests":3,"prior_algebra_tests_inherited_not_rerun":4})).unwrap()).unwrap();
    let mut driver =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/driver.rs").unwrap();
    let mut driver_edits = vec![];
    edit(
        &mut driver,
        &mut driver_edits,
        "[\"A\",\"B\"].contains",
        "[\"A\",\"B\",\"M\"].contains",
    );
    edit(&mut driver,&mut driver_edits,"zopro_core::coalesce_query_probe::b(std::hint::black_box(&g2),std::hint::black_box(&mut words))","if query==\"M\"{zopro_core::coalesce_query_probe::b_mixed(std::hint::black_box(&g2),std::hint::black_box(&mut words))}else{zopro_core::coalesce_query_probe::b(std::hint::black_box(&g2),std::hint::black_box(&mut words))}");
    edit(
        &mut driver,
        &mut driver_edits,
        "\"B_word_reset_inside_timer\":query==\"B\"",
        "\"B_word_reset_inside_timer\":query!=\"A\"",
    );
    fs::create_dir_all(r("driver/src")).unwrap();
    fs::write(r("driver/src/main.rs"), &driver).unwrap();
    fs::write(r("driver.rs"), driver).unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/perf_support.rs",
        r("driver/src/perf_support.rs"),
    )
    .unwrap();
    fs::write(r("driver/Cargo.toml"),"[package]\nname=\"zopro-g2-indexed-mixed-top\"\nversion=\"0.0.0\"\nedition=\"2021\"\npublish=false\n[workspace]\n[dependencies]\nzopro-core={path=\"../source/crates/zopro-core\"}\nzopro-bn254={path=\"../source/crates/zopro-bn254\"}\nserde_json=\"=1.0.149\"\nsha2=\"=0.10.9\"\nrayon=\"1.10\"\nlibc=\"0.2\"\n[profile.release]\nopt-level=3\nlto=\"fat\"\ncodegen-units=1\n").unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/query-wrapper176/fixtures.json",
        r("fixtures.json"),
    )
    .unwrap();
    let mut env = read("/home/ubuntu/src/zopro-perf-work/solver-a-stream176/environment.json");
    env["RUSTFLAGS"] = json!(format!(
        "{} --cfg zopro_indexed_mixed_top --cfg zopro_coalesce_probe",
        manifest["rustflags"].as_str().unwrap()
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
    fn scan(p: &Path, pins: &mut BTreeMap<String, String>) {
        for e in fs::read_dir(p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                scan(&p, pins)
            } else {
                pins.insert(p.display().to_string(), hash(&fs::read(p).unwrap()));
            }
        }
    }
    let mut pins = BTreeMap::new();
    scan(&r("source"), &mut pins);
    scan(&r("driver"), &mut pins);
    for f in [
        "setup.rs",
        "design.md",
        "algebra.rs",
        "native-tests.rs",
        "environment.json",
        "source-audit.json",
        "fixtures.json",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(&fs::read(p).unwrap()));
    }
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
    println!("Prepared178files/171exact/all176recover; original routines exact; six private clone inverse audits; three new native curve/resource tests. No measurements.");
}

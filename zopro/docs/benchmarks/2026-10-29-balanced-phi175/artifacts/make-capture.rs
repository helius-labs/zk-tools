use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-phi175";
fn rep(s: &mut String, a: &str, b: &str) {
    assert!(s.contains(a), "missing{a}");
    *s = s.replace(a, b);
}
fn main() {
    let root = Path::new(R);
    assert!(!root.join("capture.rs").exists());
    let mut p = fs::read_to_string(
        "/home/ubuntu/src/zopro-perf-work/balanced-ordinary175/capture-setup.rs",
    )
    .unwrap();
    rep(
        &mut p,
        "audit[\"originals_exact\"], 174",
        "audit[\"originals_exact\"], 173",
    );
    rep(
        &mut p,
        "audit[\"accepted_balanced13_byte_exact\"]",
        "audit[\"accepted_KH_and_A_exact\"]",
    );
    let a = p.find("    let required=").unwrap();
    let b = p[a..].find("    for (label, count)").unwrap() + a;
    p.replace_range(a..b,"    let required=[\"msm_affine::g2_borrowed::balanced::tests::exact_signed_component_reconstruction_and_geometry\",\"msm_affine::g2_borrowed::balanced::tests::balanced_phi_matches_independent_generator_sums\",\"msm_affine::g2_borrowed::balanced::tests::compiled_dispatch_matches_policy\"];\n");
    rep(
        &mut p,
        "[(\"control\", 124), (\"candidate\", 126)]",
        "[(\"control\", 127), (\"candidate\", 127)]",
    );
    rep(&mut p, "balanced_ordinary_cfg", "balanced_phi_cfg");
    rep(&mut p, "16500", "18154");
    rep(&mut p, "17005", "18592");
    rep(&mut p, "17083", "18670");
    rep(
        &mut p,
        "        if label == \"candidate\" {\n            for name in required {",
        "        {\n            for name in required {",
    );
    let a = p.find("    let mut ids =").unwrap();
    p.insert_str(a,"    assert_eq!(read(r(\"code-audit.json\"))[\"passed\"],true);\n    for (path,h) in read(r(\"code-audit.json\"))[\"pins\"].as_object().unwrap(){assert_eq!(hash(path),h.as_str().unwrap());pin(&mut pins,path);}\n");
    let a = p.find("    for name in [\n        \"design.md\"").unwrap();
    let b = p[a..].find("    ] {").unwrap() + a;
    p.replace_range(a..b,r#"    for name in [
        "design.md", "history-audit.md", "balanced.rs", "bootstrap.rs", "setup.rs", "native.rs", "native-helper",
        "make-capture.rs", "capture-setup.rs", "capture.rs", "capture-helper", "pmu-feasibility.log",
        "audit.rs", "audit-helper", "audit.log", "code-audit.json", "source-audit.json", "identities.json",
        "prior-proof-sha256.json", "jobs.json", "cases.json", "preflight-cases.json", "native/result.json",
        "native/control/artifacts.json", "native/candidate/artifacts.json", "frozen-control.json", "frozen-candidate.json",
"#);
    fs::write(root.join("capture-setup.rs"), p).unwrap();
    let o = fs::read_to_string("/home/ubuntu/src/zopro-perf-work/balanced-ordinary175/capture.rs")
        .unwrap();
    let a = o.find("fn prepare() {").unwrap();
    let b = o.find("fn push(").unwrap();
    let mut s = format!("{}include!(\"capture-setup.rs\");\n{}", &o[..a], &o[b..]);
    rep(
        &mut s,
        "const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/balanced-kh-production\";",
        "const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/radix-groups175\";",
    );
    rep(&mut s, "balanced-ordinary175", "balanced-phi175");
    rep(&mut s, "17005", "18592");
    rep(&mut s, "17083", "18670");
    rep(
        &mut s,
        "[(\"control\", 124), (\"candidate\", 126)]",
        "[(\"control\", 127), (\"candidate\", 127)]",
    );
    rep(
        &mut s,
        "completed balanced ordinary G1 primary pilot;not adopted",
        "completed balanced G2 phi primary pilot;not adopted",
    );
    rep(&mut s, "\"OFF\":124,\"ON\":126", "\"OFF\":127,\"ON\":127");
    rep(&mut s,"ordinary width12 with22balanced11/12spans;all9nativepreflights;primary32only15blockpilot;acceptedfused13exact;noadditionalqualification/adoption","two signed127bit G2 components with11balanced11/12spans;two parts;all9nativepreflights;primary32only15blockpilot;KH/Aexact;noadditionalqualification/adoption");
    fs::write(root.join("capture.rs"), s).unwrap();
    println!("Generated fixed165step/7902SDK/78Gnark fullSDKpilot;prior18592->18670 only after verification");
}

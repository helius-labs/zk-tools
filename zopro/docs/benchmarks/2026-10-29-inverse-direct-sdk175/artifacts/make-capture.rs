use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175";
fn replace(s: &mut String, from: &str, to: &str) {
    assert!(s.contains(from), "missing {from}");
    *s = s.replace(from, to);
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("capture.rs").exists());
    let old = "/home/ubuntu/src/zopro-perf-work/fq-fips175";
    let mut prepare = fs::read_to_string(format!("{old}/capture-setup.rs")).unwrap();
    replace(
        &mut prepare,
        "audit[\"source_files\"], 175",
        "audit[\"source_files\"], 176",
    );
    replace(
        &mut prepare,
        "audit[\"originals_exact\"], 173",
        "audit[\"originals_exact\"], 174",
    );
    replace(
        &mut prepare,
        "reverse175_by_unedit_and_official_generator",
        "reverse175_exact",
    );
    replace(
        &mut prepare,
        "field[\"passed_tests\"].as_array().unwrap().len(), 33",
        "field[\"passed_tests\"].as_array().unwrap().len(), 34",
    );
    let a = prepare.find("    for path in [").unwrap();
    let b = prepare[a..].find("    let code = read").unwrap() + a;
    prepare.replace_range(a..b, "");
    replace(
        &mut prepare,
        "actual_G1_G2_IFMA_callers_changed",
        "actual_Fq_Fr_inverse_functions_changed",
    );
    let a = prepare.find("    fn pin_tree(").unwrap();
    let b = prepare[a..].find("    let mut ids =").unwrap() + a;
    prepare.replace_range(a..b, "");
    replace(&mut prepare, "\"fq_fips_cfg\"", "\"direct_finish_cfg\"");
    replace(&mut prepare, "prior.len(), 17239", "prior.len(), 17830");
    replace(&mut prepare, "17317", "18076");
    replace(&mut prepare, "17395", "18154");
    let a = prepare
        .find("    for name in [\n        \"design.md\"")
        .unwrap();
    let b = prepare[a..].find("    ] {").unwrap() + a;
    let list = r#"    for name in [
        "design.md", "history-audit.md", "bounds.md", "direct_finish.rs",
        "bootstrap.rs", "make-capture.rs", "code-audit.rs", "code-audit.json",
        "native.rs", "native-helper", "setup.rs", "capture-setup.rs",
        "capture.rs", "capture-helper", "pmu-feasibility.log",
        "source-audit.json", "identities.json", "prior-proof-sha256.json",
        "jobs.json", "cases.json", "preflight-cases.json", "native/result.json",
        "native/control/artifacts.json", "native/candidate/artifacts.json",
        "frozen-control.json", "frozen-candidate.json",
"#;
    prepare.replace_range(a..b, list);
    fs::write(r.join("capture-setup.rs"), &prepare).unwrap();
    let original = fs::read_to_string(format!("{old}/capture.rs")).unwrap();
    let a = original.find("fn prepare() {").unwrap();
    let b = original.find("fn push(").unwrap();
    let mut source = format!(
        "{}include!(\"capture-setup.rs\");\n{}",
        &original[..a],
        &original[b..]
    );
    replace(
        &mut source,
        "const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/fft-eight-columns175\";",
        "const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/serial-phi175\";",
    );
    replace(&mut source, "fq-fips175", "inverse-direct-sdk175");
    replace(&mut source, "17317", "18076");
    replace(&mut source, "17395", "18154");
    replace(&mut source, "            33\n", "            34\n");
    replace(
        &mut source,
        "completed Fq diagonal FIPS primary pilot;not adopted",
        "completed direct inverse-finish full SDK primary pilot;not adopted",
    );
    replace(
        &mut source,
        "\"native_field_tests_each_new_execution\":33",
        "\"native_field_tests_each_new_execution\":34",
    );
    replace(&mut source,"\"new_field_test_cases\":\"Fq8/R256 ordinary and fused plus16-product chains per existing fixture\"","\"field_qualification_scope\":\"existing 68672 signed-residue cases and full original inverse/field suite, inputs inherited and freshly rerun in both coherent SDK builds; zero newly distinct field cases\"");
    replace(&mut source,"native8FqR256ordinaryandfuseddiagonalFIPS;newdataflow/sameintegerresult;all9nativepreflights;primary32only15blockpilot;noadditionalqualification/adoption","direct bounded signed Euclidean finish reduction; exact prior helper; actual complete SDK; all9preflights; primary32only15blockpilot; no additional qualification/adoption");
    fs::write(r.join("capture.rs"), source).unwrap();
    println!("Generated fixed full-SDK pilot controller; separate native OFF/ON, 15 blocks, 7902 requests/78 Gnark proofs planned, ledger18076->18154 only on verification");
}

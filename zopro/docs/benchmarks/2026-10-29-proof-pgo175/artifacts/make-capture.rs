use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
fn rep(s: &mut String, a: &str, b: &str) {
    assert!(s.contains(a), "missing{a}");
    *s = s.replace(a, b);
}
fn main() {
    let root = Path::new(R);
    assert!(!root.join("capture.rs").exists());
    let mut p =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/balanced-phi175/capture-setup.rs")
            .unwrap();
    rep(
        &mut p,
        "audit[\"source_files\"], 176",
        "audit[\"source_files\"], 175",
    );
    rep(
        &mut p,
        "audit[\"originals_exact\"], 173",
        "audit[\"originals_exact\"], 175",
    );
    rep(&mut p,"    assert_eq!(audit[\"reverse175_exact\"], true);\n    assert_eq!(audit[\"accepted_KH_and_A_exact\"], true);","    assert_eq!(audit[\"compiler_only_candidate\"],true);\n    assert_eq!(read(r(\"native-use/result.json\"))[\"status\"],\"complete\");\n    for (file,h) in read(r(\"use-pins.json\")).as_object().unwrap(){assert_eq!(hash(file),h.as_str().unwrap());pin(&mut pins,file);}");
    let a = p.find("    let required=").unwrap();
    let b=p[a..].find("        assert_eq!(\n            read(r(format!(\"native/{label}-prover/tests.json\")))").unwrap()+a;
    p.replace_range(a..b,"    for (label, count) in [(\"control\", 124), (\"candidate\", 124)] {\n        let native=if label==\"candidate\"{\"native-use\"}else{\"native\"};\n        let t=read(r(format!(\"{native}/{label}-core/tests.json\")));\n        assert_eq!(t[\"passed_tests\"].as_array().unwrap().len(),count);\n        assert_eq!(t[\"unfiltered\"],true);\n        let field=read(r(format!(\"{native}/{label}-field/tests.json\")));\n        assert_eq!(field[\"passed_tests\"].as_array().unwrap().len(),33);\n        assert_eq!(field[\"unfiltered\"],true);\n");
    rep(&mut p, "native/{label}", "{native}/{label}");
    rep(&mut p, "native/candidate-sdk", "native-use/candidate-sdk");
    rep(&mut p, "balanced_phi_cfg", "profile_use");
    let a = p.find("    let mut prior:").unwrap();
    let b = p[a..].find("    let all =").unwrap() + a;
    p.replace_range(a..b,r#"    let training=read(r("training-audit.json"));assert_eq!(training["passed"],true);assert_eq!(training["proof_ledger"],18769);
    let mut prior:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),18670);
    for (file,h) in read(r("training-proof-inventory.json")).as_object().unwrap(){assert_eq!(hash(file),h.as_str().unwrap());pin(&mut pins,file);if file.ends_with(".proof"){assert!(prior.insert(h.as_str().unwrap().into()));}}
    assert_eq!(prior.len(),18769);save(r("comparison-prior-proof-sha256.json"),&json!(prior));
"#);
    let a = p.find("    for name in [\n").unwrap();
    let b = p[a..].find("    ] {").unwrap() + a;
    p.replace_range(a..b,r#"    for name in [
        "design.md","history-audit.md","bootstrap.rs","setup.rs","native.rs","native-helper","train-driver.rs","train.rs","train-helper",
        "make-capture.rs","capture-setup.rs","capture.rs","capture-helper","pmu-feasibility.log","audit.rs","audit-helper","audit.log","code-audit.json",
        "source-audit.json","identities.json","comparison-prior-proof-sha256.json","training-audit.json","training-proof-inventory.json","training/result.json",
        "jobs.json","cases.json","preflight-cases.json","native/result.json","native-use/result.json","use-pins.json",
        "native/control/artifacts.json","native-use/candidate/artifacts.json","frozen-control.json","frozen-candidate.json",
"#);
    rep(&mut p, "18592->18670", "18769->18847");
    fs::write(root.join("capture-setup.rs"), p).unwrap();
    let mut s =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/balanced-phi175/capture.rs").unwrap();
    rep(&mut s, "balanced-phi175", "proof-pgo175");
    rep(&mut s, "18592", "18769");
    rep(&mut s, "18670", "18847");
    rep(
        &mut s,
        "prior-proof-sha256.json",
        "comparison-prior-proof-sha256.json",
    );
    rep(&mut s,"[(\"control\", 127), (\"candidate\", 127)] {","[(\"control\", 124), (\"candidate\", 124)] {\n        let native=if label==\"candidate\"{\"native-use\"}else{\"native\"};");
    rep(&mut s, "native/{label}", "{native}/{label}");
    rep(
        &mut s,
        "completed balanced G2 phi primary pilot;not adopted",
        "completed current175 proof-only PGO SDK pilot;not adopted",
    );
    rep(&mut s, "\"OFF\":127,\"ON\":127", "\"OFF\":124,\"ON\":124");
    rep(
        &mut s,
        "\"native_field_tests_inherited\":33,\"new_field_tests\":0",
        "\"native_field_tests_each_fresh_execution\":33,\"new_field_input_cases\":0",
    );
    rep(&mut s,"two signed127bit G2 components with11balanced11/12spans;two parts;all9nativepreflights;primary32only15blockpilot;KH/Aexact;noadditionalqualification/adoption","compiler-only proof-only PGO fromcurrent175alloriginalsexact;99trainingproofsexcludedfromlatency;all9nativepreflights;primary32only15blockpilot;noadditionalqualification/adoption");
    fs::write(root.join("capture.rs"), s).unwrap();
    println!("PreparedfirstPGOUSEperformancepilotcontroller165steps7902SDK78Gnark;ledger18769->18847onlyverified;trainingprior18670unchanged");
}

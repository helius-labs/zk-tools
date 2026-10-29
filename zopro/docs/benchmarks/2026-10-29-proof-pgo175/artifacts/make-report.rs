use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
fn rep(s: &mut String, a: &str, b: &str) {
    assert!(s.contains(a), "missing{a}");
    *s = s.replace(a, b);
}
fn main() {
    let root = Path::new(R);
    assert!(!root.join("report.rs").exists());
    let mut s =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/balanced-phi175/report.rs").unwrap();
    rep(&mut s, "balanced-phi175", "proof-pgo175");
    rep(
        &mut s,
        "a[\"gate_passed\"], false",
        "a[\"gate_passed\"], true",
    );
    rep(&mut s, "18670", "18847");
    let a = s
        .find("    let audit = read(r(\"source-audit.json\"));")
        .unwrap();
    let b = s[a..].find("    let mut paths = vec![];").unwrap() + a;
    s.replace_range(a..b,r#"    let audit=read(r("source-audit.json"));assert_eq!(audit["originals_exact"],175);
    for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(r("source").join(p)),h.as_str().unwrap());assert_eq!(hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),h.as_str().unwrap());}
"#);
    rep(&mut s, "source.len(), 176", "source.len(), 175");
    rep(
        &mut s,
        "[(\"native\", 10), (\"runs\", 165)]",
        "[(\"native\",9),(\"training\",109),(\"native-use\",6),(\"runs\",165)]",
    );
    rep(&mut s, "receipts.len(), 175", "receipts.len(), 289");
    rep(&mut s, "streams, 700", "streams, 1156");
    rep(&mut s,"    let inv = read(r(\"proof-inventory.json\"));","    let mut inv=read(r(\"proof-inventory.json\"));\n    for(p,h)in read(r(\"training-proof-inventory.json\")).as_object().unwrap(){assert!(inv.as_object_mut().unwrap().insert(p.clone(),h.clone()).is_none());}");
    rep(&mut s, "proof.len(), 156", "proof.len(), 354");
    let a = s.find("    for f in [\n        \"analysis.json\"").unwrap();
    let b = s[a..].find("    ] {").unwrap() + a;
    s.replace_range(a..b,r#"    for f in [
        "analysis.json","finish.log","design.md","history-audit.md","bootstrap.rs","setup.rs","native.rs","train-driver.rs","train.rs","train-tests.log",
        "training-audit.json","training-proof-inventory.json","training-prepare.log","training-cases.json","training-jobs.json","training-plan.json","training-pins.json",
        "audit.rs","audit.log","code-audit.json","source-audit.json","control-flags.txt","generate-flags.txt","candidate-flags.txt",
        "build-pins.json","build-plan.json","use-pins.json","use-plan.json","use-prepare.log","frozen-control.json","frozen-generate.json","frozen-candidate.json",
        "frozen-control-cargo.jsonl","frozen-generate-cargo.jsonl","frozen-candidate-cargo.jsonl","native/control/artifacts.json","native/generate/artifacts.json","native-use/candidate/artifacts.json",
        "identities.json","make-capture.rs","capture-setup.rs","capture.rs","pins.json","run-plan.json","jobs.json","cases.json","preflight-cases.json",
        "prior-proof-sha256.json","comparison-prior-proof-sha256.json","proof-inventory.json","pmu-feasibility.log","prepare.log","host-before.txt","host-after.txt",
        "initial-capture-controller/capture-setup.rs","initial-capture-controller/capture.rs","initial-capture-controller/make-capture.rs","initial-capture-controller/prepare.log",
        "initial-capture-controller/helper-sha256.txt","initial-capture-controller/failure.json","initial-capture-controller/repair.json","make-report.rs","report.rs","report-body.rs",
"#);
    rep(
        &mut s,
        "if f == \"pmu-feasibility.log\" {",
        "if f == \"pmu-feasibility.log\" || f == \"train-tests.log\" {",
    );
    let a = s.find("    let close = json!(").unwrap();
    let b = s[a..].find("    save(d.join(\"closeout.json\")").unwrap() + a;
    s.replace_range(a..b,r#"    archive_training(&d);
    let close=json!({"primary_latency_pilot_gate_passed":true,"candidate_all8_qualified":false,"native_plain_steps":9,"training_steps":109,"native_USE_steps":6,"performance_steps":165,"receipts":289,"original_streams_byte_exact":1156,"private_source_files":175,"originals_exact":175,"training_SDK_requests":126,"performance_SDK_requests":7902,"total_SDK_requests":8028,"training_Gnark":99,"performance_Gnark":78,"new_Gnark_total":177,"proof_public_members":354,"proof_ledger":18847,"closing_pins_verified":pins.as_object().unwrap().len(),"adopted":false,"production_changed":false,"goal_active_unmet":true,"accepted_all8_CPU10x_rows":0,"no_successful_measurements_or_tests_repeated":true,"initial_prepare_failure_before_capture_retained":true});
"#);
    rep(&mut s,"Archived balancedphi175;7902SDK78Gnark ledger18847;no adoption","Archived proofPGO175 positivepilot;8028SDK177Gnarkledger18847;no adoption;broaderqualificationrequired");
    fs::write(root.join("report.rs"), s).unwrap();
}

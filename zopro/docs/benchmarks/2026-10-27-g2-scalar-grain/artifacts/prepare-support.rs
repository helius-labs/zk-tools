use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-g2-scalar-grain");
    let oldroot = "/tmp/zopro-grouping-prefix-cutoff";
    let mut prep = fs::read_to_string(format!("{oldroot}/prepare-sdk.rs"))
        .unwrap()
        .replace(oldroot, "/tmp/zopro-g2-scalar-grain")
        .replace("native-v2/", "native/")
        .replace("source-pins-v2.json", "source-pins.json")
        .replace("preparation-failures.json", "preparation-failure.json")
        .replace("117", "116");
    let start = prep.find("fn prior() -> BTreeSet<String> {").unwrap();
    let end = prep.find("fn prepare() {").unwrap();
    let prior = r#"fn prior() -> BTreeSet<String> {
    let p = read("/tmp/zopro-grouping-prefix-cutoff/performance/protocol.json");
    let mut seen: BTreeSet<String> = serde_json::from_value(p["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 9448);
    let mut duplicates=0;
    for inventory in ["/tmp/zopro-grouping-prefix-cutoff/proof-inventory.json", "/tmp/zopro-current-c7a-20261027/proof-inventory.json"] {
        for (file, want) in read(inventory).as_object().unwrap() {
            assert_eq!(hash(file),want.as_str().unwrap());
            if file.ends_with(".proof") {
                assert_eq!(fs::metadata(file).unwrap().len(),164);
                if !seen.insert(want.as_str().unwrap().to_owned()) {duplicates+=1;}
            }
        }
    }
    assert_eq!(duplicates,9);
    assert_eq!(seen.len(),10180);
    seen
}
"#;
    prep.replace_range(start..end, prior);
    prep = prep.replace("9448", "10189");
    // The seed protocol intentionally retains its historical 9448 identities.
    prep = prep.replace(
        "assert_eq!(seen.len(), 10189);\n    let mut duplicates",
        "assert_eq!(seen.len(), 9448);\n    let mut duplicates",
    );
    prep = prep
        .replace("prior9439", "prior10180")
        .replace("prior10189", "prior10189");
    prep = prep.replace(
        "json!([\"1x2-w32\", \"4x2-w32\"])",
        "json!([\"1x2-w32\", \"2x2-w32\"])",
    );
    fs::write(r.join("prepare-sdk.rs"), prep).unwrap();
    let a = fs::read_to_string(format!("{oldroot}/analyze.rs"))
        .unwrap()
        .replace(oldroot, "/tmp/zopro-g2-scalar-grain")
        .replace("9448", "10189")
        .replace("9904", "10645")
        .replace("candidate_core_tests\":117", "candidate_core_tests\":116")
        .replace(
            "exact_in_loop_cardinality_stop",
            "minimum_phi_decomposition_grain128",
        )
        .replace("PrefixCutoff", "G2ScalarGrain");
    fs::write(r.join("analyze.rs"), a).unwrap();
    fs::copy(format!("{oldroot}/stats.rs"), r.join("stats.rs")).unwrap();
    fs::copy(r.join("design.md"), r.join("sdk-design.md")).unwrap();
    println!("Prepared fixed15block SDK support; prior10180/preflight10189/final10645; primary+2x2 profiles");
}

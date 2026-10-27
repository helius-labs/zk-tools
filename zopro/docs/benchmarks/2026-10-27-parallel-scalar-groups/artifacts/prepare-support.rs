use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-parallel-scalar-groups");
    let oldroot = "/tmp/zopro-g2-scalar-grain";
    let mut prep = fs::read_to_string(format!("{oldroot}/prepare-sdk.rs"))
        .unwrap()
        .replace(oldroot, r.to_str().unwrap());
    let start = prep.find("fn prior() -> BTreeSet<String> {").unwrap();
    let end = prep.find("fn prepare() {").unwrap();
    let prior = r#"fn prior() -> BTreeSet<String> {
    let protocol=read("/tmp/zopro-g2-scalar-grain/performance/protocol.json");
    let mut seen:BTreeSet<String>=serde_json::from_value(protocol["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(),10189);
    let mut duplicates=0;
    for(file,want)in read("/tmp/zopro-g2-scalar-grain/proof-inventory.json").as_object().unwrap(){
        assert_eq!(hash(file),want.as_str().unwrap());
        if file.ends_with(".proof") {
            assert_eq!(fs::metadata(file).unwrap().len(),164);
            if !seen.insert(want.as_str().unwrap().to_owned()){duplicates+=1;}
        }
    }
    assert_eq!(duplicates,9);assert_eq!(seen.len(),10645);seen
}
"#;
    prep.replace_range(start..end, prior);
    prep = prep.replace(
        "            116\n        } else",
        "            119\n        } else",
    );
    prep = prep
        .replace("\"finish-native-plan.rs\",", "\"prepare-native.rs\",")
        .replace("\"preparation-failure.json\",", "\"memory-review.md\",");
    prep = prep.replace(
        "json!([\"1x2-w32\", \"2x2-w32\"])",
        "json!([\"1x2-w32\", \"4x2-w32\"])",
    );
    prep = prep.replace(
        "assert_eq!(seen.len(), 10189);",
        "assert_eq!(seen.len(), 10654);",
    );
    prep = prep.replace("\"next_prior_count\":10189", "\"next_prior_count\":10654");
    prep = prep
        .replace("prior10180", "prior10645")
        .replace("prior10189", "prior10654");
    fs::write(r.join("prepare-sdk.rs"), prep).unwrap();
    let a = fs::read_to_string(format!("{oldroot}/analyze.rs"))
        .unwrap()
        .replace(oldroot, r.to_str().unwrap())
        .replace("10189", "10654")
        .replace("10645", "11110")
        .replace("candidate_core_tests\":116", "candidate_core_tests\":119")
        .replace(
            "minimum_phi_decomposition_grain128",
            "parallel_exact_scalar_grouping",
        )
        .replace("G2ScalarGrain", "ParallelScalarGroups");
    fs::write(r.join("analyze.rs"), a).unwrap();
    fs::copy(format!("{oldroot}/stats.rs"), r.join("stats.rs")).unwrap();
    fs::copy(r.join("design.md"), r.join("sdk-design.md")).unwrap();
    println!("Prepared fixed SDK support:prior10645/preflight10654/final11110;candidate119tests;6profiles primary+4x2");
}

use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-clean-fft-leaves");
    let old = "/tmp/zopro-parallel-scalar-groups";
    let mut s = fs::read_to_string(format!("{old}/prepare-sdk.rs"))
        .unwrap()
        .replace(old, r.to_str().unwrap());
    let begin = s.find("fn prior() -> BTreeSet<String> {").unwrap();
    let end = s.find("fn prepare() {").unwrap();
    let prior = r#"fn prior() -> BTreeSet<String> {
    let p=read("/tmp/zopro-parallel-scalar-groups/performance/protocol.json");
    let mut seen:BTreeSet<String>=serde_json::from_value(p["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(),10654);let mut duplicates=0;
    for(file,want)in read("/tmp/zopro-parallel-scalar-groups/proof-inventory.json").as_object().unwrap(){assert_eq!(hash(file),want.as_str().unwrap());if file.ends_with(".proof"){assert_eq!(fs::metadata(file).unwrap().len(),164);if !seen.insert(want.as_str().unwrap().to_owned()){duplicates+=1;}}}
    assert_eq!(duplicates,9);assert_eq!(seen.len(),11110);seen
}
"#;
    s.replace_range(begin..end, prior);
    let before="        if case[\"name\"] == \"1x2-w32\" {\n            case[\"rounds\"] = json!(151);\n        }";
    let after="        if case[\"name\"] == \"1x2-w32\" {\n            case[\"rounds\"] = json!(151);\n        } else if case[\"name\"] == \"1x2-w16\" {\n            case[\"rounds\"] = json!(501);\n        }";
    assert_eq!(s.matches(before).count(), 1);
    s = s.replace(before, after);
    s = s.replace(
        "json!([\"1x2-w32\", \"4x2-w32\"])",
        "json!([\"1x2-w32\", \"merge-54x1-w32\"])",
    );
    s = s
        .replace("\"research-audit.md\",", "\"integration-review.md\",")
        .replace("\"memory-review.md\",", "\"integration-review.md\",");
    s = s
        .replace(
            "assert_eq!(seen.len(), 10654);",
            "assert_eq!(seen.len(), 11119);",
        )
        .replace("\"next_prior_count\":10654", "\"next_prior_count\":11119")
        .replace("prior10645", "prior11110")
        .replace("prior10654", "prior11119");
    fs::write(r.join("prepare-sdk.rs"), s).unwrap();
    let s = fs::read_to_string(format!("{old}/analyze.rs"))
        .unwrap()
        .replace(old, r.to_str().unwrap())
        .replace("10654", "11119")
        .replace("11110", "11575")
        .replace("27624", "43374")
        .replace("27660", "43410")
        .replace(
            "parallel_exact_scalar_grouping",
            "clean_bounded_serial_fft_leaves",
        )
        .replace("ParallelScalarGroups", "CleanFFTLeaves");
    fs::write(r.join("analyze.rs"), s).unwrap();
    fs::copy(format!("{old}/stats.rs"), r.join("stats.rs")).unwrap();
    fs::copy(r.join("design.md"), r.join("sdk-design.md")).unwrap();
    println!("Prepared fixed15block support:primary16 501rounds;43410SDK465Gnark;prior11110/preflight11119/final11575");
}

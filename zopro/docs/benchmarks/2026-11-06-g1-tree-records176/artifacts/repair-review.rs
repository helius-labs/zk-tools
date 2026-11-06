use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-records176";
fn main() {
    let old = fs::read(format!("{R}/review-before-filter-count/review.json")).unwrap();
    let mut v: Value = serde_json::from_slice(&old).unwrap();
    assert_eq!(v["review_passed"], true);
    assert_eq!(v["portable_original_tests_filtered"], 125);
    let stdout = fs::read(format!("{R}/runs/tests-portable/stdout.log")).unwrap();
    let exit: Value =
        serde_json::from_slice(&fs::read(format!("{R}/runs/tests-portable/exit.json")).unwrap())
            .unwrap();
    assert_eq!(exit["returncode"], 0);
    assert_eq!(
        format!("{:x}", Sha256::digest(&stdout)),
        exit["stdout_sha256"]
    );
    let text = std::str::from_utf8(&stdout).unwrap();
    let summary = text
        .lines()
        .find(|s| s.starts_with("test result:"))
        .unwrap();
    assert_eq!(summary, v["summaries"]["portable"].as_str().unwrap());
    let filtered = summary
        .split(';')
        .find(|s| s.contains("filtered out"))
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert_eq!(filtered, 101);
    v["portable_original_tests_filtered"] = json!(filtered);
    v["native_only_original_tests_absent_from_portable_executable"] = json!(125 - filtered);
    v["derived_metadata_correction"]=json!("Original review hardcoded125 portable filtered tests; actual portable executable has101, with24native-only originals absent. Corrected only this metadata; no tests/builds/source checks or main review rerun.");
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&v).unwrap(),
    )
    .unwrap();
    fs::write(format!("{R}/review-metadata-repair.json"),serde_json::to_vec_pretty(&json!({"passed":true,"original_review_sha256":format!("{:x}",Sha256::digest(old)),"actual_portable_filtered":filtered,"absent_native_only_originals":24,"main_review_reexecuted":false,"new_tests_or_builds":0})).unwrap()).unwrap();
    let p = format!("{R}/review.rs");
    let source = fs::read_to_string(&p).unwrap();
    let field = "\"portable_original_tests_filtered\":125";
    assert_eq!(source.matches(field).count(), 1);
    let anchor = "    let result = json!";
    assert_eq!(source.matches(anchor).count(), 1);
    let source=source.replace(field,"\"portable_original_tests_filtered\":portable_filtered").replace(anchor,"    let portable_filtered = summaries[\"portable\"].split(';').find(|s| s.contains(\"filtered out\")).unwrap().split_whitespace().next().unwrap().parse::<usize>().unwrap();\n    let result = json!");
    fs::write(p, source).unwrap();
    println!("Metadata repairPASS: portable101filtered/24native-only originals absent; all earlier review evidence reused; NO native/test/build/reviewrerun");
}

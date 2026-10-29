// Offline report refresh. Does not run a benchmark or verifier.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

const ROOT: &str = "/home/ubuntu/src/zk-tools/zopro";
const SNAPSHOT: &str = "7faaeca0cb882ce636f290795c4d1d593fad39bd";

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn hash(path: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(path).unwrap()))
}
fn write(path: impl AsRef<Path>, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn main() {
    let root = Path::new(ROOT);
    let summary = root.join("docs/benchmarks/2026-10-29-current-cpu-summary");
    let report = root.join("docs/benchmarks/2026-10-29-cpu-benchmark-report");
    let mut data = read(summary.join("data.json"));
    let previous = read(report.join("data.json"));
    assert_eq!(previous["rows"], data["rows"], "accepted rows changed");
    assert_eq!(data["historical_distinct_Gnark_ledger"], 22210);
    assert_eq!(data["accepted_original8_CPU10x"], 0);
    assert_eq!(data["accepted_source_files"], 175);
    let md = fs::read_to_string(report.join("README.md")).unwrap();
    for row in data["rows"].as_array().unwrap() {
        let shape = row["shape"].as_str().unwrap();
        let local_name = if shape == "merge-54x1" { "54→1" } else { shape };
        let local = &row["local_accepted"];
        assert!(md.contains(&format!("|{local_name}|{:.3}|{:.2}|",
            local["ms"].as_f64().unwrap(), local["max_RSS_MiB"].as_f64().unwrap())));
        let aws = &row["aws_best_tested_descriptive"];
        assert!(md.contains(&format!("|{shape}|{:.3}|{}|{:.3}|{}|{:.2}|{:.2}|",
            aws["plain"]["ms"].as_f64().unwrap(), aws["plain"]["workers"],
            aws["aeglos"]["ms"].as_f64().unwrap(), aws["aeglos"]["workers"],
            aws["plain"]["max_RSS_MiB"].as_f64().unwrap(),
            aws["aeglos"]["max_RSS_MiB"].as_f64().unwrap())));
    }
    for dir in [&report, &summary, &root.join("docs/benchmarks/2026-10-29-instruction-profile175")] {
        let text = fs::read_to_string(dir.join("README.md")).unwrap();
        for tail in text.split("](").skip(1) {
            let target = tail.split(')').next().unwrap();
            if !target.contains("://") && !target.starts_with('#') {
                assert!(dir.join(target.split('#').next().unwrap()).is_file(), "missing link: {target}");
            }
        }
    }
    data["source_commit"] = json!(SNAPSHOT);
    data["snapshot_kind"] = json!("Consolidated accepted benchmarks and completed experiments, including instruction-profile175 published with this refresh; no new measurements");
    data["upstream_summary_source_commit"] = read(summary.join("data.json"))["source_commit"].clone();
    data["instruction_profile_interpretation"] = json!({
        "absolute_sample_period_coverage_validated": false,
        "period_totals_below_preceding_nonsampled_counts": true,
        "cause_of_period_difference": "unresolved; separate runs",
        "separate_event_profiles_measure_local_CPI": false,
        "sample_shares_are_exclusive_stage_costs": false,
        "new_performance_qualification": false
    });
    let pins = read(summary.join("source-hashes.json"));
    let mut sources = BTreeMap::new();
    for (path, expected) in pins.as_object().unwrap() {
        assert_eq!(hash(root.join(path)), expected.as_str().unwrap());
        sources.insert(path.clone(), expected.as_str().unwrap().to_owned());
    }
    for name in ["README.md", "data.json", "source-hashes.json", "generate.rs", "SHA256SUMS"] {
        let path = summary.join(name);
        sources.insert(path.strip_prefix(root).unwrap().display().to_string(), hash(&path));
    }
    write(report.join("data.json"), &data);
    write(report.join("source-hashes.json"), &json!(sources));
    let mut names = fs::read_dir(&report).unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file() && path.file_name().unwrap() != "SHA256SUMS")
        .collect::<Vec<_>>();
    names.sort();
    let sums = names.iter().map(|path| {
        format!("{}  {}\n", hash(path), path.file_name().unwrap().to_str().unwrap())
    }).collect::<String>();
    fs::write(report.join("SHA256SUMS"), sums).unwrap();
    println!("Refreshed report: 9 unchanged accepted rows; 8 CPU/Metal references; {} source hashes; ledger 22210; no new captures", sources.len());
}

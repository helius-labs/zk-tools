use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new(R);
    let summary = Path::new(Z).join("docs/benchmarks/2026-10-29-current-cpu-summary");
    let report = Path::new(Z).join("docs/benchmarks/2026-10-30-g2-balanced-sdk175");
    assert!(report.join("README.md").exists());
    let a = read(root.join("analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(a["independent_proof_count"], 24943);
    for f in ["README.md", "data.json", "source-hashes.json"] {
        assert_eq!(
            hash(summary.join(f)),
            hash(root.join("inherited-summary").join(f))
        );
    }
    let out=Command::new("jq").args(["--slurpfile","a"]).arg(root.join("analysis.json")).args([".latest_G2_balanced_SDK_experiment=($a[0]+{report:\"../2026-10-30-g2-balanced-sdk175/README.md\"}) | .historical_distinct_Gnark_ledger=24943"]).arg(summary.join("data.json")).output().unwrap();
    assert!(out.status.success());
    fs::write(summary.join("data.json"), out.stdout).unwrap();
    let mut md = fs::read_to_string(summary.join("README.md")).unwrap();
    md.push_str(&format!("\n\n## Balanced G2 windows in complete SDK proofs\n\nThe qualified variable-width G2 route is measured in full fresh verified requests across all9circuits at32workers and primary16, against exactaccepted and freshOFF. The31-block full-request gate is **{}**, failed checks `{}`; diagnostic qualification **{}**. Caller gains are not substituted for these proof results. All55,788requests verify internally and972fresh exports independently pass Gnark, advancing the unique proof ledger23,971→24,943. Native127core/8prover per variant and portable101core/8prover pass; field sources remain exact. Production and accepted local/AWS rows remain unchanged pending any required further qualification. [Full results, memory and profiles](../2026-10-30-g2-balanced-sdk175/README.md).\n",a["gate_passed"],a["failed_checks"],a["diagnostics_qualified"]));
    fs::write(summary.join("README.md"), md).unwrap();
    let mut pins = read(summary.join("source-hashes.json"));
    for p in [
        report.join("README.md"),
        report.join("artifacts/analysis.json"),
        report.join("artifacts/final-source-review.json"),
    ] {
        pins[p.strip_prefix(Z).unwrap().to_str().unwrap()] = json!(hash(&p));
    }
    fs::write(
        summary.join("source-hashes.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let sums = [
        "README.md",
        "data.json",
        "source-hashes.json",
        "generate.rs",
    ]
    .iter()
    .map(|p| format!("{}  {p}\n", hash(summary.join(p))))
    .collect::<String>();
    fs::write(summary.join("SHA256SUMS"), sums).unwrap();
    println!("UpdatedbalancedG2SDKrolling summary;acceptedrowsunchanged,ledger24943");
}

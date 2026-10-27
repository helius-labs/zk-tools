//! Exploratory offline instruction classification after the frozen census.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
const R: &str = "/tmp/zopro-current-pmu";
fn main() {
    let log = fs::read_to_string(format!("{R}/g1-annotation.log")).unwrap();
    assert!(log.contains("(771 samples, percent: local period)"));
    let mut classes: BTreeMap<&str, u64> = BTreeMap::new();
    let mut total = 0u64;
    let mut stack_expression = 0u64;
    for line in log.lines() {
        let Some((period, line)) = line.split_once(':') else {
            continue;
        };
        let Ok(period) = period.trim().parse::<u64>() else {
            continue;
        };
        if period == 0 {
            continue;
        }
        let (_, assembly) = line.split_once(':').unwrap();
        let asm = assembly.trim();
        let opcode = asm.split_whitespace().next().unwrap();
        let class = if opcode.starts_with("vpmadd52") {
            "IFMA"
        } else if ["vperm", "vshuf", "vpunpck", "vunpck"]
            .iter()
            .any(|p| opcode.starts_with(p))
        {
            "vector shuffle"
        } else if opcode.starts_with("vmov") {
            "vector move"
        } else if opcode.starts_with('v') {
            "other vector arithmetic/mask/shift"
        } else {
            "scalar/control/call"
        };
        *classes.entry(class).or_default() += period;
        total += period;
        if asm.contains("(%rsp") || asm.contains("(%rbp") {
            stack_expression += period;
        }
    }
    let rows: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-g1-raw-scratch/performance/rows.json").unwrap(),
    )
    .unwrap();
    let profile = rows
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["phase"] == "profile" && r["case"] == "1x2-w32" && r["variant"] == "accepted")
        .unwrap();
    let matched: Vec<_> = profile["profile"]["symbol_periods"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(name, _)| {
            name.starts_with("zopro_core::msm::stream_g1_impl") && name.contains(", true, true, &")
        })
        .collect();
    assert_eq!(matched.len(), 1);
    assert_eq!(matched[0].1.as_u64().unwrap(), total);
    let percentages: BTreeMap<_, _> = classes
        .iter()
        .map(|(c, n)| (*c, 100. * (*n as f64) / (total as f64)))
        .collect();
    let binary = "/tmp/zopro-clean-fft-leaves/native/candidate-sdk";
    let raw = "/tmp/zopro-g1-raw-scratch/performance/profile/b0/1x2-w32/accepted/perf.data";
    let sha = |p: &str| format!("{:x}", Sha256::digest(fs::read(p).unwrap()));
    assert_eq!(
        sha(binary),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let out = json!({"scope":"exploratory offline annotation of already captured exact accepted14daG1 stream; no new sampling/proofs","source_profile":raw,"source_profile_sha256":sha(raw),"source_report_commit":"a555ad9286214c28c5f4508e57aa2088d8d876de","binary":binary,"binary_sha256":sha(binary),"annotation_sha256":sha(&format!("{R}/g1-annotation.log")),"symbol":matched[0].0,"symbol_samples":771,"total_period":total,"period_matches_archived_symbol_exactly":true,"opcode_periods":classes,"opcode_percent":percentages,"explicit_rsp_rbp_address_expression_period":stack_expression,"explicit_rsp_rbp_address_expression_percent":100.*stack_expression as f64/total as f64,"limitations":["cycle sampling is not precise per-instruction latency; skid and surrounding dependencies matter","address-expression weight is not all stack traffic, spills, cache misses or removable time","opcode categories describe flat sampled positions; arithmetic and data movement are not experimentally isolated","one historical matched exact-SDK profile, not repeated or pooled into PMU census"]});
    fs::write(
        format!("{R}/instruction-audit.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

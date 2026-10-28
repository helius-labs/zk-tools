use std::fs;
fn once(s: &mut String, old: &str, new: &str) {
    assert_eq!(s.matches(old).count(), 1, "{old}");
    *s = s.replace(old, new);
}
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/scalar-square-sdk";
    let new = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor-sdk";
    let mut s = fs::read_to_string(format!("{old}/capture.rs")).unwrap();
    once(&mut s, old, new);
    once(
        &mut s,
        "\"make-capture.rs\",\"make-finish.rs\"",
        "\"make-sdk.rs\",\"make-sdk\"",
    );
    s = s.replace(
        "/zopro-perf-work/inverse-finish-sdk/",
        "/zopro-perf-work/scalar-square-sdk/",
    );
    once(
        &mut s,
        "assert_eq!(prior.len(), 12884);",
        "assert_eq!(prior.len(), 12983);",
    );
    once(
        &mut s,
        "assert_eq!(prior.len(), 12785);",
        "assert_eq!(prior.len(), 12884);",
    );
    fs::write("capture.rs", s).unwrap();
    let mut s = fs::read_to_string(format!("{old}/finish.rs")).unwrap();
    once(&mut s, old, new);
    once(
        &mut s,
        "2026-10-28-scalar-square-sdk",
        "2026-10-28-g1-weighted-anchor-sdk",
    );
    once(
        &mut s,
        "assert_eq!(seen.len(), 12983);",
        "assert_eq!(seen.len(), 13082);",
    );
    once(
        &mut s,
        "assert_eq!(seen.len(), 12884);",
        "assert_eq!(seen.len(), 12983);",
    );
    s = s
        .replace(
            "\"independent_proof_count\":12983",
            "\"independent_proof_count\":13082",
        )
        .replace("ledger12983", "ledger13082");
    once(
        &mut s,
        "s.contains(\"zopro_core::solver\")",
        "s.contains(\"stream_g1\") || s.contains(\"BatchAdd\")",
    );
    s = s
        .replace("solver_symbol_period", "point_caller_symbol_period")
        .replace(
            "\"solver_profiles\":profile_table",
            "\"point_profiles\":profile_table",
        );
    fs::write("finish.rs", s).unwrap();
    println!("Prepared mature fixedSDKcapture/analyzer for pureG1anchor;12983prior->13082after99verifiednew");
}

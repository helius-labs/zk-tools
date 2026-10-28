use std::{fs, path::Path};
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor-sdk";
    let new = "/home/ubuntu/src/zopro-perf-work/rerandomized-glv-sdk";
    let mut s = fs::read_to_string(format!("{old}/capture.rs")).unwrap();
    once(&mut s, old, new);
    s = s.replace(
        "/zopro-perf-work/scalar-square-sdk/",
        "/zopro-perf-work/g1-weighted-anchor-sdk/",
    );
    once(
        &mut s,
        "assert_eq!(prior.len(), 12983);",
        "assert_eq!(prior.len(), 13082);",
    );
    once(
        &mut s,
        "assert_eq!(prior.len(), 12884);",
        "assert_eq!(prior.len(), 12983);",
    );
    fs::write(Path::new(new).join("capture.rs"), s).unwrap();
    let mut s = fs::read_to_string(format!("{old}/finish.rs")).unwrap();
    once(&mut s, old, new);
    once(
        &mut s,
        "2026-10-28-g1-weighted-anchor-sdk",
        "2026-10-28-rerandomized-glv-sdk",
    );
    once(
        &mut s,
        "assert_eq!(seen.len(), 13082);",
        "assert_eq!(seen.len(), 13181);",
    );
    once(
        &mut s,
        "assert_eq!(seen.len(), 12983);",
        "assert_eq!(seen.len(), 13082);",
    );
    s = s
        .replace(
            "\"independent_proof_count\":13082",
            "\"independent_proof_count\":13181",
        )
        .replace("ledger13082", "ledger13181");
    once(
        &mut s,
        "s.contains(\"stream_g1\") || s.contains(\"BatchAdd\")",
        "s.contains(\"groth16\")",
    );
    s = s
        .replace("point_caller_symbol_period", "assembly_symbol_period")
        .replace(
            "\"point_profiles\":profile_table",
            "\"assembly_profiles\":profile_table",
        );
    fs::write(Path::new(new).join("finish.rs"), s).unwrap();
    let mut s = fs::read_to_string(format!("{old}/native.rs")).unwrap();
    let start = s.find("fn freeze(").unwrap();
    s = s[start..].to_string();
    s = s.replace("zopro_g1_weighted_anchor", "zopro_glv_blinding");
    let prefix = fs::read_to_string("native-prefix.rs").unwrap();
    fs::write("native.rs", format!("{prefix}\n{s}")).unwrap();
    println!(
        "Prepared mature fixedSDKcapture/analyzer/nativefreezer;13082prior->13181onlyafter99Gnark"
    );
}

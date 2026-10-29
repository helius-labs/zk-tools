use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
fn rep(s: &mut String, a: &str, b: &str) {
    assert!(s.contains(a), "missing{a}");
    *s = s.replace(a, b);
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let mut s =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/balanced-phi175/native.rs").unwrap();
    rep(&mut s, "balanced-phi175", "proof-pgo175");
    rep(
        &mut s,
        "[\"control\", \"candidate\"]",
        "[\"control\", \"generate\", \"candidate\"]",
    );
    let a = s
        .find("        fs::read_to_string(r(if label == \"candidate\"")
        .unwrap();
    let b = s[a..].find("        .unwrap()\n").unwrap() + a + "        .unwrap()\n".len();
    s.replace_range(
        a..b,
        "        fs::read_to_string(r(format!(\"{label}-flags.txt\"))).unwrap()\n",
    );
    rep(&mut s,"r(format!(\"native/libraries-{label}/stdout.log\"))","r(format!(\"{}/libraries-{label}/stdout.log\",if label==\"candidate\" {\"native-use\"}else{\"native\"}))");
    rep(
        &mut s,
        "target-{label}/release/.fingerprint",
        "target-{label}/x86_64-unknown-linux-gnu/release/.fingerprint",
    );
    rep(
        &mut s,
        "Some(\"freeze\") => freeze(&a[2]),",
        "Some(\"use\") => prepare_use(),\n        Some(\"freeze\") => freeze(&a[2]),",
    );
    fs::write(r.join("native.rs"), s).unwrap();
}

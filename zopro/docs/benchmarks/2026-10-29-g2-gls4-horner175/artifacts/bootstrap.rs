use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let oldpath=Path::new(Z).join("docs/benchmarks/2026-10-22-universal-g2-gls4/source/crates/zopro-core/src/msm_affine/g2_gls4.rs");
    let old = fs::read_to_string(&oldpath).unwrap();
    fs::copy(&oldpath, r.join("historical-gls4.rs")).unwrap();
    fs::copy(
        Path::new(Z).join("docs/benchmarks/2026-10-22-universal-g2-gls4/integer-oracle.json"),
        r.join("historical-integer-oracle.json"),
    )
    .unwrap();
    let a = old.find("const U:").unwrap();
    let b = old.find("const BETA:").unwrap();
    let c = old.find("#[inline(always)]\nfn quotient_low").unwrap();
    let d = old.find("fn transform(").unwrap();
    let mut body=format!("//! Original local GLS4 lattice and digit bodies; GnarkCrypto v0.21.0 attribution.\n//! Sources and exact bound are retained in historical-integer-oracle.json.\nuse super::digit;\n{}{}",&old[a..b],&old[c..d]);
    body = body
        .replace("fn split(", "pub(super) fn split(")
        .replace("fn component_digit(", "pub(super) fn component_digit(");
    fs::write(r.join("lattice.rs"), body).unwrap();
    let old = fs::read_to_string("/home/ubuntu/src/zopro-perf-work/fr-dot3-175/native.rs").unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let source = format!(
        "{}{}\n{}\n{}",
        &old[..a],
        fs::read_to_string(r.join("setup.rs")).unwrap(),
        fs::read_to_string(r.join("code-audit.rs")).unwrap(),
        &old[b..]
    )
    .replace("/home/ubuntu/src/zopro-perf-work/fr-dot3-175", R);
    fs::write(r.join("native.rs"), source).unwrap();
}

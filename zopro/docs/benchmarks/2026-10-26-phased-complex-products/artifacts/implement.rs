use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-phased-complex-products");
    let dst = r.join("candidate");
    let p = dst.join("crates/zopro-bn254/src/ifma.rs");
    let original = fs::read_to_string(&p).unwrap();
    let old = fs::read_to_string(
        "/tmp/zopro-complex-shared-product/overlay/crates/zopro-bn254/src/ifma.rs",
    )
    .unwrap();
    let start = old
        .find("    /// Complex product with three shared raw products")
        .unwrap();
    let end = old[start..]
        .find("    #[inline(always)]\n    fn product<const FUSED")
        .unwrap()
        + start;
    let shared=old[start..end].replace("    #[inline(always)]\n    pub fn complex_product_shared", "    #[cfg(zopro_phased_complex_products)]\n    #[inline(always)]\n    pub fn complex_product_shared");
    let anchor = "    #[inline(always)]\n    pub fn mul(&self, b: &Self) -> Self {";
    assert_eq!(original.matches(anchor).count(), 1);
    let block = fs::read_to_string(r.join("field-block.rs.txt")).unwrap();
    let added = "    // BEGIN SHARED_CIOS_CONTROL\n".to_owned()
        + &shared
        + "    // END SHARED_CIOS_CONTROL\n\n"
        + &block;
    let current = original.replacen(anchor, &(added.clone() + anchor), 1);
    assert_eq!(current.replace(&added, ""), original);
    fs::write(&p, current).unwrap();
    let o = Command::new("python3")
        .arg(dst.join("scripts/ifma8.py"))
        .output()
        .unwrap();
    fs::write(r.join("generator.stdout.log"), o.stdout).unwrap();
    fs::write(r.join("generator.stderr.log"), o.stderr).unwrap();
    assert!(o.status.success());
    let base: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap(),
    )
    .unwrap();
    let mut changed = Vec::new();
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        let got = hash(dst.join(n));
        if got != h.as_str().unwrap() {
            assert!(
                n == "crates/zopro-bn254/src/ifma.rs" || n == "crates/zopro-bn254/src/ifma8.rs"
            );
            changed.push(json!({"path":n,"sha256":got,"original_sha256":h}));
        }
    }
    assert_eq!(changed.len(), 2);
    // The generated file differs only by precisely generated inserted methods.
    let generated = fs::read_to_string(dst.join("crates/zopro-bn254/src/ifma8.rs")).unwrap();
    let mut stripped = generated.clone();
    for (label, endlabel) in [
        ("SHARED_CIOS_CONTROL", "SHARED_CIOS_CONTROL"),
        ("PHASED_COMPLEX_PRODUCTS", "PHASED_COMPLEX_PRODUCTS"),
    ] {
        let st = stripped.find(&format!("    // BEGIN {label}\n")).unwrap();
        let marker = format!("    // END {endlabel}\n\n");
        let en = stripped.find(&marker).unwrap() + marker.len();
        stripped.replace_range(st..en, "");
    }
    assert_eq!(
        stripped,
        fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/ifma8.rs")
            .unwrap()
    );
    fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changed":changed,"other_168_unchanged":true,"removing_additions_recovers_whole_original_field_files":true,"no_core_dispatch":true,"shared_control_method_exact_old_body":true})).unwrap()).unwrap();
    println!("Two field files changed by additions only; all168other source pins unchanged; no core dispatch.");
}

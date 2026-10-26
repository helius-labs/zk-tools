use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn change(s: &mut String, from: &str, to: &str, n: usize) {
    assert_eq!(s.matches(from).count(), n, "{from}");
    *s = s.replace(from, to);
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let dst = root.join("candidate");
    let p = dst.join("crates/zopro-bn254/src/ifma.rs");
    let base = fs::read_to_string(&p).unwrap();
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
    let control = &old[start..end];
    let mut new = control.to_owned();
    change(
        &mut new,
        "pub fn complex_product_shared",
        "pub fn complex_product_streamed",
        1,
    );
    change(&mut new, "            let mut b = [zero; 5];\n", "", 1);
    change(&mut new, "            let mut cb = zero;\n", "", 1);
    change(
        &mut new,
        "                let bv = _mm256_add_epi64(_mm256_add_epi64(b0.0[j], b1.0[j]), cb);\n",
        "",
        1,
    );
    change(
        &mut new,
        "                cb = _mm256_srli_epi64::<52>(bv);\n",
        "",
        1,
    );
    change(
        &mut new,
        "                b[j] = _mm256_and_si256(bv, mask);\n",
        "",
        1,
    );
    change(
        &mut new,
        "            let mut x = [zero; 6];",
        "            let mut cb = zero;\n            let mut x = [zero; 6];",
        1,
    );
    change(&mut new,"                ($i:literal) => {{\n                    macro_rules! product", "                ($i:literal) => {{\n                    // Stream the exact summed-B digit used by this row.\n                    let bv = _mm256_add_epi64(_mm256_add_epi64(b0.0[$i], b1.0[$i]), cb);\n                    cb = _mm256_srli_epi64::<52>(bv);\n                    let bi = _mm256_and_si256(bv, mask);\n                    macro_rules! product",1);
    change(&mut new, "b[$i]", "bi", 2);
    let cfg = "    #[cfg(zopro_streamed_complex_b)]\n    #[inline(always)]\n    pub fn ";
    let control = control.replacen("    #[inline(always)]\n    pub fn ", cfg, 1);
    new = new.replacen("    #[inline(always)]\n    pub fn ", cfg, 1);
    let added = "    // BEGIN STREAMED_COMPLEX_CONTROL\n".to_owned()
        + &control
        + "    // END STREAMED_COMPLEX_CONTROL\n\n    // BEGIN STREAMED_COMPLEX_B\n"
        + &new
        + "    // END STREAMED_COMPLEX_B\n\n";
    let anchor = "    #[inline(always)]\n    pub fn mul(&self, b: &Self) -> Self {";
    assert_eq!(base.matches(anchor).count(), 1);
    let current = base.replacen(anchor, &(added.clone() + anchor), 1);
    assert_eq!(current.replace(&added, ""), base);
    fs::write(&p, current).unwrap();
    fs::write(root.join("field-additions.rs.txt"), added).unwrap();
    let output = Command::new("python3")
        .arg(dst.join("scripts/ifma8.py"))
        .output()
        .unwrap();
    fs::write(root.join("generator.stdout.log"), output.stdout).unwrap();
    fs::write(root.join("generator.stderr.log"), output.stderr).unwrap();
    assert!(output.status.success());
    let manifest: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap(),
    )
    .unwrap();
    let mut changes = Vec::new();
    for (name, digest) in manifest["source_files_sha256"].as_object().unwrap() {
        let h = hash(dst.join(name));
        if h != digest.as_str().unwrap() {
            assert!(
                name == "crates/zopro-bn254/src/ifma.rs"
                    || name == "crates/zopro-bn254/src/ifma8.rs"
            );
            changes.push(json!({"path":name,"sha256":h,"original_sha256":digest}));
            let mut text = fs::read_to_string(dst.join(name)).unwrap();
            for label in ["STREAMED_COMPLEX_CONTROL", "STREAMED_COMPLEX_B"] {
                let start = text.find(&format!("    // BEGIN {label}\n")).unwrap();
                let marker = format!("    // END {label}\n\n");
                let end = text.find(&marker).unwrap() + marker.len();
                text.replace_range(start..end, "");
            }
            assert_eq!(
                text,
                fs::read_to_string(Path::new("/home/ubuntu/src/zk-tools/zopro").join(name))
                    .unwrap()
            );
        }
    }
    assert_eq!(changes.len(), 2);
    fs::write(root.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changed":changes,"other_168_unchanged":true,"removing_additions_recovers_original_whole_files":true,"no_core_dispatch":true,"old_shared_method_body_exact":true,"streamed_change":"Only exactB-sum construction location and lifetime; all product/cancellation expressions unchanged."})).unwrap()).unwrap();
    let mut probe = fs::read_to_string("/tmp/zopro-phased-complex-products/probe.rs").unwrap();
    probe = probe
        .replace("zopro_phased_complex_products", "zopro_streamed_complex_b")
        .replace("complex_product_phased", "complex_product_streamed")
        .replace("complex_phased_codegen", "complex_streamed_codegen")
        .replace("phased", "streamed");
    fs::write(root.join("probe.rs"), probe).unwrap();
    fs::create_dir(root.join("oracle")).unwrap();
    for name in ["Cargo.toml", "Cargo.lock"] {
        let text =
            fs::read_to_string(Path::new("/tmp/zopro-phased-complex-products/oracle").join(name))
                .unwrap()
                .replace("phased-complex-probe", "streamed-complex-probe");
        fs::write(root.join("oracle").join(name), text).unwrap();
    }
    println!("Inserted streamed-B and exact shared control only; original168files unchanged, both whole field originals recover by removal");
}

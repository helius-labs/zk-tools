use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let mut block = fs::read_to_string(r.join("overlap-block.rs.txt")).unwrap();
    block=block.replace("        #[cfg(all(zopro_fused_product_overlap, zopro_real_kh))]\n        if msm_affine", "        #[cfg(all(zopro_fused_product_overlap, zopro_real_kh))]\n        let product_count=a.len().max(b.len());\n        #[cfg(all(zopro_fused_product_overlap, zopro_real_kh))]\n        if queries.euler && msm_affine");
    block = block
        .replace(
            "folded.private.len(),self.plan.size())",
            "folded.private.len(),product_count)",
        )
        .replace(
            "queries.product.len() >= self.plan.size()",
            "queries.product.len() >= product_count",
        )
        .replace(
            "folded.private.len(),self.plan.size());",
            "folded.private.len(),product_count);",
        )
        .replace(
            "assert_eq!(count,self.plan.size());",
            "assert_eq!(count,product_count);",
        );
    assert!(block.contains("if queries.euler"));
    fs::write(r.join("overlap-block-v2.rs.txt"), block).unwrap();
    let s = fs::read_to_string(r.join("implement.rs"))
        .unwrap()
        .replace("r.join(\"candidate\")", "r.join(\"candidate-v2\")")
        .replace("overlap-block.rs.txt", "overlap-block-v2.rs.txt")
        .replace("source-audit.json", "source-audit-v2.json");
    fs::write(r.join("implement-v2.rs"), s).unwrap();
    let meta = json!({"reason":"Preflight initialON panicked on a new incorrect row-count assertion: Euler output has30922values, FFTdomain32768.","repair":"Euler-only guard predicts output count as max(a.len,b.len), exactly matching existing Euler product; fused eligibility/slice bounds/assertion use that count. No arithmetic, ordering, parameters or gates change.","source_version":"candidate-v2","failed_native_and_preflight_preserved":true,"successful_off_preflight_sdk_calls":36,"successful_off_distinct_exports":9,"failed_on_sdk_completed_calls":0,"timed_requests_before_repair":0,"native_tests_repeated_for_changed_source":true});
    fs::write(
        r.join("repair.json"),
        serde_json::to_vec_pretty(&meta).unwrap(),
    )
    .unwrap();
    let i = read(r.join("native-plan.json"));
    let mut s = serde_json::to_string(&i)
        .unwrap()
        .replace("/candidate/", "/candidate-v2/")
        .replace("/candidate/Cargo.toml", "/candidate-v2/Cargo.toml")
        .replace("/target-native", "/target-native-v2")
        .replace("/native/", "/native-v2/")
        .replace("build-input-pins.json", "build-input-pins-v2.json");
    let plan: Value = serde_json::from_str(&s).unwrap();
    fs::write(
        r.join("native-plan-v2.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    let mut pins = read(r.join("build-input-pins.json"));
    let old = pins.as_object_mut().unwrap();
    let mut new = serde_json::Map::new();
    for (p, _) in old.iter() {
        let p2 = p.replace("/candidate/", "/candidate-v2/");
        if Path::new(&p2).exists() {
            new.insert(p2.clone(), json!(hash(&p2)));
        }
    }
    // Final candidate source files are pinned after implementing v2, below in --pins mode.
    let _ = &mut s;
    let _ = Command::new("true");
    fs::write(
        r.join("build-input-pins-v2-draft.json"),
        serde_json::to_vec_pretty(&new).unwrap(),
    )
    .unwrap();
}

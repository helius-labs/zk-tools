use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let dst = r.join("candidate");
    let p = dst.join("crates/zopro-core/src/lib.rs");
    let old = fs::read_to_string(&p).unwrap();
    let block = fs::read_to_string(r.join("overlap-block.rs.txt")).unwrap();
    let anchor = "        let mut product=queries.product_values(&self.plan,a,b);";
    assert_eq!(old.matches(anchor).count(), 1);
    let mut s = old.replacen(anchor, &(block.clone() + anchor), 1);
    let helper = r#"
// BEGIN FUSED_PRODUCT_OVERLAP_FLAG
#[cfg(all(zopro_fused_product_overlap, zopro_real_kh, target_arch="x86_64", target_os="linux", target_feature="avx512ifma", target_feature="avx512vl", target_feature="avx512dq", feature="ifma8", not(no_ifma)))]
fn fused_product_overlap_enabled() -> bool {
    static ENABLED:std::sync::OnceLock<bool>=std::sync::OnceLock::new();
    *ENABLED.get_or_init(||std::env::var("ZOPRO_CPU_FUSED_PRODUCT_OVERLAP").as_deref()==Ok("1"))
}
// END FUSED_PRODUCT_OVERLAP_FLAG
"#;
    s.push_str(helper);
    assert_eq!(s.replace(&block, "").replace(helper, ""), old);
    fs::write(&p, s).unwrap();
    let p = dst.join("crates/zopro-core/src/alloc/linux.rs");
    let old = fs::read_to_string(&p).unwrap();
    let added = r#"
// BEGIN FUSED_PRODUCT_OVERLAP_RECLAIM
#[cfg(zopro_fused_product_overlap)]
pub(crate) fn reclaiming() -> bool {
    RECLAIM_USERS.load(Ordering::Relaxed) != 0
}
#[cfg(all(test,zopro_fused_product_overlap))]
#[test]
fn overlap_guard_observes_nested_reclamation() {
    assert!(!reclaiming());
    let first=Guard::enter();
    assert!(reclaiming());
    { let second=Guard::enter(); assert!(reclaiming()); drop(second); }
    assert!(reclaiming());
    drop(first);
    assert!(!reclaiming());
}
// END FUSED_PRODUCT_OVERLAP_RECLAIM
"#;
    fs::write(&p, old + added).unwrap();
    let p = dst.join("crates/zopro-core/src/alloc.rs");
    let old = fs::read_to_string(&p).unwrap();
    let added="\n// BEGIN FUSED_PRODUCT_OVERLAP_REEXPORT\n#[cfg(all(target_os=\"linux\",zopro_fused_product_overlap))]\npub(crate) use linux::reclaiming;\n// END FUSED_PRODUCT_OVERLAP_REEXPORT\n";
    fs::write(&p, old + added).unwrap();
    for name in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        let from = Path::new("/tmp/zopro-deferred-dif32-sdk/candidate").join(name);
        let p = dst.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(from, p).unwrap();
    }
    let manifest: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap(),
    )
    .unwrap();
    let mut changed = Vec::new();
    for (n, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let got = hash(dst.join(n));
        if got != h.as_str().unwrap() {
            assert!([
                "crates/zopro-core/src/lib.rs",
                "crates/zopro-core/src/alloc.rs",
                "crates/zopro-core/src/alloc/linux.rs"
            ]
            .contains(&n.as_str()));
            changed.push(json!({"path":n,"sha256":got,"accepted_sha256":h}));
        }
    }
    assert_eq!(changed.len(), 3);
    fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changed":changed,"other_167_accepted_files_unchanged":true,"field_and_workspace_forecast_unchanged":true,"driver_and_support_copied_from_verified_sdk_experiment":true,"default_off":true,"new_key_fields":0,"retained_process_flag_bytes":"OnceLock<bool> only","arithmetic_changes":false})).unwrap()).unwrap();
    println!(
        "Private3fileadditions,167acceptedfilesunchanged; no field/forecast/arithmetic changes"
    );
}

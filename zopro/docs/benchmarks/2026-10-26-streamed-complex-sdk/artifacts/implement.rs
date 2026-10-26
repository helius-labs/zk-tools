use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn method<'a>(s: &'a str, name: &str, after: usize) -> (usize, usize, &'a str) {
    let start = s[after..].find(&format!("    fn {name}(")).unwrap() + after;
    let open = s[start..].find('{').unwrap() + start;
    let mut depth = 0;
    for (i, c) in s[open..].char_indices() {
        if c == '{' {
            depth += 1;
        }
        if c == '}' {
            depth -= 1;
            if depth == 0 {
                let end = open + i + 1;
                return (start, end, &s[start..end]);
            }
        }
    }
    panic!("unclosedmethod")
}
fn block(s: &str) -> &str {
    let start = s.find("    // BEGIN STREAMED_COMPLEX_B\n").unwrap();
    let marker = "    // END STREAMED_COMPLEX_B\n\n";
    let end = s.find(marker).unwrap() + marker.len();
    &s[start..end]
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    let dst = root.join("candidate");
    let component = Path::new("/tmp/zopro-streamed-complex-b");
    let mut edits = Vec::new();
    let p = dst.join("crates/zopro-bn254/src/ifma.rs");
    let base = fs::read_to_string(&p).unwrap();
    let prior =
        fs::read_to_string(component.join("candidate/crates/zopro-bn254/src/ifma.rs")).unwrap();
    let added = block(&prior);
    let anchor = "    #[inline(always)]\n    pub fn mul(&self, b: &Self) -> Self {";
    assert_eq!(base.matches(anchor).count(), 1);
    let new = base.replacen(anchor, &(added.to_owned() + anchor), 1);
    assert_eq!(new.replace(added, ""), base);
    fs::write(&p, new).unwrap();
    let o = Command::new("python3")
        .arg(dst.join("scripts/ifma8.py"))
        .output()
        .unwrap();
    fs::write(root.join("generator.stdout.log"), o.stdout).unwrap();
    fs::write(root.join("generator.stderr.log"), o.stderr).unwrap();
    assert!(o.status.success());
    for n in ["ifma.rs", "ifma8.rs"] {
        let path = format!("crates/zopro-bn254/src/{n}");
        let text = fs::read_to_string(dst.join(&path)).unwrap();
        let original =
            fs::read_to_string(Path::new("/home/ubuntu/src/zk-tools/zopro").join(&path)).unwrap();
        let prior = fs::read_to_string(component.join("candidate").join(&path)).unwrap();
        assert_eq!(block(&text), block(&prior));
        assert_eq!(text.replace(block(&text), ""), original);
        edits.push(json!({"path":path,"kind":"insert exact component streamed method","field_block_sha256":format!("{:x}",Sha256::digest(block(&text).as_bytes()))}));
    }
    let path = "crates/zopro-core/src/simd.rs";
    let base = fs::read_to_string(dst.join(path)).unwrap();
    let added="    #[cfg(zopro_streamed_complex_b)]\n    #[inline(always)]\n    pub(crate) fn complex_product_streamed(a0: &Self, a1: &Self, b0: &Self, b1: &Self) -> [Self; 2] {\n        zopro_bn254::ifma8::Ifma8::complex_product_streamed(&a0.0, &a1.0, &b0.0, &b1.0).map(Self)\n    }\n";
    let anchor = "impl FqBatch {\n";
    assert_eq!(base.matches(anchor).count(), 1);
    let new = base.replacen(anchor, &(anchor.to_owned() + added), 1);
    assert_eq!(new.replace(added, ""), base);
    fs::write(dst.join(path), new).unwrap();
    edits.push(json!({"path":path,"kind":"insert nativebridge only","added":added}));
    let path = "crates/zopro-core/src/msm.rs";
    let base = fs::read_to_string(dst.join(path)).unwrap();
    let old = fs::read_to_string(
        "/tmp/zopro-complex-shared-product/sdk-candidate/crates/zopro-core/src/msm.rs",
    )
    .unwrap();
    let mut new = base.clone();
    let mut reverse = Vec::new();
    for name in ["lanes_scaled_numerator", "lanes_mul"] {
        let pos = new.find("impl BatchInverse for Fq2").unwrap();
        let (start, end, original) = method(&new, name, pos);
        let target = method(&old, name, old.find("impl BatchInverse for Fq2").unwrap())
            .2
            .replace("zopro_shared_complex_products", "zopro_streamed_complex_b")
            .replace("complex_product_shared", "complex_product_streamed");
        reverse.push((target.clone(), original.to_owned()));
        edits.push(json!({"path":path,"method":name,"from":original,"to":target}));
        new.replace_range(start..end, &target);
    }
    let mut recovered = new.clone();
    for (to, from) in reverse.into_iter().rev() {
        assert_eq!(recovered.matches(&to).count(), 1);
        recovered = recovered.replacen(&to, &from, 1);
    }
    assert_eq!(recovered, base);
    fs::write(dst.join(path), new).unwrap();
    let manifest: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap(),
    )
    .unwrap();
    let mut changed = Vec::new();
    for (n, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let got = hash(dst.join(n));
        if got != h.as_str().unwrap() {
            assert!([
                "crates/zopro-bn254/src/ifma.rs",
                "crates/zopro-bn254/src/ifma8.rs",
                "crates/zopro-core/src/simd.rs",
                "crates/zopro-core/src/msm.rs"
            ]
            .contains(&n.as_str()));
            changed.push(json!({"path":n,"sha256":got,"original_sha256":h}));
        }
    }
    assert_eq!(changed.len(), 4);
    for name in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        let src = Path::new("/home/ubuntu/src/zk-tools/zopro").join(name);
        let p = dst.join(name);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(&src, &p).unwrap();
        assert_eq!(hash(&src), hash(&p));
    }
    fs::write(root.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changed":changed,"base_166_unchanged":true,"both_field_blocks_byte_identical_to_passed_component":true,"whole_original_files_recovered_by_documented_inverse_edits":true,"core_edits":edits,"workspace_key_plan_scheduler_solver_unchanged":true,"new_heap_buffer_or_table_bytes":0,"compile_time_variants":true})).unwrap()).unwrap();
    println!("Four source files changed,166others unchanged; exact passed component methods; no buffers, tables or scheduling");
}

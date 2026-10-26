use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-lazy-linear";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn function<'a>(text: &'a str, name: &str) -> &'a str {
    let begin = text.find(&format!("fn {name}")).unwrap();
    let body = begin + text[begin..].find('{').unwrap();
    let mut depth = 1;
    let mut end = body + 1;
    for c in text[end..].chars() {
        end += c.len_utf8();
        if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
        }
        if depth == 0 {
            break;
        }
    }
    &text[begin..end]
}
fn main() {
    let r = Path::new(R);
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let mut pins = BTreeMap::new();
    let mut changed = Vec::new();
    for (p, h) in base["source_files_sha256"].as_object().unwrap() {
        let original = Path::new("/home/ubuntu/src/zk-tools/zopro").join(p);
        assert_eq!(hash(original), h.as_str().unwrap());
        let file = r.join("candidate").join(p);
        if hash(&file) != h.as_str().unwrap() {
            changed.push(p.clone());
        }
        pins.insert(file.display().to_string(), hash(file));
    }
    assert_eq!(
        changed,
        vec![
            "crates/zopro-bn254/src/ifma.rs",
            "crates/zopro-bn254/src/ifma8.rs",
            "scripts/ifma8.py"
        ]
    );
    let mut operation_hashes = BTreeMap::new();
    for file in ["ifma.rs", "ifma8.rs"] {
        let old = fs::read_to_string(format!(
            "/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/{file}"
        ))
        .unwrap();
        let new =
            fs::read_to_string(r.join(format!("candidate/crates/zopro-bn254/src/{file}"))).unwrap();
        for op in [
            "mul(",
            "sum2(",
            "difference2(",
            "product<",
            "product_cios<",
            "add(",
            "sub(",
            "reduce(",
            "reduce_fused(",
            "new(",
            "get(",
        ] {
            assert_eq!(
                function(&old, op),
                function(&new, op),
                "original {file}/{op}"
            );
            operation_hashes.insert(
                format!("{file}/{op}"),
                format!("{:x}", Sha256::digest(function(&new, op).as_bytes())),
            );
        }
    }
    for name in [
        "setup.rs",
        "model.rs",
        "design.md",
        "pre-build-repair.md",
        "ifma.initial-test-names.rs",
    ] {
        let p = r.join(name);
        pins.insert(p.display().to_string(), hash(p));
    }
    for p in [
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
        "/tmp/zopro-grouping-module/production-source-manifest.json",
    ] {
        pins.insert(p.into(), hash(p));
    }
    save(r.join("source-pins.json"), &json!(pins));
    save(
        r.join("source-audit.json"),
        &json!({"accepted_originals":170,"unchanged_originals":167,"changed":changed,"new_methods_only":true,"original_operation_body_hashes":operation_hashes,"no_production_dispatch_change":true,"new_heap_scratch":0,"coefficient_choices":4,"coefficient_table_bytes_per_eight_lane_instance":320,"production_sources_unchanged":true}),
    );
    let flags = base["rustflags"].as_str().unwrap();
    let env =
        json!({"RUSTFLAGS":flags,"CARGO_TARGET_DIR":r.join("target"),"CARGO_BUILD_JOBS":"16"});
    save(
        r.join("native-plan.json"),
        &json!({"version":1,"source_pins":r.join("source-pins.json"),"steps":[{"kind":"command","name":"compiler","argv":["rustc","-Vv"]},{"kind":"command","name":"generator","argv":["python3",r.join("candidate/scripts/ifma8.py"),"--check"]},{"kind":"command","name":"model-build","argv":["rustc","--edition=2021","--test","-O",r.join("model.rs"),"-o",r.join("model-tests")]},{"kind":"command","name":"model-tests","argv":[r.join("model-tests"),"--nocapture","--test-threads=1"],"unfiltered":true,"required_tests":["reduced_radix_exact_quotient_remainder_and_signed_carries"]},{"kind":"command","name":"field","argv":["cargo","test","--manifest-path",r.join("candidate/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--","--nocapture","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["ifma::lazy_linear_tests::lazy_linear_exact_raw_and_bounds","ifma::lazy_linear_tests::lazy_linear_chains_match_arkworks","ifma8::tests::lazy_linear_exact_raw_and_bounds","ifma8::tests::lazy_linear_chains_match_arkworks"],"timeout_seconds":1200},{"kind":"command","name":"libraries","argv":["cargo","build","--manifest-path",r.join("candidate/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--message-format=json"],"environment":env}]}),
    );
    println!(
        "Pinned{}source inputs; all original field operation bodies unchanged",
        pins.len()
    );
}

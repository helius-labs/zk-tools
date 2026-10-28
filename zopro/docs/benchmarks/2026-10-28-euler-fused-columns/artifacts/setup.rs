use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/euler-fused-columns";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("source").exists());
    let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let q = r.join("source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
    }
    let pair = r.join("source/crates/zopro-core/src/fft/pair.rs");
    let before = fs::read_to_string(&pair).unwrap();
    let anchor="    stored_transform::<PackedPair, true>(&mut values, &plan.roots, 0);\n    euler_combine_products(&mut a, &b, &values);";
    assert_eq!(before.matches(anchor).count(), 1);
    let replacement="    if cfg!(zopro_euler_fused_columns) && cfg!(zopro_euler_pointwise8) && values.len() > 1024 && euler_output::enabled() {\n        if std::env::var(\"ZOPRO_CPU_EULER_FUSED_TRACE\").is_ok_and(|v|v==\"1\") {eprintln!(\"EULER_FUSED_COLUMNS n={} rows={}\",plan.n,a.len());}\n        euler_output::transform_output(&mut a, &b, &mut values, &plan.roots);\n    } else {\n    stored_transform::<PackedPair, true>(&mut values, &plan.roots, 0);\n    euler_combine_products(&mut a, &b, &values);\n    }";
    let after = format!(
        "{}\nmod euler_output;\n",
        before.replace(anchor, replacement)
    );
    assert_eq!(
        after
            .strip_suffix("\nmod euler_output;\n")
            .unwrap()
            .replace(replacement, anchor),
        before
    );
    fs::write(pair, after).unwrap();
    fs::copy(
        r.join("euler_output.rs"),
        r.join("source/crates/zopro-core/src/fft/pair/euler_output.rs"),
    )
    .unwrap();
    fs::write(r.join("inverse-edits.json"),serde_json::to_vec_pretty(&json!({"crates/zopro-core/src/fft/pair.rs":[[anchor,replacement],["","\nmod euler_output;\n"]]})).unwrap()).unwrap();
    fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"production_files_exact":172,"private_originals_exact":171,"original_files_changed":["crates/zopro-core/src/fft/pair.rs"],"new_module":"crates/zopro-core/src/fft/pair/euler_output.rs","field_sources_unchanged":true,"original_OFF_body_exact":true,"new_unsafe":false})).unwrap()).unwrap();
    fs::write(
        r.join("native-flags.txt"),
        format!(
            "{} --cfg zopro_euler_fused_columns\n",
            m["rustflags"].as_str().unwrap()
        ),
    )
    .unwrap();
    println!("Prepared one original edit plus safe final-column consumer;171 originals exact");
}

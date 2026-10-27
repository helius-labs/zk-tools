use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-g2-scalar-grain");
    let m = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    let mut pins = BTreeMap::new();
    for name in ["fresh", "candidate"] {
        for (f, h) in read(r.join(name).join(".xtask/source-pins.json"))
            .as_object()
            .unwrap()
        {
            assert_eq!(hash(f), h.as_str().unwrap());
            pins.insert(f.clone(), h.as_str().unwrap().to_owned());
        }
    }
    for (field, sha) in [
        ("standard_cli_source", "standard_cli_source_sha256"),
        ("standard_cli_helper", "standard_cli_helper_sha256"),
    ] {
        let f = m[field].as_str().unwrap();
        assert_eq!(hash(f), m[sha].as_str().unwrap());
        pins.insert(f.into(), hash(f));
    }
    save(
        r.join("preparation-failure.json"),
        &json!({"stage":"before workflow construction/build/test/SDK","error":"initial helper expected standard_cli_support_source instead of actual standard_cli_helper manifest field","repair":"finish source pins and workflow using unchanged completed snapshots; no source, build or successful test repeated","production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("finish-native-plan.rs"),
        r.join("edits.json"),
        r.join("source-audit.json"),
        r.join("preparation-failure.json"),
        Path::new("/tmp/zopro-composed-adoption/production-source-manifest.json").into(),
        Path::new("/tmp/zopro-xtask-fast-screen-target/release/xtask").into(),
    ] {
        pins.insert(f.display().to_string(), hash(&f));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let text = fs::read_to_string("/tmp/zopro-grouping-prefix-cutoff/native-plan-v2.json")
        .unwrap()
        .replace(
            "/tmp/zopro-grouping-prefix-cutoff",
            "/tmp/zopro-g2-scalar-grain",
        )
        .replace("/native-v2/", "/native/")
        .replace("source-pins-v2.json", "source-pins.json");
    let mut plan: Value = serde_json::from_str(&text).unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["kind"] == "command" {
            if let Some(t) = step
                .get_mut("required_tests")
                .and_then(|v| v.as_array_mut())
            {
                t.retain(|x| {
                    x!="msm_affine::grouping::tests::prefix_cutoff_matches_independent_ordered_map"
                });
            }
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Repaired manifest helper field; original snapshots intact; native workflow ready");
}

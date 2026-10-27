use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-tree-kernel-split";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn compact(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
fn main() {
    let r = Path::new(R);
    let parent = Path::new("/tmp/zopro-adaptive-tree/candidate");
    let m = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        assert_eq!(hash(r.join("candidate").join(p)), hash(parent.join(p)));
    }
    let path = r.join("candidate/crates/zopro-core/src/msm.rs");
    let original = fs::read_to_string(&path).unwrap();
    let mut text = original.clone();
    let start = text.find("fn reduce_layer<C: CurveConfig>(").unwrap();
    let a = text[start..].find("    type F<C> =").unwrap() + start;
    let b = text[a..]
        .find("    // All vector inputs were captured")
        .unwrap()
        + a;
    let arithmetic = text[a..b].to_string();
    let predicate = "    let use_prefix = F::<C>::TREE_PREFIX && m >= 32;\n";
    assert_eq!(arithmetic.matches(predicate).count(), 1);
    let body = arithmetic
        .replace(predicate, "")
        .replace("use_prefix", "PREFIX");
    let dispatch="    if C::Base::TREE_PREFIX && s.pairs.len() >= 32 {\n        pairs::apply::<C, true>(points, s);\n    } else {\n        pairs::apply::<C, false>(points, s);\n    }\n";
    text.replace_range(a..b, dispatch);
    let module = "\nmod pairs;\n";
    text.push_str(module);
    assert_eq!(
        text.strip_suffix(module)
            .unwrap()
            .replace(dispatch, &arithmetic),
        original
    );
    fs::write(&path, text).unwrap();
    let helper=format!("use super::*;\n\n/// Apply ordinary disjoint pairs; selection and exceptional compaction stay in the caller.\npub(super) fn apply<C: CurveConfig, const PREFIX: bool>(\n    points: &mut [LimbPoint<C::Base>],\n    s: &mut Scratch<C>,\n) where C::Base: BatchInverse,\n{{\n{body}}}\n");
    let helper_path = r.join("candidate/crates/zopro-core/src/msm/pairs.rs");
    fs::write(&helper_path, &helper).unwrap();
    assert_eq!(
        compact(&helper[helper.find("    type F<C> =").unwrap()..helper.rfind('}').unwrap()]),
        compact(&body)
    );
    save(
        r.join("edits.json"),
        &json!({"parent_path":parent.join("crates/zopro-core/src/msm.rs"),"parent_sha256":hash(parent.join("crates/zopro-core/src/msm.rs")),"arithmetic_old":arithmetic,"dispatch_new":dispatch,"appended_module":module,"approved_helper_source":helper}),
    );
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_candidate_originals":169,"changed":["crates/zopro-core/src/msm.rs"],"new_module":"crates/zopro-core/src/msm/pairs.rs","parent_caller_bytes_recovered":true,"arithmetic_tokens_exact_except_const_predicate":true,"threshold_unchanged":32,"no_inline_attributes":true,"no_new_allocations_or_storage":true,"field_source_unchanged":true,"production_changed":false}),
    );
    let mut pins = BTreeMap::new();
    for (p, _) in m["source_files_sha256"].as_object().unwrap() {
        let f = r.join("candidate").join(p);
        pins.insert(f.display().to_string(), hash(f));
    }
    for name in [
        "candidate/crates/zopro-core/src/msm/pairs.rs",
        "setup.rs",
        "design.md",
        "edits.json",
        "source-audit.json",
    ] {
        let f = r.join(name);
        pins.insert(f.display().to_string(), hash(f));
    }
    for path in [
        "/tmp/zopro-adaptive-tree/source-audit.json",
        "/tmp/zopro-adaptive-tree/candidate/crates/zopro-core/src/msm.rs",
        "/tmp/zopro-adaptive-tree/codegen-audit.json",
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
    ] {
        pins.insert(path.to_string(), hash(path));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let mut p: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-adaptive-tree/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-adaptive-tree", R),
    )
    .unwrap();
    for step in p["steps"].as_array_mut().unwrap() {
        if step["name"] == "adaptive" {
            step["name"] = json!("split");
        }
    }
    save(r.join("native-plan.json"), &p);
    println!("Frozen same32pairrule with const-specialized ordinary-pair module;111coreexpected;no source math/storage change");
}

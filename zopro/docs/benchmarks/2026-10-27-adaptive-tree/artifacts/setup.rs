use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-adaptive-tree";
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
    let r = Path::new(R);
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let prior = Path::new("/tmp/zopro-g2-tree-records/candidate");
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        assert_eq!(hash(r.join("candidate").join(p)), hash(prior.join(p)));
    }
    let fit = read("/tmp/zopro-tree-record-crossover/analysis.json");
    assert_eq!(fit["component_advances"], true);
    assert_eq!(fit["training_threshold_actual_pairs"], 32);
    let path = r.join("candidate/crates/zopro-core/src/msm.rs");
    let mut text = fs::read_to_string(&path).unwrap();
    let original = text.clone();
    let a = text.find("fn reduce_layer<C: CurveConfig>(").unwrap();
    let b = text[a..].find("// The inverses of four chains").unwrap() + a;
    let old = text[a..b].to_string();
    assert_eq!(old.matches("F::<C>::TREE_PREFIX").count(), 3);
    let new = old
        .replace(
            "    let m = s.pairs.len();",
            "    let m = s.pairs.len();\n    let use_prefix = F::<C>::TREE_PREFIX && m >= 32;",
        )
        .replace("if !F::<C>::TREE_PREFIX", "if !use_prefix")
        .replace("if F::<C>::TREE_PREFIX", "if use_prefix");
    text.replace_range(a..b, &new);
    let old_test="                    if Fq2::TREE_PREFIX {assert_eq!(scratch.groups.capacity(),0);}\n                    else {assert_eq!(scratch.prefix_records.capacity(),0);}";
    let new_test="                    if !Fq2::TREE_PREFIX {assert_eq!(scratch.prefix_records.capacity(),0);}";
    assert_eq!(text.matches(old_test).count(), 1);
    text = text.replace(old_test, new_test);
    let tests = fs::read_to_string(r.join("adaptive-tests.rs")).unwrap();
    text.push_str(&tests);
    assert_eq!(
        text.strip_suffix(&tests)
            .unwrap()
            .replace(new_test, old_test)
            .replace(&new, &old),
        original
    );
    fs::write(&path, text).unwrap();
    save(
        r.join("edits.json"),
        &json!({"parent_source":"/tmp/zopro-g2-tree-records/candidate/crates/zopro-core/src/msm.rs","parent_sha256":hash(prior.join("crates/zopro-core/src/msm.rs")),"old_layer":old,"new_layer":new,"old_test_assertion":old_test,"new_test_assertion":new_test,"appended_tests":tests}),
    );
    let changed: Vec<_> = manifest["source_files_sha256"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(p, h)| hash(r.join("candidate").join(p)) != h.as_str().unwrap())
        .map(|(p, _)| p)
        .collect();
    assert_eq!(changed, vec!["crates/zopro-core/src/msm.rs"]);
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_candidate_originals":169,"changed":changed,"parent_bytes_recovered":true,"parent_accepted_recovery_audit":"/tmp/zopro-g2-tree-records/implementation-audit.json","threshold_fixed_by_prior_training_and_heldout":32,"field_source_unchanged":true,"arithmetic_formulas_unchanged":true,"one_per_layer_structural_guard":true,"no_new_runtime_experiment_selector":true,"no_circuit_core_selector":true,"no_component_wrapper_or_probe":true,"production_changed":false}),
    );
    let mut pins = BTreeMap::new();
    for (p, _) in manifest["source_files_sha256"].as_object().unwrap() {
        let p = r.join("candidate").join(p);
        pins.insert(p.display().to_string(), hash(p));
    }
    for name in [
        "setup.rs",
        "adaptive-tests.rs",
        "design.md",
        "edits.json",
        "source-audit.json",
    ] {
        let p = r.join(name);
        pins.insert(p.display().to_string(), hash(p));
    }
    for p in [
        "/tmp/zopro-tree-record-crossover/analysis.json",
        "/tmp/zopro-tree-record-crossover/training-rule.json",
        "/tmp/zopro-g2-tree-records/implementation-audit.json",
        "/tmp/zopro-g2-tree-records/candidate/crates/zopro-core/src/msm.rs",
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
    ] {
        pins.insert(p.to_string(), hash(p));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let mut plan: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-g2-tree-records/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-g2-tree-records", R),
    )
    .unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["name"] == "core" {
            step["required_tests"]=json!(["msm::adaptive_tree_tests::adaptive_boundaries_preserve_outputs_and_bound_retained_record_capacity","msm::tree_prefix_tests::prefix_tree_complete_pairs_tails_and_reuse"]);
        }
        if step["name"] == "tree" {
            step["name"] = json!("adaptive");
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Frozen adaptive m>=32 G2 tree;oneoriginal changed;111coreexpected;no timingyet");
}

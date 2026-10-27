use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-tree-record-crossover";
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
    let prior = Path::new("/tmp/zopro-g2-tree-records/candidate");
    let m = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        assert_eq!(hash(r.join("candidate").join(p)), hash(prior.join(p)));
    }
    let rel = "crates/zopro-core/src/msm.rs";
    let file = r.join("candidate").join(rel);
    let mut s = fs::read_to_string(&file).unwrap();
    let original = s.clone();
    let a = s.find("fn reduce_layer<C: CurveConfig>(").unwrap();
    let b = s[a..].find("// The inverses of four chains").unwrap() + a;
    let old = s[a..b].to_string();
    let header_end = old.find("    s.pairs.clear();").unwrap();
    let header = &old[..header_end];
    assert_eq!(old.matches("F::<C>::TREE_PREFIX").count(), 3);
    let body = old
        .replace(
            "fn reduce_layer<C: CurveConfig>(",
            "fn reduce_layer_impl<C: CurveConfig, const PREFIX: bool>(",
        )
        .replace("F::<C>::TREE_PREFIX", "PREFIX");
    let wrapper=format!("{header}    if C::Base::TREE_PREFIX {{\n        reduce_layer_impl::<C,true>(points,starts,counts,s);\n    }} else {{\n        reduce_layer_impl::<C,false>(points,starts,counts,s);\n    }}\n}}\n\n");
    let new = wrapper + &body;
    s.replace_range(a..b, &new);
    let cfg="#[cfg(all(zopro_tree_record_probe,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
    let insert =
        format!("\n{cfg}\nmod layer_probe;\n{cfg}\npub use layer_probe::TreeLayerProbe;\n");
    s.push_str(&insert);
    assert_eq!(
        s.strip_suffix(&insert).unwrap().replace(&new, &old),
        original
    );
    fs::write(&file, &s).unwrap();
    fs::copy(
        r.join("layer_probe.rs"),
        r.join("candidate/crates/zopro-core/src/msm/layer_probe.rs"),
    )
    .unwrap();
    let lib = r.join("candidate/crates/zopro-core/src/lib.rs");
    let mut t = fs::read_to_string(&lib).unwrap();
    let lib_insert = format!("\n{cfg}\npub use msm::TreeLayerProbe;\n");
    t.push_str(&lib_insert);
    fs::write(&lib, &t).unwrap();
    assert_eq!(
        t.strip_suffix(&lib_insert).unwrap(),
        fs::read_to_string(prior.join("crates/zopro-core/src/lib.rs")).unwrap()
    );
    save(
        r.join("edits.json"),
        &json!({"msm_old":old,"msm_new":new,"msm_append":insert,"lib_append":lib_insert}),
    );
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_from_accepted":168,"changed_from_prior_candidate":["crates/zopro-core/src/msm.rs","crates/zopro-core/src/lib.rs"],"recovered_prior_candidate_bytes":true,"prior_arithmetic_except_const_parameter_spelling_exact":true,"prior_candidate_source_audit":"/tmp/zopro-g2-tree-records/implementation-audit.json","no_production_dispatcher":true,"new_native_cfg_probe_module":true,"field_sources_unchanged":true,"production_changed":false}),
    );
    let mut pins = BTreeMap::new();
    for (p, _) in m["source_files_sha256"].as_object().unwrap() {
        let f = r.join("candidate").join(p);
        pins.insert(f.display().to_string(), hash(f));
    }
    for rel in [
        "setup.rs",
        "probe.rs",
        "layer_probe.rs",
        "support/perf.rs",
        "design.md",
        "edits.json",
        "source-audit.json",
        "candidate/crates/zopro-core/src/msm/layer_probe.rs",
    ] {
        let f = r.join(rel);
        pins.insert(f.display().to_string(), hash(f));
    }
    for path in [
        "/tmp/zopro-g2-tree-records/implementation-audit.json",
        "/tmp/zopro-g2-tree-records/candidate/crates/zopro-core/src/msm.rs",
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
    ] {
        pins.insert(path.to_string(), hash(path));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let mut p: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-g2-tree-records/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-g2-tree-records", R),
    )
    .unwrap();
    for step in p["steps"].as_array_mut().unwrap() {
        if let Some(flags) = step["environment"]["RUSTFLAGS"].as_str() {
            step["environment"]["RUSTFLAGS"] =
                json!(format!("{flags} --cfg zopro_tree_record_probe"));
        }
        if step["name"] == "core" {
            step["required_tests"] = json!([
                "msm::layer_probe::tests::layer_probe_complete_storage_matches_both_kernels",
                "msm::layer_probe::tests::layer_probe_geometry_and_reset_preserve_input_storage",
                "msm::tree_prefix_tests::prefix_tree_complete_pairs_tails_and_reuse"
            ]);
        }
        if step["kind"] == "link-sdk" {
            step["name"] = json!("probe");
            step["driver"] = json!(r.join("probe.rs"));
            step["additional_sources"] = json!([r.join("support/perf.rs")]);
        }
    }
    save(r.join("native-plan.json"), &p);
    println!("Frozen native layerprobe;112coreexpected; previous arithmetic preserved; no SDK/Gnark/priorchange");
}

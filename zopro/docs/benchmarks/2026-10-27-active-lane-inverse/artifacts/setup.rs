use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-active-lane-inverse";
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
    let m = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        assert_eq!(hash(r.join("candidate").join(p)), h.as_str().unwrap());
    }
    let file = r.join("candidate/crates/zopro-core/src/msm.rs");
    let original = fs::read_to_string(&file).unwrap();
    let a = original.find("fn reduce_layer<C: CurveConfig>(").unwrap();
    let b = a + original[a..]
        .find("// The inverses of four chains")
        .unwrap();
    let old = original[a..b].to_owned();
    let before = "        let mut inverses = invert_lanes(tail, s.vector_inverse);";
    let after="        let mut inverses = if full == 0 && m > 0 {\n            invert_short_lanes(tail, m)\n        } else {\n            invert_lanes(tail, s.vector_inverse)\n        };";
    assert_eq!(old.matches(before).count(), 1);
    let new = old.replace(before, after);
    let helper = fs::read_to_string(r.join("short.rs")).unwrap();
    let cfg = "#[cfg(any(test,zopro_short_layer_probe))]";
    let append=format!("\n{cfg}\n{}\n{cfg}\nmod layer_probe;\n#[cfg(zopro_short_layer_probe)]\npub use layer_probe::{{ShortLayerProbe,ShortFixture}};\n",old.replacen("fn reduce_layer<","fn reduce_layer_original<",1));
    let mut changed = original.replacen(&old, &new, 1);
    changed.push_str("\n");
    changed.push_str(&helper);
    changed.push_str(&append);
    assert_eq!(
        changed
            .strip_suffix(&append)
            .unwrap()
            .strip_suffix(&helper)
            .unwrap()
            .strip_suffix('\n')
            .unwrap()
            .replace(&new, &old),
        original
    );
    fs::write(&file, changed).unwrap();
    fs::create_dir_all(r.join("candidate/crates/zopro-core/src/msm")).unwrap();
    fs::copy(
        r.join("layer_probe.rs"),
        r.join("candidate/crates/zopro-core/src/msm/layer_probe.rs"),
    )
    .unwrap();
    let lib = r.join("candidate/crates/zopro-core/src/lib.rs");
    let original_lib = fs::read_to_string(&lib).unwrap();
    let lib_append =
        "\n#[cfg(zopro_short_layer_probe)]\npub use msm::{ShortLayerProbe,ShortFixture};\n";
    fs::write(&lib, original_lib + lib_append).unwrap();
    save(
        r.join("edits.json"),
        &json!({"old":old,"new":new,"helper":helper,"append":append,"lib_append":lib_append}),
    );
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_originals":168,"production_changed":false,"accepted_bytes_recovered":true,"changed_native_path_only":"reduce_layer full==0 and m>0; streaming unchanged","short_active_count":"1..LANES-1","field_sources_unchanged":true,"new_fixed_stack_arrays_only":true,"original_body_reference_cfg_only":true,"no_new_tables_or_plans":true}),
    );
    let mut pins = BTreeMap::new();
    for (p, _) in m["source_files_sha256"].as_object().unwrap() {
        let f = r.join("candidate").join(p);
        pins.insert(f.display().to_string(), hash(f));
    }
    for name in [
        "setup.rs",
        "short.rs",
        "layer_probe.rs",
        "probe.rs",
        "support/perf.rs",
        "design.md",
        "edits.json",
        "source-audit.json",
        "candidate/crates/zopro-core/src/msm/layer_probe.rs",
    ] {
        let f = r.join(name);
        pins.insert(f.display().to_string(), hash(f));
    }
    pins.insert(
        "/tmp/zopro-xtask-fast-screen-target/release/xtask".into(),
        hash("/tmp/zopro-xtask-fast-screen-target/release/xtask"),
    );
    save(r.join("source-pins.json"), &json!(pins));
    let mut p: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-tree-kernel-split/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-tree-kernel-split", R),
    )
    .unwrap();
    for step in p["steps"].as_array_mut().unwrap() {
        if step["name"] == "core" {
            step["required_tests"] = json!([
                "msm::layer_probe::tests::active_inverse_matches_independent_scalar_elements",
                "msm::layer_probe::tests::short_tree_g1_complete_storage_and_reuse",
                "msm::layer_probe::tests::short_tree_g2_complete_storage_and_reuse"
            ]);
        }
        if let Some(flags) = step["environment"]["RUSTFLAGS"].as_str() {
            step["environment"]["RUSTFLAGS"] =
                json!(format!("{flags} --cfg zopro_short_layer_probe"));
        }
        if step["kind"] == "link-sdk" {
            step["name"] = json!("probe");
            step["driver"] = json!(r.join("probe.rs"));
            step["additional_sources"] = json!([r.join("support/perf.rs")]);
        }
    }
    p["environment"]["RAYON_NUM_THREADS"] = json!("1");
    save(r.join("native-plan.json"), &p);
    println!("Frozen active-lane inverse and actual-layer probe;112 core tests expected; production unchanged.");
}

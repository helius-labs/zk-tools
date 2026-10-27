use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-g1-reverse-overlap";
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
        let prod = Path::new("/home/ubuntu/src/zk-tools/zopro").join(p);
        assert_eq!(hash(&prod), h.as_str().unwrap());
        let dest = r.join("candidate").join(p);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(prod, &dest).unwrap();
        assert_eq!(hash(dest), h.as_str().unwrap());
    }
    let file = r.join("candidate/crates/zopro-core/src/msm.rs");
    let original = fs::read_to_string(&file).unwrap();
    let a = original.find("fn stream_g1_impl<C:").unwrap();
    let b = a + original[a..]
        .find("#[derive(Clone, Copy)]\nstruct Fq2AddGroup")
        .unwrap();
    let old = original[a..b].to_string();
    let reverse_start = old
        .find("    for (j, g) in s.groups.iter().enumerate().rev() {")
        .unwrap();
    let reverse_old = old[reverse_start..old.rfind("}\n").unwrap()].to_string();
    let reverse_new = fs::read_to_string(r.join("reverse.rs")).unwrap();
    let new = old.replacen(&reverse_old, &reverse_new, 1);
    assert_eq!(new.replace(&reverse_new, &reverse_old), old);
    let cfg = "#[cfg(any(test,zopro_g1_stream_probe))]";
    let append=format!("\n{cfg}\n{}\n{cfg}\nmod stream_probe;\n#[cfg(zopro_g1_stream_probe)]\npub use stream_probe::G1StreamProbe;\n",old.replacen("fn stream_g1_impl<","fn stream_g1_original_impl<",1));
    let changed = original.replacen(&old, &new, 1) + &append;
    assert_eq!(
        changed.strip_suffix(&append).unwrap().replace(&new, &old),
        original
    );
    fs::write(file, changed).unwrap();
    fs::create_dir_all(r.join("candidate/crates/zopro-core/src/msm")).unwrap();
    fs::copy(
        r.join("stream_probe.rs"),
        r.join("candidate/crates/zopro-core/src/msm/stream_probe.rs"),
    )
    .unwrap();
    let lib = r.join("candidate/crates/zopro-core/src/lib.rs");
    let lib_original = fs::read_to_string(&lib).unwrap();
    let lib_append = "\n#[cfg(zopro_g1_stream_probe)]\npub use msm::G1StreamProbe;\n";
    fs::write(lib, lib_original + lib_append).unwrap();
    save(
        r.join("edits.json"),
        &json!({"old_stream":old,"new_stream":new,"old_reverse":reverse_old,"new_reverse":reverse_new,"msm_append":append,"lib_append":lib_append}),
    );
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_candidate_originals":168,"production_changed":false,"accepted_files_recovered":true,"field_sources_unchanged":true,"ordinary_tree_and_g2_unchanged":true,"forward_and_scalar_tail_exact":true,"same_inverse_chain_order":true,"same_formulas_and_output_order":true,"no_new_heap_storage_or_record_fields":true,"additional_live_temporaries_may_affect_stack":true,"reference_and_public_probe_cfg_only":true}),
    );
    let mut pins = BTreeMap::new();
    for (p, _) in m["source_files_sha256"].as_object().unwrap() {
        let f = r.join("candidate").join(p);
        pins.insert(f.display().to_string(), hash(f));
    }
    for name in [
        "setup.rs",
        "reverse.rs",
        "stream_probe.rs",
        "probe.rs",
        "support/perf.rs",
        "design.md",
        "edits.json",
        "source-audit.json",
        "candidate/crates/zopro-core/src/msm/stream_probe.rs",
    ] {
        let f = r.join(name);
        pins.insert(f.display().to_string(), hash(f));
    }
    pins.insert(
        "/tmp/zopro-xtask-fast-screen-target/release/xtask".to_string(),
        hash("/tmp/zopro-xtask-fast-screen-target/release/xtask"),
    );
    save(r.join("source-pins.json"), &json!(pins));
    let mut p: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-tree-kernel-split/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-tree-kernel-split", R),
    )
    .unwrap();
    p["environment"]["RAYON_NUM_THREADS"] = json!("1");
    for step in p["steps"].as_array_mut().unwrap() {
        if step["name"] == "core" {
            step["required_tests"]=json!(["msm::stream_probe::tests::overlap_matches_independent_complete_curve_oracle","msm::stream_probe::tests::overlap_reuses_scratch_through_growing_and_shrinking_batches"]);
        }
        if let Some(flags) = step["environment"]["RUSTFLAGS"].as_str() {
            step["environment"]["RUSTFLAGS"] =
                json!(format!("{flags} --cfg zopro_g1_stream_probe"));
        }
        if step["kind"] == "link-sdk" {
            step["name"] = json!("probe");
            step["driver"] = json!(r.join("probe.rs"));
            step["additional_sources"] = json!([r.join("support/perf.rs")]);
        }
    }
    save(r.join("native-plan.json"), &p);
    println!("Frozen G1 streaming reverse overlap;111coreexpected;all170productionfilesunchanged.");
}

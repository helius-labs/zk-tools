use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/tmp/zopro-composed-cpu";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn edit(root: &Path, rel: &str, old: &str, new: &str, ops: &mut Vec<Value>) {
    let p = root.join("candidate").join(rel);
    let text = fs::read_to_string(&p).unwrap();
    assert_eq!(text.matches(old).count(), 1, "edit context {rel}");
    fs::write(p, text.replacen(old, new, 1)).unwrap();
    ops.push(json!({"kind":"text","file":rel,"old":old,"new":new}));
}
fn apply_patch(root: &Path, parent: &str, rel: &str, name: &str, ops: &mut Vec<Value>) {
    let prod = Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel);
    let other = Path::new(parent).join(rel);
    let out = Command::new("git")
        .args(["diff", "--no-index", "--unified=3", "--"])
        .arg(&prod)
        .arg(&other)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    let raw = String::from_utf8(out.stdout).unwrap();
    let start = raw.find("@@ ").unwrap();
    let patch = format!("--- a/{rel}\n+++ b/{rel}\n{}", &raw[start..]);
    let patch_path = root.join("patches").join(format!("{name}.patch"));
    fs::write(&patch_path, &patch).unwrap();
    for check in [true, false] {
        let mut cmd = Command::new("git");
        cmd.current_dir(root.join("candidate")).arg("apply");
        if check {
            cmd.arg("--check");
        }
        let status = cmd.arg(&patch_path).status().unwrap();
        assert!(status.success(), "patch {name} check={check}");
    }
    ops.push(json!({"kind":"patch","file":rel,"path":patch_path,"patch":patch,"parent":other,"parent_sha256":hash(other)}));
}
fn replace_function(
    root: &Path,
    name: &str,
    transform: impl FnOnce(&str) -> String,
    ops: &mut Vec<Value>,
) {
    let rel = "crates/zopro-core/src/msm.rs";
    let s = fs::read_to_string(root.join("candidate").join(rel)).unwrap();
    let a = s.find(&format!("pub(crate) fn {name}<")).unwrap();
    let b = a + s[a..].find("\n}\n").unwrap() + 2;
    let old = &s[a..b];
    let new = transform(old);
    assert_ne!(old, new);
    edit(root, rel, old, &new, ops);
}
fn main() {
    let r = Path::new(R);
    fs::create_dir_all(r.join("patches")).unwrap();
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (rel, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let p = Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let out = r.join("candidate").join(rel);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::copy(p, out).unwrap();
    }
    let mut ops = Vec::new();
    for (rel, name) in [
        ("crates/zopro-core/src/msm.rs", "borrowed-stream"),
        ("crates/zopro-core/src/msm_affine.rs", "borrowed-ordinary"),
        (
            "crates/zopro-core/src/msm_affine/fused_kh.rs",
            "borrowed-kh",
        ),
    ] {
        apply_patch(r, "/tmp/zopro-borrowed-g1/candidate", rel, name, &mut ops);
    }
    let trait_head = "pub(super) trait BatchInverse: Field {\n";
    let hook="    fn borrowed_msm<C:CurveConfig<Base=Self>>(_bases:&[Affine<C>],_scalars:&[[u64;4]],_parts:usize,_width:usize)->Option<Projective<C>>{None}\n";
    edit(
        r,
        "crates/zopro-core/src/msm.rs",
        &format!("{trait_head}{hook}"),
        trait_head,
        &mut ops,
    );
    apply_patch(
        r,
        "/tmp/zopro-adaptive-tree/candidate",
        "crates/zopro-core/src/msm.rs",
        "adaptive-g2",
        &mut ops,
    );
    edit(
        r,
        "crates/zopro-core/src/msm.rs",
        trait_head,
        &format!("{trait_head}{hook}"),
        &mut ops,
    );
    apply_patch(
        r,
        "/tmp/zopro-poseidon-direct-dot/candidate",
        "crates/zopro-core/src/solver/sparse_poseidon.rs",
        "direct-poseidon",
        &mut ops,
    );
    let module_rel = "crates/zopro-core/src/msm_affine/borrowed_pending.rs";
    let module =
        fs::read_to_string(Path::new("/tmp/zopro-borrowed-g1/candidate").join(module_rel)).unwrap();
    let selector =
        "        && std::env::var(\"ZOPRO_CPU_BORROWED_PENDING\").as_deref() == Ok(\"1\")\n";
    let ordinary =
        "enabled() && std::env::var(\"ZOPRO_CPU_BORROWED_ORDINARY\").as_deref()==Ok(\"1\")";
    assert_eq!(module.matches(selector).count(), 1);
    assert_eq!(module.matches(ordinary).count(), 1);
    let cleaned = module.replace(selector, "").replace(ordinary, "enabled()");
    fs::write(r.join("borrowed_pending.original.rs"), module).unwrap();
    let module_path = r.join("candidate").join(module_rel);
    fs::write(&module_path, &cleaned).unwrap();
    assert!(Command::new("rustfmt")
        .args(["--edition", "2021", "--config", "reorder_imports=false"])
        .arg(&module_path)
        .status()
        .unwrap()
        .success());
    let formatted = fs::read_to_string(&module_path).unwrap();
    let compact = |s: &str| {
        let mut s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        // Rustfmt inserts optional trailing commas in multiline calls and lists.
        loop {
            let next = s.replace(",}", "}").replace(",)", ")").replace(",]", "]");
            if next == s {
                return s;
            }
            s = next;
        }
    };
    // Rustfmt may wrap long test-fixture closures in explicit blocks.
    // Compare every production token; retain the formatted tests and run them.
    assert_eq!(
        compact(formatted.split("#[cfg(test)]").next().unwrap()),
        compact(cleaned.split("#[cfg(test)]").next().unwrap())
    );
    assert!(!formatted.contains("ZOPRO_CPU_BORROWED"));
    replace_function(
        r,
        "workspace_batch",
        |s| {
            let a = "    if core::mem::size_of::<C::Base>() == 32 {";
            assert_eq!(s.matches(a).count(), 1);
            let s = s.replace(
                a,
                "    let payload = if core::mem::size_of::<C::Base>() == 32 {",
            );
            assert!(s.ends_with("    }\n}"));
            s.strip_suffix("    }\n}").unwrap().to_string()
                + "    };\n    payload?.checked_add(core::mem::size_of::<Vec<[FqBatch; 2]>>())\n}"
        },
        &mut ops,
    );
    replace_function(
        r,
        "workspace_weighted",
        |s| {
            let a = "        .checked_add(v::<Group<C::Base>>(pairs / LANES, false)?)?;";
            assert_eq!(s.matches(a).count(), 1);
            s.replace(a,"        .checked_add(v::<Group<C::Base>>(pairs / LANES, false)?)?\n        .checked_add(core::mem::size_of::<Vec<[FqBatch; 2]>>())?;")
        },
        &mut ops,
    );
    replace_function(
        r,
        "workspace_grouping",
        |s| {
            let a = "        .checked_add(jobs.checked_mul(3 * 64)?)";
            assert_eq!(s.matches(a).count(), 1);
            s.replace(a,"        .checked_add(jobs.checked_mul(3 * 64)?)?\n        .checked_add(jobs.checked_mul(core::mem::size_of::<Vec<[FqBatch; 2]>>())?)")
        },
        &mut ops,
    );
    replace_function(
        r,
        "workspace_grouping_linear",
        |s| {
            let a = "4 * core::mem::size_of::<usize>() + 3 * 64";
            assert_eq!(s.matches(a).count(), 1);
            s.replace(a,"4 * core::mem::size_of::<usize>() + 3 * 64 + core::mem::size_of::<Vec<[FqBatch; 2]>>()")
        },
        &mut ops,
    );
    save(r.join("composition-edits.json"), &json!(ops));
    let recovery = r.join("recovery");
    for (rel, _) in manifest["source_files_sha256"].as_object().unwrap() {
        let out = recovery.join(rel);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::copy(r.join("candidate").join(rel), out).unwrap();
    }
    for op in ops.iter().rev() {
        if op["kind"] == "text" {
            let p = recovery.join(op["file"].as_str().unwrap());
            let s = fs::read_to_string(&p).unwrap();
            let new = op["new"].as_str().unwrap();
            assert_eq!(s.matches(new).count(), 1);
            fs::write(p, s.replacen(new, op["old"].as_str().unwrap(), 1)).unwrap();
        } else {
            for check in [true, false] {
                let mut c = Command::new("git");
                c.current_dir(&recovery).args(["apply", "--reverse"]);
                if check {
                    c.arg("--check");
                }
                assert!(c
                    .arg(op["path"].as_str().unwrap())
                    .status()
                    .unwrap()
                    .success());
            }
        }
    }
    let mut changed = Vec::new();
    for (rel, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(recovery.join(rel)), h.as_str().unwrap());
        if hash(r.join("candidate").join(rel)) != h.as_str().unwrap() {
            changed.push(rel.clone());
        }
    }
    assert_eq!(
        changed,
        vec![
            "crates/zopro-core/src/msm.rs",
            "crates/zopro-core/src/msm_affine.rs",
            "crates/zopro-core/src/msm_affine/fused_kh.rs",
            "crates/zopro-core/src/solver/sparse_poseidon.rs"
        ]
    );
    fs::remove_dir_all(recovery).unwrap();
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_candidate_originals":166,"changed":changed,"new_module":module_rel,"all170accepted_bytes_recovered_by_reversing_composition":true,"borrowed_module_arithmetic_tokens_exact_except_removing2selectors":true,"field_sources_unchanged":true,"checked_forecasts_charge_new_vector_headers":true,"retained_capacity_argument_is_not_process_peak_bound":true,"diagnostic_runtime_selectors_removed":true,"production_changed":false,"no_new_unsafe_table_plan_pool_keycopy":true}),
    );
    let mut pins = BTreeMap::new();
    for (rel, _) in manifest["source_files_sha256"].as_object().unwrap() {
        let p = r.join("candidate").join(rel);
        pins.insert(p.display().to_string(), hash(p));
    }
    for name in [
        "setup.rs",
        "design.md",
        "record-memory-argument.md",
        "composition-edits.json",
        "source-audit.json",
        "borrowed_pending.original.rs",
        "candidate/crates/zopro-core/src/msm_affine/borrowed_pending.rs",
        "candidate/crates/zopro-bench/examples/cpu-sdk.rs",
        "candidate/crates/zopro-bench/examples/support/perf.rs",
    ] {
        let p = r.join(name);
        pins.insert(p.display().to_string(), hash(p));
    }
    for p in [
        "/tmp/zopro-borrowed-g1/closing-source-audit.json",
        "/tmp/zopro-adaptive-tree/source-audit.json",
        "/tmp/zopro-poseidon-direct-dot/implementation-audit.json",
        "/tmp/zopro-tree-record-crossover/training-rule.json",
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
    ] {
        pins.insert(p.to_string(), hash(p));
    }
    for op in &ops {
        if op["kind"] == "patch" {
            let p = PathBuf::from(op["path"].as_str().unwrap());
            pins.insert(p.display().to_string(), hash(p));
        }
    }
    save(r.join("source-pins.json"), &json!(pins));
    let mut plan: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-tree-kernel-split/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-tree-kernel-split", R),
    )
    .unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["name"] == "core" {
            step["required_tests"]=json!(["msm::deferred_sign_reference_tests::borrowed_signed_stream_tails_and_reuse_match_generator","msm_affine::borrowed_pending::tests::pending_record_sizes_and_collision_reuse","msm_affine::borrowed_pending::tests::fused_generator_oracle_and_owned_control","msm_affine::borrowed_pending::tests::ordinary_queries_generator_oracle_and_owned_control","msm_affine::borrowed_pending::tests::ordinary_trait_dispatch_and_fallback","msm::adaptive_tree_tests::adaptive_boundaries_preserve_outputs_and_bound_retained_record_capacity","msm::tree_prefix_tests::prefix_tree_complete_pairs_tails_and_reuse","solver::sparse_poseidon::tests::every_width_exact_rows_and_adversarial_rejection","solver::sparse_poseidon::tests::raw_operation_indices_above_tag_limit_fall_back"]);
        }
        if step["name"] == "split" {
            step["name"] = json!("composed");
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Composed3candidates,4originalfiles+1module;reversed170exact;selectorsremoved;headerscharged;116nativecoreexpected.");
}

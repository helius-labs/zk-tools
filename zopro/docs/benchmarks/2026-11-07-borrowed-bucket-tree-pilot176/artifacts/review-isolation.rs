use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176/native";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn receipt(root: &str, step: &Value) {
    let name = step["name"].as_str().unwrap();
    let prefix = format!("{root}/{name}");
    let e = load(format!("{prefix}/exit.json"));
    assert_eq!(e["success"], true);
    assert_eq!(e["returncode"], 0);
    let c = load(format!("{prefix}/command.json"));
    let a = c["argv"].as_array().unwrap();
    let b = step["argv"].as_array().unwrap();
    assert_eq!(&a[a.len() - b.len()..], b);
    for s in ["stdout", "stderr"] {
        assert_eq!(
            hash(format!("{prefix}/{s}.log")),
            e[format!("{s}_sha256")].as_str().unwrap()
        );
    }
}
fn main() {
    assert!(!Path::new(&format!("{R}/native-isolation-review.json")).exists());
    assert_eq!(
        load(format!("{R}/native-review.json"))["review_passed"],
        true
    );
    let before = load(format!("{R}/core-fixed-pins.json"));
    let after = load(format!("{R}/core-isolated-pins.json"));
    assert_eq!(before.as_object().unwrap().len(), 178);
    assert_eq!(after.as_object().unwrap().len(), 178);
    let relative = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let mut exact = 0;
    for (p, h) in after.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        let rel = p.strip_prefix(&format!("{R}/source-isolated/")).unwrap();
        let old = format!("{R}/source-fixed/{rel}");
        assert_eq!(hash(&old), before[&old].as_str().unwrap());
        if rel != relative {
            assert_eq!(h.as_str().unwrap(), hash(&old));
            exact += 1;
        }
    }
    assert_eq!(exact, 177);
    let mut parent = fs::read_to_string(format!("{R}/source-isolated/{relative}")).unwrap();
    let module = "#[cfg(any(zopro_borrowed_bucket_tree, test))]\n";
    assert_eq!(parent.matches(module).count(), 1);
    parent = parent.replace(module, "");
    let positive = "    #[cfg(zopro_borrowed_bucket_tree)]\n";
    assert_eq!(parent.matches(positive).count(), 3);
    parent = parent.replace(positive, "");
    let negative = "    #[cfg(not(zopro_borrowed_bucket_tree))]\n    let tree = false;\n";
    assert_eq!(parent.matches(negative).count(), 1);
    parent = parent.replace(negative, "");
    assert_eq!(
        parent,
        fs::read_to_string(format!("{R}/source-fixed/{relative}")).unwrap()
    );
    let plan = load(format!("{R}/core-isolated-plan.json"));
    assert_eq!(plan["steps"].as_array().unwrap().len(), 1);
    assert_eq!(
        load(format!("{R}/core-isolated-runs/result.json"))["status"],
        "complete"
    );
    let step = &plan["steps"][0];
    receipt(&format!("{R}/core-isolated-runs"), step);
    let t = load(format!("{R}/core-isolated-runs/core-off/tests.json"));
    assert_eq!(t["unfiltered"], true);
    assert_eq!(t["passed_tests"].as_array().unwrap().len(), 130);
    for required in step["required_tests"].as_array().unwrap() {
        assert!(t["passed_tests"].as_array().unwrap().contains(required));
    }
    assert!(!step["environment"]["RUSTFLAGS"]
        .as_str()
        .unwrap()
        .contains("--cfg zopro_borrowed_bucket_tree"));
    let old_core = load(format!("{R}/core-fixed-plan.json"));
    let on = old_core["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "core-on")
        .unwrap();
    assert!(on["environment"]["RUSTFLAGS"]
        .as_str()
        .unwrap()
        .ends_with("--cfg zopro_borrowed_bucket_tree"));
    receipt(&format!("{R}/core-fixed-runs"), on);
    let inherited = load(format!("{R}/core-fixed-runs/core-on/tests.json"));
    assert_eq!(inherited["passed_tests"].as_array().unwrap().len(), 130);
    assert_eq!(inherited["unfiltered"], true);
    let sdk = load(format!("{R}/sdk-build-fixed-plan.json"));
    let mut passed = 0;
    for s in sdk["steps"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["name"].as_str().unwrap().starts_with("prover-"))
    {
        receipt(&format!("{R}/sdk-build"), s);
        let a = s["argv"].as_array().unwrap();
        assert!(a.iter().any(|v| v.as_str().is_some_and(
            |v| v.starts_with("RUSTFLAGS=") && v.ends_with("--cfg zopro_borrowed_bucket_tree")
        )));
        let name = s["name"].as_str().unwrap();
        let tests = load(format!("{R}/sdk-build/{name}/tests.json"));
        assert_eq!(tests["unfiltered"], true);
        passed += tests["passed_tests"].as_array().unwrap().len();
    }
    assert_eq!(passed, 8);
    let out = json!({"review_passed":true,"final_source_files":178,"source_pin_checks":178,"unchanged_vs_qualified_ON_files":177,"ON_parent_cfg_equivalence_passed":true,"final_OFF_tests_passed":130,"ON_core_tests_inherited":130,"ON_prover_tests_inherited":8,"no_successful_ON_suites_replayed":true,"final_source":format!("{R}/source-isolated"),"new_proofs":0,"new_timings":0,"adopted":false,"limits":"Exact cfg-selection source equivalence and receipts, not full Rust macro expansion, independent curve/field oracle, measured memory or speed. Fresh coherent final library links and SDK proof preflight remain separate stages."});
    fs::write(
        format!("{R}/native-isolation-review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("isolation review PASS;177same+exactONcfgparent;130freshOFF/130inheritedON/8prover");
}

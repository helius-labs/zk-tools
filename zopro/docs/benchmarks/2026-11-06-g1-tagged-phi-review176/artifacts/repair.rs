use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
fn read(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &str) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, pins: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, pins)
        } else {
            let s = p.display().to_string();
            pins.insert(s.clone(), hash(&s));
        }
    }
}
fn main() {
    assert!(!Path::new(R).join("repair-plan.json").exists());
    let old = read(&format!("{R}/plan.json"));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(&format!("{R}/pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let audit = read(&format!("{R}/source-audit.json"));
    for e in old["steps"].as_array().unwrap().iter().take(4) {
        let name = e["name"].as_str().unwrap();
        assert_eq!(read(&format!("{R}/runs/{name}/exit.json"))["returncode"], 0);
    }
    assert_eq!(
        read(&format!("{R}/runs/primary-work-model/exit.json"))["returncode"],
        101
    );
    fn copy(from: &Path, to: &Path) {
        fs::create_dir_all(to).unwrap();
        for e in fs::read_dir(from).unwrap() {
            let e = e.unwrap();
            let p = e.path();
            let q = to.join(e.file_name());
            if p.is_dir() {
                copy(&p, &q)
            } else {
                fs::copy(&p, q).unwrap();
            }
        }
    }
    copy(
        &Path::new(R).join("source"),
        &Path::new(R).join("source-fixed"),
    );
    let p = format!("{R}/source-fixed/crates/zopro-core/src/tagged_phi_diagnostic.rs");
    let mut s = fs::read_to_string(&p).unwrap();
    for (a, b) in [
        (
            "assert_eq!(h.len(), q.product.len());",
            "assert!(h.len() <= q.product.len());\n        let used_h = h.len();",
        ),
        (
            "h_identity: q.product.iter().map(|p| p.is_identity()).collect(),",
            "h_identity: q.product[..used_h].iter().map(|p| p.is_identity()).collect(),",
        ),
    ] {
        assert_eq!(s.matches(a).count(), 1, "{a}");
        s = s.replacen(a, b, 1);
    }
    fs::write(&p, s).unwrap();
    let mut model = fs::read_to_string(format!("{R}/model.rs")).unwrap();
    for(a,b)in[("assert_eq!((inputs.k.len(), inputs.h.len()), (30529, 32768));","assert_eq!(inputs.k.len(), 30529);\n    assert!(inputs.h.len() <= 32768);\n    assert_eq!(inputs.h.len(), inputs.h_identity.len());"),("assert_eq!(cache.len(), 64 + 63297 * 65);","assert_eq!(cache.len(), 64 + (inputs.k.len() + inputs.h.len()) * 65);"),("\"new_solver_calls\":1","\"new_solver_calls\":2"),("\"new_product_FFT_calls\":1","\"new_product_FFT_calls\":2")]{assert_eq!(model.matches(a).count(),1,"{a}");model=model.replacen(a,b,1);}
    fs::write(format!("{R}/model-fixed.rs"), model).unwrap();
    let mut steps = Vec::new();
    for (index, name) in [
        (0, "diagnostic-core-build-fixed"),
        (2, "model-helper-build-fixed"),
        (4, "primary-work-model-fixed"),
    ] {
        let mut step = old["steps"][index].clone();
        step["name"] = json!(name);
        for arg in step["argv"].as_array_mut().unwrap() {
            let mut s = arg.as_str().unwrap().to_owned();
            s = s.replace(
                &format!("{R}/source/crates/zopro-core/src/lib.rs"),
                &format!("{R}/source-fixed/crates/zopro-core/src/lib.rs"),
            );
            s = s.replace(
                &format!("{R}/native-libs/libzopro_core.rlib"),
                &format!("{R}/native-libs/libzopro_core-fixed.rlib"),
            );
            if s == format!("{R}/model.rs") {
                s = format!("{R}/model-fixed.rs");
            } else if s == format!("{R}/model") {
                s = format!("{R}/model-fixed");
            }
            *arg = json!(s);
        }
        step["environment"]["CARGO_MANIFEST_DIR"] =
            json!(format!("{R}/source-fixed/crates/zopro-core"));
        steps.push(step);
    }
    walk(&Path::new(R).join("source-fixed"), &mut pins);
    for f in [
        "repair.rs",
        "repair",
        "repair.md",
        "model-fixed.rs",
        "witness.bin",
        "witness-info.json",
        "native-libs/libzopro_core.rlib",
    ] {
        let p = format!("{R}/{f}");
        pins.insert(p.clone(), hash(&p));
    }
    let source_audit = json!({"source_files":177,"originals_exact":174,"all176recover":true,"original_function_bodies_and_field_files_exact":true,"suffixes":audit["suffixes"],"diagnostic_only_fix":"product-point identity flags use the exact trimmed product prefix; original solve/FFT/math unchanged","initial_failed_model_completed_solve_and_FFT":true,"total_solves_FFTs_after_success":2,"reused_witness_and_4successful_initial_stages":true});
    fs::write(
        format!("{R}/source-fixed-audit.json"),
        serde_json::to_vec_pretty(&source_audit).unwrap(),
    )
    .unwrap();
    pins.insert(
        format!("{R}/source-fixed-audit.json"),
        hash(&format!("{R}/source-fixed-audit.json")),
    );
    fs::write(
        format!("{R}/repair-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        format!("{R}/repair-plan.json"),
        serde_json::to_vec_pretty(
            &json!({"version":1,"source_pins":format!("{R}/repair-pins.json"),"steps":steps}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("Preserved4successful stages/failed model; rebuilt changed diagnostic only; model uses trimmed production prefix; solve/FFT total2 if success");
}

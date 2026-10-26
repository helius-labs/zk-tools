use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    let mut plan: Value =
        serde_json::from_slice(&fs::read(root.join("build-plan-initial.json")).unwrap()).unwrap();
    let steps = plan["steps"].as_array_mut().unwrap();
    let mut compile = steps[1].clone();
    compile["name"] = json!("oracle-dependencies");
    compile["argv"] = json!([
        "cargo",
        "test",
        "--manifest-path",
        root.join("oracle/Cargo.toml"),
        "--offline",
        "--locked",
        "--release",
        "--bin",
        "probe",
        "--no-run"
    ]);
    compile.as_object_mut().unwrap().remove("unfiltered");
    compile.as_object_mut().unwrap().remove("required_tests");
    let deps = root.join("target-oracle/release/deps");
    let library = |prefix: &str| {
        let v = fs::read_dir(&deps)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| {
                p.file_name().unwrap().to_str().unwrap().starts_with(prefix)
                    && p.extension().is_some_and(|s| s == "rlib")
            })
            .collect::<Vec<_>>();
        assert_eq!(v.len(), 1);
        v[0].clone()
    };
    let ark = library("libark_bn254-");
    let ff = library("libark_ff-");
    let mut link = steps[2].clone();
    link["name"] = json!("frozen-oracle-link");
    let mut argv = link["argv"].as_array().unwrap().clone();
    argv.push(json!("--test"));
    argv.push(json!("--extern"));
    argv.push(json!(format!("ark_ref={}", ark.display())));
    argv.push(json!("--extern"));
    argv.push(json!(format!("ark_ff={}", ff.display())));
    argv.push(json!("-L"));
    argv.push(json!(format!("dependency={}", deps.display())));
    let out = argv.iter().position(|v| v == "-o").unwrap();
    argv[out + 1] = json!(root.join("oracle-tests"));
    link["argv"] = json!(argv);
    let mut tests = steps[1].clone();
    tests["argv"] = json!([root.join("oracle-tests"), "--test-threads=1", "--nocapture"]);
    let executable = steps[2].clone();
    let compiler = steps[0].clone();
    plan["steps"] = json!([compiler, compile, link, tests, executable]);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(root.join("input-pins-initial.json")).unwrap()).unwrap();
    pins.insert(
        root.join("probe.rs").to_str().unwrap().into(),
        hash(root.join("probe.rs")),
    );
    for p in [
        ark,
        ff,
        root.join("oracle/Cargo.lock"),
        root.join("repair.rs"),
        root.join("probe-initial.rs"),
        root.join("build-plan-initial.json"),
        root.join("input-pins-initial.json"),
    ] {
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    plan["source_pins"] = json!(root.join("input-pins-v2.json"));
    fs::write(
        root.join("input-pins-v2.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("build-plan-v2.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    fs::write(root.join("repair.json"),serde_json::to_vec_pretty(&json!({"initial_failure":"Missing Arkworks AdditiveGroup trait import for ZERO in an oracle test; compile failed before any test or measurement.","repair":"Add only the trait import. Retain first source/plan/logs. Oracle dependencies compiled by Cargo; tests and measured probe both explicitly link the frozen accepted BN254 library. Fresh Cargo dependency/test executable is not measured or run.","arithmetic_changed":false,"timing_gate_changed":false,"pins":pins.len()})).unwrap()).unwrap();
    println!("Frozen revised workflow {} pins", pins.len());
}

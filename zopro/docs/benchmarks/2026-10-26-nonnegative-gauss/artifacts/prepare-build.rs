use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-nonnegative-gauss");
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let mut pins = BTreeMap::new();
    for (name, digest) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(name)),
            digest.as_str().unwrap()
        );
        let p = root.join("candidate").join(name);
        pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    }
    let artifacts = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let field = artifacts["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(
        hash(field),
        artifacts["input_pins"][field].as_str().unwrap()
    );
    for name in [
        "experiment.json",
        
        "implement.rs",
        "field-additions.rs.txt",
        "source-audit.json",
        "prepare-tests.rs",
        "positive-prefix.rs.txt",
        "positive-suffix.rs.txt",
        "implement-initial.rs",
        "row-model.rs.txt",
        "probe-tests.rs",
        "probe.rs",
        "oracle/Cargo.toml",
        "oracle/Cargo.lock",
        "prepare-build.rs",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    }
    for p in [
        field,
        "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask",
        "/tmp/zopro-grouping-module/production-source-manifest.json",
    ] {
        pins.insert(p.into(), hash(p));
    }
    let flags =
        manifest["rustflags"].as_str().unwrap().to_owned() + " --cfg zopro_gauss_positive";
    let steps = json!([
    {"kind":"command","name":"compiler","argv":["rustc","-Vv"]},
    {"kind":"command","name":"generator-check","argv":["python3",root.join("candidate/scripts/ifma8.py"),"--check"]},
    {"kind":"command","name":"field-tests","argv":["cargo","test","--manifest-path",root.join("candidate/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"required_tests":["tests::base_field","tests::scalar_field"],"unfiltered":true,"timeout_seconds":1200},
    {"kind":"command","name":"component-oracles","argv":["cargo","test","--manifest-path",root.join("oracle/Cargo.toml"),"--offline","--locked","--release","--bin","probe","--","--test-threads=1","--show-output"],"required_tests":["tests::four::exact_positive_products_and_coefficients","tests::four::arkworks_lazy_boundary_and_chains","tests::eight::exact_positive_products_and_coefficients","tests::eight::arkworks_lazy_boundary_and_chains"],"unfiltered":true,"timeout_seconds":1200},
    {"kind":"command","name":"fresh-bench-build","argv":["cargo","build","--manifest-path",root.join("oracle/Cargo.toml"),"--offline","--locked","--release","--bin","probe","--message-format=json"],"timeout_seconds":1200},
    {"kind":"command","name":"accepted-bench-link","argv":["rustc","--edition=2021","-O","-C","target-cpu=native","-C","lto=off","-C","codegen-units=1",root.join("probe.rs"),"--extern",format!("zopro_bn254={field}"),"-L","dependency=/tmp/zopro-grouping-module/target-native/release/deps","-o",root.join("accepted-probe")],"timeout_seconds":1200}
    ]);
    fs::write(
        root.join("build-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(root.join("build-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":root.join("build-input-pins.json"),"environment":{"RUSTFLAGS":flags,"CARGO_TARGET_DIR":root.join("target"),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"1"},"steps":steps})).unwrap()).unwrap();
    fs::write(root.join("work-state.json"),serde_json::to_vec_pretty(&json!({"phase":"building/oracles; no timing yet","root":root,"build_pins":pins.len(),"last_pushed_commit":"359c38fa03b0ae7d8695f614544dc08137526eb4","prover_accepted_unchanged":true,"adopted":false,"goal_achieved":false})).unwrap()).unwrap();
    println!(
        "Frozen {} build pins; only two additive field files; no production dispatch",
        pins.len()
    );
}

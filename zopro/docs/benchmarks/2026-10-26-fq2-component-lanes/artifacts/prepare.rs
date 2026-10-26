use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    let mut experiment = read(root.join("experiment.json"));
    experiment["timing"]["checks"]=json!("Every final canonical Fq value in every output state and updated scale must agree across modes in the corresponding pattern/block, not only a checksum. All results and failures retained; no owned CPU overlap.");
    fs::write(
        root.join("experiment.json"),
        format!("{}\n", serde_json::to_string_pretty(&experiment).unwrap()),
    )
    .unwrap();
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("accepted-source/.xtask/source-pins.json"))).unwrap();
    let accepted = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let library = accepted["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(
        hash(library),
        accepted["input_pins"][library].as_str().unwrap()
    );
    let serde="/home/ubuntu/src/zk-tools/zopro/xtask/target/release/deps/libserde_json-cdebb9338c2784ad.rlib";
    let xtask = "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask";
    for p in [
        root.join("experiment.json"),
        root.join("design.md"),
        root.join("probe.rs"),
        root.join("prepare.rs"),
        root.join("oracle/Cargo.toml"),
        Path::new(library).into(),
        Path::new(serde).into(),
        Path::new(xtask).into(),
    ] {
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    fs::copy(
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        root.join("accepted-manifest.json"),
    )
    .unwrap();
    fs::write(
        root.join("input-pins.json"),
        format!("{}\n", serde_json::to_string_pretty(&pins).unwrap()),
    )
    .unwrap();
    fs::write(root.join("build-plan.json"),format!("{}\n",serde_json::to_string_pretty(&json!({"version":1,"source_pins":root.join("input-pins.json"),"environment":{"CARGO_TARGET_DIR":root.join("target-oracle"),"CARGO_BUILD_JOBS":"16","RUSTFLAGS":"-C target-cpu=native --cfg zopro_cios_cancel_carry --cfg zopro_horizontal_limb_sum"},"steps":[
        {"kind":"command","name":"compiler","argv":["rustc","-Vv"]},
        {"kind":"command","name":"oracle-tests","argv":["cargo","test","--manifest-path",root.join("oracle/Cargo.toml"),"--offline","--release","--bin","probe","--","--test-threads=1","--nocapture"],"unfiltered":true,"required_tests":["tests::independent_arkworks_operations_and_lazy_chains","tests::modulus_corners_and_half_order","tests::complete_curve_finish_matches_independent_arkworks","tests::changing_state_workloads_match_all_output_words"],"timeout_seconds":1200},
        {"kind":"command","name":"frozen-library-link","argv":["rustc","--edition=2021","-O","-C","target-cpu=native","-C","codegen-units=1","-C","lto=off",root.join("probe.rs"),"--extern",format!("zopro_bn254={library}"),"--extern",format!("serde_json={serde}"),"-L","dependency=/tmp/zopro-grouping-module/target-native/release/deps","-L","dependency=/home/ubuntu/src/zk-tools/zopro/xtask/target/release/deps","-o",root.join("probe")],"timeout_seconds":1200}
    ]})).unwrap())).unwrap();
    println!(
        "Frozen {} source/artifact pins; production BN254 {}",
        pins.len(),
        hash(library)
    );
}

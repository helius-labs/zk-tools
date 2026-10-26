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
    let r = Path::new("/tmp/zopro-phased-complex-products");
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let mut pins = BTreeMap::<String, String>::new();
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
        let p = r.join("candidate").join(n);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let a = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let field = a["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(hash(field), a["input_pins"][field].as_str().unwrap());
    let serde="/home/ubuntu/src/zk-tools/zopro/xtask/target/release/deps/libserde_json-cdebb9338c2784ad.rlib";
    let xtask = "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask";
    for name in [
        "experiment.json",
        "range-argument.md",
        "field-block.rs.txt",
        "implement.rs",
        "prepare-probe.rs",
        "source-audit.json",
        "probe.rs",
        "probe-tests.rs",
        "oracle/Cargo.toml",
        "oracle/Cargo.lock",
        "prepare-build.rs",
    ] {
        let p = r.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    for p in [
        field,
        serde,
        xtask,
        "/tmp/zopro-grouping-module/production-source-manifest.json",
    ] {
        pins.insert(p.into(), hash(p));
    }
    fs::write(
        r.join("build-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let flags = read("/tmp/zopro-deferred-dif32-sdk/native-plan.json")["environment"]["RUSTFLAGS"]
        .as_str()
        .unwrap()
        .replace(" --cfg zopro_deferred_dif32", "")
        + " --cfg zopro_phased_complex_products";
    let steps = json!([
    {"kind":"command","name":"compiler","argv":["rustc","-Vv"]},
    {"kind":"command","name":"generator-check","argv":["python3",r.join("candidate/scripts/ifma8.py"),"--check"]},
    {"kind":"command","name":"field-tests","argv":["cargo","test","--manifest-path",r.join("candidate/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"required_tests":["tests::base_field","tests::scalar_field"],"unfiltered":true,"timeout_seconds":1200},
    {"kind":"command","name":"component-oracles","argv":["cargo","test","--manifest-path",r.join("oracle/Cargo.toml"),"--offline","--locked","--release","--bin","probe","--","--test-threads=1","--show-output"],"required_tests":["tests::four::exact_signed_products_and_coefficients","tests::four::arkworks_lazy_boundary_and_chains","tests::eight::exact_signed_products_and_coefficients","tests::eight::arkworks_lazy_boundary_and_chains"],"unfiltered":true,"timeout_seconds":1200},
    {"kind":"command","name":"fresh-bench-build","argv":["cargo","build","--manifest-path",r.join("oracle/Cargo.toml"),"--offline","--locked","--release","--bin","probe"],"timeout_seconds":1200},
    {"kind":"command","name":"accepted-bench-link","argv":["rustc","--edition=2021","-O","-C","target-cpu=native","-C","lto=off","-C","codegen-units=1",r.join("probe.rs"),"--extern",format!("zopro_bn254={field}"),"-L","dependency=/tmp/zopro-grouping-module/target-native/release/deps","-o",r.join("accepted-probe")],"timeout_seconds":1200}
    ]);
    let plan = json!({"version":1,"source_pins":r.join("build-input-pins.json"),"environment":{"CARGO_TARGET_DIR":r.join("target"),"CARGO_BUILD_JOBS":"16","RUSTFLAGS":flags},"steps":steps});
    fs::write(
        r.join("build-plan.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    let p = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(p).unwrap();
    let text="# CURRENT — 2026-10-26: PHASED COMPLEX COMPONENT BUILDING; LAST PUSH 74e6c00c\n\nGoal active/unmet, no budget. Latest completed DIF32 SDK negative pushed74e6c00c, production unchanged. New declared private component /tmp/zopro-phased-complex-products. Three exact Karatsuba products in two Comba phases: D=A-B,S=-(A+B), finish signed real REDC+p, then form exact sums andadd thirdproductC toS for imaginaryREDC. Same3products/2reductions, different from oldshared rowCIOS. No heap/keycache/table/core dispatch. Only2fieldfilesaddedmethods recoveroriginalwholebytes;168others unchanged. Modelbounds32Bprepack/<2^62mixed. Need exactBigInt representative+coeff oracle andArk algebra/chains,4/8laneFq/FrR256/R260,33existingfieldtests/generator. New build-plan frozen with186source/tool/librarypins; inspectactualworkflowhandle status. Futurefixed31blocks4patterns4variants(acceptedordinary/freshordinary/sharedCIOS/phasedComba),496processes100kiterations3x2kprimes,ALLlatencythen4affineprofiles20miterations999Hzusercycles no-buildid-cache. No timingstarted/adoption/SDKGnarkexports. Commit/push completed result before another experiment. Component gain requires separatelyfrozen actualG2callerandwholeSDK follow-up before adoption. Olderheadershistorical.\n\n";
    fs::write(p, text.to_owned() + &old).unwrap();
    println!("Frozen{}buildpins; no new production dispatch", pins.len());
}

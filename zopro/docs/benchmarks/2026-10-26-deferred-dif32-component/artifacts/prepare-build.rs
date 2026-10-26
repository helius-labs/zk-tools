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
    let root = Path::new("/tmp/zopro-deferred-dif32");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("candidate/.xtask/source-pins.json"))).unwrap();
    let accepted = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let field = accepted["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(hash(field), accepted["input_pins"][field].as_str().unwrap());
    let serde="/home/ubuntu/src/zk-tools/zopro/xtask/target/release/deps/libserde_json-cdebb9338c2784ad.rlib";
    let xtask = "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask";
    for p in [
        root.join("experiment.json"),
        root.join("range-argument.md"),
        root.join("field-block.rs.txt"),
        root.join("implement.rs"),
        root.join("source-audit.json"),
        root.join("probe.rs"),
        root.join("oracle/Cargo.toml"),
        root.join("oracle/Cargo.lock"),
        root.join("prepare-build.rs"),
        Path::new(field).into(),
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
        root.join("build-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let old = read("/tmp/zopro-outlined-snapshot-fence/native-plan.json");
    let flags = old["environment"]["RUSTFLAGS"]
        .as_str()
        .unwrap()
        .replace(" --cfg zopro_front_snapshot_fence", "")
        + " --cfg zopro_deferred_dif32";
    let plan = json!({"version":1,"source_pins":root.join("build-input-pins.json"),"environment":{"CARGO_TARGET_DIR":root.join("target"),"CARGO_BUILD_JOBS":"16","RUSTFLAGS":flags},"steps":[
    {"kind":"command","name":"compiler","argv":["rustc","-Vv"]},
    {"kind":"command","name":"generator-check","argv":["python3",root.join("candidate/scripts/ifma8.py"),"--check"]},
    {"kind":"command","name":"field-tests","argv":["cargo","test","--manifest-path",root.join("candidate/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"required_tests":["tests::base_field","tests::scalar_field"],"unfiltered":true,"timeout_seconds":1200},
    {"kind":"command","name":"component-oracles","argv":["cargo","test","--manifest-path",root.join("oracle/Cargo.toml"),"--offline","--locked","--release","--bin","probe","--","--test-threads=1","--nocapture"],"required_tests":["tests::exact_integer_four_lanes","tests::exact_integer_eight_lanes","tests::direct_fourier_four_lanes","tests::direct_fourier_eight_lanes","tests::repeated_tiles_match_existing_field_arithmetic","tests::mixed_representation_is_rejected"],"unfiltered":true,"timeout_seconds":1200},
    {"kind":"command","name":"fresh-bench-build","argv":["cargo","build","--manifest-path",root.join("oracle/Cargo.toml"),"--offline","--locked","--release","--bin","probe"],"timeout_seconds":1200},
    {"kind":"command","name":"accepted-bench-link","argv":["rustc","--edition=2021","-O","-C","target-cpu=native","-C","lto=off","-C","codegen-units=1",root.join("probe.rs"),"--extern",format!("zopro_bn254={field}"),"--extern",format!("serde_json={serde}"),"-L","dependency=/tmp/zopro-grouping-module/target-native/release/deps","-L","dependency=/home/ubuntu/src/zk-tools/zopro/xtask/target/release/deps","-o",root.join("accepted-probe")],"timeout_seconds":1200}
    ]});
    fs::write(
        root.join("build-plan.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    let handoff = Path::new("/home/ubuntu/src/zopro-cpu-perf-handoff.md");
    let old = fs::read_to_string(handoff).unwrap();
    let now="# CURRENT — 2026-10-26: DEFERRED DIF32 COMPONENT BUILDING; LAST PUSH a958e046\n\nGoal ACTIVE/UNMET, no budget. New root /tmp/zopro-deferred-dif32. Mechanism: normalized R260 values up to64p inside fixed5-stageDIF32, canonicalroots<p,64p<R implieseachhighREDC<2p;31endcorrectionpasses vs80lowoutputs normallycorrected, plusonebiasedsubcarry instead conditionalborrowcorrection. Samefieldmultiply,no heap/table/key cache/scheduler/core dispatch. Full independent bounds and researchscope range-argument.md; Scott Slothful reduction primaryabstract read, PDF403, no importedperformanceclaim. Onlynewblocks inifma.rs/generatedifma8.rs; removeblock recoversbothwholefilesbyteforbyte,168otherfilesidentical. Six newprobe tests pending:4/8lane exactBigUintREDC/CIOScoefficientmodel, Arkdirect32DFT with q/column twists, normalizedrawbounds/compactroundtrip, repeatedtiles,mixedrejection. Existingfieldtests/generator required. Frozen31blocks3geometries3variants279processes100000tiles+3x2000primes; then3profiles5milliontiles999Hzcycles:u --no-buildid-cache. No SDKdispatch/proofs; production170manifest/SDKunchanged. COMMIT+PUSH completed experiment. Inspect nativeworkflowhandle, retain failures; no timingreroll. No AWS.\n\n";
    fs::write(handoff, now.to_owned() + &old).unwrap();
    println!(
        "Frozen {} build inputs and six component oracles",
        pins.len()
    );
}

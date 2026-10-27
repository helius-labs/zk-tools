use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-clean-fft-qualification";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
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
    let r = Path::new(R);
    let old = read("/tmp/zopro-clean-fft-leaves/perf-plan.json");
    let mut pins: BTreeMap<String, String> = serde_json::from_value(old["pins"].clone()).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h)
    }
    assert_eq!(
        hash(r.join("concurrent.rs")),
        "131ff918453b8574722732c742300946bb44cccfcf67f46cb24bf51be3a515b0"
    );
    for n in [
        "design.md",
        "warm.rs",
        "forester.rs",
        "concurrent.rs",
        "build.rs",
        "build",
    ] {
        let p = r.join(n);
        pins.insert(p.display().to_string(), hash(&p));
    }
    let mut steps = Vec::new();
    for (name, receipt, artifactlog) in [
        (
            "accepted",
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
            "/tmp/zopro-composed-cpu/native/on-libraries/stdout.log",
        ),
        (
            "composed",
            "/tmp/zopro-clean-fft-leaves/native/candidate/artifacts.json",
            "/tmp/zopro-clean-fft-leaves/native/candidate-libraries/stdout.log",
        ),
    ] {
        let a = read(receipt);
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins.insert(p.clone(), h.as_str().unwrap().into());
        }
        assert!(
            Path::new(artifactlog).exists(),
            "missing exact Cargo artifact log"
        );
        assert_eq!(
            hash(artifactlog),
            a["cargo_artifacts_sha256"].as_str().unwrap()
        );
        pins.insert(artifactlog.into(), hash(artifactlog));
        for (driver, src) in [
            ("warm", "warm.rs"),
            ("concurrent", "concurrent.rs"),
            ("forester", "forester.rs"),
        ] {
            if name == "accepted" && driver == "forester" {
                continue;
            }
            steps.push(json!({"kind":"link-sdk","name":format!("{name}-{driver}"),"artifacts":artifactlog,"driver":r.join(src)}));
        }
    }
    let portable = read("/tmp/zopro-grouping-module/portable-plan.json");
    let mut env = portable["environment"].clone();
    env["CARGO_TARGET_DIR"] = json!(r.join("target-portable"));
    let mut psteps = Vec::new();
    for (package, name, extra) in [
        ("zopro-bn254", "field", vec![]),
        ("zopro-core", "core", vec![]),
        ("zopro-prover", "prover", vec!["--test", "cubic"]),
    ] {
        let mut argv: Vec<String> = [
            "cargo",
            "test",
            "--manifest-path",
            "/tmp/zopro-clean-fft-leaves/candidate/Cargo.toml",
            "--offline",
            "--locked",
            "--release",
            "-p",
            package,
            "--lib",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        argv.extend(extra.into_iter().map(str::to_owned));
        argv.extend(["--".into(), "--test-threads=1".into()]);
        let required = if name == "core" {
            vec!["msm::adaptive_tree_tests::adaptive_boundaries_preserve_outputs_and_bound_retained_record_capacity","msm::tree_prefix_tests::prefix_tree_complete_pairs_tails_and_reuse","solver::sparse_poseidon::tests::every_width_exact_rows_and_adversarial_rejection","solver::sparse_poseidon::tests::raw_operation_indices_above_tag_limit_fall_back"]
        } else {
            vec![]
        };
        psteps.push(json!({"kind":"command","name":name,"argv":argv,"required_tests":required,"unfiltered":true,"timeout_seconds":1200}));
    }
    psteps.push(json!({"kind":"command","name":"libraries","argv":["cargo","build","--manifest-path","/tmp/zopro-clean-fft-leaves/candidate/Cargo.toml","--offline","--locked","--release","-p","zopro-bench","--lib","--message-format=json"],"timeout_seconds":1200}));
    psteps.push(json!({"kind":"link-sdk","name":"portable","artifacts":r.join("portable/libraries/stdout.log"),"driver":"/tmp/zopro-clean-fft-leaves/candidate/crates/zopro-bench/examples/cpu-sdk.rs","additional_sources":["/tmp/zopro-clean-fft-leaves/candidate/crates/zopro-bench/examples/support/perf.rs"],"rustc_flags":["-O","-C","target-cpu=x86-64","-C","lto=fat","-C","codegen-units=1"]}));
    save(r.join("build-pins.json"), &json!(pins));
    save(
        r.join("native-plan.json"),
        &json!({"version":1,"source_pins":r.join("build-pins.json"),"steps":steps}),
    );
    save(
        r.join("portable-plan.json"),
        &json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":env,"steps":psteps}),
    );
    println!("Prepared exact-native qualification links and separate source-identical portable suites/build");
}

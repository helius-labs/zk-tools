use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
// A primary Cargo target may expose a mutable top-level .rlib. Its hashed
// .rmeta artifact names the immutable companion .rlib in deps instead.
fn field_artifact(log: &str) -> String {
    let text = fs::read_to_string(log).unwrap();
    let artifacts: Vec<_> = text
        .lines()
        .filter_map(|s| serde_json::from_str::<Value>(s).ok())
        .filter(|v| v["reason"] == "compiler-artifact" && v["target"]["name"] == "zopro_bn254")
        .collect();
    assert_eq!(artifacts.len(), 1);
    let names = artifacts[0]["filenames"].as_array().unwrap();
    let meta: Vec<_> = names
        .iter()
        .filter_map(|s| {
            s.as_str()
                .filter(|s| s.ends_with(".rmeta") && s.contains("/deps/libzopro_bn254-"))
        })
        .collect();
    assert_eq!(meta.len(), 1);
    let library = Path::new(meta[0]).with_extension("rlib");
    assert!(library.is_file());
    library.display().to_string()
}
fn main() {
    let fresh = field_artifact("native/fresh-library/stdout.log");
    let candidate = field_artifact("native/candidate-library/stdout.log");
    assert_ne!(fresh, candidate);
    assert_ne!(hash(&fresh), hash(&candidate));
    let old: Value = serde_json::from_slice(&fs::read("probe-artifacts.json").unwrap()).unwrap();
    assert_eq!(old["fresh"]["sha256"], old["candidate"]["sha256"]);
    assert_eq!(
        old["fresh"]["library_sha256"],
        old["candidate"]["library_sha256"]
    );
    assert_eq!(
        hash(&candidate),
        old["candidate"]["library_sha256"].as_str().unwrap()
    );
    let off_fingerprint = Path::new(&fresh)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("libzopro_bn254-")
        .unwrap();
    let on_fingerprint = Path::new(&candidate)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_prefix("libzopro_bn254-")
        .unwrap();
    let read = |s: &str| -> Value {
        serde_json::from_slice(
            &fs::read(format!(
                "target/release/.fingerprint/zopro-bn254-{s}/lib-zopro_bn254.json"
            ))
            .unwrap(),
        )
        .unwrap()
    };
    let f = read(off_fingerprint);
    let c = read(on_fingerprint);
    assert!(!f["rustflags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v == "zopro_direct_finish_reduction"));
    assert!(c["rustflags"]
        .as_array()
        .unwrap()
        .iter()
        .any(|v| v == "zopro_direct_finish_reduction"));
    let cmd: Value = serde_json::from_slice(&fs::read("link-fresh-command.json").unwrap()).unwrap();
    let oldlib = old["fresh"]["library"].as_str().unwrap();
    let mut args: Vec<String> = serde_json::from_value(cmd["argv"].clone()).unwrap();
    for arg in &mut args {
        if arg == &format!("zopro_bn254={oldlib}") {
            *arg = format!("zopro_bn254={fresh}");
        }
        if arg == "/home/ubuntu/src/zopro-perf-work/inverse-direct-finish/fresh-probe" {
            *arg = "/home/ubuntu/src/zopro-perf-work/inverse-direct-finish/corrected-fresh-probe"
                .into();
        }
    }
    fs::write(
        "corrected-fresh-link-command.json",
        serde_json::to_vec_pretty(&json!(args)).unwrap(),
    )
    .unwrap();
    let output = Command::new("rustc").args(args).output().unwrap();
    fs::write("corrected-fresh-link-stdout.log", output.stdout).unwrap();
    fs::write("corrected-fresh-link-stderr.log", output.stderr).unwrap();
    assert!(output.status.success());
    let corrected = hash("corrected-fresh-probe");
    assert_ne!(corrected, old["candidate"]["sha256"].as_str().unwrap());
    let audit = json!({"intended_fresh_OFF_is_actually_ON":true,"fresh_candidate_binary_sha256_identical":true,"fresh_candidate_library_sha256_identical":true,"cause":"Cargo primary-target mutable top-level .rlib overwritten by later candidate build before probes linked","immutable_fresh_OFF":{"library":fresh,"sha256":hash(&fresh),"fingerprint":f,"linked_corrected_probe_sha256":corrected,"executed":false},"immutable_candidate_ON":{"library":candidate,"sha256":hash(&candidate),"fingerprint":c},"completed_timing_repeated":false,"timing_qualification_passed":false,"algorithm_speed_decision":"unqualified; not rejected by identical ON/ON comparison","arithmetic_correctness_tests_unchanged":true,"next_requirement":"Use hashed immutable rmeta companion libraries and require distinct OFF/ON hashes before a separate actual-caller study. Preserve completed primitive observations; do not rerun this schedule."});
    fs::write(
        "control-identity-audit.json",
        serde_json::to_vec_pretty(&audit).unwrap(),
    )
    .unwrap();
    println!("ConfirmedidenticalONcontrols;recoveredandlinkedimmutableOFFwithoutcapture");
}

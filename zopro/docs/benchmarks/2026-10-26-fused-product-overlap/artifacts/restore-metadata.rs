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
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let mut original = BTreeMap::new();
    for line in fs::read_to_string(r.join("native/libraries/stdout.log"))
        .unwrap()
        .lines()
    {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            if v["reason"] == "compiler-artifact" {
                if let Some(name) = v["target"]["name"].as_str() {
                    if [
                        "crossbeam_deque",
                        "crossbeam_epoch",
                        "crossbeam_utils",
                        "rayon_core",
                    ]
                    .contains(&name)
                    {
                        for p in v["filenames"].as_array().unwrap() {
                            if p.as_str().unwrap().ends_with(".rlib") {
                                assert!(original
                                    .insert(name.to_owned(), p.as_str().unwrap().to_owned())
                                    .is_none());
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(original.len(), 4);
    let path = r.join("linked-scheduler-libraries-v1.json");
    fs::write(
        &path,
        format!("{}\n", serde_json::to_string_pretty(&original).unwrap()),
    )
    .unwrap();
    let oldpath = r.join("linked-scheduler-libraries.json");
    let expected = read(r.join("input-pins.json"))[oldpath.to_str().unwrap()]
        .as_str()
        .unwrap()
        .to_owned();
    assert_eq!(hash(&path), expected);
    fs::copy(&oldpath, r.join("linked-scheduler-libraries-v2.json")).unwrap();
    let relocation = json!({"original_metadata_path":oldpath,"original_expected_sha256":expected,"preserved_original_metadata":path,"method":"Reconstruct exact original four-library JSON from retained v1 Cargo artifact receipts; require byte SHA equal to frozen v1 input pin.","reason":"v2 protocol reused the unversioned scheduler metadata path after v1 preflight had terminated. No original source, binary, library, proof or command receipt changed; current361pins still refer to v2 metadata and remain unchanged.","current_v2_metadata_sha256":hash(&oldpath),"metadata_relocated_only":true});
    fs::write(
        r.join("pin-relocation.json"),
        serde_json::to_vec_pretty(&relocation).unwrap(),
    )
    .unwrap();
    fs::write(r.join("analysis-invocation-repair.json"),serde_json::to_vec_pretty(&json!({"measurement_session":26157,"perf_status":"complete","initial_summary_command_exit":101,"reader_error":"analyze requires2CLIarguments; invokedwith0 (left0right2) after alltiming/profilesalreadycomplete.","repair":"Invoke frozen analyzer with ROOT OUTPUT; perf-plan.json is a byte-identical copy of perf-plan-v2.json to meet its existing filename interface.","original_analyzer_source_unchanged":true,"config_alias_sha256":hash(r.join("perf-plan.json")),"original_config_sha256":hash(r.join("perf-plan-v2.json")),"measurement_rerun":false,"gate_changes":false,"new_output":"pilot-result.json"})).unwrap()).unwrap();
    println!("Original scheduler metadata recovered byte-exact{}; currentv2metadata retained; analyzerCLIrepair recorded",expected);
}

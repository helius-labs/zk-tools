use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
};
const R: &str = "/tmp/zopro-pmu-c7a-20261027";
const O: &str = "/tmp/zopro-current-c7a-20261027";
const M: &str = "/tmp/zopro-clean-fft-adoption/production-source-manifest.json";
const BASE: &str =
    "558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-cr-c7a-20261027-081750";
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
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    let current = read(M);
    let files = current["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 172);
    for (f, h) in files {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(f)),
            h.as_str().unwrap()
        );
        assert_eq!(hash(r.join("current").join(f)), h.as_str().unwrap());
    }
    for (name, n) in [
        ("current-field", 33),
        ("current-core", 119),
        ("current-prover", 8),
    ] {
        assert_eq!(
            read(r.join(format!("native/{name}/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            n
        );
    }
    let old = read(Path::new(O).join("context/manifest.json"));
    let mut pins = old["container_pins"].as_object().unwrap().clone();
    for key in [
        "/usr/local/bin/cr-current",
        "/usr/local/bin/cr-previous",
        "/usr/local/bin/cr-runner",
    ] {
        assert!(pins.remove(key).is_some());
    }
    let c = r.join("context");
    fs::create_dir(&c).unwrap();
    let receipt = read(r.join("native/current/artifacts.json"));
    assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
    let binary = receipt["binary"].as_str().unwrap();
    assert_eq!(hash(binary), receipt["binary_sha256"].as_str().unwrap());
    for (f, h) in receipt["input_pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    for (source, name, path) in [
        (
            Path::new(binary).to_owned(),
            "cr-current",
            "/usr/local/bin/cr-current",
        ),
        (r.join("runner"), "pm-runner", "/usr/local/bin/pm-runner"),
    ] {
        fs::copy(&source, c.join(name)).unwrap();
        pins.insert(path.into(), json!(hash(source)));
    }
    let before = read("/tmp/zopro-current-pmu/inputs.json");
    let mut seen: BTreeSet<String> =
        serde_json::from_value(before["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 12125);
    for (f, h) in read("/tmp/zopro-current-pmu/proof-inventory.json")
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert!(seen.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(seen.len(), 12143);
    let case = old["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["shape"] == "1x2")
        .unwrap()
        .clone();
    let name = Path::new(case["image"].as_str().unwrap())
        .file_name()
        .unwrap();
    let actual = Path::new(current["basis_images"].as_str().unwrap()).join(name);
    assert_eq!(
        hash(actual),
        pins[case["image"].as_str().unwrap()].as_str().unwrap()
    );
    let pool = read(Path::new(O).join("aeglos-pool-source-identity.json"));
    let aeglos = read("/tmp/zopro-aeglos-windowed-ablation/build-manifest.json");
    assert_eq!(
        aeglos["aeglos_commit"],
        "0290613e8eeca17a4ce88f6f2d726e8e06a5385d"
    );
    assert_eq!(
        hash(aeglos["binary"].as_str().unwrap()),
        aeglos["binary_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(aeglos["native_archive"].as_str().unwrap()),
        aeglos["native_archive_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(pool["local_source"].as_str().unwrap()),
        pool["sha256"].as_str().unwrap()
    );
    assert_eq!(
        pins["/usr/local/bin/aeglos-ablation"],
        aeglos["binary_sha256"]
    );
    save(
        r.join("aeglos-source-identity.json"),
        &json!({"pool":pool,"build":aeglos}),
    );
    let mut helperpins = BTreeMap::new();
    for file in [
        "design.md",
        "runner.rs",
        "support/config.rs",
        "support/results.rs",
        "prepare-native.rs",
        "prepare-image.rs",
        "analyze.rs",
        "close.rs",
    ] {
        helperpins.insert(file.to_owned(), hash(r.join(file)));
    }
    let base = Command::new("docker")
        .args(["image", "inspect", BASE, "--format", "{{.Id}}"])
        .output()
        .unwrap();
    assert!(base.status.success());
    let baseid = String::from_utf8(base.stdout).unwrap().trim().to_owned();
    assert_eq!(
        baseid,
        "sha256:58d95101b875f9108d4508344bba6e38efa50812df2f2abb8e89cefd44c4aa59"
    );
    let manifest = json!({"scope":"current172-source Zopro and pinned Aeglos0290613 primary32/96 warm PMU+profile diagnostic","parent_image_id":baseid,"container_pins":pins,"prior_proof_sha256":seen,"environment":current["environment"],"case":case,"symbol_categories":old["symbol_categories"],"accepted_source_manifest_sha256":hash(M),"source_files":172,"source_changed":false,"design_sha256":hash(r.join("design.md")),"helper_source_hashes":helperpins,"current_rustflags":read(r.join("native-plan.json"))["steps"][2]["environment"]["RUSTFLAGS"],"current_field_tests":33,"current_core_tests":119,"raw_tests":2,"cubic_tests":6,"adopted_new_source":false,"acceptance_measurement":false,"planned_local_calls":8,"planned_local_exports":2,"planned_cloud_calls":13112,"planned_cloud_exports":28,"planned_final_count":12173});
    save(c.join("manifest.json"), &manifest);
    fs::write(c.join("Dockerfile"),format!("FROM {BASE}\nCOPY cr-current pm-runner /usr/local/bin/\nCOPY manifest.json /cr/manifest.json\nENTRYPOINT [\"/usr/local/bin/pm-runner\"]\n")).unwrap();
    let log = fs::File::create(r.join("docker-build.log")).unwrap();
    assert!(Command::new("docker")
        .args(["build", "--pull=false", "-t", "zopro-pmu-c7a:preflight"])
        .arg(&c)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap()
        .success());
    save(
        r.join("image-prepared.json"),
        &json!({"status":"built; no SDK preflight/publication/provisioning yet","container_pins":pins.len(),"parent_image_checked":true,"manifest_sha256":hash(c.join("manifest.json")),"current_sdk_sha256":receipt["binary_sha256"]}),
    );
    println!("Prepared exact172-source C7a/current+Aeglos preflight image; no AWS resources");
}

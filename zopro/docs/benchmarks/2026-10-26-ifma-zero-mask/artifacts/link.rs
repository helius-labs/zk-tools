use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    let log = fs::read_to_string(root.join("build/field/stdout.log")).unwrap();
    let mut libs = Vec::new();
    for line in log.lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            if v["reason"] == "compiler-artifact" && v["target"]["name"] == "zopro_bn254" {
                for p in v["filenames"].as_array().unwrap() {
                    if p.as_str().unwrap().ends_with(".rlib") {
                        libs.push(p.as_str().unwrap().to_owned());
                    }
                }
            }
        }
    }
    assert_eq!(libs.len(), 1);
    let artifact: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-grouping-module/native/candidate/artifacts.json").unwrap(),
    )
    .unwrap();
    let accepted = artifact["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(
        hash(accepted),
        artifact["input_pins"][accepted].as_str().unwrap()
    );
    let mut links = Vec::new();
    for (label, lib) in [("accepted", accepted), ("candidate", libs[0].as_str())] {
        let deps = Path::new(lib).parent().unwrap();
        let argv = vec![
            "--edition=2021".to_owned(),
            "-O".into(),
            "-C".into(),
            "target-cpu=native".into(),
            "-C".into(),
            "lto=off".into(),
            "-C".into(),
            "codegen-units=1".into(),
            root.join("field-probe.rs").to_str().unwrap().into(),
            "--extern".into(),
            format!("zopro_bn254={lib}"),
            "-L".into(),
            format!("dependency={}", deps.display()),
            "-o".into(),
            root.join(format!("{label}-probe")).to_str().unwrap().into(),
        ];
        let out = Command::new("rustc").args(&argv).output().unwrap();
        fs::write(root.join(format!("link-{label}.stdout.log")), out.stdout).unwrap();
        fs::write(root.join(format!("link-{label}.stderr.log")), out.stderr).unwrap();
        assert!(out.status.success());
        links.push(json!({"label":label,"argv":argv,"library":lib,"library_sha256":hash(lib),"probe_sha256":hash(root.join(format!("{label}-probe"))),"returncode":out.status.code()}));
    }
    fs::write(
        root.join("links.json"),
        serde_json::to_vec_pretty(&json!(links)).unwrap(),
    )
    .unwrap();
    println!("Linked both24wrapperfield probes from exactlibraries; not timed");
}

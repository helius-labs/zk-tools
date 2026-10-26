use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
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
    let read = |p: &Path| -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() };
    let mut plan = read(&root.join("profile-plan.json"));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(&root.join("profile-input-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "content pin {p}");
    }
    let exit = read(&root.join("profiles/capture-reference/exit.json"));
    let receipt = Command::new("sudo")
        .args(["-n", "find", "/root/.debug", "-samefile"])
        .arg(root.join("probe"))
        .arg("-print")
        .output()
        .unwrap();
    assert!(receipt.status.success());
    let cache = String::from_utf8(receipt.stdout).unwrap();
    assert!(cache.contains("/root/.debug/tmp/zopro-fq2-component-lanes/probe/"));
    fs::write(root.join("profile-repair.json"),serde_json::to_vec_pretty(&json!({"cause":"First perf capture completed, then workflow rejected changed executable inode metadata. perf created an actual hard link to the pinned executable in its build-id cache; bytes and mtime remain unchanged. All200content pins rechecked.","command_exit":exit,"same_inode_cache_path":cache.trim(),"repair":"Reuse the existing successful reference capture; do not repeat it. Decode it, then capture interleaved with --no-buildid-cache. All unprofiled measurements are unchanged. Subsequent guard takes current inode metadata and retains all content pins.","reference_capture_repeated":false,"latency_repeated":false})).unwrap()).unwrap();
    let mut steps = plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>();
    for step in &mut steps {
        if step["name"] == "capture-interleaved" {
            let argv = step["argv"].as_array_mut().unwrap();
            let i = argv.iter().position(|x| x == "record").unwrap();
            argv.insert(i + 1, json!("--no-buildid-cache"));
        }
    }
    plan["steps"] = json!(steps);
    plan["source_pins"] = json!(root.join("profile-input-pins-v2.json"));
    for name in [
        "repair-profile.rs",
        "profile-repair.json",
        "profiles/failure.json",
        "profiles/capture-reference/exit.json",
        "profiles/capture-reference/stdout.log",
        "profiles/capture-reference/stderr.log",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    fs::write(
        root.join("profile-input-pins-v2.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("profile-plan-v2.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    println!(
        "All content pins unchanged; retain successful first capture, execute remaining five steps"
    );
}

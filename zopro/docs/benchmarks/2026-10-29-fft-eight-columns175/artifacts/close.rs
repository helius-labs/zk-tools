use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fft-eight-columns175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-fft-eight-columns175";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
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
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let n = e.file_name().to_string_lossy().to_string();
        if e.file_type().unwrap().is_dir() {
            if !n.starts_with("target-") && n != "frozen" {
                walk(&e.path(), out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn seal() {
    let d = Path::new(D);
    let mut paths = Vec::new();
    walk(d, &mut paths);
    let mut pins = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if ["artifact-index.json", "SHA256SUMS"].contains(&rel.as_str()) {
            continue;
        }
        pins.insert(rel, hash(p));
    }
    save(d.join("artifact-index.json"), &json!(pins));
    pins.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        pins.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Sealed {} hashes", pins.len());
}
fn archive() {
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let a = read(r.join("analysis.json"));
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    assert_eq!(a["independent_proof_count"], 17317);
    assert_eq!(a["new_independent_exports"], 78);
    assert_eq!(a["production_changed"], false);
    assert_eq!(a["adopted"], false);
    assert_eq!(a["accepted_full_performance_gate_passed"], false);
    for (p, h) in read(r.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut paths = Vec::new();
    walk(r, &mut paths);
    let mut binaries = BTreeMap::new();
    let mut data = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(r).unwrap();
        let name = p.file_name().unwrap().to_str().unwrap();
        if name.starts_with("git-")
            || name.ends_with(".note")
            || [
                "archive.log",
                "archive.stderr.log",
                "strict-hashes.log",
                "commit.log",
                "push.log",
                "remote.log",
            ]
            .contains(&name)
        {
            continue;
        }
        if p.extension().is_some_and(|s| s == "data") {
            assert!(rel.starts_with("perf"));
            assert!(Command::new("sudo")
                .args(["-n", "chown", "ubuntu:ubuntu"])
                .arg(&p)
                .status()
                .unwrap()
                .success());
            data.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let mut f = fs::File::open(&p).unwrap();
        let mut header = [0; 8];
        let n = f.read(&mut header).unwrap();
        if header[..n].starts_with(b"\x7fELF") || header[..n].starts_with(b"!<arch>") {
            binaries.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let to = d.join("artifacts").join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&p, &to).unwrap();
        assert_eq!(hash(p), hash(to));
    }
    assert_eq!(data.len(), 3);
    save(d.join("artifacts/local-perf-data.json"), &json!(data));
    save(d.join("artifacts/local-binary-pins.json"), &json!(binaries));
    let mut source = Vec::new();
    walk(&d.join("artifacts/source"), &mut source);
    assert_eq!(source.len(), 176);
    assert!(Command::new(r.join("report-helper"))
        .status()
        .unwrap()
        .success());
    seal();
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive/seal"),
    }
}

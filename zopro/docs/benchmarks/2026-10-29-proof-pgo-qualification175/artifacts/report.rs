use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-qualification175";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-proof-pgo-qualification175";
const M:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash(p: impl AsRef<Path>) -> String {
    digest(&fs::read(p).unwrap())
}
fn encoded(b: &[u8]) -> Value {
    match std::str::from_utf8(b) {
        Ok(s) => json!({"encoding":"utf8","data":s,"sha256":digest(b)}),
        Err(_) => {
            json!({"encoding":"hex","data":b.iter().map(|c|format!("{c:02x}")).collect::<String>(),"sha256":digest(b)})
        }
    }
}
fn walk(p: &Path, files: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), files)
        } else if e.file_type().unwrap().is_file() {
            files.push(e.path());
        }
    }
}
fn compressed(d: &Path, name: &str, value: &Value) {
    let bytes = serde_json::to_vec(value).unwrap();
    let raw = r(format!("{name}.json"));
    fs::write(&raw, &bytes).unwrap();
    let target = d.join(format!("{name}.json.gz"));
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&raw)
        .stdout(Stdio::from(fs::File::create(&target).unwrap()))
        .status()
        .unwrap()
        .success());
    let o = Command::new("gzip")
        .arg("-dc")
        .arg(target)
        .output()
        .unwrap();
    assert!(o.status.success());
    assert_eq!(o.stdout, bytes);
}
fn seal() {
    let d = Path::new(D);
    let mut paths = vec![];
    walk(d, &mut paths);
    let mut sums = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if rel != "artifact-index.json" && rel != "SHA256SUMS" {
            sums.insert(rel, hash(p));
        }
    }
    save(d.join("artifact-index.json"), &json!(sums));
    sums.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        sums.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    for (p, h) in &sums {
        assert_eq!(hash(d.join(p)), *h);
    }
    println!("Sealed {} hashes / {} files", sums.len(), sums.len() + 1);
}
include!("report-body.rs");
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive|seal"),
    }
}

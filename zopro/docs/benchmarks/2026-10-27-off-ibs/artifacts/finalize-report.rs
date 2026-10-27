use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/off-ibs";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-off-ibs";
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap(); let mut h = Sha256::new(); let mut b = [0; 65536];
    loop { let n = f.read(&mut b).unwrap(); if n == 0 { break; } h.update(&b[..n]); }
    format!("{:x}", h.finalize())
}
fn main() {
    let r=Path::new(R); let d=Path::new(D); let p=d.join("artifact-index.json");
    let mut index:Value=serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    for (f,h) in index.as_object().unwrap() { assert_eq!(hash(d.join("artifacts").join(f)),h.as_str().unwrap()); }
    for (from,to) in [
        ("decode-runs/script-b0-off-ownership-failure/stderr.log","postprocess-ownership-error.log"),
        ("decode-runs/script-b0-off-ownership-failure/exit.json","postprocess-ownership-error-exit.json"),
        ("decode-runs/script-b0-off-ownership-failure/command.json","postprocess-ownership-error-command.json"),
        ("finish.log","closeout-finish.log"),
        ("finalize-report.rs","finalize-report.rs"),
    ] {
        assert!(index.get(to).is_none()); let dest=d.join("artifacts").join(to);
        fs::copy(r.join(from),&dest).unwrap(); assert_eq!(hash(r.join(from)),hash(&dest));
        index[to]=json!(hash(dest));
    }
    fs::write(p,serde_json::to_vec_pretty(&index).unwrap()).unwrap();
    println!("Checked original archive and added closure receipts: {} artifact hashes",index.as_object().unwrap().len());
}

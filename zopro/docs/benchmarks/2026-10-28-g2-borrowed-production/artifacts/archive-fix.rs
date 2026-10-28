use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-production";
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
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let old=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json");
    let mut source_counts = BTreeMap::new();
    for root in ["source", "control-source"] {
        let mut files = old["source_files_sha256"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        if root == "source" {
            for f in [
                "crates/zopro-core/src/msm/g2_borrowed.rs",
                "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
            ] {
                files.push(f.into());
            }
        }
        let mut pins = BTreeMap::new();
        for f in files {
            let from = r.join(root).join(&f);
            let to = d.join("artifacts").join(root).join(&f);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(&from, &to).unwrap();
            let h = hash(&from);
            assert_eq!(hash(to), h);
            pins.insert(f, h);
        }
        source_counts.insert(root, pins.len());
        fs::write(
            d.join(format!("artifacts/{root}-complete-pins.json")),
            serde_json::to_vec_pretty(&pins).unwrap(),
        )
        .unwrap();
    }
    let receipt = json!({"source_counts":source_counts,"reason":"generic extension filter omitted non-text-extension inputs; copy every declared source file before publication","timing_or_verification_repeated":false,"captured_helper_changed":false});
    fs::write(
        d.join("artifacts/archive-coverage.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    let mut files = vec![];
    walk(d, &mut files);
    files.sort();
    let mut index = BTreeMap::new();
    for f in files {
        let rel = f.strip_prefix(d).unwrap().display().to_string();
        if ["SHA256SUMS", "artifact-index.json"].contains(&rel.as_str()) {
            continue;
        }
        index.insert(rel, hash(f));
    }
    fs::write(
        d.join("artifact-index.json"),
        serde_json::to_vec_pretty(&index).unwrap(),
    )
    .unwrap();
    index.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!(
        "Complete source snapshots174/172;{}hashed archive members",
        index.len()
    );
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
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
fn file(src: &Path, dst: &Path, root: &Path, index: &mut BTreeMap<String, String>) {
    assert!(src.is_file());
    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    let h = hash(src);
    assert!(!dst.exists());
    fs::copy(src, dst).unwrap();
    assert_eq!(hash(dst), h);
    assert!(index
        .insert(dst.strip_prefix(root).unwrap().to_str().unwrap().into(), h)
        .is_none());
}
fn tree(src: &Path, dst: &Path, root: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(src).unwrap() {
        let p = e.unwrap().path();
        let d = dst.join(p.file_name().unwrap());
        if p.is_dir() {
            tree(&p, &d, root, index)
        } else {
            file(&p, &d, root, index)
        }
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-instruction-census");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-instruction-census");
    let dst = report.join("artifacts");
    assert!(!dst.exists());
    let mut index = BTreeMap::new();
    for name in [
        "experiment.json",
        "analysis-template.json",
        "prepare-extraction.rs",
        "extraction-pins.json",
        "extraction-plan.json",
        "extraction.log",
        "source-provenance.json",
        "prepare-analysis.rs",
        "analysis-config.json",
        "analysis-pins.json",
        "analysis-plan.json",
        "analysis.log",
        "invalid-hash-config.json",
        "invalid-hash-check.sh",
        "invalid-hash.stdout.log",
        "invalid-hash.stderr.log",
        "invalid-hash.exit.json",
        "tool-tests.log",
        "tool-clippy.log",
        "tool-build.log",
        "tool-format.log",
        "xtask-Cargo.toml",
        "xtask-Cargo.lock",
        "audit.rs",
        "audit.log",
        "closeout.json",
        "instruction-summary.json",
        "g2-primary-sites.json",
        "write-report.rs",
        "publish.rs",
    ] {
        file(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    for name in ["extraction", "analysis-workflow", "analysis", "tool-before"] {
        tree(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    tree(
        Path::new("/home/ubuntu/src/zk-tools/zopro/xtask/src"),
        &dst.join("tool-source"),
        &dst,
        &mut index,
    );
    file(
        Path::new("/home/ubuntu/src/zk-tools/zopro/xtask/README.md"),
        &dst.join("tool-source/README.md"),
        &dst,
        &mut index,
    );
    let excluded = index
        .keys()
        .filter(|p| {
            p.starts_with("extraction/")
                && p.ends_with("/stdout.log")
                && (p.starts_with("extraction/metadata-") || p.starts_with("extraction/objdump-"))
        })
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(excluded.len(), 11);
    let before = read(root.join("closeout.json"));
    assert_eq!(before["all_samples"], 297412);
    assert_eq!(before["new_benchmark_sdk_calls"], 0);
    fs::write(
        dst.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    let expected = index.len() - excluded.len() + 5;
    fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":expected,"sha256":index,"local_only":excluded,"scope":"Complete offline reader diagnostic: source, input pins, command receipts, tests, all nine per-IP and per-function outputs. Nine expanded metadata streams and two full disassemblies local-only, hash-indexed; no new proof/capture. Original raw/compact files retain parent archive identities."})).unwrap()).unwrap();
    fs::write(
        report.join(".gitignore"),
        "/artifacts/extraction/metadata-*/stdout.log\n/artifacts/extraction/objdump-*/stdout.log\n",
    )
    .unwrap();
    fs::write(report.join(".gitattributes"), "artifacts/** -whitespace\n").unwrap();
    println!("Published {} hash-indexed files; {expected} expected Git files; 11 large extraction streams local-only",index.len());
}

use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: &Path) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn file(src: &Path, dst: &Path, root: &Path, index: &mut BTreeMap<String, String>) {
    assert!(!dst.exists());
    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    let h = hash(src);
    fs::copy(src, dst).unwrap();
    assert_eq!(hash(dst), h);
    assert!(index
        .insert(dst.strip_prefix(root).unwrap().to_str().unwrap().into(), h)
        .is_none());
}
fn tree(src: &Path, dst: &Path, root: &Path, index: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(src).unwrap() {
        let p = entry.unwrap().path();
        let d = dst.join(p.file_name().unwrap());
        if p.is_dir() {
            tree(&p, &d, root, index)
        } else {
            assert!(p.is_file());
            file(&p, &d, root, index);
        }
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-ifma-zero-mask");
    let dst = report.join("artifacts");
    assert!(!dst.exists());
    let mut index = BTreeMap::new();
    for n in [
        "experiment.json",
        "prepare.json",
        "prepare.rs",
        "edits.json",
        "accepted-manifest.json",
        "field-probe.rs",
        "wrappers.json",
        "isa.rs",
        "link.rs",
        "links.json",
        "link-accepted.stdout.log",
        "link-accepted.stderr.log",
        "link-candidate.stdout.log",
        "link-candidate.stderr.log",
        "build.rs",
        "build-pins.json",
        "build-plan.json",
        "build.log",
        "source-audit.json",
        "analyze.rs",
        "codegen.json",
        "codegen.txt",
        "accepted-symbols.txt",
        "candidate-symbols.txt",
        "isa-symbols.txt",
        "accepted-mask48-data.log",
        "closeout.rs",
        "closeout-initial.rs",
        "closeout-repair.json",
        "closeout.json",
        "write-report.rs",
        "publish.rs",
    ] {
        file(&root.join(n), &dst.join(n), &dst, &mut index);
    }
    tree(&root.join("build"), &dst.join("build"), &dst, &mut index);
    for n in [
        "crates/zopro-bn254/src/ifma.rs",
        "crates/zopro-bn254/src/ifma8.rs",
    ] {
        file(
            &root.join("candidate").join(n),
            &dst.join("candidate").join(n),
            &dst,
            &mut index,
        );
        file(
            &Path::new("/home/ubuntu/src/zk-tools/zopro").join(n),
            &dst.join("original").join(n),
            &dst,
            &mut index,
        );
    }
    for n in [
        "Cargo.toml",
        "Cargo.lock",
        "crates/zopro-bn254/Cargo.toml",
        "scripts/ifma8.py",
        ".xtask/snapshot.json",
        ".xtask/source-pins.json",
    ] {
        file(
            &root.join("candidate").join(n),
            &dst.join("candidate").join(n),
            &dst,
            &mut index,
        );
    }
    let expected = index.len() + 5;
    fs::write(
        dst.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":expected,"sha256":index,"scope":"Bounded ISA/compiler audit, all58 exact disassemblies, both changed field sources and originals, source manifest/reconstruction metadata, commands/receipts/conformance. Other168unmodified sources remain atacceptedmanifest digests. No new runtimebenchmark, proof or profile. Executables/libraries/compiler caches represented by exactsource/input hashes."})).unwrap()).unwrap();
    fs::write(
        report.join(".gitignore"),
        "/artifacts/*-probe\n/artifacts/isa\n/artifacts/target/\n",
    )
    .unwrap();
    fs::write(report.join(".gitattributes"), "artifacts/** -whitespace\n").unwrap();
    println!(
        "Published{}hashfiles expectedGit{expected}; alltext, no newproofs/profiles/binaries",
        index.len()
    );
}

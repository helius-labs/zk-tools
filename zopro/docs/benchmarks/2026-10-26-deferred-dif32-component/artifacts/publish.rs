use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
};
fn hash(p: &Path) -> String {
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
    assert!(src.symlink_metadata().unwrap().file_type().is_file());
    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    let digest = if src.extension().is_some_and(|e| e == "data") {
        let before = fs::metadata(src).unwrap();
        let h = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(src)
            .output()
            .unwrap();
        assert!(h.status.success());
        let digest = String::from_utf8(h.stdout)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_owned();
        let out = fs::File::create(dst).unwrap();
        assert!(Command::new("sudo")
            .args(["-n", "cat"])
            .arg(src)
            .stdout(Stdio::from(out))
            .status()
            .unwrap()
            .success());
        let after = fs::metadata(src).unwrap();
        assert_eq!(before.len(), after.len());
        assert_eq!(before.modified().unwrap(), after.modified().unwrap());
        digest
    } else {
        fs::copy(src, dst).unwrap();
        hash(src)
    };
    assert_eq!(hash(dst), digest);
    assert!(index
        .insert(
            dst.strip_prefix(root).unwrap().to_str().unwrap().into(),
            digest
        )
        .is_none());
}
fn tree(src: &Path, dst: &Path, root: &Path, index: &mut BTreeMap<String, String>) {
    let mut v = fs::read_dir(src)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<_>>();
    v.sort();
    for p in v {
        let out = dst.join(p.file_name().unwrap());
        if p.symlink_metadata().unwrap().file_type().is_dir() {
            tree(&p, &out, root, index)
        } else {
            file(&p, &out, root, index)
        }
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32");
    let report = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-deferred-dif32-component",
    );
    let dst = report.join("artifacts");
    assert!(!dst.exists());
    let mut index = BTreeMap::new();
    for name in [
        "experiment.json",
        "range-argument.md",
        "field-block.rs.txt",
        "implement.rs",
        "prepare.json",
        "source-audit.json",
        "generator.stdout.log",
        "generator.stderr.log",
        "lock.log",
        "accepted-manifest.json",
        "probe.rs",
        "probe-initial.rs",
        "prepare-build.rs",
        "repair.rs",
        "repair.json",
        "repair-parser.rs",
        "oracle-qualification.json",
        "build-input-pins.json",
        "build-input-pins-v2.json",
        "build-input-pins-v3.json",
        "build-plan.json",
        "build-plan-initial.json",
        "build-plan-v2.json",
        "build-plan-v3.json",
        "build.log",
        "build-v2.log",
        "build-v3.log",
        "timing.rs",
        "analyze.rs",
        "timing-input-pins.json",
        "timing-order.json",
        "timing-plan.json",
        "timing.log",
        "fresh-field-libraries.json",
        "component-result.json",
        "profile.rs",
        "profile-input-pins.json",
        "profile-plan.json",
        "profiles.log",
        "codegen.json",
        "codegen.txt",
        "profile-analysis.json",
        "closeout.rs",
        "closeout.json",
        "write-report.rs",
        "publish.rs",
        "accepted.perf.data",
        "fresh.perf.data",
        "deferred.perf.data",
    ] {
        file(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    for name in [
        "candidate",
        "oracle",
        "build",
        "build-v2",
        "build-v3",
        "timing",
        "profiles",
    ] {
        tree(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    let raw = index.keys().filter(|p| p.ends_with(".perf.data")).count();
    let expanded = index
        .keys()
        .filter(|p| {
            (p.starts_with("profiles/decode-") || p.starts_with("profiles/loss-"))
                && p.ends_with("/stdout.log")
        })
        .count();
    assert_eq!(raw, 3);
    assert_eq!(expanded, 6);
    let fixtures = index
        .keys()
        .filter(|p| {
            p.starts_with("candidate/") && (p.ends_with(".proof") || p.ends_with(".public"))
        })
        .count();
    assert_eq!(fixtures, 2);
    let staged = index.len() - raw - expanded + 5;
    fs::write(
        dst.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":staged,"raw_local_only":raw,"expanded_local_only":expanded,"fixed_source_fixture_members":fixtures,"new_sdk_proofs":0,"sha256":index,"scope":"Hash-verified source/build/model/measurement/profile archive; exact executable/library hashes retained instead of compiler caches/binaries. Three raw captures and six expanded streams retained locally, ignored Git."})).unwrap()).unwrap();
    fs::write(report.join(".gitignore"),"/artifacts/*.perf.data\n/artifacts/profiles/decode-*/stdout.log\n/artifacts/profiles/loss-*/stdout.log\n").unwrap();
    fs::write(report.join(".gitattributes"), "artifacts/** -whitespace\n").unwrap();
    println!(
        "Archive{}files, expectedGit{staged};3raw6expanded local-only, no new proofs",
        index.len()
    );
}

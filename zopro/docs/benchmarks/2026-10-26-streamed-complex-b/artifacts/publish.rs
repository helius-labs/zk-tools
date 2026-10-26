use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    os::unix::fs::FileTypeExt,
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
    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    let digest = if src.file_name().is_some_and(|n| n == "perf.data") {
        let before = fs::metadata(src).unwrap();
        let o = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(src)
            .output()
            .unwrap();
        assert!(o.status.success());
        let h = String::from_utf8(o.stdout)
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
        h
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
    let mut entries = fs::read_dir(src)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<_>>();
    entries.sort();
    for p in entries {
        let name = p.file_name().unwrap();
        let kind = p.symlink_metadata().unwrap().file_type();
        if kind.is_dir() {
            tree(&p, &dst.join(name), root, index)
        } else if kind.is_fifo() {
            assert!(name == "ctl" || name == "ack");
        } else {
            assert!(kind.is_file());
            if name != "candidate-sdk" {
                file(&p, &dst.join(name), root, index);
            }
        }
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-streamed-complex-b");
    let dst = report.join("artifacts");
    assert!(!dst.exists());
    let mut index = BTreeMap::new();
    for name in [
        "experiment.json",
        "range-argument.md",
        "accepted-manifest.json",
        "prepare.json",
        "implement.rs",
        "field-additions.rs.txt",
        "source-audit.json",
        "generator.stdout.log",
        "generator.stderr.log",
        "probe.rs",
        "probe-initial.rs",
        "probe-tests.rs",
        "row-model.rs.txt",
        "prepare-oracle.rs",
        "prepare-build.rs",
        "build-input-pins.json",
        "build-plan.json",
        "build.log",
        "repair-harness.rs",
        "harness-repair.json",
        "build-input-pins-v2.json",
        "build-plan-v2.json",
        "build-v2.log",
        "setup-timing.rs",
        "timing.rs",
        "analyze.rs",
        "codegen.json",
        "codegen.txt",
        "codegen-screen.json",
        "timing-input-pins.json",
        "timing-order.json",
        "timing-plan.json",
        "timing.log",
        "component-result.json",
        "profile.rs",
        "profile-input-pins.json",
        "profile-plan.json",
        "profiles.log",
        "prepare-closeout.rs",
        "closeout.rs",
        "closeout.json",
        "profile-analysis.json",
        "write-report.rs",
        "prepare-publish.rs",
        "publish.rs",
        "update-work.rs",
        "work-state.json",
    ] {
        file(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    for name in [
        "candidate",
        "oracle",
        "build",
        "build-v2",
        "timing",
        "profiles",
        "profiles-data",
    ] {
        tree(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    let raw = index.keys().filter(|p| p.ends_with("/perf.data")).count();
    let expanded = index
        .keys()
        .filter(|p| {
            p.starts_with("profiles/")
                && (p.contains("/decode-") || p.contains("/loss-"))
                && p.ends_with("/stdout.log")
        })
        .count();
    assert_eq!(raw, 4);
    assert_eq!(expanded, 8);
    let fixtures = index
        .keys()
        .filter(|p| p.ends_with(".proof") || p.ends_with(".public"))
        .count();
    assert_eq!(fixtures, 2);
    assert!(index
        .keys()
        .filter(|p| p.ends_with(".proof") || p.ends_with(".public"))
        .all(|p| p.starts_with("candidate/")));
    let expected = index.len() - raw - expanded + 5;
    fs::write(
        dst.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":expected,"raw_local_only":raw,"expanded_local_only":expanded,"new_proof_public_members":0,"additional_source_fixture_members":fixtures,"sha256":index,"scope":"Complete field component source, declared protocol, both test-harness versions, build/test receipts, all496observations, four profiles and full analyses. Four raw captures and eight expanded streams local-only; executables/caches represented by pinned hashes and sources. Component passed; no SDK/prover adoption."})).unwrap()).unwrap();
    fs::write(report.join(".gitignore"),"/artifacts/profiles-data/**/perf.data\n/artifacts/profiles/decode-*/stdout.log\n/artifacts/profiles/loss-*/stdout.log\n").unwrap();
    fs::write(report.join(".gitattributes"), "artifacts/** -whitespace\n").unwrap();
    println!(
        "Archive{}hashfiles expectedGit{expected},4raw8expandedlocalonly,0newproof2fixedfixtures",
        index.len()
    );
}

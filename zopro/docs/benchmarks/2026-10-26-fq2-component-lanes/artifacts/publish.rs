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
    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    let digest = if src.extension().is_some_and(|n| n == "data") {
        let before = fs::metadata(src).unwrap();
        let h = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(src)
            .output()
            .unwrap();
        assert!(h.status.success());
        let h = String::from_utf8(h.stdout)
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
        let q = dst.join(p.file_name().unwrap());
        let t = p.symlink_metadata().unwrap().file_type();
        assert!(!t.is_symlink());
        if t.is_dir() {
            tree(&p, &q, root, index)
        } else {
            assert!(t.is_file());
            file(&p, &q, root, index)
        }
    }
}
fn main() {
    let src = Path::new("/tmp/zopro-fq2-component-lanes");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-fq2-component-lanes");
    let dst = report.join("artifacts");
    assert!(!dst.exists());
    let mut index = BTreeMap::new();
    for name in [
        "experiment.json",
        "design.md",
        "probe.rs",
        "probe-initial.rs",
        "prepare.rs",
        "repair.rs",
        "repair.json",
        "timing.rs",
        "analyze.rs",
        "profile.rs",
        "repair-profile.rs",
        "profile-repair.json",
        "closeout.rs",
        "write-report.rs",
        "publish.rs",
        "accepted-manifest.json",
        "input-pins.json",
        "input-pins-initial.json",
        "input-pins-v2.json",
        "build-plan.json",
        "build-plan-initial.json",
        "build-plan-v2.json",
        "build.log",
        "build-v2.log",
        "timing-input-pins.json",
        "timing-order.json",
        "timing-plan.json",
        "timing.log",
        "component-result.json",
        "profile-input-pins.json",
        "profile-input-pins-v2.json",
        "profile-plan.json",
        "profile-plan-v2.json",
        "profiles.log",
        "profiles-v2.log",
        "codegen.json",
        "codegen.txt",
        "helper-codegen.json",
        "helper-codegen.txt",
        "profile-analysis.json",
        "closeout.json",
        "reference.perf.data",
        "interleaved.perf.data",
    ] {
        file(&src.join(name), &dst.join(name), &dst, &mut index);
    }
    for name in [
        "accepted-source",
        "oracle",
        "build",
        "build-v2",
        "timing",
        "profiles",
        "profiles-v2",
    ] {
        tree(&src.join(name), &dst.join(name), &dst, &mut index);
    }
    let raw = index.keys().filter(|p| p.ends_with(".perf.data")).count();
    let expanded = index
        .keys()
        .filter(|p| {
            p.starts_with("profiles-v2/")
                && (p.contains("/decode-") || p.contains("/loss-"))
                && p.ends_with("/stdout.log")
        })
        .count();
    assert_eq!(raw, 2);
    // Decode/loss names are immediate children, not nested path substrings.
    let expanded = index
        .keys()
        .filter(|p| p.starts_with("profiles-v2/decode-") || p.starts_with("profiles-v2/loss-"))
        .filter(|p| p.ends_with("/stdout.log"))
        .count();
    assert_eq!(expanded, 4);
    fs::write(
        dst.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"raw_perf_captures":raw,"expanded_streams":expanded,"additional_source_fixture_members":2,"new_sdk_proofs":0,"sha256":index,"scope":"Complete frozen source/protocol/build/oracle/latency/profile/analysis archive. Binary and library identities retained in pins rather than copying executable/build caches. Raw captures and four expanded streams local-only."})).unwrap()).unwrap();
    fs::write(report.join(".gitignore"),"/artifacts/*.perf.data\n/artifacts/profiles-v2/decode-*/stdout.log\n/artifacts/profiles-v2/loss-*/stdout.log\n").unwrap();
    fs::write(report.join(".gitattributes"), "artifacts/** -whitespace\n").unwrap();
    println!(
        "ARCHIVE VERIFIED {} files,2raw4expanded local-only,zero new proofs",
        index.len()
    );
}

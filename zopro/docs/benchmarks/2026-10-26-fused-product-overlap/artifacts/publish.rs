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
    let root = Path::new("/tmp/zopro-fused-product-overlap");
    let report = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-fused-product-overlap",
    );
    let dst = report.join("artifacts");
    assert!(!dst.exists());
    let mut index = BTreeMap::new();
    for name in [
        "experiment.json",
        "accepted-manifest.json",
        "prepare.json",
        "prepare-v2.json",
        "overlap-block.rs.txt",
        "overlap-block-v2.rs.txt",
        "implement.rs",
        "implement-v2.rs",
        "source-audit.json",
        "source-audit-v2.json",
        "prepare-build.rs",
        "build-input-pins.json",
        "native-plan.json",
        "native.log",
        "repair.rs",
        "repair.json",
        "pin-repair.rs",
        "build-input-pins-v2-draft.json",
        "build-input-pins-v2.json",
        "native-plan-v2.json",
        "native-v2.log",
        "prepare-protocol.rs",
        "prepare-protocol-v2.rs",
        "protocol.rs",
        "protocol-v2.rs",
        "layout.rs",
        "activation-audit.rs",
        "activation-audit-v2.rs",
        "activation-audit-v2.json",
        "all-nine-template.json",
        "all-nine-template-v2.json",
        "perf-template.json",
        "perf-template-v2.json",
        "inventory-preflight.sh",
        "inventory-preflight-v2.sh",
        "validate-preflight.jq",
        "validate-preflight-v2.jq",
        "input-pins.json",
        "input-pins-v2.json",
        "linked-scheduler-libraries.json",
        "linked-scheduler-libraries-v1.json",
        "linked-scheduler-libraries-v2.json",
        "preflight-plan.json",
        "preflight-plan-v2.json",
        "preflight.log",
        "preflight-v2.log",
        "initial-proof-inventory.json",
        "preflight-proof-inventory-v2.json",
        "prepare-perf.rs",
        "prepare-perf-v2.rs",
        "perf-plan.json",
        "perf-plan-v2.json",
        "perf.log",
        "analyze.rs",
        "analysis.stdout.log",
        "analysis.stderr.log",
        "analysis-invocation-repair.json",
        "pilot-result.json",
        "pin-audit.rs",
        "pin-diff-audit.json",
        "restore-metadata.rs",
        "pin-relocation.json",
        "codegen.rs",
        "codegen.json",
        "codegen.txt",
        "configuration-profile-notes.json",
        "closeout.rs",
        "closeout.json",
        "proof-inventory.json",
        "profile-analysis.json",
        "validation-notes.json",
        "write-report.rs",
        "prepare-publish.rs",
        "publish.rs",
        "work-state.json",
    ] {
        file(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    for name in [
        "candidate",
        "candidate-v2",
        "native",
        "native-v2",
        "preflight",
        "preflight-v2",
        "preflight-proofs",
        "preflight-proofs-v2",
        "perf",
    ] {
        tree(&root.join(name), &dst.join(name), &dst, &mut index);
    }
    let raw = index.keys().filter(|p| p.ends_with("/perf.data")).count();
    let expanded = index
        .keys()
        .filter(|p| p.ends_with("/symbols/stdout.log"))
        .count();
    assert_eq!(raw, 9);
    assert_eq!(expanded, 9);
    let new_members = index
        .keys()
        .filter(|p| {
            (p.starts_with("perf/")
                || p.starts_with("preflight-proofs/")
                || p.starts_with("preflight-proofs-v2/"))
                && (p.ends_with(".proof") || p.ends_with(".public"))
        })
        .count();
    assert_eq!(new_members, 492);
    let fixtures = index
        .keys()
        .filter(|p| {
            (p.starts_with("candidate/") || p.starts_with("candidate-v2/"))
                && (p.ends_with(".proof") || p.ends_with(".public"))
        })
        .count();
    assert_eq!(fixtures, 4);
    let expected = index.len() - raw - expanded + 5;
    fs::write(
        dst.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":expected,"new_proof_public_members":new_members,"source_fixture_file_copies":fixtures,"distinct_source_fixture_members":2,"source_versions":2,"raw_local_only":raw,"expanded_local_only":expanded,"sha256":index,"scope":"Both source versions, original failed preflight, full native/preflight/latency/profile receipts, all246proofexports and metadata/analysis repairs. Nine raw captures and nine expanded streams local-only, hash-indexed. Executables/compiler caches represented by pinned identities and sources."})).unwrap()).unwrap();
    fs::write(
        report.join(".gitignore"),
        "/artifacts/**/perf.data\n/artifacts/**/symbols/stdout.log\n",
    )
    .unwrap();
    fs::write(report.join(".gitattributes"), "artifacts/** -whitespace\n").unwrap();
    println!("Archive{}hashfiles expectedGit{expected},492newmembers4sourcefixturecopies(2distinct),9raw9expandedlocalonly",index.len());
}

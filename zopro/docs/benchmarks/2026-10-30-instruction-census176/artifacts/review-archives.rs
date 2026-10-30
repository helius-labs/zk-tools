use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path, out: &mut BTreeSet<String>, base: &Path) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let t = e.file_type().unwrap();
        assert!(!t.is_symlink());
        if t.is_dir() {
            walk(&e.path(), out, base)
        } else {
            assert!(t.is_file());
            out.insert(e.path().strip_prefix(base).unwrap().display().to_string());
        }
    }
}
fn main() {
    let r = Path::new("/home/ubuntu/src/zopro-perf-work/instruction-census176");
    let d = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-instruction-census176",
    );
    let check = r.join("archive-review");
    assert!(!check.exists());
    fs::create_dir(&check).unwrap();
    let expected = read(d.join("artifacts/archive-file-hashes.json"));
    let mut archived = BTreeSet::new();
    for name in ["extraction", "analysis", "analysis-workflow"] {
        let archive = d.join(format!("artifacts/{name}.tar.gz"));
        let listing = Command::new("tar")
            .args(["-tzf"])
            .arg(&archive)
            .output()
            .unwrap();
        assert!(listing.status.success());
        for line in String::from_utf8(listing.stdout).unwrap().lines() {
            let p = Path::new(line);
            assert!(!p.is_absolute());
            assert!(!p.components().any(|c| matches!(
                c,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )));
            assert!(p.starts_with(name));
        }
        let s = Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&check)
            .status()
            .unwrap();
        assert!(s.success());
    }
    walk(&check, &mut archived, &check);
    let declared = expected
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(archived, declared);
    for (f, h) in expected.as_object().unwrap() {
        assert_eq!(hash(check.join(f)), h.as_str().unwrap());
        assert_eq!(hash(r.join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        read(d.join("artifacts/findings.json")),
        read(r.join("findings.json"))
    );
    assert_eq!(
        read(d.join("artifacts/independent-review.json"))["passed"],
        true
    );
    let findings = read(r.join("findings.json"));
    assert_eq!(findings["samples"], 42495);
    assert_eq!(findings["proof_ledger"], 27353);
    let m=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    for (rel, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel)),
            h.as_str().unwrap()
        );
    }
    let ledger = read(
        "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default-compat175/proof-sha256-after.json",
    );
    assert_eq!(ledger.as_array().unwrap().len(), 27353);
    assert_eq!(findings["all_current_local_rows"], m["ms"]);
    assert_eq!(findings["all_eight_M5_comparisons"], m["m5"]);
    let md = fs::read_to_string(d.join("README.md")).unwrap();
    let mut rest = md.as_str();
    let mut links = 0;
    while let Some(i) = rest.find("](") {
        rest = &rest[i + 2..];
        let j = rest.find(')').unwrap();
        let link = rest[..j].split('#').next().unwrap();
        if !link.starts_with("http") {
            assert!(d.join(link).exists(), "{link}");
            links += 1;
        }
        rest = &rest[j + 1..];
    }
    let result = json!({"passed":true,"all_three_archives_exact_member_sets_and_hashes":true,"archive_regular_files":archived.len(),"source176_unchanged":true,"accepted_local_rows_and_all_eight_CPU_Metal_comparisons_exact":true,"proof_ledger_unchanged":27353,"new_proofs_timings_profiles_tests":0,"readme_local_links_checked":links,"goal_achieved":false});
    fs::write(
        d.join("artifacts/archive-review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Archive/source/accepted-row/ledger/link audit PASS: {} regular archived files, source176 unchanged, ledger27353",archived.len());
}

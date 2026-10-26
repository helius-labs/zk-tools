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
            if name != "candidate-sdk" && name != "off-sdk" && name != "on-sdk" {
                file(&p, &dst.join(name), root, index);
            }
        }
    }
}
fn main(){
let root=Path::new("/tmp/zopro-g2-owned-finish");let report=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-g2-owned-finish");let dst=report.join("artifacts");assert!(!dst.exists());let mut index=BTreeMap::new();
for name in ["experiment.json","design.md","prepare.json","implement.rs","g2_owned_finish.rs.txt","tests.rs.txt","source-audit.json","accepted-manifest.json","prepare-build.rs","build-input-pins.json","native-plan.json","native.log","prepare-protocol.rs","protocol.rs","layout.rs","codegen.rs","codegen.json","codegen.txt","accepted-symbols.txt","off-symbols.txt","on-symbols.txt","linked-scheduler-libraries.json","all-nine-template.json","prior-proof-audit.json","inventory-preflight.sh","validate-preflight.jq","input-pins.json","preflight-plan.json","preflight.log","preflight-proof-inventory.json","perf-template.json","prepare-perf.rs","perf-plan.json","perf.log","analyze.rs","pilot-result.json","prepare-closeout.rs","closeout.rs","closeout.json","profile-analysis.json","proof-inventory.json","work-state.json","write-report.rs","prepare-publish.rs","publish.rs"]{file(&root.join(name),&dst.join(name),&dst,&mut index);}
for name in ["candidate","native","preflight","preflight-proofs","perf"]{tree(&root.join(name),&dst.join(name),&dst,&mut index);}
let raw=index.keys().filter(|p|p.ends_with("/perf.data")).count();let expanded=index.keys().filter(|p|p.starts_with("perf/profile/")&&p.ends_with("/symbols/stdout.log")).count();assert_eq!(raw,9);assert_eq!(expanded,9);let members=index.keys().filter(|p|p.ends_with(".proof")||p.ends_with(".public")).count();let fixtures=index.keys().filter(|p|p.starts_with("candidate/")&&(p.ends_with(".proof")||p.ends_with(".public"))).count();assert_eq!(fixtures,2);assert_eq!(members,476);let expected=index.len()-raw-expanded+5;
fs::write(dst.join("SHA256SUMS"),index.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":expected,"raw_local_only":raw,"expanded_local_only":expanded,"new_proof_public_members":474,"additional_source_fixture_members":fixtures,"sha256":index,"scope":"Complete isolated owned-batch dataflow source, protocol, all OFF/ON tests and SDK links, preflight and fixed wholeSDK latency/profiles. No instruction-census/primitive benchmark stage. Executables/caches represented by pinned identities; raw+expanded9profiles local-only. Not an adoption."})).unwrap()).unwrap();fs::write(report.join(".gitignore"),"/artifacts/perf/profile/**/perf.data\n/artifacts/perf/profile/**/symbols/stdout.log\n").unwrap();fs::write(report.join(".gitattributes"),"artifacts/** -whitespace\n").unwrap();println!("Archived{}hashfiles expectedGit{expected},9raw9expandedlocalonly474newproof/publicmembers2fixtures",index.len());}

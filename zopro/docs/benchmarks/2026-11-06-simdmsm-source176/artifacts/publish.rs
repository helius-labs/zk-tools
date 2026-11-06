use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/simdmsm-source176";
const P: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-simdmsm-source176";
fn h(b: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(b.as_ref()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else {
            assert!(e.file_type().unwrap().is_file());
            out.push(e.path());
        }
    }
}
fn main() {
    assert!(!Path::new(P).exists());
    let analysis = read(format!("{R}/analysis.json"));
    assert_eq!(analysis["pins_verified"], 196);
    assert_eq!(analysis["accepted_production_files_verified"], 176);
    assert_eq!(analysis["new_runtime_candidate"], false);
    for (p, sha) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(h(fs::read(p).unwrap()), sha.as_str().unwrap());
    }
    let step = Path::new(R).join("workflow/source-formula-audit");
    let exit = read(step.join("exit.json"));
    assert_eq!(exit["success"], true);
    assert_eq!(exit["returncode"], 0);
    for stream in ["stdout", "stderr"] {
        assert_eq!(
            h(fs::read(step.join(format!("{stream}.log"))).unwrap()),
            exit[format!("{stream}_sha256")].as_str().unwrap()
        );
    }
    let command = read(step.join("command.json"));
    let plan = read(format!("{R}/plan.json"));
    assert_eq!(
        &command["argv"].as_array().unwrap()[4..],
        plan["steps"][0]["argv"].as_array().unwrap()
    );
    assert_eq!(
        read(format!("{R}/workflow/result.json"))["status"],
        "complete"
    );
    let ledger = read(
        "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176/proof-sha256-after.json",
    );
    assert_eq!(ledger.as_array().unwrap().len(), 30846);
    let a = Path::new(P).join("artifacts");
    fs::create_dir_all(&a).unwrap();
    for f in [
        "audit.rs",
        "publish.rs",
        "analysis.json",
        "design.md",
        "reading.md",
        "pins.json",
        "plan.json",
        "source-pins.sha256",
    ] {
        let b = fs::read(format!("{R}/{f}")).unwrap();
        fs::write(a.join(f), &b).unwrap();
        assert_eq!(fs::read(a.join(f)).unwrap(), b);
    }
    let mut paths = vec![];
    walk(&Path::new(R).join("workflow"), &mut paths);
    paths.sort();
    let mut receipts = serde_json::Map::new();
    for p in paths {
        let b = fs::read(&p).unwrap();
        let utf8 = String::from_utf8(b.clone()).unwrap();
        assert_eq!(utf8.as_bytes(), b);
        receipts.insert(
            p.strip_prefix(R).unwrap().to_string_lossy().to_string(),
            json!({"sha256":h(&b),"bytes":b.len(),"utf8":utf8}),
        );
    }
    fs::write(
        a.join("receipts.json"),
        serde_json::to_vec_pretty(&receipts).unwrap(),
    )
    .unwrap();
    let mut source = serde_json::Map::new();
    for f in ["pair_ifma.c", "pip_ifma.c", "padd.c", "types.h"] {
        let b = fs::read(format!("{R}/{f}")).unwrap();
        let dir = if f == "types.h" { "include" } else { "src" };
        source.insert(f.into(),json!({"sha256":h(&b),"bytes":b.len(),"url":format!("https://github.com/JR-account/SimdMSM/blob/5e28e01f84f3c56e63952fd2099dabddc5b55809/AVX-MSM/demo/381/{dir}/{f}")}));
    }
    let publication = json!({"scope":"read-only structural source audit; no separate semantic reviewer or native/performance qualification","pins_verified":196,"workflow_commands":1,"receipt_members":receipts.len(),"receipt_byte_roundtrips_verified":true,"source_identities":source,"audit_binary_sha256":h(fs::read(format!("{R}/audit")).unwrap()),"xtask_sha256":h(fs::read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask").unwrap()),"external_source_and_executables_included":false,"accepted_production_changed":false,"new_proofs":0,"ledger":30846,"goal_status":"active/unmet0/8"});
    fs::write(
        a.join("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
    )
    .unwrap();
    fs::copy(
        format!("{R}/report-draft.md"),
        Path::new(P).join("README.md"),
    )
    .unwrap();
    let mut all = vec![];
    walk(Path::new(P), &mut all);
    all.sort();
    let mut sums = String::new();
    for p in &all {
        sums.push_str(&format!(
            "{}  {}\n",
            h(fs::read(p).unwrap()),
            p.strip_prefix(P).unwrap().display()
        ));
    }
    fs::write(Path::new(P).join("SHA256SUMS"), sums).unwrap();
    println!("Published {} files/{} exact UTF8 receipts;196pins/176originals/ledger30846 verified; no new performance",all.len()+1,receipts.len());
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text =
        std::str::from_utf8(&b).unwrap_or_else(|_| panic!("expected text member: {}", p.display()));
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
}
fn archive(p: &Path, v: &Value) -> (usize, String) {
    let b = serde_json::to_vec(v).unwrap();
    for m in v.as_object().unwrap().values() {
        let text = m["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(text.len(), m["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(text), m["sha256"]);
    }
    let mut c = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(p).unwrap())
        .spawn()
        .unwrap();
    c.stdin.take().unwrap().write_all(&b).unwrap();
    assert!(c.wait().unwrap().success());
    let o = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(p)
        .output()
        .unwrap();
    assert!(o.status.success());
    assert_eq!(o.stdout, b);
    (v.as_object().unwrap().len(), hash(&b))
}
fn main() {
    assert!(!Path::new(D).exists());
    let review = read(format!("{R}/review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(review["scalar_rows_checked"], 61451);
    assert_eq!(review["new_proofs"], 0);
    for (p, h) in read(format!("{R}/repair-pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report = fs::read_to_string(format!("{R}/report.md")).unwrap();
    let old = fs::read_to_string(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-word-export176/README.md",
    )
    .unwrap();
    let section = &old[old
        .find("## Accepted CPU and Metal comparisons, unchanged")
        .unwrap()..];
    let section = &section[..section.find("\n[Analysis]").unwrap()];
    report.push_str("\n");
    report.push_str(section);
    report.push_str("\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [design](artifacts/design.md), [question and history](artifacts/question.md), [source audit](artifacts/source-audit.json), [fixed diagnostic audit](artifacts/source-fixed-audit.json), [initial workflow](artifacts/plan.json), [repair workflow](artifacts/repair-plan.json), [pins](artifacts/repair-pins.json), [repair scope](artifacts/repair.md), [six exact changed-source members](artifacts/source-deltas.json.gz), [exact helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Unchanged base files are bound by their accepted-source manifest hashes; the two full source snapshots remain local. No private input-cache/witness/key/image bytes, executable/library blobs or performance recordings are included.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "design.md",
        "question.md",
        "source-audit.json",
        "source-fixed-audit.json",
        "plan.json",
        "pins.json",
        "repair-plan.json",
        "repair-pins.json",
        "repair.md",
        "witness-info.json",
        "model-session.json",
        "model.rs",
        "model-fixed.rs",
        "diagnostic.rs",
        "prepare.rs",
        "repair.rs",
        "loader.rs",
        "digit.rs",
        "witness.rs",
        "cases.json",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut sources = BTreeMap::new();
    for dir in ["source", "source-fixed"] {
        for file in [
            "crates/zopro-core/src/lib.rs",
            "crates/zopro-core/src/msm_affine.rs",
            "crates/zopro-core/src/tagged_phi_diagnostic.rs",
        ] {
            let name = format!("{dir}/{file}");
            sources.insert(name.clone(), member(&Path::new(R).join(name)));
        }
    }
    assert_eq!(sources.len(), 6);
    let (scount, ssha) = archive(&art.join("source-deltas.json.gz"), &json!(sources));
    let mut receipts = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            receipts.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    for dir in ["runs", "runs-fixed"] {
        for p in walk(&Path::new(R).join(dir)) {
            assert!(p
                .extension()
                .is_some_and(|e| ["json", "log"].iter().any(|s| e == *s)));
            receipts.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    let (rcount, rsha) = archive(&art.join("helpers-and-receipts.json.gz"), &json!(receipts));
    let publication = json!({"changed_source_members":scount,"exact_UTF8_helper_receipt_members":rcount,"source_archive_uncompressed_sha256":ssha,"receipt_archive_uncompressed_sha256":rsha,"all_member_hashes_and_roundtrips_verified":true,"review_passed":true,"workflow_commands":8,"successful_commands":7,"retained_failed_command":1,"source_copies_recovered":352,"new_solver_FFT_calls":2,"new_witness_preparations":1,"scalar_rows_checked":61451,"private_cache_witness_key_image_bytes_included":false,"runtime_arithmetic_candidate":false,"new_latency_or_profile_measurements":0,"new_proofs":0,"goal_status":"active/unmet0/8"});
    fs::write(
        art.join("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
    )
    .unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(&fs::read(&p).unwrap()),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/6source deltas/{rcount}exactUTF8members; allroundtrips/checksumsPASS; no private inputs/binaries/measurements",walk(Path::new(D)).len());
}

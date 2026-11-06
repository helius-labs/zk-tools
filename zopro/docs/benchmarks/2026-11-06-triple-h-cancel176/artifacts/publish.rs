use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-cancel176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-triple-h-cancel176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walk(&p))
        } else {
            out.push(p)
        }
    }
    out.sort();
    out
}
fn main() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    let review = read(format!("{R}/review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(
        review["symbolic_polynomial_certificate"]["both_polynomial_differences_exactly_zero"],
        true
    );
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report = fs::read_to_string(format!("{R}/report.md")).unwrap();
    report.push_str("\n| Curve | Triples | New H-only sites | Old dH sites | Two binary sites |\n|---|---:|---:|---:|---:|\n");
    for f in a["fields"].as_array().unwrap() {
        for row in f["batches"].as_array().unwrap() {
            report.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                f["curve"].as_str().unwrap(),
                row["triples"],
                row["new_H_only"]["native_vector_IFMA_template_sites"],
                row["old_dH"]["native_vector_IFMA_template_sites"],
                row["two_binary"]["native_vector_IFMA_template_sites"]
            ));
        }
    }
    report.push_str("\n## Next stage and limits\n\nA native **complete unweighted G1 reduction** screen is justified. The proposed two-vector prefix/H record must be implemented and tested for exceptions, tails, scratch reuse, resource bounds and input lifetime. Reverse results must go to their original disjoint input slots; compaction follows all reverse stores. This is essential if original coordinates are reread. It is not yet a qualified memory saving. Weighted binary reductions retain their original identity and cannot simply become ternary. See the [native continuation](artifacts/next-native.md).\n\nNo native IFMA kernel, timings, CPU profiles, process peak/minimum RAM, pressure/swap, phone/ARM/TEE, concurrency, huge-circuit, AWS/Aeglos or new proof result is added. The model establishes no new2×path or all-eight10×achievement. Passing algebra does not permit production adoption.\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("## Accepted CPU and Metal comparisons, unchanged")
        .unwrap()..];
    let section = &section[..section.find("\n[Analysis]").unwrap()];
    report.push_str(section);
    report.push_str("\n\n[Analysis](artifacts/analysis.json), [independent symbolic/receipt/work review](artifacts/review.json), [derivation](artifacts/derivation.md), [design](artifacts/design.md), [new formula](artifacts/cancel.rs), [unchanged original models](artifacts/original.rs), [curve study](artifacts/study.rs), [native next stage](artifacts/next-native.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [exact source/receipts archive](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). No external paper text, private inputs, keys/images/witnesses/proofs or executable/library blobs are included.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "derivation.md",
        "design.md",
        "cancel.rs",
        "original.rs",
        "study.rs",
        "prepare.rs",
        "next-native.md",
        "pins.json",
        "plan.json",
        "session.json",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            let b = fs::read(&p).unwrap();
            let text = std::str::from_utf8(&b).unwrap();
            members.insert(
                p.strip_prefix(R).unwrap().display().to_string(),
                json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text}),
            );
        }
    }
    for p in walk(&Path::new(R).join("runs")) {
        let b = fs::read(&p).unwrap();
        let text = std::str::from_utf8(&b).unwrap();
        members.insert(
            p.strip_prefix(R).unwrap().display().to_string(),
            json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text}),
        );
    }
    let count = members.len();
    let bytes = serde_json::to_vec(&members).unwrap();
    let mut child = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(art.join("source-and-receipts.json.gz")).unwrap())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(child.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(art.join("source-and-receipts.json.gz"))
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), v["sha256"]);
    }
    let publication = json!({"exact_UTF8_members":count,"all_member_hashes_and_archive_roundtrip_verified":true,"review_passed":true,"symbolic_identities_exact":true,"workflow_commands":4,"all_first_success":true,"new_focused_tests":2,"old_tests_filtered":3,"toy_attempts":5832,"BN254_attempts":11264,"common_domain_batch_triples":496,"native_IFMA_implementation_or_timing":false,"new_proofs":0,"goal_status":"active/unmet0/8"});
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
    println!("Published{}files/{count}exactUTF8members; symbolic/scalar screen verified; no native performance or proofs",walk(Path::new(D)).len());
}

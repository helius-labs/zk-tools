use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/word-export176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-word-export176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn gzip(p: &Path, b: &[u8]) {
    let mut c = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(p).unwrap())
        .spawn()
        .unwrap();
    c.stdin.take().unwrap().write_all(b).unwrap();
    assert!(c.wait().unwrap().success());
    let output = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(p)
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b);
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
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = String::from_utf8(b.clone()).unwrap();
    let v = json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text});
    assert_eq!(v["utf8"].as_str().unwrap().as_bytes(), b);
    v
}
fn source_member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let hex = b.iter().map(|v| format!("{v:02x}")).collect::<String>();
    let decoded = hex
        .as_bytes()
        .chunks_exact(2)
        .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(decoded, b);
    json!({"bytes":b.len(),"sha256":hash(&b),"hex":hex})
}
fn main() {
    assert!(Path::new(D).join("README.md").exists());
    assert!(!Path::new(D)
        .join("artifacts/candidate-source.json.gz")
        .exists());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(read(format!("{R}/review.json"))["review_passed"], true);
    let art = Path::new(D).join("artifacts");
    fs::copy(format!("{R}/publish.rs"), art.join("publish.rs")).unwrap();
    fs::copy(
        format!("{R}/publish-repair.rs"),
        art.join("publish-repair.rs"),
    )
    .unwrap();
    fs::copy(
        format!("{R}/publication-repair.md"),
        art.join("publication-repair.md"),
    )
    .unwrap();
    let p = Path::new(D).join("README.md");
    let mut report = fs::read_to_string(&p).unwrap();
    report.push_str("\nThe candidate source archive uses lossless hex for all178files, preserving checked-in binary test vectors. The initial publisher treated source as UTF-8 and stopped at a binary test-vector file; its source/log remain archived. Publication resumed only at source/receipt/profile archives and checksums, reusing successful report/artifact/analysis writes. No native test, build, measurement or review was repeated. See [publication repair](artifacts/publication-repair.md).\n");
    fs::write(p, report).unwrap();
    let mut sources = BTreeMap::new();
    for p in walk(&Path::new(R).join("caller-source")) {
        let name = p
            .strip_prefix(Path::new(R).join("caller-source"))
            .unwrap()
            .display()
            .to_string();
        assert!(pins_source_match(&p));
        sources.insert(name, source_member(&p));
    }
    assert_eq!(sources.len(), 178);
    gzip(
        &art.join("candidate-source.json.gz"),
        &serde_json::to_vec(&sources).unwrap(),
    );
    let mut receipts = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log", "txt"].iter().any(|s| e == *s))
        {
            receipts.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    for dir in [
        "native",
        "codegen",
        "caller-native",
        "caller-native-repair",
        "runs",
        "stat",
        "initial-caller-preparation",
    ] {
        for p in walk(&Path::new(R).join(dir)) {
            if p.extension().is_some_and(|e| {
                ["rs", "md", "json", "log", "txt", "csv"]
                    .iter()
                    .any(|s| e == *s)
            }) {
                receipts.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
            }
        }
    }
    let count = receipts.len();
    let encoded = serde_json::to_vec(&receipts).unwrap();
    gzip(&art.join("helpers-and-receipts.json.gz"), &encoded);
    let decoded: Value = serde_json::from_slice(&encoded).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(hash(b), v["sha256"]);
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
    }
    for (name, p) in a["profiles"].as_object().unwrap() {
        let b = fs::read(format!("{R}/perf/{name}.data")).unwrap();
        assert_eq!(hash(&b), p["raw_sha256"]);
        gzip(&art.join(format!("{name}.data.gz")), &b);
    }
    let publication = json!({"source_members":178,"exact_UTF8_helper_receipt_members":count,"lossless_raw_profiles":12,"all_member_hashes_and_compression_roundtrips_verified":true,"main_review_passed":true,"counter_review_passed":true,"successful_workflow_commands":675,"caller_processes":600,"profile_samples":93849,"gate_passed":false,"adopted":false,"new_proofs":0,"goal_status":"active/unmet0/8"});
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
    println!("Published{}files/178source/{count}exactUTF8members/12losslessprofiles;checksums and all roundtrips verified",walk(Path::new(D)).len());
}
fn pins_source_match(p: &Path) -> bool {
    let pins = read(format!("{R}/pins.json"));
    pins[p.display().to_string()] == hash(&fs::read(p).unwrap())
}

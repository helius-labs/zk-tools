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
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    let review = read(format!("{R}/review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(review["successful_workflow_commands"], 675);
    assert_eq!(a["gate_passed"], false);
    assert_eq!(
        read(format!("{R}/counter-review.json"))["review_passed"],
        true
    );
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report = fs::read_to_string(format!("{R}/report-head.md")).unwrap();
    report.push_str("\n| Context | Frozen µs | OFF µs | ON µs | ON/OFF [interval] | Wins vs OFF | ON/frozen [interval] | Control calibration |\n|---|---:|---:|---:|---|---:|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let x = &row["pairs"]["ON/OFF"];
        let y = &row["pairs"]["ON/frozen"];
        let c = &row["pairs"]["OFF/frozen"];
        let pass = c["low"].as_f64().unwrap() >= 1. / 1.03 && c["high"].as_f64().unwrap() <= 1.03;
        report.push_str(&format!("| {} | {:.3} | {:.3} | {:.3} | {:.6} [{:.6}, {:.6}] | {}/15 | {:.6} [{:.6}, {:.6}] | {} |\n",row["context"].as_str().unwrap(),row["median_ns_per_wave"]["frozen"].as_f64().unwrap()/1000.,row["median_ns_per_wave"]["OFF"].as_f64().unwrap()/1000.,row["median_ns_per_wave"]["ON"].as_f64().unwrap()/1000.,x["median"].as_f64().unwrap(),x["low"].as_f64().unwrap(),x["high"].as_f64().unwrap(),x["wins"],y["median"].as_f64().unwrap(),y["low"].as_f64().unwrap(),y["high"].as_f64().unwrap(),if pass{"pass"}else{"fail"}));
    }
    report.push_str("\nFailed checks:\n\n```text\n");
    for f in a["failed_checks"].as_array().unwrap() {
        report.push_str(f.as_str().unwrap());
        report.push('\n');
    }
    report.push_str("```\n\n");
    let previous=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-primary-instruction-census176/README.md").unwrap();
    let section = &previous[previous
        .find("## Accepted CPU and Metal comparisons, unchanged")
        .unwrap()..];
    let section = &section[..section.find("\n[Full analysis]").unwrap()];
    report.push_str(section);
    report.push_str("\n\n[Analysis](artifacts/analysis.json.gz), [main review](artifacts/review.json), [counter review](artifacts/counter-review.json), [caller design](artifacts/caller-design.md), [source audit](artifacts/caller-source-audit.json), [library and caller identities](artifacts/caller-identities.json), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [exact candidate source archive](artifacts/candidate-source.json.gz), [exact helpers/receipts archive](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Twelve lossless raw cycle profiles are included under artifacts. Executables, libraries, private keys/images/witnesses/proofs and external copyrighted source are excluded.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "design.md",
        "caller-design.md",
        "word_export.rs",
        "review.rs",
        "review.json",
        "counter-review.rs",
        "counter-review.json",
        "caller-source-audit.json",
        "source-audit.json",
        "caller-reference.json",
        "caller-identities.json",
        "field-info.json",
        "codegen-audit.json",
        "plan.json",
        "pins.json",
        "caller-native-repair-plan.json",
        "caller-native-repair-pins.json",
        "native-repair.md",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    for f in ["analysis.json", "profile-symbols.json"] {
        gzip(
            &art.join(format!("{f}.gz")),
            &fs::read(format!("{R}/{f}")).unwrap(),
        );
    }
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

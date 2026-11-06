use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/primary-instruction-census176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176";
const P: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-primary-instruction-census176";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn h(b: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(b.as_ref()))
}
fn hash(p: impl AsRef<Path>) -> String {
    h(fs::read(p).unwrap())
}
fn walk(p: &Path, v: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        assert!(!e.file_type().unwrap().is_symlink());
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), v)
        } else {
            v.push(e.path());
        }
    }
}
fn gz(src: &Path, dst: &Path) {
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(src)
        .stdout(Stdio::from(fs::File::create(dst).unwrap()))
        .status()
        .unwrap()
        .success());
    let back = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(dst)
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(back.stdout, fs::read(src).unwrap());
}
fn main() {
    assert!(!Path::new(P).exists());
    let review = read(r("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(review["exact_selected_samples"], 16420);
    assert_eq!(review["instruction_sites"], 37973);
    for (p, sha) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), sha.as_str().unwrap());
    }
    let mut report = fs::read_to_string(r("report.md")).unwrap();
    let previous = fs::read_to_string(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g2-aos-gather176/README.md",
    )
    .unwrap();
    let start = previous.find("|Shape|Zopro ms|").unwrap();
    let end = start + previous[start..].find("\n\n").unwrap();
    report.push_str(&previous[start..end]);
    report.push_str("\n\nMerge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.\n\n[Full analysis](artifacts/analysis.json.gz), [independent review](artifacts/review.json), [census helper](artifacts/census.rs), [reviewer](artifacts/review.rs), [design](artifacts/design.md), [pins](artifacts/pins.json), [workflow](artifacts/plan.json), [exact source/command/disassembly/decoded-profile archive](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Raw profiles, executables, libraries and private proof/key/witness inputs are excluded.\n");
    let artifacts = Path::new(P).join("artifacts");
    fs::create_dir_all(&artifacts).unwrap();
    fs::write(Path::new(P).join("README.md"), report).unwrap();
    for f in [
        "census.rs",
        "review.rs",
        "review.json",
        "design.md",
        "pins.json",
        "plan.json",
        "publish.rs",
    ] {
        let b = fs::read(r(f)).unwrap();
        fs::write(artifacts.join(f), &b).unwrap();
        assert_eq!(fs::read(artifacts.join(f)).unwrap(), b);
    }
    gz(&r("analysis.json"), &artifacts.join("analysis.json.gz"));
    let mut files = vec![];
    for e in fs::read_dir(R).unwrap() {
        let e = e.unwrap();
        if !e.file_type().unwrap().is_file() {
            continue;
        }
        let name = e.file_name().to_string_lossy().to_string();
        if name.starts_with("checkpoint") || name.contains("session") {
            continue;
        }
        let ext = e
            .path()
            .extension()
            .map(|s| s.to_string_lossy().to_string());
        if matches!(
            ext.as_deref(),
            Some("rs" | "json" | "md" | "log" | "stdout" | "stderr")
        ) {
            files.push(e.path());
        }
    }
    walk(&r("workflow"), &mut files);
    files.sort();
    let mut archive = BTreeMap::new();
    for p in files {
        let b = fs::read(&p).unwrap();
        let s = String::from_utf8(b.clone()).unwrap();
        assert_eq!(s.as_bytes(), b);
        archive.insert(
            p.strip_prefix(R).unwrap().to_string_lossy().to_string(),
            json!({"bytes":b.len(),"sha256":h(&b),"utf8":s}),
        );
    }
    for w in [16, 32] {
        let name = format!("cycles-1x2-w{w}-accepted");
        for kind in ["decode", "metadata", "events", "loss"] {
            for file in ["stdout.log", "stderr.log", "command.json", "exit.json"] {
                let p = format!("{Q}/runs/{kind}-{name}/{file}");
                let b = fs::read(&p).unwrap();
                let s = String::from_utf8(b.clone()).unwrap();
                assert_eq!(s.as_bytes(), b);
                archive.insert(
                    format!("inherited/runs/{kind}-{name}/{file}"),
                    json!({"bytes":b.len(),"sha256":h(&b),"utf8":s}),
                );
            }
        }
    }
    let bytes = serde_json::to_vec(&archive).unwrap();
    for v in archive.values() {
        let s = v["utf8"].as_str().unwrap();
        assert_eq!(s.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(h(s), v["sha256"]);
    }
    let tmp = r("publication.tmp");
    fs::write(&tmp, &bytes).unwrap();
    gz(&tmp, &artifacts.join("source-and-receipts.json.gz"));
    fs::remove_file(tmp).unwrap();
    let info = json!({"UTF8_members":archive.len(),"all_member_hashes_and_roundtrips_verified":true,"archive_uncompressed_sha256":h(&bytes),"analysis_uncompressed_sha256":hash(r("analysis.json")),"existing_raw_profiles_redecoded_or_recorded":false,"raw_profile_bytes_included":false,"new_runtime_candidate":false,"new_native_tests":0,"new_proofs":0,"goal_status":"active/unmet0/8"});
    fs::write(
        artifacts.join("publication.json"),
        serde_json::to_vec_pretty(&info).unwrap(),
    )
    .unwrap();
    let mut all = vec![];
    walk(Path::new(P), &mut all);
    all.sort();
    let mut sums = String::new();
    for p in &all {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(p),
            p.strip_prefix(P).unwrap().display()
        ));
    }
    fs::write(Path::new(P).join("SHA256SUMS"), sums).unwrap();
    println!(
        "Published{}files/{}exactUTF8members; all hashes/roundtripsPASS; no new performance",
        all.len() + 1,
        archive.len()
    );
}

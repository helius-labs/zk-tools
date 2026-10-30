use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-balanced175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new(R);
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let manifest = read(Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    let original = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(original.len(), 175);
    let audit = read(root.join("source-audit.json"));
    for tree in ["source"] {
        let mut exact = 0;
        for (p, h) in original {
            let accepted = fs::read(Path::new(Z).join(p)).unwrap();
            assert_eq!(hash(&accepted), h.as_str().unwrap());
            let mut actual = fs::read(root.join(tree).join(p)).unwrap();
            if actual == accepted {
                exact += 1;
            } else {
                let edit = &audit["edits"][p];
                assert!(edit.is_object());
                let mut text = String::from_utf8(actual).unwrap();
                if let Some(suffix) = edit["suffix"].as_str() {
                    assert!(text.ends_with(suffix));
                    text.truncate(text.len() - suffix.len());
                }
                if let Some(after) = edit["after"].as_str() {
                    assert_eq!(text.matches(after).count(), 1);
                    text = text.replacen(after, edit["before"].as_str().unwrap(), 1);
                }
                actual = text.into_bytes();
                assert_eq!(actual, accepted, "reverse{tree}/{p}");
            }
        }
        assert_eq!(exact, 172);
    }
    let old_root=Path::new("/home/ubuntu/src/zopro-perf-work/g2-queue-scan175");
    let old_audit=read(old_root.join("source-audit.json"));
    for(p,h)in original{
        let bytes=fs::read(old_root.join("source-frozen").join(p)).unwrap();
        if hash(&bytes)!=h.as_str().unwrap(){
            let suffix=old_audit["edits"][p]["suffix"].as_str().unwrap();
            let text=String::from_utf8(bytes).unwrap();
            assert_eq!(hash(text.strip_suffix(suffix).unwrap().as_bytes()),h.as_str().unwrap());
        }
    }
    let a = read(root.join("analysis.json"));
    assert_eq!(a["failed_checks"], json!([]));
    assert_eq!(a["caller_gate"], true);
    assert_eq!(a["diagnostic_qualified"], true);
    assert_eq!(a["eligible_for_SDK_stage"], true);
    let pairs = a["pairs"].as_object().unwrap();
    for (k, v) in pairs {
        let s = &v["statistics"];
        let mut ratios = s["ratios"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(ratios.len(), 31);
        let wins = ratios.iter().filter(|x| **x < 1.).count();
        ratios.sort_by(f64::total_cmp);
        assert!((ratios[15] - s["median"].as_f64().unwrap()).abs() < 1e-14);
        assert!((ratios[9] - s["low"].as_f64().unwrap()).abs() < 1e-14);
        assert!((ratios[21] - s["high"].as_f64().unwrap()).abs() < 1e-14);
        let pass = if k.starts_with("1x2-w32/") {
            ratios[21] < 1. && wins >= 21
        } else {
            ratios[21] <= 1.02
        };
        assert_eq!(v["passed"], pass);
    }
    let jobs = read(root.join("jobs.json"));
    assert_eq!(jobs.as_array().unwrap().len(), 396);
    assert_eq!(
        jobs.as_array()
            .unwrap()
            .iter()
            .map(|j| j["calls"].as_u64().unwrap() + 3)
            .sum::<u64>(),
        9216
    );
    let doc = Path::new(Z).join("docs/benchmarks/2026-10-30-g2-balanced175");
    let arc = read(doc.join("artifacts/archive-review.json"));
    let mut members = 0;
    for (kind, name) in [
        ("source", "source"),
        ("receipts", "receipts"),
        ("observations", "observations-and-plan"),
        ("inherited_summary", "inherited-summary"),
    ] {
        let p = doc.join("artifacts").join(format!("{name}.json.gz"));
        assert_eq!(hash(&fs::read(&p).unwrap()), arc[kind]["sha256"]);
        let output = Command::new("gzip").args(["-dc"]).arg(p).output().unwrap();
        assert!(output.status.success());
        let map: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(map.as_object().unwrap().len() as u64, arc[kind]["members"]);
        for (_, entry) in map.as_object().unwrap() {
            let b = if let Some(s) = entry["utf8"].as_str() {
                s.as_bytes().to_vec()
            } else {
                let h = entry["hex"].as_str().unwrap();
                (0..h.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
                    .collect()
            };
            assert_eq!(hash(&b), entry["sha256"]);
            assert_eq!(b.len() as u64, entry["bytes"]);
            members += 1;
        }
    }
    let out = json!({"capture_pins_verified":pins.as_object().unwrap().len(),"production_files_verified":175,"OFF_ON_source_reverses_all175":true,"baseline_source_reverses_all175":true,"all_individual_pair_gate_statistics_verified":true,"archived_members_verified":members,"no_new_capture_test_or_oracle":true,"goal_achieved":false});
    fs::write(
        root.join("review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("{out}");
}

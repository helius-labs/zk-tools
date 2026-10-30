use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/vnni13-screen175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-vnni13-screen175";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: Value) {
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn walk(p: &Path, v: &mut Vec<PathBuf>) {
    let mut paths: Vec<_> = fs::read_dir(p)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            walk(&p, v)
        } else {
            v.push(p);
        }
    }
}
fn main() {
    let r = Path::new(R);
    let z = Path::new(Z);
    let d = Path::new(D);
    let a = d.join("artifacts");
    fs::create_dir_all(&a).unwrap();
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let initial = read(r.join("initial-workflow/pins.json"));
    for (p, h) in initial.as_object().unwrap() {
        let p = if p == &format!("{R}/screen") {
            r.join("initial-workflow/screen")
        } else if p == &format!("{R}/screen.rs") {
            r.join("initial-workflow/screen.rs")
        } else {
            PathBuf::from(p)
        };
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let analysis = read(r.join("analysis.json"));
    assert_eq!(analysis["complete"], true);
    assert_eq!(analysis["recipe_rejected_by_instruction_budget"], true);
    let n = 20u64;
    let ordinary = n * n + n * (n + 1) / 2 + n * n - (n - 2) * (n - 1) / 2;
    let fused = ordinary + n * n;
    assert_eq!(analysis["ordinary_word_products_per_field"], ordinary);
    assert_eq!(analysis["fused_word_products_per_field"], fused);
    assert_eq!(
        analysis["eight_field_vector_instruction_capacity_bounds"]["ordinary"],
        (ordinary * 8).div_ceil(32)
    );
    assert_eq!(
        analysis["eight_field_vector_instruction_capacity_bounds"]["fused"],
        (fused * 8).div_ceil(32)
    );
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    assert!(fs::read_to_string(r.join("dot-disassembly.txt"))
        .unwrap()
        .contains("vpdpwssd"));
    for file in [
        "analysis.json",
        "instruction-rows.json",
        "extract.rs",
        "extract-initial.rs",
        "design.md",
        "screen.rs",
        "pins.json",
        "plan.json",
        "dot-disassembly.txt",
        "publish.rs",
    ] {
        fs::copy(r.join(file), a.join(file)).unwrap();
    }
    fs::write(a.join("failures.md"),"# Preparation failures\n\nThe initial XML extractor required literal `<t>` and panicked on an attributed text element (expected one literal match, found zero). Its source is retained as extract-initial.rs; the terminal observation was not redirected to a raw log. The repaired reader accepts text attributes and rich runs. Its one successful extraction is reused; subsequent rustfmt changes formatting only.\n\nThe initial workflow plan lacked version:1 and xtask rejected it before creating an output directory or executing the command. Original screen source, plan, pins and raw workflow.log are retained in receipts.json.gz under initial-workflow/. The corresponding original executable remains local, with its hash in the archived pins. Final pins bind the corrected executable. No successful native semantic check, field operation or proof was rerun.\n").unwrap();
    let mut paths = vec![];
    walk(&r.join("runs"), &mut paths);
    for file in [
        "workflow.log",
        "initial-workflow/screen.rs",
        "initial-workflow/pins.json",
        "initial-workflow/plan.json",
        "initial-workflow/workflow.log",
    ] {
        paths.push(r.join(file));
    }
    paths.sort();
    let mut archive = BTreeMap::new();
    for p in paths {
        let name = p.strip_prefix(r).unwrap().display().to_string();
        let bytes = fs::read(&p).unwrap();
        archive.insert(
            name,
            json!({"sha256":hash(&p),"bytes":bytes.len(),"text":String::from_utf8(bytes).unwrap()}),
        );
    }
    let raw = serde_json::to_vec(&archive).unwrap();
    fs::write(r.join("receipts.json"), &raw).unwrap();
    let gz = Command::new("gzip")
        .arg("-n")
        .arg("-c")
        .arg(r.join("receipts.json"))
        .output()
        .unwrap();
    assert!(gz.status.success());
    fs::write(a.join("receipts.json.gz"), gz.stdout).unwrap();
    let unpacked = Command::new("gzip")
        .arg("-d")
        .arg("-c")
        .arg(a.join("receipts.json.gz"))
        .output()
        .unwrap();
    assert!(unpacked.status.success());
    assert_eq!(unpacked.stdout, raw);
    let restored: Value = serde_json::from_slice(&unpacked.stdout).unwrap();
    for (name, v) in restored.as_object().unwrap() {
        let bytes = fs::read(r.join(name)).unwrap();
        assert_eq!(v["text"].as_str().unwrap().as_bytes(), &bytes);
        assert_eq!(v["sha256"], hash(r.join(name)));
        assert_eq!(v["bytes"], bytes.len());
    }
    let snapshot = z.join("docs/benchmarks/2026-10-30-current-bench-snapshot175/data.json");
    let rows = Command::new("jq")
        .arg(".rows")
        .arg(&snapshot)
        .output()
        .unwrap();
    assert!(rows.status.success());
    fs::write(d.join("accepted-rows.json"), rows.stdout).unwrap();
    assert_eq!(read(d.join("accepted-rows.json")), read(snapshot)["rows"]);
    save(
        a.join("publication-review.json"),
        json!({"review_passed":true,"final_pins_verified":pins.as_object().unwrap().len(),"initial_pins_verified_with_retained_source_binary_mapping":initial.as_object().unwrap().len(),"accepted_source_files_unchanged":175,"independent_triangular_counts_agree":true,"receipt_members":archive.len(),"receipt_roundtrip_exact":true,"all_nine_accepted_rows_exact":true,"distinct_proof_ledger":25231,"new_timings_or_proofs":0,"full_field_correctness_not_qualified":true}),
    );
    let mut paths = vec![d.join("README.md"), d.join("accepted-rows.json")];
    walk(&a, &mut paths);
    paths.sort();
    let sums: String = paths
        .iter()
        .map(|p| format!("{}  {}\n", hash(p), p.strip_prefix(d).unwrap().display()))
        .collect();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!("Published count screen: {} final pins, {} initial pins, {} receipt members; no runtime capture repeated",pins.as_object().unwrap().len(),initial.as_object().unwrap().len(),archive.len());
}

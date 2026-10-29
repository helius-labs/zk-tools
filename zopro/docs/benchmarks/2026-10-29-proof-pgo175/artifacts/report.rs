use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-proof-pgo175";
const M:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash(p: impl AsRef<Path>) -> String {
    digest(&fs::read(p).unwrap())
}
fn encoded(b: &[u8]) -> Value {
    match std::str::from_utf8(b) {
        Ok(s) => json!({"encoding":"utf8","data":s,"sha256":digest(b)}),
        Err(_) => {
            json!({"encoding":"hex","data":b.iter().map(|c|format!("{c:02x}")).collect::<String>(),"sha256":digest(b)})
        }
    }
}
fn walk(p: &Path, files: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), files)
        } else if e.file_type().unwrap().is_file() {
            files.push(e.path());
        }
    }
}
fn compressed(d: &Path, name: &str, value: &Value) {
    let bytes = serde_json::to_vec(value).unwrap();
    let raw = r(format!("{name}.json"));
    fs::write(&raw, &bytes).unwrap();
    let target = d.join(format!("{name}.json.gz"));
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&raw)
        .stdout(Stdio::from(fs::File::create(&target).unwrap()))
        .status()
        .unwrap()
        .success());
    let o = Command::new("gzip")
        .arg("-dc")
        .arg(target)
        .output()
        .unwrap();
    assert!(o.status.success());
    assert_eq!(o.stdout, bytes);
}
fn seal() {
    let d = Path::new(D);
    let mut paths = vec![];
    walk(d, &mut paths);
    let mut sums = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if rel != "artifact-index.json" && rel != "SHA256SUMS" {
            sums.insert(rel, hash(p));
        }
    }
    save(d.join("artifact-index.json"), &json!(sums));
    sums.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        sums.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    for (p, h) in &sums {
        assert_eq!(hash(d.join(p)), *h);
    }
    println!("Sealed {} hashes / {} files", sums.len(), sums.len() + 1);
}
fn archive() {
    let a = read(r("analysis.json"));
    assert_eq!(a["gate_passed"], true);
    assert_eq!(a["SDK_requests"], 7902);
    assert_eq!(a["independent_proof_count"], 18847);
    assert_eq!(a["new_independent_exports"], 78);
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read(M);
    assert_eq!(
        hash(manifest["native_binary"].as_str().unwrap()),
        manifest["native_binary_sha256"].as_str().unwrap()
    );
    let d = Path::new(D).join("artifacts");
    assert!(!d.exists());
    fs::create_dir_all(&d).unwrap();
    let audit = read(r("source-audit.json"));
    assert_eq!(audit["originals_exact"], 175);
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(r("source").join(p)), h.as_str().unwrap());
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    let mut paths = vec![];
    walk(&r("source"), &mut paths);
    let mut source = BTreeMap::new();
    for p in paths {
        source.insert(
            p.strip_prefix(r("source")).unwrap().display().to_string(),
            encoded(&fs::read(p).unwrap()),
        );
    }
    assert_eq!(source.len(), 175);
    compressed(&d, "source-snapshot", &json!(source));
    let mut receipts = vec![];
    let mut streams = 0;
    for (w, n) in [
        ("native", 9),
        ("training", 109),
        ("native-use", 6),
        ("runs", 165),
    ] {
        let result = read(r(format!("{w}/result.json")));
        assert_eq!(result["status"], "complete");
        assert_eq!(result["steps"].as_array().unwrap().len(), n);
        for name in result["steps"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            let mut files = BTreeMap::new();
            for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
                let bytes = fs::read(r(format!("{w}/{name}/{f}"))).unwrap();
                if f == "exit.json" {
                    let v: Value = serde_json::from_slice(&bytes).unwrap();
                    assert_eq!(v["returncode"], 0);
                    assert_eq!(v["success"], true);
                }
                files.insert(f, encoded(&bytes));
                streams += 1;
            }
            receipts.push(json!({"workflow":w,"name":name,"files":files}));
        }
        for f in [
            "result.json",
            "source-pins.json",
            "protocol.json",
            "completed.json",
        ] {
            fs::copy(r(format!("{w}/{f}")), d.join(format!("{w}-{f}"))).unwrap();
        }
    }
    assert_eq!(receipts.len(), 289);
    assert_eq!(streams, 1156);
    compressed(&d, "workflow-receipts", &json!(receipts));
    let mut inv = read(r("proof-inventory.json"));
    for (p, h) in read(r("training-proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert!(inv
            .as_object_mut()
            .unwrap()
            .insert(p.clone(), h.clone())
            .is_none());
    }
    let mut proof = BTreeMap::new();
    for (p, h) in inv.as_object().unwrap() {
        let b = fs::read(p).unwrap();
        assert_eq!(digest(&b), h.as_str().unwrap());
        proof.insert(p, encoded(&b));
    }
    assert_eq!(proof.len(), 354);
    compressed(&d, "proof-public-members", &json!(proof));
    let mut profiles = vec![];
    let mut raw = BTreeMap::new();
    for (name, p) in a["diagnostics"]["profiles"].as_object().unwrap() {
        let mut symbols = vec![];
        let mut period = 0u64;
        for s in p["symbols"].as_array().unwrap() {
            let raw = s["symbol"].as_str().unwrap();
            let pretty = rustc_demangle::try_demangle(raw)
                .map(|s| format!("{s:#}"))
                .unwrap_or_else(|_| raw.into());
            let w = s["period"].as_u64().unwrap();
            if pretty.starts_with("zopro_core::msm::g2_borrowed::apply::<") {
                period += w;
            }
            symbols.push(json!({"raw":raw,"demangled":pretty,"samples":s["samples"],"period":w}));
        }
        profiles.push(json!({"name":name,"samples":p["samples"],"defining_g2_apply_period_fraction":period as f64/p["period"].as_u64().unwrap() as f64,"symbols":symbols,"zero_lost":p["lost"]["zero_lost"]}));
        let path = r(format!("perf/{name}.data"));
        assert!(Command::new("sudo")
            .args(["-n", "chown", "ubuntu:ubuntu"])
            .arg(&path)
            .status()
            .unwrap()
            .success());
        raw.insert(
            path.display().to_string(),
            json!({"sha256":hash(&path),"bytes":fs::metadata(&path).unwrap().len()}),
        );
    }
    assert_eq!(profiles.len(), 3);
    compressed(&d, "demangled-profile-symbols", &json!(profiles));
    save(d.join("local-perf-data.json"), &json!(raw));
    let mut code_files = BTreeMap::new();
    let mut paths = vec![];
    walk(&r("codegen"), &mut paths);
    for p in paths {
        code_files.insert(
            p.strip_prefix(R).unwrap().display().to_string(),
            encoded(&fs::read(p).unwrap()),
        );
    }
    compressed(&d, "linked-code", &json!(code_files));
    for f in [
        "analysis.json",
        "finish.log",
        "design.md",
        "history-audit.md",
        "bootstrap.rs",
        "setup.rs",
        "native.rs",
        "train-driver.rs",
        "train.rs",
        "train-tests.log",
        "training-audit.json",
        "training-proof-inventory.json",
        "training-prepare.log",
        "training-cases.json",
        "training-jobs.json",
        "training-plan.json",
        "training-pins.json",
        "audit.rs",
        "audit.log",
        "code-audit.json",
        "source-audit.json",
        "control-flags.txt",
        "generate-flags.txt",
        "candidate-flags.txt",
        "build-pins.json",
        "build-plan.json",
        "use-pins.json",
        "use-plan.json",
        "use-prepare.log",
        "frozen-control.json",
        "frozen-generate.json",
        "frozen-candidate.json",
        "frozen-control-cargo.jsonl",
        "frozen-generate-cargo.jsonl",
        "frozen-candidate-cargo.jsonl",
        "native/control/artifacts.json",
        "native/generate/artifacts.json",
        "native-use/candidate/artifacts.json",
        "identities.json",
        "make-capture.rs",
        "capture-setup.rs",
        "capture.rs",
        "pins.json",
        "run-plan.json",
        "jobs.json",
        "cases.json",
        "preflight-cases.json",
        "prior-proof-sha256.json",
        "comparison-prior-proof-sha256.json",
        "proof-inventory.json",
        "pmu-feasibility.log",
        "prepare.log",
        "host-before.txt",
        "host-after.txt",
        "initial-capture-controller/capture-setup.rs",
        "initial-capture-controller/capture.rs",
        "initial-capture-controller/make-capture.rs",
        "initial-capture-controller/prepare.log",
        "initial-capture-controller/helper-sha256.txt",
        "initial-capture-controller/failure.json",
        "initial-capture-controller/repair.json",
        "make-report.rs",
        "report.rs",
        "report-body.rs",
    ] {
        let to = d.join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        if f == "pmu-feasibility.log" || f == "train-tests.log" {
            save(
                to.with_extension("log.json"),
                &encoded(&fs::read(r(f)).unwrap()),
            );
        } else {
            fs::copy(r(f), to).unwrap();
        }
    }
    save(
        d.join("accepted-comparison.json"),
        &json!({"manifest":M,"manifest_sha256":hash(M),"m5":manifest["m5"],"ms":manifest["ms"],"native_binary_sha256":manifest["native_binary_sha256"],"inherited_not_remeasured":true,"primary_peak_RSS_MiB":134,"full_performance_gate_passed":false,"physical16_calibration_engineering_exception_retained":true,"final_artifact_gate_passed":true}),
    );
    archive_training(&d);
    let close = json!({"primary_latency_pilot_gate_passed":true,"candidate_all8_qualified":false,"native_plain_steps":9,"training_steps":109,"native_USE_steps":6,"performance_steps":165,"receipts":289,"original_streams_byte_exact":1156,"private_source_files":175,"originals_exact":175,"training_SDK_requests":126,"performance_SDK_requests":7902,"total_SDK_requests":8028,"training_Gnark":99,"performance_Gnark":78,"new_Gnark_total":177,"proof_public_members":354,"proof_ledger":18847,"closing_pins_verified":pins.as_object().unwrap().len(),"adopted":false,"production_changed":false,"goal_active_unmet":true,"accepted_all8_CPU10x_rows":0,"no_successful_measurements_or_tests_repeated":true,"initial_prepare_failure_before_capture_retained":true});
    save(d.join("closeout.json"), &close);
    save(r("closeout.json"), &close);
    write_readme(&a, &manifest, &profiles, &d);
    seal();
    println!("Archived proofPGO175 positivepilot;8028SDK177Gnarkledger18847;no adoption;broaderqualificationrequired");
}
include!("report-body.rs");
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive|seal"),
    }
}

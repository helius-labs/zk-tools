use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-inverse-direct-sdk175";
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
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["SDK_requests"], 7902);
    assert_eq!(a["independent_proof_count"], 18154);
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
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let bytes = fs::read(r("source").join(p)).unwrap();
        let recovered = if p == audit["changed_file"].as_str().unwrap() {
            let text = std::str::from_utf8(&bytes).unwrap();
            let after = audit["after"].as_str().unwrap();
            assert_eq!(text.matches(after).count(), 1);
            text.strip_suffix(audit["suffix"].as_str().unwrap())
                .unwrap()
                .replacen(after, audit["before"].as_str().unwrap(), 1)
                .into_bytes()
        } else {
            bytes.clone()
        };
        assert_eq!(digest(&recovered), h.as_str().unwrap());
        if digest(&bytes) == h.as_str().unwrap() {
            exact += 1;
        }
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(exact, 174);
    let mut paths = vec![];
    walk(&r("source"), &mut paths);
    let mut source = BTreeMap::new();
    for p in paths {
        source.insert(
            p.strip_prefix(r("source")).unwrap().display().to_string(),
            encoded(&fs::read(p).unwrap()),
        );
    }
    assert_eq!(source.len(), 176);
    compressed(&d, "source-snapshot", &json!(source));
    let mut receipts = vec![];
    let mut streams = 0;
    for (w, n) in [("native", 13), ("runs", 165)] {
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
    assert_eq!(receipts.len(), 178);
    assert_eq!(streams, 712);
    compressed(&d, "workflow-receipts", &json!(receipts));
    let inv = read(r("proof-inventory.json"));
    let mut proof = BTreeMap::new();
    for (p, h) in inv.as_object().unwrap() {
        let b = fs::read(p).unwrap();
        assert_eq!(digest(&b), h.as_str().unwrap());
        proof.insert(p, encoded(&b));
    }
    assert_eq!(proof.len(), 156);
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
            if pretty.starts_with("zopro_bn254::inverse::inverse::<") {
                period += w;
            }
            symbols.push(json!({"raw":raw,"demangled":pretty,"samples":s["samples"],"period":w}));
        }
        profiles.push(json!({"name":name,"samples":p["samples"],"defining_inverse_period_fraction":period as f64/p["period"].as_u64().unwrap() as f64,"symbols":symbols,"zero_lost":p["lost"]["zero_lost"]}));
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
        "bounds.md",
        "direct_finish.rs",
        "bootstrap.rs",
        "setup.rs",
        "native.rs",
        "code-audit.rs",
        "code-audit.json",
        "source-audit.json",
        "native-flags.txt",
        "candidate-flags.txt",
        "build-pins.json",
        "build-plan.json",
        "frozen-control.json",
        "frozen-candidate.json",
        "frozen-control-cargo.jsonl",
        "frozen-candidate-cargo.jsonl",
        "native/control/artifacts.json",
        "native/candidate/artifacts.json",
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
        "proof-inventory.json",
        "pmu-feasibility.log",
        "prepare.log",
        "host-before.txt",
        "host-after.txt",
        "initial-setup/setup.rs",
        "initial-setup/failure.json",
        "initial-controller/capture.rs",
        "initial-controller/capture-setup.rs",
        "initial-controller/make-capture.rs",
        "initial-controller/review.json",
        "report.rs",
    ] {
        let to = d.join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        if f == "pmu-feasibility.log" {
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
    let close = json!({"gate_passed":false,"SDK_requests":7902,"new_Gnark_verifications":78,"proof_public_members":156,"proof_ledger":18154,"native_workflow_steps":13,"capture_workflow_steps":165,"receipts":178,"original_streams_byte_exact":712,"private_source_files":176,"originals_exact":174,"reverse175_exact":true,"closing_pins_verified":pins.as_object().unwrap().len(),"adopted":false,"production_changed":false,"goal_active_unmet":true,"all8_CPU10x_rows":0,"no_successful_measurements_rerun":true,"old_primitive_schedule_not_repeated":true});
    save(d.join("closeout.json"), &close);
    save(r("closeout.json"), &close);
    let mut b=String::from("# Direct inverse finish: full SDK pilot\n\n**The pilot gain gate fails; no demonstrated proof-latency improvement and no adoption.** Candidate primary latency is 8.028322 ms versus 7.983257 ms for fresh OFF and 8.001953 ms for the accepted executable. The paired ON/OFF estimate is 0.114% slower, with ratio interval 0.999476–1.007180 and 5/15 wins. Against accepted it is 0.342% slower, interval 0.998736–1.006481 and 5/15 wins. Both intervals include small improvements and regressions; this is not proof of a definitive slowdown. The fresh/accepted calibration passes.\n\n## Change and control identities\n\nThe exact prior direct-finish routine reduces the signed Euclidean linear form directly modulo the field prime. It removes the old radix64 cancellation and scale-restoring field multiplication after the unchanged one-word Euclidean loop. Supported-modulus guards, GCD matrices, representations, solver, MSM, scheduler and scratch remain unchanged. It adds no heap allocation, key table or point copy. [Bounds](artifacts/bounds.md), [source audit](artifacts/source-audit.json).\n\nThe earlier primitive comparison accidentally linked ON twice and remains unqualified. This is its first full SDK integration; that primitive timing schedule is not repeated. Separate build directories and immutable hashed libraries are audited for actual cfg/features. OFF and accepted SDKs are byte-identical (`20fde19f…`); ON is `899a0cb6…`. Their field/core/prover libraries and SDK identities differ. Actual linked inverse bodies shrink from 7,723 to 7,013 bytes for each Fq copy and 7,786 to 7,078 for Fr. These static sizes establish a code change, not faster execution. [Code audit](artifacts/code-audit.json).\n\n## Complete verified requests\n\nRyzen 9950X, 16 physical / 32 logical CPUs, 32 workers pinned to CPUs 0–31. All nine shapes pass accepted/OFF/ON preflight, followed by 15 fixed rotated/reversed primary blocks, each with three primes and 151 measured requests. Loading is excluded; fresh witness solving, randomized proving, SDK verification and serialization are included. Displayed medians need not reproduce paired-ratio estimates.\n\n|Variant|Median ms|CPU ms/request|Median/max RSS MiB|M5 CPU / measured|M5 Metal / measured|\n|---|---:|---:|---:|---:|---:|\n");
    for v in ["accepted", "OFF", "ON"] {
        let c = &a["cells"][format!("1x2-w32/{v}")];
        let t = c["median_ms"].as_f64().unwrap();
        writeln!(
            b,
            "|{v}|{t:.6}|{:.3}|{:.2}/{:.2}|{:.3}×|{:.3}×|",
            c["CPU_ms"].as_f64().unwrap(),
            c["median_RSS_MiB"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap(),
            37. / t,
            24. / t
        )
        .unwrap();
    }
    b+="\nProcess high-water RSS includes loading/primes; it is not minimum RAM or an allocation bound. No memory-saving claim follows from the smaller inverse functions.\n\nBoth ON/control upper paired ratios must be below one with at least 10/15 wins. The OFF/accepted interval must lie within [1/1.03, 1.03]. Sorted ranks 4–12 of 15 have nominal 96.484375% individual coverage under independent exchangeable blocks, without a familywise guarantee. There is no minimum positive-gain floor, exclusion, extension, successful rerun, flag/layout sweep or partial promotion. Both gain conditions fail. [Design](artifacts/design.md), [all observations and checks](artifacts/analysis.json).\n\nBoth coherent native builds pass all 34 field, 124 core and 8 prover tests. The 68,672 signed-residue field cases are inherited inputs freshly executed in both builds, not newly distinct cases. Original inverse source and the candidate helper match their prior inputs byte-for-byte. The 176-file private snapshot has 174 exact originals; reversing the one original edit recovers all 175 production files.\n\nAll 7,902 complete SDK requests verify internally. All 78 fresh 164-byte exports independently pass Gnark, exact image/public-input binding and uniqueness against 18,076 prior proof identities. The ledger is now 18,154; 156 proof/public members are archived. No successful test, capture or independent verification is repeated.\n\n## CPU diagnostics\n\nAfter all latency captures and their verification, three separately gated four-event stat runs measure 31 requests each, followed by three separately gated 199 Hz user-cycle profiles of 251 requests each. Every process primes three times; FIFO gates exclude loading/primes. All six captures precede diagnostic verification/decoding. All stat events run at least 99.9%; every profile has one sampled process and zero lost records. No owned build/helper/hash/archive/other CPU job overlaps captures. The host is shared; exclusive access is not claimed.\n\n|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|\n|---|---:|---:|---:|---:|\n";
    for p in a["diagnostics"]["counters"].as_object().unwrap().values() {
        let e = &p["events"];
        writeln!(
            b,
            "|{}|{:.3}|{:.3}|{:.3}|{:.3}|",
            p["variant"].as_str().unwrap(),
            e["cycles:u"]["per_request"].as_f64().unwrap() / 1e6,
            e["instructions:u"]["per_request"].as_f64().unwrap() / 1e6,
            e["cache-misses:u"]["per_request"].as_f64().unwrap() / 1e6,
            e["branch-misses:u"]["per_request"].as_f64().unwrap() / 1e6
        )
        .unwrap();
    }
    b += "\n|Profile|Samples|Defining inverse-symbol period|\n|---|---:|---:|\n";
    for p in &profiles {
        writeln!(
            b,
            "|{}|{}|{:.2}%|",
            p["name"].as_str().unwrap(),
            p["samples"],
            100. * p["defining_inverse_period_fraction"].as_f64().unwrap()
        )
        .unwrap();
    }
    b+="\nRaw/demangled symbol weights and original decoded streams are retained. These are sampled IP periods and one diagnostic run per variant; they are not causal stall attribution, exclusive inversion cost, removable wall time or a latency selection criterion. Raw perf files remain locally with hashes.\n\nAn initial reverse-source check consumed one original newline; the pattern was fixed before native builds. A controller-generator list anchor was corrected during static review before preparing or executing captures. Initial sources and notes are retained. Neither repair changed arithmetic, timing thresholds or successful observations. All 13 native and 165 capture-workflow steps finish. The archive checks 178 receipts and 712 original streams byte-for-byte, source recovery, proof/public hashes, compressed round trips and closing pins. [Closeout](artifacts/closeout.json).\n\n## Inherited accepted all-eight CPU and Metal comparison\n\nThese historical accepted-source measurements use supplied M5 references; they are not fresh candidate all-eight latency measurements. Only the primary comparison above is newly timed. The accepted 1×2 peak RSS remains 134 MiB. Its full performance gate remains false with the recorded physical16 calibration engineering exception; the final artifact gate remains true.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n";
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let row = &manifest["m5"][shape];
        let t = row["accepted_ms"].as_f64().unwrap();
        let cpu = row["cpu_reference_ms"].as_f64().unwrap();
        let metal = row["metal_reference_ms"].as_f64().unwrap();
        writeln!(
            b,
            "|{shape}|{t:.6}|{cpu:.0}|{:.3}×|{metal:.0}|{:.3}×|{:.1}|",
            cpu / t,
            metal / t,
            cpu / 10.
        )
        .unwrap();
    }
    b+="\nAccepted 54→1 remains 162.250464 ms without a supplied M5 reference. No candidate portable/mobile/TEE, physical16, memory-pressure, concurrency, 250/4999, minimum-RAM, AWS or Aeglos qualification follows. Production source 175 / SDK20fde remains unchanged. The exact direct-finish SDK integration is closed by this failed pilot; it is not selected for all-eight qualification. The all-eight CPU 10× goal remains active and unmet (0/8).\n";
    b = b.replace(
        "Their field/core/prover libraries and SDK identities differ.",
        "OFF and ON have different field/core/prover library hashes and SDK hashes.",
    );
    fs::write(Path::new(D).join("README.md"), b).unwrap();
    seal();
    println!("Archived 7902 SDK requests /78 fresh Gnark proofs, no adoption");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive|seal"),
    }
}

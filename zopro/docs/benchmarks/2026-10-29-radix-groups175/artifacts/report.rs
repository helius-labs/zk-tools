use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/radix-groups175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-radix-groups175";
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
fn num(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn archive() {
    let a = read(r("analysis.json"));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["SDK_requests"], 20142);
    assert_eq!(a["independent_proof_count"], 18592);
    assert_eq!(a["new_independent_exports"], 438);
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
        exact += usize::from(digest(&bytes) == h.as_str().unwrap());
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
    for w in ["native", "runs"] {
        let mut names = if w == "native" {
            let completed = read(r("native/completed.json"));
            assert_eq!(completed.as_array().unwrap().len(), 10);
            assert!(!r("native/result.json").exists());
            assert_eq!(read(r("audit-review.json"))["passed"], true);
            completed.as_array().unwrap().clone()
        } else {
            let result = read(r("runs/result.json"));
            assert_eq!(result["status"], "complete");
            assert_eq!(result["steps"].as_array().unwrap().len(), 885);
            result["steps"].as_array().unwrap().clone()
        };
        if w == "native" {
            names.push(json!("code-audit"));
        }
        for name in names {
            let name = name.as_str().unwrap();
            let failed = w == "native" && name == "code-audit";
            let mut files = BTreeMap::new();
            for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
                let bytes = fs::read(r(format!("{w}/{name}/{f}"))).unwrap();
                if f == "exit.json" {
                    let v: Value = serde_json::from_slice(&bytes).unwrap();
                    assert_eq!(v["returncode"], if failed { 101 } else { 0 });
                    assert_eq!(v["success"], !failed);
                }
                files.insert(f, encoded(&bytes));
                streams += 1;
            }
            receipts.push(
                json!({"workflow":w,"name":name,"original_failed_audit":failed,"files":files}),
            );
        }
        for f in ["source-pins.json", "protocol.json", "completed.json"] {
            fs::copy(r(format!("{w}/{f}")), d.join(format!("{w}-{f}"))).unwrap();
        }
    }
    fs::copy(r("runs/result.json"), d.join("runs-result.json")).unwrap();
    fs::copy(r("native/failure.json"), d.join("native-failure.json")).unwrap();
    assert_eq!(receipts.len(), 896);
    assert_eq!(streams, 3584);
    compressed(&d, "workflow-receipts", &json!(receipts));
    let inv = read(r("proof-inventory.json"));
    let mut proof = BTreeMap::new();
    for (p, h) in inv.as_object().unwrap() {
        let bytes = fs::read(p).unwrap();
        assert_eq!(digest(&bytes), h.as_str().unwrap());
        proof.insert(p, encoded(&bytes));
    }
    assert_eq!(proof.len(), 876);
    compressed(&d, "proof-public-members", &json!(proof));
    let mut profiles = vec![];
    let mut raw = BTreeMap::new();
    for (name, p) in a["diagnostics"]["profiles"].as_object().unwrap() {
        let mut symbols = vec![];
        let mut period = 0u64;
        assert_eq!(p["lost"]["zero_lost"], true);
        assert_eq!(p["PIDs"].as_array().unwrap().len(), 1);
        for s in p["symbols"].as_array().unwrap() {
            let symbol = s["symbol"].as_str().unwrap();
            let pretty = rustc_demangle::try_demangle(symbol)
                .map(|s| format!("{s:#}"))
                .unwrap_or_else(|_| symbol.into());
            let weight = s["period"].as_u64().unwrap();
            if pretty.starts_with("zopro_core::msm_affine::grouping::scalar_groups") {
                period += weight;
            }
            symbols.push(
                json!({"raw":symbol,"demangled":pretty,"samples":s["samples"],"period":weight}),
            );
        }
        profiles.push(json!({"name":name,"samples":p["samples"],"defining_grouping_entry_period_fraction":period as f64/p["period"].as_u64().unwrap() as f64,"symbols":symbols,"zero_lost":true}));
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
    let mut code = BTreeMap::new();
    for dir in ["codegen", "codegen-v2"] {
        let mut paths = vec![];
        walk(&r(dir), &mut paths);
        for p in paths {
            code.insert(
                p.strip_prefix(R).unwrap().display().to_string(),
                encoded(&fs::read(p).unwrap()),
            );
        }
    }
    compressed(&d, "linked-code", &json!(code));
    for f in [
        "analysis.json",
        "finish.log",
        "design.md",
        "history-audit.md",
        "memory-review.md",
        "radix.rs",
        "bootstrap.rs",
        "setup.rs",
        "native.rs",
        "code-audit.rs",
        "code-audit-v2.rs",
        "code-audit-v2.json",
        "make-code-audit-v2.rs",
        "audit.rs",
        "audit.log",
        "audit-review.json",
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
        "initial-audit-controller/capture-setup.rs",
        "initial-audit-controller/make-capture.rs",
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
    let close = json!({"gate_passed":false,"SDK_requests":20142,"new_Gnark_verifications":438,"proof_public_members":876,"proof_ledger":18592,"successful_native_workflow_steps":10,"original_native_workflow_exit":1,"original_audit_exit":101,"corrected_audit_direct_exec_exit":0,"corrected_audit_is_xtask_receipt":false,"capture_workflow_steps":885,"receipts":896,"original_streams_byte_exact":3584,"private_source_files":176,"originals_exact":174,"reverse175_exact":true,"closing_pins_verified":pins.as_object().unwrap().len(),"adopted":false,"production_changed":false,"goal_active_unmet":true,"all8_CPU10x_rows":0,"no_successful_measurements_rerun":true});
    save(d.join("closeout.json"), &close);
    save(r("closeout.json"), &close);
    let mut b=String::from("# Radix grouping: full SDK comparison\n\n**The full performance gate fails; this candidate is not adopted.** Primary ON latency is 8.012793 ms versus 8.045524 ms for fresh OFF and 8.025797 ms for the accepted executable. The paired savings estimates are +38.883 µs against OFF and +10.861 µs against accepted; both intervals cross parity and both win counts miss the fixed gain condition. The original-eight aggregate is 2.073% slower than OFF and 3.947% slower than accepted. Primary maximum observed RSS falls to 86 MiB from 135/134.5 MiB, respectively. These positive observations are retained without claiming a qualified speedup or minimum RAM.\n\n## Implementation and controls\n\nThe proof-local builder replaces the owned full-scalar hash map with four stable byte-radix passes over packed 32-bit fingerprints and 32-bit original indices. Full 256-bit comparison resolves every fingerprint collision; no equality decision depends on the fingerprint. An all-equal run uses a linear scan, other runs use exact scalar/index sorting. Original input order determines class numbering and membership order. Sampling, 80/95% cutoffs, resource reservation and canonicalization remain unchanged. The original builder is retained when the checked workspace forecast cannot fit its existing reservation. There are no key lookup tables, point/key copies, added dependencies, new parallel work or persistent caches. [Source audit](artifacts/source-audit.json), [workspace review](artifacts/memory-review.md).\n\nThe explicit payload forecast covers both radix vectors, classes, returned metadata and scatter counters using the existing checked capacity helpers. It must fit the untouched old grouping-cost bound before allocation. This is an allocation-payload bound, not RSS or minimum RAM; compiler frames, allocator retention and other proof stages remain outside it. No low-memory or huge-circuit qualification is inferred.\n\nAccepted SDK is `20fde19f…`, fresh OFF `e84d0701…`, ON `cd72bf0b…`. OFF is not byte-identical to accepted, although its grouping entry has the same 7,624-byte size; ON is 13,661 bytes. LLVM inlines the radix builder into that entry. ON-only standard-library sort specializations carry the radix comparison closure in their generic names; they are not standalone defining radix functions. Actual linked code, cfg/features and immutable library hashes are retained. Static code size is not speed evidence. [Corrected code audit](artifacts/code-audit-v2.json).\n\n## Full verified latency matrix\n\nRyzen 9950X, 16 physical / 32 logical CPUs; 32 workers pinned to CPUs 0–31. All nine shapes have accepted/OFF/ON preflight and 15 fixed rotated/reversed blocks. Each process primes three times; primary blocks measure 151 requests and other shapes 31. Loading is excluded from latency; witness solving, randomized proving, SDK verification and serialization are included. Displayed cell medians need not reproduce paired estimates.\n\n|Shape|Variant|Median ms|CPU ms/request|Median/max RSS MiB|\n|---|---|---:|---:|---:|\n");
    let shapes = [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
        "merge-54x1",
    ];
    for shape in shapes {
        for v in ["accepted", "OFF", "ON"] {
            let c = &a["cells"][format!("{shape}-w32/{v}")];
            writeln!(
                b,
                "|{shape}|{v}|{:.6}|{:.3}|{:.2}/{:.2}|",
                num(c, "median_ms"),
                num(c, "CPU_ms"),
                num(c, "median_RSS_MiB"),
                num(c, "max_RSS_MiB")
            )
            .unwrap();
        }
    }
    b+="\nRSS is process high-water memory including loading and primes. Lower primary RSS does not establish the required RAM, pressure behavior or a causal explanation for latency.\n\n|Shape|Comparison|Paired ratio|Interval|Wins / 15|Paired saving µs|\n|---|---|---:|---:|---:|---:|\n";
    for shape in shapes {
        for comparison in ["OFF/accepted", "ON/OFF", "ON/accepted"] {
            let p = &a["pairs"][format!("{shape}-w32/{comparison}")];
            writeln!(
                b,
                "|{shape}|{comparison}|{:.6}|{:.6}–{:.6}|{}|{:+.3}|",
                num(p, "median"),
                num(p, "low"),
                num(p, "high"),
                p["wins"],
                1000. * num(p, "paired_saving_ms")
            )
            .unwrap();
        }
    }
    b += "\n|Original-eight equal-log aggregate|Ratio|Interval|Wins / 15|\n|---|---:|---:|---:|\n";
    for (name, p) in a["original8_aggregate"].as_object().unwrap() {
        writeln!(
            b,
            "|{name}|{:.6}|{:.6}–{:.6}|{}|",
            num(p, "median"),
            num(p, "low"),
            num(p, "high"),
            p["wins"]
        )
        .unwrap();
    }
    b+="\nRanks 4–12 of 15 have nominal 96.484375% individual coverage under independent exchangeable blocks, without a familywise guarantee. The fixed gate requires OFF/accepted intervals inside [1/1.03, 1.03], ON upper nonregression ratios ≤1.03 against both controls for every shape, and primary plus original-eight aggregate upper ratios below one with at least 10/15 wins against both controls. OFF/accepted calibration fails on 4×2, 8×2 and 12×2. Both primary gain conditions, both aggregate gain conditions and several large-shape nonregression conditions fail. All 18 failed checks remain in the analysis. No threshold, exclusion, extension, successful rerun, fingerprint-width sweep or shape-subset promotion follows these results. [Fixed design](artifacts/design.md), [all observations and checks](artifacts/analysis.json).\n\n## Correctness and audit history\n\nBoth fresh OFF and ON pass all 128 core and 8 prover tests. The four new core tests check stable radix ordering, exact grouping metadata against an independent ordered-map oracle across 256 cases including forced total fingerprint collisions, 522,288 checked workspace cases, and compiled OFF/ON dispatch. Existing G1/G2 weighted-generator and grouping cases remain passing. All 17 field files are byte-exact; 33 field-test results are inherited, not rerun. The private snapshot contains 176 files, with 174 unchanged originals; reversing one edit recovers all 175 production files.\n\nAll 20,142 complete SDK requests verify internally. All 438 fresh 164-byte exports pass independent Gnark verification, exact image/public-input binding and uniqueness against the 18,154 prior proof identities. The proof ledger is 18,592, with 876 proof/public members archived.\n\nThe native workflow completed its ten build/test/freeze/link steps, then failed its original code audit (exit 101). That audit incorrectly required a standalone defining radix child symbol. The corrected audit checks all 366 original build pins, immutable library selection, test/dispatch receipts and actual grouping code plus ON-only collision-sort specializations. It was executed directly and observed exit 0 before the first measurement; `audit.log` is retained, and this is not represented as an xtask receipt. Initial sources, the original failure and the corresponding pre-capture controller repair are archived. No implementation, measurement gate or successful test/build was changed or repeated. All 885 capture-workflow steps complete. [Audit review](artifacts/audit-review.json), [original failure](artifacts/native-failure.json).\n\n## CPU diagnostics\n\nAll latency captures precede their 405 Gnark verifications. Three FIFO-gated four-event stat runs then measure 31 requests each, and three FIFO-gated 199 Hz user-cycle profiles measure 251 requests each. All diagnostic captures precede their verification and decoding; loading and three primes are outside each FIFO gate. All events run at least 99.9%; each profile has one sampled process and zero lost records. No owned build/helper/hash/archive/other CPU job overlaps captures; the shared host is not claimed exclusive.\n\n|Variant|Cycles/request, million|Instructions/request, million|Cache misses/request, million|Branch misses/request, million|\n|---|---:|---:|---:|---:|\n";
    for p in a["diagnostics"]["counters"].as_object().unwrap().values() {
        let e = &p["events"];
        writeln!(
            b,
            "|{}|{:.3}|{:.3}|{:.3}|{:.3}|",
            p["variant"].as_str().unwrap(),
            num(&e["cycles:u"], "per_request") / 1e6,
            num(&e["instructions:u"], "per_request") / 1e6,
            num(&e["cache-misses:u"], "per_request") / 1e6,
            num(&e["branch-misses:u"], "per_request") / 1e6
        )
        .unwrap();
    }
    b += "\n|Profile|Samples|Defining grouping-entry sampled period|\n|---|---:|---:|\n";
    for p in profiles {
        writeln!(
            b,
            "|{}|{}|{:.2}%|",
            p["name"].as_str().unwrap(),
            p["samples"],
            100. * num(&p, "defining_grouping_entry_period_fraction")
        )
        .unwrap();
    }
    b+="\nDefining-entry attribution uses the demangled source-path prefix, not a radix name embedded inside a standard-library generic context. Raw/demangled symbols and original decoded streams remain available. These are sampled IP periods and single diagnostic runs, not causal stall attribution, exclusive grouping cost or removable wall time. Raw perf data remain local with hashes.\n\n## Supplied CPU and Metal comparisons\n\nThe following accepted/OFF/ON speedups use the fresh full matrix above and the supplied M5 timings. They are comparisons across machines; no new M5 run is claimed. All eight ON CPU 10× conditions remain false.\n\n|Shape|M5 CPU ms|Accepted/OFF/ON CPU speedup|M5 Metal ms|Accepted/OFF/ON Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|\n";
    for shape in &shapes[..8] {
        let row = &a["new_reference_comparisons"][format!("{shape}-w32")];
        let v = &row["variants"];
        let rf = &row["reference"];
        writeln!(
            b,
            "|{shape}|{:.0}|{:.3}× / {:.3}× / {:.3}×|{:.0}|{:.3}× / {:.3}× / {:.3}×|{:.1}|",
            num(rf, "cpu_ms"),
            num(&v["accepted"], "CPU_speedup"),
            num(&v["OFF"], "CPU_speedup"),
            num(&v["ON"], "CPU_speedup"),
            num(rf, "metal_ms"),
            num(&v["accepted"], "Metal_speedup"),
            num(&v["OFF"], "Metal_speedup"),
            num(&v["ON"], "Metal_speedup"),
            num(rf, "cpu_ms") / 10.
        )
        .unwrap();
    }
    b+="\n54→1 has no supplied M5 reference. The accepted historical production primary remains 8.022165 ms / 134 MiB maximum RSS; production source 175 and SDK `20fde19f…` are unchanged. Its full performance gate remains false with the recorded physical16 calibration engineering exception, and its final artifact gate remains true. No candidate portable/mobile/TEE, physical16, memory-pressure, concurrency, 250/4999, minimum-RAM, AWS or Aeglos qualification follows this matrix. The archive retains 896 receipts (including the failed original audit), 3,584 original streams, all private sources and proof/public members, with checked compression round trips and closing pins. [Closeout](artifacts/closeout.json).\n\nThis exact universal radix builder is closed by its failed full matrix. The lower primary RSS and small positive primary estimates remain evidence; they do not justify adoption or rescue a subset of the fixed experiment. The all-eight CPU 10× goal remains active and unmet (0/8).\n";
    fs::write(Path::new(D).join("README.md"), b).unwrap();
    seal();
    println!("Archived radix groups:20142 SDK/438 new Gnark/ledger18592; no adoption");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive|seal"),
    }
}

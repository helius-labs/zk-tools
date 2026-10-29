use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const R: &str = "/home/ubuntu/src/zopro-perf-work/complex-rowq175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-complex-rowq175";
const M: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hash(p: impl AsRef<Path>) -> String {
    digest(&fs::read(p).unwrap())
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn compressed(v: &Value, name: &str, d: &Path) {
    let bytes = serde_json::to_vec(v).unwrap();
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
    let out = Command::new("gzip")
        .arg("-dc")
        .arg(target)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
}
fn encoded(bytes: &[u8]) -> Value {
    match std::str::from_utf8(bytes) {
        Ok(text) => json!({"encoding":"utf8","data":text,"sha256":digest(bytes)}),
        Err(_) => {
            json!({"encoding":"hex","data":bytes.iter().map(|b|format!("{b:02x}")).collect::<String>(),"sha256":digest(bytes)})
        }
    }
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
    let d = Path::new(D).join("artifacts");
    assert!(!d.exists());
    fs::create_dir_all(&d).unwrap();
    let a = read(r("analysis.json"));
    assert_eq!(a["gates"]["caller_passed"], false);
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read(M);
    assert_eq!(
        hash(manifest["native_binary"].as_str().unwrap()),
        manifest["native_binary_sha256"].as_str().unwrap()
    );
    assert_eq!(manifest["full_performance_gate_passed"], false);
    assert_eq!(manifest["final_artifact_gate_passed"], true);
    let audit = read(r("source-audit.json"));
    let mut source = BTreeMap::new();
    let mut exact = 0usize;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let bytes = fs::read(r("source").join(p)).unwrap();
        let mut recovered = bytes.clone();
        if let Some(suffix) = audit["suffixes"][p].as_str() {
            let n = recovered.len() - suffix.len();
            assert_eq!(&recovered[n..], suffix.as_bytes());
            recovered.truncate(n);
        }
        if p.ends_with("msm/g2_borrowed.rs") {
            let mut text = String::from_utf8(recovered).unwrap();
            for edit in audit["borrowed_body_edits"].as_array().unwrap() {
                let old = edit[0].as_str().unwrap();
                let new = edit[1].as_str().unwrap();
                assert_eq!(text.matches(new).count(), 1);
                text = text.replacen(new, old, 1);
            }
            recovered = text.into_bytes();
        }
        assert_eq!(digest(&recovered), h.as_str().unwrap());
        if digest(&bytes) == h.as_str().unwrap() {
            exact += 1;
        }
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(exact, 170);
    let mut paths = vec![];
    walk(&r("source"), &mut paths);
    paths.sort();
    for p in paths {
        let bytes = fs::read(&p).unwrap();
        source.insert(
            p.strip_prefix(r("source")).unwrap().display().to_string(),
            encoded(&bytes),
        );
    }
    assert_eq!(source.len(), 177);
    compressed(&json!(source), "private-source-snapshot", &d);
    let mut receipts = vec![];
    let mut streams = 0usize;
    let mut local = vec![];
    for (workflow, count) in [("native", 7), ("linked", 2), ("runs", 880)] {
        let result = read(r(format!("{workflow}/result.json")));
        assert_eq!(result["status"], "complete");
        assert_eq!(result["steps"].as_array().unwrap().len(), count);
        for name in result["steps"].as_array().unwrap() {
            let name = name.as_str().unwrap();
            let mut files = BTreeMap::new();
            for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
                let p = r(format!("{workflow}/{name}/{f}"));
                let bytes = fs::read(&p).unwrap();
                if f == "exit.json" {
                    let v: Value = serde_json::from_slice(&bytes).unwrap();
                    assert_eq!(v["returncode"], 0);
                    assert_eq!(v["success"], true);
                }
                let record = if workflow == "runs" && name.starts_with("loss-") && f == "stdout.log"
                {
                    local.push(json!({"path":p,"bytes":bytes.len(),"sha256":digest(&bytes),"reason":"expanded perf records remain local; parsed zero-loss result retained"}));
                    json!({"local_path":p,"sha256":digest(&bytes),"bytes":bytes.len()})
                } else {
                    streams += 1;
                    encoded(&bytes)
                };
                files.insert(f, record);
            }
            receipts.push(json!({"workflow":workflow,"name":name,"files":files}));
        }
        for f in [
            "result.json",
            "protocol.json",
            "source-pins.json",
            "completed.json",
        ] {
            fs::copy(
                r(format!("{workflow}/{f}")),
                d.join(format!("{workflow}-{f}")),
            )
            .unwrap();
        }
    }
    assert_eq!(receipts.len(), 889);
    assert_eq!(streams, 3552);
    assert_eq!(local.len(), 4);
    compressed(&json!(receipts), "workflow-receipts", &d);
    compressed(&read(r("observations.json")), "observations", &d);
    let mut profiles = vec![];
    let mut total_samples = 0u64;
    for p in a["profiles"].as_array().unwrap() {
        let name = p["name"].as_str().unwrap();
        assert_eq!(
            hash(p["path"].as_str().unwrap()),
            p["sha256"].as_str().unwrap()
        );
        let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
        let mut symbols: BTreeMap<String, (u64, u64)> = BTreeMap::new();
        for l in text
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        {
            let c: Vec<_> = l.split_whitespace().collect();
            let v = symbols.entry(c[3..].join(" ")).or_default();
            v.0 += 1;
            v.1 += c[1].parse::<u64>().unwrap();
        }
        let samples: u64 = symbols.values().map(|v| v.0).sum();
        let period: u64 = symbols.values().map(|v| v.1).sum();
        assert_eq!(samples, p["samples"].as_u64().unwrap());
        total_samples += samples;
        let mut apply = 0;
        let mut rows = vec![];
        for (raw, (n, w)) in symbols {
            let pretty = rustc_demangle::try_demangle(&raw)
                .map(|n| format!("{n:#}"))
                .unwrap_or_else(|_| raw.clone());
            if pretty.starts_with("zopro_core::msm::g2_borrowed::") && pretty.contains("apply") {
                apply += w;
            }
            rows.push(json!({"raw":raw,"demangled":pretty,"samples":n,"period":w}));
        }
        rows.sort_by_key(|r| std::cmp::Reverse(r["period"].as_u64().unwrap()));
        profiles.push(json!({"name":name,"samples":samples,"period":period,"borrowed_apply_period_fraction":apply as f64/period as f64,"lost":p["lost"],"symbols":rows}));
    }
    compressed(&json!(profiles), "demangled-profile-symbols", &d);
    let mut code = vec![];
    for mode in ["frozen", "off", "on"] {
        let name = format!("codegen-{mode}-0.asm.txt");
        let raw = fs::read_to_string(r(&name)).unwrap();
        let mut instructions = 0;
        let mut stackrefs = 0;
        let mut frame = 0u64;
        for l in raw.lines() {
            let Some((address, body)) = l.split_once(':') else {
                continue;
            };
            let address = address.trim();
            if address.is_empty() || !address.chars().all(|c| c.is_ascii_hexdigit()) {
                continue;
            }
            instructions += 1;
            if body.contains("(%rsp)") {
                stackrefs += 1;
            }
            if let Some(v) = body
                .trim()
                .strip_prefix("sub    $0x")
                .and_then(|s| s.strip_suffix(",%rsp"))
            {
                frame += u64::from_str_radix(v, 16).unwrap();
            }
        }
        let display = raw
            .lines()
            .map(str::trim_end)
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(d.join(&name), format!("{}\n", display.trim_end())).unwrap();
        code.push(json!({"mode":mode,"instruction_sites":instructions,"IFMA_sites":raw.matches("vpmadd52").count(),"RSP_memory_operand_lines":stackrefs,"literal_stack_reservation_bytes":frame,"original_sha256":digest(raw.as_bytes()),"display_sha256":hash(d.join(&name)),"display_normalization":"trim trailing line whitespace and EOF blank lines only","static_not_dynamic":true}));
    }
    save(d.join("static-code.json"), &json!(code));
    save(d.join("local-only-streams.json"), &json!(local));
    save(
        d.join("accepted-comparison.json"),
        &json!({"manifest":M,"manifest_sha256":hash(M),"native_binary":manifest["native_binary"],"native_binary_sha256":manifest["native_binary_sha256"],"ms":manifest["ms"],"m5":manifest["m5"],"inherited_not_remeasured":true,"full_performance_gate_passed":false,"final_artifact_gate_passed":true,"primary_peak_RSS_MiB":134}),
    );
    for f in [
        "analysis.json",
        "analyze.log",
        "model.json",
        "model.stderr.log",
        "model-build.log",
        "model/Cargo.toml",
        "model/Cargo.lock",
        "model/src/main.rs",
        "bounds.md",
        "history-audit.md",
        "design.md",
        "kernel.rs",
        "field-tests.rs",
        "pair-prefix.rs",
        "make-kernel.rs",
        "make-native.rs",
        "setup.rs",
        "native.rs",
        "native-flags.txt",
        "driver.rs",
        "make-driver.rs",
        "probe.rs",
        "perf.rs",
        "caller_probe.rs",
        "source-audit.json",
        "build-pins.json",
        "build-plan.json",
        "link-pins.json",
        "link-plan.json",
        "pins.json",
        "run-plan.json",
        "jobs.json",
        "field-qualification.json",
        "native-tests.json",
        "inherited-frozen-tests.json",
        "artifacts-frozen.json",
        "artifacts-off.json",
        "artifacts-on.json",
        "codegen.json",
        "topology.json",
        "compiler.txt",
        "host.txt",
        "machine.txt",
        "initial-link-controller/driver.rs",
        "initial-link-controller/make-driver.rs",
        "initial-link-controller/failure.json",
        "report.rs",
    ] {
        let target = d.join(f);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(r(f), target).unwrap();
    }
    let close = json!({"caller_gate_passed":false,"adopted":false,"production_changed":false,"SDK_requests":0,"independent_proof_ledger":18076,"original_source_files_verified":175,"private_source_files":177,"private_originals_exact":170,"private_reverse175_exact":true,"final_pins_verified":pins.as_object().unwrap().len(),"workflow_receipts":889,"original_streams_encoded_byte_exact":3552,"expanded_loss_stdout_streams_local_hashed":4,"observations":868,"profile_samples":total_samples,"original8_CPU10x_achieved":0,"goal_active_unmet":true,"controller_failure_retained":true,"successful_native_tests_and_captures_rerun":false});
    save(d.join("closeout.json"), &close);
    save(r("closeout.json"), &close);
    let mut body=String::from("# Shared complex quotient preparation in the G2 caller\n\n**Rejected at the actual-caller gate; no SDK trial or adoption.** The paired aggregate of the two G2 sizes at 32 workers is 2.23% slower than fresh OFF (ratio interval 1.019249–1.026561), with 0/31 wins. It is 2.68% slower than the byte-identical frozen baseline (1.023973–1.029234), with 0/31 wins. All control calibrations pass; gain and nonregression gates fail.\n\n## Method\n\nThe new operator shares low-word inverse weights across both outputs of a normal or conjugate complex product. Three low-word products replace four per row when preparing Montgomery quotients. The original two fused CIOS product and reduction bodies, including exact lazy outputs and the `2p` negation correction, remain unchanged. It uses 315 IFMA sites per complex product versus 300 for the original and 322 for two independent row-quotient operations. This changes quotient preparation, not the number of full field products or reductions. Only the borrowed G2 slope numerator and final-y product use the operator. Norms, batch inversion, packing, tails, geometry and scratch remain original. No heap allocation, per-key table or copied point array is added. [Bounds](artifacts/bounds.md), [history audit](artifacts/history-audit.md).\n\n## Fixed caller measurements\n\nRyzen 9950X, 16 physical / 32 logical CPUs; Rust 1.98.1 / LLVM 22.1.8. Solo pins CPU 2, physical16 pins CPUs 0–15, and logical32 pins 0–31. The fixed 31 rotated/reversed blocks cover nine contexts and three variants. Each worker performs 8,192 measured batches after three separately reset 1,024-batch primes. Operands change each batch. The primary statistic is common wall time per batch wave, including all workers' completion. Paired changes need not equal ratios of displayed medians.\n\n|Caller|Size|Workers|Frozen µs|Fresh OFF µs|Candidate µs|Paired change/OFF|Wins/OFF|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for (k, n) in [
        ("g2-borrowed-signed", 200),
        ("g2-borrowed-signed", 352),
        ("g1-borrowed-signed", 200),
    ] {
        for (t, w) in [("solo", 1), ("physical16", 16), ("logical32", 32)] {
            let values: Vec<_> = (0..3)
                .map(|m| {
                    a["cells"][format!("{k}/{n}/{t}/m{m}")]["wall_ns_per_wave"]["median"]
                        .as_f64()
                        .unwrap()
                        / 1000.
                })
                .collect();
            let p = &a["pairs"][format!("{k}/{n}/{t}/2-over-1")];
            writeln!(
                body,
                "|{}|{n}|{w}|{:.3}|{:.3}|{:.3}|{:+.3}%|{}/31|",
                if k.starts_with("g2") {
                    "G2 borrowed signed"
                } else {
                    "G1 unchanged control"
                },
                values[0],
                values[1],
                values[2],
                (p["median"].as_f64().unwrap() - 1.) * 100.,
                p["wins"]
            )
            .unwrap();
        }
    }
    body+="\nG2 size 200 improves 0.804% at solo, but the unchanged G1 control also improves 0.787%. G2 size 352 solo has an uncertain 0.097% gain. Both G2 sizes regress with 16 and 32 workers. These observations do not establish a transferable solo arithmetic benefit.\n\nThe predeclared gate requires every fresh/frozen calibration interval within factor 1.03; every candidate nonregression upper ratio ≤1.03 against both controls; and both logical32 G2 sizes, individually and as their equal-log-weight aggregate, to have an upper ratio <1 and ≥21/31 wins against both controls. Rank endpoints 10–22 of 31 have nominal 97.055% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extensions, successful reruns, threshold/flag/layout sweep or subset promotion is used. The size-352 logical32 candidate/frozen nonregression upper ratio is 1.030763. [Design](artifacts/design.md), [analysis](artifacts/analysis.json).\n\n## Correctness and memory\n\nThe new independent BigUint model passes 60,168 complex products, 601,680 quotient checks and 601,680 post-row integer checks across two primes, both Montgomery radices, normal/conjugate forms, lazy boundaries, random operands and changing chains. Two new native field tests pass 690,176 output-lane comparisons against both Arkworks and exact original lazy digits. The unchanged 33 field tests are inherited. Fresh OFF and ON each pass 128 unfiltered core tests, including independent borrowed G2 generator, tail and reuse checks. The frozen baseline binary, libraries, source and 128 tests are reused byte-for-byte from the prior caller experiment after hash, flag and reversible-source checks; those tests were not rerun.\n\nAll 868 capture processes pass independent output oracles; output digests and capacities match. Every timed interval allocates zero times. G2 scratch remains 16,000 bytes per worker at size 200 and 28,160 at 352; fixture-reference vectors remain 3,200/5,632 bytes. G1 control scratch remains 40,000 bytes with 3,200 reference bytes. These are caller capacities, not full-proof peak RAM or minimum RAM.\n\nAn initial controller assertion omitted the ignored/measured fields in Rust's test summary and failed before linking or measurement. Its corrected parser also checks all four native lane counters. Initial controller sources and the failure are retained; successful native tests were not repeated. No arithmetic change followed successful qualification. [Source audit](artifacts/source-audit.json), [qualification](artifacts/field-qualification.json).\n\n## Generated code and profiles\n\n|Build|Instruction sites|IFMA sites|RSP memory operand lines|Literal stack reservation bytes|\n|---|---:|---:|---:|---:|\n";
    for c in &code {
        writeln!(
            body,
            "|{}|{}|{}|{}|{}|",
            c["mode"].as_str().unwrap(),
            c["instruction_sites"],
            c["IFMA_sites"],
            c["RSP_memory_operand_lines"],
            c["literal_stack_reservation_bytes"]
        )
        .unwrap();
    }
    body+="\nCounts use the actual linked borrowed G2 apply function and `objdump --no-show-raw-insn`, so byte-wrap lines are absent. They are static sites, not executed instruction counts, measured spill events or causal stall attribution. Additional quotient preparation coexists with larger code and stack use; these data do not isolate the regression's cause. [Code audit](artifacts/static-code.json).\n\nFour FIFO-gated `cycles:u` profiles follow all latency measurements at 199 Hz, 262,144 batches per worker, G2 size 200, solo/logical32, OFF/ON. Enable follows priming/reset and disable precedes output oracles. All recordings precede ownership changes and offline decoding.\n\n|Profile|Samples|Borrowed apply period|Lost|\n|---|---:|---:|---:|\n";
    for p in &profiles {
        writeln!(
            body,
            "|{}|{}|{:.2}%|0|",
            p["name"].as_str().unwrap(),
            p["samples"],
            100. * p["borrowed_apply_period_fraction"].as_f64().unwrap()
        )
        .unwrap();
    }
    body+="\nEach recording has one sampled process. Complete raw/demangled symbol weights and byte-exact decoded streams are archived compressed. Raw perf files and four expanded loss streams stay locally with hashes. Sampling confirms execution in the intended caller; it does not prove a specific latency cause. No owned build, helper, hash, archive or other benchmark overlaps capture.\n\nAll 880 capture-workflow steps, seven native steps and two link steps finish successfully. The archive retains 889 receipts and 3,552 original streams byte-for-byte, plus four local hash-only loss streams, 868 observations and all 177 private source files. Reversing private edits recovers all 175 production originals; 170 are already exact. Closing verification checks all 769 pins and the accepted SDK hash. [Closeout](artifacts/closeout.json).\n\n## Inherited accepted CPU and Metal comparisons\n\nHistorical accepted local proof measurements versus supplied M5 references; **not remeasured in this caller experiment**. Accepted 1×2 peak RSS remains 134 MiB. The accepted full performance gate remains false, with its physical16 calibration engineering exception retained; its final artifact gate remains true.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n";
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
        let c = row["cpu_reference_ms"].as_f64().unwrap();
        let m = row["metal_reference_ms"].as_f64().unwrap();
        writeln!(
            body,
            "|{shape}|{t:.6}|{c:.0}|{:.3}×|{m:.0}|{:.3}×|{:.1}|",
            c / t,
            m / t,
            c / 10.
        )
        .unwrap();
    }
    body+="\nAccepted 54→1 remains 162.250464 ms, without a supplied M5 reference. This experiment makes zero SDK proof requests or new Gnark verifications; the independent ledger remains 18,076. No new AWS, Aeglos, mobile, TEE, huge-circuit, pressure, minimum-RAM or concurrent-server-throughput conclusion follows. The all-eight CPU 10× goal remains active and unmet (0/8). This exact shared-quotient caller integration is closed.\n";
    fs::write(Path::new(D).join("README.md"), body).unwrap();
    seal();
    println!("Archived {total_samples} profile samples; no adoption");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive|seal"),
    }
}

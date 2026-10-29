use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/row-quotient175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-row-quotient175";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn filehash(p: impl AsRef<Path>) -> String {
    hash(&fs::read(p).unwrap())
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path());
        }
    }
}
fn compress(v: &Value, name: &str, d: &Path) {
    let bytes = serde_json::to_vec(v).unwrap();
    let raw = r(name);
    fs::write(&raw, &bytes).unwrap();
    let to = d.join(format!("{name}.gz"));
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&raw)
        .stdout(Stdio::from(fs::File::create(&to).unwrap()))
        .status()
        .unwrap()
        .success());
    let out = Command::new("gzip").arg("-dc").arg(to).output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
}
fn seal() {
    let d = Path::new(D);
    let mut files = vec![];
    walk(d, &mut files);
    let mut sums = BTreeMap::new();
    for p in files {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if rel != "artifact-index.json" && rel != "SHA256SUMS" {
            sums.insert(rel, filehash(&p));
        }
    }
    save(d.join("artifact-index.json"), &json!(sums));
    sums.insert(
        "artifact-index.json".into(),
        filehash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        sums.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    for (p, h) in &sums {
        assert_eq!(filehash(d.join(p)), *h);
    }
    println!("Sealed{}hashes/{}files", sums.len(), sums.len() + 1);
}
fn report() {
    let a = read(r("analysis.json"));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(filehash(p), h.as_str().unwrap());
    }
    let sdk = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
    assert_eq!(
        filehash(sdk),
        "20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f"
    );
    let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let mut receipts = BTreeMap::new();
    let mut observations = vec![];
    let mut streams = 0;
    for e in fs::read_dir(r("runs")).unwrap() {
        let e = e.unwrap();
        if !e.file_type().unwrap().is_dir() {
            continue;
        }
        let name = e.file_name().into_string().unwrap();
        let mut files = BTreeMap::new();
        for file in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
            if file == "stdout.log" && name.starts_with("loss-") {
                continue;
            }
            let path = e.path().join(file);
            let text = fs::read_to_string(&path).unwrap();
            if file == "exit.json" {
                let v: Value = serde_json::from_str(&text).unwrap();
                assert_eq!(v["returncode"], 0);
                assert_eq!(v["success"], true);
            }
            if file == "stdout.log" {
                for l in text.lines().filter_map(|l| l.strip_prefix("RESULT ")) {
                    let v: Value = serde_json::from_str(l).unwrap();
                    observations.push(json!({"name":name,"result":v}));
                }
            }
            let v = json!({"sha256":hash(text.as_bytes()),"text":text});
            assert_eq!(v["sha256"], filehash(&path));
            assert_eq!(
                v["text"].as_str().unwrap().as_bytes(),
                fs::read(path).unwrap()
            );
            streams += 1;
            files.insert(file, v);
        }
        receipts.insert(name, files);
    }
    assert_eq!(receipts.len(), 1144);
    assert_eq!(streams, 4572);
    assert_eq!(observations.len(), 1132);
    save(r("observations.json"), &json!(observations));
    let mut fields = BTreeMap::new();
    for (p, h) in read(r("field-original-pins.json")).as_object().unwrap() {
        let bytes = fs::read(p).unwrap();
        let mut recovered = bytes.clone();
        if p.ends_with("ifma8.rs") {
            let suffix = b"\ninclude!(\"../../kernel.rs\");\ninclude!(\"../../test-field.rs\");\n";
            assert!(recovered.ends_with(suffix));
            recovered.truncate(recovered.len() - suffix.len());
        }
        assert_eq!(hash(&recovered), h.as_str().unwrap());
        fields.insert(Path::new(p).strip_prefix(R).unwrap().display().to_string(),json!({"text":String::from_utf8(bytes.clone()).unwrap(),"sha256":hash(&bytes),"recovered_sha256":h}));
    }
    assert_eq!(fields.len(), 17);
    let mut profiles = BTreeMap::new();
    let mut samples = 0;
    for (name, p) in a["profiles"].as_object().unwrap() {
        let mut covered = 0u64;
        let mut symbols = vec![];
        for (raw, weight) in p["symbols"].as_object().unwrap() {
            let first = raw.split_whitespace().next().unwrap();
            let demangled = rustc_demangle::try_demangle(first)
                .map(|n| format!("{n:#}"))
                .unwrap_or_else(|_| first.into());
            if demangled.starts_with("probe::chain") || demangled.starts_with("probe::fused") {
                covered += weight.as_u64().unwrap();
            }
            symbols.push(json!({"raw":raw,"demangled":demangled,"period":weight}));
        }
        let fraction = covered as f64 / p["period"].as_u64().unwrap() as f64;
        profiles.insert(name,json!({"samples":p["samples"],"lost":p["lost"],"vector_loop_period_fraction":fraction,"symbols":symbols}));
        samples += p["samples"].as_u64().unwrap();
    }
    let mut hot = BTreeMap::new();
    for build in ["frozen", "candidate"] {
        let records: Vec<_> = a["static_code"][build]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| {
                s["symbol"]
                    .as_str()
                    .unwrap()
                    .contains("zopro_bn254::FqConfig, true,")
            })
            .cloned()
            .collect();
        hot.insert(build, records);
        let asm = fs::read_to_string(r(format!("{build}-assembly.txt"))).unwrap();
        let mut selected = String::new();
        let mut keep = false;
        for line in asm.lines() {
            if line.ends_with(">:") {
                keep = line.contains("zopro_bn254::FqConfig, true,")
                    && ["chain", "streams", "fused"]
                        .iter()
                        .any(|n| line.contains(&format!("<probe::{n}::")));
            }
            if keep {
                writeln!(selected, "{line}").unwrap();
            }
        }
        assert!(!selected.is_empty());
        fs::write(r(format!("{build}-selected.s")), selected).unwrap();
    }
    save(
        r("profile-review.json"),
        &json!({"samples":samples,"profiles":profiles,"raw_v0_demangled":true,"limits":"Wholeprocess capture includes setup/prime/scalar oracle; function loop coverage uses defining probe::chain/probe::fused. Small sample counts descriptive; no causal stall or exclusive CPU claim."}),
    );
    save(r("hot-code.json"), &json!(hot));
    let review = json!({"status":"complete","gate_passed":false,"gain_aggregate_passed_both_controls":true,"nonregression_failed":a["nonregression_failures"],"model_cases":60168,"row_quotient_checks":300840,"separate_carry_packing_cases":16412,"native_Arkworks_plus_exact_raw_lane_comparisons":345088,"native_new_tests":2,"unchanged_field_tests_inherited":33,"old_test_vectors_applied_to_new_kernel":true,"workflow_steps":1144,"receipts":receipts.len(),"encoded_streams_exact":streams,"observed_results":observations.len(),"pins_closed":pins.as_object().unwrap().len(),"field_source17_recovery_exact":true,"production175_unchanged":true,"accepted_SDK_sha256":filehash(sdk),"profile_samples":samples,"SDK_requests":0,"new_proofs":0,"proof_ledger":18076,"adopted":false,"goal_achieved":false,"paper_scope":"2023/604 primary indexed excerpt only; directPDF403,notfullpaperread; no upstream code adopted","decision":"No blanket replacement or posthoc subset promotion from failed declared nonregression gate; positive FqR256 aggregate and fused cases retained"});
    save(r("postcapture-review.json"), &review);
    let d = Path::new(D).join("artifacts");
    assert!(!Path::new(D).exists());
    fs::create_dir_all(&d).unwrap();
    for f in [
        "setup.rs",
        "setup.log",
        "design.md",
        "bounds.md",
        "history-audit.md",
        "make-control.rs",
        "control.rs",
        "report.rs",
        "field-original-pins.json",
        "kernel.rs",
        "test-field.rs",
        "src/bin/model.rs",
        "src/bin/row-audit.rs",
        "src/bin/probe.rs",
        "Cargo.toml",
        "Cargo.lock",
        "model.json",
        "model.stderr.log",
        "row-audit.json",
        "row-audit-build.log",
        "native-tests.log",
        "native-build.log",
        "frozen-build.log",
        "prepare.log",
        "finish.log",
        "pins.json",
        "declaration.json",
        "run-plan.json",
        "runs/result.json",
        "analysis.json",
        "observations.json",
        "hot-code.json",
        "profile-review.json",
        "postcapture-review.json",
        "frozen-selected.s",
        "candidate-selected.s",
    ] {
        let to = d.join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(r(f), &to).unwrap();
        assert_eq!(filehash(r(f)), filehash(to));
    }
    compress(&json!(receipts), "run-receipts.json", &d);
    compress(&json!(fields), "field-snapshot.json", &d);
    let binaries: BTreeMap<_, _> = [
        "setup",
        "control",
        "frozen-probe",
        "target/release/probe",
        "target/release/model",
        "target/release/row-audit",
    ]
    .into_iter()
    .map(|f| (f, filehash(r(f))))
    .collect();
    save(d.join("local-binary-pins.json"), &json!(binaries));
    let agg = &a["aggregates"][1];
    let mut md = String::from("# Row-local Montgomery quotient preparation\n\n");
    writeln!(md,"The intended Fq/R256 three-context aggregate improves **{:.3}% against fresh control**, ratio {:.9}, interval [{:.9}, {:.9}], **{}/31 wins**. The full component gate is **false**: both fields' R260 four-stream nonregression checks fail against both controls. No blanket replacement, SDK integration or production adoption is selected.\n",100.*(1.-agg["median"].as_f64().unwrap()),agg["median"].as_f64().unwrap(),agg["low"].as_f64().unwrap(),agg["high"].as_f64().unwrap(),agg["wins"]).unwrap();
    md+="## Arithmetic\n\nAt each CIOS row, distributivity rewrites the cancellation digit using the old low coefficient and precomputed current-operation input contributions. With B=2^52 and I=−p⁻¹ modB, prepare a0I and c0I, then five values w_i=lo(a0I·b_i)+lo(c0I·d_i) in fused mode. The row digit is lo(I·t_old0)+w_i modulo B (or 2^48 in the final mixed row). It no longer waits for the current raw product row. It still depends on the preceding row's state.\n\nThe original input products, cancelled-low carry, modulus rows, final packing, normalization and fused correction are unchanged. This uses only the existing one-word inverse, unlike the [paired](../2026-10-28-paired-cancel/README.md) and [three-row](../2026-10-29-triple-cancel175/README.md) quotient-prefix methods. Ordinary/fused templates use **105/161 IFMAs versus 100/150**, including per-operation preparation and the zero-state first-row simplification. Five additional row vectors and prepared low operands can increase register pressure. There is no key-dependent table, retained cache, heap buffer, point copy or representation change. [Derivation and bounds](artifacts/bounds.md).\n\nThe primary indexed excerpt of [the NTRU Prime AVX2 technical report](https://eprint.iacr.org/2023/604.pdf), section 2.1, supplies background for placing an inverse-mod-word factor on a known operand. The direct PDF fetch returned 403 and the full paper was not read. This is an independently derived multirow 52/48-bit adaptation; no upstream code or paper speedup is adopted. The [history audit](artifacts/history-audit.md) records overlapping MSM ideas already tested elsewhere.\n\n## Fixed component results\n\nRyzen 9950X, one thread on CPU 2. Thirty-one fixed rotated/reversed blocks compare ordinary frozen code built before the includes, fresh ordinary and the candidate sharing one executable. Each process primes three times for 2,000 iterations and measures 100,000 iterations. Input/output conversions and independent scalar checks are outside the timer. Per-operation preparation is in the source call; invariant work in these fixed-operand patterns may be hoisted by the compiler. This is not an actual bucket caller or full SDK benchmark. No owned build/helper/hash/archive/second workload overlaps capture.\n\nValues are nanoseconds per field-lane update, divided by four again for independent streams. Paired changes use within-block ratios and need not equal ratios of displayed medians. Positive means slower.\n\n|Case|Frozen|Fresh|Candidate|Paired change vs fresh|Wins/fresh|\n|---|---:|---:|---:|---:|---:|\n";
    for row in a["rows"].as_array().unwrap() {
        writeln!(
            md,
            "|{}|{:.6}|{:.6}|{:.6}|{:+.3}%|{}/31|",
            row["case"].as_str().unwrap(),
            row["median_ns"]["frozen"].as_f64().unwrap(),
            row["median_ns"]["fresh"].as_f64().unwrap(),
            row["median_ns"]["rowq"].as_f64().unwrap(),
            100. * (row["rowq_ratios"]["fresh"]["median"].as_f64().unwrap() - 1.),
            row["rowq_ratios"]["fresh"]["wins"]
        )
        .unwrap();
    }
    md+="\nAll four fused contexts improve against both controls with 31/31 wins. Fq/R256 fused improves 3.773% against fresh, while Fq/R256 four-stream ordinary arithmetic regresses 1.385%, also visible rather than discarded. Both R260 four-stream cases lose all blocks; their upper bounds exceed 1.03. The aggregate gain does not override those declared failures.\n\n|Fq/R256 aggregate against|Median|Interval|Wins|\n|---|---:|---|---:|\n";
    for (i, label) in ["frozen", "fresh"].into_iter().enumerate() {
        let v = &a["aggregates"][i];
        writeln!(
            md,
            "|{label}|{:.9}|[{:.9}, {:.9}]|{}/31|",
            v["median"].as_f64().unwrap(),
            v["low"].as_f64().unwrap(),
            v["high"].as_f64().unwrap(),
            v["wins"]
        )
        .unwrap();
    }
    writeln!(md,"\nCalibration failures: `{}`. Nonregression failures: `{}`. Both aggregate gain checks pass. The full gate requires all of these checks; it remains false.\n",a["calibration_failures"],a["nonregression_failures"]).unwrap();
    md+="The fixed gate requires each fresh/frozen interval within factor 1.03, candidate upper≤1.03 against both controls in all twelve cases, and the intended Fq/R256 three-case geometric aggregate upper<1 with at least 21/31 wins against both. Individual ranks 10–22 of 31 have nominal 97.055% conditional coverage under exchangeable independent blocks, without familywise guarantees. No gain floor, exclusions, extensions, successful reruns, threshold changes, flag/geometry sweeps or subset promotion occur.\n\n## Validation and profiles\n\nThe exact BigUint model passes **60,168 products and 300,840 cancellation-digit comparisons** across both primes and radices, ordinary/fused products, inclusive2p edges, full-width random tuples and changing chains. Each raw input-row integer and cancellation digit agrees; exact final REDC and lazy correction agree. A separate **16,412-case redundant-coefficient audit** validates cancelled-low carry and postrow52/mixed48 packing identities. It uses different cases and does not repeat the successful full model. All new quotient-only partials stay below3B; the copied coefficient envelope stays below2^61.\n\nTwo new native tests pass **345,088 Arkworks lane comparisons plus exact lazy-digit equality** to ordinary CIOS, including eight-step chains, both primes/radices and inclusive2p values. These reuse the earlier oracle case families on the new kernel; they are not new distinct mathematical vectors. The 33 unchanged native field tests are filtered and inherited. All17 original field files recover exactly after removing the two includes;16 are byte-exact throughout. All175 production sources and accepted SDK20fde remain exact. No native build, model, test or successful capture fails or is repeated in this experiment.\n\nEvery one of the 1,116 latency processes,12preflights and four profiles independently checks32canonical outputs outside its timer. Chain/fused duplicate eight final states into four slots; only streams4 has32independent final states. All corresponding digests agree. There is no timed heap allocation in these fixed-array loops.\n\nFour whole-process199Hz cycles:u profiles follow all latency,30million iterations each (Fq/R256 chain/fused, fresh/candidate). Setup, primes and scalar checking are included in those recordings. All profiles finish before ownership changes and decoding; perf decodes as the new file owner. No privileged-input ownership failure or recapture occurs.\n\n|Profile|Samples|Vector-loop period fraction|Lost|\n|---|---:|---:|---:|\n";
    for (name, v) in &profiles {
        writeln!(
            md,
            "|{name}|{}|{:.2}%|{}|",
            v["samples"],
            100. * v["vector_loop_period_fraction"].as_f64().unwrap(),
            v["lost"]
        )
        .unwrap();
    }
    writeln!(md,"\nTotal {samples} samples, one PID each, zero lost. Rust v0 symbols are demangled before loop attribution. Small counts are descriptive, not an exclusive stall cause or removable proof-time estimate. Whole-function instruction/IFMA/RSP sites include setup and cold paths; stack operands are not measured spills or peak RSS. [Linked code](artifacts/hot-code.json) and selected disassembly are retained.\n").unwrap();
    md+="All1,144workflow steps complete;291closing pins pass. The compact archive encodes1,144command receipts and4,572original command/exit/stdout/stderr streams (four expanded loss stdout dumps remain local with hashes),1,132RESULT observations and17private field-source members. Every encoded member and compressed stream is checked byte-for-byte. ELF/library artifacts and raw perf data remain local with SHA identities. Zero SDK requests or new proof exports occur; the independent proof ledger remains**18,076**. No accepted latency/RSS, AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/minRAM, concurrency or throughput result follows.\n\n## Inherited CPU and Metal comparison\n\nAccepted local measurements against supplied historical M5 references; not newly measured here. Primary peak remains134MiB. The original full-performance-gate failure and physical16 calibration engineering exception remain.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n";
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
        let v = &manifest["m5"][shape];
        writeln!(
            md,
            "|{shape}|{:.6}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap(),
            v["required_ms_for_CPU10x"].as_f64().unwrap()
        )
        .unwrap();
    }
    md+="\nThe all-eight CPU10× goal remains active/unmet(0/8). Accepted54→1 remains162.250464ms without a supplied M5 reference.\n\n[Design](artifacts/design.md), [bounds](artifacts/bounds.md), [full results](artifacts/analysis.json), [observations](artifacts/observations.json), [source/tool pins](artifacts/pins.json), [review](artifacts/postcapture-review.json).\n";
    fs::write(Path::new(D).join("README.md"), md).unwrap();
    seal();
    println!("Archived1144receipts/4572streams/1132observations; gainpasses/nonregfails; ledger18076unchanged");
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("seal") {
        seal()
    } else {
        report()
    }
}

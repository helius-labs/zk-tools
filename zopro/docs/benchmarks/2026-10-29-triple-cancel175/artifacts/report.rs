use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-cancel175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-triple-cancel175";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["gate_passed"], false);
    let m=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let mut receipts = BTreeMap::new();
    let mut observations = Vec::new();
    for e in fs::read_dir(format!("{R}/runs")).unwrap() {
        let e = e.unwrap();
        if !e.file_type().unwrap().is_dir() {
            continue;
        }
        let name = e.file_name().into_string().unwrap();
        let mut files = BTreeMap::new();
        for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
            let path = e.path().join(f);
            if f == "stdout.log" && name.starts_with("loss-") {
                continue;
            }
            let text = fs::read_to_string(&path).unwrap();
            if f == "exit.json" {
                let exit: Value = serde_json::from_str(&text).unwrap();
                assert_eq!(exit["returncode"], 0);
                assert_eq!(exit["success"], true);
            }
            if f == "stdout.log" {
                for line in text.lines().filter_map(|l| l.strip_prefix("RESULT ")) {
                    let v: Value = serde_json::from_str(line).unwrap();
                    observations.push(json!({"name":name,"result":v}));
                }
            }
            files.insert(f, json!({"sha256":hash(path),"text":text}));
        }
        receipts.insert(name, files);
    }
    assert_eq!(receipts.len(), 1144);
    assert_eq!(observations.len(), 1132);
    save(format!("{R}/run-receipts.json"), &json!(receipts));
    save(format!("{R}/observations.json"), &json!(observations));
    let mut code = BTreeMap::new();
    for build in ["frozen", "candidate"] {
        let asm = fs::read_to_string(format!("{R}/{build}-assembly.txt")).unwrap();
        let mut keep = false;
        let mut selected = String::new();
        for line in asm.lines() {
            if line.ends_with(">:") {
                keep = ["chain", "streams", "fused"]
                    .iter()
                    .any(|p| line.contains(&format!("<probe::{p}::<zopro_bn254::FqConfig, true,")));
            }
            if keep {
                writeln!(selected, "{line}").unwrap();
            }
        }
        fs::write(format!("{R}/{build}-selected.s"), selected).unwrap();
        let records: Vec<_> = a["static_code"][build]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| {
                ["chain", "streams", "fused"].iter().any(|p| {
                    v["symbol"]
                        .as_str()
                        .unwrap()
                        .contains(&format!("<probe::{p}::<zopro_bn254::FqConfig, true,"))
                })
            })
            .map(|v| {
                let mut v = v.clone();
                let frames: Vec<_> = v["stack_subtractions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|l| {
                        l.as_str()
                            .unwrap()
                            .split_once(':')
                            .unwrap()
                            .1
                            .trim()
                            .starts_with("sub ")
                    })
                    .cloned()
                    .collect();
                v.as_object_mut().unwrap().remove("stack_subtractions");
                v["literal_stack_reservations"] = json!(frames);
                v
            })
            .collect();
        code.insert(build, records);
    }
    save(
        format!("{R}/hot-code.json"),
        &json!({"records":code,"full_assembly_local_hashes":{"frozen":hash(format!("{R}/frozen-assembly.txt")),"candidate":hash(format!("{R}/candidate-assembly.txt"))},"limits":"Original stack_subtractions field includes vector subtracts with RSP; only scalar sub immediate,RSP is a literal reservation. RSP lines are not measured spills. Whole-function static sites include setup and cold paths, not dynamic instruction counts."}),
    );
    let review = json!({"workflow_steps":1144,"all_steps_complete":true,"latency_processes":1116,"preflight_processes":12,"profiles":4,"captured_source_unchanged":true,"native_initial_failure":"One missing macro-arm closing brace prevented both first test and first fresh-probe builds. No native execution/timing happened; source/logs retained. One-character repair applied to prefix.rs/kernel.rs, followed by successful tests and build. No successful tests/captures repeated.","draft_model":"Unexecuted/uncompiled prefix-only draft is retained and explicitly labeled; complete model includes actual final48/52packing and exactfullREDC equality.","API_scope":"Private prototype only, original generic arithmetic unchanged; no production dispatch or code adoption","SDK_requests":0,"new_exports":0,"ledger":17811,"goal_achieved":false});
    save(format!("{R}/postcapture-review.json"), &review);
    let mut md=String::from("# Three-row Montgomery quotient prefix\n\n**Rejected at the component gate; no SDK integration or adoption.** The intended Fq/R256 aggregate is 0.795% slower than fresh ordinary CIOS, with 0/31 winning blocks. Control calibration and all nonregression checks pass, but neither aggregate gain check passes. Production remains unchanged.\n\n## Change\n\nThe private eight-lane kernel forms three input product rows before cancellation, then computes three low quotient digits using a 156-bit negative inverse. Their calculation includes the carry from the second inverse-product coefficient. Once those digits are known, the original cancelled-low identity applies them at three offsets. The original last two CIOS rows, mixed 48-bit packing, final normalization and fused correction remain unchanged. [Algebra and bounds](artifacts/bounds.md).\n\nThis differs from the [two-digit prefix](../2026-10-28-paired-cancel/README.md), the earlier [full-width truncated quotient](../2026-10-24-truncated-redc-component/README.md), and [FIPS scanning](../2026-10-29-fq-fips175/README.md). Those results remain separate; this is not a rerun of their implementations. The template uses 106/156 ordinary/fused IFMAs, versus accepted 100/150 and paired 104/154. It has eight temporary coefficients in the prefix and returns to six for the remaining rows. Two extra inverse words cost 16 logical constant bytes per field; there is no heap, key table, persistent cache, new representation or copied point array.\n\n## Fixed component measurements\n\nRyzen 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare frozen ordinary code built before the private includes, fresh ordinary and the new prefix in one fresh executable. Each process runs three 2,000-iteration primes and 100,000 measured iterations. Conversions and scalar output checks are outside the timer. No owned build/helper/hash/archive/second workload overlaps captures.\n\nUnits are ns per field-lane update, additionally divided by four for independent streams. Paired change is the median within-block new/fresh ratio; positive means slower, and need not equal a ratio of displayed medians.\n\n|Case|Frozen|Fresh|Three-row prefix|Paired change|Wins/fresh|\n|---|---:|---:|---:|---:|---:|\n");
    for row in a["rows"].as_array().unwrap() {
        writeln!(
            md,
            "|{}|{:.6}|{:.6}|{:.6}|{:+.3}%|{}/31|",
            row["case"].as_str().unwrap(),
            row["median_ns"]["frozen"].as_f64().unwrap(),
            row["median_ns"]["fresh"].as_f64().unwrap(),
            row["median_ns"]["triple"].as_f64().unwrap(),
            100. * (row["triple_ratios"]["fresh"]["median"].as_f64().unwrap() - 1.),
            row["triple_ratios"]["fresh"]["wins"]
        )
        .unwrap();
    }
    md.push_str("\nThe Fq/R256 aggregate is 1.007946 [1.006634, 1.009716] versus fresh and 1.007801 [1.006309, 1.009604] versus frozen, both 0/31 wins. Ordinary mixed chains span parity, independent streams regress about 2.1%, and fused chains regress about 0.39% versus fresh.\n\nPositive R260 estimates are retained. Fq/R260 streams improve 0.575% versus fresh [ratio0.990039–0.996290,29/31wins] and0.535% versus frozen [0.993310–0.998938,25/31]. Fr/R260 chains improve0.406% versus fresh [0.994781–0.997780,25/31] and0.326% versus frozen [0.993225–0.999621,22/31]. These are context-specific component observations, not a gain in the intended mixed-R256 arithmetic or complete prover. No subset is promoted after seeing results.\n\nThe predeclared gate requires the three Fq/R256 patterns' geometric aggregate upper ratio below1 and at least21/31wins against both controls, all12cases' upper nonregression bounds≤1.03 against both controls, and each fresh/frozen calibration within factor1.03. There is no minimum gain floor. Individual order ranks10–22of31 have nominal97.055% conditional coverage under exchangeable independent blocks, without a familywise guarantee. No observations are excluded, extended or rerun, and no threshold/flag/layout rescue is selected.\n\n## Correctness and linked code\n\nThe exact interpreter passes **60,168** full reductions across Fq/Fr, R256/R260, ordinary/fused, inclusive2p corners, fixed random tuples and changing chains. Every three-digit quotient and post-prefix integer matches an independent BigUint modular-inverse calculation; mixed final packing and full exact REDC representatives also match. All modeled coefficients stay below2^61. The unexecuted initial prefix-only draft remains labeled separately.\n\nTwo new native tests pass **345,088** field-lane comparisons against Arkworks, plus exact lazy-digit equality against ordinary CIOS. Compile-time bit-lifted inverse constants match the independently generated three-word fixtures. Native tests cover both primes/radices, ordinary/fused products, lazy boundaries and eight-step chains. The33unchanged field tests are filtered out and inherited. An initial missing macro brace prevented native test and fresh-probe compilation; the failed sources/logs remain. The one-character wrapper repair preceded all successful native execution and timing.\n\nAll1,116latency processes,12preflights and four profiles check32returned canonical fields against the independent scalar affine-power formula after timing. Chain/fused duplicate eight final states into four slots, not32independentchains. All corresponding digests agree. There is no timed heap allocation in these fixed-array arithmetic loops.\n\nActual linked Fq/R256 loops confirm the expected IFMAs:\n\n|Loop|Fresh/new IFMA sites|Fresh/new instruction sites|Fresh/new RSP operand lines|Fresh/new literal stack reservation bytes|\n|---|---:|---:|---:|---:|\n|Dependent chain|100/106|307/331|42/58|384/704|\n|Fused chain|150/156|344/407|44/84|448/1536|\n\nThese are static whole-function observations, not dynamic instructions, measured spills or process RAM. The longer live prefix comes with larger generated frames and more stack-reference sites. It does not establish an exclusive cause for the measured differences. [Hot-code audit](artifacts/hot-code.json) filters literal scalar stack reservations; the original broad `stack_subtractions` observer field also included vector subtracts.\n\n## Profiles and provenance\n\nFour separate whole-process cycles:u recordings follow all latency, at199Hz for30,000,000iterations (Fq/R256 chain/fused, fresh/new). They include setup, primes and output checks. All recordings finish before decoding.\n\n|Profile|Samples|Lost|\n|---|---:|---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        writeln!(md, "|{name}|{}|{}|", p["samples"], p["lost"]).unwrap();
    }
    md.push_str("\nAll619samples belong to one PID per recording with zero lost records; matching arithmetic loops lead sampled period. Small profile counts are descriptive, not an exclusive stall diagnosis or removable proof-time estimate. Original perf recordings and expanded loss dumps remain local with hashes.\n\nAll1,144workflow steps complete and289pins close. Removing two includes from the private eight-lane module recovers all17originalfieldfiles; the other16are exact. All175production sources remain unchanged. Original CIOS bodies and probe/oracle source remain exact; the appended helper replaces only the prefix of its separately named method. No core/prover rebuild or new verifier run occurs.\n\n[All observed results](artifacts/observations.json) and [encoded run receipts](artifacts/run-receipts.json) retain original command/exit/stdout/stderr text with SHA256. Initial failed-build sources and logs are retained. ZeroSDKrequests or proof exports occur; independent proof ledger remains**17,811**. No accepted latency/RSS, AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/minRAM, concurrency or throughput result follows.\n\n## Inherited CPU and Metal comparison\n\nAccepted local timings against supplied historical M5 references; not newly measured here. Primary peak RSS remains134MiB. The prior full-performance-gate failure and16-worker baseline-calibration engineering exception remain.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
        let v = &m["m5"][shape];
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
    md.push_str("\nAll-eight CPU10× goal remains active/unmet (0/8). Accepted54→1 remains162.250464ms without a supplied M5 reference.\n\n[Design](artifacts/design.md), [bounds](artifacts/bounds.md), [history](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [source/tool pins](artifacts/pins.json), [artifact hashes](artifact-index.json).\n");
    let d = Path::new(D);
    fs::create_dir_all(d.join("artifacts")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let files = [
        "setup.rs",
        "setup.log",
        "field-original-pins.json",
        "prefix.rs",
        "prefix-initial.rs",
        "kernel.rs",
        "kernel-initial.rs",
        "test-field.rs",
        "repair.rs",
        "model.rs",
        "model-draft-prefix-only.rs",
        "model.json",
        "model-build.log",
        "model.stderr.log",
        "design.md",
        "bounds.md",
        "history-audit.md",
        "Cargo.toml",
        "Cargo.lock",
        "src/bin/probe.rs",
        "native-tests-initial.log",
        "native-build-initial.log",
        "native-tests.log",
        "native-build.log",
        "frozen-build.log",
        "make-control.rs",
        "clean-control.rs",
        "control.rs",
        "control-build.log",
        "pins.json",
        "declaration.json",
        "run-plan.json",
        "prepare.log",
        "finish.log",
        "analysis.json",
        "observations.json",
        "run-receipts.json",
        "postcapture-review.json",
        "hot-code.json",
        "frozen-selected.s",
        "candidate-selected.s",
        "runs/result.json",
        "report.rs",
        "report-build.log",
    ];
    let mut index = BTreeMap::new();
    for f in files {
        let p = Path::new(R).join(f);
        let q = d.join("artifacts").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        assert_eq!(hash(&p), hash(&q));
        index.insert(format!("artifacts/{f}"), hash(q));
    }
    save(d.join("artifact-index.json"), &json!(index));
    index.insert("README.md".into(), hash(d.join("README.md")));
    index.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    let mut sums = String::new();
    for (f, h) in &index {
        writeln!(sums, "{h}  {f}").unwrap();
    }
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!(
        "Archived{}hashes/{}files;gatefalse;ledger17811unchanged",
        index.len(),
        index.len() + 1
    );
}

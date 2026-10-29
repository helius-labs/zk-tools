use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-product-sub175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-g2-product-sub175";
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
    let mut receipts = BTreeMap::new();
    let mut raw = BTreeMap::new();
    for e in fs::read_dir(format!("{R}/runs")).unwrap() {
        let e = e.unwrap();
        if !e.file_type().unwrap().is_dir() {
            continue;
        }
        let name = e.file_name().into_string().unwrap();
        let mut files = BTreeMap::new();
        for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
            let p = e.path().join(f);
            assert!(p.is_file());
            if name.starts_with("loss-") && f == "stdout.log" {
                raw.insert(p.display().to_string(), hash(&p));
                continue;
            }
            let text = fs::read_to_string(&p).unwrap();
            if f == "exit.json" {
                let exit: Value = serde_json::from_str(&text).unwrap();
                assert_eq!(exit["success"], true);
                assert_eq!(exit["returncode"], 0);
            }
            files.insert(f, json!({"sha256":hash(&p),"text":text}));
        }
        receipts.insert(name, files);
    }
    assert_eq!(receipts.len(), 392);
    save(format!("{R}/run-receipts.json"), &json!(receipts));
    save(format!("{R}/local-only-sha256.json"), &json!(raw));
    let mut code = BTreeMap::new();
    for build in ["frozen", "candidate"] {
        let asm = fs::read_to_string(format!("{R}/{build}-assembly.txt")).unwrap();
        let mut name = String::new();
        let mut lines = Vec::new();
        let mut blocks = BTreeMap::new();
        for line in asm.lines() {
            if line.ends_with(">:") {
                if ["chain", "streams", "affine"]
                    .iter()
                    .any(|s| name.contains(&format!("<probe::{s}::<")))
                {
                    blocks.insert(name.clone(), lines.clone());
                }
                name = line.into();
                lines.clear();
            } else {
                lines.push(line.to_owned());
            }
        }
        let mut records = Vec::new();
        let mut selected = String::new();
        for (name, lines) in blocks {
            writeln!(selected, "{name}").unwrap();
            for l in &lines {
                writeln!(selected, "{l}").unwrap();
            }
            let instructions: Vec<_> = lines
                .iter()
                .filter_map(|l| l.split_once(':').map(|(_, b)| b.trim()))
                .filter(|s| !s.is_empty())
                .collect();
            records.push(json!({"symbol":name,"instruction_sites":instructions.len(),"IFMA_sites":instructions.iter().filter(|l|l.contains("vpmadd52")).count(),"RSP_operand_lines":instructions.iter().filter(|l|l.contains("%rsp")).count(),"literal_stack_reservations":instructions.iter().filter(|l|l.starts_with("sub ")&&l.contains("%rsp")).collect::<Vec<_>>(),"branches_and_calls":instructions.iter().filter(|l|l.starts_with('j')||l.starts_with("call")).count()}));
        }
        fs::write(format!("{R}/{build}-selected.s"), selected).unwrap();
        code.insert(build, records);
    }
    save(
        format!("{R}/hot-code.json"),
        &json!({"records":code,"whole_disassembly_local_hashes":{"frozen":hash(format!("{R}/frozen-assembly.txt")),"candidate":hash(format!("{R}/candidate-assembly.txt"))},"limits":"Static whole function sites include loops, alternatives and cold correction code. Chain also contains the scaled alternative. Counts are not dynamic instructions, spills or stack/RSS bounds."}),
    );
    let mut profile_summary = BTreeMap::new();
    for (name, p) in a["profiles"].as_object().unwrap() {
        let total = p["symbols"]
            .as_object()
            .unwrap()
            .values()
            .map(|v| v.as_u64().unwrap())
            .sum::<u64>();
        let loop_period = p["symbols"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(s, _)| s.contains("5chain") || s.contains("6affine"))
            .map(|(_, v)| v.as_u64().unwrap())
            .sum::<u64>();
        profile_summary.insert(name,json!({"samples":p["samples"],"loop_period_fraction":loop_period as f64/total as f64,"total_period":total,"lost":p["lost"]}));
    }
    save(format!("{R}/profile-summary.json"), &json!(profile_summary));
    let review = json!({"component_gate":false,"all392steps_successful":true,"captured_source_unchanged":true,"original175_production_unchanged":true,"source_recovery":"Remove two private include lines from ifma8.rs; other16fieldfiles exact","prototype_API_preconditions":"OnlyMIXED=true,FUSED=true,modulus<2^254 and canonical anchor used/qualified; helpers do not implement public unsupported dispatch","declaration_wording_correction":"all12caseupper in declaration denotes an erroneous count label; executable gate and pre-capture design apply8candidate/control nonregression and4fresh/frozen calibration checks. No threshold/scope/result changed.","initial_model_build":"BigUint shift type ambiguity repaired before first model execution; initial source/log retained. Also corrected min accumulator initialization before first successful model execution.","profiles":"Wholeprocess includes scalar oracle. Affine dominated by run() scalar verification; vectorloop weighted fractions retained, no exclusive microarchitecture diagnosis.","source_comment":"Currentlanes_mul comment saysKaratsuba but native branch is2fused components; comments alone not used asworkcounts.","SDK_requests":0,"exports":0,"ledger":17811,"adopted":false,"goal_achieved":false});
    save(format!("{R}/postcapture-review.json"), &review);
    let mut md=String::from("# G2 fused product components minus canonical anchors\n\n**Rejected at the arithmetic-component gate; no bucket-caller or SDK integration.** The affine expression is 6.653% slower against fresh ordinary arithmetic. The four-workload aggregate is 17.496% slower, with 0/31 winning blocks. Every workload regresses; control calibration passes. Production remains unchanged.\n\n## Arithmetic and scope\n\nThe current G2 reverse pass computes a complex product, then subtracts a canonical Y anchor. The private native eight-lane helper preserves the fused CIOS product/cancellation/mixed-radix prefix. It folds `p − anchor` into the final signed carry pass of each real/imaginary component, then uses the original conditional 2p correction. With raw fused result T<3p and anchor<p, the new total satisfies 0<T+p−anchor<4p, so one correction restores normalized output below2p. Canonical values match; lazy representatives may differ. [Bounds](artifacts/bounds.md) specify the native mixed-R256 preconditions.\n\nThe earlier [G1 signed-final-subtraction study](../2026-10-28-signed-final-sub174/README.md) and [failed SDK trial](../2026-10-28-signed-final-sub-sdk174/README.md) remain separate. This tests the two-product G2 range, bias and correction rather than repeating their ordinary-product helper. No new representation, key table, buffer, persistent cache, point copy or heap allocation is added. No source-level operation saving is treated as a speed prediction.\n\n## Fixed component results\n\nRyzen 9950X, one thread pinned to CPU 2. Thirty-one fixed rotated/reversed blocks compare frozen ordinary code built before the includes, fresh ordinary and candidate methods sharing one new executable. Each process runs three 2,000-iteration primes and 100,000 measured iterations. Input/output conversion and independent scalar checking are outside the timer. No owned build/helper/hash/archive/second workload overlaps capture.\n\nThe affine workload spells out the current native G2 denominator, numerator, real scaling, square and coordinate expressions, changing state after each iteration. It uses arbitrary fields and a supplied factor: it omits inversion, batch construction, bucket collisions and scheduling, and is **not a valid-curve loop or actual borrowed-G2 caller**. Product-sub chains and streams use canonical constant anchors.\n\nValues are ns per complex field element, additionally divided by four for independent streams. Percentage change is the median paired per-block candidate/fresh ratio; positive means slower.\n\n|Workload|Frozen|Fresh|Candidate|Paired change|Wins/fresh|\n|---|---:|---:|---:|---:|---:|\n");
    for row in a["rows"].as_array().unwrap() {
        writeln!(
            md,
            "|{}|{:.6}|{:.6}|{:.6}|+{:.3}%|{}/31|",
            row["pattern"].as_str().unwrap(),
            row["median_ns"]["frozen"].as_f64().unwrap(),
            row["median_ns"]["fresh"].as_f64().unwrap(),
            row["median_ns"]["candidate"].as_f64().unwrap(),
            100. * (row["paired"]["fresh"]["median"].as_f64().unwrap() - 1.),
            row["paired"]["fresh"]["wins"]
        )
        .unwrap();
    }
    md.push_str("\nCandidate/fresh aggregate is1.174963 [1.174181,1.175648],0/31wins; candidate/frozen is1.175141 [1.174821,1.175606],0/31. Affine/fresh is1.066529 [1.065332,1.068204]. All eight candidate/control nonregression checks and all four gain checks fail. The declared requirement was improving affine and equal-weight aggregate upper bounds with at least21/31wins against both controls, every context upper≤1.03, and each fresh/frozen calibration within factor1.03. The declaration's `all12caseupper` count label was mistaken; the pre-capture design and executable distinguish eight nonregression and four calibration checks. No gate changed.\n\nIntervals use ranks10–22of31, nominal97.055% conditional on exchangeable independent blocks, without a familywise guarantee. All372latency observations and four preflights remain. No exclusions, sample extensions, threshold changes or successful reruns occur. This exact recipe is closed without a subset or layout rescue.\n\n## Correctness and profiles\n\nTwo new field tests pass **172,544** changing-chain lane comparisons against Arkworks across Fq/Fr, fused sums/differences, lazy and inclusive2p multiplicands, canonical anchors and normalized output bounds. The33unchanged native field tests are filtered out and inherited. The independent BigUint model passes **21,568** exact mixed-R256 fused REDC cases and **8,324** wide unnormalized signed carry cases. An initial ambiguous shift type was repaired before its first successful execution; the initial source/log remain.\n\nEvery process checks32canonicalFq2outputs outside timing. The first three workloads use a scalar closed-form affine-power oracle; affine uses repeated independent scalarFq2 math. Chain/scaled/affine duplicate8finalstates into four slots, so32outputs do not mean32independentstates. All corresponding final digests agree.\n\nFour whole-process cycles:u profiles follow all latency: chain/affine fresh/candidate,1,000,000iterations at199Hz. They include setup/primes/scalar verification; every profile result passes its oracle.\n\n|Profile|Samples|Vector-loop period fraction|Lost|\n|---|---:|---:|---:|\n");
    for (name, p) in &profile_summary {
        writeln!(
            md,
            "|{name}|{}|{:.3}%|{}|",
            p["samples"],
            100. * p["loop_period_fraction"].as_f64().unwrap(),
            p["lost"]
        )
        .unwrap();
    }
    md.push_str("\nAffine profiles are dominated by the intentionally untimed scalar oracle in `run()`. Their total samples cannot be attributed to the vector arithmetic. Short chain recordings are descriptive and not resampled. All four profiles have one PID and zero lost records. [Hot code](artifacts/hot-code.json) and selected linked disassembly preserve both paths; whole-function instruction/stack sites include alternatives and cold corrections, not dynamic costs or measured spills. No exclusive dependency, branch, cache or spill cause is established.\n\n## Provenance and retained evidence\n\nAll392workflow steps complete;288source/tool/library/binary pins close. The175production source hashes remain exact. Removing two include lines from the private eight-lane field module recovers all17originalfieldfiles; other16stay exact. The private methods are qualified only for the mixed-R256 fused call and canonical anchor preconditions, not as a public generic dispatch. No original prover/core suite or verifier is repeated.\n\n[All observations](artifacts/observations.json) retain every result. [Run receipts](artifacts/run-receipts.json) encode original command/exit/stdout/stderr bytes as UTF-8 text with SHA256, including zero-length streams; they can be reconstructed exactly. Four expanded perf loss dumps and original perf recordings remain local with hashes, keeping raw padding-heavy dumps out of Git. Raw source/build failures are retained.\n\nZeroSDKrequests, proof exports or new Gnark verifications occur; ledger remains**17,811**. There is no new accepted proving-time/RSS result, AWS/Aeglos comparison, mobile/ARM/TEE, huge-circuit, pressure/minRAM, concurrency or throughput qualification.\n\n## Inherited CPU and Metal comparison\n\nAccepted local timings against supplied historical M5 references, not measured by this component. Accepted primary peak RSS remains134MiB. The prior full-performance-gate failure and16-worker calibration engineering exception remain.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
        let v = &a["inherited_M5"][shape];
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
    md.push_str("\nAll-eight CPU10× goal remains active/unmet (0/8); accepted54→1 remains162.250464ms without a supplied M5 reference.\n\n[Design](artifacts/design.md), [declaration](artifacts/declaration.json), [model](artifacts/model.json), [kernel](artifacts/kernel.rs), [analysis](artifacts/analysis.json), [review](artifacts/postcapture-review.json), [pins](artifacts/pins.json), [artifact hashes](artifact-index.json).\n");
    let d = Path::new(D);
    fs::create_dir_all(d.join("artifacts/src/bin")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut index = BTreeMap::new();
    for f in [
        "setup.rs",
        "setup.log",
        "field-original-pins.json",
        "bounds.md",
        "design.md",
        "kernel.rs",
        "test-field.rs",
        "src/bin/probe.rs",
        "Cargo.toml",
        "Cargo.lock",
        "model.rs",
        "model-initial.rs",
        "model.json",
        "model-build.log",
        "model-build-initial.log",
        "model.stderr.log",
        "native-tests.log",
        "frozen-build.log",
        "native-build.log",
        "control.rs",
        "control-build.log",
        "declaration.json",
        "pins.json",
        "run-plan.json",
        "prepare.log",
        "finish.log",
        "observations.json",
        "analysis.json",
        "raw-profile-pins.json",
        "local-only-sha256.json",
        "run-receipts.json",
        "hot-code.json",
        "frozen-selected.s",
        "candidate-selected.s",
        "profile-summary.json",
        "postcapture-review.json",
        "report.rs",
        "report-build.log",
        "runs/result.json",
    ] {
        let from = Path::new(R).join(f);
        let to = d.join("artifacts").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        assert_eq!(hash(from), hash(&to));
        index.insert(format!("artifacts/{f}"), hash(to));
    }
    save(d.join("artifact-index.json"), &json!(index));
    index.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    index.insert("README.md".into(), hash(d.join("README.md")));
    let mut sums = String::new();
    for (p, h) in &index {
        writeln!(sums, "{h}  {p}").unwrap();
    }
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!(
        "Archived{}hashes/{}files; gatefalse;ledger17811unchanged",
        index.len(),
        index.len() + 1
    );
}

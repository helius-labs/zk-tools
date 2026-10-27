use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-g1-raw-scratch";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-g1-raw-scratch";
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
fn n(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn archive(root: &Path, dir: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let rel = p.strip_prefix(root).unwrap();
        let s = rel.to_str().unwrap();
        if e.file_type().unwrap().is_dir() {
            if !e.file_name().to_str().unwrap().starts_with("target") {
                archive(root, &p, dest, index)
            }
        } else if ["rs", "md", "json", "jsonl", "log", "txt", "proof", "public"]
            .contains(&p.extension().and_then(|v| v.to_str()).unwrap_or(""))
            && (!s.starts_with("candidate/")
                || [
                    "candidate/crates/zopro-core/src/msm.rs",
                    "candidate/crates/zopro-core/src/simd.rs",
                    "candidate/crates/zopro-core/src/msm/g1_storage.rs",
                ]
                .contains(&s)
                || s.starts_with("candidate/.xtask/"))
            && (!s.starts_with("fresh/") || s.starts_with("fresh/.xtask/"))
        {
            let out = dest.join(rel);
            fs::create_dir_all(out.parent().unwrap()).unwrap();
            fs::copy(&p, &out).unwrap();
            assert_eq!(hash(&p), hash(&out));
            assert!(index.insert(s.into(), hash(out)).is_none());
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["SDK_requests"], 43410);
    assert_eq!(a["independent_exports"], 465);
    assert_eq!(a["next_prior_count"], 12125);
    assert_eq!(
        read(r.join("performance/result.json"))["status"],
        "complete"
    );
    for (f, h) in read(r.join("performance/protocol.json"))["pins"]
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let m = read("/tmp/zopro-clean-fft-adoption/production-source-manifest.json");
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 172);
    let edits = read(r.join("edits.json"));
    assert_eq!(edits["edits"].as_array().unwrap().len(), 2);
    for (f, h) in originals {
        assert_eq!(hash(Path::new(P).join(f)), h.as_str().unwrap());
        if let Some(e) = edits["edits"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e["file"] == f.as_str())
        {
            assert_eq!(
                fs::read_to_string(Path::new(P).join(f)).unwrap(),
                e["before"].as_str().unwrap()
            );
            assert_eq!(
                fs::read_to_string(r.join("candidate").join(f)).unwrap(),
                e["after"].as_str().unwrap()
            );
        } else {
            assert_eq!(hash(r.join("candidate").join(f)), h.as_str().unwrap());
        }
        assert_eq!(hash(r.join("fresh").join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(
            r.join("candidate")
                .join(edits["added_file"].as_str().unwrap())
        ),
        edits["added_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 930);
    for (f, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    for mode in ["fresh", "candidate"] {
        assert_eq!(
            read(r.join(format!("native/{mode}-core/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            if mode == "candidate" { 121 } else { 119 }
        );
        assert_eq!(
            read(r.join(format!("native/{mode}-prover/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
    }
    let rows = read(r.join("performance/rows.json"));
    let profiles: Vec<_> = rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["phase"] == "profile")
        .collect();
    assert_eq!(profiles.len(), 6);
    let mut samples = 0u64;
    let mut raw = BTreeMap::new();
    for row in &profiles {
        assert_eq!(row["profile"]["lost_records"], 0);
        assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
        samples += row["profile"]["samples"].as_u64().unwrap();
        let path = r
            .join("performance/profile/b0")
            .join(row["case"].as_str().unwrap())
            .join(row["variant"].as_str().unwrap())
            .join("perf.data");
        raw.insert(path.display().to_string(), hash(&path));
    }
    for mode in ["fresh", "candidate"] {
        let receipt = read(r.join(format!("native/{mode}/artifacts.json")));
        let path = receipt["binary"].as_str().unwrap();
        raw.insert(path.to_owned(), hash(path));
    }
    save(r.join("raw-local-only-sha256.json"), &json!(raw));
    let primary = &a["cases"]["1x2-w32"];
    let pair = &primary["pairs"]["candidate/accepted"];
    let mut md=format!("# Lossless packed G1 batch records\n\nFull performance gate: **{}**. Primary candidate latency is **{:.6} ms**, versus **{:.6} ms accepted** and **{:.6} ms fresh**. Its paired candidate/accepted median ratio is **{:.9}**, interval **[{:.9}, {:.9}]**, {}/15 wins, with a paired saving of **{:.3} µs**. Positive and negative estimates remain recorded without a gain floor; process medians and paired estimates use different aggregation.\n\nThe candidate is not adopted by this study. Accepted 172-file bounded-FFT source and SDK14da8231 remain unchanged. The all-eight supplied M5 CPU 10× goal remains active, achieved {}/8.\n\n## Implementation\n\nG1 streaming records retain all five values using existing lossless numerical SIMD packing: 1,600→1,280 bytes per eight-point record. Reverse arithmetic expands each stored operand. No coordinate recomputation, key reread, new field arithmetic, runtime flag, table, pool or unsafe code is introduced. G2, tree scratch and unsupported four-lane/portable storage remain unchanged. Existing signs, tails, initialization, batching and windows remain. The resource forecast deliberately charges the old five-expanded-value record, so narrower storage cannot increase admission during the study. Two original files change and one private module is added; 170 other original files are exact. See the [storage review](artifacts/integration-review.md).\n\nThis differs from earlier compact/late-coordinate trials, which removed and reconstructed or reloaded fields. The hypothesis is a different cache and spill tradeoff. A 20% record reduction is not a claim of 20% total RAM or latency improvement.\n\n## Complete SDK results\n\nFifteen fixed rotated/reversed blocks use accepted, fresh and candidate binaries. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on physical CPUs 0–15. Primary32 uses 151 requests after three primes, physical16 uses 501, and other shapes use 31. All 450 latency processes finish before six separate 199 Hz profiles. Loading and primes are excluded from latency and included in process RSS; fresh solve, randomness, prove, internal verification and serialization are timed. No helper, build, hash, archive or second owned CPU workload overlaps requests. Required independent verifiers run serially between captures. No successful capture is extended, omitted or repeated.\n\n|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval|Wins|Saving µs|RSS A/C MiB|\n|---|---:|---:|---:|---:|---|---:|---:|---:|\n",a["full_gate_passed"],n(&primary["modes"]["candidate"]["ms"]),n(&primary["modes"]["accepted"]["ms"]),n(&primary["modes"]["fresh"]["ms"]),n(&pair["median_ratio"]),n(&pair["median_confidence"]["lower"]),n(&pair["median_confidence"]["upper"]),pair["wins"],n(&pair["saving_us"]),a["original8_CPU10x_rows"]);
    for name in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
        "merge-54x1-w32",
        "1x2-w16",
    ] {
        let c = &a["cases"][name];
        let p = &c["pairs"]["candidate/accepted"];
        let ci = &p["median_confidence"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.9}|[{:.9}, {:.9}]|{}/15|{:.3}|{:.2}/{:.2}|",
            n(&c["modes"]["accepted"]["ms"]),
            n(&c["modes"]["fresh"]["ms"]),
            n(&c["modes"]["candidate"]["ms"]),
            n(&p["median_ratio"]),
            n(&ci["lower"]),
            n(&ci["upper"]),
            p["wins"],
            n(&p["saving_us"]),
            n(&c["modes"]["accepted"]["maxrss_mib"]),
            n(&c["modes"]["candidate"]["maxrss_mib"])
        )
        .unwrap();
    }
    md.push_str("\n## Frozen checks\n\nAll ten fresh/accepted individual median intervals must stay within factor 1.03; candidate upper bounds must be ≤1.02 against both controls. Primary32 upper bounds must be below parity; original-eight within-block geometric aggregate upper bounds must be ≤1. Fifteen-block order intervals use ranks 4–12, nominal coverage 96.484375% under independent/exchangeable blocks, without simultaneous familywise guarantees. No failed check is relabeled.\n\n");
    for (field, label) in [
        ("calibration_failures", "Calibration"),
        ("regressions", "Nonregression bounds"),
        ("primary_gain_failures", "Primary gain"),
        ("aggregate_failures", "Aggregate"),
    ] {
        writeln!(
            md,
            "{label}: {} failure(s).",
            a[field].as_array().unwrap().len()
        )
        .unwrap();
        for f in a[field].as_array().unwrap() {
            writeln!(md, "\n- {}", f.as_str().unwrap()).unwrap();
        }
        md.push('\n');
    }
    for pair in ["candidate/accepted", "candidate/fresh"] {
        let v = &a["original8"][pair];
        writeln!(
            md,
            "Original-eight {pair}: {:.9}, interval [{:.9}, {:.9}].\n",
            n(&v["median"]),
            n(&v["lower"]),
            n(&v["upper"])
        )
        .unwrap();
    }
    md.push_str("## Warm profiles\n\nSampled CPU shares are not exclusive critical-path costs, memory bandwidth or removable wall-time fractions. The six profiles report zero lost records and one SDK PID each.\n\n|Case|Variant|Samples|CPU ms/request|G1 %|G2 %|Inverse %|Epoch %|Scheduler %|\n|---|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in &profiles {
        let p = &row["profile"];
        let c = &p["category_percent"];
        let s = &row["summary"];
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|",
            row["case"].as_str().unwrap(),
            row["variant"].as_str().unwrap(),
            p["samples"],
            (n(&s["user_cpu_ms"]) + n(&s["system_cpu_ms"])) / n(&s["rounds"]),
            n(&c["G1_arithmetic"]),
            n(&c["G2_arithmetic"]),
            n(&c["inverse"]),
            n(&c["epoch"]),
            n(&c["scheduler"])
        )
        .unwrap();
    }
    md.push_str("\n## Supplied M5 references\n\nThese are supplied historical CPU/Metal references, not new Apple measurements. Ratios below use the candidate local32 medians. No 54 reference was supplied.\n\n|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU 10× ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for name in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
    ] {
        let c = &a["cases"][name];
        let t = n(&c["modes"]["candidate"]["ms"]);
        let cpu = n(&c["reference"]["cpu_ms"]);
        let metal = n(&c["reference"]["metal_ms"]);
        writeln!(
            md,
            "|{}|{t:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
            c["shape"].as_str().unwrap(),
            cpu / t,
            metal / t,
            cpu / 10.
        )
        .unwrap();
    }
    write!(md,"\n## Validation and scope\n\nFresh passes 119 native core tests; candidate passes 121. Two new contracts compare stored arithmetic against scalar fields and check the footprint/forecast bound. Existing complete-curve G1 oracles exercise signs, owned/borrowed batches, all store and inverse modes, tails and scratch reuse. Both builds pass two raw and six cubic tests; candidate portable/no-IFMA consumer checks pass. Native field source and its 33-test qualification are inherited unchanged, not newly executed here.\n\nNine preflights add 36 SDK calls/nine exports; latency adds 41,850/450 and profiles add 1,524/six. Total **43,410 SDK calls, 465 distinct independently Gnark-verified exports and 930 proof/public members**. All proofs are 164 bytes with exact fixture bindings and fresh against prior/new identities. The ledger reaches **12,125 from 11,660**. All pins are checked again at closure. Six profiles contain {samples} samples. Raw perf data and executables remain local with hashes; decoded symbols, proofs and receipts are archived.\n\nNo new AWS, Aeglos, physical-phone/ARM/TEE, memory-pressure, minimum-RAM, throughput or giant-circuit performance claim is made. A positive result requires separate compatibility, concurrent and resource review before adoption. The original all-eight CPU 10× goal remains active.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [rows](artifacts/performance/rows.json), [source edits](artifacts/edits.json), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [raw hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).\n").unwrap();
    save(
        r.join("closeout.json"),
        &json!({"status":"complete lossless packed G1 batch records experiment","full_performance_gate_passed":a["full_gate_passed"],"new_source_adopted":false,"source_files_unchanged":172,"SDK_requests":43410,"distinct_independent_exports":465,"proof_public_members":930,"next_prior_count":12125,"profiles":6,"cycle_samples":samples,"owned_live_jobs":[],"new_AWS_resources":[],"original8_CPU10x_rows":a["original8_CPU10x_rows"],"goal_achieved":false}),
    );
    fs::create_dir_all(d.join("artifacts")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    println!("Closed G1 raw scratch: {} artifact hashes;43410SDK465Gnark;ledger12125;fullpass{};source unchanged",index.len(),a["full_gate_passed"]);
}

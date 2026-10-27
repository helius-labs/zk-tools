use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-g1-reference-staging";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-g1-reference-staging";
const M: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let ty = e.file_type().unwrap();
        let rel = p.strip_prefix(root).unwrap();
        let s = rel.display().to_string();
        if ty.is_dir() {
            if !e.file_name().to_str().unwrap().starts_with("target") {
                archive(root, &p, out, index)
            }
        } else if ty.is_file() {
            if (s.starts_with("candidate/") || s.starts_with("fresh/"))
                && !s.ends_with(".xtask/snapshot.json")
                && !s.ends_with(".xtask/source-pins.json")
                && s != "candidate/crates/zopro-core/src/msm.rs"
            {
                continue;
            }
            if !["rs", "md", "json", "jsonl", "txt", "log", "proof", "public"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            {
                continue;
            }
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            assert_eq!(hash(&p), hash(&dest));
            index.insert(s, hash(&dest));
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["SDK_requests"], 13980);
    assert_eq!(a["independent_exports"], 285);
    assert_eq!(a["next_prior_count"], 9439);
    assert_eq!(a["full_gate_passed"], false);
    assert_eq!(
        read(r.join("performance/result.json"))["status"],
        "complete"
    );
    let pins = read(r.join("performance/protocol.json"))["pins"].clone();
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    let manifest = read(M);
    let src = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(src.len(), 171);
    for (p, h) in src {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        )
    }
    assert_eq!(
        hash(manifest["native_binary"].as_str().unwrap()),
        manifest["native_binary_sha256"].as_str().unwrap()
    );
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 570);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    for mode in ["fresh", "candidate"] {
        let t = if mode == "fresh" {
            r.join("native/fresh-core/tests.json")
        } else {
            r.join("native-v2/candidate-core/tests.json")
        };
        assert_eq!(read(t)["passed_tests"].as_array().unwrap().len(), 116);
    }
    let rows = read(r.join("performance/rows.json"));
    let profiles: Vec<_> = rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["phase"] == "profile")
        .collect();
    assert_eq!(profiles.len(), 6);
    let samples: u64 = profiles
        .iter()
        .map(|v| v["profile"]["samples"].as_u64().unwrap())
        .sum();
    assert_eq!(samples, 85134);
    for p in &profiles {
        assert_eq!(p["profile"]["lost_records"], 0);
        assert_eq!(p["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
    }
    let codegen = read(r.join("codegen.json"));
    for row in codegen["rows"].as_array().unwrap() {
        let file = row["disassembly"]
            .as_str()
            .unwrap()
            .replace(".asm.txt", ".normalized.txt");
        assert_eq!(hash(file), row["normalized_sha256"].as_str().unwrap());
    }
    let mut md=String::from("# G1 temporary arrays of references\n\nThe reference-staging candidate establishes **no complete-proof gain** and is not adopted. Primary32 measures **8.468905 ms**, versus accepted **8.379486 ms** and fresh **8.340873 ms**. Paired median ratios are **1.007710 / 1.011824**, with3/9and2/9wins; both primary intervals include parity. The original-eight aggregate intervals also include parity. Several nonregression bounds and the physical16 control-calibration bound fail. Positive estimates for other cases remain in the results below.\n\nThe adopted combined implementation and exact accepted824164SDK remain unchanged. This experiment adds no field primitive, intrinsic, table, cache, pool or flag. The original all-eight supplied CPU10× goal remains active/unmet0/8.\n\n## Change and actual SDK code\n\nOnly stream_g1_impl in msm.rs changes: two [Affine;LANES] staging arrays become [&Affine;LANES] in each of the two existing forward-record storage modes. Both still feed the original FqBatch constructor; one Borrow call per operand/lane, index/sign accesses, field arithmetic, inverse order, record writes, tails and outputs are preserved. Safe references end in each synchronous group. There is no new unsafe code or heap allocation. All170other accepted source files remain exact; reversing the recorded four edits recovers all171accepted bytes. This concerns temporary arithmetic input staging, separate from adopted borrowed pending/collision records and prior G2 index/tree gather trials.\n\nStatic inspection uses the actual standard SDK binaries, not isolated packing wrappers. All five fresh/accepted normalized stream instruction shapes agree. Candidate shapes differ, but arithmetic-site counts do not fall. The table shows two canonical monomorphs; counts include the whole function, not just a runtime loop.\n\n|Canonical stream|Variant|Text bytes|Instructions|Stack-reference sites|Memory-operand sites|IFMA sites|\n|---|---|---:|---:|---:|---:|---:|\n");
    for row in codegen["rows"].as_array().unwrap().iter().filter(|v| {
        v["variant"] != "fresh"
            && v["symbol"]
                .as_str()
                .unwrap()
                .contains("G1Config, true, true,")
    }) {
        let s = row["symbol"].as_str().unwrap();
        writeln!(
            md,
            "|{}|{}|{}|{}|{}|{}|{}|",
            if s.contains("true, true, &") {
                "borrowed"
            } else {
                "owned"
            },
            row["variant"].as_str().unwrap(),
            row["bytes"],
            row["counts"]["instructions"],
            row["counts"]["stack_reference_instructions"],
            row["counts"]["memory_operand_instructions"],
            row["counts"]["IFMA_sites"]
        )
        .unwrap();
    }
    md.push_str("\nThree normalizer tests retain stack offsets, registers, constants and local branch destinations while normalizing RIP displacements and external symbol addresses. Matching shapes do not prove identical global referents or dynamic behavior. Static stack-reference counts are not dynamic spill counts or stack/RSS savings. Replacing a source-level point array does not establish that actual copies were removed or identify a cause for the measured result.\n\n## Verified complete SDK requests\n\nRyzen9950X, nine rotated/reversed fresh-process blocks; all nine shapes at32workers plus primary16 on CPUs0–15. Exact accepted824164, source-identical fresh control and one-file candidate use the same standard driver/support and native flags. Each32-worker process runs31fresh verified SDK requests after3primes; physical16 runs151after3. Witness solving, fresh blinding/proving, SDK verification and serialization are timed; loading/primes are excluded. All270latency processes finish before six separate warm profiles. No short screen, extension, successful repeat, exclusion or postcapture source/flag change occurs.\n\nTimes are medians of process medians. Ratios, savings, wins and intervals use within-block pairs; differences or ratios of displayed medians need not agree. RSS is the maximum lifetime process peak across blocks, including loading/primes.\n\n|Case|Accepted ms|Fresh ms|Candidate ms|Paired /accepted|Individual median interval|Wins /accepted|Paired saving µs|Accepted / candidate RSS MiB|\n|---|---:|---:|---:|---:|---|---:|---:|---:|\n");
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
        let pair = &c["pairs"]["candidate/accepted"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|[{:.6}, {:.6}]|{}/9|{:.3}|{:.2} / {:.2}|",
            c["modes"]["accepted"]["ms"].as_f64().unwrap(),
            c["modes"]["fresh"]["ms"].as_f64().unwrap(),
            c["modes"]["candidate"]["ms"].as_f64().unwrap(),
            pair["median_ratio"].as_f64().unwrap(),
            pair["median_confidence"]["lower"].as_f64().unwrap(),
            pair["median_confidence"]["upper"].as_f64().unwrap(),
            pair["wins"],
            pair["saving_us"].as_f64().unwrap(),
            c["modes"]["accepted"]["maxrss_mib"].as_f64().unwrap(),
            c["modes"]["candidate"]["maxrss_mib"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\nPrimary32 candidate/accepted interval[0.994633,1.026071] and candidate/fresh[0.997064,1.017231] do not establish an improvement; their paired point estimates are0.77%/1.18%slower. Original-eight aggregate medians1.003491/1.005158 have intervals[0.994612,1.008147]/[0.995196,1.009576].\n\nPositive paired estimates are retained:4x2 saves132.059µs versus accepted,12x2 saves120.436µs and24x2 saves205.709µs. Their intervals include parity, and the4x2paired ratio versus fresh is1.007400 despite its lower displayed candidate median. Merge24 saves51.576µs versus accepted but is slower against fresh, with individual interval[1.000248,1.009783] above parity. These are not called zero or discarded; they do not justify adopting this candidate globally.\n\nPrimary32 aggregate CPU time/request is192.026ms accepted,190.823fresh and191.187candidate. The lower CPU point estimate against accepted does not reproduce against fresh; it is not a demonstrated reduction in CPU work. RSS is also mixed: primary112.75/110.25MiB,24x21285.39/1295.80MiB and54 1847.52/1846.78MiB. No universal memory saving or minimumRAM is established.\n\n## Frozen gate and profiles\n\nThe individual distribution-free median intervals have96.09375%nominal coverage for nine independent/exchangeable blocks (ranks2..8), not familywise coverage. Frozen requirements: all10fresh/accepted intervals wholly within factor1.03; candidate upper≤1.02 against both controls in all10; primary32 upper<1 against both; original-eight geometric aggregate upper≤1 against both. No gain floor or failed-bound waiver is applied. An unmet bound does not by itself prove a causal slowdown.\n\n");
    for (label, key) in [
        ("Calibration failures", "calibration_failures"),
        ("Nonregression failures", "regressions"),
        ("Primary gain failures", "primary_gain_failures"),
        ("Aggregate failures", "aggregate_failures"),
    ] {
        writeln!(md, "{label}: `{}`.\n", a[key]).unwrap();
    }
    md.push_str("Six warm199Hzcycles:u profiles run after all latency,251requests after3primes each:85,134samples, one sampled SDK PID each and zero lost records. Their per-request CPU and period-weighted categories are descriptive; different process runs and sampling/inlining prevent interpreting shares as exclusive wall cost, dynamic copy/spill counts or a cache cause.\n\n|Case|Variant|Samples|CPU ms/request|G1 arithmetic %|Epoch %|\n|---|---|---:|---:|---:|---:|\n");
    for row in &profiles {
        let s = &row["summary"];
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.2}|{:.2}|",
            row["case"].as_str().unwrap(),
            row["variant"].as_str().unwrap(),
            row["profile"]["samples"],
            (s["user_cpu_ms"].as_f64().unwrap() + s["system_cpu_ms"].as_f64().unwrap())
                / s["rounds"].as_f64().unwrap(),
            row["profile"]["category_percent"]["G1_arithmetic"]
                .as_f64()
                .unwrap(),
            row["profile"]["category_percent"]["epoch"]
                .as_f64()
                .unwrap()
        )
        .unwrap();
    }
    md.push_str("\n## Supplied M5 references\n\nHistorical supplied CPU/Metal figures are not fresh Apple measurements. Candidate numbers remain experimental; no54reference was supplied.\n\n|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
    ] {
        let c = &a["cases"][name];
        let t = c["modes"]["candidate"]["ms"].as_f64().unwrap();
        if let Some(cpu) = c["reference"]["cpu_ms"].as_f64() {
            let metal = c["reference"]["metal_ms"].as_f64().unwrap();
            writeln!(
                md,
                "|{}|{t:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
                c["shape"].as_str().unwrap(),
                cpu / t,
                metal / t,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(md, "|54→1|{t:.6}|—|—|—|—|—|").unwrap();
        }
    }
    md.push_str("\n## Correctness, provenance and next step\n\nFresh control and candidate each pass all116unfiltered native core tests and raw2/cubic6; retained independent G1 signed-stream/tail/reuse/owned/borrowed/fused/ordinary generator oracles cover the edit. Candidate portable and noIFMA consumer checks pass. Field sources and field qualification remain unchanged; no new field-suite, portable-unit, pressure or huge-circuit qualification is claimed here.\n\nThe initial native workflow completed fresh116core/prover8/libraries, then stopped before invoking rustc because the standalone SDK driver/support are separate manifest fields and absent from the copied171library/source files. A new repaired workflow links the existing exact pinned driver/support and reuses those completed tests/libraries unchanged; it runs the remaining candidate tests/builds. Original plan/failure/source and repair receipts are retained. No arithmetic source, compiler flag, successful test or request capture was rerolled for the repair.\n\nAll nine four-worker candidate preflights pass36SDKcalls/nine independent exports. Latency adds12,420calls/270exports and profiles1,524calls/six exports. Total **13,980SDKcalls**, **285distinct independently Gnark-verified exports**, **570proof/public members**, ledger **9,154→9,439**. Every SDK call verifies internally; all final exports meet164-byte length, exact public binding and freshness against all prior/new identities. No helper/build/hash/archive/independent verifier or second owned CPU workload overlaps timing/profiles. Closing audits rehash all pins/proof members and all171production source bytes; exact824164acceptedSDK remains unchanged.\n\nDo not adopt or retune reference-array staging, other pointer construction syntax or incidental code layout from these results. The source-array appearance did not establish the expected CPU reduction in the real caller. Pursue a distinct algorithmic hypothesis with a concrete CPU-work argument. Existing Montgomery-first grouping and prior failed bitmap/idle-yield/empty-victim policies are not new directions. The adopted3.13%combined gain remains in place. No new AWS/Aeglos/phone/ARM/TEE/large-circuit/pressure/minRAM/throughput result is claimed; previous C7a comparisons remain previous-source measurements.\n\n[Design](artifacts/design.md), [SDK protocol](artifacts/sdk-design.md), [source audit](artifacts/source-audit.json), [code generation](artifacts/codegen.json), [analysis](artifacts/analysis.json), [native repair](artifacts/native-repair.json), [proof inventory](artifacts/proof-inventory.json), [raw local profile hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).\n");
    save(
        r.join("closeout.json"),
        &json!({"status":"complete G1 reference-array staging SDK experiment","full_gate_passed":false,"adopted":false,"SDK_requests":13980,"independent_gnark_exports":285,"proof_public_members":570,"next_prior_count":9439,"production_sources_checked_unchanged":171,"candidate_changed_source_files":1,"fresh_core_tests":116,"candidate_core_tests":116,"raw_tests_per_variant":2,"cubic_tests_per_variant":6,"field_qualification_inherited":true,"profiles":6,"cycle_samples":samples,"closing_pins_checked":pins.as_object().unwrap().len(),"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    fs::create_dir_all(d).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!(
        "Closed G1 reference staging;{}archivehashes;13980SDK285Gnark;ledger9439;noadoption",
        index.len()
    );
}

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-g1-reverse-sdk";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-g1-reverse-sdk";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let rel = p.strip_prefix(root).unwrap();
        let name = e.file_name();
        let name = name.to_str().unwrap();
        let ty = e.file_type().unwrap();
        if ty.is_dir() {
            if name.starts_with("target") {
                continue;
            }
            archive(root, &p, out, index);
        } else if ty.is_file() {
            let s = rel.display().to_string();
            if s.starts_with("candidate/") && !s.ends_with("crates/zopro-core/src/msm.rs") {
                continue;
            }
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !["rs", "md", "json", "jsonl", "log", "txt", "proof", "public"].contains(&ext) {
                continue;
            }
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            assert_eq!(hash(&dest), hash(&p));
            index.insert(s, hash(dest));
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["next_prior_count"], 8595);
    assert_eq!(a["SDK_requests"], 13980);
    assert_eq!(a["independent_exports"], 285);
    let pins = read(r.join("performance/protocol.json"))["pins"].clone();
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 570);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(
        hash("/tmp/zopro-grouping-module/native/candidate-sdk"),
        "03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d"
    );
    let closeout = json!({"status":"G1 reverse-group overlap complete SDK study closed","full_gate_passed":a["full_gate_passed"],"production_sources_checked_unchanged":170,"changed_candidate_originals":1,"unchanged_candidate_originals":169,"component_native_core_tests_reused":111,"raw_tests_reused":2,"cubic_tests_reused":6,"field_source_qualification_inherited":true,"closing_pins_checked":pins.as_object().unwrap().len(),"SDK_requests":13980,"independent_gnark_exports":285,"proof_public_members":570,"next_prior_count":8595,"original8_CPU10x_rows":a["original8_CPU10x_rows"],"profiles":6,"adopted":false,"goal_achieved":false});
    save(r.join("closeout.json"), &closeout);
    let mut md=format!("# G1 reverse-group overlap: complete SDK requests\n\nFull performance gate: **{}**. Supplied CPU10× rows: **{}/8**. Accepted production remains unchanged; no source is adopted by this report. Small reliable gains have no minimum cutoff.\n\nThe standard SDK changes only the G1 streaming reverse loop to the exact body measured in the [preceding component](../2026-10-27-g1-reverse-overlap/README.md). Two consecutive groups expose the next slope before finishing the current point outputs, preserving the original single inverse chain and descending output order. Forward records, scalar tails, signed/canonical guards, field formulas, storage, capacities, point order, scheduler, G2 and affine trees remain unchanged. There is no diagnostic reference/probe/export in this standard build, second inversion chain, new field helper, table, cache, execution plan, point copy or heap storage. Extra live vectors and code size remain possible register/stack/cache tradeoffs.\n\nThe component's targeted signed-batch aggregate improved0.817–0.818%, with7/7 winning block aggregates and all40cases calibrated/nonregressing. That supports this whole-proof study without predicting its gain. Independent111core/raw2/cubic6/generator/portable/no-IFMA results from that exact stream body are pinned and **reused**, not freshly rerun for the standard SDK. All169other original files and all170production files remain unchanged. Inverse source edits recover accepted bytes.\n\n## Verified request latency\n\nRyzen9950X. Nine rotated/reversed fresh-process blocks, all nine shapes at32workers plus primary16 on CPUs0–15. Three variants: exact accepted SDK, exact reused fresh original baseline and new standard candidate. All use the existing native compiler/runtime configuration and fat-LTO/one-codegen-unit driver; controls are reused with source/library/consumer/binary hashes. Each32-worker process runs31timed requests after3primes; primary16 runs151after3. Timed work includes witness solving, fresh blinding/proving, SDK verification and serialization; loading/primes are excluded. All270latency processes precede six separate profiles. There is no short screen, extension, successful repeat, case exclusion or postcapture source/runtime change.\n\nTimes are medians of process medians; paired ratios and savings use within-block observations. Ratios/differences of displayed medians need not match paired estimates. RSS is the maximum lifetime process peak across blocks, including loading/primes; it is not minimum RAM or incremental scratch. CPU includes coordination.\n\n|Case|Accepted ms|Fresh ms|Overlap ms|Paired /accepted|Individual median interval|Wins /accepted|Paired saving µs|Accepted / overlap RSS MiB|Overlap CPU ms/request|\n|---|---:|---:|---:|---:|---|---:|---:|---:|---:|\n",a["full_gate_passed"],a["original8_CPU10x_rows"]);
    let p = read(r.join("performance/protocol.json"));
    for config in p["cases"].as_array().unwrap() {
        let name = config["name"].as_str().unwrap();
        let c = &a["cases"][name];
        let m = &c["modes"];
        let q = &c["pairs"]["overlap/accepted"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|[{:.6}, {:.6}]|{}/9|{:.3}|{:.2} / {:.2}|{:.3}|",
            m["accepted"]["ms"].as_f64().unwrap(),
            m["fresh"]["ms"].as_f64().unwrap(),
            m["overlap"]["ms"].as_f64().unwrap(),
            q["median_ratio"].as_f64().unwrap(),
            q["median_confidence"]["lower"].as_f64().unwrap(),
            q["median_confidence"]["upper"].as_f64().unwrap(),
            q["wins"],
            q["saving_us"].as_f64().unwrap(),
            m["accepted"]["maxrss_mib"].as_f64().unwrap(),
            m["overlap"]["maxrss_mib"].as_f64().unwrap(),
            m["overlap"]["cpu_ms_per_request"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\n## Frozen qualification\n\nIndividual distribution-free median intervals have at least95% nominal coverage under independent/exchangeable block assumptions. Nine-block endpoints2..8 have96.09375% coverage under those assumptions; no simultaneous/familywise guarantee is claimed. Fresh/accepted calibration intervals must lie wholly within factor1.03 in all10cases. Candidate nonregression upper bounds must be≤1.02 against accepted/fresh in all10. Primary32 gain upper bounds must be<1 against both controls. Original-eight within-block geometric aggregate upper bounds must be≤1 against both. Primary16 requires nonregression, not strict gain. There is no minimum gain floor or waiver of a failed bound. An unmet bound can indicate insufficient precision and is not by itself proof of a causal regression. Performance success would still require resource/pressure/compatibility review before adoption.\n\n");
    writeln!(md,"Calibration failures: `{}`.\n\nNonregression failures: `{}`.\n\nPrimary gain failures: `{}`.\n\nOriginal-eight aggregate failures: `{}`.\n\nOriginal-eight aggregate intervals: `{}`.\n",a["calibration_failures"],a["regressions"],a["primary_gain_failures"],a["aggregate_failures"],a["original8"]).unwrap();
    md.push_str("[Analysis](artifacts/analysis.json) retains every paired ratio, saving, interval, block and failure. Positive signals and adverse cases remain together.\n\n## Supplied M5 references\n\nThese CPU/Metal timings were supplied historically and are not fresh Apple measurements. No54→1reference was supplied. Candidate times remain experimental.\n\n|Shape|Overlap ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for config in p["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let name = config["name"].as_str().unwrap();
        let c = &a["cases"][name];
        let ms = c["modes"]["overlap"]["ms"].as_f64().unwrap();
        if let (Some(cpu), Some(metal)) = (
            c["reference"]["cpu_ms"].as_f64(),
            c["reference"]["metal_ms"].as_f64(),
        ) {
            writeln!(
                md,
                "|{}|{ms:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
                c["shape"].as_str().unwrap(),
                cpu / ms,
                metal / ms,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(md, "|{}|{ms:.6}|—|—|—|—|—|", c["shape"].as_str().unwrap()).unwrap();
        }
    }
    md.push_str("\n## Correctness, resources and profiles\n\nAll nine candidate shapes pass a four-worker functional preflight, one request after three primes:36SDKcalls/nine fresh independently Gnark-verified exports. The latency matrix adds12,420calls/270exports. After all latency, primary32 and4×2 profiles run all three variants,251timed requests after3primes:1,524calls/six exports. Total **13,980SDK calls**, **285distinct independently verified exports**, **570proof/public members**; independent proof ledger **8,310→8,595**. Every export has164-byte proof length, pinned public binding and freshness against all prior/new identities. No subinventory is counted twice. SDK verification also runs on every measured request.\n\nThe exact component stream body passed independent complete-curve output/reuse checks across signed/canonical/inverse/storage modes, empty/full/odd/even boundaries and tails. These component receipts and unchanged field qualification are inherited; actual standard-SDK correctness is established here by the all-shape preflight and every fresh request/export. No new record fields, record allocations or heap arrays are introduced. Whole-process RSS and CPU remain measured outcomes, not formal stack, allocator or universal RAM guarantees. Phone, memory-pressure, concurrent-caller, giant-circuit and compatibility qualification beyond the pinned component receipts remain unclaimed.\n\nSix warm199Hzcycles:u profiles follow every latency capture; each samples one own process with zero lost records. Decoded categories, symbols and resource receipts are retained in [rows](artifacts/performance/rows.json); raw profiles remain local with hashes. Sampled periods are not removable proof-wall fractions or cache/spill causality. No helper/build/hash/archive/verifier overlaps a timed capture. Closing checks rehash all source/library/tool/helper/control pins and all proof/public members. No fresh AWS/Aeglos, phone, TEE, giant, swap, minimum-RAM, pressure, throughput or concurrency result is claimed.\n\n");
    for variant in p["variants"].as_array().unwrap() {
        let binary = variant["binary"].as_str().unwrap();
        writeln!(
            md,
            "{} executable: `{binary}`; SHA-256 `{}`.\n",
            variant["name"].as_str().unwrap(),
            hash(binary)
        )
        .unwrap();
    }
    md.push_str("[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!(
        "Closed {}archivedhashes,13980SDK285Gnark;prior8595;fullgate{};CPU10x{}/8",
        index.len(),
        a["full_gate_passed"],
        a["original8_CPU10x_rows"]
    );
}

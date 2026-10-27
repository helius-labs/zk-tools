use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-composed-cpu";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-composed-cpu-sdk";
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
            if name.starts_with("target") || name == "recovery" {
                continue;
            }
            archive(root, &p, out, index);
        } else if ty.is_file() {
            let s = rel.display().to_string();
            if s.starts_with("candidate/")
                && ![
                    "candidate/crates/zopro-core/src/msm.rs",
                    "candidate/crates/zopro-core/src/msm_affine.rs",
                    "candidate/crates/zopro-core/src/msm_affine/fused_kh.rs",
                    "candidate/crates/zopro-core/src/msm_affine/borrowed_pending.rs",
                    "candidate/crates/zopro-core/src/solver/sparse_poseidon.rs",
                ]
                .contains(&s.as_str())
            {
                continue;
            }
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ![
                "rs", "md", "json", "jsonl", "log", "txt", "patch", "proof", "public",
            ]
            .contains(&ext)
            {
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
    assert_eq!(a["next_prior_count"], 9060);
    assert_eq!(a["SDK_requests"], 22260);
    assert_eq!(a["independent_exports"], 465);
    let pins = read(r.join("performance/protocol.json"))["pins"].clone();
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 930);
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
    assert_eq!(
        read(r.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        116
    );
    let closeout = json!({"status":"composed CPU implementation complete SDK study closed","full_gate_passed":a["full_gate_passed"],"production_sources_checked_unchanged":170,"changed_candidate_originals":4,"unchanged_candidate_originals":166,"new_private_module":true,"native_core_tests":116,"raw_tests":2,"cubic_tests":6,"field_source_qualification_inherited":true,"closing_pins_checked":pins.as_object().unwrap().len(),"SDK_requests":22260,"independent_gnark_exports":465,"proof_public_members":930,"next_prior_count":9060,"original8_CPU10x_rows":a["original8_CPU10x_rows"],"profiles":6,"adopted":false,"goal_achieved":false});
    save(r.join("closeout.json"), &closeout);
    let mut md=format!("# Combined borrowed G1, adaptive G2 and Poseidon formulas\n\nFull performance gate: **{}**. Supplied CPU10× rows: **{}/8**. Accepted production remains unchanged; this report adopts no source. Small reliable gains have no minimum cutoff.\n\nThis is a new combination of three previously measured implementations. [Borrowed G1 operands](../2026-10-26-borrowed-g1/README.md) improved primary32 but failed physical16 nonregression. [Adaptive G2 records](../2026-10-27-adaptive-tree/README.md) retained primary and large-shape gains but failed calibration and4×2 checks. [Direct width-three Poseidon](../2026-10-27-poseidon-direct-dot/README.md) retained positive estimates but failed physical16 qualification. Those study verdicts remain unchanged. This prospective combination is neither an unchanged rerun nor a waiver of their failures. The rejected two-group G1 reverse overlap is absent.\n\n## Implementation and memory\n\nG1 pending and collision records borrow existing point slices through safe synchronous lifetimes, retaining the original owned fallback and nativeIFMA/deferred-sign/packed/direct plus geometry guards. G2 tree layers use the earlier fixed32actual-ordinary-pair rule with prefix/norm records; other layers retain original arithmetic. Width-three recognized Poseidon rows use the existing fused two-product operation and an ordinary third product; all other widths and original buffer writes remain. Field primitives, point order, tree/query/task topology, pool, key/image format and fresh blinding remain unchanged.\n\nThe composition changes four of170originals and adds one private module. Reversing every recorded patch reproduces all170accepted files byte-for-byte. Borrowed production arithmetic is checked against its parent after removing two experimental selector checks and formatting; no public probe or diagnostic reference copy is added. Forecasts conservatively retain the original owned-window payload estimates and explicitly charge the additional vector headers using checked arithmetic. There is no new point table, copied key, duplicate solver plan, proof/witness cache or circuit/core selector.\n\n[Memory argument](artifacts/record-memory-argument.md) separates retained payload bounds from allocator/transient/process peaks. The G2 retained two-buffer payload fits the prior record bound under the pinned implementation; its extra header is separately charged. Borrowed entries are16bytes versus72owned on64-bit builds and omit unused scalar arrays, without taking that saving as lower admission charges. None of these layout facts is a minimum-RAM, universal pressure or process-RSS guarantee.\n\n## Verified request latency\n\nRyzen9950X, fifteen rotated/reversed fresh-process blocks over all nine shapes at32workers plus primary16 on CPUs0–15. Exact accepted and exact reused fresh baseline SDKs retain source/library/consumer/binary pins; only the combined candidate is newly built with the standard native fat-LTO/one-codegen-unit driver. Each32-worker process runs31timed fresh SDK requests after3primes; physical16 runs151after3. Timed work includes witness solving, fresh blinding/proving, SDK verification and serialization, excluding loading/primes. All450latency processes precede six separate profiles. There is no short screen, extension, successful repeat, exclusion or postcapture source change.\n\nTimes are medians of process medians; ratios, savings and intervals use within-block observations. Ratios/differences of displayed medians need not match paired estimates. RSS is the maximum lifetime process peak across blocks, including loading/primes; CPU includes coordination.\n\n|Case|Accepted ms|Fresh ms|Combined ms|Paired /accepted|Individual median interval|Wins /accepted|Paired saving µs|Accepted / combined RSS MiB|Combined CPU ms/request|\n|---|---:|---:|---:|---:|---|---:|---:|---:|---:|\n",a["full_gate_passed"],a["original8_CPU10x_rows"]);
    let p = read(r.join("performance/protocol.json"));
    for config in p["cases"].as_array().unwrap() {
        let name = config["name"].as_str().unwrap();
        let c = &a["cases"][name];
        let m = &c["modes"];
        let q = &c["pairs"]["composed/accepted"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|[{:.6}, {:.6}]|{}/15|{:.3}|{:.2} / {:.2}|{:.3}|",
            m["accepted"]["ms"].as_f64().unwrap(),
            m["fresh"]["ms"].as_f64().unwrap(),
            m["composed"]["ms"].as_f64().unwrap(),
            q["median_ratio"].as_f64().unwrap(),
            q["median_confidence"]["lower"].as_f64().unwrap(),
            q["median_confidence"]["upper"].as_f64().unwrap(),
            q["wins"],
            q["saving_us"].as_f64().unwrap(),
            m["accepted"]["maxrss_mib"].as_f64().unwrap(),
            m["composed"]["maxrss_mib"].as_f64().unwrap(),
            m["composed"]["cpu_ms_per_request"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\n## Frozen qualification\n\nIndividual distribution-free median intervals have at least95% nominal coverage under independent/exchangeable block assumptions. Fifteen-block endpoints4..12 have96.484375% coverage under those assumptions; no simultaneous/familywise guarantee is claimed. Fifteen blocks were specified before captures for this distinct implementation, not added to closed data. Fresh/accepted calibration intervals must lie wholly within factor1.03 in all10cases. Combined nonregression upper bounds must be≤1.02 against both controls in all10. Primary32 strict gain upper bounds must be<1 against both. Original-eight within-block geometric aggregate upper bounds must be≤1 against both. Primary16 requires nonregression only. No positive-gain floor or failed-bound waiver is used. Unmet bounds can reflect insufficient precision and do not by themselves prove a causal slowdown. Performance success still requires resource/pressure/compatibility qualification before adoption.\n\n");
    writeln!(md,"Calibration failures: `{}`.\n\nNonregression failures: `{}`.\n\nPrimary gain failures: `{}`.\n\nOriginal-eight aggregate failures: `{}`.\n\nOriginal-eight aggregate intervals: `{}`.\n",a["calibration_failures"],a["regressions"],a["primary_gain_failures"],a["aggregate_failures"],a["original8"]).unwrap();
    md.push_str("[Analysis](artifacts/analysis.json) retains every paired ratio, saving, interval, block and failure. Positive estimates remain alongside adverse cases.\n\n## Supplied M5 references\n\nHistorical supplied CPU/Metal figures are not fresh Apple measurements. No54→1reference was supplied. Candidate values are experimental.\n\n|Shape|Combined ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for config in p["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let c = &a["cases"][config["name"].as_str().unwrap()];
        let ms = c["modes"]["composed"]["ms"].as_f64().unwrap();
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
    md.push_str("\n## Correctness and scope\n\nAll116native core tests pass, including retained independent borrowed G1 signed-stream/ordinary/KH generator, owned-control, collision, tail, dispatch and reuse oracles; G2 complete-curve/boundary/retained-capacity checks; and all-width Poseidon symbolic/full-buffer/error comparisons. Raw2/cubic6, generator and portable/no-IFMA consumer checks pass. Field source and its accepted qualification are unchanged. The clean selectors, combined source and header forecasts are tested together. Initial source-audit preparation stopped before any native build on formatting normalization and an extraction newline; the archived setup revisions were repaired before native testing/captures, without changing arithmetic.\n\nAll nine candidate shapes pass four-worker preflight, one request after three primes:36SDKcalls/nine fresh independently Gnark-verified exports. The fixed latency matrix adds20,700calls/450exports. After all latency, primary32 and4×2 profiles run all three variants,251requests after3primes:1,524calls/six exports. Total **22,260SDK calls**, **465distinct independently verified exports**, **930proof/public members**; ledger **8,595→9,060**. Every export has164-byte proof length, pinned public binding, freshness against all prior/new identities and independent Gnark verification; every SDK call also verifies. Subinventories are not counted twice.\n\nSix warm199Hzcycles:u profiles follow all latency, each sampling one own process with zero lost records. [Rows](artifacts/performance/rows.json) retain decoded categories, symbols and process receipts; raw profiles stay local with hashes. Period shares are not removable proof-wall fractions or cache/allocator causes. No helper/build/hash/archive/verifier overlaps a timed capture. Closing audits rehash all source/library/consumer/helper/tool/control pins and proof/public members. All170production sources and the exact accepted SDK remain unchanged.\n\nLifetime RSS is not incremental workspace or minimumRAM. Requested-capacity arguments do not establish transient allocator, stack, cgroup, giant-key or mobile bounds. No new AWS/Aeglos, phone, TEE,193GiB,swap,pressure,concurrent-caller,throughput or minimum-RAM qualification is claimed by this latency report.\n\n");
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
    md.push_str("[Design](artifacts/design.md), [source recovery](artifacts/source-audit.json), [composition edits](artifacts/composition-edits.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!(
        "Closed{}hashes,22260SDK465Gnark;prior9060;fullgate{};CPU10x{}/8",
        index.len(),
        a["full_gate_passed"],
        a["original8_CPU10x_rows"]
    );
}

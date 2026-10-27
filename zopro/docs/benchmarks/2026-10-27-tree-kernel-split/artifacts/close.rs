use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-tree-kernel-split";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-tree-kernel-split";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["SDK_requests"], 18628);
    assert_eq!(a["next_prior_count"], 8310);
    let p = read(r.join("performance/protocol.json"));
    for (f, h) in p["pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let m = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(f)),
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
        111
    );
    assert_eq!(
        read(r.join("codegen-audit.json"))["split_structure_observed"],
        true
    );
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 754);
    for (f, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert_eq!(fs::metadata(f).unwrap().len(), 164);
        }
    }
    let close = json!({"status":"const-specialized ordinary-pair SDK study complete","full_gate_passed":a["full_gate_passed"],"threshold_actual_pairs":32,"native_core_tests":111,"new_mirror_tests":0,"raw_tests":2,"cubic_tests":6,"field_source_qualification_inherited":true,"split_codegen_observed":true,"original_files":170,"unchanged_candidate_originals":169,"new_private_module":true,"production_sources_checked_unchanged":170,"closing_pins_checked":p["pins"].as_object().unwrap().len(),"SDK_requests":18628,"independent_gnark_exports":377,"proof_public_members":754,"next_prior_count":8310,"profiles":8,"original8_CPU10x_rows":a["original8_CPU10x_rows"],"adopted":false,"goal_achieved":false});
    save(r.join("closeout.json"), &close);
    let mut md=format!("# Specialized ordinary-pair tree kernels\n\nComplete SDK gate passes: **{}**. Supplied CPU10× rows: **{}/8**. No change is adopted; accepted production remains unchanged. Small reliable gains have no minimum cutoff.\n\nThe candidate preserves the [parent adaptive rule](../2026-10-27-adaptive-tree/README.md): native G2 layers use prefix records from32 actual ordinary pairs, otherwise the original kernel. Pair selection and complete exceptional compaction remain in the caller. Ordinary-pair forward/inverse/reverse/scalar-tail work moves into a private module with a compile-time boolean, yielding separately compiled kernels. The threshold, formulas, topology, tile size, data order, storage and scheduler remain fixed.\n\nThere are no inline annotations, new LLVM/ISA flags, key tables, execution plans, pools, point-input copies, runtime experiment flags or circuit/core selectors. Reversing the caller edits recovers the exact parent; arithmetic tokens differ only in predicate spelling and function boundary. The prior audit recovers accepted source. One original file changes and one private module is added;169 originals and all170 production files remain unchanged. Existing111 core tests cover the moved work, so no tests mirroring the factor are added.\n\n## Verified request latency\n\nRyzen9950X, nine rotated/reversed fresh-process blocks over all nine shapes at32workers plus primary16 on CPUs0–15. Controls are the exact accepted SDK, exact reused fresh original baseline and exact parent adaptive SDK; their source/library/consumer/binary pins are rechecked, with no new control build claimed. The new candidate uses the same compiler/native flags/fat LTO/one code-generation unit/standard SDK driver.\n\nEach32-worker process runs31 timed complete requests after3 primes; primary16 runs151 after3. Timed work includes witness construction, fresh blinded proving, SDK verification and serialization; loading/primes are excluded. All360 latency processes precede eight separate warm199Hz profiles, both primary32 and4×2, all four variants,251 timed calls after3 primes. There is no short screen, extension, threshold revision, exclusion or repeated successful capture.\n\nTimes are medians of process medians; ratios and microsecond savings pair within blocks. Lifetime RSS includes loading/primes and is not minimum RAM. CPU includes coordination.\n\n|Case|Accepted ms|Fresh ms|Parent ms|Split ms|Split/accepted|Split/fresh|Split/parent|Paired saving vs accepted µs|Split RSS MiB|Split CPU ms/request|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",a["full_gate_passed"],a["original8_CPU10x_rows"]);
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
        let m = &c["modes"];
        let q = &c["pairs"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.6}|{:.3}|{:.2}|{:.3}|",
            m["accepted"]["ms"].as_f64().unwrap(),
            m["fresh"]["ms"].as_f64().unwrap(),
            m["parent"]["ms"].as_f64().unwrap(),
            m["split"]["ms"].as_f64().unwrap(),
            q["split/accepted"]["geometric_ratio"].as_f64().unwrap(),
            q["split/fresh"]["geometric_ratio"].as_f64().unwrap(),
            q["split/parent"]["geometric_ratio"].as_f64().unwrap(),
            q["split/accepted"]["saving_us"].as_f64().unwrap(),
            m["split"]["maxrss_mib"].as_f64().unwrap(),
            m["split"]["cpu_ms_per_request"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\n## Frozen qualification\n\nIndividual distribution-free median intervals have at least95% coverage under independent/exchangeable block assumptions. With nine blocks, endpoints2..8 give96.09375% nominal coverage under those assumptions; no simultaneous/familywise guarantee is claimed. All observations remain and determine the ranks.\n\nFresh/accepted calibration intervals must lie wholly within factor1.03 for every case. Split nonregression upper bounds must be≤1.02 against accepted, fresh and parent in every case. Fixed24×2 and54→1 gain upper bounds must be<1 against accepted/fresh. Original-eight within-block geometric aggregate upper bounds must be≤1 against both original controls. Primary must pass nonregression; the eventual all-eight CPU10× goal is unchanged. Any performance pass still requires resource/pressure/compatibility review before adoption.\n\n");
    for key in [
        "calibration_failures",
        "regressions",
        "large_gain_failures",
        "aggregate_failures",
    ] {
        writeln!(md, "{key}: `{}`.\n", a[key]).unwrap();
    }
    md.push_str("[Analysis](artifacts/analysis.json) retains every block ratio, interval, estimate and failure. Crossing a bound indicates insufficient support for the required limit and does not by itself establish a causal slowdown. No adverse case is dropped or gate relaxed.\n\n## Supplied M5 references\n\nHistorical supplied CPU/Metal values, not fresh Apple measurements. No54reference was supplied. Experimental candidate values follow.\n\n|Shape|Split ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
        let ms = c["modes"]["split"]["ms"].as_f64().unwrap();
        if let (Some(cpu), Some(metal)) = (
            c["reference"]["cpu_ms"].as_f64(),
            c["reference"]["metal_ms"].as_f64(),
        ) {
            writeln!(
                md,
                "|{}|{ms:.6}|{cpu:.0}|{:.3}×|{metal:.0}|{:.3}×|{:.1}|",
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
    md.push_str("\n## Code generation, correctness and memory\n\nBefore any SDK capture, the actual split G2 caller is2,531bytes and references both outlined helper instantiations: original49,309bytes and prefix50,576bytes. The parent combined G2 function is81,580bytes. Total code size is larger; an individual selected kernel is smaller. This confirms structure, not fewer dynamic instructions, fewer cache misses or latency causality. The unchanged G1 choice was inlined by the compiler. [Code-generation audit](artifacts/codegen-audit.json), symbol listings and caller disassembly retain addresses, sizes and binary hashes. No annotations force this layout.\n\nAll111 native core tests, raw2/cubic6, generator and portable/no-IFMA consumers pass. They include the independent complete-curve tree oracle and the31/32/33 boundary plus growing/shrinking scratch-capacity tests. Formula/scalar-tail/compaction bodies remain unchanged. Field source and accepted qualification are inherited unchanged. No component probe or recorder is linked into SDK proving.\n\nThe factor adds no allocations or record storage. Both inherited record buffers can retain capacity; the existing test checks actual combined retained record bytes through its sequence against the original maximum exact Group bound. That finite coverage is not a universal allocator proof or process-RAM bound. Extra metadata, task stacks, allocator retention and pressure behavior still need formal qualification before adoption. Lower code/record bytes do not imply a universal RSS benefit or minimum RAM.\n\nPreflight covers all nine candidate circuits at4workers, one timed call after3 primes:36 SDK calls/nine Gnark exports. Matrix:16,560 SDK calls/360 exports. Eight profiles:2,032 calls/eight exports. Every exported proof passes independent verification,164-byte length, pinned public binding and freshness against7,933 prior identities plus new exports.\n\nTotal **18,628 SDK calls**, **377 distinct independently verified exports**, **754 proof/public members**; proof ledger **7,933 → 8,310**. All170 production files and the exact accepted SDK remain unchanged. The original all-eight CPU10× objective remains active and unproven.\n\nProfiles are automatic and follow all latency. Every capture samples one own process with zero lost records. Decoded categories/symbols and resource receipts remain in [rows](artifacts/performance/rows.json). Sampled periods do not give removable wall fractions or establish cache causality. No helper/build/hash/archive/verifier overlaps a timed capture. Closing checks rehash all pins and proof members. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed.\n\n");
    for v in p["variants"].as_array().unwrap() {
        let binary = v["binary"].as_str().unwrap();
        writeln!(
            md,
            "{} executable: `{binary}`; SHA-256 `{}`.\n",
            v["name"].as_str().unwrap(),
            hash(binary)
        )
        .unwrap();
    }
    md.push_str("[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    fs::copy(
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        r.join("accepted-source-manifest.json"),
    )
    .unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    for (f, h) in &index {
        assert_eq!(&hash(d.join("artifacts").join(f)), h);
    }
    for (f, h) in inventory.as_object().unwrap() {
        assert_eq!(
            hash(
                d.join("artifacts")
                    .join(Path::new(f).strip_prefix(r).unwrap())
            ),
            h.as_str().unwrap()
        );
    }
    save(d.join("artifact-index.json"), &json!(index));
    println!(
        "{}archivedhashes/754proofmembers checked;18628SDK377Gnark;prior8310;noadoption",
        index.len()
    );
}
fn archive(root: &Path, here: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    let mut files: Vec<_> = fs::read_dir(here)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    for f in files {
        let rel = f.strip_prefix(root).unwrap();
        let s = rel.display().to_string();
        if s.starts_with("target-") {
            continue;
        }
        if f.is_dir() {
            archive(root, &f, dest, index);
            continue;
        }
        if s.starts_with("candidate/")
            && ![
                "candidate/crates/zopro-core/src/msm.rs",
                "candidate/crates/zopro-core/src/msm/pairs.rs",
            ]
            .contains(&s.as_str())
        {
            continue;
        }
        if !["json", "jsonl", "rs", "md", "txt", "log", "proof", "public"]
            .iter()
            .any(|x| f.extension().is_some_and(|e| e == *x))
        {
            continue;
        }
        let target = dest.join(rel);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(&f, &target).unwrap();
        let h = hash(&target);
        assert_eq!(h, hash(&f));
        index.insert(s, h);
    }
}

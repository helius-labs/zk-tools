use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-adaptive-tree";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-adaptive-tree";
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
    assert_eq!(a["SDK_requests"], 17612);
    assert_eq!(a["next_prior_count"], 7933);
    let p = read(r.join("performance/protocol.json"));
    for (f, h) in p["pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (f, h) in manifest["source_files_sha256"].as_object().unwrap() {
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
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 746);
    for (f, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert_eq!(fs::metadata(f).unwrap().len(), 164);
        }
    }
    let close = json!({"status":"adaptive G2 layer records complete verified SDK study closed","threshold_actual_ordinary_pairs":32,"full_gate_passed":a["full_gate_passed"],"native_core_tests":111,"raw_tests":2,"cubic_tests":6,"field_source_qualification_inherited":true,"source_originals":170,"unchanged_candidate_originals":169,"production_sources_checked_unchanged":170,"controls_reused_unchanged":3,"closing_pins_checked":p["pins"].as_object().unwrap().len(),"SDK_requests":17612,"independent_gnark_exports":373,"proof_public_members":746,"next_prior_count":7933,"profiles":4,"original8_CPU10x_rows":a["original8_CPU10x_rows"],"adopted":false,"goal_achieved":false,"new_runtime_experiment_selector":false,"no_short_screen_or_extension":true});
    save(r.join("closeout.json"), &close);
    let mut md=format!("# Adaptive G2 tree-layer records\n\nThe complete SDK performance gate passes: **{}**. CPU10× rows among the eight supplied shapes: **{}/8**. No change is adopted by this report; production remains unchanged. Small reproducible gains have no minimum cutoff.\n\nThe candidate chooses prefix/norm records for native G2 layers with at least32 ordinary affine pairs and the original kernel for smaller layers. This threshold was fixed by the preceding [training/heldout component](../2026-10-27-tree-record-crossover/README.md), before any current SDK capture. It does not depend on circuit identity, core count or current timing results. The [unconditional prefix candidate](../2026-10-27-g2-tree-records/README.md) remains a separate, frozen comparator with its prior global failure and retained large-shape positives.\n\nOne guard is computed after pair selection and controls the existing forward, record-reservation and reverse branches. Field formulas, scalar tails, complete exceptional-pair compaction, tree topology, tile size, point order, query dispatch and scheduling remain unchanged. G1 and portable/no-IFMA paths retain the original kernel. Only `msm.rs` changes; reversing the new edits recovers the prior candidate, whose existing audit recovers accepted source. The other169 originals are exact. There is no new table, plan, pool, point-input copy, field primitive or runtime experiment selector.\n\n## Complete verified request latency\n\nRyzen9950X. Nine rotated/reversed fresh-process blocks over all nine circuits at32workers plus primary16 on CPUs0–15. Four variants: exact accepted SDK, reused fresh original baseline, reused unconditional prefix candidate, new adaptive candidate. Their source/library/driver/binary pins are checked; no new control builds are claimed. All use the same native flags, standard SDK driver, fat LTO and one code-generation unit.\n\nEvery timed call includes witness work, fresh blinded proving, SDK verification and serialization. Loading and three primes are excluded. Each32-worker process has31 timed calls; primary16 has151. All360 latency processes precede four separate warm-gated199Hz primary32 profiles,251 timed calls after3 primes each. There is no short screen, conditional extension, successful capture repeat, threshold revision or case exclusion.\n\nDisplayed times are medians of process medians. Ratios pair blocks geometrically; savings pair block medians and may differ from subtracting displayed times. CPU includes coordination; lifetime RSS includes loading/primes and is not minimum RAM.\n\n|Case|Accepted ms|Fresh ms|Unconditional ms|Adaptive ms|Adaptive/accepted|Adaptive/fresh|Adaptive/unconditional|Paired saving vs accepted µs|Adaptive RSS MiB|Adaptive CPU ms/request|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",a["full_gate_passed"],a["original8_CPU10x_rows"]);
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
            m["tree"]["ms"].as_f64().unwrap(),
            m["adaptive"]["ms"].as_f64().unwrap(),
            q["adaptive/accepted"]["geometric_ratio"].as_f64().unwrap(),
            q["adaptive/fresh"]["geometric_ratio"].as_f64().unwrap(),
            q["adaptive/tree"]["geometric_ratio"].as_f64().unwrap(),
            q["adaptive/accepted"]["saving_us"].as_f64().unwrap(),
            m["adaptive"]["maxrss_mib"].as_f64().unwrap(),
            m["adaptive"]["cpu_ms_per_request"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\n## Frozen qualification\n\nThe rule uses individual distribution-free median intervals with at least95% coverage under independent/exchangeable block assumptions. For nine blocks, endpoints are the second-smallest and second-largest ratios, nominal coverage96.09375% under those assumptions. This is not a simultaneous/familywise guarantee. All observations remain and determine the order statistics.\n\nCalibration requires fresh/accepted intervals wholly within factor1.03 for every case. Adaptive nonregression requires upper bounds≤1.02 against accepted, fresh and unconditional in every case. Fixed24×2 and54→1 gain checks require upper bounds<1 against accepted and fresh, to preserve the earlier large-shape signal without a shape-based dispatcher. The original-eight within-block geometric aggregate requires an upper bound≤1 against accepted and fresh. This structural change need not strictly improve primary in isolation; primary and every other case must pass nonregression, while the original eventual all-eight CPU10× objective remains intact. Any passing performance result still needs resource/pressure/compatibility review before adoption.\n\n");
    for key in [
        "calibration_failures",
        "regressions",
        "large_gain_failures",
        "aggregate_failures",
    ] {
        writeln!(md, "{key}: `{}`.\n", a[key]).unwrap();
    }
    md.push_str("[Full analysis](artifacts/analysis.json) retains every block, ratio, interval, saving and failure. Crossing an interval bound indicates insufficient support for the required limit; it is not by itself a causal slowdown measurement. No adverse cell is removed and no gate is relaxed.\n\n## Supplied M5 CPU and Metal comparisons\n\nHistorical references supplied by the user, not remeasured. No54reference was supplied. Rows are experimental candidate measurements.\n\n|Shape|Adaptive ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
        let ms = c["modes"]["adaptive"]["ms"].as_f64().unwrap();
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
    md.push_str("\n## Correctness, memory and profiling\n\nAll111 native core tests, two raw tests, six cubic tests, generator checks and portable/no-IFMA consumers pass. The existing240-case independent complete-curve layer oracle remains, with its unconditional old-buffer assertion adapted for the new selection. The new test covers31/32/33 ordinary pairs and a growing/shrinking sequence through2048, both inverse modes, every output/count, and retained buffer capacities. Field source and accepted qualification are inherited unchanged.\n\nBoth record vectors can retain allocations as layer sizes change. Native G2 below the threshold needs at most three old Group records; larger layers use two-vector prefix records instead of nine-vector Group records. The new test checks the actual combined retained record bytes against the prior maximum exact Group bound through its sequence. This finite coverage is not a universal allocator proof, process-RAM limit or minimum-RAM claim. The additional empty Vec header and struct padding, task metadata, allocation margins and pressure behavior still need formal review before adoption.\n\nAll nine adaptive circuits pass four-worker preflight, one timed call after3 primes:36 SDK calls and nine independent Gnark exports. The matrix adds16,560 SDK calls/360 exports, and profiles add1,016 calls/four exports. Every exported proof passes independent verification,164-byte length, pinned public binding and freshness against7,560 prior identities plus new exports.\n\n");
    writeln!(md,"Total **17,612 SDK calls**, **373 distinct independently verified exports**, **746 proof/public members**; proof ledger **7,560 → 7,933**. All170 production files and the accepted SDK remain unchanged. The all-eight CPU10× objective remains active and unproven.\n").unwrap();
    md.push_str("Profiles are collected automatically after all latency. Each capture has one sampled process and zero lost records; decoded categories and symbols remain in [rows](artifacts/performance/rows.json). Sampled cycle shares do not give removable wall-time fractions or isolate cache/memory causality. No helper, build, hash, archive or verifier overlaps a timed process. Closing checks rehash all pins and proof members. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed here.\n\n");
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
        "{}archivedhashes/746proofmemberschecked;17612SDK373Gnark;prior7933;noadoption",
        index.len()
    );
}
fn archive(root: &Path, here: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    let mut paths: Vec<_> = fs::read_dir(here)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for f in paths {
        let rel = f.strip_prefix(root).unwrap();
        let s = rel.display().to_string();
        if s.starts_with("target-") {
            continue;
        }
        if f.is_dir() {
            archive(root, &f, dest, index);
            continue;
        }
        if s.starts_with("candidate/") && s != "candidate/crates/zopro-core/src/msm.rs" {
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

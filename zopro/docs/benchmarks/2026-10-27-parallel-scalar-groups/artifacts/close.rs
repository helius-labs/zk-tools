use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-parallel-scalar-groups";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-parallel-scalar-groups";
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
fn archive(root: &Path, dir: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let t = e.file_type().unwrap();
        let rel = p.strip_prefix(root).unwrap();
        let s = rel.to_str().unwrap();
        if t.is_dir() {
            if !e.file_name().to_str().unwrap().starts_with("target") {
                archive(root, &p, dest, index)
            }
        } else if t.is_file() {
            if (s.starts_with("fresh/") || s.starts_with("candidate/"))
                && !s.ends_with(".xtask/source-pins.json")
                && !s.ends_with(".xtask/snapshot.json")
                && s != "candidate/crates/zopro-core/src/msm_affine/grouping.rs"
                && s != "candidate/crates/zopro-core/src/msm_affine/grouping/parallel.rs"
            {
                continue;
            }
            if !["rs", "md", "json", "jsonl", "txt", "log", "proof", "public"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            {
                continue;
            }
            let o = dest.join(rel);
            fs::create_dir_all(o.parent().unwrap()).unwrap();
            fs::copy(&p, &o).unwrap();
            assert_eq!(hash(&p), hash(&o));
            assert!(index.insert(s.into(), hash(&o)).is_none());
        }
    }
}
fn n(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["SDK_requests"], 27660);
    assert_eq!(a["independent_exports"], 465);
    assert_eq!(a["next_prior_count"], 11110);
    assert_eq!(
        read(r.join("performance/result.json"))["status"],
        "complete"
    );
    for (p, h) in read(r.join("performance/protocol.json"))["pins"]
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    let sources = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(sources.len(), 171);
    let edits = read(r.join("edits.json"));
    let changed = edits["file"].as_str().unwrap();
    for (p, h) in sources {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        let candidate = r.join("candidate").join(p);
        if p == changed {
            let new = fs::read_to_string(candidate).unwrap();
            let recovered = new
                .replace(
                    edits["after"].as_str().unwrap(),
                    edits["before"].as_str().unwrap(),
                )
                .replace(
                    edits["module_after"].as_str().unwrap(),
                    edits["module_before"].as_str().unwrap(),
                );
            assert_eq!(
                recovered,
                fs::read_to_string(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)).unwrap()
            );
        } else {
            assert_eq!(hash(candidate), h.as_str().unwrap());
        }
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 930);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let added = edits["added_file"].as_str().unwrap();
    assert_eq!(
        hash(r.join("candidate").join(added)),
        edits["added_file_sha256"].as_str().unwrap()
    );
    for mode in ["fresh", "candidate"] {
        assert_eq!(
            read(r.join(format!("native/{mode}-core/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            if mode == "candidate" { 119 } else { 116 }
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
        .filter(|x| x["phase"] == "profile")
        .collect();
    assert_eq!(profiles.len(), 6);
    let mut raw = BTreeMap::new();
    let mut samples = 0u64;
    for row in &profiles {
        assert_eq!(row["profile"]["lost_records"], 0);
        assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
        samples += row["profile"]["samples"].as_u64().unwrap();
        let p = r
            .join("performance/profile/b0")
            .join(row["case"].as_str().unwrap())
            .join(row["variant"].as_str().unwrap())
            .join("perf.data");
        raw.insert(p.display().to_string(), hash(p));
    }
    save(r.join("raw-local-only-sha256.json"), &json!(raw));
    let primary = &a["cases"]["1x2-w32"];
    let p = &primary["pairs"]["candidate/accepted"];
    let pf = &primary["pairs"]["candidate/fresh"];
    let ci = &p["median_confidence"];
    let mut md=format!("# Parallel exact scalar grouping\n\nComplete SDK performance gate: **{}**. The private candidate takes **{:.6} ms** on primary, versus **{:.6} ms accepted** and **{:.6} ms fresh**. Paired candidate/accepted is **{:.9}**, individual median interval **[{:.9}, {:.9}]**, {}/15 wins, with a median paired saving of **{:.3} µs**. Candidate/fresh is **{:.9}**. Positive estimates are retained without a gain floor. Displayed medians and paired estimates have different aggregation.\n\nThe candidate is not adopted in this experiment. The exact accepted 171-file implementation and local 824164 SDK remain unchanged. The original all-eight supplied M5 CPU 10× goal remains active, achieved {}/8.\n\n## Change\n\nThe candidate stably partitions nonzero scalar indices by their low three bits and builds eight disjoint exact maps in the existing Rayon pool. Each map records the original first index. Summed cardinality preserves the existing cutoff; reconstruction traverses original positions to reproduce the exact values, counts, starts and stable input-index lists. The original sample, zero-aware policy and initial-capacity rule remain. Small inputs and single-worker pools retain the original code. No point arithmetic, scalar representation, bucket geometry, key preparation, queue implementation or persistent table changes. One original file changes and one private module is added; the other 170 originals are exact. Inverse edits recover all 171 production files.

The temporary index/representative buffer uses eight bytes per nonzero input. Before allocating it, checked bounds include the sample set, all map-growth envelopes, descriptors and reconstruction arrays; both peaks must fit the existing grouping reservation. Otherwise the untouched reservation returns to the serial fallback. Maps drop before reconstruction and pairs before final list allocation. Consumer layout and inherited bounds remain. This is no universal RSS bound; allocator retention, stacks, concurrent callers and actual pressure still need qualification. See memory-review.md. There is no new unsafe code, runtime flag, ISA setting, worker pool, synthetic universal cap or shard-count sweep.\n\nRetained stage recordings showed serial primary A/G2 grouping maps that ultimately decline. This prototype targets their critical serial work rather than task-search policy. It differs from earlier exact-prefix/bitmap/borrowed-key trials; their original outcomes remain. No new C7a latency or causal prediction follows from those older recordings. The reviewed thesis clustering mechanism overlaps an already unfavorable search census and supplies no new performance claim here.\n\n## Full SDK results\n\nRyzen 9950X, fifteen fixed rotated/reversed fresh-process blocks. All nine shapes use 32 workers on CPUs 0–31; primary also uses 16 workers on physical CPUs 0–15. Both primary cases use 151 requests after three primes, other cases 31 after three. All 450 latency processes finish before six separate warm profiles. Loading/setup/primes are excluded from latency and included in lifetime RSS. Fresh solving, randomness, proving, internal SDK verification and serialization are timed. No owned build, helper, hash, archive, independent verifier or second benchmark overlaps captures. No successful capture is excluded, extended or repeated.\n\nTimes are medians of process medians; ratios and savings pair blocks. RSS is the maximum whole-process observation, not minimum RAM. Individual intervals assume independent/exchangeable blocks and do not provide familywise coverage.\n\n|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval|Wins|Saving µs|RSS A/C MiB|\n|---|---:|---:|---:|---:|---|---:|---:|---:|\n",a["full_gate_passed"],n(&primary["modes"]["candidate"]["ms"]),n(&primary["modes"]["accepted"]["ms"]),n(&primary["modes"]["fresh"]["ms"]),n(&p["median_ratio"]),n(&ci["lower"]),n(&ci["upper"]),p["wins"],n(&p["saving_us"]),n(&pf["median_ratio"]),a["original8_CPU10x_rows"]);
    let names = [
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
    ];
    for name in names {
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
    md.push_str("\nAll fresh/control pairs, CPU observations and every block are retained in analysis.json. Original-eight paired geometric aggregates:\n\n|Comparison|Median ratio|Individual interval|\n|---|---:|---|\n");
    for control in ["accepted", "fresh"] {
        let key = format!("candidate/{control}");
        let c = &a["original8"][&key];
        writeln!(
            md,
            "|{key}|{:.9}|[{:.9}, {:.9}]|",
            n(&c["median"]),
            n(&c["lower"]),
            n(&c["upper"])
        )
        .unwrap();
    }
    md.push_str("\n## Frozen performance qualification\n\nEvery fresh/accepted individual median interval must lie wholly within factor 1.03. Candidate upper bounds must be ≤1.02 against both controls in every case. Primary32 must have upper <1 against both; the original-eight within-block geometric aggregate must have upper ≤1 against both. Fifteen blocks use ranks 4..12, with 96.484375% nominal individual coverage under independent/exchangeable-block assumptions. There is no minimum gain floor or post-result exclusion, threshold change or unchanged rerun. Known physical16 variability remains visible. The earlier adopted source's engineering exception is unchanged and does not waive this study's checks.\n\n");
    for key in [
        "calibration_failures",
        "regressions",
        "primary_gain_failures",
        "aggregate_failures",
    ] {
        writeln!(md, "{key}: `{}`.\n", a[key]).unwrap();
    }
    md.push_str("## CPU and profiles\n\nWarm user+system CPU is summed across threads per request. Profile shares are sampled user-cycle weights with inlining/attribution limits, not removable wall-time fractions or exclusive critical-path costs. Profiles occur after latency and do not replace its acceptance observations.\n\n|Case|Variant|CPU ms/request|Whole-process RSS MiB|\n|---|---|---:|---:|\n");
    for name in ["1x2-w32", "2x2-w32", "merge-54x1-w32", "1x2-w16"] {
        for mode in ["accepted", "fresh", "candidate"] {
            let c = &a["cases"][name]["modes"][mode];
            writeln!(
                md,
                "|{name}|{mode}|{:.3}|{:.2}|",
                n(&c["cpu_ms_per_request"]),
                n(&c["maxrss_mib"])
            )
            .unwrap();
        }
    }
    writeln!(md,"\nThe six separate 199 Hz profiles contain {samples} samples, zero lost records and one sampled SDK PID each. Each measures 251 requests after three primes.\n\n|Case|Variant|Samples|Profile CPU ms/request|Epoch %|Scheduler %|G1 %|G2 %|\n|---|---|---:|---:|---:|---:|---:|---:|").unwrap();
    for row in &profiles {
        let q = &row["profile"]["category_percent"];
        let pct = |k: &str| q[k].as_f64().unwrap_or(0.);
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|",
            row["case"].as_str().unwrap(),
            row["variant"].as_str().unwrap(),
            row["profile"]["samples"],
            (n(&row["summary"]["user_cpu_ms"]) + n(&row["summary"]["system_cpu_ms"]))
                / n(&row["summary"]["rounds"]),
            pct("epoch"),
            pct("scheduler"),
            pct("G1_arithmetic"),
            pct("G2_arithmetic")
        )
        .unwrap();
    }
    md.push_str("\n## Supplied M5 references\n\nHistorical supplied CPU/Metal values, not fresh Apple measurements. No 54→1 reference was supplied. Candidate values are experimental.\n\n|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for name in names.into_iter().take(8) {
        let c = &a["cases"][name];
        let ms = n(&c["modes"]["candidate"]["ms"]);
        let cpu = n(&c["reference"]["cpu_ms"]);
        let metal = n(&c["reference"]["metal_ms"]);
        writeln!(
            md,
            "|{}|{ms:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
            c["shape"].as_str().unwrap(),
            cpu / ms,
            metal / ms,
            cpu / 10.
        )
        .unwrap();
    }
    md.push_str("\n## Validation and scope\n\nFresh passes all 116 native core tests and candidate 119, including three new parallel-grouping tests over exact stable metadata, cutoff boundaries, adversarial shard populations, zero/full-word inputs, one/two/four-worker pools, nested/concurrent callers and checked memory/overflow fallback. Both pass two raw-image and six cubic tests; candidate portable/no-IFMA consumers pass. Existing complete-curve grouping oracles remain unfiltered. Field source and the accepted 33-test native field qualification are inherited, with no fresh field-suite claim. Ordinary RSS and the conservative workspace review do not establish minimum RAM, pressure or concurrent-server behavior.\n\nNine four-worker candidate preflights add 36 SDK calls/nine exports; latency adds 26,100/450; profiles 1,524/six. Total **27,660 SDK calls, 465 distinct independently Gnark-verified exports and 930 proof/public members**, ledger **10,645→11,110**. All proofs are 164 bytes, fixture-bound and fresh against all preceding/new exports; every SDK call verifies internally. Source, library, executable, input and proof pins are rehashed at closure.\n\nAll initial setup, build and observer receipts are retained. No successful capture, test or verifier is repeated as part of analysis or publication. Raw perf.data files remain local with hashes; privileged capture ownership is transferred to the experiment owner after measurement for reading, without altering bytes or mode.\n\nThis local study creates no AWS resources and makes no new Aeglos, phone, ARM, TEE, swap, minimum-RAM, pressure, concurrent-throughput or huge-circuit claim. Any performance pass still requires a separate compatibility/concurrency/resource review before adoption. The original all-eight CPU10× objective remains active.\n\n[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [raw rows](artifacts/performance/rows.json), [source edit](artifacts/edits.json), [native workflow](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [raw profile hashes](artifacts/raw-local-only-sha256.json), [closeout](artifacts/closeout.json), [artifact hashes](artifact-index.json).\n");
    save(
        r.join("closeout.json"),
        &json!({"status":"complete parallel scalar-grouping experiment","full_performance_gate_passed":a["full_gate_passed"],"new_source_adopted":false,"source_files_unchanged":171,"SDK_requests":27660,"distinct_independent_exports":465,"proof_public_members":930,"next_prior_count":11110,"profiles":6,"cycle_samples":samples,"owned_live_jobs":[],"new_AWS_resources":[],"original8_CPU10x_rows":a["original8_CPU10x_rows"],"goal_achieved":false}),
    );
    fs::create_dir_all(d.join("artifacts")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    println!("Closed parallel scalar grouping: {}hashes;27660SDK465Gnark;ledger11110;fullpass{};source unchanged",index.len(),a["full_gate_passed"]);
}

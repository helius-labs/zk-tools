use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-grouping-prefix-cutoff";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-grouping-prefix-cutoff";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
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
                && s != "candidate/crates/zopro-core/src/msm_affine/grouping.rs"
                && s != "candidate/crates/zopro-core/src/msm_affine/grouping/tests.rs"
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
fn n(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn readable_report(mut s: String) -> String {
    for (before, after) in [
        ("Other positive C/A estimates include4x227.862µs,8x262.879µs,16x2201.129µs,24x243.522µs,merge24222.771µs andmerge54106.468µs.", "Other positive C/A estimates are 27.862 µs (4×2), 62.879 µs (8×2), 201.129 µs (16×2), 43.522 µs (24×2), 222.771 µs (24→1 merge) and 106.468 µs (54→1 merge)."),
        ("RSS is mixed: primary134.50/135.25MiB,24x21302.91/1352.50MiB,54 1851.55/1872.32MiB.", "RSS is mixed, accepted/candidate: primary 134.50/135.25 MiB; 24×2 1302.91/1352.50 MiB; 54→1 1851.55/1872.32 MiB."),
        ("exact824164", "exact 824164"), ("accepted824164SDK", "accepted 824164 SDK"),
        ("all-eight supplied M5 CPU10×", "all-eight supplied M5 CPU 10×"), ("achieved0/8", "achieved 0/8"),
        ("other169accepted", "other 169 accepted"), ("all171accepted", "all 171 accepted"),
        ("minimumRAM", "minimum RAM"), ("Ryzen9950X", "Ryzen 9950X"),
        ("at32workers", "at 32 workers"), ("primary16 on CPUs0–15", "primary16 on CPUs 0–15"),
        ("run151requests", "run 151 requests"), ("other32-worker", "other 32-worker"),
        ("run31after", "run 31 after"), ("All450latency", "All 450 latency"),
        ("median0.999520", "median 0.999520"), ("interval[", "interval ["),
        ("],8/15wins", "], 8/15 wins"), ("C/fresh0.988795", "C/fresh 0.988795"),
        ("],15/15wins", "], 15/15 wins"), ("C/A0.998992", "C/A 0.998992"),
        ("C/fresh0.990165", "C/fresh 0.990165"), ("The2x2C/A", "The 2×2 C/A"),
        ("the2%nonregression", "the 2% nonregression"), ("Primary16has", "Primary16 has"),
        ("increases191.158→192.047ms/request", "increases 191.158→192.047 ms/request"),
        ("fresh192.875", "fresh 192.875"), ("ranks4..12 for15blocks,96.484375%nominal", "ranks 4..12 for 15 blocks, with 96.484375% nominal"),
        ("all10fresh/accepted", "all 10 fresh/accepted"), ("factor1.03", "factor 1.03"),
        ("in all10", "in all 10"), ("separate199Hzuser-cycle", "separate 199 Hz user-cycle"),
        ("contain84,224samples", "contain 84,224 samples"), ("No54M5reference", "No 54→1 M5 reference"),
        ("passes116unfiltered", "passes 116 unfiltered"), ("candidate117", "candidate 117"),
        ("checks100independent", "checks 100 independent"), ("raw2/cubic6prover", "two raw and six cubic prover"),
        ("portable/noIFMAconsumer", "portable/no-IFMA consumer"), ("contributes36SDKcalls/nine", "contributes 36 SDK calls/nine"),
        ("latency26,100calls/450exports", "latency 26,100 calls/450 exports"),
        ("profiles1,524calls/six", "profiles 1,524 calls/six"), ("27,660SDKcalls,465distinct", "27,660 SDK calls, 465 distinct"),
        ("930proof/public", "930 proof/public"), ("meet164-byte", "meet 164-byte"),
        ("rehash450pins,930", "rehash 450 pins, 930"), ("and171production", "and 171 production"),
        ("Accepted824164SDK", "Accepted 824164 SDK"), ("CPU10× goal remains active0/8", "CPU 10× goal remains active, 0/8"),
    ] { s = s.replace(before, after); }
    s
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["SDK_requests"], 27660);
    assert_eq!(a["independent_exports"], 465);
    assert_eq!(a["next_prior_count"], 9904);
    assert_eq!(a["full_gate_passed"], false);
    assert_eq!(
        read(r.join("performance/result.json"))["status"],
        "complete"
    );
    let pins = read(r.join("performance/protocol.json"))["pins"].clone();
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read(M);
    let sources = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(sources.len(), 171);
    for (p, h) in sources {
        assert_eq!(hash(Path::new(P).join(p)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(manifest["native_binary"].as_str().unwrap()),
        manifest["native_binary_sha256"].as_str().unwrap()
    );
    let edits = read(r.join("edits.json"));
    let f = edits["file"].as_str().unwrap();
    let candidate = fs::read_to_string(r.join("candidate").join(f)).unwrap();
    assert_eq!(
        candidate
            .replace(
                edits["after"].as_str().unwrap(),
                edits["before"].as_str().unwrap()
            )
            .replace(
                edits["new_insert"].as_str().unwrap(),
                edits["old_insert"].as_str().unwrap()
            ),
        fs::read_to_string(Path::new(P).join(f)).unwrap()
    );
    let t = edits["tests"].as_str().unwrap();
    let test_original = fs::read_to_string(Path::new(P).join(t)).unwrap();
    assert_eq!(
        fs::read_to_string(r.join("candidate").join(t)).unwrap(),
        test_original + &fs::read_to_string(r.join("oracle-test.rs")).unwrap()
    );
    let inv = read(r.join("proof-inventory.json"));
    assert_eq!(inv.as_object().unwrap().len(), 930);
    for (p, h) in inv.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for (mode, count) in [("fresh", 116), ("candidate", 117)] {
        assert_eq!(
            read(r.join(format!("native-v2/{mode}-core/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            count
        );
        assert_eq!(
            read(r.join(format!("native-v2/{mode}-prover/tests.json")))["passed_tests"]
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
        .filter(|v| v["phase"] == "profile")
        .collect();
    assert_eq!(profiles.len(), 6);
    let samples: u64 = profiles
        .iter()
        .map(|v| v["profile"]["samples"].as_u64().unwrap())
        .sum();
    assert_eq!(samples, 84224);
    for p in &profiles {
        assert_eq!(p["profile"]["lost_records"], 0);
        assert_eq!(p["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
    }
    let mut raw = BTreeMap::new();
    for line in fs::read_to_string(r.join("raw-profile-sha256.txt"))
        .unwrap()
        .lines()
    {
        let (h, p) = line.split_once("  ").unwrap();
        assert_eq!(h.len(), 64);
        assert!(raw.insert(p.to_owned(), h.to_owned()).is_none());
    }
    assert_eq!(raw.len(), 6);
    save(r.join("raw-local-only-sha256.json"), &json!(raw));
    let mut md=String::from("# Exact early rejection of scalar grouping\n\nThe candidate retains a **3.998 µs primary paired saving versus accepted (0.048%)**, with an interval including parity. It improves against the fresh control, but five fresh/accepted calibration bounds fail and the full performance gate is false. It is **not adopted**. Positive estimates remain reported; no minimum gain floor is applied.\n\nPrimary32 process-median latencies are **8.325779 ms accepted, 8.424946 ms fresh and 8.330548 ms candidate**. These displayed medians differ from within-block paired estimates. The accepted 171-file implementation and exact824164 SDK remain unchanged. The all-eight supplied M5 CPU10× objective remains active, achieved0/8.\n\n## Change\n\nThe existing scalar-grouping loop now computes floor(n*percent/100) without overflow and stops on the next vacant hash entry after that many distinct nonzero values have been seen. Distinct counts cannot decrease, so full traversal would necessarily reject the same input. Occupied values at the boundary remain valid. Initial sampling, exact equality/hash, cutoff, insertion order, zero handling, capacity/reservation and successful stable output packing remain unchanged. This adds a comparison per new distinct value on accepted inputs and may affect generated code. It is separate from earlier unsuccessful bitmap lower-bound and borrowed-key candidates.\n\nOnly grouping.rs changes in non-test code; one independent oracle is appended to grouping/tests.rs. The other169accepted files remain exact. Closing inverse edits recover all171accepted bytes. There is no new buffer, point/key table, persistent state, field primitive, runtime selector, ISA flag, unsafe code or scheduler change. Existing memory forecasts are retained conservatively; no universal RSS reduction or minimumRAM is established.\n\n## Complete SDK results\n\nRyzen9950X, fifteen fixed rotated/reversed fresh-process blocks, three variants, all nine shapes at32workers plus primary16 on CPUs0–15. Both primary cases run151requests after three primes; other32-worker cases run31after three primes. All450latency processes precede six separate warm profiles. Timed work includes fresh solving, randomness, proving, SDK verification and serialization. Loading/setup/primes are excluded from latency and included in lifetime RSS. There is no short screen, extension, excluded observation, successful repeat or source/flag adjustment after capture.\n\nTimes are medians of process medians; ratios, wins and savings use within-block pairs. RSS is the maximum whole-process peak across all blocks.\n\n|Case|Accepted ms|Fresh ms|Candidate ms|Paired C/A|Individual interval C/A|Wins|Paired saving µs|RSS A/C MiB|\n|---|---:|---:|---:|---:|---|---:|---:|---:|\n");
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
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|[{:.6}, {:.6}]|{}/15|{:.3}|{:.2}/{:.2}|",
            n(&c["modes"]["accepted"]["ms"]),
            n(&c["modes"]["fresh"]["ms"]),
            n(&c["modes"]["candidate"]["ms"]),
            n(&p["median_ratio"]),
            n(&p["median_confidence"]["lower"]),
            n(&p["median_confidence"]["upper"]),
            p["wins"],
            n(&p["saving_us"]),
            n(&c["modes"]["accepted"]["maxrss_mib"]),
            n(&c["modes"]["candidate"]["maxrss_mib"])
        )
        .unwrap();
    }
    md.push_str("\n|Case|Paired C/fresh|Individual interval C/fresh|Wins|CPU ms/request accepted/fresh/candidate|\n|---|---:|---|---:|---:|\n");
    for name in names {
        let c = &a["cases"][name];
        let p = &c["pairs"]["candidate/fresh"];
        writeln!(
            md,
            "|{name}|{:.6}|[{:.6}, {:.6}]|{}/15|{:.3}/{:.3}/{:.3}|",
            n(&p["median_ratio"]),
            n(&p["median_confidence"]["lower"]),
            n(&p["median_confidence"]["upper"]),
            p["wins"],
            n(&c["modes"]["accepted"]["cpu_ms_per_request"]),
            n(&c["modes"]["fresh"]["cpu_ms_per_request"]),
            n(&c["modes"]["candidate"]["cpu_ms_per_request"])
        )
        .unwrap();
    }
    md.push_str("\nPrimary C/A median0.999520, interval[0.996252,1.001717],8/15wins; C/fresh0.988795, interval[0.984105,0.991206],15/15wins. Original-eight paired aggregate C/A0.998992 with interval[0.998428,1.001136]; C/fresh0.990165 with interval[0.970950,0.993175]. The first comparison in each case includes parity. This preserves the small positive estimate without treating the fresh-control benefit as a demonstrated gain over accepted.\n\nOther positive C/A estimates include4x227.862µs,8x262.879µs,16x2201.129µs,24x243.522µs,merge24222.771µs andmerge54106.468µs. Their individual intervals include parity. The2x2C/A interval[1.000546,1.009451] is above parity, an observed slowdown in that comparison despite passing the2%nonregression bound. Primary16has a positive candidate displayed-median difference but a negative paired saving; keep the paired result. Primary CPU increases191.158→192.047ms/request versus accepted, while improving versus fresh192.875; there is no demonstrated whole-proof CPU-work reduction versus accepted.\n\nRSS is mixed: primary134.50/135.25MiB,24x21302.91/1352.50MiB,54 1851.55/1872.32MiB. These lifetime maxima do not establish minimumRAM, an architecture cap or universal memory regression/saving. The algorithm keeps original allocations/forecasts and can own fewer elements only on early rejection.\n\n## Frozen checks and profiles\n\nIndividual exact median intervals use ranks4..12 for15blocks,96.484375%nominal coverage under independent/exchangeable block assumptions, not familywise coverage. Requirements: all10fresh/accepted intervals within factor1.03; candidate upper≤1.02 against both controls in all10; primary32 upper<1 against both; original-eight aggregate upper≤1 against both. No failed bound is waived and no gain floor is introduced. Failure to bound regression is not alone proof of a causal slowdown.\n\n");
    for (label, key) in [
        ("Calibration failures", "calibration_failures"),
        ("Nonregression failures", "regressions"),
        ("Primary gain failures", "primary_gain_failures"),
        ("Aggregate failures", "aggregate_failures"),
    ] {
        writeln!(md, "{label}: `{}`.\n", a[key]).unwrap();
    }
    md.push_str("Six separate199Hzuser-cycle profiles contain84,224samples, one sampled SDK PID per capture and zero lost records. These runs follow all latency. CPU and period-weighted categories are descriptive, not exclusive wall fractions or evidence of cache, spill, frequency or per-entry causes. No local helper/build/hash/archive/independent-verifier/second benchmark overlaps timed capture.\n\n|Case|Variant|Samples|CPU ms/request|Grouping %|G1 %|Epoch %|\n|---|---|---:|---:|---:|---:|---:|\n");
    for row in &profiles {
        let s = &row["summary"];
        let cat = &row["profile"]["category_percent"];
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.3}|{:.2}|{:.2}|",
            row["case"].as_str().unwrap(),
            row["variant"].as_str().unwrap(),
            row["profile"]["samples"],
            (n(&s["user_cpu_ms"]) + n(&s["system_cpu_ms"])) / n(&s["rounds"]),
            n(&cat["grouping"]),
            n(&cat["G1_arithmetic"]),
            n(&cat["epoch"])
        )
        .unwrap();
    }
    md.push_str("\n## Supplied M5 comparison\n\nM5 CPU/Metal values are supplied historical references, not new Apple measurements. Candidate remains experimental. No54M5reference was supplied.\n\n|Shape|Candidate ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for name in &names[..9] {
        let c = &a["cases"][name];
        let time = n(&c["modes"]["candidate"]["ms"]);
        if let Some(cpu) = c["reference"]["cpu_ms"].as_f64() {
            let metal = n(&c["reference"]["metal_ms"]);
            writeln!(
                md,
                "|{}|{time:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
                c["shape"].as_str().unwrap(),
                cpu / time,
                metal / time,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(md, "|54→1|{time:.6}|—|—|—|—|—|").unwrap();
        }
    }
    md.push_str("\n## Correctness, provenance and decision\n\nFresh control passes116unfiltered native core tests; candidate117. The added test checks100independent ordered-map cases across both sample policies, cutoff boundaries, zeros, full-word keys, early/late duplicates and several input sizes. Decision, first-occurrence values, counts and stable indices agree. Existing independent curve/MSM, concurrency and fallback tests remain included. Both variants pass raw2/cubic6prover tests; candidate portable/noIFMAconsumer checks pass. Field source and qualification remain inherited; no new portable-unit, pressure, giant or physical-phone qualification is claimed here.\n\nThe initial workflow plan was rejected before any build because helper JSON mutation inserted a null argv key into link steps. The first repair helper asserted a nonexistent key before writes. Original sources/plan/logs are retained; corrected repair removes only that null key and routes receipt paths to the new native-v2 output. No successful test, build or capture was repeated for the repair; candidate source and flags remain fixed.\n\nPreflight contributes36SDKcalls/nine exports; latency26,100calls/450exports; profiles1,524calls/six exports. Total **27,660SDKcalls,465distinct independently Gnark-verified exports,930proof/public members**, ledger **9,439→9,904**. Every SDK call verifies internally; all final exports meet164-byte proof, exact public binding and freshness against all prior/new identities. Closing audits rehash450pins,930proof/public members and171production source files. Accepted824164SDK remains exact.\n\nDo not adopt or reroll this unchanged candidate. Before attributing candidate/fresh gains, inspect exact existing build/link identities and actual caller code: several source-identical fresh controls fail calibration against accepted. This does not erase positive candidate/accepted estimates, resolve uncertainty or establish a cache/frequency cause. Use already captured artifacts rather than repeat acceptance timing. Prior opposite/modal scalar and FFT-overlap directions are already covered; see the linked audit. No new AWS/Aeglos/ARM/TEE/large/minRAM/pressure/concurrency/throughput claim is made. Earlier C7a figures belong to an older source. The original all-eight CPU10× goal remains active0/8.\n\n[Design](artifacts/design.md), [fixed protocol](artifacts/sdk-design.md), [source audit](artifacts/source-audit.json), [analysis](artifacts/analysis.json), [raw rows](artifacts/performance/rows.json), [proof inventory](artifacts/proof-inventory.json), [preparation repair](artifacts/preparation-failures.json), [next-direction audit](artifacts/next-direction-audit.md), [raw profile hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).\n");
    save(
        r.join("closeout.json"),
        &json!({"status":"complete exact grouping-prefix-cutoff SDK experiment","full_gate_passed":false,"adopted":false,"SDK_requests":27660,"independent_gnark_exports":465,"proof_public_members":930,"next_prior_count":9904,"production_sources_checked_unchanged":171,"candidate_changed_source_files":2,"fresh_core_tests":116,"candidate_core_tests":117,"raw_tests_per_variant":2,"cubic_tests_per_variant":6,"profiles":6,"cycle_samples":samples,"closing_pins_checked":pins.as_object().unwrap().len(),"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    fs::create_dir_all(d).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), readable_report(md)).unwrap();
    println!(
        "Closed prefix cutoff: {} archive hashes;27660SDK465Gnark;ledger9904;noadoption",
        index.len()
    );
}

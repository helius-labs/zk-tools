use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fmt::Write, fs, path::Path, process::Command};

const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const DEST: &str = "docs/benchmarks/2026-10-30-current-bench-report";
const SUMMARY: &str = "docs/benchmarks/2026-10-29-current-cpu-summary";
const TRIAL: &str = "docs/benchmarks/2026-10-30-g2-balanced-sdk175";
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &Path) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn number(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn main() {
    let z = Path::new(Z);
    let d = z.join(DEST);
    fs::create_dir_all(&d).unwrap();
    let s = read(&z.join(SUMMARY).join("data.json"));
    let a = read(&z.join(TRIAL).join("artifacts/analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["independent_proof_count"], 24943);
    assert_eq!(s["rows"].as_array().unwrap().len(), 9);
    let mut md = String::from("# CPU prover benchmark report\n\n30 October 2026. Snapshot of accepted results and the completed balanced G2 full-proof experiment, based on repository commit `bb28d95f01502af2f6c15d42662e285637adb724`. No additional timing captures were run to write this document.\n\nAccepted local **1×2 remains 8.022165 ms with 134 MiB peak process RSS**. The latest candidate measures **8.013709 ms / 133.75 MiB**, with a paired gain of **0.416%** against the accepted executable measured in the same trial. It is **not adopted**: two physical-16 nonregression checks and the original-eight aggregate gain check versus fresh OFF fail.\n\nOn the latest measured AWS c7a.24xlarge, accepted Zopro is **11.915591 ms at 32 workers**, versus **7.959184 ms at 96 workers for Aeglos Sorted**. Zopro has lower best-tested latency on six of nine circuits and lower observed RSS on all nine. The new candidate has not been measured on AWS. The supplied M5 CPU 10× objective remains **0/8**.\n\n## Accepted local proof results\n\nRyzen 9 9950X, 16 physical cores, 32 logical CPUs, 128 GiB RAM, shared host; 32 workers below. Milliseconds and MiB. CPU and Metal values are supplied historical M5 references, not new matched-hardware Apple measurements. Speedup divides reference time by Zopro time.\n\n|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in s["rows"].as_array().unwrap() {
        let local = &row["local_accepted"];
        let r = &row["M5_reference_and_local_ratios"];
        let shape = row["shape"].as_str().unwrap();
        if r.is_null() {
            writeln!(
                md,
                "|{shape}|{:.6}|{:.2}|—|—|—|—|—|",
                number(&local["ms"]),
                number(&local["max_RSS_MiB"])
            )
            .unwrap();
        } else {
            assert_eq!(r["goal10x"], false);
            writeln!(
                md,
                "|{shape}|{:.6}|{:.2}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",
                number(&local["ms"]),
                number(&local["max_RSS_MiB"]),
                number(&r["cpu_reference_ms"]),
                number(&r["cpu_speedup"]),
                number(&r["metal_reference_ms"]),
                number(&r["metal_speedup"]),
                number(&r["required_ms_for_CPU10x"])
            )
            .unwrap();
        }
    }
    md.push_str("\nThere is no M5 reference for 54→1. Reaching 3.7 ms on primary 1×2 requires approximately 54% lower latency than the accepted result.\n\nAccepted measurements use 15 process blocks, three priming proofs, then 151 primary or 31 other requests per process. SDK timings include fresh witness solving, randomized proving, internal verification and serialization. Loading/preparation are outside the latency interval and inside lifetime RSS. Peak RSS is the largest process high-water mark across blocks; it is not minimum RAM or incremental proof workspace. The local SDK executable SHA-256 is `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f`.\n\nThe accepted build's original performance gate failed physical-16 control calibration. Adoption was an explicit engineering exception; a separate final-artifact qualification passed. The latest candidate does not inherit that exception. [Adoption](../2026-10-29-balanced-kh-adoption/README.md), [final artifact](../2026-10-29-balanced-kh-production/README.md).\n\n## Latest full-proof experiment\n\nThe candidate replaces the indexed narrow G2 top window with eleven balanced ordinary windows. It reuses borrowed points, the existing affine batches and LIFO queue. It adds no per-key lookup expansion, copied key point array, duplicate execution plan or persistent cache. Field arithmetic and workspace forecasts remain unchanged; unsupported shapes retain their original G2 route. [Implementation and source qualification](../2026-10-30-g2-balanced-sdk175/README.md).\n\nThis is a separate 31-block trial against the exact accepted executable and a fresh build with the candidate disabled (OFF). All nine shapes use 32 workers; primary also uses 16 physical cores. Blocks rotate and reverse case/variant order, with three primes followed by 151 primary or 31 other requests. No owned build, analyzer or independent workload overlaps capture. The following accepted-column values are fresh observations, not replacements for the accepted qualification table above.\n\n|Case|Accepted ms|Fresh OFF ms|Candidate ms|Candidate peak RSS MiB|\n|---|---:|---:|---:|---:|\n");
    let cases = [
        "1x2-w32",
        "1x2-w16",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
        "merge-54x1-w32",
    ];
    for case in cases {
        writeln!(
            md,
            "|{case}|{:.6}|{:.6}|{:.6}|{:.2}|",
            number(&a["cells"][format!("{case}/accepted")]["median_ms"]),
            number(&a["cells"][format!("{case}/OFF")]["median_ms"]),
            number(&a["cells"][format!("{case}/ON")]["median_ms"]),
            number(&a["cells"][format!("{case}/ON")]["max_RSS_MiB"])
        )
        .unwrap();
    }
    md.push_str("\nPaired candidate/control ratios below preserve block pairing and need not equal ratios of the displayed medians. Lower is faster. Bounds use sorted ranks 10–22 of 31, giving nominal 97.055% individual coverage under independent/exchangeable blocks, without a familywise guarantee.\n\n|Comparison|Paired ratio|Bounds|Wins /31|Declared check|\n|---|---:|---|---:|---|\n");
    for key in [
        "1x2-w32/ON/accepted",
        "1x2-w32/ON/OFF",
        "1x2-w16/ON/accepted",
        "1x2-w16/ON/OFF",
    ] {
        let p = &a["pairs"][key];
        let q = &p["statistics"];
        writeln!(
            md,
            "|{key}|{:.9}|[{:.9}, {:.9}]|{}|{}|",
            number(&q["median"]),
            number(&q["low"]),
            number(&q["high"]),
            q["wins"],
            if p["passed"] == true { "Pass" } else { "Fail" }
        )
        .unwrap();
    }
    for control in ["accepted", "OFF"] {
        let p = &a["original8_aggregates"][control];
        let q = &p["statistics"];
        writeln!(
            md,
            "|Original-eight aggregate / {control}|{:.9}|[{:.9}, {:.9}]|{}|{}|",
            number(&q["median"]),
            number(&q["low"]),
            number(&q["high"]),
            q["wins"],
            if p["passed"] == true { "Pass" } else { "Fail" }
        )
        .unwrap();
    }
    md.push_str("\nPrimary32 gain checks pass against both controls: 0.416% versus accepted and 0.729% versus OFF. The former corresponds to about 33 μs on the contemporaneous accepted median. The earlier complete G2 query improved 3.110% versus its frozen baseline, but that larger component gain did not transfer proportionally to complete proofs. [Query results](../2026-10-30-g2-balanced175/README.md).\n\nThe original-eight equal-log aggregate improves an estimated 0.647% versus accepted, with bounds below parity. Versus OFF, the estimate improves 0.305% but the bounds span parity. Physical-16 estimates also improve, but both upper bounds exceed the declared 1.02 nonregression tolerance. Those checks fail through uncertainty; the experiment does not establish a 3% physical-16 slowdown. All control-calibration, maximum-RSS and diagnostic checks pass. The complete gate is false and the candidate is not adopted. No favorable subset, changed threshold or unchanged rerun replaces that decision.\n\nPrimary32 summed CPU service falls from 182.948 to 171.309 ms/request versus accepted (about 6.36%), while wall latency improves 0.416% by the paired estimate. This difference does not isolate a cause. Six PMU panels meet running-coverage checks and six fixed-period profiles pass attribution/loss checks. Balanced-route symbols appear in ON profiles and not in either control. Existing absolute sample-period accounting limits remain; profile shares are qualitative, not exclusive costs or removable wall time. [All observations, RSS, counters, profiles and failed checks](../2026-10-30-g2-balanced-sdk175/README.md).\n\n## Latest AWS comparison with Aeglos\n\nc7a.24xlarge has 96 physical cores without SMT. The benchmark container has a 96-CPU quota, 30 GiB RAM and no swap; this is a measurement setting, not an architecture requirement. Each row selects the fastest median among 32/64/96 workers for each prover, with RSS from that selection. This is descriptive; the source report retains the full equal-worker matrix. AWS uses the accepted source built for Zen 4. Aeglos uses Sorted at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked.\n\n|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro RSS MiB|Aeglos RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for row in s["rows"].as_array().unwrap() {
        let p = &row["aws_best_tested_descriptive"]["plain"];
        let b = &row["aws_best_tested_descriptive"]["aeglos"];
        writeln!(
            md,
            "|{}|{:.6}|{}|{:.6}|{}|{:.2}|{:.2}|",
            row["shape"].as_str().unwrap(),
            number(&p["ms"]),
            p["workers"],
            number(&b["ms"]),
            b["workers"],
            number(&p["max_RSS_MiB"]),
            number(&b["max_RSS_MiB"])
        )
        .unwrap();
    }
    md.push_str("\nAWS measurements use 15 process blocks after three primes, with 101 primary or 17 other timed requests per process. For 54→1 the selected peaks are 1.83 GiB for Zopro and 12.81 GiB for Aeglos. These configuration-specific observations do not isolate the reason for the memory or latency difference. [Full AWS report and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).\n\n|1×2 workers|Zopro ms|Aeglos ms|Zopro summed CPU ms/request|Aeglos summed CPU ms/request|\n|---|---:|---:|---:|---:|\n|32|11.915591|11.867991|256.128|328.248|\n|64|11.994779|8.773943|420.399|430.446|\n|96|13.777708|7.959184|704.247|513.394|\n\nAeglos lowers 1×2 latency as workers increase in this sweep. Zopro spends substantially more CPU without reducing latency. Critical-path work and coordination remain useful investigation targets; RAM capacity, cache behavior, syscalls or synchronization have not individually been established as the explanation. Local SMT measurements cannot predict this AWS scaling directly. No new AWS or matched local Aeglos capture was added by the balanced G2 trial.\n\n## Conclusions and next work\n\nThe new G2 geometry saves CPU and yields a small measured primary proof gain, without a large lookup cache. It does not establish the required full-matrix improvement. Accepted performance stays unchanged. The best next experiment should remove work from the full-proof critical path or improve useful parallel execution, with an automatic profile and complete-query check before another full SDK qualification. An isolated arithmetic win alone is insufficient evidence for adoption.\n\nThe earlier combined grouping/drain candidate remains a useful separate result: 1×2 measured 7.846378 ms / 87 MiB and a 2.47% paired gain versus accepted, but its declared 8×2 nonregression and physical-16 calibration checks failed. It also remains unadopted. [Complete record](../2026-10-29-cached-drain175/README.md).\n\n## Verification and limits\n\nThe latest trial completes all 1,974 workflow steps: 55,788 internally verified requests including primes and 972 fresh 164-byte exports independently accepted by Gnark. Exported proofs are the last timed request per process, exported after timing. Public inputs, image and verification-key bindings match; new hashes are distinct from the prior ledger. The retained distinct-proof count advances **23,971 → 24,943**. Source, proof/public and receipt archives retain hashes and round-trip checks. Native OFF/ON each pass 127 core and eight prover tests; portable ON passes 101 core and eight prover tests. All 17 field source files are byte-identical to accepted; the existing 33 field tests are inherited. Writing this summary adds no proof or timing samples.\n\nThese observations do not establish minimum RAM, memory-pressure/swap behavior, ARM/mobile/TEE performance, forester 250/4999 performance or concurrent throughput. No multi-GiB lookup expansion is proposed. Latest AWS benchmark resources were removed.\n\n[Machine-readable snapshot](data.json), [source hashes](source-hashes.json), [checksums](SHA256SUMS), [full new trial](../2026-10-30-g2-balanced-sdk175/README.md), [rolling history](../2026-10-29-current-cpu-summary/README.md). Earlier dated benchmark documents remain historical snapshots.\n");
    fs::write(d.join("README.md"), md).unwrap();
    let data = Command::new("jq").args(["--slurpfile","a"]).arg(z.join(TRIAL).join("artifacts/analysis.json"))
        .args(["{as_of:\"2026-10-30\",snapshot_repository_commit:\"bb28d95f01502af2f6c15d42662e285637adb724\",new_captures_for_document:false,accepted_source_files:175,historical_distinct_Gnark_ledger:24943,rows:.rows,latest_full_proof_experiment:($a[0]+{report:\"../2026-10-30-g2-balanced-sdk175/README.md\"})}"])
        .arg(z.join(SUMMARY).join("data.json")).output().unwrap();
    assert!(data.status.success());
    fs::write(d.join("data.json"), data.stdout).unwrap();
    let mut pins = serde_json::Map::new();
    for rel in [
        format!("{SUMMARY}/data.json"),
        format!("{SUMMARY}/README.md"),
        format!("{TRIAL}/README.md"),
        format!("{TRIAL}/artifacts/analysis.json"),
        format!("{TRIAL}/artifacts/publication-review.json"),
        "docs/benchmarks/2026-10-29-proof-pgo-c7a175/README.md".into(),
        "docs/benchmarks/2026-10-29-proof-pgo-c7a175/artifacts/analysis.json".into(),
        "docs/benchmarks/2026-10-29-balanced-kh-production/artifacts/analysis.json".into(),
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"
            .into(),
    ] {
        pins.insert(rel.clone(), Value::String(hash(&z.join(rel))));
    }
    fs::write(
        d.join("source-hashes.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let sums = [
        "README.md",
        "data.json",
        "source-hashes.json",
        "generate.rs",
    ]
    .map(|f| format!("{}  {f}\n", hash(&d.join(f))))
    .concat();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!("Wrote current report: nine local/AWS shapes, eight CPU/Metal references, latest complete SDK trial; no new captures");
}

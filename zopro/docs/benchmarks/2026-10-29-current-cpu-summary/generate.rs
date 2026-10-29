use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const DEST: &str = "docs/benchmarks/2026-10-29-current-cpu-summary";
const SHAPES: [&str; 9] = [
    "1x2",
    "2x2",
    "4x2",
    "8x2",
    "12x2",
    "16x2",
    "24x2",
    "merge-24x1",
    "merge-54x1",
];
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn select_records(value: &Value, fields: &[&str]) -> Value {
    Value::Object(
        value
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, record)| {
                (
                    name.clone(),
                    Value::Object(
                        fields
                            .iter()
                            .map(|&field| {
                                assert!(!record[field].is_null(), "Missing {field} in {name}");
                                (field.to_string(), record[field].clone())
                            })
                            .collect(),
                    ),
                )
            })
            .collect(),
    )
}
fn main() {
    let root = Path::new(Z);
    let dest = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join(DEST));
    let names = [
        "2026-10-29-balanced-kh-adoption",
        "2026-10-29-balanced-kh-production",
        "2026-10-29-proof-pgo175",
        "2026-10-29-proof-pgo-qualification175",
        "2026-10-29-proof-pgo-c7a175",
        "2026-10-29-g2-part-ready175",
        "2026-10-29-fq6-toom175",
        "2026-10-29-early-x175",
        "2026-10-29-early-x-sdk175",
        "2026-10-29-swiss-groups175",
        "2026-10-29-swiss-groups-sdk175",
        "2026-10-29-swiss-control-audit175",
        "2026-10-29-wire-union-census175",
        "2026-10-29-shared-wire-sdk175",
        "2026-10-29-glv-kh-borrowed175",
        "2026-10-29-signed-difference175",
    ];
    let mut sources = BTreeMap::new();
    for name in names {
        for rel in ["README.md", "artifacts/analysis.json"] {
            let p = root.join("docs/benchmarks").join(name).join(rel);
            if p.exists() {
                sources.insert(p.strip_prefix(root).unwrap().display().to_string(), hash(p));
            }
        }
    }
    let manifest_rel =
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
    let mp = root.join(manifest_rel);
    sources.insert(manifest_rel.into(), hash(&mp));
    let m = read(mp);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 175);
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(root.join(f)), h.as_str().unwrap());
    }
    let local = read(
        root.join("docs/benchmarks/2026-10-29-balanced-kh-production/artifacts/analysis.json"),
    );
    let aws =
        read(root.join("docs/benchmarks/2026-10-29-proof-pgo-c7a175/artifacts/analysis.json"));
    let early =
        read(root.join("docs/benchmarks/2026-10-29-early-x-sdk175/artifacts/analysis.json"));
    let swiss =
        read(root.join("docs/benchmarks/2026-10-29-swiss-groups-sdk175/artifacts/analysis.json"));
    let audit = read(
        root.join("docs/benchmarks/2026-10-29-swiss-control-audit175/artifacts/analysis.json"),
    );
    let union =
        read(root.join("docs/benchmarks/2026-10-29-wire-union-census175/artifacts/analysis.json"));
    let shared =
        read(root.join("docs/benchmarks/2026-10-29-shared-wire-sdk175/artifacts/analysis.json"));
    let glv =
        read(root.join("docs/benchmarks/2026-10-29-glv-kh-borrowed175/artifacts/analysis.json"));
    assert_eq!(glv["gate_passed"], false);
    assert_eq!(glv["independent_proof_count"], 21259);
    let signed =
        read(root.join("docs/benchmarks/2026-10-29-signed-difference175/artifacts/analysis.json"));
    assert_eq!(signed["gate_passed"], false);
    assert_eq!(signed["ledger"], 21259);
    assert_eq!(shared["gate_passed"], false);
    assert_eq!(shared["independent_proof_count"], 21133);
    assert_eq!(audit["no_new_capture"], true);
    assert_eq!(audit["byte_identical_code_symbols"], 2918);
    assert_eq!(union["necessary_model_gate_passed"], true);
    assert_eq!(union["performance_claim"], false);
    assert_eq!(union["proof_ledger"], 21007);
    assert_eq!(union["rows"].as_array().unwrap().len(), 9);
    for row in union["rows"].as_array().unwrap() {
        assert_eq!(row["exact_metadata_equal"], true);
    }
    assert_eq!(swiss["gate_passed"], false);
    assert_eq!(swiss["independent_proof_count"], 21007);
    let caller = read(root.join("docs/benchmarks/2026-10-29-early-x175/artifacts/analysis.json"));
    assert_eq!(caller["component_gate"], true);
    assert_eq!(early["gate_passed"], false);
    assert_eq!(early["production_changed"], false);
    assert_eq!(early["new_independent_exports"], 126);
    assert_eq!(early["SDK_requests"], 15954);
    assert_eq!(local["final_artifact_candidate_gate_passed"], true);
    assert_eq!(local["gate_passed"], false);
    assert_eq!(aws["complete"], true);
    assert_eq!(aws["performance_gate_passed"], false);
    let mut rows = vec![];
    let mut zopro_faster = 0;
    let mut local10 = 0;
    let mut aws10 = 0;
    for shape in SHAPES {
        let cell = &local["cells"][format!("{shape}-w32/ON")];
        let mut best = BTreeMap::new();
        for variant in ["plain", "aeglos"] {
            let workers = [32, 64, 96]
                .into_iter()
                .min_by(|&a, &b| {
                    aws["cells"][format!("{shape}-w{a}/{variant}")]["ms"]
                        .as_f64()
                        .unwrap()
                        .total_cmp(
                            &aws["cells"][format!("{shape}-w{b}/{variant}")]["ms"]
                                .as_f64()
                                .unwrap(),
                        )
                })
                .unwrap();
            let c = &aws["cells"][format!("{shape}-w{workers}/{variant}")];
            best.insert(variant,json!({"workers":workers,"ms":c["ms"],"CPU_ms":c["CPU_ms"],"max_RSS_MiB":c["max_RSS_MiB"]}));
        }
        if best["plain"]["ms"].as_f64().unwrap() < best["aeglos"]["ms"].as_f64().unwrap() {
            zopro_faster += 1;
        }
        assert!(
            best["plain"]["max_RSS_MiB"].as_f64().unwrap()
                < best["aeglos"]["max_RSS_MiB"].as_f64().unwrap()
        );
        let reference = if shape == "merge-54x1" {
            Value::Null
        } else {
            let r = &m["m5"][shape];
            assert_eq!(r["accepted_ms"], cell["median_ms"]);
            let cpu = r["cpu_reference_ms"].as_f64().unwrap();
            if cell["median_ms"].as_f64().unwrap() <= cpu / 10. {
                local10 += 1;
            }
            if best["plain"]["ms"].as_f64().unwrap() <= cpu / 10. {
                aws10 += 1;
            }
            r.clone()
        };
        rows.push(json!({"shape":shape,"local_accepted":{"ms":cell["median_ms"],"workers":32,"CPU_ms":cell["CPU_ms"],"max_RSS_MiB":cell["max_RSS_MiB"]},"aws_best_tested_descriptive":best,"M5_reference_and_local_ratios":reference}));
    }
    assert_eq!(zopro_faster, 6);
    assert_eq!(local10, 0);
    assert_eq!(aws10, 0);
    let mut md=String::from("# Current CPU prover benchmarks\n\n29 October 2026\n\nThe accepted Zopro build measures **8.022 ms locally for 1×2**. On the latest AWS c7a.24xlarge comparison, Zopro measures **11.916 ms versus 7.959 ms for pinned Aeglos Sorted**, using each engine's fastest tested worker count. Zopro has lower best-count latency on six of nine AWS circuits and lower observed peak process RSS on all nine. The supplied M5 CPU 10× target remains **unmet on all eight reference shapes**.\n\nProduction remains the accepted 175-file implementation. Recent PGO, G2 scheduling, arithmetic, grouping and GLV K/H experiments have not replaced it.\n\n## Measurement scope\n\nLocal measurements use a Ryzen 9 9950X with 16 physical cores, 32 logical CPUs and 128 GiB RAM; the accepted table uses 32 workers. AWS measurements use one c7a.24xlarge with 96 physical cores and no SMT, testing 32, 64 and 96 workers. AWS runs use a network-disabled container with a 96-CPU quota, 30 GiB memory limit and no swap. That container limit is a benchmark setting, not a prover architecture cap.\n\nBoth tables report medians of 15 process medians after three priming proofs. Local primary processes measure 151 requests and other shapes 31; AWS primary processes measure 101 and other shapes 17. Timers include fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading and preparation are outside latency and inside lifetime process RSS. RSS is the maximum process high-water mark across the corresponding blocks, in MiB; it is not minimum RAM or incremental proof workspace.\n\nThe AWS Zopro builds use the exact accepted source compiled for Zen 4; the local executable targets the Ryzen machine. These are different executables. Aeglos uses pinned commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, Sorted engine, with prepared-key cache hits checked. The comparison does not claim to represent every upstream configuration.\n\n## Accepted local results and M5 references\n\nM5 CPU and Metal numbers are supplied historical references, not new Apple measurements. Speedups are reference time divided by Zopro time. The final column gives the latency required for a CPU 10× result.\n\n|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in &rows {
        let s = row["shape"].as_str().unwrap();
        let c = &row["local_accepted"];
        let rf = &row["M5_reference_and_local_ratios"];
        if rf.is_null() {
            writeln!(
                md,
                "|54→1|{:.3}|{:.2}|—|—|—|—|—|",
                c["ms"].as_f64().unwrap(),
                c["max_RSS_MiB"].as_f64().unwrap()
            )
            .unwrap();
        } else {
            writeln!(
                md,
                "|{s}|{:.3}|{:.2}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",
                c["ms"].as_f64().unwrap(),
                c["max_RSS_MiB"].as_f64().unwrap(),
                rf["cpu_reference_ms"].as_f64().unwrap(),
                rf["cpu_speedup"].as_f64().unwrap(),
                rf["metal_reference_ms"].as_f64().unwrap(),
                rf["metal_speedup"].as_f64().unwrap(),
                rf["required_ms_for_CPU10x"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    md+="\nNo M5 reference was supplied for 54→1. Reaching 3.7 ms for local 1×2 requires approximately **54% lower latency** than the accepted 8.022 ms result.\n\nThe accepted build's full performance gate remains **false** because the physical-16 fresh-control/accepted calibration interval exceeded its declared range. Adoption was an explicit engineering exception; all candidate gain/nonregression checks and the separately declared final-artifact gate passed. This exception is retained here, and is not transferred to later experiments. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [complete final-artifact measurements](../2026-10-29-balanced-kh-production/README.md).\n\n## Latest AWS comparison with Aeglos\n\nEach engine's displayed value is its fastest median among 32, 64 and 96 workers. These selections describe this matrix; they are not a paired comparison across worker counts or a universal worker policy. Peak RSS comes from the selected worker count. The full report retains every same-worker comparison and its paired bounds.\n\n|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro peak RSS MiB|Aeglos peak RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|\n";
    for row in &rows {
        let p = &row["aws_best_tested_descriptive"]["plain"];
        let a = &row["aws_best_tested_descriptive"]["aeglos"];
        writeln!(
            md,
            "|{}|{:.3}|{}|{:.3}|{}|{:.2}|{:.2}|",
            row["shape"].as_str().unwrap(),
            p["ms"].as_f64().unwrap(),
            p["workers"],
            a["ms"].as_f64().unwrap(),
            a["workers"],
            p["max_RSS_MiB"].as_f64().unwrap(),
            a["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    md+="\nAeglos leads 1×2, 2×2 and 4×2; Zopro leads the remaining six circuits in this descriptive selection. For 54→1, Zopro measures 122.397 ms with 1.83 GiB peak RSS, versus 173.236 ms with 12.81 GiB for Aeglos. These memory observations do not establish why the latency differs. [Full AWS matrix, paired comparisons and raw artifacts](../2026-10-29-proof-pgo-c7a175/README.md).\n\nThe corresponding Zopro speedups against the same supplied M5 references are:\n\n|Circuit|AWS Zopro ms|M5 CPU speedup|M5 Metal speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|\n";
    for row in rows
        .iter()
        .filter(|v| !v["M5_reference_and_local_ratios"].is_null())
    {
        let rf = &row["M5_reference_and_local_ratios"];
        let ms = row["aws_best_tested_descriptive"]["plain"]["ms"]
            .as_f64()
            .unwrap();
        writeln!(
            md,
            "|{}|{ms:.3}|{:.3}×|{:.3}×|{:.1}|",
            row["shape"].as_str().unwrap(),
            rf["cpu_reference_ms"].as_f64().unwrap() / ms,
            rf["metal_reference_ms"].as_f64().unwrap() / ms,
            rf["required_ms_for_CPU10x"].as_f64().unwrap()
        )
        .unwrap();
    }
    md+="\n## CPU scaling evidence\n\nThe AWS 1×2 worker sweep shows different scaling behavior:\n\n|Workers|Zopro latency ms|Aeglos latency ms|Zopro CPU ms per proof|Aeglos CPU ms per proof|\n|---|---:|---:|---:|---:|\n";
    for workers in [32, 64, 96] {
        let z = &aws["cells"][format!("1x2-w{workers}/plain")];
        let a = &aws["cells"][format!("1x2-w{workers}/aeglos")];
        writeln!(
            md,
            "|{workers}|{:.3}|{:.3}|{:.3}|{:.3}|",
            z["ms"].as_f64().unwrap(),
            a["ms"].as_f64().unwrap(),
            z["CPU_ms"].as_f64().unwrap(),
            a["CPU_ms"].as_f64().unwrap()
        )
        .unwrap();
    }
    md+="\nZopro's sampled epoch/scheduler residence rises from 14.09% at 32 workers to 61.48% at 96. Those flat CPU sample categories are not removable wall-time fractions. The evidence directs investigation toward useful work availability, critical paths and coordination, but does not isolate memory capacity, cache misses, clock behavior, syscalls or fences as the cause.\n\nThe container recorded zero memory/swap/OOM event deltas. It also recorded 39 CPU-throttled periods over the entire workflow, including preparation, verification and profiling; that is not a timed-only or per-engine attribution. The measurements do not establish performance under memory pressure, minimum RAM, phone/ARM/TEE performance, concurrent throughput, or forester 250/4999 behavior.\n";
    md += "\n## Latest arithmetic screen\n\nDirect signed CIOS differences improve the affine arithmetic expression by 7.38% against fresh ordinary code, with 31/31 wins and paired ratio 0.926221229 [0.925461631, 0.927912346]. The four-pattern aggregate improves 0.45%, also 31 wins, but the dependent chain regresses 2.89% and product-plus-scaling 3.81%. Three nonregression checks fail, so the declared generic component gate remains false.\n\nThe routine has no new heap buffer or key table. Its BigInt model passes 30,084 exact cases, and two native tests pass 172,544 chained Arkworks lane comparisons. All 372 latency processes and four gated profiles complete; no new SDK proof is measured. This is an arithmetic-expression gain, not faster proving. The result selects a separately declared actual-G2-batch qualification while retaining all synthetic regressions and the failed original gate. [Full component report](../2026-10-29-signed-difference175/README.md).\n";
    md += "\n## Latest local full proof experiment\n\nBorrowed signed GLV inside the fused K/H stream is slower. It reuses the accepted signed split with ten balanced component windows and two partitions, transforms temporary high-component buckets, and borrows the original K/H point operands. Earlier separate-query G1 GLV trials remain negative; this trial tests a materially revised caller.\n\n|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|\n|---|---:|---:|---:|\n";
    for variant in ["accepted", "OFF", "ON"] {
        let c = &glv["cells"][format!("1x2-w32/{variant}")];
        writeln!(
            md,
            "|{variant}|{:.6}|{:.3}|{:.2}|",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    md += "\nON/accepted paired ratio is 1.042733048 [1.036482844, 1.045527578], with zero wins in 31 blocks and a 346.494 µs slowdown estimate. ON/OFF is 1.055244019 [1.051573654, 1.057443162], also zero wins. Fresh OFF/accepted calibration passes; OFF is 1.24% faster by its paired estimate, a control observation rather than a candidate gain. Both candidate gain checks fail. Production remains unchanged.\n\nThe new implementation has no persistent key table or transformed point copy, but adds scalar scratch, partition merges and bucket transforms. Fewer weighted reductions did not offset these costs in the full request. CPU time rises and observed peak RSS is slightly higher. The captures do not isolate each contribution. Native suites pass 124/127 core and eight prover tests OFF/ON; both ON profiles sample the new route, with zero lost records. [Full GLV K/H pilot](../2026-10-29-glv-kh-borrowed175/README.md).\n";
    md += "\n## Previous shared classification experiment\n\nShared A/B witness classification reduces modeled hashing visits by about 40%, but the actual native caller is slower. It builds one proof-local dictionary over existing witness values, reconstructs both stable local groupings, and overlaps that work with product FFT preparation.\n\n|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|\n|---|---:|---:|---:|\n";
    for variant in ["accepted", "OFF", "ON"] {
        let c = &shared["cells"][format!("1x2-w32/{variant}")];
        writeln!(
            md,
            "|{variant}|{:.6}|{:.3}|{:.2}|",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    md += "\nON/accepted paired ratio is 1.159005632 [1.153296967, 1.159845819], with zero wins in 31 blocks and a 1.287 ms slowdown estimate. ON/OFF is 1.163638735 [1.159946487, 1.167238766], also zero wins. Fresh OFF/accepted calibration passes. The speed pilot fails, and production remains unchanged.\n\nObserved maximum lifetime RSS falls from 136.75 MiB accepted to 99.25 MiB ON, 37.50 MiB or 27.42% lower in this capture. This memory observation does not establish minimum RAM or pressure behavior. Thirty-one fixed blocks use 151 requests after three primes. This fresh accepted 8.130 ms measurement is separate from the complete nine-shape qualification above. The change reaches the canonical-wire path used by 1×2 and 2×2; larger paths receive correctness preflights without a new latency qualification. [Full native caller pilot and profiles](../2026-10-29-shared-wire-sdk175/README.md).\n";
    md+="\n## Previous grouped dictionary experiment\n\nThe index-only grouping candidate uses grouped hash-table probing and stores class indices instead of complete scalar keys. Its real-input constructor benchmark passes, but the full-proof pilot fails calibration and the gain check against accepted production. The fresh OFF control is 10.04% slower by its paired estimate.\n\n|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|\n|---|---:|---:|---:|\n";
    for variant in ["accepted", "OFF", "ON"] {
        let c = &swiss["cells"][format!("1x2-w32/{variant}")];
        writeln!(
            md,
            "|{variant}|{:.6}|{:.3}|{:.2}|",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    md+="\nON/accepted paired ratio is 0.997222755 [0.995722920, 1.000449909], with 20/31 wins and a 22.313 µs saving estimate. Bounds include parity. ON/OFF is 0.907667496 [0.903037230, 0.909997208], with 31 wins, but OFF/accepted calibration fails. Production remains unchanged.\n\nObserved ON peak RSS is 102.50 MiB versus 133.50 MiB accepted, 31 MiB or 23.22% lower in this primary capture. This memory observation is retained without claiming minimum RAM, pressure behavior or a universal reduction. Thirty-one fixed blocks use 151 requests after three primes. Their fresh accepted 8.031 ms is a separate capture from the earlier complete nine-shape 8.022 ms qualification above. [Full proof result](../2026-10-29-swiss-groups-sdk175/README.md), [constructor evidence](../2026-10-29-swiss-groups175/README.md).\n";
    md += "\n## Diagnostics and implications\n\nThe earlier dictionary trial's control gap remains unresolved after an offline audit found identical selected G2/G1 kernel bytes. That earlier failure is retained. In contrast, fresh control calibration passes in the new shared-classification trial, whose candidate still loses every block. [Earlier control audit](../2026-10-29-swiss-control-audit175/README.md).\n\nThe [shared-wire screen](../2026-10-29-wire-union-census175/README.md) established exact metadata and fewer hashing visits across nine fixtures. The native trial demonstrates that this reduction is insufficient for a proof-speed gain. A global classification pass changes the dependency graph: both query MSMs await it and quotient preparation. Its latency regression rules out adoption of this implementation; the flat CPU profiles do not isolate the regression's cause. Further work should preserve query readiness and target arithmetic or data movement on the critical path.\n\nBoth grouping experiments retain lower observed primary RSS. A separately declared memory/latency qualification may investigate that tradeoff with a nonregression bound set before measurement, while preserving each original failed speed gate. Neither candidate has been adopted under such a tradeoff. No AWS measurement follows the matrix above.\n";
    md+="\n## Earlier local full proof experiment\n\nThe early-X candidate moves an existing coordinate correction earlier in the batched affine addition. Its caller benchmark passes with a 0.320% aggregate gain versus fresh OFF, but this does not translate into a verified full-proof gain. Production remains unchanged.\n\nThe fixed 31-block primary comparison uses the same accepted executable as the local table, fresh OFF and candidate ON, with 151 requests per process after three primes. Its freshly measured accepted result is 8.040 ms; the 8.022 ms table above retains the earlier complete nine-shape qualification. These are separate captures.\n\n|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|\n|---|---:|---:|---:|\n";
    for variant in ["accepted", "OFF", "ON"] {
        let c = &early["cells"][format!("1x2-w32/{variant}")];
        writeln!(
            md,
            "|{variant}|{:.6}|{:.3}|{:.2}|",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    md += "\n|Paired comparison|Latency ratio|Bounds|Wins / 31|\n|---|---:|---|---:|\n";
    for control in ["OFF", "accepted"] {
        let c = &early["pairs"][format!("1x2-w32/ON/{control}")];
        writeln!(
            md,
            "|ON/{control}|{:.9}|[{:.9}, {:.9}]|{}|",
            c["median"].as_f64().unwrap(),
            c["low"].as_f64().unwrap(),
            c["high"].as_f64().unwrap(),
            c["wins"]
        )
        .unwrap();
    }
    md+="\nThe paired saving versus OFF is 6.021 µs with bounds spanning parity. Versus accepted, ON is 32.431 µs slower by the paired estimate. Both prospective gain checks fail; control calibration passes. Bounds have nominal 97.055% individual median coverage under independent/exchangeable blocks. The candidate is not adopted. [Full experiment and verification receipts](../2026-10-29-early-x-sdk175/README.md), [caller measurements](../2026-10-29-early-x175/README.md).\n";
    md+="\n## Recent experiment decisions\n\n|Experiment|Measured result|Decision|\n|---|---|---|\n|[Local proof-only PGO](../2026-10-29-proof-pgo-qualification175/README.md)|Primary pilot 0.828% faster versus accepted; newly measured seven-shape aggregate 0.771% faster. Full qualification failed physical-16 control calibration.|Not adopted; positive local estimates retained.|\n|[Same PGO profile on AWS](../2026-10-29-proof-pgo-c7a175/README.md)|All-eight aggregate approximately flat at 32 workers, 5.704% slower at 64 and 4.997% slower at 96. Seventeen nonregression checks failed.|Not adopted.|\n|[G2 partition readiness](../2026-10-29-g2-part-ready175/README.md)|Local 1×2 ON 8.066 ms, OFF 8.071 ms, accepted 7.991 ms. Paired ON/OFF interval spans parity; primary gain, 8×2 nonregression and all-eight gain checks failed.|Not adopted.|\n|[Five-product Fq6 arithmetic](../2026-10-29-fq6-toom175/README.md)|Correct field results, but complete final exponentiation 0.625% slower versus fresh OFF. Component aggregate/gain checks failed.|No prover integration.|\n|[Early-X caller and full proof](../2026-10-29-early-x-sdk175/README.md)|Caller aggregate 0.320% faster versus OFF; full proof ON 8.068 ms, OFF 8.083 ms, accepted 8.040 ms. Full-proof gain checks fail.|Not adopted; component gain retained.|\n|[Index-only grouped dictionary](../2026-10-29-swiss-groups-sdk175/README.md)|Constructor aggregate 55.93% faster in its compiled context; full proof ON 8.012 ms versus accepted 8.031 ms, with uncertainty spanning parity. Peak RSS 102.50 versus 133.50 MiB.|Not adopted; accepted-relative gain and control calibration checks fail.|\n|[Shared witness classification](../2026-10-29-shared-wire-sdk175/README.md)|ON 9.401 ms versus accepted 8.130 ms; paired regression 15.90%, zero wins. Peak RSS 99.25 versus 136.75 MiB. Control calibration passes.|Not adopted; both gain checks fail.|\n|[Borrowed GLV fused K/H](../2026-10-29-glv-kh-borrowed175/README.md)|ON 8.481 ms versus accepted 8.140 ms; paired regression 4.27%, zero wins. CPU time 193.316 versus 182.861 ms/request; peak RSS 137.00 versus 133.75 MiB.|Not adopted; both gain checks fail and control calibration passes.|\n|[Direct signed CIOS difference](../2026-10-29-signed-difference175/README.md)|Affine expression 7.38% faster, aggregate 0.45% faster; dependent chain and scaled-product regressions fail three nonregression checks.|Generic component gate fails; actual-G2-batch qualification selected separately.|\n\nThe current prover combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. These experiments add no adopted per-key lookup expansion. Further work should target the dominant MSM arithmetic and its critical path; the latest component gains have not established another full-proof improvement.\n\n## Verification and reproducibility\n\nThe accepted final-artifact qualification passed 124 native core and eight prover tests, 101 portable core and eight prover tests, plus 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct Gnark-verified exports across its three engines; all 1,221 cloud exports were also reverified after transport, without counting that repeat as new proofs. The G2 partition-readiness experiment adds 13,824 SDK requests and 246 distinct independent exports. Fq6 and early-X component work add no proofs. The early-X SDK experiment adds 15,954 requests and 126 distinct independent Gnark exports. The grouped-dictionary SDK pilot adds a further 15,954 requests and 126 independent exports. The shared-classification SDK pilot adds another 15,954 internally verified requests and 126 fresh independent exports. The GLV K/H SDK pilot adds a further 15,954 internally verified requests and 126 fresh independent exports. The historical distinct-proof ledger is **21,259**.\n\nAll of the latest AWS benchmark resources were removed. Source reports retain exact compiler flags, binary identities, keys and fixture bindings, proof bytes, process blocks, raw profiles, verification receipts and failed checks. This summary consolidates completed runs; it does not introduce new timing or verification samples. Full precision and selected worker counts are in [data.json](data.json), with [source hashes](source-hashes.json).\n";
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("README.md"), md).unwrap();
    let latest_cells = select_records(&early["cells"], &["median_ms", "CPU_ms", "max_RSS_MiB"]);
    let latest_pairs = select_records(
        &early["pairs"],
        &[
            "median",
            "low",
            "high",
            "wins",
            "paired_saving_ms",
            "nominal_coverage",
        ],
    );
    let mut data = json!({"as_of":"2026-10-29","source_commit":"c0a71aea1a2cc39c3ae6b5c5d2c08121b37e4a43","new_timings":false,"new_tests_or_proofs":false,"accepted_source_files":175,"accepted_original8_CPU10x":0,"AWS_plain_original8_CPU10x":0,"AWS_lower_Zopro_best_medians":6,"AWS_circuits":9,"historical_distinct_Gnark_ledger":21007,"latest_local_experiment":{"report":"../2026-10-29-swiss-groups-sdk175/README.md","cells":select_records(&swiss["cells"], &["median_ms","CPU_ms","max_RSS_MiB"]),"pairs":select_records(&swiss["pairs"], &["median","low","high","wins","paired_saving_ms","nominal_coverage"]),"gate_passed":swiss["gate_passed"],"new_independent_exports":swiss["new_independent_exports"],"SDK_requests":swiss["SDK_requests"],"production_changed":swiss["production_changed"]},"previous_local_experiment":{"report":"../2026-10-29-early-x-sdk175/README.md","cells":latest_cells,"pairs":latest_pairs,"gate_passed":early["gate_passed"],"SDK_requests":early["SDK_requests"],"new_independent_exports":early["new_independent_exports"],"production_changed":early["production_changed"],"parent_component_gate_passed":caller["component_gate"]},"all_source175_current_hashes_verified":true,"rows":rows});
    data["latest_control_audit"] = json!({
        "report": "../2026-10-29-swiss-control-audit175/README.md",
        "analysis": audit,
        "root_cause_resolved": false
    });
    data["latest_operation_count_screen"] = json!({
        "report": "../2026-10-29-wire-union-census175/README.md",
        "analysis": union,
        "native_caller_prototype_measured": false
    });
    data["latest_operation_count_screen"]["native_caller_prototype_measured"] = json!(true);
    data["historical_distinct_Gnark_ledger"] = json!(21133);
    data["previous_grouped_dictionary_experiment"] = data["latest_local_experiment"].clone();
    data["latest_local_experiment"] = json!({
        "report": "../2026-10-29-shared-wire-sdk175/README.md",
        "cells": select_records(&shared["cells"], &["median_ms", "CPU_ms", "max_RSS_MiB"]),
        "pairs": select_records(&shared["pairs"], &["median", "low", "high", "wins", "paired_saving_ms", "nominal_coverage"]),
        "gate_passed": shared["gate_passed"],
        "production_changed": shared["production_changed"],
        "new_independent_exports": shared["new_independent_exports"],
        "SDK_requests": shared["SDK_requests"]
    });
    data["previous_shared_classification_experiment"] = data["latest_local_experiment"].clone();
    data["historical_distinct_Gnark_ledger"] = json!(21259);
    data["latest_local_experiment"] = json!({
        "report": "../2026-10-29-glv-kh-borrowed175/README.md",
        "cells": select_records(&glv["cells"], &["median_ms", "CPU_ms", "max_RSS_MiB"]),
        "pairs": select_records(&glv["pairs"], &["median", "low", "high", "wins", "paired_saving_ms", "nominal_coverage"]),
        "gate_passed": glv["gate_passed"], "production_changed": glv["production_changed"],
        "new_independent_exports": glv["new_independent_exports"], "SDK_requests": glv["SDK_requests"]
    });
    data["latest_arithmetic_component"] = json!({
        "report": "../2026-10-29-signed-difference175/README.md", "gate_passed": signed["gate_passed"],
        "rows": signed["rows"], "aggregates": signed["aggregates"],
        "calibration_failures": signed["calibration_failures"], "nonregression_failures": signed["nonregression_failures"],
        "gain_failures": signed["gain_failures"], "SDK_requests":0, "exports":0,
        "next": "separately declared actual-G2-batch qualification; generic component gate remains failed"
    });
    fs::write(
        dest.join("data.json"),
        serde_json::to_vec_pretty(&data).unwrap(),
    )
    .unwrap();
    fs::write(
        dest.join("source-hashes.json"),
        serde_json::to_vec_pretty(&sources).unwrap(),
    )
    .unwrap();
    fs::write(dest.join("generate.rs"), fs::read(file!()).unwrap()).unwrap();
    for (p, h) in &sources {
        assert_eq!(hash(root.join(p)), *h);
    }
    let files = [
        "README.md",
        "data.json",
        "source-hashes.json",
        "generate.rs",
    ];
    fs::write(
        dest.join("SHA256SUMS"),
        files
            .iter()
            .map(|p| format!("{}  {p}\n", hash(dest.join(p))))
            .collect::<String>(),
    )
    .unwrap();
    println!("Wrote current bench summary: nine local/AWS rows, eight CPU+Metal references; sources{}pins; no newmeasurements",sources.len());
}

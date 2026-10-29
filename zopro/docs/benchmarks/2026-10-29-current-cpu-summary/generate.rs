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
    let instruction_profiles=read(root.join("docs/benchmarks/2026-10-29-instruction-profile175/artifacts/analysis.json"));
    assert_eq!(instruction_profiles["independent_proof_count"],22210);
    let worker_work=read(root.join("docs/benchmarks/2026-10-29-worker-work175/artifacts/analysis.json"));
    assert_eq!(worker_work["independent_proof_count"],22198);
    let stage_trace=read(root.join("docs/benchmarks/2026-10-29-stage-uprobes175/artifacts/analysis.json"));
    assert_eq!(stage_trace["independent_proof_count"],22189);
    let interleaved=read(root.join("docs/benchmarks/2026-10-29-interleaved-complex175/artifacts/analysis.json"));
    assert_eq!(interleaved["complete"],true);
    let basis=read(root.join("docs/benchmarks/2026-10-29-golden-basis175/artifacts/analysis.json"));
    assert_eq!(basis["ledger"],22144);
    let cached_x=read(root.join("docs/benchmarks/2026-10-29-g2-cached-x175/artifacts/analysis.json"));
    assert_eq!(cached_x["complete"],true);
    let cached_sdk=read(root.join("docs/benchmarks/2026-10-29-cached-hash-sdk175/artifacts/analysis.json"));
    assert_eq!(cached_sdk["new_independent_exports"],441);
    assert_eq!(cached_sdk["independent_proof_count"],22144);
    let cached_hash = read(root.join("docs/benchmarks/2026-10-29-cached-hash-groups175/artifacts/analysis.json"));
    assert_eq!(cached_hash["complete"],true);
    let diagnostic = read(root.join("docs/benchmarks/2026-10-29-representative-54-diagnostic175/artifacts/analysis.json"));
    assert_eq!(diagnostic["complete"], true);
    assert_eq!(diagnostic["independent_proof_count"],21703);
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
        "2026-10-29-signed-g2-caller175",
        "2026-10-29-deferred-groups175",
        "2026-10-29-representative-groups175",
        "2026-10-29-representative-sdk175",
        "2026-10-29-representative-54-diagnostic175",
        "2026-10-29-cached-hash-groups175",
        "2026-10-29-cached-hash-sdk175",
        "2026-10-29-g2-cached-x175",
        "2026-10-29-golden-basis175",
        "2026-10-29-interleaved-complex175",
        "2026-10-29-stage-uprobes175",
        "2026-10-29-worker-work175",
        "2026-10-29-instruction-profile175",
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
    let signed_caller = read(root.join(
        "docs/benchmarks/2026-10-29-signed-g2-caller175/artifacts/analysis.json",
    ));
    assert_eq!(signed_caller["complete"], true);
    assert_eq!(signed_caller["proof_ledger"], 21259);
    assert_eq!(signed_caller["production_changed"], false);
    let deferred_groups = read(root.join(
        "docs/benchmarks/2026-10-29-deferred-groups175/artifacts/analysis.json",
    ));
    assert_eq!(deferred_groups["complete"], true);
    assert_eq!(deferred_groups["ledger"], 21259);
    let representative_groups = read(root.join(
        "docs/benchmarks/2026-10-29-representative-groups175/artifacts/analysis.json",
    ));
    assert_eq!(representative_groups["complete"], true);
    assert_eq!(representative_groups["ledger"], 21259);
    let representative_sdk = read(root.join(
        "docs/benchmarks/2026-10-29-representative-sdk175/artifacts/analysis.json",
    ));
    assert_eq!(representative_sdk["independent_proof_count"], 21697);
    assert_eq!(representative_sdk["new_independent_exports"], 438);
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

    md += "\n## Latest cached hash grouping constructor\n\nThe revision retains one u64 hash per distinct class in a separate proof-local vector, so table growth avoids representative-input hashing. Hashes and the dictionary are released before scalar materialization; original output-phase memory bounds remain. Full-value equality, stable class order and the original sample/cutoff policy remain unchanged.\n\n";
    writeln!(md,"The newly declared constructor gate **{}**. Nine native component tests cover whole outputs, collisions, original allocation forecasts and exactly one hash per nonzero input through forced growth. Thirty-one fixed blocks retain all 18 real A/B inputs against the source-extracted original and exact previous representative constructor; three zero-loss profiles follow latency. No proof timing or adoption follows from this constructor screen. [Full results](../2026-10-29-cached-hash-groups175/README.md).\n",if cached_hash["component_gate"]==true{"passes"}else{"fails"}).unwrap();
    md += "|All 18 inputs versus|Ratio|Bounds|Wins / 31|\n|---|---:|---|---:|\n";
    for (k,v) in cached_hash["aggregates"].as_object().unwrap(){writeln!(md,"|{k}|{:.9}|[{:.9}, {:.9}]|{}|",v["median"].as_f64().unwrap(),v["low"].as_f64().unwrap(),v["high"].as_f64().unwrap(),v["wins"]).unwrap();}
    md += "\n## Previous representative grouping constructor\n\nThe new one-pass constructor retains compact representative-row/count records and row classes, copying full scalar values only after exact admission. It removes the previous candidate's second input hash pass, preserves query independence and adds no key table, execution plan or point copy.\n\n";
    writeln!(md, "The separately declared constructor gate **{}**. Thirty-one fixed blocks cover all 18 real A/B inputs against original and one-pass Swiss builders. Eight component tests pass, including full-output oracles and checked source-extracted allocation forecasts. Three gated profiles have zero lost records. No proof timing or adoption is inferred. [Full constructor report](../2026-10-29-representative-groups175/README.md).\n", if representative_groups["component_gate"] == true { "passes" } else { "fails" }).unwrap();
    md += "|Constructor aggregate versus|Ratio|Bounds|Wins / 31|\n|---|---:|---|---:|\n";
    for (label, v) in representative_groups["aggregates"].as_object().unwrap() {
        writeln!(md, "|{label}|{:.9}|[{:.9}, {:.9}]|{}|", v["median"].as_f64().unwrap(), v["low"].as_f64().unwrap(), v["high"].as_f64().unwrap(), v["wins"]).unwrap();
    }
    md += "\n";
    md += "\n## Previous two pass grouping constructor\n\nThe deferred builder first determines exact admission using a compact dictionary of borrowed row indices. Only admitted inputs allocate class/value/count arrays; a second pass converts the same dictionary to class indices in place. It preserves query independence and adds no key table, plan or point copy.\n\n";
    writeln!(md, "The separately declared constructor gate **{}**. Thirty-one fixed blocks compare source-extracted original, one-pass Swiss and deferred builders on all 18 real A/B inputs. Every result matches an independent ordered-map oracle; eight component tests and three zero-loss profiles pass. This is constructor evidence, with no new proof timing or adoption. [Full constructor results](../2026-10-29-deferred-groups175/README.md).\n", if deferred_groups["component_gate"] == true { "passes" } else { "fails" }).unwrap();
    md += "|Constructor aggregate versus|Ratio|Bounds|Wins / 31|\n|---|---:|---|---:|\n";
    for (label, v) in deferred_groups["aggregates"].as_object().unwrap() {
        writeln!(md, "|{label}|{:.9}|[{:.9}, {:.9}]|{}|", v["median"].as_f64().unwrap(), v["low"].as_f64().unwrap(), v["high"].as_f64().unwrap(), v["wins"]).unwrap();
    }
    md += "\n";
    md += "\n## Latest arithmetic screen\n\nDirect signed CIOS differences improve the affine arithmetic expression by 7.38% against fresh ordinary code, with 31/31 wins and paired ratio 0.926221229 [0.925461631, 0.927912346]. The four-pattern aggregate improves 0.45%, also 31 wins, but the dependent chain regresses 2.89% and product-plus-scaling 3.81%. Three nonregression checks fail, so the declared generic component gate remains false.\n\nThe routine has no new heap buffer or key table. Its BigInt model passes 30,084 exact cases, and two native tests pass 172,544 chained Arkworks lane comparisons. All 372 latency processes and four gated profiles complete; no new SDK proof is measured. This is an arithmetic-expression gain, not faster proving. A separately declared actual-G2-batch qualification follows below, retaining all synthetic regressions and the failed original gate. [Full component report](../2026-10-29-signed-difference175/README.md).\n";
    md += "\n## Latest complete G2 caller experiment\n\nThe exact signed-difference helper is now measured in complete owned and borrowed G2 prefix batches, including inversion, packing, coordinate output and scatter. Eighteen contexts cover 32/200/352 pairs at 1/16/32 workers; 199/351-pair tails receive correctness preflights. Fifteen fixed blocks compare frozen original libraries, fresh OFF and candidate ON, with no new table, point copy or timed allocation.\n\n";
    writeln!(md, "The separately declared caller gate **{}**. The earlier generic arithmetic gate remains failed, and production remains unchanged. No SDK proof timing or independent proof is added. All 930 workflow steps complete, including six gated profiles with CPU IDs/timestamps and zero lost records. [Full caller results and failed checks](../2026-10-29-signed-g2-caller175/README.md).\n", if signed_caller["component_gate"] == true { "passes" } else { "fails" }).unwrap();
    md += "|Caller aggregate|Candidate/control ratio|Bounds|Wins / 15|\n|---|---:|---|---:|\n";
    for (name, value) in signed_caller["aggregates"].as_object().unwrap() {
        writeln!(md, "|{name}|{:.9}|[{:.9}, {:.9}]|{}|", value["median"].as_f64().unwrap(), value["low"].as_f64().unwrap(), value["high"].as_f64().unwrap(), value["wins"]).unwrap();
    }
    md += "\n";

    md += "\n## Latest cached hash SDK experiment\n\nThe constructor stores hashes only during proof-local table construction, then drops them before admitted scalar materialization. The complete SDK follows its passing all-18 constructor screen. Earlier uncached gates remain failed.\n\n";
    writeln!(md,"The new full-request gate **{}**. Fifteen fixed blocks retain all nine circuits at 32 workers against accepted and fresh OFF. The trial makes 20,904 SDK requests and 441 fresh independent Gnark exports, with nine zero-loss primary/54 profiles after latency. Production remains unchanged pending any required further qualification. [Full results and failed checks](../2026-10-29-cached-hash-sdk175/README.md).\n",if cached_sdk["gate_passed"]==true{"passes"}else{"fails"}).unwrap();
    md += "|Circuit|Accepted ms|OFF ms|ON ms|Accepted peak RSS MiB|OFF peak RSS MiB|ON peak RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|\n";
    for shape in SHAPES {let k=format!("{shape}-w32");let c=|v:&str,f:&str|cached_sdk["cells"][format!("{k}/{v}")][f].as_f64().unwrap();writeln!(md,"|{shape}|{:.6}|{:.6}|{:.6}|{:.2}|{:.2}|{:.2}|",c("accepted","median_ms"),c("OFF","median_ms"),c("ON","median_ms"),c("accepted","max_RSS_MiB"),c("OFF","max_RSS_MiB"),c("ON","max_RSS_MiB")).unwrap();}
    md += "\n## Previous representative SDK experiment\n\nOne-pass representative grouping is now measured across all nine circuits in complete fresh SDK requests. Its original constructor gate remains failed. This separately declared protocol compares exact accepted, fresh OFF and ON in 15 rotated/reversed blocks at 32 workers, including whole-request latency and lifetime RSS.\n\n";
    writeln!(md, "The new full-request performance gate **{}**. Calibration intervals are retained as diagnostics under this prospective protocol; both candidate/control gain and nonregression comparisons remain required, together with relative RSS bounds. No production adoption follows from this stage alone. [Complete SDK report and failed checks](../2026-10-29-representative-sdk175/README.md).\n", if representative_sdk["gate_passed"] == true { "passes" } else { "fails" }).unwrap();
    md += "|Circuit|Accepted ms|OFF ms|ON ms|ON peak RSS MiB|\n|---|---:|---:|---:|---:|\n";
    for shape in SHAPES {
        let case = format!("{shape}-w32");
        let value = |variant: &str, field: &str| representative_sdk["cells"][format!("{case}/{variant}")][field].as_f64().unwrap();
        writeln!(md, "|{shape}|{:.6}|{:.6}|{:.6}|{:.2}|", value("accepted","median_ms"), value("OFF","median_ms"), value("ON","median_ms"), value("ON","max_RSS_MiB")).unwrap();
    }
    md += "\n";
    md += "\n## Previous GLV full proof experiment\n\nBorrowed signed GLV inside the fused K/H stream is slower. It reuses the accepted signed split with ten balanced component windows and two partitions, transforms temporary high-component buckets, and borrows the original K/H point operands. Earlier separate-query G1 GLV trials remain negative; this trial tests a materially revised caller.\n\n|Variant|1×2 ms|CPU ms per request|Peak RSS MiB|\n|---|---:|---:|---:|\n";
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
    md+="\n## Recent experiment decisions\n\n|Experiment|Measured result|Decision|\n|---|---|---|\n|[Local proof-only PGO](../2026-10-29-proof-pgo-qualification175/README.md)|Primary pilot 0.828% faster versus accepted; newly measured seven-shape aggregate 0.771% faster. Full qualification failed physical-16 control calibration.|Not adopted; positive local estimates retained.|\n|[Same PGO profile on AWS](../2026-10-29-proof-pgo-c7a175/README.md)|All-eight aggregate approximately flat at 32 workers, 5.704% slower at 64 and 4.997% slower at 96. Seventeen nonregression checks failed.|Not adopted.|\n|[G2 partition readiness](../2026-10-29-g2-part-ready175/README.md)|Local 1×2 ON 8.066 ms, OFF 8.071 ms, accepted 7.991 ms. Paired ON/OFF interval spans parity; primary gain, 8×2 nonregression and all-eight gain checks failed.|Not adopted.|\n|[Five-product Fq6 arithmetic](../2026-10-29-fq6-toom175/README.md)|Correct field results, but complete final exponentiation 0.625% slower versus fresh OFF. Component aggregate/gain checks failed.|No prover integration.|\n|[Early-X caller and full proof](../2026-10-29-early-x-sdk175/README.md)|Caller aggregate 0.320% faster versus OFF; full proof ON 8.068 ms, OFF 8.083 ms, accepted 8.040 ms. Full-proof gain checks fail.|Not adopted; component gain retained.|\n|[Index-only grouped dictionary](../2026-10-29-swiss-groups-sdk175/README.md)|Constructor aggregate 55.93% faster in its compiled context; full proof ON 8.012 ms versus accepted 8.031 ms, with uncertainty spanning parity. Peak RSS 102.50 versus 133.50 MiB.|Not adopted; accepted-relative gain and control calibration checks fail.|\n|[Shared witness classification](../2026-10-29-shared-wire-sdk175/README.md)|ON 9.401 ms versus accepted 8.130 ms; paired regression 15.90%, zero wins. Peak RSS 99.25 versus 136.75 MiB. Control calibration passes.|Not adopted; both gain checks fail.|\n|[Borrowed GLV fused K/H](../2026-10-29-glv-kh-borrowed175/README.md)|ON 8.481 ms versus accepted 8.140 ms; paired regression 4.27%, zero wins. CPU time 193.316 versus 182.861 ms/request; peak RSS 137.00 versus 133.75 MiB.|Not adopted; both gain checks fail and control calibration passes.|\n|[Direct signed CIOS difference](../2026-10-29-signed-difference175/README.md)|Affine expression 7.38% faster, aggregate 0.45% faster; dependent chain and scaled-product regressions fail three nonregression checks.|Generic component gate fails; actual-G2-batch qualification selected separately.|\n\nThe current prover combines vector field arithmetic, batched affine MSM additions, borrowed point operands, scalar endomorphism decomposition and balanced fused K/H windows. These experiments add no adopted per-key lookup expansion. Further work should target the dominant MSM arithmetic and its critical path; the latest component gains have not established another full-proof improvement.\n\n## Verification and reproducibility\n\nThe accepted final-artifact qualification passed 124 native core and eight prover tests, 101 portable core and eight prover tests, plus 28,000 SDK requests and 505 distinct independent Gnark proof exports. The latest AWS comparison adds 37,272 SDK requests and 1,248 distinct Gnark-verified exports across its three engines; all 1,221 cloud exports were also reverified after transport, without counting that repeat as new proofs. The G2 partition-readiness experiment adds 13,824 SDK requests and 246 distinct independent exports. Fq6 and early-X component work add no proofs. The early-X SDK experiment adds 15,954 requests and 126 distinct independent Gnark exports. The grouped-dictionary SDK pilot adds a further 15,954 requests and 126 independent exports. The shared-classification SDK pilot adds another 15,954 internally verified requests and 126 fresh independent exports. The GLV K/H SDK pilot adds a further 15,954 internally verified requests and 126 fresh independent exports. The new all-nine representative SDK qualification adds 20,802 internally verified requests and 438 fresh independent exports. The historical distinct-proof ledger is **22,144**.\n\nAll of the latest AWS benchmark resources were removed. Source reports retain exact compiler flags, binary identities, keys and fixture bindings, proof bytes, process blocks, raw profiles, verification receipts and failed checks. This summary consolidates completed runs; it does not introduce new timing or verification samples. Full precision and selected worker counts are in [data.json](data.json), with [source hashes](source-hashes.json).\n";
    fs::create_dir_all(&dest).unwrap();

    md += "\n## Frozen 54 to 1 CPU diagnostic\n\nThe exact accepted, OFF and ON SDK executables are profiled on 54→1 at 32 workers, with no source rebuild or new latency qualification. Each engine has one gated four-event PMU capture and one separate 199 Hz cycles profile, each covering 31 fresh requests after three primes. All six captures precede offline analysis, and all six exported proofs pass independent Gnark verification. The original all-nine candidate gate remains failed and production is unchanged. [Full diagnostic and counters](../2026-10-29-representative-54-diagnostic175/README.md).\n\n|Executable|Retired instructions per proof|Cycles per proof|IPC|Profile samples|Lost records|\n|---|---:|---:|---:|---:|---:|\n";
    for v in ["accepted","OFF","ON"] {
        let c=&diagnostic["counters"][v]; let p=&diagnostic["profiles"][v];
        if c["qualified"] == true {
            writeln!(md,"|{v}|{:.0}|{:.0}|{:.3}|{}|0|",c["events"]["instructions:u"]["per_request"].as_f64().unwrap(),c["events"]["cycles:u"]["per_request"].as_f64().unwrap(),c["instructions_per_cycle"].as_f64().unwrap(),p["samples"]).unwrap();
        }else{writeln!(md,"|{v}|unqualified|unqualified|unqualified|{}|0|",p["samples"]).unwrap();}
    }
    md += "\nThese single captures locate sampled work; they do not supply paired speed bounds or assign exclusive wall time to flat symbols. Raw counters retain event running percentages and unavailable or multiplexed events. This diagnostic adds 204 SDK requests and six fresh independent exports, taking the distinct-proof ledger to **21,703**. The all-eight CPU 10× objective remains unmet; the accepted CPU and Metal comparisons above are unchanged.\n";

    md += "\n## Cached packed X coordinates in complete G2 batches\n\nThe new native streaming path retains packed Ax/Bx in six-vector per-batch prefix records, eliminating reverse input-X loads/packing while preserving all field formulas. reserve_exact keeps typed capacity below the existing nine-vector workspace allowance. Actual cache bytes are reported rather than requiring them equal to the smaller control records. There is no key-sized cache.\n\n";
    writeln!(md,"The separately declared caller gate **{}**. Fifteen blocks retain all 18 owned/borrowed G2 contexts across 1/16/32 workers and 32/200/352 pairs, plus 199/351 tails in preflight. All outputs match generator oracles and all timed allocation counts are zero. This is a complete batch-wave measurement, not SDK latency. [Full caller results](../2026-10-29-g2-cached-x175/README.md).\n",if cached_x["component_gate"]==true{"passes"}else{"fails"}).unwrap();
    md += "|Aggregate|Ratio|Bounds|Wins / 15|\n|---|---:|---|---:|\n";
    for(k,v)in cached_x["aggregates"].as_object().unwrap(){writeln!(md,"|{k}|{:.9}|[{:.9}, {:.9}]|{}|",v["median"].as_f64().unwrap(),v["low"].as_f64().unwrap(),v["high"].as_f64().unwrap(),v["wins"]).unwrap();}
    md += "\nThe component adds no SDK requests, independent proof exports or adoption. The distinct-proof ledger remains 22,144, and the accepted CPU/Metal and AWS comparisons above are unchanged.\n";
    md+="\n## Latest trace-one Fq2 basis arithmetic screen\n\n";
    writeln!(md,"The separately declared component gate **{}**. A different polynomial basis, j²=j+1, reduces complex multiplication to 250 IFMA sites but charges its norm, conjugation and exact-divide-by-three square. Six contexts use 31 blocks against exact-field frozen and fresh controls. Basis conversion is outside the clock: this is an optimistic arithmetic screen, not proof latency or a key-format change. Production remains unchanged. [Full results](../2026-10-29-golden-basis175/README.md).\n\n|Context|Candidate/fresh ratio|Bounds|Wins / 31|\n|---|---:|---|---:|",if basis["gate_passed"]==true{"passes"}else{"fails"}).unwrap();
    for row in basis["rows"].as_array().unwrap(){let p=&row["paired"]["fresh"];writeln!(md,"|{}|{:.9}|[{:.9}, {:.9}]|{}|",row["pattern"].as_str().unwrap(),p["median"].as_f64().unwrap(),p["low"].as_f64().unwrap(),p["high"].as_f64().unwrap(),p["wins"]).unwrap();}
    writeln!(md,"\nCalibration failures: `{}`. Nonregression failures: `{}`. Gain failures: `{}`. No SDK proofs or new Gnark exports; the ledger remains 22,144. No new AWS measurement or source adoption follows this arithmetic screen. All earlier failed gates and positive contexts remain.\n",basis["calibration_failures"],basis["nonregression_failures"],basis["gain_failures"]).unwrap();

    md+="\n## Latest joint ordinary-basis complex caller\n\n";
    writeln!(md,"The separately declared caller gate **{}**. Both ordinary-basis complex outputs share a row-CIOS schedule, with the complemented second input streamed one digit at a time. The arithmetic count stays300 IFMA sites; field basis, square, norm, scratch and point formulas stay unchanged. Fifteen blocks compare all18 owned/borrowed contexts with exact frozen and fresh OFF controls, including1/16/32worker topologies. All output/capacity checks and zero timed allocations pass. [Full results](../2026-10-29-interleaved-complex175/README.md).\n\n|Aggregate|Ratio|Bounds|Wins / 15|\n|---|---:|---|---:|",if interleaved["component_gate"]==true{"passes"}else{"fails"}).unwrap();
    for(k,p)in interleaved["aggregates"].as_object().unwrap(){writeln!(md,"|{k}|{:.9}|[{:.9}, {:.9}]|{}|",p["median"].as_f64().unwrap(),p["low"].as_f64().unwrap(),p["high"].as_f64().unwrap(),p["wins"]).unwrap();}
    writeln!(md,"\nFailed checks: `{}`. No SDK proofs, new Gnark exports or adoption. The ledger remains22,144; all earlier failed gates and positive contexts remain. These are complete caller waves, not proof timings.\n",interleaved["failed_checks"]).unwrap();

    md+="\n## Frozen accepted SDK stage diagnostic\n\nEntry/return probes observe nine function boundaries in the exact accepted executable, without a source rebuild. Seven blocks compare plain and traced primary requests at1/16/32workers. These are instrumented stage intervals, not replacement latency results; nested/parallel spans overlap and K/H feed is partial. [Full diagnostic](../2026-10-29-stage-uprobes175/README.md).\n\n|Workers|Plain ms|Traced ms|Paired ratio|Bounds|Within factor1.03|\n|---|---:|---:|---:|---|---|\n";
    for w in[1,16,32]{let p=&stage_trace["overhead"][w.to_string()];writeln!(md,"|{w}|{:.6}|{:.6}|{:.9}|[{:.9}, {:.9}]|{}|",p["plain_median_ms"].as_f64().unwrap(),p["traced_median_ms"].as_f64().unwrap(),p["paired_ratio"].as_f64().unwrap(),p["low"].as_f64().unwrap(),p["high"].as_f64().unwrap(),p["within_factor1_03"]).unwrap();}
    md+="\nAll21recorded processes have31completeSDKintervals, balanced entry/return stacks, one PID and zero lost records. The trial adds1440SDKrequests and45fresh independent Gnark exports; the ledger is now22,189. All probes in the owned group were removed. Production and all accepted local/AWS/M5 tables remain unchanged. No source adoption or new performance qualification follows.\n";

    md+="\n## Retired work across worker counts\n\nThree fixed PMU blocks compare the same frozen SDK at1/16/32workers with no uprobes or source rebuild. Counts include worker-policy differences as well as runtime work; CPU time is not an instruction count. No new latency qualification or adoption follows. [Full diagnostic](../2026-10-29-worker-work175/README.md).\n\n|Workers|Instructions/request|Cycles/request|IPC|CPU ms/request|All panels qualified|\n|---|---:|---:|---:|---:|---|\n";
    for w in[1,16,32]{let p=&worker_work["topologies"][w.to_string()];let num=|v:&Value,n:usize|v.as_f64().map(|v|format!("{v:.n$}")).unwrap_or_else(||"—".into());writeln!(md,"|{w}|{}|{}|{}|{}|{}|",num(&p["counters"]["instructions:u"]["median"],0),num(&p["counters"]["cycles:u"]["median"],0),num(&p["median_IPC"],3),num(&p["median_CPU_ms_per_request"],3),p["all_panels_qualified"]).unwrap();}
    md+="\nAll nine new exports independently verify; the trial adds306SDKrequests and9newGnark proofs. Three existing topology preflights are reused. The ledger is now22,198; production, all accepted local/AWS/M5 tables and the prior failed parallel-trace overhead checks remain unchanged.\n";

    md+="\n## Frozen SDK instruction and cycle attribution\n\nTwelve FIFO-gated profiles compare separate instructions:u and cycles:u sampling at16/32workers, three blocks each, without rebuilding or changing the accepted executable. Flat period shares are diagnostic, not exclusive costs or measured per-IP CPI. [Full profiles](../2026-10-29-instruction-profile175/README.md).\n\n|Topology/event|Samples|All attribution panels qualified|\n|---|---:|---|\n";
    for(k,p)in instruction_profiles["aggregates"].as_object().unwrap(){writeln!(md,"|{k}|{}|{}|",p["samples"],p["all_attribution_qualified"]).unwrap();}
    md+="\nThe trial adds1,848SDKrequests and12fresh independent Gnark exports; the ledger is now22,210. No source adoption or new latency qualification follows. All accepted local/AWS/M5 tables and earlier failed gates remain unchanged.\n";
    fs::write(dest.join("README.md"),format!("{}\n",md.trim_end())).unwrap();


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
    let mut data = json!({"as_of":"2026-10-29","source_commit":"dd162f8557df2f98c5c55a32c7b0c4f632333866","new_timings":false,"new_tests_or_proofs":false,"accepted_source_files":175,"accepted_original8_CPU10x":0,"AWS_plain_original8_CPU10x":0,"AWS_lower_Zopro_best_medians":6,"AWS_circuits":9,"historical_distinct_Gnark_ledger":21007,"latest_local_experiment":{"report":"../2026-10-29-swiss-groups-sdk175/README.md","cells":select_records(&swiss["cells"], &["median_ms","CPU_ms","max_RSS_MiB"]),"pairs":select_records(&swiss["pairs"], &["median","low","high","wins","paired_saving_ms","nominal_coverage"]),"gate_passed":swiss["gate_passed"],"new_independent_exports":swiss["new_independent_exports"],"SDK_requests":swiss["SDK_requests"],"production_changed":swiss["production_changed"]},"previous_local_experiment":{"report":"../2026-10-29-early-x-sdk175/README.md","cells":latest_cells,"pairs":latest_pairs,"gate_passed":early["gate_passed"],"SDK_requests":early["SDK_requests"],"new_independent_exports":early["new_independent_exports"],"production_changed":early["production_changed"],"parent_component_gate_passed":caller["component_gate"]},"all_source175_current_hashes_verified":true,"rows":rows});
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
    data["latest_actual_G2_caller"] = json!({
        "report": "../2026-10-29-signed-g2-caller175/README.md",
        "gate_passed": signed_caller["component_gate"],
        "aggregates": signed_caller["aggregates"],
        "failed_checks": signed_caller["failed_checks"],
        "native_field_tests": signed_caller["native_field_tests"],
        "native_core_tests": signed_caller["native_core_tests"],
        "native_prover_tests": signed_caller["native_prover_tests"],
        "SDK_requests": 0, "new_independent_exports": 0, "production_changed": false
    });
    data["latest_arithmetic_component"]["next"] = json!("actual G2 caller qualification completed separately; generic arithmetic gate remains failed");
    data["latest_grouping_constructor"] = json!({
        "report": "../2026-10-29-deferred-groups175/README.md",
        "gate_passed": deferred_groups["component_gate"],
        "aggregates": deferred_groups["aggregates"],
        "failed_checks": deferred_groups["failed_checks"],
        "inputs": deferred_groups["inputs"],
        "native_tests": deferred_groups["native_tests"],
        "SDK_requests": 0, "new_proofs": 0, "production_changed": false
    });
    data["previous_deferred_grouping_constructor"] = data["latest_grouping_constructor"].clone();
    data["latest_grouping_constructor"] = json!({
        "report": "../2026-10-29-representative-groups175/README.md",
        "gate_passed": representative_groups["component_gate"],
        "aggregates": representative_groups["aggregates"],
        "failed_checks": representative_groups["failed_checks"],
        "inputs": representative_groups["inputs"],
        "native_tests": representative_groups["native_tests"],
        "SDK_requests":0, "new_proofs":0, "production_changed":false
    });
    data["previous_GLV_full_proof_experiment"] = data["latest_local_experiment"].clone();
    data["latest_local_experiment"] = json!({
        "report":"../2026-10-29-representative-sdk175/README.md",
        "cells":select_records(&representative_sdk["cells"], &["median_ms","CPU_ms","max_RSS_MiB"]),
        "pairs":select_records(&representative_sdk["pairs"], &["median","low","high","wins","paired_saving_ms","nominal_coverage"]),
        "original8_aggregates":representative_sdk["original8_aggregates"],
        "gate_passed":representative_sdk["gate_passed"],
        "failed_checks":representative_sdk["failed_checks"],
        "calibration_checks":representative_sdk["calibration_checks"],
        "calibration_gate_required":false,"SDK_requests":20802,"new_independent_exports":438,
        "production_changed":false
    });
    data["historical_distinct_Gnark_ledger"] = json!(21697);
    data["latest_54_diagnostic"] = diagnostic;
    data["historical_distinct_Gnark_ledger"] = json!(21703);
    data["previous_representative_grouping_constructor"] = data["latest_grouping_constructor"].clone();
    data["latest_grouping_constructor"] = json!({"report":"../2026-10-29-cached-hash-groups175/README.md","gate_passed":cached_hash["component_gate"],"aggregates":cached_hash["aggregates"],"pairs":select_records(&cached_hash["pairs"], &["median","low","high","wins","nominal_coverage"]),"failed_checks":cached_hash["failed_checks"],"inputs":cached_hash["inputs"],"native_tests":9,"SDK_requests":0,"new_proofs":0,"production_changed":false});
    data["previous_representative_full_proof_experiment"]=data["latest_local_experiment"].clone();
    data["latest_local_experiment"]=json!({"report":"../2026-10-29-cached-hash-sdk175/README.md","cells":select_records(&cached_sdk["cells"], &["median_ms","CPU_ms","max_RSS_MiB"]),"pairs":select_records(&cached_sdk["pairs"], &["median","low","high","wins","paired_saving_ms","nominal_coverage"]),"original8_aggregates":cached_sdk["original8_aggregates"],"gate_passed":cached_sdk["gate_passed"],"failed_checks":cached_sdk["failed_checks"],"calibration_checks":cached_sdk["calibration_checks"],"calibration_gate_required":false,"SDK_requests":20904,"new_independent_exports":441,"production_changed":false});
    data["historical_distinct_Gnark_ledger"]=json!(22144);
    data["previous_signed_G2_caller"]=data["latest_actual_G2_caller"].clone();
    data["latest_actual_G2_caller"]=json!({"report":"../2026-10-29-g2-cached-x175/README.md","gate_passed":cached_x["component_gate"],"aggregates":cached_x["aggregates"],"pairs":select_records(&cached_x["pairs"], &["median","low","high","wins"]),"failed_checks":cached_x["failed_checks"],"native_core_tests":cached_x["native_core_tests"],"native_prover_tests":cached_x["native_prover_tests"],"SDK_requests":0,"new_independent_exports":0,"production_changed":false});
    data["source_commit"]=json!("accbb63d4bc84265ef212e303cccc19a50a6e6d7");
    data["latest_basis_component"]=json!({"report":"../2026-10-29-golden-basis175/README.md","gate_passed":basis["gate_passed"],"rows":basis["rows"],"aggregates":basis["aggregates"],"calibration_failures":basis["calibration_failures"],"nonregression_failures":basis["nonregression_failures"],"gain_failures":basis["gain_failures"],"SDK_requests":0,"new_proofs":0,"conversion_outside_clock":true,"production_changed":false});
    data["source_commit"]=json!("f5aa862c646a887388e3f5ed40650280a5399555");
    data["previous_cached_X_G2_caller"]=data["latest_actual_G2_caller"].clone();
    data["latest_actual_G2_caller"]=json!({"report":"../2026-10-29-interleaved-complex175/README.md","gate_passed":interleaved["component_gate"],"aggregates":interleaved["aggregates"],"pairs":select_records(&interleaved["pairs"], &["median","low","high","wins"]),"failed_checks":interleaved["failed_checks"],"native_field_tests":interleaved["native_field_tests"],"native_core_tests":interleaved["native_core_tests"],"native_prover_tests":interleaved["native_prover_tests"],"SDK_requests":0,"new_independent_exports":0,"production_changed":false});
    data["source_commit"]=json!("e323fe88b81cc510ab6477ae865a6a7f90a96dd9");
    data["historical_distinct_Gnark_ledger"]=json!(22189);
    data["latest_stage_diagnostic"]=json!({"report":"../2026-10-29-stage-uprobes175/README.md","diagnostic_only":true,"overhead":stage_trace["overhead"],"stages":stage_trace["stages"],"SDK_requests":1440,"new_independent_exports":45,"independent_proof_count":22189,"source_rebuilds":0,"production_changed":false});
    data["source_commit"]=json!("a4ac1d8ae88ebb020fb83f5c51e036741d07cb2d");
    data["historical_distinct_Gnark_ledger"]=json!(22198);
    data["latest_worker_work_diagnostic"]=json!({"report":"../2026-10-29-worker-work175/README.md","diagnostic_only":true,"topologies":worker_work["topologies"],"all_counter_panels_qualified":worker_work["all_counter_panels_qualified"],"SDK_requests":306,"new_independent_exports":9,"independent_proof_count":22198,"source_rebuilds":0,"production_changed":false});
    data["source_commit"]=json!("7faaeca0cb882ce636f290795c4d1d593fad39bd");
    data["historical_distinct_Gnark_ledger"]=json!(22210);
    data["latest_instruction_profile_diagnostic"]=json!({"report":"../2026-10-29-instruction-profile175/README.md","diagnostic_only":true,"aggregates":instruction_profiles["aggregates"],"all_attribution_qualified":instruction_profiles["all_attribution_qualified"],"SDK_requests":1848,"new_independent_exports":12,"independent_proof_count":22210,"source_rebuilds":0,"production_changed":false});
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

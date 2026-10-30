use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};

const DEST: &str = "docs/benchmarks/2026-10-30-current-bench-snapshot175";
const PRIOR: &str = "docs/benchmarks/2026-10-30-current-bench-report";
const SUMMARY: &str = "docs/benchmarks/2026-10-29-current-cpu-summary";
const G2: &str = "docs/benchmarks/2026-10-30-g2-balanced-sdk175";
const TIMELINE: &str = "docs/benchmarks/2026-10-30-query-timeline175";
const READINESS: &str = "docs/benchmarks/2026-10-30-wire-readiness175";
const COMMIT: &str = "7c3b320dcfe2d89aa7ba1d908917c10c9c4356bc";

fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &Path) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    // Run from the repository root. Reuse historical tables without changing their bytes.
    let z = Path::new("zopro");
    let d = z.join(DEST);
    let s = read(&z.join(SUMMARY).join("data.json"));
    let g = read(&z.join(G2).join("artifacts/analysis.json"));
    let t = read(&z.join(TIMELINE).join("artifacts/analysis.json"));
    let w = read(&z.join(READINESS).join("artifacts/analysis.json"));
    assert_eq!(s["historical_distinct_Gnark_ledger"], 25231);
    assert_eq!(s["accepted_original8_CPU10x"], 0);
    assert_eq!(s["rows"].as_array().unwrap().len(), 9);
    assert_eq!(g["complete"], true);
    assert_eq!(g["gate_passed"], false);
    assert_eq!(t["complete"], true);
    assert_eq!(t["independent_proof_count"], 25231);
    assert_eq!(
        t["topology_qualified"],
        json!({"1x2-w1":true,"1x2-w16":false,"1x2-w32":true})
    );
    assert_eq!(
        t["stages"]["1x2-w32"]["last_query"],
        json!({"A":889,"B2":1656,"KH":2136,"ties":0})
    );
    assert_eq!(w["complete"], true);
    assert_eq!(w["distinct_proof_ledger"], 25231);
    for case in w["cases"].as_object().unwrap().values() {
        assert_eq!(case["source_order_prefix_blocked_at90percent"], false);
    }

    let prior = fs::read_to_string(z.join(PRIOR).join("README.md")).unwrap();
    let table_start = prior.find("## Accepted local proof results").unwrap();
    let table_end = prior.find("## Conclusions and next work").unwrap();
    let mut md = format!("# CPU prover benchmark snapshot\n\n30 October 2026. Evidence through repository commit `{COMMIT}`. This document consolidates completed measurements; writing it adds no benchmark, test, proof or AWS execution.\n\nAccepted local **1×2 remains 8.022165 ms / 134 MiB peak process RSS**. The latest AWS c7a.24xlarge comparison remains **11.915591 ms at 32 workers for Zopro versus 7.959184 ms at 96 workers for Aeglos Sorted**. Zopro has lower best-tested latency on six of nine AWS circuits and lower observed RSS on all nine. The supplied M5 CPU 10× objective remains **0/8**.\n\nThe latest optimization candidate, balanced G2 windows, records a **0.416% paired primary proof gain** versus accepted but fails its complete qualification gate and remains unadopted. Subsequent stage profiling and source-order readiness analysis clarify where to investigate; neither changes production or establishes a speedup. The cumulative distinct Gnark-verified export ledger is **25,231**.\n\n");
    md.push_str(&prior[table_start..table_end]);
    md = md.replace(
        "## Latest full-proof experiment",
        "## Latest full-proof optimization experiment (not adopted)",
    );
    md.push_str(r#"## New diagnostic evidence

The newer [in-process stage timeline](../2026-10-30-query-timeline175/README.md) uses the accepted algorithm with low-frequency timestamps. Thirty-one fixed blocks compare the exact accepted executable, fresh OFF and traced at one, 16 and 32 workers. OFF and traced share an executable. Both control comparisons fit the declared factor 1.01 at one and 32 workers; both physical-16 checks fail and those observations remain instrumented-only. No timing sample or topology is discarded or repeated.

At 32 workers, fresh accepted/OFF/traced medians are **7.955612 / 7.954394 / 7.946960 ms**. These are diagnostic observations of the existing algorithm, not a newly accepted implementation or a replacement for the qualification table.

|32-worker stage or ordering observation|Result|
|---|---:|
|Native prefix through product completion, median|1.864279 ms|
|Solver interval, median|1.131876 ms|
|Product interval, median|0.544177 ms|
|Three-query join interval, median|5.237387 ms|
|K/H returns last|2,136 / 4,681 (45.6%)|
|B2 returns last|1,656 / 4,681 (35.4%)|
|A returns last|889 / 4,681 (19.0%)|
|Last query return minus second-last, median|0.008335 ms|

Solver and product are nested within the prefix; query functions run parallel work and can wait for internal Rayon tasks. These separate medians cannot be added to reconstruct a proof. Return order is not an exclusive CPU cost, scheduler-ready timestamp or removable wall-time fraction. The evidence does not establish G2, K/H or A as the sole bottleneck. It explains why a G2 component gain alone is insufficient to predict a full-proof gain, without establishing a causal model. At one worker, all three queries contribute serially; B2 returns last by execution order.

The [witness/query readiness census](../2026-10-30-wire-readiness175/README.md) then examines two exact accepted solver programs without solving witnesses or timing proofs:

|Query|Points|Ready at 50% of program ops|Key-order prefix at 50%|Ready at 90%|Key-order prefix at 90%|
|---|---:|---:|---:|---:|---:|
|1x2/A|21,006|10,621|2,896 (13.8%)|18,930|6,722 (32.0%)|
|1x2/B|30,089|14,573|4,248 (14.1%)|26,991|9,964 (33.1%)|
|2x2/A|36,637|18,548|3,563 (9.7%)|33,024|7,119 (19.4%)|
|2x2/B|52,274|25,256|5,209 (10.0%)|46,883|10,521 (20.1%)|

These percentages count serialized program operations, not elapsed time or readiness under the parallel schedule. Many operands become available before their key-order predecessors. The declared rejection condition—no prefix progress beyond initially known inputs by 90%—is false in all four populations. This therefore constrains simple sequential streaming assumptions without rejecting all streaming designs or qualifying an implementation. H readiness and folded K membership are not reconstructed. Final solver use also does not authorize early witness mutation: public correction and proving still consume values.

## Conclusions and next work

No new production speedup is accepted. Balanced G2 saves approximately 6.36% summed CPU service on primary32 versus its contemporaneous accepted control, but its paired wall gain is 0.416% and the complete gate fails. On AWS, adding workers to Zopro 1×2 increases CPU service without improving latency, while Aeglos scales in this sweep. The measured gap cannot currently be attributed solely to RAM capacity, cache misses, allocation, syscalls or one query.

The next implementation should target shared MSM work, arithmetic used across queries, or useful parallel execution. Before building a streaming pipeline, measure readiness under the actual parallel schedule and account for gathering, scalar grouping, G2 high/low component order and workspace admission. Overlap can hide work but does not by itself remove CPU work. Any arithmetic proposal must include carry, packing and representation-conversion costs before a component result is interpreted as a proof result. Fixed modulus constants are compatible with this direction; multi-GiB key lookup expansions are outside the intended architecture.

Exact integer dot-product arithmetic is an exploratory research lead only: no native kernel, correctness qualification or performance measurement has been completed for it. This report assigns it no expected gain. Closed arithmetic and overlap experiments retain their recorded outcomes.

The earlier combined grouping/drain candidate measured 1×2 at 7.846378 ms / 87 MiB with a 2.47% paired gain versus accepted, but failed 8×2 nonregression and physical-16 calibration. It remains a separate unadopted result. [Complete record](../2026-10-29-cached-drain175/README.md).

## Verification and limits

|Completed study|Validation retained|New distinct Gnark exports|Ledger after study|
|---|---|---:|---:|
|Balanced G2 full SDK|1,974 workflow steps; 55,788 internally verified requests including primes; native OFF/ON 127 core + 8 prover tests each; portable ON 101 core + 8 prover tests|972|24,943|
|In-process query timeline|576 workflow steps; 43,002 internally verified requests including primes; 126 core + 8 prover tests, including two journal tests|288|25,231|
|Source-order readiness census|7 workflow steps; 3 structural tests; producer counts and online frontier agree at all 11 cuts; no witness solves or proofs|0|25,231|

Independent Gnark exports are the last timed proof from each selected process, written after timing; they are not every internally verified request. Proof/public/image/verification-key bindings, hash distinctness and archive round trips are retained in the source reports. The counts above summarize existing receipts and are not newly executed validation. All 17 field sources remain identical to accepted; existing field qualification is inherited. The timeline adds no PMU or sampled-profile capture. Its separate offline review reproduces all latency cells, paired intervals, stage medians and return counts without rerunning a proof. The census uses two readiness accounting methods sharing a structural parser; it is not an independent arithmetic oracle.

No owned compiler, helper, hash, archive, analyzer or unrelated workload overlapped performance capture. All nine accepted local/AWS rows and eight supplied M5 CPU/Metal rows remain unchanged. No new matched local Aeglos or AWS comparison follows from these diagnostics. Historical reports retain their original decisions and values.

RSS is observed lifetime process peak, including loading/preparation; it is not minimum RAM, a workspace bound or a result under memory pressure. These small-circuit results do not establish forester 250/4999 performance, swap behavior, mobile/ARM/TEE performance or concurrent throughput. The latest AWS benchmark report records removal of its owned resources; this document makes no new AWS inventory check. Comparisons apply to the pinned configurations and do not generalize to all upstream implementations.

[Machine-readable snapshot](data.json), [source hashes](source-hashes.json), [checksums](SHA256SUMS), [document generator](generate.rs), [prior snapshot](../2026-10-30-current-bench-report/README.md), [rolling experiment history](../2026-10-29-current-cpu-summary/README.md).
"#);
    fs::write(d.join("README.md"), &md).unwrap();

    // jq preserves source numeric tokens; avoid a Rust floating-point JSON round trip.
    let output = Command::new("jq")
        .arg("--arg").arg("commit").arg(COMMIT)
        .arg("--slurpfile").arg("g").arg(z.join(G2).join("artifacts/analysis.json"))
        .arg("--slurpfile").arg("t").arg(z.join(TIMELINE).join("artifacts/analysis.json"))
        .arg("--slurpfile").arg("w").arg(z.join(READINESS).join("artifacts/analysis.json"))
        .arg(r#"{as_of:"2026-10-30",snapshot_repository_commit:$commit,new_captures_for_document:false,accepted_source_files:175,historical_distinct_Gnark_ledger:.historical_distinct_Gnark_ledger,accepted_original8_CPU10x:.accepted_original8_CPU10x,AWS_plain_original8_CPU10x:.AWS_plain_original8_CPU10x,rows:.rows,latest_full_proof_experiment:$g[0],latest_query_timeline_diagnostic:$t[0],latest_wire_readiness_census:$w[0]}"#)
        .arg(z.join(SUMMARY).join("data.json")).output().unwrap();
    assert!(output.status.success());
    fs::write(d.join("data.json"), &output.stdout).unwrap();
    let mut pins = serde_json::Map::new();
    for rel in [
        format!("{PRIOR}/README.md"),
        format!("{PRIOR}/data.json"),
        format!("{SUMMARY}/data.json"),
        format!("{G2}/README.md"),
        format!("{G2}/artifacts/analysis.json"),
        format!("{TIMELINE}/README.md"),
        format!("{TIMELINE}/artifacts/analysis.json"),
        format!("{TIMELINE}/artifacts/offline-review.json"),
        format!("{READINESS}/README.md"),
        format!("{READINESS}/artifacts/analysis.json"),
        format!("{READINESS}/artifacts/review.json"),
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
    println!("Wrote snapshot: 9 accepted local/AWS rows, 8 CPU/Metal references, 3 completed studies; no captures");
}

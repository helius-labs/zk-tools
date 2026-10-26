use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-control-rescheduling";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-control-rescheduling";
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
    let rows = read(r.join("observations-final.json"));
    let decision = read(r.join("decision.json"));
    assert_eq!(decision["next_prior_count"], 6814);
    let counts = &decision["switch_reason_counts"];
    let runnable: u64 = counts
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.starts_with("R/") || k.starts_with("R+/"))
        .map(|(_, v)| v.as_u64().unwrap())
        .sum();
    let pct = 100. * counts["R/yield"].as_f64().unwrap() / runnable as f64;
    let mut md=format!("# Warm SDK scheduling diagnostic\n\nThe fixed exponent-chain candidate remains useful: its preceding full SDK study observed a paired 0.389% primary result with 32 workers improvement. This diagnostic does not change that study’s failed full qualification or adopt the candidate. Small positive gains have no minimum acceptance floor. The measured component saving was 32.512 µs = 0.032512 ms, about 0.38% of an 8.56 ms proof; it is not zero.\n\nThe new finding is that **{pct:.3}% of runnable SDK switch-outs occurred inside sched_yield**, while only nine switch-outs carried the raw preemption marker across all 18 traces. The high involuntary-switch counter should not be interpreted as evidence of heavy external preemption. Other tasks used 0.023–0.477% of selected CPU capacity in runs with 16 workers and 0.827–1.367% in runs with 32 workers, including observer/system activity. These observations weaken an external-CPU-load explanation for these captures; they do not establish the cause of earlier untraced slowdowns, exclude other effects, or qualify a waiting-policy rewrite.\n\n[Linux 6.8 scheduler source](https://github.com/torvalds/linux/blob/v6.8/kernel/sched/core.c#L6254) uses the involuntary counter for a runnable task; [sched_yield](https://github.com/torvalds/linux/blob/v6.8/kernel/sched/core.c#L8018) reschedules without setting a sleeping state. Upstream source is not asserted byte-identical to this Ubuntu kernel. The trace directly records syscall entry/exit and scheduler state on kernel 6.8.0-139-generic.\n\nThe two slower processes with 16 workers were accepted in block0 and OFF in block2, so the behavior is not confined to the arithmetic candidate. Yield call count alone does not explain it: their yield calls/proof were lower than several faster runs, while summed yield-associated off-CPU duration was larger. This is an observed association under instrumentation, not an estimate of recoverable wall time. Caller futex waiting is expected while worker tasks run and is not automatically a latency defect. Previously failed pause/spin/wait-policy experiments remain closed.\n\n## Captures\n\nAMD Ryzen 9 9950X, with 16 physical cores and 32 logical CPUs. Three rotated/reversed blocks, accepted/OFF/ON, 16 workers on CPUs 0–15 and 32 workers on CPUs 0–31. Each process loads the existing image, primes three fresh proofs, then captures 31 complete verified SDK requests. Instrumentation adds CLOCK_MONOTONIC brackets outside each elapsed timer, uses the standard warm FIFO control, and records scheduler/yield/futex events system-wide on the selected CPUs. No prover, Rayon, kernel, allocator or arithmetic source changes. These are diagnostic timings with observer overhead, not replacement latency qualification.\n\n|Block|Workers|Variant|Instrumented median ms|Yield calls/proof|Runnable switches/proof|Other CPU %|Idle CPU %|Yield off-CPU ms/proof, summed threads|\n|---|---:|---|---:|---:|---:|---:|---:|---:|\n");
    for row in rows.as_array().unwrap() {
        let c = &row["case"];
        let states = &row["sdk_switch_states"];
        let nr = states["R"].as_u64().unwrap_or(0) + states["R+"].as_u64().unwrap_or(0);
        writeln!(
            md,
            "|{}|{}|{}|{:.6}|{:.1}|{:.1}|{:.4}|{:.3}|{:.3}|",
            c["block"],
            c["workers"],
            c["variant"].as_str().unwrap(),
            row["median_instrumented_ms"].as_f64().unwrap(),
            row["syscall_entries"]["yield"].as_f64().unwrap() / 31.,
            nr as f64 / 31.,
            100. * row["occupancy_ns"]["other"].as_f64().unwrap()
                / row["cpu_capacity_ns"].as_f64().unwrap(),
            100. * row["occupancy_ns"]["idle"].as_f64().unwrap()
                / row["cpu_capacity_ns"].as_f64().unwrap(),
            row["sdk_off_cpu_ns_summed_over_threads"]["inside_yield"]
                .as_f64()
                .unwrap()
                / 31e6
        )
        .unwrap();
    }
    md.push_str("\nOff-CPU and syscall durations overlap across threads. They cannot be subtracted from proof wall time. Per-CPU own/other/idle/unknown occupancy conserves masked CPU capacity over the union of the 31 clock brackets; it does not identify which running worker was on the proof’s critical path. CPU occupancy also does not measure memory bandwidth, frequency or work performed.\n\n## Trace validation\n\nAll 18 raw captures contain 17,679,911 tracepoint samples, with zero recorded lost samples and zero scheduler continuity errors intersecting proof brackets. Sample counts agree with the independent perf script decoder, including its two unknown/exited-task display lines. Full-stream continuity gaps outside proof brackets remain recorded; no successful capture is repeated or excluded. Both readers’ results, censored syscall/off-CPU pairs, raw metadata and amendments are retained. Rusage accounts the enclosing loop while trace counts use the per-request brackets: sixteen processes match runnable/sleep switch counts exactly; one has two additional runnable switch-outs and another has one additional sleeping switch-out in the bracket accounting. This difference is preserved rather than forcing equality.\n\nThe original reader first failed on perf’s specialized scheduler formatting. That display also omits the preemption bit, as shown by the [libtraceevent scheduler printer](https://github.com/rostedt/libtraceevent/blob/master/plugins/plugin_sched_switch.c#L117). A separate Rust reader checks the saved file’s event identifiers, sample masks, monotonic clock, record bounds, raw payload layout and state bits. Its first invocation rejected RAW alignment padding; the separately named corrected reader preserves the 64-byte semantic payload with optional four-byte padding. Old sources, failed receipts and partial output remain local or archived as appropriate. Helper repairs occur before SDK capture or after all 18 captures; no reader repair reruns a proof. Six retained-analyzer tests and three raw-reader tests pass, including nanosecond parsing, disjoint windows, conservation, syscall censoring, continuity failures, preemption and rejecting unexpected sample layouts.\n\nSystem-wide perf rings use 128 pages/CPU, approximately 8/16 MiB with 16/32 CPUs. This observer cost is separate from prover RSS. Raw perf files and decoded/numeric event streams stay local with hashes because they contain unrelated task identities. Only sanitized aggregates, SDK logs, commands, formats, source and proof exports are published.\n\n## Proof and source validation\n\nFunctional preflight: three processes with four workers, 12 SDK requests, three independent Gnark exports. Scheduling captures:18 processes, 612 SDK requests, 18 independent exports. Combined 624 complete verified SDK requests, 21 distinct 164-byte exported proofs, 42 proof/public members. Every export passes pinned public-input binding, freshness against 6,793 prior identities and independent Gnark verification. The next prior ledger is 6,814; use the combined [all-proof-inventory](artifacts/all-proof-inventory.json), not both that ledger and its constituent inventories. Rechecks and postprocessing add no exports.\n\nAll 170 production source files and the exact accepted/candidate library, binary, source, input and tool pins remain unchanged. Original native/portable arithmetic correctness evidence is inherited; no new arithmetic implementation is claimed here. No new AWS, Aeglos, phone, TEE, giant-circuit, memory-pressure, minimum-RAM, throughput or untraced latency qualification. The full [fixed-chain SDK study](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied CPU and Metal comparisons; 0/8 meet the CPU 10× goal. The latest [C7a comparison](../2026-10-26-g2-tree-sdk/README.md) is unchanged.\n\nNext: retain the small positive arithmetic candidate and assess a separately declared clean Rust integration screen, without the runtime experiment selector. Do not turn these perturbed traces into an adoption decision or repeat failed generic waiting-policy changes.\n\n[Decision](artifacts/decision.json),[all observations](artifacts/observations-final.json),[frozen design](artifacts/design.md),[capture protocol](artifacts/capture-plan.json),[artifact hashes](artifact-index.json).\n");
    fs::write(d.join("README.md"), md).unwrap();
    let p = r.join("publish.rs");
    fs::copy(&p, d.join("artifacts/publish.rs")).unwrap();
    let mut hashes: BTreeMap<String, String> =
        serde_json::from_value(read(d.join("artifact-index.json"))).unwrap();
    hashes.insert("publish.rs".into(), hash(&p));
    for (rel, h) in &hashes {
        assert_eq!(hash(d.join("artifacts").join(rel)), *h);
    }
    save(d.join("artifact-index.json"), &json!(hashes));
    println!("Report and {} artifact hashes verified", hashes.len());
}

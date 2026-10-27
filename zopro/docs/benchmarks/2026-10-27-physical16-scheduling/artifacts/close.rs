use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-physical16-scheduling";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-physical16-scheduling";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let ty = e.file_type().unwrap();
        if ty.is_dir() {
            if e.file_name() != "smoke" {
                archive(root, &p, out, index)
            }
        } else if ty.is_file()
            && ["rs", "md", "json", "log", "proof", "public", "txt"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let rel = p.strip_prefix(root).unwrap();
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            assert_eq!(hash(&p), hash(&dest));
            index.insert(rel.display().to_string(), hash(&dest));
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["offline_decoder_revision"], 3);
    assert_eq!(a["next_prior_count"], 9069);
    assert_eq!(a["independent_exports"], 9);
    assert_eq!(read(r.join("workflow/result.json"))["status"], "complete");
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 18);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let files = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 170);
    for (p, h) in files {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        )
    }
    let rows = a["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 9);
    let sum = |key: &str| {
        rows.iter()
            .map(|v| v["scheduling"][key].as_u64().unwrap())
            .sum::<u64>()
    };
    let counts = json!({"cycles_samples":sum("cycles_samples"),"sample_records":sum("sample_records"),"sched_yield_entries":sum("sched_yield_entries"),"switch_outs":sum("switch_outs"),"runnable_outs":sum("runnable_outs"),"yield_associated_outs":sum("yield_associated_outs"),"next_own_runnable_outs":sum("next_own_runnable_outs"),"next_other_runnable_outs":sum("next_other_runnable_outs"),"trace_preempt_outs":sum("preempt_outs"),"unknown_state_outs":sum("unknown_state_outs"),"unmatched_in":sum("unmatched_in"),"unmatched_out":sum("unmatched_out")});
    save(r.join("summary-counts.json"), &counts);
    let source = Path::new("/usr/src/linux-headers-6.8.0-139/include/trace/events/sched.h");
    save(
        r.join("kernel-semantics.json"),
        &json!({"running_kernel":"6.8.0-139-generic","upstream_version_reviewed":"v6.8","upstream_perf_source":"https://github.com/torvalds/linux/blob/v6.8/kernel/events/core.c#L9092","upstream_perf_source_sha256":hash(r.join("kernel-events-core-6.8.c")),"upstream_perf_flag_condition":"!sched_in && task->on_rq","local_matching_kernel_trace_header":source,"local_trace_header_sha256":hash(source),"upstream_trace_source":"https://github.com/torvalds/linux/blob/v6.8/include/trace/events/sched.h#L190","actual_scheduler_preempt_argument_marker":"sched_switch prev_state '+'","interpretation":"Perf SWITCH preempt flag is not sufficient to distinguish sched_yield from scheduler preemption. Use actual sched_switch state, retain unknown states and observer limits."}),
    );
    let mut post = BTreeMap::new();
    for n in [
        "analyze-v2.rs",
        "analyze-v2",
        "analyze-v3.rs",
        "analyze-v3",
        "analyzer-tests-v2.log",
        "analyzer-tests-v3.log",
        "close.rs",
    ] {
        post.insert(r.join(n).display().to_string(), hash(r.join(n)));
    }
    save(r.join("postcapture-tools.json"), &json!(post));
    save(
        r.join("closeout.json"),
        &json!({"status":"complete scheduling diagnostic","SDK_requests":4536,"independent_gnark_exports":9,"proof_public_members":18,"next_prior_count":9069,"production_sources_checked_unchanged":170,"pins_checked":pins.as_object().unwrap().len(),"counts":counts,"adopted":false,"prior_composed_full_gate_passed":false,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    let mut md=String::from("# Physical16 scheduling and control variation\n\nThe exact accepted, fresh and combined SDK binaries all show varying runnable switch-outs while their CPU work stays relatively stable. Most runnable switch-outs occur shortly after an own-thread yield and switch to another thread of the same prover. This is evidence of scheduling churn within the observed workload; it does not establish that the earlier unprofiled control variation has the same cause. The combined candidate's established [primary32 and aggregate gains](../2026-10-27-composed-cpu-sdk/README.md) remain recorded. Its frozen calibration failure remains unresolved; no source is adopted here.\n\n## Capture and scope\n\nRyzen 9950X, Linux 6.8.0-139, sixteen workers on CPUs 0–15. Three rotated/reversed blocks use the exact three SDK executables, images, requests and runtime settings from the preceding study. Each process performs 501 fresh SDK requests after three primes. Existing FIFO acknowledgements enable perf events only for the warm loop. The pinned Rust xtask runs all nine captures and nine independent verifications serially before offline decoding. No unrelated owned workload overlaps a capture; no successful run is repeated or omitted.\n\nEach capture records 199 Hz user-cycle samples, sched_switch, sched_yield/futex entries and context-switch metadata. Recording every trace event creates substantial observer work; these numbers are diagnostic and cannot replace acceptance latency. All runs have one sampled SDK PID and 17 observed threads. The inspected cgroup scope and ancestors have no CPU quota; scheduler statistics are disabled, so no /proc runqueue-delay measurement is claimed.\n\n|Run|Observed SDK median ms|p99 ms|CPU ms/request|Yield calls/request|Runnable outs/request|To own thread %|Yield-associated outs %|RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in rows {
        let s = &row["scheduling"];
        let f = |key: &str| s[key].as_f64().unwrap();
        let sdk = &row["sdk"];
        writeln!(
            md,
            "|{}|{:.6}|{:.3}|{:.3}|{:.1}|{:.1}|{:.2}|{:.2}|{:.2}|",
            row["run"]["name"].as_str().unwrap(),
            row["median_ms"].as_f64().unwrap(),
            row["p99_ms"].as_f64().unwrap(),
            (sdk["user_cpu_ms"].as_f64().unwrap() + sdk["system_cpu_ms"].as_f64().unwrap()) / 501.,
            f("sched_yield_entries") / 501.,
            f("runnable_outs") / 501.,
            100. * f("next_own_runnable_outs")
                / (f("next_own_runnable_outs") + f("next_other_runnable_outs")),
            100. * f("yield_associated_outs") / f("runnable_outs"),
            sdk["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.
        )
        .unwrap();
    }
    md.push_str("\n## What the traces establish\n\nRunnable switch-outs vary from about 227 to 1,227 per request. Summed sleeping off-CPU time stays around 41–44 thread-ms/request; runnable off-CPU time varies much more. CPU work is about 132–134 ms/request for the controls and 126–127 for the combined candidate. The candidate's lower CPU work is useful corroboration, but tracing and the small diagnostic matrix cannot establish an unprofiled latency gain or qualify adoption.\n\n|Run|Yield-associated off-CPU ms/request|Other runnable off-CPU ms/request|Sleeping off-CPU ms/request|Unknown off-CPU ms/request|\n|---|---:|---:|---:|---:|\n");
    for row in rows {
        let s = &row["scheduling"];
        writeln!(
            md,
            "|{}|{:.3}|{:.3}|{:.3}|{:.3}|",
            row["run"]["name"].as_str().unwrap(),
            s["yield_associated_off_cpu_ms"].as_f64().unwrap() / 501.,
            s["other_runnable_off_cpu_ms"].as_f64().unwrap() / 501.,
            s["sleep_off_cpu_ms"].as_f64().unwrap() / 501.,
            s["unknown_off_cpu_ms"].as_f64().unwrap() / 501.
        )
        .unwrap();
    }
    md.push_str("\nThese are sums across threads and can exceed proof wall time. A yield association means a same-TID yield entry within 100 µs before a nonpreempted runnable switch-out; it is not exclusive causal attribution. Futex entries do not prove actual blocking. Off-CPU time is not measured runqueue delay, and a switch to another own thread does not prove the displaced thread was on the proof's critical path.\n\nOn upstream Linux 6.8, [perf's SWITCH preempt flag](https://github.com/torvalds/linux/blob/v6.8/kernel/events/core.c#L9092) tests whether the outgoing task remains on its runqueue. It can therefore label yield-related switches. The actual scheduler preempt argument is represented separately by the '+' state in the [sched_switch tracepoint](https://github.com/torvalds/linux/blob/v6.8/include/trace/events/sched.h#L190), also checked against this host's matching kernel headers. Observed state records here contain R, S and D, with no '+' marker. That is a statement about retained observations, not proof that no external interference occurred.\n\nThe corrected decoder joins synthetic SWITCH records to sampled own TIDs, retains boundary/unmatched records and uses tracepoint state for classification. Version 1 missed synthetic metadata because perf prints COMM/TID differently from samples. Version 2 parsed it but initially treated perf's runnable flag as actual preemption; the kernel-source check corrected that in version 3. All sources and intermediate summaries are retained and marked superseded; only version 3 supports the interpretation above. Four parser tests cover actual metadata format, unrelated TIDs, state/flag disagreement, lost records, mixed processes and interval conservation. No capture is repeated for these offline repairs.\n\n");
    writeln!(md,"Final totals: **{} cycle samples**, **{} sampled events**, **{} yield entries**, **{} switch-outs**; **{} unknown-state outs**, **{} unmatched IN / {} unmatched OUT** records. Boundary records are retained rather than silently assigned time. All captures report zero lost records. Cycle-period totals and event counts match the original decoder exactly. Raw traces and perf data remain local with hashes; public summaries omit other-task names and futex addresses.\n",counts["cycles_samples"],counts["sample_records"],counts["sched_yield_entries"],counts["switch_outs"],counts["unknown_state_outs"],counts["unmatched_in"],counts["unmatched_out"]).unwrap();
    md.push_str("## Validation and next work\n\nAll **4,536 SDK calls** verify internally. All **nine distinct fresh exports** pass independent Gnark verification, 164-byte length, fixture-public binding and freshness against the prior 9,060 identities; ledger **9,060→9,069**, with 18 proof/public members. No native arithmetic is rebuilt or retested for this diagnostic. Closing audits verify all 170 production sources and all 597 original input pins unchanged.\n\nLarge yield counts were already observed in the [earlier syscall/idle-policy work](../2026-10-22-universal-idle-pause-sdk/README.md), where simply replacing yields reduced kernel work without improving latency. This evidence does not justify repeating that policy or assuming fewer syscalls guarantee speed. The useful addition here is direct own-thread runnable-switch attribution on the exact combined/control artifacts.\n\nNext qualify the combined candidate's memory, portable paths and concurrent-call behavior, preserving its measured gains and the unresolved physical16 calibration result. Do not rerun unchanged acceptance timing or tune a yield threshold from these traces. Supplied CPU10× targets remain **0/8**; all-shape CPU/Metal comparisons remain in the [preceding complete latency report](../2026-10-27-composed-cpu-sdk/README.md#supplied-m5-references). No new AWS/Aeglos, minimum-RAM, phone, TEE or large-circuit result is claimed.\n\n[Design](artifacts/design.md), [version-3 analysis](artifacts/analysis.json), [kernel semantics](artifacts/kernel-semantics.json), [proof inventory](artifacts/proof-inventory.json), [raw local hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!(
        "Closed diagnostic with {} archive hashes; 9 fresh Gnark exports; no adoption",
        index.len()
    );
}

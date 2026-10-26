# Warm SDK scheduling diagnostic

The fixed exponent-chain candidate remains useful: its preceding full SDK study observed a paired 0.389% primary result with 32 workers improvement. This diagnostic does not change that study’s failed full qualification or adopt the candidate. Small positive gains have no minimum acceptance floor. The measured component saving was 32.512 µs = 0.032512 ms, about 0.38% of an 8.56 ms proof; it is not zero.

The new finding is that **98.514% of runnable SDK switch-outs occurred inside sched_yield**, while only nine switch-outs carried the raw preemption marker across all 18 traces. The high involuntary-switch counter should not be interpreted as evidence of heavy external preemption. Other tasks used 0.023–0.477% of selected CPU capacity in runs with 16 workers and 0.827–1.367% in runs with 32 workers, including observer/system activity. These observations weaken an external-CPU-load explanation for these captures; they do not establish the cause of earlier untraced slowdowns, exclude other effects, or qualify a waiting-policy rewrite.

[Linux 6.8 scheduler source](https://github.com/torvalds/linux/blob/v6.8/kernel/sched/core.c#L6254) uses the involuntary counter for a runnable task; [sched_yield](https://github.com/torvalds/linux/blob/v6.8/kernel/sched/core.c#L8018) reschedules without setting a sleeping state. Upstream source is not asserted byte-identical to this Ubuntu kernel. The trace directly records syscall entry/exit and scheduler state on kernel 6.8.0-139-generic.

The two slower processes with 16 workers were accepted in block0 and OFF in block2, so the behavior is not confined to the arithmetic candidate. Yield call count alone does not explain it: their yield calls/proof were lower than several faster runs, while summed yield-associated off-CPU duration was larger. This is an observed association under instrumentation, not an estimate of recoverable wall time. Caller futex waiting is expected while worker tasks run and is not automatically a latency defect. Previously failed pause/spin/wait-policy experiments remain closed.

## Captures

AMD Ryzen 9 9950X, with 16 physical cores and 32 logical CPUs. Three rotated/reversed blocks, accepted/OFF/ON, 16 workers on CPUs 0–15 and 32 workers on CPUs 0–31. Each process loads the existing image, primes three fresh proofs, then captures 31 complete verified SDK requests. Instrumentation adds CLOCK_MONOTONIC brackets outside each elapsed timer, uses the standard warm FIFO control, and records scheduler/yield/futex events system-wide on the selected CPUs. No prover, Rayon, kernel, allocator or arithmetic source changes. These are diagnostic timings with observer overhead, not replacement latency qualification.

|Block|Workers|Variant|Instrumented median ms|Yield calls/proof|Runnable switches/proof|Other CPU %|Idle CPU %|Yield off-CPU ms/proof, summed threads|
|---|---:|---|---:|---:|---:|---:|---:|---:|
|0|16|accepted|10.985199|10424.8|654.1|0.0556|25.824|9.406|
|0|16|off|10.246886|10944.3|178.7|0.0853|20.344|1.874|
|0|16|on|10.306278|11223.2|308.1|0.0231|22.224|3.726|
|0|32|accepted|8.932816|17689.9|460.2|0.8273|28.071|7.024|
|0|32|off|8.802400|17181.5|468.9|0.9524|27.590|5.789|
|0|32|on|9.127474|18129.6|1726.1|0.9500|33.338|21.371|
|1|32|accepted|8.825433|17207.0|588.1|0.8650|27.589|8.373|
|1|32|on|9.079533|18325.1|1167.5|0.8802|30.728|15.394|
|1|32|off|8.740453|16976.2|481.1|1.3671|26.445|6.814|
|1|16|accepted|10.344349|10619.2|194.1|0.0845|19.268|2.223|
|1|16|on|10.260882|11133.5|124.8|0.0572|19.824|1.502|
|1|16|off|10.297321|10806.1|270.2|0.1116|21.213|3.343|
|2|16|on|10.413370|10810.3|155.1|0.1802|20.584|2.201|
|2|16|accepted|10.450740|10691.5|174.3|0.0622|20.008|2.243|
|2|16|off|11.089826|10351.6|956.9|0.4769|28.002|11.852|
|2|32|on|8.777703|17841.8|455.1|0.8787|27.111|5.909|
|2|32|accepted|8.917707|17320.3|437.2|1.1524|27.424|6.591|
|2|32|off|8.983651|17869.3|596.7|0.9320|28.322|7.125|

Off-CPU and syscall durations overlap across threads. They cannot be subtracted from proof wall time. Per-CPU own/other/idle/unknown occupancy conserves masked CPU capacity over the union of the 31 clock brackets; it does not identify which running worker was on the proof’s critical path. CPU occupancy also does not measure memory bandwidth, frequency or work performed.

## Trace validation

All 18 raw captures contain 17,679,911 tracepoint samples, with zero recorded lost samples and zero scheduler continuity errors intersecting proof brackets. Sample counts agree with the independent perf script decoder, including its two unknown/exited-task display lines. Full-stream continuity gaps outside proof brackets remain recorded; no successful capture is repeated or excluded. Both readers’ results, censored syscall/off-CPU pairs, raw metadata and amendments are retained. Rusage accounts the enclosing loop while trace counts use the per-request brackets: sixteen processes match runnable/sleep switch counts exactly; one has two additional runnable switch-outs and another has one additional sleeping switch-out in the bracket accounting. This difference is preserved rather than forcing equality.

The original reader first failed on perf’s specialized scheduler formatting. That display also omits the preemption bit, as shown by the [libtraceevent scheduler printer](https://github.com/rostedt/libtraceevent/blob/master/plugins/plugin_sched_switch.c#L117). A separate Rust reader checks the saved file’s event identifiers, sample masks, monotonic clock, record bounds, raw payload layout and state bits. Its first invocation rejected RAW alignment padding; the separately named corrected reader preserves the 64-byte semantic payload with optional four-byte padding. Old sources, failed receipts and partial output remain local or archived as appropriate. Helper repairs occur before SDK capture or after all 18 captures; no reader repair reruns a proof. Six retained-analyzer tests and three raw-reader tests pass, including nanosecond parsing, disjoint windows, conservation, syscall censoring, continuity failures, preemption and rejecting unexpected sample layouts.

System-wide perf rings use 128 pages/CPU, approximately 8/16 MiB with 16/32 CPUs. This observer cost is separate from prover RSS. Raw perf files and decoded/numeric event streams stay local with hashes because they contain unrelated task identities. Only sanitized aggregates, SDK logs, commands, formats, source and proof exports are published.

## Proof and source validation

Functional preflight: three processes with four workers, 12 SDK requests, three independent Gnark exports. Scheduling captures:18 processes, 612 SDK requests, 18 independent exports. Combined 624 complete verified SDK requests, 21 distinct 164-byte exported proofs, 42 proof/public members. Every export passes pinned public-input binding, freshness against 6,793 prior identities and independent Gnark verification. The next prior ledger is 6,814; use the combined [all-proof-inventory](artifacts/all-proof-inventory.json), not both that ledger and its constituent inventories. Rechecks and postprocessing add no exports.

All 170 production source files and the exact accepted/candidate library, binary, source, input and tool pins remain unchanged. Original native/portable arithmetic correctness evidence is inherited; no new arithmetic implementation is claimed here. No new AWS, Aeglos, phone, TEE, giant-circuit, memory-pressure, minimum-RAM, throughput or untraced latency qualification. The full [fixed-chain SDK study](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied CPU and Metal comparisons; 0/8 meet the CPU 10× goal. The latest [C7a comparison](../2026-10-26-g2-tree-sdk/README.md) is unchanged.

Next: retain the small positive arithmetic candidate and assess a separately declared clean Rust integration screen, without the runtime experiment selector. Do not turn these perturbed traces into an adoption decision or repeat failed generic waiting-policy changes.

[Decision](artifacts/decision.json),[all observations](artifacts/observations-final.json),[frozen design](artifacts/design.md),[capture protocol](artifacts/capture-plan.json),[artifact hashes](artifact-index.json).

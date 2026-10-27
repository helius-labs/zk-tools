# Physical16 scheduling and control variation

The exact accepted, fresh and combined SDK binaries all show varying runnable switch-outs while their CPU work stays relatively stable. Most runnable switch-outs occur shortly after an own-thread yield and switch to another thread of the same prover. This is evidence of scheduling churn within the observed workload; it does not establish that the earlier unprofiled control variation has the same cause. The combined candidate's established [primary32 and aggregate gains](../2026-10-27-composed-cpu-sdk/README.md) remain recorded. Its frozen calibration failure remains unresolved; no source is adopted here.

## Capture and scope

Ryzen 9950X, Linux 6.8.0-139, sixteen workers on CPUs 0–15. Three rotated/reversed blocks use the exact three SDK executables, images, requests and runtime settings from the preceding study. Each process performs 501 fresh SDK requests after three primes. Existing FIFO acknowledgements enable perf events only for the warm loop. The pinned Rust xtask runs all nine captures and nine independent verifications serially before offline decoding. No unrelated owned workload overlaps a capture; no successful run is repeated or omitted.

Each capture records 199 Hz user-cycle samples, sched_switch, sched_yield/futex entries and context-switch metadata. Recording every trace event creates substantial observer work; these numbers are diagnostic and cannot replace acceptance latency. All runs have one sampled SDK PID and 17 observed threads. The inspected cgroup scope and ancestors have no CPU quota; scheduler statistics are disabled, so no /proc runqueue-delay measurement is claimed.

|Run|Observed SDK median ms|p99 ms|CPU ms/request|Yield calls/request|Runnable outs/request|To own thread %|Yield-associated outs %|RSS MiB|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|b0-accepted|10.485218|12.343|131.917|11608.7|227.2|98.42|98.56|93.25|
|b0-fresh|10.693050|16.127|132.280|11437.9|504.3|99.25|98.89|89.25|
|b0-composed|10.215649|12.644|126.802|11699.1|276.6|98.09|98.57|93.50|
|b1-accepted|10.740930|17.329|132.770|11421.7|527.1|98.89|98.81|94.50|
|b1-composed|11.022993|16.578|125.954|11064.0|1001.3|99.34|99.00|93.25|
|b1-fresh|11.167857|17.288|132.859|11148.1|839.2|99.13|98.87|96.00|
|b2-composed|11.553825|16.697|125.724|10830.0|1227.1|99.57|99.10|92.75|
|b2-accepted|11.951906|17.302|131.994|10910.6|1194.3|99.04|98.85|94.00|
|b2-fresh|10.863782|15.580|133.674|11586.0|434.7|98.89|98.79|95.50|

## What the traces establish

Runnable switch-outs vary from about 227 to 1,227 per request. Summed sleeping off-CPU time stays around 41–44 thread-ms/request; runnable off-CPU time varies much more. CPU work is about 132–134 ms/request for the controls and 126–127 for the combined candidate. The candidate's lower CPU work is useful corroboration, but tracing and the small diagnostic matrix cannot establish an unprofiled latency gain or qualify adoption.

|Run|Yield-associated off-CPU ms/request|Other runnable off-CPU ms/request|Sleeping off-CPU ms/request|Unknown off-CPU ms/request|
|---|---:|---:|---:|---:|
|b0-accepted|4.660|2.812|41.441|0.000|
|b0-fresh|9.790|7.857|42.098|0.000|
|b0-composed|5.384|2.796|41.900|0.000|
|b1-accepted|10.299|8.051|42.046|0.000|
|b1-composed|19.545|14.144|41.804|0.000|
|b1-fresh|16.532|13.881|42.483|0.000|
|b2-composed|24.042|18.308|41.264|0.000|
|b2-accepted|23.408|20.305|42.749|0.000|
|b2-fresh|8.320|5.674|43.624|0.000|

These are sums across threads and can exceed proof wall time. A yield association means a same-TID yield entry within 100 µs before a nonpreempted runnable switch-out; it is not exclusive causal attribution. Futex entries do not prove actual blocking. Off-CPU time is not measured runqueue delay, and a switch to another own thread does not prove the displaced thread was on the proof's critical path.

On upstream Linux 6.8, [perf's SWITCH preempt flag](https://github.com/torvalds/linux/blob/v6.8/kernel/events/core.c#L9092) tests whether the outgoing task remains on its runqueue. It can therefore label yield-related switches. The actual scheduler preempt argument is represented separately by the '+' state in the [sched_switch tracepoint](https://github.com/torvalds/linux/blob/v6.8/include/trace/events/sched.h#L190), also checked against this host's matching kernel headers. Observed state records here contain R, S and D, with no '+' marker. That is a statement about retained observations, not proof that no external interference occurred.

The corrected decoder joins synthetic SWITCH records to sampled own TIDs, retains boundary/unmatched records and uses tracepoint state for classification. Version 1 missed synthetic metadata because perf prints COMM/TID differently from samples. Version 2 parsed it but initially treated perf's runnable flag as actual preemption; the kernel-source check corrected that in version 3. All sources and intermediate summaries are retained and marked superseded; only version 3 supports the interpretation above. Four parser tests cover actual metadata format, unrelated TIDs, state/flag disagreement, lost records, mixed processes and interval conservation. No capture is repeated for these offline repairs.

Final totals: **108683 cycle samples**, **57789292 sampled events**, **50954739 yield entries**, **4197442 switch-outs**; **4 unknown-state outs**, **3 unmatched IN / 148 unmatched OUT** records. Boundary records are retained rather than silently assigned time. All captures report zero lost records. Cycle-period totals and event counts match the original decoder exactly. Raw traces and perf data remain local with hashes; public summaries omit other-task names and futex addresses.

## Validation and next work

All **4,536 SDK calls** verify internally. All **nine distinct fresh exports** pass independent Gnark verification, 164-byte length, fixture-public binding and freshness against the prior 9,060 identities; ledger **9,060→9,069**, with 18 proof/public members. No native arithmetic is rebuilt or retested for this diagnostic. Closing audits verify all 170 production sources and all 597 original input pins unchanged.

Large yield counts were already observed in the [earlier syscall/idle-policy work](../2026-10-22-universal-idle-pause-sdk/README.md), where simply replacing yields reduced kernel work without improving latency. This evidence does not justify repeating that policy or assuming fewer syscalls guarantee speed. The useful addition here is direct own-thread runnable-switch attribution on the exact combined/control artifacts.

Next qualify the combined candidate's memory, portable paths and concurrent-call behavior, preserving its measured gains and the unresolved physical16 calibration result. Do not rerun unchanged acceptance timing or tune a yield threshold from these traces. Supplied CPU10× targets remain **0/8**; all-shape CPU/Metal comparisons remain in the [preceding complete latency report](../2026-10-27-composed-cpu-sdk/README.md#supplied-m5-references). No new AWS/Aeglos, minimum-RAM, phone, TEE or large-circuit result is claimed.

[Design](artifacts/design.md), [version-3 analysis](artifacts/analysis.json), [kernel semantics](artifacts/kernel-semantics.json), [proof inventory](artifacts/proof-inventory.json), [raw local hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).

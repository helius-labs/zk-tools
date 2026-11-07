# Coefficient pre-add: primary full-proof pilot

The fixed pre-add candidate **does not improve primary full-proof latency and is not adopted**. It passes correctness, but fails the declared gain checks against both controls. Fresh OFF/accepted calibration passes. This closes this arithmetic recipe without an unchanged reroll or a compiler, worker, layout or favorable-subset rescue.

## Matched latency

Local Ryzen 9 9950X, 32 logical workers pinned to CPUs 0–31; accepted, fresh OFF and ON SDKs from the [integration qualification](../2026-11-07-cios-preadd-sdk176/README.md). Fifteen fixed rotated/reversed blocks; each process performs three primes and 151 complete SDK requests. Each block contributes one process mean per arm. All latency processes finish before proof verification, counters or profiles; no other owned CPU workload overlaps recording. No blocks are excluded or added.

|Arm|Median process mean, ms|Median process maximum RSS, MiB|Largest observed process maximum RSS, MiB|
|---|---:|---:|---:|
|Accepted|7.962069|131.25|132.75|
|Fresh OFF|8.031122|129.50|134.75|
|ON|8.020403|131.25|134.75|

Ratios below are paired by block; they need not equal ratios of the displayed medians. Lower is faster. Sorted paired-ratio ranks 4 through 12 give nominal 96.484375% marginal median coverage conditional on independent, exchangeable blocks; these are not familywise intervals.

|Comparison|Median ratio|Interval|Wins of 15|
|---|---:|---:|---:|
|OFF/accepted|1.008430|0.995919–1.014064|4|
|ON/OFF|1.002348|0.994566–1.008505|6|
|ON/accepted|1.004561|1.000527–1.010937|3|

The frozen gate requires ON's upper ratio below 1 and at least 10 wins against **both** controls, with OFF/accepted bounds inside [1/1.03, 1.03]. Calibration passes; both gain checks fail. There is no minimum gain threshold. ON/OFF is inconclusive within this interval, and ON/accepted is unfavorable. The candidate has no demonstrated full-proof benefit.

## Counters and profiles

After latency, each arm runs 31 requests for one FIFO-gated PMU panel and 251 requests for one profile, after three primes. User-cycle profiles use a period of 16 million and 512 mmap pages. All counters report 100% running. These single diagnostic panels are not replicated latency estimates or causal memory measurements.

|User counter per request|OFF|ON|ON/OFF|
|---|---:|---:|---:|
|Cycles|813,327,689|813,483,947|1.000192|
|Instructions|1,389,372,033|1,445,390,977|1.040320|
|Cache misses|2,733,774|2,865,421|1.048156|
|Branch misses|914,809|898,854|0.982560|

The native dependency graph was shorter, while this SDK panel executes about 4% more instructions with essentially unchanged user cycles. That supports the decision to reject the recipe; it does not establish which microarchitectural resource causes the result. The static graph did not predict full-proof latency.

Accepted/OFF/ON profiles contain 12,333/12,203/12,281 samples, zero recorded LOST events, and 1.59%/1.93%/1.73% unknown symbols. Review checks event/header metadata, 16-million periods, one PID per profile, 33 TIDs and CPUs 0–31. G1 streaming, G2 borrowed apply, Crossbeam epoch pinning and Fq inversion remain prominent in all three profiles. Symbol samples describe sampled user cycles, not exclusive wall time, critical-path delay or DRAM traffic. No exact IP-to-ELF attribution or independent second LOST decoder is claimed.

## Correctness and memory

All 51 new final exports pass the pinned external Gnark verifier and freshness/public/image bindings: 45 latency exports and six diagnostic exports. This stage completes **7,794 SDK requests including primes**. With the prior 27 preflight exports/108 requests, the study has 78 fresh exports and 7,902 requests. The canonical ledger advances 31,414→31,465; old ledgers remain immutable. Successful preflight proofs, builds, tests and native models are inherited without replay.

A separate offline reviewer reconstructs paired gates from raw samples, checks 117 capture/verification/offline receipts and their stream hashes, checks source pins, reconciles all 51 new exports and independently reads counter/sample totals. This is a second parser and arithmetic review, not a second cryptographic implementation. All capture and review workflow commands pass on their first execution. Before recording, preparation corrected the reviewer's completion-field and perf PID/TID-format assumptions; the original preparation source is retained. An additional shell JSON-plan syntax error was corrected before any caller decode ran.

No table, proving-key copy, retained plan, pool or heap workspace is added. Actual emitted caller inspection matters: G1 streaming's prologue RSP subtraction stays at 4,288 bytes, while G2 apply grows from 5,952 to 6,080 bytes, **128 bytes more per active frame**. Both push six registers and align the stack separately. These two selected prologues exclude alignment loss, nested frames and concurrent workers; they are not a complete stack bound. Their static IFMA sites are 800→797 and 1,450→1,447, respectively; static site counts are not dynamic operation counts. RSS includes setup/mappings and is neither minimum RAM nor an incremental live-memory bound. No mobile, huge-circuit, pressure or swap qualification is established.

## Outcome and evidence

Production remains unchanged. Accepted historical matched local 32-worker 1×2 remains **7.915095 ms**, against supplied M5 CPU 37 ms and Metal 24 ms. This pilot's accepted control is 7.962069 ms; it is retained as a contemporaneous control, not substituted for the accepted all-shape baseline. The verified CPU 10× target remains unmet on all eight shapes. No new AWS or Aeglos comparison is made. [Inherited all-shape comparisons](../2026-11-07-g2-real-norm-z176/README.md).

[Protocol](artifacts/protocol.json), [latency analysis](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [independent review](artifacts/independent-review.json) and [caller stack review](artifacts/caller-stack-analysis.json) contain the results. Sources, plans, pins, host context and a compressed archive of exact raw receipts/streams accompany them; the archive is checked against the originals. Proof/public payloads, keys, requests, binaries, libraries and raw perf data remain private; their hashes are retained. Private root: `/home/ubuntu/src/zopro-perf-work/cios-preadd-sdk176/primary-pilot`.

The next useful question is how to remove executed main-MSM work. This experiment provides no basis for another carry-chain-only tweak with more instructions and unchanged multiplication volume.

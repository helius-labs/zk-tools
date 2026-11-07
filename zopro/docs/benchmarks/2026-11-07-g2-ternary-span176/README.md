# Original and ternary G2 query service traces

All four single-worker cases pass the declared observer checks. None of the eight threaded cases passes calibration, so the threaded graphs remain instrumented-only evidence. This diagnostic does not resolve the cause of the ordinary threaded ternary slowdown and establishes no optimization or adoption. Accepted production and proof timings remain unchanged.

The [preceding reciprocal-codec trial](../2026-11-07-ternary-reciprocal-caller176/README.md) found a consistent improvement over the old ternary codec, while still losing to the original algorithm at 16 and 32 workers. Aggregate CPU profiles could not identify per-query completion delays. This experiment records individual query roots, windows, partitions, forking phases, nonforking leaf service, weighted collectors, and final Horner work. It does not tune or reopen the rejected candidate.

## Implementation and qualification

The existing qualified [service recorder](../2026-10-29-service-trace175/README.md) is reused byte-for-byte as a prefix. Its global-pool reset/snapshot entrypoints do not fit this benchmark's private Rayon pool: resetting on the main thread would address the wrong pool, and taking the extra caller snapshot inside a worker would duplicate that worker. Appended entrypoints broadcast over the explicit private pool from the non-worker driver thread. The original event format, clocks, TLS parent restoration, capacity, and overflow rules are unchanged.

One new private-pool test passes, covering 1/4-worker reset, unique IDs, parent containment, CPU clocks, repeated snapshots, and disabled mode. The two old recorder tests and existing point/field/codec/full-MSM qualifications are inherited, not rerun. Six parent files contain recoverable diagnostic insertions or Phi scope wrappers; 172 of the 178 input files stay exact. Inverse edits recover all 178 input files, including the driver’s original math, decoder, oracle, affinity, scalar reset and normalization behavior. All 176 production files remain unchanged.

Each worker and caller keeps at most 1,024 records of 80 bytes: at 32 workers, at most 2.579 MiB of recorder payload before metadata and snapshots. Overflow never grows the buffer. This is an observer allocation, not a production RAM policy or process-memory bound. CPU clocks cover only nonforking feeds, Phi maps, pre-normalization work, group/indexed/weighted/merge leaves, weighted projective collectors and the final Horner combination. Surrounding Rayon phases record elapsed spans only. Explicit IDs carry parent relationships across callbacks.

The fresh consumer uses O3, native CPU targeting, fat LTO, and one codegen unit. Its SHA-256 is `094dcb8b22e29a7b1cd492bdb5ec1cffdccb230679c5f5dc612485b3861c7dbc`. OFF and ON share that executable. FROZEN is the exact preceding consumer, `28bce6f68ae92bf79acfc22161f9a59c17c6aaf34e432dd389cf26d2cceeb35a`. B selects the original algorithm; T selects the fixed reciprocal ternary algorithm. Arithmetic, grouping, buckets, queues, batching, pool geometry and 512 KiB worker stacks remain fixed.

## Fixed protocol

Both retained populations run at 1/16/32 workers, pinned to CPU 2, CPUs 0–15, or CPUs 0–31. All 36 preflight processes pass their existing independent full Arkworks MSM oracles, with three primes, one observed query, and one postcheck each. All 12 traced preflight graphs pass exact phase counts, ID uniqueness, parent containment, CPU-clock checks, serial nonoverlap, and zero-drop checks. Preflight clocks do not select performance results.

The main run contains exactly three rotated/reversed blocks × six contexts × two algorithms × three variants = 108 processes. Each has three primes, seventeen observed calls, and one postcheck. Every result is oracle-checked. The comparison unit is the arithmetic mean of the seventeen individual `samples_ns` in every variant. Fresh outer-loop clocks include observer work and are not compared. API scopes stay inside each sample; recorder reset, snapshot, count summaries and graph output stay outside. Those operations can affect caches, allocation and scheduling context and remain subject to calibration.

Case order rotates by block modulo six and reverses on odd blocks. Algorithm order rotates/reverses by block plus original case index; variant order additionally uses the original algorithm index, B=0/T=1. All elapsed/count/drop summaries remain. ON graphs are retained at fixed observed indices 0, 8 and 16: nine per algorithm/context, 108 main graphs plus 12 preflight graphs. There are no exclusions, added blocks, rerolls, capacity changes or fastest-graph selections. No owned build, hash, analysis, publication or second workload overlaps recording.

For each algorithm/context, every OFF/FROZEN block ratio must fit factor 1.05 and its median must fit factor 1.03. Every ON/OFF block ratio must be at most 1.10 and its median at most 1.05. These are engineering checks on a small fixed set, not confidence intervals or evidence of unbiased tracing. Every retained graph must also pass semantic coverage and have zero drops.

## Observer results

Times are medians of three process sample means, in milliseconds. The ratio columns are medians of paired block ratios and need not equal ratios of the displayed times. B is original; T is reciprocal ternary. All threaded rows are instrumented-only.

| Population / workers / algorithm | FROZEN ms | OFF ms | ON ms | OFF/FROZEN | ON/OFF | Qualified |
|---|---:|---:|---:|---:|---:|---|
| 1×2 / 1 / B | 36.960364 | 37.140578 | 37.379311 | 1.004876 | 1.006428 | yes |
| 1×2 / 1 / T | 33.704217 | 33.434515 | 33.522868 | 0.992605 | 1.002643 | yes |
| 1×2 / 16 / B | 3.488403 | 3.874317 | 3.887597 | 1.108383 | 1.005460 | no |
| 1×2 / 16 / T | 3.770242 | 3.902359 | 3.924599 | 1.039405 | 0.991154 | no |
| 1×2 / 32 / B | 3.074663 | 3.416306 | 3.480471 | 1.112193 | 1.015266 | no |
| 1×2 / 32 / T | 3.282474 | 3.411691 | 3.318454 | 1.046279 | 0.984893 | no |
| 2×2 / 1 / B | 39.184525 | 39.888666 | 39.967332 | 1.017970 | 1.001065 | yes |
| 2×2 / 1 / T | 36.100798 | 36.004237 | 36.135800 | 0.997412 | 1.004514 | yes |
| 2×2 / 16 / B | 3.938305 | 4.529553 | 4.534695 | 1.150128 | 0.995357 | no |
| 2×2 / 16 / T | 4.241605 | 4.633003 | 4.578304 | 1.092276 | 0.959642 | no |
| 2×2 / 32 / B | 3.572792 | 4.352666 | 4.382141 | 1.218281 | 1.028396 | no |
| 2×2 / 32 / T | 3.780927 | 4.326075 | 4.260233 | 1.137967 | 0.978773 | no |

There are nine failed checks: calibration in all eight threaded cases and observer overhead in 2×2/16/T, whose largest ON/OFF block ratio is 1.111527. Other observer-overhead checks pass. Full block ratios remain in the archived analysis. No failure is widened or removed.

The instrumented primary 32-worker ranking reverses the preceding uninstrumented result: T appears faster than B here. Because calibration fails, that reversal is not an optimization result or evidence that the ordinary algorithm ranking changed. Disabled instrumentation, altered driver cadence and changed code generation are not isolated by this experiment.

## Qualified single-worker service

The following values are independent medians across nine retained primary single-worker graphs, in CPU milliseconds. They must not be added to reconstruct a median total. Service includes observer, allocator, point movement and bookkeeping costs inside the marked scopes.

| Covered stage | Original B | Ternary T |
|---|---:|---:|
| Streaming feeds | 23.023 | 24.155 |
| Group leaves | 0.010 | 0.025 |
| Indexed-top leaves | 7.776 | 0.000 |
| Weighted leaves | 3.047 | 4.719 |
| Merge leaves | 0.897 | 1.427 |
| Phi maps | 0.909 | 1.429 |
| Pre-normalization work | 0.185 | 0.078 |
| Weighted projective collectors | 0.139 | 0.216 |
| Final Horner combination | 0.036 | 0.073 |
| Total covered serial service | 36.032 | 32.143 |

Ternary removes the indexed-top route but increases several other stages. The final Horner increment is about 37 microseconds in this qualified single-worker observation; it does not explain a multi-millisecond improvement opportunity. Covered service excludes unmarked setup and normalization and is not total process CPU time.

Every graph checks eleven windows and two point partitions per window. Both algorithms execute forty ordinary feed scopes and twenty finish scopes. Weighted leaf counts are 81 for B and 131 for T; merge leaf counts are 47 and 77. Total graph event counts are 767/395 for primary B/T and 881/509 for secondary B/T, including the latter's grouping. Counts describe actual hook coverage, not equivalent-cost operations or retired instructions.

At one worker, window 10 returns last in all nine graphs for both algorithms; that reflects serial execution order. The indexed top has a different service cost from the ternary carry. In threaded graphs, last-window identities vary, but those graphs do not qualify as unperturbed evidence. Their enclosing spans include helped work and waiting. Mathematical availability markers are not enqueue timestamps, and leaf clocks do not identify runnable/idle states, DRAM bandwidth, page-fault service or exclusive critical-path causality. This observer is closed without a revised capture or scheduler change.

## Evidence and unchanged goal

Independent review verifies 148 successful command receipts including analysis, 144 query processes, 2,448 complete queries, 612 main observed trace-count summaries, 120 retained graphs, 76,560 event records and 2,015 source-pin occurrences. It independently reconstructs schedules, sample means, block bounds, service totals, graph relationships and nine failed checks, and checks the source and driver inverse edits. The initial analysis source was corrected before execution for the managed receipt's intentional omission of `RUSTC`; actual compiler arguments were already bound. No successful old test, trace, oracle or decoder is replayed. The verified-proof ledger remains 32,196, with no new SDK proofs.

Recorder payload is not a full memory bound. Maximum observed ON process RSS ranges from 15.25 to 134 MiB across these contexts and includes fixture loading, priming, recorder state, retained snapshots and allocator effects. It is neither production prover peak nor minimum required RAM. There is no new production cache, key table or execution plan, and no pressure/mobile/large-circuit/swap qualification.

The accepted SDK's inherited local proof times and supplied M5 references remain:

| Shape | Local CPU ms | M5 CPU ms | M5 Metal ms | CPU 10× limit ms |
|---|---:|---:|---:|---:|
| 1×2 | 8.041470 | 37 | 24 | 3.7 |
| 2×2 | 11.237839 | 59 | 30 | 5.9 |
| 4×2 | 16.143230 | 109 | 48 | 10.9 |
| 8×2 | 24.397442 | 189 | 74 | 18.9 |
| 12×2 | 33.736405 | 284 | 101 | 28.4 |
| 16×2 | 40.082192 | 335 | 124 | 33.5 |
| 24×2 | 65.765454 | 528 | 176 | 52.8 |
| Merge 24 | 65.350546 | 507 | 175 | 50.7 |
| Merge 54 | 161.495859 | — | — | — |

All-eight CPU 10× status remains 0/8. Cross-machine references do not establish equal hardware efficiency. This diagnostic provides no fresh AWS/Aeglos comparison, cloud inventory or cloud resource. The next optimization must have a distinct work-reduction mechanism; these calibration failures do not justify another observer or closed-candidate parameter reroll.

[Evidence archive](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), and [archive checksum](artifacts/SHA256SUMS) retain design, code, changed source parents/recorder, plans, receipts, graph text, analyses and review. Binaries, libraries, targets, private proving inputs and raw perf recordings are excluded. Preceding reports supply the original snapshot and oracle provenance. Original absolute paths identify the working environment and must be reconstructed before executing plans elsewhere.

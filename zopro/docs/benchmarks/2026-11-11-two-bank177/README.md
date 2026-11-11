# Two temporary G1 accumulators per bucket

**The work screen and native correctness prototype pass. Performance remains unmeasured; nothing is adopted.** The next step is a short complete-caller comparison including construction, final merging, destruction and peak memory.

Each logical bucket has two temporary affine accumulators sharing the original ready batch. An incoming point uses the first free accumulator, preferring bank0; it enters the existing LIFO queue only when both are busy. Queue capacity, flush threshold and sparse projective arithmetic stay the same. The final operation normalizes sparse projective contributions into bank0, then merges bank1 in one full affine batch. Equal-X exceptions from either bank route to the logical projective accumulator; cancellation clears only the selected affine accumulator. Key points remain borrowed.

This differs from the [two-arrival triple formula](../2026-11-06-g1-two-pending-count176/README.md) and [bucket-directed queue](../2026-11-09-bucket-ready-queue176/README.md). It changes accumulation state while retaining the existing ordinary affine-add kernel. There is no persistent table, expanded key, solver plan or new production pool.

## Primary K/H work model

The retained primary cache supplies 30,529 K and 30,922 H rows. All twenty original balanced spans, widths12/13 and capacities200/352 remain. Both models consume 1,170,733 signed nonzero/nonidentity arrivals. The one-bank model reproduces the prior baseline counters; all 1,229,020 extracted digits match an independent bit reader, and every final logical bucket conserves its signed integer-token sum.

|Generic noncancelling work|One bank|Two banks|
|---|---:|---:|
|First affine assignments|70,681|107,129|
|Streaming affine updates|1,088,726|1,063,206|
|Final bank-merge affine updates|0|36,448|
|Total affine updates|1,088,726|1,099,654|
|Queue arrivals|68,404|2,312|
|Projective arrivals|11,326|398|
|Adds to existing projective accumulators|6,911|397|
|Projective normalization buckets|4,415|1|
|Stream inversion stages|3,713|3,628|
|Final bank-merge inversion stages|0|20|
|Total inversion stages|3,713|3,648|
|Full eight-lane groups|136,081|137,438|
|Scalar-tail affine updates|78|150|

The candidate replaces 10,928 projective arrivals with affine updates. After charging all final bank merges, affine updates plus projective arrivals remain **1,100,052**. Projective accumulator adds fall94.3% and inversion stages fall1.75%; vector groups and scalar tails increase. All four prospective necessary checks pass. These are separate operation counts, not equivalent cycle costs, complete proof latency or a twofold gain forecast. Curve cancellations and exceptions are absent from the model and checked separately in the native prototype.

The second affine bank adds **4,587,520 bytes (4.375 MiB)** across all twenty windows, plus8,960 busy-bit bytes. This is **not total extra RAM**. The single final merge also allocates borrowed pairs and grows the existing prefix scratch beyond200/352 updates, potentially to a full2,048/4,096-bucket batch. Allocator growth, simultaneous liveness, admission and peak RSS are unmeasured. Production would need an admitted optional workspace and the original fallback; the private diagnostic has neither production dispatch nor that admission mechanism.

## Native qualification

The isolated snapshot has173 byte-identical original files, three original files with appended diagnostic adapters, and one new algorithm child. Existing field arithmetic and original one-bank Window bodies remain exact. Both complete bucket outputs are checked against independent Arkworks projective accumulation. Point inputs stay unchanged.

The first suite passes117 nonempty cases plus two empty cases. It covers sizes1/63/65/2048/4096, capacities8/200/352, four burst/spread/sign patterns, identity/repeated/opposite points, both-busy queues, spills, cancellation and final normalization. Pools of1/4/16/32 workers use512KiB stacks; four OS callers also share one eight-worker pool. Observed direct-feed events include20,329 secondary routes,274,698 queued operands,10,116 flushes,1,684 same-X cases and976 inverse-point cases. These counters do not include every event inside queue draining.

A separate follow-up reuses the same compiled native libraries and forces **both accumulators occupied in every bucket**. Four cases cross2,048/4,096 nonexceptional final merges with streaming capacities200/352. This adds12,288 forced bank merges and24,576 output comparisons without replaying the first suite. Total native bucket comparisons are **182,538**. These are public synthetic state tests, not actual query/SDK proofs, portable-device qualification or resource measurements.

The scalar screen takes0.031s. The corrected native build takes34.988s; its first suite takes0.124s and the dense follow-up0.073s. These are command durations excluding preparation/analysis, not candidate performance measurements. One earlier native compilation fails because the diagnostic child entry point was only parent-visible. Changing that visibility to crate-visible fixes the adapter; the failed source, pins and receipt remain. No successful numerical test is repeated. Nine workflow commands are retained: eight successes and that compile failure.

Separate same-agent readers verify count conservation, batch histograms, source recovery, pins, command/stream receipts, native build linkage and the unchanged proof ledger. They do not replay the numerical model or independently reimplement curve arithmetic. A preparer has a retained unused-import warning; no warning-only rebuild occurs.

## Continuation

Freeze a short complete-caller screen against the actual one-bank implementation, retaining the three executable backings and representative worker counts. Charge the full bank merge, larger scratch, allocation/drop and memory high-water mark. Use real input distributions and correctness checks. Do not infer proof gains from spill counts, omit the large final merge, or start an all-shape SDK campaign before this mechanism survives a caller screen. A useful result then needs admitted production integration and missing release checks.

Accepted local primary proof remains **7.821726 ms /127.5 MiB peak RSS**, against the3.7ms CPU target. The [all-eight CPU and Metal comparison](../2026-11-11-cost-map177/README.md#accepted-full-proof-reference) is unchanged; the verified CPU10× goal remains **active and unmet,0/8**. Canonical verified exports remain **53,917**, SHA-256 `a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873`. No new proof, SDK/AWS/Aeglos timing, production change, push or publication occurs.

[Model protocol](artifacts/protocol.md), [native protocol](artifacts/native-protocol.md), [work counts](artifacts/result.json), [native child](artifacts/two_bank-fixed.rs), [review](artifacts/review.json), [dense review](artifacts/dense-review.json), [sources/plans/pins/receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS). Private input caches, point/key/witness/proof payloads, unchanged production sources and compiled binaries are excluded. Reproduction requires the pinned local inputs and dependencies.

# Bounded replay of production G2 arithmetic batches

30 October 2026. A diagnostic capability following the [negative bounded-delta SDK result](../2026-10-30-loose-delta-sdk175/README.md). Production remains unchanged. This is not a latency comparison or an adoption experiment.

The recent synthetic caller benchmark improved while the SDK regressed. This stage captures actual production batch operands, signs, lengths and results, then replays the unchanged kernel with independent Arkworks addition checks. It supplies a more representative arithmetic input set; it does not claim to reproduce proof scheduling or predict proving speed.

## Capture and replay

Two private G2 prefix entry/exit hooks record owned and signed-borrowed calls. The census includes all lengths, full vector groups, tails and negative signs across four fresh requests per shape. At most four batches per caller and fixed length band are retained: 8–31, 32–127, 128–255, 256–511 and 512 pairs. Sub-vector and larger calls remain in the census but are not captured. Each file contains only touched old buckets, incoming points, encoded destinations and outputs. No whole key or bucket array is copied.

At most 40 files of at most 32+388×512 bytes are written per process: **7,947,520 bytes**, below 8 MiB. Raw coordinates remain local with hashes. The first calls in each band are diagnostic samples affected by parallel execution order, not a random or frequency-proportional population sample.

Replay preserves field operands, signs, point order and tails. It compacts distinct destinations and incoming point storage, so it does not preserve key/bucket locality or original addresses. Repeated calls include resetting destination copies; borrowed reference construction and independent oracles precede any future timing loop. No replay timing was collected here.

## Observed populations

The calls/points below cover all three primes and one measured request; instrumentation makes these unsuitable for proof-latency comparisons. Prefix population excludes other curve kernels and exceptional fallback paths. Full length/sign histograms and each captured batch’s hash are in the analysis.

|Shape|Prefix calls|Point contributions|Tail points|Negative contributions|Captured batches|Captured pairs|Local operand KiB|
|---|---:|---:|---:|---:|---:|---:|---:|
|12x2|19688|4042716|1404|0|16|5387|2041.67|
|16x2|22264|4553276|1308|0|16|5342|2024.62|
|1x2|10608|2149392|592|1032532|20|3323|1259.73|
|24x2|29060|5897532|1308|0|20|5142|1948.96|
|2x2|10940|2224384|608|1069012|16|3152|1194.81|
|4x2|12636|2618524|2044|0|16|4937|1871.16|
|8x2|16008|3290632|1544|0|16|5090|1929.13|
|merge-24x1|27048|5509916|1212|0|16|5337|2022.72|
|merge-54x1|51404|10378576|1328|0|16|4948|1875.33|

All **152 captured batches / 42658 point pairs** independently match Arkworks signed addition. Every fixture also runs one reset call followed by three reset calls through the unchanged native prefix kernel, with outputs equal to the captured production results. Total raw operand payload is **16556168 bytes**, kept locally.

## Population weighting and next test

The dominant observed prefix context is 200 pairs for every tested shape. The table accounts for all observed prefix contributions, not estimated CPU or wall-time costs.

|Shape|Dominant caller / pairs|Point contributions|Share of prefix contributions|
|---|---|---:|---:|
|12x2|owned / 200|3827200|94.669%|
|16x2|owned / 200|4342400|95.369%|
|1x2|signed borrowed / 200|2052000|95.469%|
|24x2|owned / 200|5701600|96.678%|
|2x2|signed borrowed / 200|2118400|95.235%|
|4x2|owned / 200|2416800|92.296%|
|8x2|owned / 200|3091200|93.939%|
|merge-24x1|owned / 200|5299200|96.176%|
|merge-54x1|owned / 200|10170400|97.994%|

For 1×2, signed borrowed 200-pair calls contribute **2,052,000 of 2,149,392 points (95.469%)**. Owned 512-pair calls add 81,920 points (3.811%); the earlier synthetic caller matrix did not time size 512. Its 18-context aggregate instead weights owned/borrowed 32/200/352 and one/16/32 workers equally. That aggregate is valid within its declared scope, but it is not the production population.

The earlier bounded-delta caller’s dominant signed-borrowed 200-pair/32-worker comparison has ON/OFF ratio **0.995330931**, bounds **[0.993716932, 1.002293063]**, and **10/15 wins**. Its interval spans parity even though the all-context aggregate passes. This did not establish an isolated gain in the main observed context. These population facts do not prove why the full SDK became slower, and neither old gate is reopened or relabeled. [Earlier caller observations](../2026-10-30-loose-delta175/README.md).

A source/history review identifies a distinct next hypothesis: remove multiplication by one at the first G2 norm-prefix group and first reverse factor, and omit the dead final inverse update. The previous endpoint experiments changed G1’s stored-numerator scheme; they remain rejected. The current G2 prefix stores only chain/norm records, so this hypothesis changes a different recurrence. It is unimplemented and has no speed prediction. Before timing, emitted code must be checked for existing elimination; a candidate would need the real 200/512-pair fixtures, tails, the dominant caller, and complete-proof qualification. [Earlier G1 endpoint result](../2026-10-25-batch-endpoints-sdk/README.md).

A fresh paper audit rechecked [SRE](https://arxiv.org/abs/2610.27452) against the existing [shared-window queue trial](../2026-10-25-shared-point-window-queue/README.md). Its reuse mechanism already has local studies; no paper speedup or implementation is transferred to this diagnostic.

## Qualification and records

The private source has 176 files: 171 of 175 originals are byte-exact; reversing four parent edits and removing the child recovers all 175. All 17 field source files and every arithmetic body are unchanged. Native qualification passes 127 unfiltered core tests and eight prover tests. Three new tests cover codec/sign/tail/reuse behavior across 14 lengths, malformed/ambiguous records and exact capture bounds. Field qualification is inherited from unchanged sources. No portable build is claimed.

The modified standard SDK consumer adds only diagnostic finalization after its request loop. All **36 SDK requests** internally verify. All **nine fresh 164-byte exports** independently pass pinned Gnark with exact public/image/verification-key bindings and freshness against 23,962 prior identities; the distinct-proof ledger is **23,971**, with 18 proof/public archive members. All captures finish before independent verification, coordinate audits, hashing and publication. No independent owned CPU workload overlaps capture. Original helpers are compiled and pinned before capture.

Initial setup stopped before any native compilation because the source audit decoded a binary member as UTF-8. The audit now decodes only edited Rust files; initial source and failure description are retained. The first standalone auditor compilation also stopped before execution: Arkworks 0.5 exposes the ZERO constant through AdditiveGroup, whose import was missing. Adding that import fixed compilation; the successful native suites were reused. Original auditor/controller sources and compiler output are retained. No successful test, capture or proof verification was repeated. Source, command receipts and proof archives round-trip byte-exactly. Raw coordinates are not included.

[Analysis and histograms](artifacts/analysis.json), [capture plan](artifacts/plan.json), [design and limits](artifacts/design.md), [source recovery](artifacts/source-audit.json), [recorder/replay](artifacts/g2_replay.rs), [independent auditor](artifacts/fixture-audit.rs), [complete private source](artifacts/source.json.gz), [native/capture/audit receipts](artifacts/receipts.json.gz), [proof bytes](artifacts/proofs.json.gz), [proof inventory](artifacts/proof-inventory.json), [local coordinate hashes](artifacts/raw-coordinate-pins.json), [archive review](artifacts/archive-review.json).

The next arithmetic comparison can consume these real operands alongside the existing synthetic cases. It must declare its own complete-caller comparison and still pass full SDK, resource and final-artifact qualification before adoption. The rejected bounded-delta gate remains failed. No new speed, RSS, minimum-RAM, memory-pressure, mobile/ARM/TEE, huge-circuit, concurrency or AWS/Aeglos result is established.

## CPU and Metal references

These accepted local results and supplied historical M5 references are inherited, not new measurements. All eight CPU 10× targets remain unmet. Accepted local 1×2 remains 8.022165 ms / 134 MiB. Latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96; the historical physical-16 adoption exception is unchanged.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms / 1831.87 MiB, with no supplied M5 reference. [Current comparison](../2026-10-29-current-cpu-summary/README.md), [dated benchmark report](../2026-10-30-cpu-benchmark-report/README.md).

[Offline population review](artifacts/population-review.json) and [review source](artifacts/postcapture-review.rs) add interpretation after capture; all original pinned helpers, raw captures and analysis remain unchanged.

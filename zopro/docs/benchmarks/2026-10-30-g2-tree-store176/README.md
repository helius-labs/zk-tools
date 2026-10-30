# G2 tree coordinate output stores

The output-only G2 coordinate-store candidate was **not adopted**. Native correctness passed, but the fixed complete-tree caller comparison failed its performance gate. Production source176 and its accepted proof results are unchanged.

The candidate keeps field canonicalization, input packing, arithmetic, inversion, pair order, tree geometry and exceptional handling. It combines canonical component words into direct 64-byte Fq2 stores at existing destinations. It adds no production heap allocation, table, key copy or execution plan. The diagnostic caller owns resettable input copies; these are benchmark fixtures, not an architectural change.

## Complete-tree results

Each cell uses 15 separate-process blocks, two variants and 128 complete reductions per worker after three primes. Times below are median wave microseconds per iteration, including identical input reset and worker/barrier tail. These are **tree-caller timings, not proof latency**. The paired ratio median can differ from the ratio of the two marginal medians. Values below one favor the candidate.

| Points | Workers | Control µs | Candidate µs | Paired ratio | Paired interval | Wins |
|---:|---:|---:|---:|---:|---:|---:|
| 256 | 1 | 16.645 | 16.916 | 1.015725 | [1.013501, 1.018643] | 0/15 |
| 1024 | 1 | 48.692 | 49.233 | 1.011122 | [1.008291, 1.012696] | 0/15 |
| 256 | 16 | 34.611 | 34.141 | 0.985223 | [0.914742, 1.050037] | 8/15 |
| 1024 | 16 | 83.276 | 80.857 | 0.992236 | [0.788984, 1.064675] | 8/15 |
| 256 | 32 | 44.889 | 45.923 | 0.997396 | [0.823744, 1.060522] | 8/15 |
| 1024 | 32 | 118.505 | 120.685 | 1.023463 | [0.863945, 1.102653] | 6/15 |

The predeclared primary equal-log aggregate across both 32-worker sizes was **0.951082 [0.890833, 1.050479], 9/15 wins**. It fails the required upper bound below one and at least 10 wins. The context upper-bound limit of 1.03 also fails. Threaded results have wide dispersion; the lower aggregate median is not established as a gain. Both single-worker cases were consistently slower. No exclusions, block extensions, geometry sweep, unchanged reroll or full-SDK measurement followed this failure.

Intervals use ranks 4–12 of 15 paired values: nominal marginal 96.484375% sign intervals, without a family-wise guarantee. There is no positive-gain floor. Whole processes were bound to CPU 2, CPUs 0–15 or CPUs 0–31 respectively on this 16-core/32-thread host; individual workers were not pinned. A Rayon pool with the declared worker count executes the reductions, while an external coordinator controls ready/start/done/release barriers.

## Correctness and emitted code

Unfiltered native core suites passed **127 OFF / 129 ON** tests. Two shared tests cover complete generator-sum tree references, tails, exceptional inputs, empty buckets, reset/reuse and 1/4/32-worker pools with 512 KiB stacks. Two candidate-only tests cover scalar field references, lazy/boundary values, permuted destinations and untouched holes. All 18 accepted field source files are byte-identical; the existing 35 native field tests are inherited, not rerun.

The actual linked reducer emitted direct 64-byte destination stores. Its static size changed from 155,349 to 155,859 bytes; instruction count from 26,650 to 26,601; RSP-operand instructions from 4,778 to 4,889. Prologue stack subtractions total 7,616 versus 7,232 bytes, excluding pushes, alignment and callees. IFMA and scalar `mulx` counts are unchanged. Static counts do not measure executed work, spill traffic or removable latency. The smaller stack frame did not establish a complete-caller gain.

All 180 processes completed: **385,140 complete tree calls**, including 376,320 timed calls, with 11,760 independently constructed generator-sum checks across primes and final outputs. Intermediate timed iterations are not checked individually. Reference construction, allocation and output checks are outside the clock. No new proof, verifier export or CPU profile was produced; the canonical verified-proof ledger remains **27,353**. The prior instruction census supplied the profiling evidence for this hypothesis; its 5.93% generic G2 reduction sample share was never a promise of removable wall time.

The private snapshot contains 178 files: 173 accepted originals unchanged, three changed parents and two diagnostic/codec children. The control retains the original output body. The prebuild driver was changed from `scope` to `in_place_scope` before any build or timing so the coordinator does not occupy a worker at its barriers; both prebuild versions and their plans are retained. No successful test or timing run was repeated.

## Accepted proof comparison — inherited, unchanged

These are the previously accepted local 32-worker SDK measurements, not measurements of this rejected candidate. M5 references are the supplied CPU and Metal timings. Speedups are reference latency divided by accepted local latency. No fresh AWS, Aeglos, phone/ARM/TEE, huge-circuit, concurrent-proof or minimum-RAM measurement is claimed.

| Shape | Zopro ms | Peak RSS MiB | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | 10× CPU target ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 132.75 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2x2 | 11.151932 | 275.00 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4x2 | 16.086386 | 312.25 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8x2 | 24.179453 | 517.25 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12x2 | 33.884455 | 706.50 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16x2 | 40.077381 | 842.00 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24x2 | 65.850777 | 1300.50 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24 | 65.060937 | 1168.19 | 507 | 7.793× | 175 | 2.690× | 50.7 |

The all-eight 10× CPU objective remains unmet: **0/8** targets achieved. The primary accepted result is 7.928660 ms against a 3.7 ms target. The inherited 54-1 merge result is 161.883899 ms / 1,876.96 MiB peak RSS, without a supplied M5 reference. Latest AWS comparisons predate accepted source176; this study changes none of them.

## Evidence

[Design](artifacts/design.md), [raw rows and analysis](artifacts/caller-analysis.json), [independent review](artifacts/review.json), [linked-code counts](artifacts/codegen.json), [timing decision](artifacts/codegen-decision.json), [source audit](artifacts/source-audit.json), [accepted proof rows](artifacts/accepted-local-rows.json).

Archives retain the [exact private source](artifacts/private-source.tar.gz), [workflow sources and frozen plans](artifacts/workflow.tar.gz), [native command/test/link receipts](artifacts/native-receipts.tar.gz), [all 180 caller receipts](artifacts/caller-receipts.tar.gz) and [linked disassembly](artifacts/codegen.tar.gz). Executables and frozen libraries stay local with recorded hashes. No key image or private witness data is included. [Checksums](SHA256SUMS).

Related evidence: [accepted integration](../2026-10-30-g2-vector-sign-adoption176/README.md), [instruction census](../2026-10-30-instruction-census176/README.md), [earlier joint G1 point I/O experiment](../2026-10-24-point-io-component/README.md), [G2 staged arithmetic](../2026-10-30-g2-staged175/README.md). This result closes the exact output-only G2 store candidate; it does not establish that all coordinate-layout approaches are ineffective. Further work toward the required large gain needs a distinct mechanism that reduces dominant MSM work.

# Native segmented SIMD G1 columns

**Rejected at the complete-filler gate; no production adoption.** The32-worker equal-log aggregate is14.428%slower: paired ratio1.144285[1.119459,1.162425],0/15winning blocks. Every case fails the3%context upper-bound limit. Production source176, accepted proof timings and the verified-proof ledger remain unchanged.

The prior structural screen found about49%fewer logical field conversions with unchanged point-addition counts. The complete native implementation performs more total work and uses more memory. Logical conversion counts did not predict an overall gain. No fullSDK or proof-latency trial follows this failed component result.

## Implemented mechanism

For each ordinary signed width12window, histogram/scatter borrowed input indices, split original buckets into segments of at most8points, sort similarly sized segments into8-lane cohorts, and keep their coordinates in native SIMD form through each update. The bounded circular schedule runs at most25cohorts per wave. Three-field prefix records use the existing inverse and affine addition formula. Export partial sums once, then merge them using the existing256-original-bucket group tree.

New handwritten field helpers canonicalize normalized values below2p without a host transpose, compare canonical coordinates and select lanes. Inactive, identity and equal-X cases use denominator1; complete scalar group operations handle identity/doubling/cancellation, and only ordinary lanes receive the vector result. New coordinates are canonical before the next canonical signed-difference step. Generated arithmetic and generators remain byte-identical. No per-key table, transformed/copied key, second solver plan, assembly, pool or production hook is added. Proof-local native columns, partial points and index metadata are additional scratch.

The control executes the original borrowed_pending Window::new/feed/finish bodies through a diagnostic wrapper. The candidate is a separate private snapshot; the failed unsplit-column layout and earlier G2 output-store result remain closed.

## Complete ordinary-filler latency

Each pass fills all21ordinary windows of one raw pre-group Aquery. Loading, independent oracle construction and pool creation are excluded. Histogram/sorting/scattering, all field and exception work, range repair, export/partial merging, allocation, output drop and pool dispatch/wait are included. Narrow top, final weighting, solving and proof verification are outside this scope. These are **filler timings, not proof latency**; raw inputs are not automatically final grouped-query populations.

Fifteen fixed rotated/reversed paired blocks use20timed passes per process after3primes, followed by1post-check, on the shared Ryzen9950X host. The same Rayon pool supports outer windows and candidate nested merges. Whole-process affinity is CPU2solo,0–15physical or0–31logical; workers may migrate within it. No owned build/hash/archive/second workload overlaps measurements.

| Raw shape | Workers | OFF ms/pass | ON ms/pass | Paired ON/OFF | Paired bounds | Wins/15 |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 1 | 10.566379 | 12.112119 | 1.146858 | [1.143806, 1.148143] | 0 |
| 1x2 | 16 | 1.066466 | 1.283033 | 1.191238 | [1.088519, 1.214930] | 1 |
| 1x2 | 32 | 0.854247 | 0.935392 | 1.106098 | [1.088332, 1.128970] | 0 |
| 2x2 | 1 | 16.716079 | 20.701655 | 1.240415 | [1.237781, 1.243271] | 0 |
| 2x2 | 16 | 1.651921 | 2.156809 | 1.290618 | [1.283448, 1.340248] | 0 |
| 2x2 | 32 | 1.330575 | 1.580762 | 1.182040 | [1.133989, 1.223301] | 0 |

The gate requires primary aggregate upper ratio<1 and at least10/15wins, plus every context upper≤1.03. All fail. Bounds use paired order ranks4–12/15, nominal marginal96.484375%conditional coverage without a family-wise guarantee. Marginal medians and paired medians need not divide identically. There is no gain floor, excluded block, sample extension, unchanged reroll, favorable subset or layout/width/segment rescue in this experiment.

## CPU service, faults and memory

| Raw shape | Workers | OFF CPU ms/pass | ON CPU ms/pass | OFF/ON maximum sampled process RSS MiB | OFF/ON median minor faults/pass |
|---|---:|---:|---:|---:|---:|
| 1x2 | 1 | 10.567 | 12.112 | 14.25 / 14.50 | 0.15 / 0.10 |
| 1x2 | 16 | 11.750 | 14.227 | 18.50 / 30.75 | 11.55 / 59.65 |
| 1x2 | 32 | 15.851 | 22.332 | 19.50 / 33.25 | 18.20 / 39.70 |
| 2x2 | 1 | 16.716 | 20.701 | 17.00 / 17.75 | 0.10 / 0.10 |
| 2x2 | 16 | 18.121 | 24.058 | 20.00 / 39.25 | 9.70 / 73.70 |
| 2x2 | 32 | 23.860 | 36.035 | 19.00 / 45.75 | 18.55 / 36.25 |

RSS is the process high-water sampled at the end of the timed loop, including loading, oracle arrays and primes; it is not minimumRAM or a full prover peak. Minor faults are process rusage deltas over the warm measured loop. They usually service resident/anonymous mappings without storage I/O, but this measurement does not isolate their cause or prove that faults explain the latency. The new native coordinate and exported partial arrays overlap during export, with additional maps, records, generic merge workspace and allocator effects. The count-stage payload estimates were not a memory-neutral architecture claim.

## Warm counters and profiles

After all latency, four grouped stat panels and four separate cycles:u profiles run200passes after3excluded primes on primary1x2 at16/32workers. This diagnostic duration was frozen before capture; the latency pass count remains20. FIFO acknowledgements exclude loading/primes and post-checks. All stat events run100%; single panels remain descriptive.

| Workers / arm | Cycles M/pass | Instructions M/pass | Generic cache misses M/pass | Branch misses M/pass |
|---|---:|---:|---:|---:|
| 16 / control | 60.233 | 145.908 | 0.108228 | 0.117555 |
| 16 / candidate | 72.332 | 170.320 | 0.806328 | 0.215620 |
| 32 / control | 76.696 | 150.336 | 0.149466 | 0.125600 |
| 32 / candidate | 102.700 | 175.682 | 0.908400 | 0.232859 |

At32workers, the candidate counter panel has16.86%more instructions and about6.08×the generic cache misses. These observations accompany higher CPU service and RSS; they do not isolate RAM bandwidth, a cache level or a causal fraction of the slowdown.

| Profile | Samples | Unknown share | Lost records |
|---|---:|---:|---:|
| 16 / control | 655 | 0.000% | 0 |
| 16 / candidate | 781 | 0.768% | 0 |
| 32 / control | 795 | 0.126% | 0 |
| 32 / candidate | 1057 | 1.608% | 0 |

Candidate32flat sampled-period shares include59.98%in the column-fill defining function,11.35%in the generic partial-merge caller context and5.11%in sort-function contexts. This is consistent with substantial bookkeeping and merging around the saved codecs. The fill function itself also contains inlined metadata/field/memory work; these shares cannot isolate arithmetic or be subtracted as removable wall time. Three profiles have fewer than1000samples. All3288samples are tied to one expected executablePID per recording, with all sampledTIDs matched to actual workerFORK records, allowed CPUbounds, fixed16Mperiod and zero losses. Absolute period-to-wall coverage is not validated.


The actual linked candidate fill is27,615bytes/4,956static instruction sites. The control keeps a17,927byte wrapper and two stream dispatch alternatives (the canonical stream is26,127bytes/4,414sites). These contexts have different inlining/scope, so sizes or RSPoperand counts are not a matched dynamic cost or spill comparison. Actual binaries are hash-bound in the linked-code receipt.

## Correctness, retained failures and provenance

Native control passes125unfiltered core tests; candidate passes128, including3new generator-bucket, complete exception/skew/tail and1/4/32worker512KiB-stack tests.35unchanged base field tests passed once, and the two new field helper tests separately pass against Arkworks with those35filtered out. This is inherited35plus2new qualification, not a freshly repeated37-test suite. No standalone prover/portable suite or SDK proof trial is claimed.

Two new independent Arkworks real-input oracles validate all21,006/36,637input points and canonical scalars, then compute all43,008bucket outputs per shape. They reuse saved scalar caches, with zero new solves. Every process checks all buckets in3prime passes and1post-check. All200processes complete:6,012ordinary-fill passes including checks/primes,126,252individual window fills and34,406,400bucket comparisons. Timed intermediate passes are not individually checked. No new SDK proof request or independent export occurs; ledger remains27,353.

Initial native preparation wrongly placed core's ifma8feature condition on the field child and omitted three FqBatchforwarders. Candidate core compilation failed before new helper or point tests ran. A separate corrected178-file snapshot removes that field-only condition and appends the forwarding methods. Original helper/filler arithmetic bodies are byte-identical;172accepted originals remain exact, with4append-only parents and2children. Successful control/base-field results are inherited; only unfinished/new tests and builds continue. Both snapshots and the compiler failure remain in evidence.

All200measurements completed before a read-only decoder encountered root-owned0600recordings. Ownership changed only on those four generated files after saving hashes, and every byte hash remained unchanged. The first resume reused all200processes but refused the already failed decoder directory. That directory was retained separately; a second resume reused all200again and executed only16read-only decoding commands. Final216stepresult is complete. No latency/profile/proof/test measurement repeats.

Independent review recomputes all six latency/CPU/RSS cells, paired bounds and failed gate from original command/exit/stdout/stderr receipts, all four grouped counter panels, every profile sample/symbol bin and workerPID/TID/CPUownership. It rechecks exact source/library/executable/oracle hashes and production176. No test, model, solve, capture or verifier is rerun by review.

## Accepted proof comparison — inherited, unchanged

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


No new AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap, concurrent-proof or minimumRAM qualification follows. This result closes the exact all-columns/circular-wave implementation; a future distinct mechanism needs its own protocol and evidence.

[Design](artifacts/design.md), [frozen capture protocol](artifacts/capture-design.md), [all observations](artifacts/analysis.json), [symbols](artifacts/symbols.json), [independent review](artifacts/review.json), [linked code](artifacts/codegen.json), [source audit](artifacts/source-audit-fixed.json), [preparation repair](artifacts/repair.md), [decode repair](artifacts/access-repair.md). Archives: [original private source](artifacts/source-initial.tar.gz), [corrected private source](artifacts/source-fixed.tar.gz), [workflow and helper sources](artifacts/workflow.tar.gz), [all command receipts](artifacts/receipts.tar.gz), [linked disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

The four original [counter CSV files](artifacts/stat.tar.gz) are retained separately. Raw key/scalar/point/oracle arrays, executables, immutable libraries and perf.data remain local with recorded hashes. No private witness-derived point outputs are uploaded. Related: [structural screen](../2026-10-30-g1-column-count176/README.md), [accepted production](../2026-10-30-g2-vector-sign-adoption176/README.md).

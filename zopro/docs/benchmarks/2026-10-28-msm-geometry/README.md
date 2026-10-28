# Real-request MSM batch and collision geometry

The real 1x2 prover uses batches of 200 or 352 additions; only 26 of 7,234 streaming flushes contain fewer than 64 entries. **The previous uniform small-triple caller gain does not justify a direct production replacement.** Queued collisions are frequent inside the larger batches, supporting a distinct mixed binary/triple experiment. These counters establish no saved inversion, CPU time or proving speedup.

Three separate 32-worker processes use the accepted image/request and configuration on CPUs 0–31. Each runs three unobserved primes followed by one observed fresh SDK proof. All three count maps are identical. This diagnostic has no latency timer or PMU capture; instrumented time and RAM are not reported as performance results. Accepted local 1x2 remains 8.324853 ms at 134.50 MiB peak RSS.

## Per-proof observations

| Stream | Windows | Flushes | Executed additions | Queued arrivals | Projective queue updates | Flushes below 64 |
|---|---:|---:|---:|---:|---:|---:|
| G1 borrowed, 2048 buckets, batch 200 | 21 | 1722 | 343214 | 28619 | 5331 | 1 |
| G1 borrowed, 4096 buckets, batch 352 | 19 | 2907 | 1020235 | 65305 | 14188 | 0 |
| G2 owned, 2048 buckets, batch 200 | 20 | 2605 | 516476 | 14906 | 1178 | 25 |

Queued arrivals count actual insertions, including the specialized G2 low-component and owned fused feeds. Projective queue updates count entries diverted through the existing projective path. All pending pushes equal executed entries. These totals count digit updates across windows, not unique key points.

| Stream | Flushes with matching queued destinations | Matching destinations over snapshots | Matching queued entries over snapshots |
|---|---:|---:|---:|
| G1 batch 200 | 1722 | 25846 | 28089 |
| G1 batch 352 | 2907 | 56550 | 66800 |
| G2 batch 200 | 2597 | 32673 | 41387 |

The second table is deliberately a snapshot census. A queued entry can survive several flushes and be counted repeatedly; matching entries can exceed actual arrivals, as in G2. These are not unique triple opportunities or avoided inversions. Each snapshot separately checks queued entries = matched entries + free entries, and matched entries = matching destinations + repeats. Histograms reproduce the independently accumulated flush counts, executed entries, matches and projective queue totals.

G1 batch-200 flushes have 8–16 matching destinations in 1137 cases and 17–64 in 346; batch-352 has 8–16 in 1166 and 17–64 in 1680. G2 has 8–16 in 1871 flushes. This motivates testing a sparse set of triples alongside ordinary additions, sharing the existing global inversion. It does not establish that moving those entries forward reduces subsequent flush count or projective work after charging extra multiplication, indexing and queue compaction.

The binary reduction census observes 5631 G1 calls with 310890 ordinary pairs and 3935 G2 calls with 95094 pairs. Ideal triple-slot bands 8–16/17–64 occur in 677/1828 G1 calls and 535/1111 G2 calls. These counts combine plain reductions and weighted bucket reductions. The latter require the existing binary-halving identity and copied even-index points; substituting ternary reduction into that loop would change the weighted sum. Ideal slots are neither validated point triples nor a drop-in integration opportunity.

## Validation and scope

Three new observer tests pass: signed queue classification with repeats/free entries, tree census with tails and actual ordinary-pair counts, and gated parallel publish/reset excluding warmup. Two specialized queue-arrival hooks were added after those tests without changing the tested observer module bytes. The final SDK build, internally verified proofs, independent Gnark checks and counter invariants qualify those hooks; successful observer tests were not repeated.

The private snapshot adds one counter module and diagnostic hooks in six original files. Inverse edits recover all six originals exactly; 166 other private original files and all 172 production files remain unchanged. Arithmetic kernels, scheduler, inputs and memory policy retain their original code. Window-local maps publish on drop; capture begins after warmup and ends after the SDK call returns. Diagnostic allocations are not a proposed production layout or resource bound. No owned build/helper/hash/archive or other workload overlaps observed SDK calls.

All three exported proofs are fresh against 12,524 prior hashes, have the expected public binding, and pass the existing pinned independent Gnark verifier. **12 complete internally verified SDK requests, three fresh independent exports, six proof/public members; ledger 12,524 to 12,527.** All source, executable and input pins match. There are no failed builds, tests, SDK observations or verification attempts, and no completed observation was repeated.

Only the critical 1x2 request is observed here. No other-shape geometry, AWS/Aeglos comparison, new peak RSS, minimum RAM, pressure, phone, TEE or large-circuit result is inferred. No production change or adoption follows.

Next: model and implement a mixed signed borrowed flush on actual 200/352-entry geometry, combining a sparse collision set with ordinary additions. Include signed operands, complete fallback, data movement, queue order, bounded metadata and resource admission. Require a real caller gain and complete SDK/proof verification before promotion. The existing large-G2 regression rules out applying the uniform triple formula everywhere.

## Inherited CPU and Metal comparison

The [accepted report](../2026-10-27-clean-fft-adoption/README.md), including its full-performance-gate failure and engineering exception, remains authoritative. M5 figures are supplied historical references. The original all-eight CPU 10x goal remains active and unmet, 0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains 161.341835 ms without a supplied M5 reference.

[Protocol](artifacts/design.md), [all counts](artifacts/analysis.json), [interpretation and conservation](artifacts/interpretation.json), [observer tests](artifacts/observer-tests.log), [final source audit](artifacts/source-audit-extended.json), [proof inventory](artifacts/proof-inventory.json), [pins](artifacts/pins.json), [workflow completion](artifacts/runs/result.json), [artifact hashes](artifact-index.json).

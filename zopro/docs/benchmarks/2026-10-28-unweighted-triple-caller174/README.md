# Complete unweighted G1 ternary reduction

The complete caller fails its declared integration gate. The 2,048-point indexed tile is **9.66% slower**, and the large grouped caller is **7.86% slower**. The mixed-tail fixture improves **10.35%**, but all four main cases fail. Production remains unchanged; accepted local 1×2 latency is **8.366931 ms**. [Decision](artifacts/interpretation.md).

The [first-layer census](../2026-10-28-unweighted-triple-census174/README.md) found real G1 indexed tile work beyond the range of the earlier [small three-point caller](../2026-10-28-triple-callers/README.md). This experiment tests a complete unweighted reduction at representative sizes, using modeled point inputs and deterministic index permutations. It does not replay private key points or measure proving latency.

Every call gathers prepared affine points, initializes counts, performs all reduction layers, compacts and writes affine outputs. Eligible layers combine three points when at least eight triples exist across the caller; smaller layers use the unchanged binary reduction. Two-point remainders share the same inversion chains. Partial SIMD groups use nonzero dummy denominators and suppress dummy stores. Any input identity or zero denominator rejects the layer before writes, allowing complete binary fallback. Weighted trees, streaming MSM, field arithmetic, key formats and production resource forecasts are unchanged.

## Caller results

Ryzen 9950X, CPU 2, one Rayon worker. Both variants are in one native executable. Each process performs three eight-call primes with checks, then its fixed timed calls and a final check. Indexed tile buffers are created and dropped on every call; group buffers are reused. The native caller includes allocation costs for the fresh-buffer cases.

There are 31 rotated/reversed paired process blocks per case: 434 latency processes and 14 preflights. All observations remain. Change uses the median within-block ternary/binary ratio, not the ratio of displayed medians. Intervals use ranks 10..22 of 31, nominal 97.055% under independent/exchangeable blocks, without a familywise guarantee. The per-case condition is an upper ratio below one and at least 21 wins, with no minimum gain floor. SDK integration requires all four main cases to pass; that condition is **false**.

| Case | Calls/process | Binary µs | Ternary µs | Paired change | Ratio interval | Wins | Main case |
|---|---:|---:|---:|---:|---:|---:|---|
| Tile257 | 1024 | 12.249 | 12.620 | −3.06% | 1.02849–1.03380 | 0/31 | No |
| Tile384 | 1024 | 15.604 | 15.628 | −0.38% | 0.99841–1.00669 | 10/31 | Yes |
| Tile511 | 1024 | 18.553 | 19.575 | −5.30% | 1.05012–1.05679 | 0/31 | Yes |
| Tile2048 | 256 | 54.326 | 59.553 | −9.66% | 1.09468–1.10323 | 0/31 | Yes |
| Group35973 | 16 | 911.180 | 981.903 | −7.86% | 1.07156–1.08335 | 2/31 | Yes |
| Groups31×256 | 64 | 192.279 | 209.427 | −8.83% | 1.07758–1.09294 | 1/31 | No |
| Mixed tails | 1024 | 11.306 | 10.133 | +10.35% | 0.89230–0.90052 | 31/31 | No |

Mixed tails use counts 0/1/2/3/7/8/9/23/24/25/26/47/48/49. The large group has 11,991 initial triples, matching the largest G1 job count from the census. The 256 groups of 31 points model a grouped caller and are not a captured production distribution. Tile384 is uncertain near parity; the data do not prove that every case is slower. The positive tail result is retained without changing the predeclared integration condition.

## Correctness and memory

All 124 unfiltered native core tests pass, including two new tests for full reductions, tails, multiple groups, exceptional points and unchanged input/counts on rejected layers. An external test passes against independent Arkworks generator scalar sums, covering full-width scalar arrays through 2,048 points, mixed groups, partial lanes and 216 boundary triples with zero, equal/opposite points and cancellation. Both buffer lifetimes and both algorithms are checked. Every captured caller output matches its projective reference and the other variant. The field source is exact; this is not a fresh field-suite or portable-backend qualification.

Eight FqBatch words are retained per group of eight triples; ordinary pairs use five. The memory columns below add common vectors, retained binary scratch, ternary scratch and fixed Work storage. Fresh-buffer accounting also includes the retained previous output and both Work structs. Values describe endpoint capacities after a call, **not peak RSS, transient allocation peaks, key storage or minimum RAM**. Inputs and independent references live outside these totals.

| Case | Binary bytes | Ternary bytes | Allocation calls/call, binary/ternary |
|---|---:|---:|---:|
| Tile257 | 45,136 | 49,048 | 11/10 |
| Tile384 | 67,088 | 70,320 | 12/10 |
| Tile511 | 86,416 | 94,176 | 12/10 |
| Tile2048 | 346,128 | 360,416 | 14/9 |
| Group35973 | 6,162,264 | 6,239,984 | 0/0 |
| Groups31×256 | 1,327,864 | 1,473,272 | 0/0 |
| Mixed tails | 46,952 | 49,848 | 0/0 |

Retained binary and ternary scratch coexist and are counted together. Fewer allocation calls do not establish lower latency. No resource-admission change is implemented or qualified, and no memory-pressure, phone, TEE or giant-circuit claim follows.

## Profiles and disposition

All timing processes finish before four whole-process cycles:u profiles at 199 Hz, covering Tile384 and Tile2048 with both variants. They contain 3,157 samples, one PID per recording and zero lost records. Profiles include setup, primes and checks. At 2,048 points, binary reduce_layer receives 72.1% of sampled period, while the candidate ternary layer receives 77.6% in its separate recording; scalar inverse plus inverse_nonzero receive 11.1%/8.9%. This is consistent with more kernel work outweighing the reduced inversion work, but does not isolate arithmetic, packing or cache effects or establish removable wall-time fractions.

The earlier two-update caller and this complete reduction have different batching and inversion structure. Its small-case gains do not transfer to real-size reductions. This eight-word-record ternary mechanism is closed without SDK integration or adoption. No unchanged rerun, threshold sweep or selective promotion is justified by this result.

The private snapshot has 175 files: 172 of 174 originals are byte-identical, and removing the two module/reexport suffixes recovers the remaining original files exactly. The new module is isolated behind a diagnostic cfg; production is unchanged. The frozen library set and source/input/helper pins match before and after capture. No owned build, analysis, hash, archive or second benchmark runs during capture. Complete source, command logs, decoded profiles and checksums are archived; native executables, target directories, raw perf files and perf hexadecimal dumps remain local with hashes.

This component adds no SDK request or proof export. The independent-proof ledger remains 15,553. No new AWS, Aeglos, ARM, mobile, large-circuit, pressure or concurrent-proof result is claimed.

## Inherited CPU and Metal comparison

The [accepted source174 report](../2026-10-28-g2-borrowed-adoption/README.md), including its failed performance gate and explicit engineering exception, remains authoritative. These are inherited timings; the M5 values are supplied historical references.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | Required ms for CPU10× |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.366931 | 37 | 4.422× | 24 | 2.868× | 3.7 |
| 2x2 | 11.539335 | 59 | 5.113× | 30 | 2.600× | 5.9 |
| 4x2 | 16.540233 | 109 | 6.590× | 48 | 2.902× | 10.9 |
| 8x2 | 25.136722 | 189 | 7.519× | 74 | 2.944× | 18.9 |
| 12x2 | 34.858408 | 284 | 8.147× | 101 | 2.897× | 28.4 |
| 16x2 | 41.377043 | 335 | 8.096× | 124 | 2.997× | 33.5 |
| 24x2 | 67.764001 | 528 | 7.792× | 176 | 2.597× | 52.8 |
| Merge24x1 | 67.096516 | 507 | 7.556× | 175 | 2.608× | 50.7 |

Accepted Merge54×1 remains 164.388184 ms without a supplied M5 reference. Primary accepted RSS median/max is 131.25/135.00 MiB; Merge54 median/max is 1,802.20/1,885.21 MiB. These are inherited production observations, distinct from the component capacity table. The original all-eight CPU 10× goal remains **active and unmet, 0/8**.

[Protocol](artifacts/design.md), [implementation](artifacts/reduction_probe.rs), [analysis and raw ratios](artifacts/analysis.json), [native tests](artifacts/native/core/stdout.log), [independent oracle](artifacts/native/oracle/stdout.log), [source audit](artifacts/source-audit.json), [capture pins](artifacts/pins.json), [workflow completion](artifacts/runs/result.json), [artifact index](artifact-index.json).

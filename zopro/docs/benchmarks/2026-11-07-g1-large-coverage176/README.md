# G1 coverage in the larger-query profile

The saved 4×2 profile redirects the next source audit toward streaming additions and bucket merging. Its largest G1 symbol is `BatchAdd::apply`, whose direct Fq branch dispatches to `stream_g1_impl`; the earlier tree-prefix candidate changed the separate `reduce_layer` path. The earlier large-group microbenchmark gain is not evidence of a comparable full-proof gain.

This is a read-only aggregation of existing evidence. No SDK request, proof, native kernel test, timing capture or decoder is rerun. Production176/SDK6d83 and the 32,628 independently verified-export ledger remain unchanged. No optimization is adopted.

## Saved profile and source paths

The [larger canonical-wire experiment](../2026-11-07-large-canonical-sdk176/README.md) qualified the accepted 4×2/32-worker profile with 30,160 samples, zero LOST, executable ownership, CPU/PID/TID/MMAP and fixed-period checks. This audit reuses its retained symbol summary rather than decoding the recording again.

| Symbol family | Flat samples |
|---|---:|
| G1 `BatchAdd::apply` | 3,417 |
| Separately emitted G1 streaming kernel | 1,787 |
| G1 unsigned-MSM iterator | 1,884 |
| G1 tree `reduce_layer` | 1,423 |

These are separate flat bins, not exclusive caller costs, critical-path fractions or removable wall time. Inlining can put inner work under an outer symbol. The iterator bin does not establish an allocation bottleneck. Unlisted bins remain in the original profile summary.

Source tracing distinguishes `BatchAdd::apply → Fq::stream_add → stream_g1 → stream_g1_impl` in direct mode from `apply_generic → reduce_layer` in fallback mode. Native canonical and noncanonical streaming variants remain distinct. This source trace is not a dynamic assertion that every `apply` sample took one particular branch or belonged to one query. The existing [tree coverage audit](../2026-11-06-g1-tree-production-coverage176/README.md) covered primary profiles; the new larger-query profile does not justify transferring its percentages to all shapes.

## Existing group populations

The [chunk-grouping diagnostic](../2026-11-05-chunk-grouping176/README.md) already retains anonymous A/B class-size histograms for all eight shapes. This new aggregation checks all 16 populations: class counts, nonzero membership plus excluded zeros, and logical pair-level conservation. Category zero denotes canonical **one**, not a zero scalar. No class values or point coordinates are exported.

For a worker tile containing at most 256 scalar classes, summing the 256 largest class sizes gives a conservative upper bound on its copied input-point span. Class order is unnecessary for this bound, but would be necessary for the actual tile spans. Bounds describe the previously observed fixture populations and admission, not arbitrary future witnesses.

| A query | Grouping admitted | Nonzero classes | Largest class | Maximum possible points in a 256-class tile | Logical binary pairs |
|---|:---:|---:|---:|---:|---:|
| 1×2 | no | 17,454 | 646 | 1,601 | 1,819 |
| 2×2 | yes | 19,268 | 1,231 | 2,445 | 14,302 |
| 4×2 | yes | 20,386 | 2,450 | 4,829 | 41,001 |
| 8×2 | yes | 25,236 | 4,897 | 9,310 | 92,630 |
| 12×2 | yes | 30,704 | 7,314 | 13,730 | 143,311 |
| 16×2 | yes | 34,326 | 9,867 | 18,318 | 195,910 |
| 24×2 | yes | 43,958 | 15,072 | 27,531 | 299,395 |
| Merge24 | yes | 41,272 | 14,270 | 22,601 | 286,701 |

The primary row is a population bound, not an executed grouped route: its grouping admission declined. Logical pairs count reductions needed to combine each nonzero class, ignoring point identities, exceptional formulas and actual SIMD packing. They are not executed field-operation or EC-addition counts.

For 4×2 A, 11,724 classes have four members. Classes of size 2–7 account for 36,443 of 41,001 logical pairs; the single 2,450-member class contributes 2,449. The first two logical layers contain 26,587 and 13,198 pairs. The earlier [tree caller study](../2026-11-06-g1-tree-callers176/README.md) repeatedly reduced a 35,973-point fixture on every worker. That is larger than even the 4,829-point upper bound for any 256-class A tile here. No retained A population can produce a 35,973-point tile under this bound. K-query classes, actual ordered worker spans, indexed/weighted active work and simultaneous scratch remain unmeasured by these histograms. The audit therefore makes no such claim for every G1 reduction.

## Decision and validation

Keep the old tree implementation's component gains and failed gate intact. Do not promote a favorable subset or rerun its unchanged caller benchmark. Another implementation needs a concrete useful-work or lifetime change in an actually covered path.

One such source location to inspect is `merge_buckets`: it copies each ordinary right-hand point into a temporary `(index, point)` vector before invoking streaming G1 addition. Existing streaming code already accepts borrowed affine inputs. Whether a G1-specific borrowed merge removes meaningful work requires a separate lifetime/resource/history audit, independent correctness checks and complete-caller performance evidence. This report does not implement or select that candidate, assume all `BatchAdd` samples are merging, or promise a twofold gain.

The single Rust/xtask audit command passes on its first execution. All 176 production hashes and all 11 declared input pins match. Conservation checks cover all 16 historical populations and the selected saved-profile bins; source dispatch assertions pass. No performance, minimum-RAM, pressure/swap, mobile, large-circuit, concurrent-throughput, AWS or Aeglos result is added.

## Unchanged accepted comparisons

Inherited [adoption](../2026-10-30-g2-vector-sign-adoption176/README.md) results and supplied historical M5 references follow; they are not measured by this audit.

| Shape | Local ms | M5 CPU ms | M5 Metal ms | CPU 10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| Merge24 | 65.060937 | 507 | 175 | 50.7 |

Primary's latest separate same-binary observation remains 7.991694 ms. Merge54 adoption is 161.883899 ms without a supplied M5 reference. The full all-eight CPU 10× goal remains active and unmet, **0/8**.

[Evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/evidence-members.json), [checksum](artifacts/SHA256SUMS). Includes the new aggregation, declarations, receipts, anonymous inherited histograms, retained profile summary, source definitions and published parent reports. Raw profiles, binaries/libraries, private inputs/keys/witnesses/proof payloads and continuation notes are excluded.

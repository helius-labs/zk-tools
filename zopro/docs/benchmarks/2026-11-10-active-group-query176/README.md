# Unfinished scalar-class scan comparison

**The fixed candidate is rejected.** Grouped 2×2 G2 at 32 workers has paired ON/OFF **1.017932 [1.010328, 1.033712]**, 1/15 wins, and ON/REF **1.024289 [1.008300, 1.031278]**, 0/15 wins. The grouped G1/G2 32-worker aggregate is ON/OFF **1.007957 [1.004333, 1.027606]**, 3/15 wins. One control-calibration check also fails, so control agreement is not globally qualified. No code is adopted and no full-SDK follow-up is selected.

The candidate keeps only unfinished scalar classes in a stable u16 list during subsequent pair-discovery and compaction passes. It preserves original point storage, initial copies, arithmetic expressions, pair order, grouping policy, and task geometry. One fixed 512-byte stack array per nonforking leaf is charged in the existing checked workspace forecast. It adds no heap buffer or key table. Disabled and generic release paths retain the original loop. REF is accepted source with a minimal benchmark API; OFF and ON share qualified candidate source and the same API, with only ON selecting the active scan. All three consumers use the same original field1200, native/O3/fat-LTO/one-CGU settings, allocator, driver, and Rayon dependency.

Native and generic qualification each pass five new test functions: exact old/new raw-coordinate layer trajectories and independent Arkworks class sums for G1/G2; ordinary/prepared getters; partial 256-class chunks; identities, equal/opposite points, odd tails, hot and dense classes; actual 1/4/16/32-worker pools and four concurrent callers sharing a pool with 512 KiB stacks; checked stack forecasts and overflow. The selected layer recovers the original arithmetic byte-for-byte after reversing its signature and two loop headers. Production's 176 source hashes remain unchanged.

On retained ordered 2×2-B scalar classes, the data-only census reduces source-level descriptor visits from **109,187 to 85,594 (21.608%)**, counting initial-list construction and filtering. Point-addition counts do not change. Dense size-two classes instead increase descriptor visits from 769 to 1,024. These counts do not price array initialization, indexing, branches, memory traffic, or cycles. Primary 1×2-B main scalar grouping declines; other indexed final reductions can still call the changed helper. A declined main grouping does not prove an instruction-identical complete caller.

Actual retained G1/G2 point/scalar fixtures and independent Arkworks full-MSM references are hash-pinned and reused. Each iteration runs one complete query using the pool internally. The timer includes B's scalar reset, grouping/admission, all windowing and arithmetic, merges, weighted reduction, Horner, allocation/drop, and final affine normalization. Loading, pool construction, three checked primes, a final postcheck, and oracle comparisons are outside the timer. Every timed output matches its independent full-MSM reference and input points remain unchanged. These are complete-query results, not full-proof timings.

|Context|REF ms|OFF ms|ON ms|ON/OFF [bounds]; wins|ON/REF [bounds]; wins|
|---|---:|---:|---:|---|---|
|1x2-A-w32|1.156480|1.169225|1.174755|0.993117 [0.983820, 1.015144]; 10/15|1.006244 [0.994048, 1.023365]; 6/15|
|1x2-B-w1|36.653901|36.754680|37.054085|1.007654 [1.005926, 1.009351]; 0/15|1.010652 [1.008069, 1.012677]; 0/15|
|1x2-B-w16|3.626048|3.622992|3.686061|1.015476 [1.003898, 1.032466]; 3/15|1.012689 [1.006015, 1.027417]; 2/15|
|1x2-B-w32|3.224411|3.221184|3.267291|1.015199 [1.002887, 1.029963]; 2/15|1.008559 [0.988365, 1.030930]; 4/15|
|2x2-A-w32|1.500921|1.495009|1.504127|1.007713 [0.989772, 1.021573]; 7/15|0.998142 [0.966784, 1.013342]; 8/15|
|2x2-B-w1|39.300541|39.387565|39.677833|1.007515 [1.006241, 1.009108]; 0/15|1.009098 [1.007562, 1.011901]; 0/15|
|2x2-B-w16|4.185743|4.179952|4.263569|1.014864 [1.007118, 1.023970]; 0/15|1.018142 [1.010119, 1.032745]; 2/15|
|2x2-B-w32|3.768666|3.801789|3.870890|1.017932 [1.010328, 1.033712]; 1/15|1.024289 [1.008300, 1.031278]; 0/15|

Ratios below 1 are faster. Each of 15 fixed matched blocks uses 17 queries per process, rotating/reversing context and variant order. Bounds are sorted ranks 4..12 of 15 paired process-median ratios: nominal 96.484375% marginal median coverage under independent exchangeable blocks, without a simultaneous guarantee. No samples, favorable medians, or adverse contexts are discarded. The 1×2-A ON/OFF median is 0.993117 (0.688% lower), and 2×2-A ON/REF is 0.998142 (0.186% lower); both intervals include 1 and the complete selection criteria fail.

Of 54 prospective checks, 16 fail: one of eight control calibrations, ten of sixteen nonregressions, all four gain checks, and one of eight RSS checks. The failed calibration is 2×2-A32: OFF/REF lower 0.960062 is below 1/1.03. Grouped 2×2-B32 passes calibration but is slower against both controls, so that rejection does not depend on the failed G1 calibration. All eight PMU and eight profile quality checks pass. The fixed recipe is closed without worker, threshold, curve, layout, profiling-length, or unchanged-run rescue.

|Context|REF / OFF / ON maximum process RSS MiB|
|---|---:|
|1x2-A-w32|23.75 / 23.75 / 22.00|
|1x2-B-w1|15.50 / 15.50 / 15.75|
|1x2-B-w16|27.50 / 27.50 / 27.50|
|1x2-B-w32|32.00 / 35.25 / 32.25|
|2x2-A-w32|29.00 / 28.00 / 29.25|
|2x2-B-w1|27.25 / 27.25 / 27.50|
|2x2-B-w16|43.50 / 42.25 / 44.00|
|2x2-B-w32|54.00 / 53.75 / 61.25|

The 2×2-B32 RSS screen allows max(REF, OFF)×1.05+1 MiB = 57.70 MiB; ON reaches 61.25 MiB. This lifetime process peak includes fixture loading, runtime, primes, and allocator history; it is neither incremental proof scratch nor minimum RAM. The observation does not establish that the 512-byte stack array caused the difference.

All **440 serialized workflow commands** complete successfully: 24 preflights, 360 latency processes, eight PMU panels, eight profiles, and 40 offline commands. There are 400 caller processes and 14,144 checked complete queries including primes/postchecks. All latency precedes diagnostics and all recording precedes offline work. During capture only confirmed live session 58838 was polled until terminal exit 0; no owned build/helper/hash/analyzer/report/push overlaps recording.

PMU groups have at least 99.9% running. All eight profiles qualify: **50,167 samples**, minimum **2978**, maximum unknown share **1.242%**, zero lost records, fixed 16M-cycle period, and PID/TID/CPU/MMAP/event bindings. Candidate namespaces appear in both grouped G1/G2 ON profiles. Flat sample shares include all arithmetic in those functions and are not removable wall-clock fractions.

|Single-panel context|ON/OFF instructions|ON/OFF cycles|ON/OFF cache misses|ON/OFF branch misses|
|---|---:|---:|---:|---:|
|1x2-B-w16|1.000030|1.006274|0.993233|0.992820|
|1x2-B-w32|1.000208|0.999045|1.002748|1.013867|
|2x2-B-w32|0.999141|0.999807|0.916408|1.034285|
|2x2-A-w32|0.997265|0.998369|1.003337|1.002260|

The source-level descriptor reduction corresponds to only about 0.086% fewer whole-query instructions in the grouped G2 panel and 0.274% in grouped G1. These single panels do not identify a unique cause of the latency result, memory behavior, or a cache-level bottleneck. CPU service, minor faults, switches, and RSS are retained separately in the review; the unchanged driver does not expose major faults. This result reinforces using complete-caller work as the denominator when selecting further ideas.

A separate same-agent reader verifies 975 capture pins, all 440 command/stream/managed-environment receipts, 400 caller outputs, 54 independently recomputed decisions, and 50,167 profile samples. Source qualification checks all 176 unchanged production files, private-source recovery, and identical arithmetic beneath the changed loop headers. Two qualification build failures (checked-expression propagation and Debug-only assertion syntax) and one helper compile failure (missing rustc_demangle external dependency) are preserved. Repairs reuse successful stages, change no arithmetic or thresholds, and precede all timing; no benchmark repeats.

Accepted production and the canonical **43,730** verified-proof inventory are unchanged. The full CPU AVX 10× objective remains active and unmet: **0/8** targets. The following accepted local 32-worker full-proof results and supplied M5 CPU/Metal references are inherited from the previous full-SDK campaign, not measured in this query experiment.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37.0 / 4.726×|24.0 / 3.065×|3.7|
|2x2|10.979335|59.0 / 5.374×|30.0 / 2.732×|5.9|
|4x2|15.803969|109.0 / 6.897×|48.0 / 3.037×|10.9|
|8x2|23.944017|189.0 / 7.893×|74.0 / 3.091×|18.9|
|12x2|33.703579|284.0 / 8.426×|101.0 / 2.997×|28.4|
|16x2|39.858478|335.0 / 8.405×|124.0 / 3.111×|33.5|
|24x2|66.186709|528.0 / 7.977×|176.0 / 2.659×|52.8|
|merge-24x1|65.489902|507.0 / 7.742×|175.0 / 2.672×|50.7|

Accepted 1×2 remains **7.829286 ms**, **133.50 MiB** peak RSS at 32 workers; 16-worker latency is 9.192237 ms. Earlier 7.803404 ms used the same accepted executable in another campaign. Merge54 remains 162.541995 ms without a supplied M5 reference. No new AWS/Aeglos, ARM/mobile, large-circuit, memory-pressure/swap, minimum-RAM, or concurrent-proof throughput result is added.

[Full analysis](artifacts/analysis.json), [separate review](artifacts/review.json), [CPU diagnostics](artifacts/diagnostic-summary.json), [source audit](artifacts/source-audit.json), [identities](artifacts/identities.json), [qualification](artifacts/qualification-qualification-review.json), [descriptor census](artifacts/qualification-census.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.json), [continuation and research loop](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private roots: /home/ubuntu/src/zopro-perf-work/active-group-scan176 and /home/ubuntu/src/zopro-perf-work/active-group-query176.

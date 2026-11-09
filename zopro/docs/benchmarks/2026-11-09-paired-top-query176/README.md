# Paired G2 leading components with bounded tile reduction

**The composition fails its declared query gate and is not adopted.** At 32 workers, pairing the high/low leading components is 2.49% slower than bounded batching alone. The combined path establishes no primary gain against the original query. Lower CPU work and process memory remain positive observations; they do not establish a full-proof improvement.

## Implementation and controls

The [native-qualified composition](../2026-11-09-paired-top-bundles176/README.md) publishes the original high/low leading-component computations together using a Rayon join in each original partition. It retains Phi2/Booth, width 12, two partitions, scalar grouping, original component and ordinary-window helpers, original field arithmetic, each bounded tile pair tree, endomorphism and merge/final-fold order. Each leaf remains nonforking with at most 1,024 point copies. A checked additional parent reservation is acquired before scalar mutation; denial retains the sequential fallback. No retained table, key/solver plan, scalar matrix or new runtime pool is introduced.

The inherited extra-parent forecast is 263,168 bytes at 30,089 rows, not an allocation/RSS/minimum-RAM measurement. Bounded tile metadata retains its own reservation. This measurement source adds only five process-local atomic counters for admission, completed paired parts, successful bundled calls and maximum charges. Reset/snapshot runs outside timing; route counter updates are included. Quiescent ownership is required by this private benchmark observer; it is not an overlapping-proof API.

REF links the inherited original-field/original-query core. ORIGINAL, BUNDLE, PAIR and BOTH share one fresh executable with explicit paired/bundled options 0/0, 0/1, 1/0 and 1/1. All arms use the original field, with optimized native CPU, fat LTO and one codegen unit at actual final links. There is no short-negative-beta transform. The earlier [standalone bounded reducer](../2026-11-08-indexed-tile-bundles176/README.md) is remeasured as a factorial control; its prior failed gate remains closed.

## Complete-query results

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Saved real B fixtures contain 30,089 and 52,274 points and retain independent complete Arkworks MSM oracles. Each fresh process uses one existing global Rayon pool with default stacks. Workers pin to CPU 2 solo or their index in CPUs 0–15/0–31. This placement is not an adopted production default.

The clock includes scalar reset, grouping/admission, in-place decomposition, all window work, new scheduling/counters, endomorphism, weighting, normalization, allocation/drop and joins. Fixture load, pool startup, three checked primes, output-hash checking and one postcheck are excluded. Every timed output matches the saved oracle; points remain unchanged, N+1 task counts and worker CPUs agree. Actual timed route counts are exactly one paired admission/two completed parts when paired and four successful bundled calls when bundled, zero otherwise. A denied fallback therefore cannot masquerade as a candidate observation.

Thirty preflights precede 530 latency processes. Primary 1×2/32 uses 31 rotated/reversed matched blocks of 151 queries; the other five contexts use 15 blocks of 51 queries, except solo uses 11. Cells below are medians of process means, in milliseconds. Paired ratios need not equal ratios of those marginal medians.

|Context|REF|ORIGINAL|BUNDLE|PAIR|BOTH|BOTH/ORIGINAL [95% bounds]|Wins|
|---|---:|---:|---:|---:|---:|---|---:|
|1×2 / 1|37.599417|37.550586|33.201713|37.599663|33.474105|0.891924 [0.890096, 0.893790]|15/15|
|1×2 / 16|3.571052|3.581346|3.531373|3.646893|3.602114|1.003332 [0.982407, 1.024860]|8/15|
|1×2 / 32|3.218314|3.174079|3.152516|3.245336|3.198281|1.006340 [0.993810, 1.019260]|9/31|
|2×2 / 1|40.319496|40.104799|36.263477|40.298820|36.214585|0.904539 [0.901749, 0.907192]|15/15|
|2×2 / 16|4.124651|4.132591|4.076084|4.154826|4.129083|0.996377 [0.971440, 1.022757]|8/15|
|2×2 / 32|3.663044|3.680290|3.653638|3.731750|3.758078|1.030702 [1.008185, 1.052846]|2/15|

|Primary 32-worker contrast|Paired ratio [95% bounds]|Wins/31|
|---|---|---:|
|BOTH/REF|0.995924 [0.983186, 1.009414]|15|
|BOTH/ORIGINAL|1.006340 [0.993810, 1.019260]|9|
|BOTH/BUNDLE|1.024879 [1.015017, 1.035271]|2|
|BOTH/PAIR|0.991652 [0.980971, 1.002644]|23|
|PAIR/ORIGINAL|1.014812 [1.001927, 1.028383]|9|
|BUNDLE/ORIGINAL|0.981912 [0.971013, 0.993222]|22|
|BUNDLE/REF|0.971748 [0.960628, 0.983410]|24|

Pairing alone regresses against ORIGINAL by a paired 1.48%; combining it with batching regresses 2.49% against BUNDLE. BOTH/ORIGINAL primary is 1.006340 [0.993810, 1.019260], with 9/31 wins. The 2×2/32 comparison is 1.030702 [1.008185, 1.052846], with 2/15 wins. This schedule does not convert the earlier CPU-work reduction into lower threaded latency.

Positive results are retained: BOTH improves solo 1×2/2×2 against ORIGINAL by 10.81%/9.55%, each with 15/15 wins. The BUNDLE control improves primary32 by 1.81% against ORIGINAL and 2.83% against REF in this run. These descriptive control observations do not select a post-result subset or waive the earlier standalone failure.

The equal-log two-shape32 aggregate BOTH/ORIGINAL is 1.018448 [1.005335, 1.031143]; BOTH/REF is 1.005570 [0.991414, 1.020030]. Both gain checks fail. All 60 pairwise contrasts and four aggregate comparisons remain in [analysis](artifacts/analysis.json.gz).

## Declared gate and memory

The [prospective protocol](artifacts/capture-protocol.md) uses 20,000 paired-log bootstrap resamples with a fixed seed and independent within-shape resampling for the 31/15-block aggregate. Bounds assume independent exchangeable blocks; there is no familywise guarantee. BOTH alone is the selected candidate. All six ORIGINAL/REF intervals must fit factor 1.03; all BOTH/control nonregression uppers must be ≤1.02 and peak lifetime RSS ratios ≤1.05. Primary gain requires upper<1 and at least 21/31 wins against each of four controls. The two-shape aggregate must improve against ORIGINAL and REF. No minimum positive-gain floor is imposed.

The 65-check gate fails 23 checks: twelve nonregression, four primary gain, three primary wins, two secondary calibration and two aggregate gain checks. All 24 RSS checks and the complete diagnostic qualification pass. Both failed calibrations are 2×2 at 16/32 workers; their point estimates are near parity but intervals exceed the declared range. Keep those failures and every observation. No block exclusion, sample extension, parameter/layout/worker sweep, threshold change, new exception or favorable subset follows.

|Context|Maximum REF / ORIGINAL / BUNDLE / PAIR / BOTH RSS MiB|
|---|---|
|1×2 / 1|13.00 / 13.00 / 13.00 / 13.00 / 13.00|
|1×2 / 16|23.75 / 23.50 / 23.75 / 22.75 / 23.50|
|1×2 / 32|27.00 / 27.50 / 26.50 / 24.75 / 23.50|
|2×2 / 1|23.25 / 23.25 / 23.25 / 23.25 / 23.25|
|2×2 / 16|33.00 / 33.25 / 33.75 / 33.00 / 34.00|
|2×2 / 32|35.00 / 35.50 / 36.00 / 36.00 / 34.75|

RSS is the maximum lifetime rusage among latency processes and includes loading, primes and allocator retention. It is not complete prover peak or minimum RAM. Primary32 ORIGINAL/BOTH CPU service is 70.508/64.091 ms per query; minor faults are 8.795/4.066 per query. Lower faults, RSS and aggregate CPU work do not establish lower latency or a memory-pressure cause.

## Automatic CPU profiles

All latency finishes before 15 FIFO-gated PMU panels and 15 separate fixed-period cycle profiles. All recording finishes before offline decoding/review. Only the exact live workflow handle is polled during capture; no owned helper/build/hash/analyzer/verifier/archive/push overlaps recording. All 60 PMU events have 100% scheduled coverage. All 15 profiles qualify: 133,060 samples total, zero lost events, unknown share at most 0.981%, exact 16M period and PID/TID/CPU/MMAP/FORK binding. Absolute period coverage is not validated.

|Primary32 panel|Instructions/query M|Cycles/query M|Generic cache misses/query M|
|---|---:|---:|---:|
|REF|567.716|321.783|1.144755|
|ORIGINAL|566.745|315.410|1.128487|
|BUNDLE|520.607|290.517|1.175002|
|PAIR|567.821|316.090|1.142773|
|BOTH|518.777|288.232|1.165195|

BOTH retires 8.46% fewer instructions and 8.62% fewer user cycles than ORIGINAL in these separate single diagnostic panels. At32, ordinary borrowed point-apply samples remain 4,085→4,025, while reduce-layer samples fall 1,352→971 and inverse samples 696→428. Flat shares are point apply 41.4%→45.1%, reduction 13.7%→10.9%, inverse 7.1%→4.8% and epoch 10.1%→11.1%. These are not exclusive wall-time fractions, causal critical-path attribution or proof of a cache/DRAM bottleneck. The new data contradict a latency benefit from this particular top-component concurrency change.

## Verification and decision

Native arithmetic qualification is inherited: 573 checked queries and four tests, with unchanged field/models not rebuilt or retested. The only new test checks counter reset/reuse, parallel counts and maxima on 1/4/16/32-worker small-stack pools; it passes with 131 inherited tests filtered. An initial test compilation captured a non-Send Rayon broadcast context across join closures; copying its index to usize fixes only the test. Its failed source/pins/plan/compiler receipt remain. Runtime observer, scheduler and arithmetic are unchanged by that repair. [Failure details](artifacts/tool-failure.md).

Four corrected native commands, two actual consumer links, three tool builds and 665 capture/offline commands pass once: 674 successful workflow commands plus the preserved initial compiler failure. There are 590 callers and 46,900 checked complete queries including primes/postchecks. The new 180-member snapshot recovers the qualified native 179 members and all 176 production originals; the bounded child is exact. Generic noASM/noIFMA guard compilation is not ARM execution. The private observer is not a production concurrency interface.

Prospective and result reviews pass: 587 pins, all 665 receipts/streams, all 60 pairwise contrasts, all 65 decisions and 133,060 samples. The result reader independently extracts raw means/peaks and rebuilds aggregates/gates; bootstrap and PMU/profile parsers are shared and disclosed. It reads saved decodes without replaying qualification, capture, oracles or decoding. [Result review](artifacts/review-result.json), [source audit](artifacts/source-audit.json), [release observer](artifacts/observer.rs), [driver](artifacts/driver.rs).

**Close this fixed composition without production adoption or an SDK trial.** The next search must address substantial ordinary G2/KH work, after reviewing earlier negative arithmetic/queue/schedule experiments. No replacement is selected here. Full verified ALL8+Merge54 SDK, independent Gnark, resource/admission, portable and concurrency qualification remain necessary before future adoption. [Decision](artifacts/decision.json), [continuation loop](artifacts/next.md).

## Accepted CPU and Metal comparisons, inherited

These full-proof observations use fixed worker placement, not an adopted default or this candidate. Historical standard 1×2 **7.915095 ms** remains valid; later standard **8.075143 ms** and fixed **7.869302 ms** retain their own conditions. No accepted code regression or new proof record is established.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 /4.702×|24 /3.050×|3.7|
|2×2|10.984868|59 /5.371×|30 /2.731×|5.9|
|4×2|15.734200|109 /6.928×|48 /3.051×|10.9|
|8×2|23.929023|189 /7.898×|74 /3.092×|18.9|
|12×2|33.722646|284 /8.422×|101 /2.995×|28.4|
|16×2|39.776845|335 /8.422×|124 /3.117×|33.5|
|24×2|66.183856|528 /7.978×|176 /2.659×|52.8|
|Merge24×1|65.448733|507 /7.747×|175 /2.674×|50.7|

Source: [preceding accepted comparison](../2026-11-09-paired-top-bundles176/README.md). Supplied M5 references use different hardware. Merge54 remains 162.489760 ms without an M5 reference. Production176/SDK6d83 and the 38,668-proof ledger remain unchanged. There are zero new SDK requests, Gnark exports, AWS resources or Aeglos/device/huge-circuit/pressure/swap/minimum-RAM/throughput measurements. The all-eight verified CPU10× goal remains active and unmet, **0/8**.

Private root: `/home/ubuntu/src/zopro-perf-work/paired-top-query176`. [Capture plan](artifacts/capture-plan.json), [prospective review](artifacts/prospective-review.json), [lossless source/helpers/receipts/decoded evidence](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). Executable/library blobs, raw perf.data, unchanged full source, fixture/key/witness/proof bytes, private proof ledger and FIFO contents are excluded. No capture is live.

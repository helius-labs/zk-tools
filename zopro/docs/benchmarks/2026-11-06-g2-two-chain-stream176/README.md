# Two streaming G2 prefix chains

**Not adopted.** The two-chain ordinary G2 caller fails its declared screen. At one and sixteen workers, both batch sizes lose all 15 paired blocks against fresh OFF. The two-size 32-worker aggregate is **0.71% slower versus OFF**, ratio 1.007070 [1.002989,1.017151], with 2/15 wins. No complete-query or full-proof follow-up is selected.

All six G2 control calibrations pass. The unchanged G1-w32 control calibration fails and remains in the result. That uncertainty does not rescue the consistently negative G2 observations. Accepted source176/SDK6d83 and proof performance remain unchanged.

## Fixed implementation

The original borrowed-G2 kernel accumulates a sequential product of denominator norms in each SIMD lane. This candidate interleaves exactly two prefix chains in the existing forward pass, combines their roots for the original inversion/tail path, and selects the matching inverse in the existing reverse point-finishing pass. Prefix records remain two vectors per group. It adds three root-combination vector multiplications per batch.

For roots R0/R1 and full-product inverse I, reverse chains start at I·R1 and I·R0. A group's saved same-parity prefix times its reverse inverse supplies the original inverse norm. Norms, signs, point finishing and scalar-tail statements recover byte-exact after reversing six documented edits. No field representation or arithmetic, production dispatch, Window layout, grouping, queue, point array, key, execution plan or runtime pool changes. Extra local vector state is real; equal heap records do not imply equal compiler stack.

This differs from the [four-chain phased trial](../2026-10-28-g2-phased-inverse174/README.md), which added passes that materialized inverse norms and nine root products. The [balanced heap](../2026-10-31-g2-denominator-tree176/README.md) and [retained-denominator record](../2026-11-05-g2-denominator-record176/README.md) remain separately rejected. Striping itself is not new, and no earlier failed gate is amended.

The original reference is an exact pre-existing source176 kernel caller, SHA256 `f07229ce…`; fresh OFF/ON share executable `19a37c35…`. Reference audit checks 385 parent pins, matching driver/G1 bodies and inverse G2 fixture edits, original source recovery and optimized native/fat-LTO/one-CGU consumer commands. All new registry entries match the old versions/checksums; the old lock has additional entries. FROZEN is not the accepted full SDK.

## Complete kernel results

Ryzen 9950X, 1/16/32 workers pinned to CPU2/CPUs0–15/0–31, with 512 KiB stacks. Fifteen fixed rotated/reversed process blocks, three checked 128-batch primes, reset and 4,096 timed batches per worker. Values are median whole-wave means in microseconds, not proof times. The timer includes ref assembly, dispatch, all point arithmetic and waiting. Points change every batch; final generator checks, construction and reset are outside timing. Timed allocations are zero and primed capacities remain stable.

|Context|FROZEN µs|OFF µs|ON µs|Paired ON/OFF [bounds]|Wins/15|
|---|---:|---:|---:|---|---:|
|G2-200-w1|7.667110|7.675478|7.821211|1.018894 [1.018041,1.020449]|0|
|G2-200-w16|8.935514|8.944897|9.068095|1.014768 [1.012989,1.017699]|0|
|G2-200-w32|14.384836|14.377063|14.866667|1.016712 [1.006859,1.024885]|3|
|G2-352-w1|12.990305|12.994116|13.184697|1.014713 [1.013867,1.015494]|0|
|G2-352-w16|15.168186|15.188147|15.395854|1.013935 [1.012207,1.016012]|0|
|G2-352-w32|24.858739|24.968296|24.900520|1.003102 [0.994670,1.012656]|6|
|G1-200-w1, unchanged|4.210634|4.209957|4.212875|0.999808 [0.998464,1.002074]|8|
|G1-200-w16, unchanged|4.781953|4.754690|4.758342|0.998603 [0.994568,1.005137]|9|
|G1-200-w32, unchanged|6.867822|7.006269|6.959132|0.996777 [0.961261,1.018333]|9|

|G2 two-size32 aggregate|Median|Lower|Upper|Wins/15|
|---|---:|---:|---:|---:|
|ON/OFF|1.007070|1.002989|1.017151|2|
|ON/FROZEN|1.009484|0.995231|1.027446|6|

The 352/32 marginal ON median is slightly lower than OFF, while the paired median ratio is above parity. Both are retained; pooled medians need not divide into the paired estimate. Its interval establishes no gain.

The [prospective protocol](artifacts/protocol.md) requires all control intervals within factor 1.03, all ON/control upper bounds ≤1.03, and both 32-worker G2 sizes plus their aggregate to improve with upper <1 and ≥10/15 wins against both controls. All four individual G2 gain checks and both aggregate gain checks fail. G2-200-w32 ON/FROZEN non-regression fails. G1-w32 OFF/FROZEN calibration and ON/FROZEN non-regression also fail, with wide intervals retained. [Analysis](artifacts/analysis.json) includes all nine failed checks and every comparison.

Bounds use ranks 4–12 of 15, nominal 96.484375% marginal under independent/exchangeable blocks, without familywise coverage. There is no gain floor, exclusion, extension, threshold revision, chain-count sweep, favorable-subset promotion or unchanged reroll.

## CPU work, memory and linked code

All 405 latency processes finish before six PMU panels and six separate 16Mcycles:u profiles. Priming/checks are excluded by FIFO acknowledgement, with the frozen runtime environment explicitly forwarded under sudo. Every PMU event has 100% scheduled time. Panels are descriptive single observations, not uncertainty estimates.

|32-worker panel|Instructions/wave OFF→ON|Cycles/wave OFF→ON|
|---|---:|---:|
|G2-200|3,557,253→3,581,049|1,899,945→1,904,097|
|G2-352|6,033,138→6,061,163|3,249,780→3,266,119|

Instructions rise about 0.67%/0.46%, and cycles about 0.22%/0.50%. Shorter mathematical prefix dependencies did not make this implementation faster. Generic cache observations do not identify a cache level, bandwidth limit or causal stall.

|G2 linked function|Bytes|Instruction sites|IFMA sites|RSP operand lines|Literal stack subtraction sum|
|---|---:|---:|---:|---:|---:|
|FROZEN /OFF|51,830|8,590|1,450|1,450|5,952|
|ON|55,311|9,165|1,750|1,445|6,400|

All six G1/G2 disassemblies are hash-bound. These static observations are not dynamic spills, complete frame bounds or a causal latency decomposition. The original function remains separate in the same new executable. G1's selected linked function stays 26,198 bytes in all arms.

Prefix/tail payloads per worker remain [16,000,0] bytes at 200 and [28,160,0] at 352, with no new heap vector. For 32 workers, maximum latency-process RSS counters are 7.75/7.75/7.75 MiB (FROZEN/OFF/ON) at 200 and 15/15/16 MiB at 352. Other arrays, compiler frames and allocator state are additional. RSS is diagnostic and may undercount; this establishes neither minimum RAM nor pressure/mobile feasibility.

All six profiles qualify with 63,145 samples, fixed period, expected executable/PID/TID/workerCPU/mmap/FORK evidence and zero lost records. Unknown shares are below 0.065%. Flat symbols include inlined work, and absolute period coverage is not validated. No owned compiler, helper, hash, archive, analyzer or other benchmark overlaps recording.

## Correctness and review

The unfiltered native core suite passes 128 tests, including three new scalar subset-product, changing-generator/sign/tail/reuse and small-stack 1/4/16/32-worker cases. Field/prover/image qualification is inherited unchanged, not rerun. The child retains the original ordinary nonzero-denominator/distinct-destination contract; this is not a newly qualified general exceptional-point API.

All 27 new preflights pass independent generator checks and cross-arm digest/capacity equality before latency. The 444 caller processes cover 45,708,864 kernel batches including primes, all with zero timed allocations and successful final generator checks. Intermediate timed states are not individually verified. These batches are not proofs. No SDK request, Gnark export or AWS resource is created; the distinct proof ledger remains 30,846.

All 3 native and 475 capture commands succeed on their first executions. Independent review verifies 423 current pin occurrences and 385 inherited reference pins, all actual argv/environment projections/exits/stream hashes, the 176-source recovery and six-edit kernel inverse, every raw timing/CPU/RSS/capacity/gate and profile symbol bin/ownership/CPU/period. Review passes on its first execution; no successful test or capture is repeated. The 177-file snapshot has 174 originals byte-exact and two append-only originals. Production stays unchanged.

## Accepted proof comparisons remain unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references; these are not timings of the two-chain caller. Different machines do not establish equal hardware efficiency.

|Shape|Accepted local ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10×limit ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 /4.667×|24 /3.027×|3.7|
|2x2|11.151932|59 /5.291×|30 /2.690×|5.9|
|4x2|16.086386|109 /6.776×|48 /2.984×|10.9|
|8x2|24.179453|189 /7.817×|74 /3.060×|18.9|
|12x2|33.884455|284 /8.381×|101 /2.981×|28.4|
|16x2|40.077381|335 /8.359×|124 /3.094×|33.5|
|24x2|65.850777|528 /8.018×|176 /2.673×|52.8|
|merge24x1|65.060937|507 /7.793×|175 /2.690×|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted primary RSS 132.75 MiB is inherited, not remeasured. The all-eight CPU10× goal remains active/unmet, 0/8. No new AWS/Aeglos, full-proof, huge-circuit, concurrent-throughput, device, swap, pressure or minimum-RAM comparison follows.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit.json), [protocol](artifacts/protocol.md), exact receipts/source and six raw profiles retain the experiment. Private runtime point/scalar/key/witness arrays and executables/libraries are excluded. [Checksums](SHA256SUMS).

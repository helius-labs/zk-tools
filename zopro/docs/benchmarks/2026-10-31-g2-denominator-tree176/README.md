# Balanced denominator-product tree in the G2 caller

The declared caller gate is **false**; the candidate is **not adopted**. The six G2 contexts and three unchanged G1 controls remain in the report. No SDK proof-time trial follows this result. Production source 176 and SDK `6d83…` are unchanged.

Two-size G2 aggregate at 32 workers, ON/frozen: **1.000921 [0.979944, 1.031399], 7/15**.
Two-size G2 aggregate at 32 workers, ON/OFF: **1.016548 [0.990688, 1.042530], 4/15**.

At one worker, G2 loses every paired block against fresh OFF: +0.75% at 200 points and +1.12% at 352. At 16 workers the changes are +0.86% and +1.87%, again zero wins. The 32-worker intervals do not establish improvement, and two control calibrations fail. Fewer multiplications and a shorter mathematical dependency path did not make this complete caller faster.

For m eight-lane norm groups, the original algorithm updates a sequential forward prefix and two reverse products per group. The candidate builds a balanced binary heap, inverts its root through the original scalar-tail/inverse code, then propagates inverse products downward. Each leaf ends with its inverse norm. There are m−1 internal nodes, stored in the existing first record slots; norm leaves occupy the second slots and are overwritten only after their products have been consumed. The complete-heap indexing works for non-power-of-two m. For m≥1, this uses 3(m−1) vector products instead of 3 m. At 25/44 groups, that is 72/129 instead of 75/132. The mathematical norm-product dependency depth is logarithmic, while indexing, copies, loop and register costs remain real work.

There is no additional vector, key table, transformed point copy, duplicate solver plan, pool, fixed RAM cap, unsafe code or field-arithmetic change. Point finishing, signs and scalar-tail bodies are byte-exact after reversing the documented factor substitution. The original ordinary, distinct-destination streaming contract remains; exceptional-point selection above the kernel is unchanged. This differs from earlier G1 two/four-chain and G2 four-chain phased trials, which added vector products. It is also separate from the compact point-sum tree, which changed a different reduction. No novelty claim is made.

The original caller was freshly built from accepted source and frozen before timing; it is not a new measurement of the production SDK. Fresh OFF/ON share one second executable. Both final links use optimized native/fat-LTO/one-codegen-unit flags. Two 177-file private snapshots add only a diagnostic child module and two reversible parent suffixes; 174 original files stay exact in each, and all 176 originals recover exactly. The field sources are unchanged and inherit their qualification.

All 128 native core tests pass, including three new heap/point/threaded tests. Heap sizes 0..65 cover empty, singleton and irregular trees with direct independent scalar root/product and leaf-inverse checks. Complete changing G2 generator oracles cover signed inputs, tails and reuse at 22 lengths 0..512. Small-stack pools use 1/4/16/32 workers. The tests exercise the selected native canonical/vector-inverse path; no new portable, pressure or phone qualification is claimed.

Ryzen 9950X, 16 physical cores/32 logical CPUs, shared host. Solo uses CPU 2; 16/32 workers are individually pinned to 0–15/0–31 with 512 KiB stacks. Fixtures may be stolen between workers; this is not a fixture-to-core binding claim. Each process performs three checked 128-batch primes, reset, then 4096 timed batches per fixture. Bucket values change every batch. Whole timed scope includes caller/ref assembly, pool dispatch, all point arithmetic and waiting; construction, reset and independent generator checks are outside timing. Every final bucket after each prime and timed loop matches the reference; intermediate timed states are not individually checked. Timed allocations are zero and primed scratch capacities remain stable. Times are median whole-wave means over 15 fixed rotated/reversed paired blocks, not proof latency.

|Context|Original / OFF / ON µs per wave|ON/OFF ratio[bounds],wins|ON/original ratio[bounds],wins|OFF/original calibration|
|---|---:|---|---|---|
|G2-200-w1|7.671 / 7.704 / 7.762|1.007526 [1.005720, 1.009215], 0/15|1.012649 [1.011439, 1.014596], 0/15|1.005687 [1.004619, 1.006551], 0/15|
|G2-200-w16|8.906 / 8.938 / 9.018|1.008583 [1.004334, 1.012557], 0/15|1.011548 [1.005008, 1.014843], 0/15|1.002532 [0.997555, 1.005292], 4/15|
|G2-200-w32|15.011 / 14.662 / 14.579|1.021374 [0.986057, 1.059776], 4/15|1.006597 [0.952575, 1.047412], 7/15|0.966234 [0.950800, 1.007989], 10/15|
|G2-352-w1|12.980 / 13.029 / 13.174|1.011157 [1.010314, 1.011474], 0/15|1.014127 [1.013476, 1.015370], 0/15|1.003129 [1.002328, 1.004003], 0/15|
|G2-352-w16|15.242 / 15.231 / 15.508|1.018739 [1.013285, 1.020974], 0/15|1.017440 [1.014572, 1.021311], 0/15|1.001049 [0.998032, 1.003893], 7/15|
|G2-352-w32|25.224 / 25.011 / 25.205|1.002673 [0.998449, 1.021031], 6/15|0.999804 [0.989822, 1.014555], 8/15|0.991803 [0.987379, 1.005411], 10/15|
|G1-200-w1|4.213 / 4.220 / 4.216|0.999400 [0.999255, 1.001823], 9/15|1.001215 [0.999744, 1.001864], 4/15|1.001892 [0.999205, 1.002636], 5/15|
|G1-200-w16|4.779 / 4.771 / 4.786|0.999946 [0.995085, 1.010216], 8/15|1.001803 [0.996595, 1.005593], 5/15|1.002878 [0.992957, 1.006752], 6/15|
|G1-200-w32|6.866 / 8.045 / 6.862|0.990854 [0.835620, 1.014560], 9/15|1.003298 [0.861308, 1.027978], 6/15|1.008545 [1.000059, 1.185595], 3/15|

Failed checks: `["G2-200-w32/OFF/frozen/calibration","G2-200-w32/ON/frozen/nonregression","G2-200-w32/ON/frozen/gain","G2-200-w32/ON/OFF/nonregression","G2-200-w32/ON/OFF/gain","G2-352-w32/ON/frozen/gain","G2-352-w32/ON/OFF/gain","G1-200-w32/OFF/frozen/calibration","G2-32aggregate/ON/frozen/gain","G2-32aggregate/ON/OFF/gain"]`.

The fixed gate requires both 32-worker G2 sizes and their equal-log aggregate to improve with upper ratio < 1 and ≥ 10/15 wins against both controls. All nine contexts must have upper ON/control ≤ 1.03; every OFF/original interval must fit factor 1.03. Rank 4–12 of 15 has nominal 96.484375%marginal coverage under independent exchangeable blocks, without familywise coverage. No gain floor, exclusion, extension, favorable subset, size/layout/flag sweep or unchanged reroll is used. Marginal medians and paired medians need not divide into the same ratio; the positive marginal 200/32 estimate and unchanged G1 control estimates are retained without treating them as algorithm gains.

|Context|Original / OFF / ON CPU ms per wave|Original / OFF / ON maximum-RSS counter MiB|Scratch bytes per fixture, all arms|
|---|---:|---:|---:|
|G2-200-w1|0.007672 / 0.007705 / 0.007762|2.75 / 2.75 / 2.75|`[16000,0]`|
|G2-200-w16|0.138261 / 0.138902 / 0.139659|5.00 / 5.00 / 5.00|`[16000,0]`|
|G2-200-w32|0.452707 / 0.450509 / 0.453597|7.50 / 7.75 / 7.75|`[16000,0]`|
|G2-352-w1|0.012981 / 0.013030 / 0.013175|3.00 / 3.00 / 3.00|`[28160,0]`|
|G2-352-w16|0.236210 / 0.236882 / 0.239373|9.00 / 9.25 / 9.25|`[28160,0]`|
|G2-352-w32|0.776989 / 0.777132 / 0.785190|15.25 / 15.00 / 15.25|`[28160,0]`|
|G1-200-w1|0.004214 / 0.004219 / 0.004217|2.75 / 2.75 / 2.75|`[40000,0]`|
|G1-200-w16|0.073774 / 0.073662 / 0.073850|3.75 / 3.75 / 3.75|`[40000,0]`|
|G1-200-w32|0.214182 / 0.213705 / 0.213326|5.25 / 5.00 / 5.00|`[40000,0]`|

Scratch entries are record/tail vector payloads: G2 has 16,000 bytes at 200 points and 28,160 at 352; unchanged G1 has 40,000 at 200. Other fixture arrays, compiler stack, allocator metadata and existing inputs are additional. These figures establish equal retained vector capacities, not equal total RAM or a minimum-RAM bound. Maximum-RSS counters are diagnostic because the prior page-table audit found undercounts on this host. No fresh page-table or memory-pressure qualification is claimed here.

All 405 latency processes finish before six grouped four-event counter panels and six separate fixed 16 M-user-cycle profiles. Counter panels use 16,384 batches per worker and profiles 65,536, with three checked 128-batch primes excluded by FIFO acknowledgement. No owned build/helper/hash/archive/analyzer/second workload overlaps latency/profiles. All captures precede ownership changes and decoding; before/after hashes prove recording bytes unchanged.

|Counter panel|Cycles per wave|Instructions per wave|Cache-miss events per wave|Branch-miss events per wave|Profile samples|
|---|---:|---:|---:|---:|---:|
|stat-G2-200-w32-OFF|1889434|3557199|13.321|468.921|7707|
|stat-G2-200-w32-ON|1920246|3596234|28.251|464.104|7874|
|stat-G2-200-w32-frozen|1885417|3557194|8.622|451.760|7709|
|stat-G2-352-w32-OFF|3249243|6033104|77.908|479.693|13276|
|stat-G2-352-w32-ON|3301662|6115232|135.893|569.131|13461|
|stat-G2-352-w32-frozen|3244132|6033111|64.604|462.814|13265|

Relative to OFF, ON instructions rise about 1.10%at 200 and 1.36%at 352; cycles rise about 1.63%and 1.61%. These are single descriptive panels, not paired confidence intervals or a causal attribution. Generic cache events do not identify a cache level or bandwidth limit. All events run 100%;63,292 profile samples have zero loss, expected PID/task/FORK/mapping/CPU/fixed-period evidence, at least 400 samples per profile and unknown share below 0.1%. Flat attribution and inlining do not isolate removable wall time or absolute period coverage.

|G2 linked caller|Bytes|Instruction sites|IFMA sites|RSP operand lines|Literal stack subtraction sum|
|---|---:|---:|---:|---:|---:|
|frozen|51830|8590|1450|1450|5952|
|OFF|51830|8590|1450|1450|5952|
|ON|52760|8808|1450|1375|4608|

Static IFMA site totals stay 1,450 because separate tree loops replace the old loop sites; execution counts depend on their trip counts. Static sites, RSP operands and literal stack reservations are not dynamic spills, total frames or measured stalls. A smaller literal reservation does not imply faster execution. All six actual linked G1/G2 disassemblies are hash-bound.

All 474 workflow commands and 444 caller processes succeed once. Independent review reconstructs 352 original source files across the two snapshots, recovers exact scalar-tail/finishing bodies, rechecks 385 pins and command/exit/stream hashes, recomputes all latency/CPU/counter-memory statistics and failed gates, and checks every profile symbol bin and sample identity. No successful test, capture or output check is rerun by review. No SDK request, new proof or Gnark export is produced; the canonical ledger remains 28,349 from the preceding full-SDK experiment.

Research was revisited in the primary sources for [truncated Montgomery arithmetic](https://arxiv.org/html/2410.18129v1), [SQIsign SIMD dataflow](https://arxiv.org/abs/2608.13948) and [Hrabar's thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403). Related mechanisms already have local studies; their reported results are not transferred to this prover. The selected heap follows current source dataflow and imports no external implementation.

Accepted source 176 proof comparisons below are inherited, not measured by this caller trial. Supplied M5 CPU/Metal values are historical references. The accepted adoption retains its documented physical 16 calibration engineering exception and false original full-performance gate.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37.0|4.667×|24.0|3.027×|3.7|
|2x2|11.151932|59.0|5.291×|30.0|2.690×|5.9|
|4x2|16.086386|109.0|6.776×|48.0|2.984×|10.9|
|8x2|24.179453|189.0|7.817×|74.0|3.060×|18.9|
|12x2|33.884455|284.0|8.381×|101.0|2.981×|28.4|
|16x2|40.077381|335.0|8.359×|124.0|3.094×|33.5|
|24x2|65.850777|528.0|8.018×|176.0|2.673×|52.8|
|merge-24x1|65.060937|507.0|7.793×|175.0|2.690×|50.7|

The all-eight CPU 10×objective remains active and unmet:0/8. Primary accepted latency is 7.928660 ms against 3.7 ms. Historical merge 54 is 161.883899 ms, without a supplied M5 reference. The preceding compact-SDK trial's experimental 7.873751 ms primary result was not adopted. Historical AWS/Aeglos comparisons predate source 176 and are not refreshed; no AWS resource was created. No new AWS, mobile/ARM/TEE, huge 250/4999, pressure/swap, concurrent throughput or minimum-RAM result follows.

[Protocol](artifacts/design.md), [full results](artifacts/analysis.json), [independent review](artifacts/review.json), [profile symbols](artifacts/profile-symbols.json), [linked code](artifacts/codegen.json), [workflow plan](artifacts/plan.json), [pins](artifacts/pins.json), [publication review](artifacts/publication.json). Archives: [both source snapshots](artifacts/source.tar.gz), [Rust workflow sources](artifacts/workflow.tar.gz), [command receipts](artifacts/receipts.tar.gz), [counter panels](artifacts/counters.tar.gz), [linked disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Executables/libraries and raw perf.data stay local with hashes. Root: `/home/ubuntu/src/zopro-perf-work/g2-denominator-tree176`.

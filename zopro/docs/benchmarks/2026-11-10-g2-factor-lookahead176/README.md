# G2 reverse-factor lookahead

**The fixed candidate is rejected.** Primary G2 at 32 workers has paired ON/OFF **0.999361 [0.983690, 1.010566]**, 8/15 wins, and ON/REF **1.002719 [0.992802, 1.019546]**, 6/15 wins. The 0.064% lower ON/OFF median is retained; its interval includes both improvement and regression. The two-shape G2 aggregate is ON/OFF **0.997949 [0.989035, 1.012473]**, 8/15 wins. These estimates do not qualify a speedup. One control-calibration check also fails. No adoption or full-SDK follow-up is selected.

The new schedule computes the next reverse group's inverse factor before finishing the current group's point coordinates. It retains one prefix chain, the same two factor/inverse products per group, and the original forward pass, scalar-tail service, record layout, point formulas, signs, output order, and field1200. There is no added table, key plan, heap buffer, production pool, or arithmetic product. Local vector liveness, branches, compiler stack and code layout can change. Only native signed-borrowed G2 selects it through a private compile-time flag. OFF and generic release retain original dispatch.

This differs from the earlier two-chain method, which adds three root products, and four-chain phased inversion, which materializes inverse norms in extra passes. Their negative outcomes and those of signed REDC and descriptor scans remain closed. The hypothesis is compiler/CPU overlap of independent work, not fewer mathematical operations or a prediction that proof latency can halve.

New qualification passes on first execution: 288 changing full-width cases with three passes each against independent Arkworks G2 sums and the exact original routine; four sign patterns, both FAST/inverse modes, lengths 0–511, scalar tails, scratch reuse, unchanged prefix records/capacities and immutable inputs; 1,170 exact factor schedules preserving order and two products per group; 57 actual worker/shared-caller contexts using 512 KiB stacks; and 72 complete generic full-MSM fallback cases. These are three new native test functions and one generic test. Nine build/test/link workflow commands succeed without repairs.

All 176 production source hashes remain unchanged. The private snapshot has 174 unchanged original files and two reversible parents: native dispatch/module declaration and common benchmark/test exports. Reversing only the factor scheduling edits recovers every original forward, scalar-tail and point-finishing statement. Native arithmetic reuses the original field1200. The accepted REF core/consumer from the preceding report is reused after full source/hash checks; fresh OFF and ON share the same benchmark API, exact driver/perf helper, native/O3/fat-LTO/one-CGU settings and Rayon dependency.

Real retained G1/G2 point/scalar fixtures and four independent Arkworks complete-MSM references are reused and pinned. Each loop runs one complete query using its pool internally. Scalar reset, grouping/admission, all windowing/arithmetic/merges/weighting/Horner/allocation/drop and affine normalization are timed. Loading, pool construction, three checked primes, one postcheck and reference comparisons are outside the timer. Every timed output matches its independent reference, and point inputs stay unchanged. **These are complete query timings, not full-proof latency.**

|Context|REF ms|OFF ms|ON ms|ON/OFF [bounds]; wins|ON/REF [bounds]; wins|
|---|---:|---:|---:|---|---|
|1x2-A-w32|1.165917|1.170095|1.166157|1.004538 [0.973784, 1.019677]; 7/15|0.992316 [0.974264, 1.017902]; 8/15|
|1x2-B-w1|36.659780|37.216899|37.579353|1.009732 [1.007083, 1.010923]; 0/15|1.024683 [1.022634, 1.026254]; 0/15|
|1x2-B-w16|3.617758|3.628959|3.668052|1.008540 [0.995461, 1.012360]; 5/15|1.010615 [1.001048, 1.026001]; 2/15|
|1x2-B-w32|3.237762|3.259363|3.258180|0.999361 [0.983690, 1.010566]; 8/15|1.002719 [0.992802, 1.019546]; 6/15|
|2x2-A-w32|1.506129|1.499747|1.498054|1.000299 [0.993772, 1.016203]; 7/15|0.996568 [0.983189, 1.005896]; 9/15|
|2x2-B-w1|39.257055|39.838621|40.206144|1.008720 [1.007333, 1.010006]; 0/15|1.023355 [1.021792, 1.024586]; 0/15|
|2x2-B-w16|4.168726|4.197911|4.219633|1.006486 [1.002098, 1.014445]; 3/15|1.010090 [1.002994, 1.020472]; 0/15|
|2x2-B-w32|3.785514|3.807756|3.820961|1.000783 [0.982796, 1.010222]; 7/15|1.004042 [0.991954, 1.031973]; 7/15|

Ratios below 1 are faster. Fifteen fixed matched blocks rotate/reverse contexts and arms; each process measures 17 queries. Bounds are paired process-median ratio ranks 4..12 of 15: nominal 96.484375% marginal median coverage under independent exchangeable blocks, without a simultaneous guarantee. No favorable medians or adverse observations are discarded. Primary and 2×2 G2 solo are 0.973% and 0.872% slower versus OFF, with 0/15 wins each. Positive 32-worker medians and unchanged G1 control medians remain in the table with uncertainty.

Ten of 54 prospective checks fail: one of eight calibrations (2×2-B32 OFF/REF upper 1.032273 exceeds 1.03), five of sixteen nonregression checks against REF, and all four primary/aggregate gain checks. Primary32 calibration passes, but its required gains are unproven. All eight RSS, eight PMU and eight profile quality checks pass. The fixed schedule closes without a changed lookahead distance, threshold, worker count, curve, layout, sample extension or unchanged replay.

|Context|REF / OFF / ON maximum process RSS MiB|
|---|---:|
|1x2-A-w32|24.25 / 25.50 / 23.50|
|1x2-B-w1|15.50 / 15.50 / 15.50|
|1x2-B-w16|28.00 / 26.75 / 27.75|
|1x2-B-w32|34.50 / 34.50 / 31.50|
|2x2-A-w32|29.50 / 27.25 / 29.25|
|2x2-B-w1|27.25 / 27.25 / 27.25|
|2x2-B-w16|42.50 / 43.25 / 42.25|
|2x2-B-w32|54.00 / 57.00 / 60.75|

RSS includes fixture loading, runtime, allocator history and primes; it is neither incremental scratch nor minimum RAM. No new heap buffer is added, but exact compiler stack use is not independently measured. The 2×2-B32 RSS guard is max(54,57)×1.05+1 = 60.85 MiB, and ON reaches 60.75 MiB. The screen passes; it does not establish equal memory use.

All 440 serialized capture commands complete on first execution: 24 preflights, 360 latency processes, eight PMU panels, eight profiles and 40 offline commands. There are 400 caller processes and **14,144 complete checked queries**, including 1,200 primes and 400 postchecks. All latency precedes diagnostics; every recording precedes offline analysis. During capture only confirmed live session 14544 is polled through terminal exit 0. No owned build/helper/hash/review/report/push overlaps recording.

All PMU panels meet 99.9% running. Eight profiles qualify with **50,543 samples**, minimum **2990**, maximum unknown **1.133%**, zero LOST, fixed 16M-cycle period and PID/TID/CPU/event/MMAP checks. The candidate function appears in all three changed G2 ON profiles and is absent from the unchanged G1 profile. Function sample shares include all its arithmetic and are not removable wall-clock fractions.

|Single-panel context|ON/OFF instructions|ON/OFF cycles|ON/OFF cache misses|ON/OFF branch misses|
|---|---:|---:|---:|---:|
|1x2-B-w16|1.005242|1.009708|0.993706|0.953165|
|1x2-B-w32|1.005030|1.005228|1.142857|1.018589|
|2x2-B-w32|1.004993|0.996505|0.990802|1.016540|
|2x2-A-w32|0.999968|0.998092|1.010942|0.991484|

G2 instructions rise about 0.50–0.52% in these single panels despite conserved multiplication counts. The panels do not identify a unique branch, register, stack or cache cause. CPU service, minor faults, switches and RSS are retained separately in the review; the unchanged driver does not expose major faults. A separate same-agent reader checks 407 capture pins, all 440 command/stream/managed-environment receipts, 400 caller outputs, all 54 recomputed decisions and 50,543 samples. No old field or successful unrelated suite is replayed for bookkeeping.

The [research note](artifacts/research.md) records primary sources and targeted prior-result checks. [MQX](https://arxiv.org/html/2509.12494v1) proposes new CPU instructions and studies smaller FHE moduli; its speedups are not BN254 measurements. Existing SimdMSM/Hrabar/memory-bucket ideas already have local audits or trials. No external implementation or reported speedup is imported into this schedule.

The accepted SDK and canonical **43,730** verified-proof inventory remain unchanged. The full CPU AVX 10× objective remains active and unmet, **0/8**. The following accepted local 32-worker full-proof results and supplied historical M5 CPU/Metal references are inherited from the last full-SDK campaign, not measured by this query screen.

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

Accepted primary remains **7.829286 ms / 133.50 MiB** at 32 workers and 9.192237 ms at 16. Earlier 7.803404 ms used the same accepted executable in another campaign. Merge54 remains 162.541995 ms without a supplied M5 reference. No new SDK request, exported proof, AWS/Aeglos, phone/ARM, large-circuit, memory-pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Analysis](artifacts/analysis.json), [separate review](artifacts/review.json), [qualification/build audit](artifacts/build-review.json), [source audit](artifacts/source-audit.json), [identities](artifacts/identities.json), [CPU diagnostics](artifacts/diagnostic-summary.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.json), [continuation and research loop](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: /home/ubuntu/src/zopro-perf-work/g2-factor-lookahead176.

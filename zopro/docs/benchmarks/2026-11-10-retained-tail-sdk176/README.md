# Retained-capacity SIMD tails: complete-proof comparison

**The candidate has positive aggregate evidence but is not adopted: 76/77 checks pass.** Primary32 medians are **7.723556 ms ON**, **7.816160 ms fresh OFF** and **7.772288 ms accepted REF**. Paired ON/OFF is **0.990592166 [0.983646417,0.997134324], 12/15**; ON/REF is **0.996947597 [0.986145426,1.001349429], 11/15**. The primary accepted-reference gain interval crosses parity.

Equal-log ALL8 ON/OFF is **0.994348889 [0.992132670,0.999927414], 12/15** and ON/REF is **0.994433361 [0.991258390,0.997949884], 13/15**; both aggregate gain checks pass. The primary OFF comparison also passes. All 11 calibrations, 22 nonregression checks, 22 RSS guards and 18 diagnostic-quality checks pass. These positive signals remain evidence; they do not override the one failed prospective primary-gain rule or justify an unchanged rerun. The displayed ON/REF medians differ by **48.733 µs**; that difference and the **0.305%** paired median improvement summarize different distributions.

The candidate pads an incomplete vector in the original tree reducer only when existing pair-index and selected group/prefix-record capacities already fit. It uses an ordinary nonzero dummy pair, original vector arithmetic/inversion, real-only scatter and truncation before original compaction. The [native qualification](../2026-11-10-retained-tail-lanes176/README.md) is inherited: 130 core tests, 6,960 new cases/797,472 Arkworks point comparisons and 114 threaded bucket trees. No native test or build is replayed here. All 176 production originals remain exact. No new requested capacity, table, key copy, solver plan, pool or RAM cap is added.

REF is the unchanged accepted `7aff` SDK. OFF/ON reuse the exact qualified `c6f0`/`2030` SDKs at their existing paths, without copying or rebuilding. All use original field `1200`, dense busy reset, the same allocator/environment and identical fixed-placement driver/support, with native/O3/fat-LTO/one-CGU SDK links.

## Fixed complete-proof results

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. ALL8+merge54 run at 32 workers; primary also at 1/16. Solo uses CPU2; otherwise worker i pins to CPUi and the main thread retains its context mask. The SDK driver uses **default worker stacks**, with no explicit `stack_size` request and no `RUST_MIN_STACK` setting in the launch environment. The previous SIMD-admission report incorrectly attributed the native tests' requested 512 KiB stacks to SDK timing; the actual driver and recordings are unchanged. This is a reporting correction, not a runtime change or a reason to repeat earlier measurements.

Timers include witness solving, randomized proving, internal verification and serialization. Key load, pool creation, three primes and final export are excluded. Fifteen fixed blocks rotate/reverse contexts and arms; each process measures 31 requests. Values are medians of arithmetic process means of integer-rounded SDK nanoseconds. Paired ratios need not equal quotients of displayed medians. Bounds use sorted zero-based ranks 3/7/11, nominal 96.484375% marginal median coverage conditional on independent exchangeable blocks, without familywise or stationarity guarantees. Every block is retained; there are no exclusions, extensions, threshold changes or rerolls.

|Context|REF ms|OFF ms|ON ms|ON/OFF [bounds], wins|ON/REF [bounds], wins|
|---|---:|---:|---:|---|---|
|1x2-w32|7.772288|7.816160|7.723556|0.990592166 [0.983646417,0.997134324], 12/15|0.996947597 [0.986145426,1.001349429], 11/15|
|2x2-w32|10.929386|10.911807|10.860930|0.998692951 [0.990357185,1.004938316], 9/15|0.990624198 [0.983400772,1.000930048], 11/15|
|4x2-w32|15.731643|15.762712|15.672670|0.993988535 [0.982931388,1.001762520], 11/15|0.993347215 [0.992549138,1.000645482], 11/15|
|8x2-w32|23.965774|23.914061|23.839760|0.996619221 [0.993866719,1.002804882], 10/15|0.993052723 [0.986414392,1.001146951], 11/15|
|12x2-w32|33.742953|33.765275|33.833125|1.002249436 [0.989375557,1.007456839], 5/15|0.998758334 [0.991843065,1.006784928], 8/15|
|16x2-w32|39.704242|39.762688|39.631169|0.997785346 [0.990593932,1.008242927], 10/15|0.998150888 [0.992857956,1.006057088], 9/15|
|24x2-w32|66.034077|66.050977|65.779342|0.995722204 [0.989894987,1.005848167], 9/15|0.993783924 [0.984441427,1.003590246], 10/15|
|merge-24x1-w32|65.387417|65.223247|65.106761|0.997306029 [0.994569805,0.998906443], 12/15|0.994140244 [0.990959490,1.002254304], 10/15|
|merge-54x1-w32|162.096312|162.091331|161.688304|0.996180863 [0.987964134,1.010322592], 8/15|0.997482929 [0.991674832,1.004029955], 9/15|
|1x2-w1|94.218289|93.558216|92.023892|0.983028654 [0.979314139,0.987720089], 15/15|0.976376576 [0.971802574,0.981466551], 15/15|
|1x2-w16|9.049269|9.020672|8.929519|0.988889559 [0.984074367,0.996403072], 13/15|0.986784433 [0.981479459,0.993863407], 13/15|


The fixed gate requires 11 OFF/REF calibrations within factor 1.03, 22 ON/control nonregression upper bounds≤1.02, primary32 and equal-log ALL8 gain bounds<1 with ≥10/15 wins against both controls, 22 RSS guards and 18 diagnostic-quality checks. All calibrations, nonregression, RSS and diagnostic checks pass. Only `1x2-w32/ON/REF/primary-gain` fails. Positive aggregate and OFF-comparison evidence is retained; a primary interval crossing parity does not establish either a gain or a slowdown versus accepted.

## CPU work, profiles and memory

|Context|ON/OFF instructions|ON/OFF cycles|ON/OFF branch misses|ON/OFF generic cache misses|OFF → ON G1/G2 reducer samples|
|---|---:|---:|---:|---:|---|
|1x2-w32|0.986488|0.989168|0.991977|1.084092|552/657 → 498/555|
|1x2-w16|0.987460|0.984339|0.987087|1.000691|421/558 → 430/467|
|2x2-w32|0.991730|0.986035|1.000055|0.920840|943/803 → 827/714|


All three panels execute fewer instructions and user cycles against OFF. Primary32 instructions fall from 1.370569 to 1.352050 billion per proof, and cycles from 803.813 to 795.106 million. Primary32 flat G1/G2 reducer samples are 552/657 OFF and 498/555 ON; original streaming kernels remain major populations. These are single descriptive panels and flat sample counts, not exclusive removable work, local CPI, critical-path cost or proof that one specific tail size caused the change. Generic cache misses can increase despite lower cycles.

All nine grouped PMU panels pass ≥99.9% scheduled coverage. All nine 16M-cycle profiles pass PID/TID/CPU/executable-MMAP/event checks, sample/unknown limits and zero LOST. There are **118,758 samples**, with maximum unknown share below 2.07%. Absolute period coverage is unvalidated. No extra profile or decoder is run to improve an unfavorable result.

|Context|REF / OFF / ON peak RSS MiB|REF / OFF / ON CPU ms per proof|REF / OFF / ON minor faults per proof|
|---|---:|---:|---:|
|1x2-w1|34.25 / 34.25 / 34.25|94.215 / 93.552 / 92.021|0.000 / 0.000 / 0.000|
|1x2-w16|80.75 / 79.50 / 80.50|116.006 / 115.319 / 113.747|206.452 / 203.194 / 188.742|
|1x2-w32|112.75 / 108.25 / 110.25|177.697 / 177.645 / 175.761|404.484 / 422.548 / 416.355|
|2x2-w32|269.75 / 281.00 / 277.50|257.666 / 255.366 / 254.004|1270.452 / 1280.935 / 1230.677|
|24x2-w32|1291.83 / 1288.64 / 1290.68|1926.768 / 1918.848 / 1914.618|4617.677 / 6675.774 / 5049.839|
|merge-54x1-w32|1989.32 / 1953.00 / 1985.17|4505.988 / 4515.909 / 4502.479|22447.839 / 22167.968 / 22196.677|


Every comparative RSS guard passes: ON maximum ≤1.05×control maximum+1 MiB. Peaks include image load, primes and allocator history; they are not minimum RAM, pressure/swap qualification or a universal memory reduction. The driver reports minor faults, not major-fault service. Fault counts do not establish disk paging or a causal DRAM limit. Capacity-only source behavior does not imply identical process high-water RSS.

## Verification and provenance

All 44 preflight commands pass: 22 OFF/ON SDK processes covering all nine shapes and primary at 1/16 workers, then 22 independent Gnark verifications. They contribute 88 internally verified requests and 22 unique fresh exports. The checked canonical inventory advances 44,800→44,822 only after verification/freshness checks. Preflight clocks select no performance result.

All **1,071 capture commands** finish once: 495 latency processes, 18 diagnostic processes, 45 offline commands and 513 independent Gnark verifications. Capture contributes 19,422 requests and 513 unique exports. Stage totals are **19,510 SDK requests /535 fresh independently verified exports**. Proofs are 164 bytes with expected image/public/proof bindings. The canonical inventory reaches **45,335**. The number of exported independent proofs differs from all internally produced requests. Private key/request/witness/proof payloads and full canonical-ledger payloads remain local.

Producer and separate same-agent raw reader are compiled and frozen before the first preflight. Both post-capture analyses pass on their first executions. The reader independently reconstructs 495 raw integer means, all medians/rank ratios and 77 decisions, checks 267 capture pins, proof bindings/freshness and production-source recovery. SDK structure and detailed PMU/profile parsers are shared; this is not an external audit or repeated cryptographic verification.

There are **1,126 successful workflow commands through report derivation**, with no new failed command in this stage:5 helper/binding steps, 44 preflight, 3 inventory/freeze, 1,071 capture, 2 post-analysis and 1 derivation. Publication/checkpoint and direct preparer/compiler invocations are separate. Qualification's historical setup failures remain in its linked report and are not counted as new or replayed. During preflight and recording, only the exact confirmed live handles are polled until terminal; no owned helper/build/hash/analysis/report overlaps recording. Shared-host exclusivity is not claimed.

No new AWS/Aeglos, ARM/mobile/TEE, huge 250/4999, minimum-RAM, pressure/swap or concurrent-throughput result is added. The inherited no_ifma core compile used the original native field library and is not full generic-field or physical mobile execution.

## Supplied CPU and Metal comparisons

Fresh local REF/ON observations follow. M5 CPU and Metal values are supplied references on different hardware, not fresh measurements or equal-hardware-efficiency estimates. ON is unadopted.

|Shape|Accepted REF ms|Unadopted ON ms|M5 CPU ms / REF speedup / ON speedup|M5 Metal ms / REF speedup / ON speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.772288|7.723556|37 / 4.761× / 4.791×|24 / 3.088× / 3.107×|3.7|
|2x2|10.929386|10.860930|59 / 5.398× / 5.432×|30 / 2.745× / 2.762×|5.9|
|4x2|15.731643|15.672670|109 / 6.929× / 6.955×|48 / 3.051× / 3.063×|10.9|
|8x2|23.965774|23.839760|189 / 7.886× / 7.928×|74 / 3.088× / 3.104×|18.9|
|12x2|33.742953|33.833125|284 / 8.417× / 8.394×|101 / 2.993× / 2.985×|28.4|
|16x2|39.704242|39.631169|335 / 8.437× / 8.453×|124 / 3.123× / 3.129×|33.5|
|24x2|66.034077|65.779342|528 / 7.996× / 8.027×|176 / 2.665× / 2.676×|52.8|
|merge-24x1|65.387417|65.106761|507 / 7.754× / 7.787×|175 / 2.676× / 2.688×|50.7|


Accepted primary remains about **7.77 ms**, the same executable previously observed at 7.770239 ms. No default changes. Merge54 has no supplied M5 reference. Both accepted and candidate meet **0/8** CPU10× targets; the original full goal remains **active and unmet**.

This fixed recipe closes without an unchanged rerun, worker/tail-size/parameter rescue, threshold change or favorable-subset promotion. The measurable positive signals should be retained when selecting a materially different removal of useful work or dependencies. They do not establish a twofold breakthrough or satisfy the declared complete adoption rule.

[Results](artifacts/analysis.json), [raw reader](artifacts/review-result.json), [decision](artifacts/decision.json), [fresh CPU/Metal comparisons](artifacts/fresh-comparison.json), [diagnostic summary](artifacts/diagnostic-summary.json), [protocol](artifacts/protocol.md), [identities](artifacts/identities.json), [inherited link review](artifacts/link-review.json), [preflight inventory](artifacts/preflight-review.json), [prospective review](artifacts/prospective-review.json), [proof inventory summary](artifacts/proof-inventory.json), [commands](artifacts/receipts.json), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The indexed UTF8 archive retains full rows, authored helpers, plans/pins, command streams, decoded profiles and new-export hash bindings. Executables/libraries, raw binary perf, private payloads, unchanged snapshots and full canonical-ledger payloads are excluded.

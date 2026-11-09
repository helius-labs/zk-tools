# Singleton-copy elision: complete SDK comparison

**Not adopted.** Skipping singleton scratch copies does not improve complete-proof latency. Primary 1×2 is **0.779% slower than accepted REF** and **0.870% slower than CONTROL** by paired geometric means. The original-eight aggregate is **0.476% slower than CONTROL**, with its 95% interval wholly above parity. Three of 43 prospectively declared selection checks fail; the exact candidate is closed. Accepted production remains unchanged.

The accepted build still measures **7.857170 ms** on 1×2 in this campaign, versus the candidate's **7.928927 ms**. Earlier accepted sub-8 ms measurements remain valid. This is not a new production timing record or an AWS/Aeglos comparison.

All **1,137 workflow commands**, **42,630 internally verified SDK requests** and **546 fresh unique independently Gnark-verified exports** complete successfully. The separate raw review confirms timings, peaks, proof/public/image/VK bindings, worker placement, all 30 pairwise contrasts and all 43 decisions. The all-eight CPU 10× objective remains active and unmet, **0/8**. [Results](artifacts/results.json), [raw review](artifacts/review-result.json).

## Change and measurement

The [native-qualified candidate](../2026-11-09-singleton-elision-native176/README.md) writes empty/singleton scalar classes directly into the existing dense output and copies only repeated classes into reduction scratch. Repeated-class arithmetic, exceptional points, class order, grouping admission, output format and scheduling remain original. It adds no vector, key/witness/scalar/point cache, execution plan or pool. Requested point capacity per tile cannot increase, but RSS is not determined by that bound. Main primary queries can decline dense grouping; this was disclosed before timing. No curve-addition reduction or twofold gain was predicted. Native and field qualification are inherited without replay.

REF uses the retained accepted libraries. CONTROL reuses the existing fresh accepted-source SDK from the previous campaign; it is not rebuilt again here. SINGLETON uses its coherent native core/prover/zolana libraries. All use the exact inherited fixed-placement driver, one global Rayon pool/default stacks, worker i on CPU i, main broad 0..N−1 and N+1 tasks checked before/after. This common condition is not a production pinning default. Ryzen 9950X, 16 physical cores/32 logical CPUs, shared host.

Thirty one-request preflights precede 498 latency processes: 31 primary32 blocks and 15 blocks for each other context. Each process has three full primes, then 151 primary16/32 or 51 other requests. Solving, fresh randomness, proving, internal verification and serialization are timed; loading is excluded. Context/arm order rotates and reverses. Every recording finishes before independent Gnark checks and profile decoding; no owned build/helper/analysis/archive workload overlaps recording. All raw observations remain. [Frozen design](artifacts/design.md), [protocol](artifacts/protocol.json), [prospective review](artifacts/prospective-review.json).

## Latency and selection

Values are medians of process means. Ratios are paired geometric means and need not equal quotients of the displayed medians. Intervals use the inherited 20,000-resample paired-log bootstrap, fixed seed and ranks 499/19,499; they assume exchangeable blocks without familywise or cross-host coverage.

|Context|REF ms|CONTROL ms|Candidate ms|Candidate/CONTROL [95% interval]|Wins|
|---|---:|---:|---:|---|---:|
|1x2-w32|7.857170|7.867835|7.928927|1.008698 [1.006755, 1.010517]|2/31|
|2x2-w32|11.052363|10.999189|11.029137|1.002799 [0.997217, 1.008365]|7/15|
|4x2-w32|15.952018|15.749429|15.980955|1.008458 [1.002299, 1.014941]|3/15|
|8x2-w32|24.080322|23.963587|24.338481|1.013764 [1.005626, 1.021879]|4/15|
|12x2-w32|34.012417|34.121561|33.915971|0.995188 [0.987178, 1.003979]|10/15|
|16x2-w32|40.128912|40.162845|40.152829|1.002586 [0.998940, 1.006275]|5/15|
|24x2-w32|66.348981|66.563379|66.502645|1.001218 [0.996473, 1.006034]|7/15|
|merge-24x1-w32|65.829291|65.340677|65.694962|1.005481 [0.999115, 1.011420]|5/15|
|merge-54x1-w32|163.199880|162.141161|162.878516|1.001058 [0.996081, 1.006114]|7/15|
|1x2-w16|9.226989|9.160528|9.237749|1.007424 [1.005688, 1.008827]|1/15|

Original-eight equal-log candidate/CONTROL is **1.004760111 [1.002641099, 1.006893689]**; candidate/REF is **1.001610808 [0.998889898, 1.004160670]**, without an established gain. Primary32 candidate/REF is **1.007793821 [1.006108311, 1.009386129]**, 4/31 wins; candidate/CONTROL is **1.008698075 [1.006754623, 1.010516957]**, 2/31. Its descriptive rank intervals also lie above parity.

The frozen rule requires all ten control-calibration intervals inside [1/1.03,1.03], all twenty candidate/control nonregression upper bounds ≤1.02, all ten RSS screens, both original-eight aggregate gain upper bounds <1, and automatic diagnostic quality. Primary gains/wins were explicitly descriptive before the first SDK call because this candidate can leave primary main queries unchanged; they were not removed after seeing results. All shapes still enter selection. The preceding composition's nine failed checks remain failed.

**Failed checks, retained:**

- 8×2 candidate/CONTROL nonregression: **1.013763885 [1.005626105, 1.021878895]**, 4/15 wins; upper bound exceeds 1.02.
- Original-eight aggregate gain versus CONTROL.
- Original-eight aggregate gain versus REF.

All ten calibrations, all ten RSS screens, 19/20 nonregression checks and diagnostic quality pass: 40/43 total. This is not a rejection caused by failed control calibration. There is no positive-gain floor, exclusion, extension, unchanged retry, layout/worker/threshold change or favorable-shape promotion. Positive point estimates on 12×2 and merge54 remain visible but do not rescue the full candidate.

## Memory and CPU observations

RSS is the maximum lifetime rusage value across each cell's latency processes, including loading/primes. These values are not minimum RAM or a rigorous live working-set bound. No memory-pressure, swap or device claim follows.

|Context|REF RSS MiB|CONTROL RSS MiB|Candidate RSS MiB|
|---|---:|---:|---:|
|1x2-w32|135.500|133.000|134.000|
|2x2-w32|290.500|294.500|291.250|
|4x2-w32|326.750|331.000|340.000|
|8x2-w32|541.500|536.750|530.250|
|12x2-w32|749.750|750.500|744.250|
|16x2-w32|858.500|857.750|855.000|
|24x2-w32|1365.664|1399.750|1362.250|
|merge-24x1-w32|1240.035|1246.418|1281.250|
|merge-54x1-w32|2035.039|2076.883|2044.262|
|1x2-w16|87.250|87.250|88.000|

The screen is candidate≤max(REF,CONTROL)×1.05+1 MiB in each context. Smaller requested scratch capacity does not yield consistently smaller observed RSS: merge24 candidate reaches 1281.25 MiB versus 1240.035/1246.418 MiB. All limits and observations remain in the results. The [separate allocation-lifetime diagnostic](../2026-11-09-proof-lifetime-census186/README.md) already distinguishes current Rust requested storage from free allocator arenas/RSS; its instrumented counts are not reused as latency or minimum-RAM estimates here.

[All 30 resource cells](artifacts/resources.json) retain median per-process CPU service, minor/major faults and switches per request. CPU service combines user+system within each process before taking the median. Major faults are available in the unchanged driver's raw summaries; no new driver counter was added.

Nine PMU panels and nine fixed-period warm profiles cover primary16/32 and merge54 for all arms. All 36 event values have 100% scheduled time. All 133,443 samples qualify with zero LOST, 100% assigned thread samples and maximum unknown 1.684293%. One panel/profile per arm/context is descriptive, not a repeated-counter confidence or cache/DRAM/spill/critical-path claim.

|Primary32 arm|Instructions M/request|User cycles M/request|
|---|---:|---:|
|REF|1386.883992|815.202024|
|CONTROL|1387.754065|812.620870|
|SINGLETON|1388.009087|819.480031|

The primary instruction count does not show a material work reduction. Flat profile samples remain concentrated in G1 streaming, borrowed G2 application, epoch pinning and inversion. CONTROL→candidate samples are 2666→2661 for G1 streaming, 2063→2093 for borrowed G2, 1036→1063 for pinning and 924→848 for scalar inverse. Those are not exclusive removable wall costs or causal attribution. [Complete counter/profile analysis](artifacts/diagnostic-analysis.json).

## Review and retained failures

The original frozen raw reviewer rejects exact JSON equality of a recomputed RSS-limit summary 924083.2000000001 versus reparsed 924083.2 KiB. Original source/binary/pins stay unchanged. A separate post-capture reader changes exactly one summary comparison to its existing 1e−11 relative/absolute numeric comparator. Names, booleans, integers, all raw data and threshold calculations are unchanged. The reverse edit and pre/post analysis/diagnostic/ledger hashes are checked. The corrected reader passes on its first execution, rebuilding every gate and proof binding. No capture or successful scientific stage is repeated. Shared inherited statistics/profile parsers are disclosed; this is not an external mathematical audit. [Exact repair](artifacts/review-comparison-repair.json).

Before any SDK call, the capture-preparation helper initially fails to compile its large json! under the default macro-recursion limit; adding a crate limit 256 repairs it. The failed source is retained. Compiler unused/dead-code warnings remain; no successful build was repeated to tidy warnings. All 1137 actual workflow commands succeed.

## Supplied M5 CPU and Metal comparison

These are fresh candidate measurements on this Ryzen, with supplied historical M5 references on different hardware. The candidate is rejected; the table does not claim a production speedup. All eight comparisons are retained.

|Shape|Candidate ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928927|37 / 4.666×|24 / 3.027×|3.7|
|2x2|11.029137|59 / 5.349×|30 / 2.720×|5.9|
|4x2|15.980955|109 / 6.821×|48 / 3.004×|10.9|
|8x2|24.338481|189 / 7.765×|74 / 3.040×|18.9|
|12x2|33.915971|284 / 8.374×|101 / 2.978×|28.4|
|16x2|40.152829|335 / 8.343×|124 / 3.088×|33.5|
|24x2|66.502645|528 / 7.940×|176 / 2.647×|52.8|
|merge-24x1|65.694962|507 / 7.717×|175 / 2.664×|50.7|

Merge54 is 162.878516 ms without a supplied M5 reference. The accepted build's matched 1×2 remains 7.857170 ms; earlier accepted standard 7.915095 ms/fixed 7.852740 ms remain valid, and the prior unadopted compound's 7.753327 ms is not a production record. The original all-eight CPU 10× objective remains unmet, 0/8.

## Continuation and artifacts

Close this exact copy-elision recipe. It does not supply the needed gain; another unchanged copy/layout or allocator-setting retry is not selected. Next work needs a distinct substantial ordinary G1/G2 useful-work reduction, checked against existing negative experiments and complete resource/consumer requirements. No replacement is selected. Full all-eight/merge54 proof, resource/admission/portable/concurrency qualification remains necessary before any future adoption. [Continuation loop](artifacts/next.md).

The canonical independent inventory advances 39,608→40,154 unique exports. Production 176/SDK 6d83 remain unchanged. Capture 35463 is terminal with exit code 0, its stored handle is cleared and no recording or owned cloud resource is live. No new AWS/Aeglos, ARM/phone, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result follows.

[Exact UTF8 evidence](artifacts/evidence.json.gz) preserves full analysis/raw request values, command/stream receipts, decoded profiles, immutable pins, readers and failed/repaired helper versions. Executables/libraries/raw perf and private key/witness/proof bytes/full ledger stay local. [Publication checks](artifacts/publication.json), [checksums](SHA256SUMS). Private root: `/home/ubuntu/src/zopro-perf-work/singleton-elision-sdk176`.

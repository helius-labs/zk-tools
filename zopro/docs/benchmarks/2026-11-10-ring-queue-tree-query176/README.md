# Bounded queue trees: complete-query comparison

**The candidate is slower on the primary G2 query and is not adopted.** At 32 workers it measures **4.120862 ms**, versus **3.020862 ms fresh OFF** and **3.218345 ms accepted-source REF**. Paired ON/OFF is **1.364134 [1.340028, 1.381608]**, with **0/15 wins**; ON/REF is **1.279365 [1.263986, 1.286400]**, also **0/15**. These are full G2 query timings, not proof times. Accepted local 1×2 full-proof latency remains **7.783594 ms**.

The frozen screen fails **21/46 checks**, including five OFF/REF calibrations. Calibration failure limits attribution of the exact ratios solely to the algorithm. It does not turn this artifact into an improvement: primary loses all 15 blocks against both controls at every tested worker count. All twelve diagnostic-quality and both unchanged-A stability checks pass. No observation, threshold, layout, compiler or worker context is changed after capture. This fixed integration closes without an SDK trial or unchanged rerun.

The [native-qualified queue trees](../2026-11-10-ring-queue-tree-native176/README.md) execute pending work, reduce at most 199 deferred operands by destination through balanced complete pair trees, then attach one root per bucket. Identity/double/opposite/cancelling roots retain complete behavior. The one-array unit phases, radix 128, nineteen positions, two partitions, batch 200/queue 199, leading indexed window, axes, weighting and original field kernels stay fixed. Existing Window fields stay unchanged; local roots/descriptors/grouping scratch are additional working storage charged before admission. The earlier model's FAIL2/14 and its explicit prospective native-selection exception remain recorded; neither is rerun or relabeled.

Both exact qualified ON/OFF cores and field1200 are reused. Only missing consumer links are built at native/O3/fat-LTO/one-CGU. The retained API-only REF is verified against all 176 current accepted sources and its dense-bitmap-reset flags, then reused without rebuilding. The original driver, perf support and independent Arkworks whole-MSM fixture oracles retain exact bytes. Grouping is enabled: primary ON enters route 317; grouped2×2 and unchanged A bypass it at route 0. No raw route is forced.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Fifteen fixed rotated/reversed blocks cover seven contexts and three arms. Each process makes three checked primes, seventeen measured queries and one postcheck. Solo uses CPU 2; other worker i uses CPU i, with 512 KiB requested stacks. Timing excludes loading/pool construction and includes scalar reset, grouping/admission, Phi/ring recoding, all windows/top/merges/axes/weighting, normalization, allocation and drop. Every output matches the inherited independent complete-MSM oracle and point inputs remain unchanged.

Times are medians of process medians; paired-ratio medians need not equal ratios of displayed times. Bounds are fixed ranks 4–12 of 15, nominal 96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee. All data remain.

|Context|REF ms|OFF ms|ON ms|ON/OFF [bounds], wins|ON/REF [bounds], wins|
|---|---:|---:|---:|---|---|
|1x2-A-w32|1.175493|1.170053|1.173359|0.999114 [0.987601, 1.012125], 8/15|0.993334 [0.981508, 1.006836], 9/15|
|1x2-B-w1|36.799204|38.110881|42.450548|1.113717 [1.112638, 1.115720], 0/15|1.155952 [1.151709, 1.157711], 0/15|
|1x2-B-w16|3.604765|3.419086|4.527982|1.337867 [1.321266, 1.347156], 0/15|1.258821 [1.242327, 1.268941], 0/15|
|1x2-B-w32|3.218345|3.020862|4.120862|1.364134 [1.340028, 1.381608], 0/15|1.279365 [1.263986, 1.286400], 0/15|
|2x2-B-w1|39.349859|40.371227|39.624854|0.981470 [0.980230, 0.982514], 15/15|1.007070 [1.005335, 1.007627], 1/15|
|2x2-B-w16|4.202405|3.863174|4.153011|1.079877 [1.064137, 1.091429], 0/15|0.989776 [0.980006, 1.006392], 10/15|
|2x2-B-w32|3.766541|3.443492|3.836013|1.108198 [1.094900, 1.125158], 0/15|1.017485 [1.003352, 1.022917], 2/15|

The grouped 2×2 solo estimate is positive against OFF: ON/OFF is 0.981470 [0.980230,0.982514], 15/15 wins. Its ON/REF is 1.007070 [1.005335,1.007627], 1/15. This context bypasses the ring, so that comparison does not establish an arithmetic gain from queue trees. Grouped 2×2 also shows unfavorable threaded ON/OFF differences while bypassing the new code. Their exact cause remains unisolated; no exclusive compiler-layout or scheduler explanation is claimed. The complete table and failed calibrations prevent favorable-subset promotion.

The prospective 46-check screen requires seven control calibration intervals inside factor 1.03, fourteen ON/control upper nonregression ratios ≤1.02, primary32 and equal-log two-population32 gains against both controls with upper<1 and ≥10/15 wins, unchanged-A intervals inside factor 1.03, seven RSS guards and twelve diagnostic-quality checks. There is no minimum gain percentage. Failed checks comprise five calibrations, nine nonregressions, three primary RSS guards and four gain checks. Both aggregate gain checks fail. Every named check and its input ratios remain in the [analysis](artifacts/analysis.json) and [independent review](artifacts/review.json).

Primary32 resources below are process high-water RSS and per-query service/fault/switch differences. They are not minimum RAM, measured fault service time, swap or a live ownership bound. The unchanged driver does not expose major faults.

|Arm|CPU ms/query|Maximum process RSS MiB|Minor faults/query|Voluntary switches/query|
|---|---:|---:|---:|---:|
|REF|70.047|33.25|50.824|198.471|
|OFF|71.870|35.50|51.412|202.059|
|ON|87.184|115.00|734.176|254.294|

The smaller ring bucket payload does not translate into lower process memory than accepted. Candidate primary32 RSS is 115 MiB versus 35.5 MiB OFF/33.25 MiB REF. This trial does not isolate allocation retention or minor faults as the cause of its latency. The earlier 99,728-byte/worker extra forecast is not the measured RSS increase. Query RSS must not be compared directly with full-proof RSS.

After all 315 latency processes, six grouped PMU panels and six separate cycles:u profiles each run 200 checked queries after three primes. All 348 recording processes finish before 30 offline access/decode/loss/MMAP steps. FIFO gates exclude loading/primes and postchecks. Profiles use 16,000,000-cycle period and 512 pages. All six qualify, totaling **23,777 samples**, unknown <1%, zero LOST records, with checked process/thread/CPU/executable-MMAP identities. All PMU events report 100% scheduled coverage.

|Primary32 panel|OFF|ON|
|---|---:|---:|
|Instructions/query|563390437|579051096|
|Cycles/query|324918147|390357973|
|Generic cache misses/query|1173993|3215246|
|Branch misses/query|310158|989220|

In primary32 flat profiles, inverse-category samples fall 304→136 while MSM/curve samples rise 3088→3972. The inversion-related reduction is retained, but does not establish removable wall time or a causal decomposition. Single counter panels show more whole-query instructions/cycles; generic misses do not identify a cache level, DRAM traffic or bandwidth. The fixed ring also carries unit routing, endomorphism maps, nineteen windows/axis reductions and different memory lifetimes. Fewer modeled vector products/inverse stages were insufficient at the complete caller.

All 378 capture commands succeed once: 21 preflights, 315 latency, 6 PMU, 6 profiles, 30 offline steps. All **9,168 query invocations**, including primes/postchecks, match their oracle. Frozen analysis and separate same-agent raw reader pass first execution; the reader independently reconstructs ratios/resources/all 46 decisions while sharing sampling/PMU text parsers. All 614 frozen pins and 176 production identities close. Old fixture-oracle qualification is inherited, not counted again as new cryptographic verification.

Preparation has one retained helper compile failure: an extra delimiter in build review, after both consumer links and the raw reader succeed. The corrected reader checks both workflows; capture preparation changes only build-result/pin metadata paths. The two changed helpers compile; successful consumer links/raw reader and all native qualification are reused. Across seven build attempts, 378 capture commands and two post-analysis commands, **387 workflow commands comprise 386 successes and one retained compile failure**. Setup/prospective/publication helper calls are outside this receipt count. Every raw observation and failure is retained; no successful scientific capture is repeated.

No SDK request, proof export, Gnark verification, AWS/Aeglos, phone/ARM/TEE, huge 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is added. Canonical proof inventory remains 44,265. The full CPU10x goal remains active and unmet, **0/8**. The accepted full-proof CPU/Metal observations below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md), not remeasured here.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.783594|37 /4.754x|24 /3.083x|3.7|
|2x2|10.932731|59 /5.397x|30 /2.744x|5.9|
|4x2|15.757088|109 /6.918x|48 /3.046x|10.9|
|8x2|23.973295|189 /7.884x|74 /3.087x|18.9|
|12x2|33.827269|284 /8.396x|101 /2.986x|28.4|
|16x2|39.900917|335 /8.396x|124 /3.108x|33.5|
|24x2|66.041083|528 /7.995x|176 /2.665x|52.8|
|merge-24x1|65.717994|507 /7.715x|175 /2.663x|50.7|

Accepted primary16 remains 9.016864 ms; merge54 remains 162.170158 ms without a supplied M5 reference. Inherited full-proof primary32 peak 111.25 MiB belongs to the short 31-request campaign; older 133.50 MiB observations remain valid in a different campaign. No memory saving follows from this study.

[Decision](artifacts/decision.json), [analysis summary](artifacts/analysis.json), [raw reader](artifacts/review.json), [protocol](artifacts/protocol.md), [artifact identities](artifacts/identities.json), [build review](artifacts/build-review.json), [prospective freeze](artifacts/prospective-review.json), [workflow receipts](artifacts/receipts.json), [preserved failure](artifacts/failures.md), [continuation/autoresearch loop](artifacts/next.md), [local binary/perf identities](artifacts/retained-local-artifacts.json), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Raw binary/perf payloads, unchanged source snapshots, private fixture/key/point/scalar/witness/proof payloads and full ledgers are excluded. Reproduction needs the pinned private fixtures and qualified native artifacts.

Per-job rows and full command records remain in the indexed archive. Public JSON omits only those duplicated payloads; [raw/public hashes](artifacts/public-views.json) and the [view builder](artifacts/compact-publication.rs) bind both representations. Statistics, gates and observations are unchanged.

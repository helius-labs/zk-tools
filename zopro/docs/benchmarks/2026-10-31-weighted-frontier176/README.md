# Joint completion of weighted-group frontiers

**Rejected at the complete weighted-window gate; not adopted.** The 32-worker aggregate is **31.48% slower for G1** and **37.14% slower for G2** against fresh OFF, with zero winning blocks. Requested-heap peaks also exceed the declared 5% limit in all threaded contexts. Production source 176/SDK `6d83…` is unchanged, and no full-proof trial follows.

Solo improves by **10.06–11.66%** against OFF, with all 15 blocks won in all four cases. These positives remain in the result. They do not establish a threaded prover gain. Two control-calibration failures are retained; the large regressions also occur in six calibrated threaded contexts.

|Curve/control|32-worker two-size ON/control ratio [bounds], wins|
|---|---|
|G1/OFF|1.314823 [1.284592, 1.323173], 0/15|
|G1/frozen|1.301685 [1.289779, 1.321599], 0/15|
|G2/OFF|1.371441 [1.343354, 1.390222], 0/15|
|G2/frozen|1.366729 [1.335801, 1.379228], 0/15|


The candidate preserves the original 256-bucket arithmetic groups and their first five complete reduction levels. It copies each group's live weighted-correction segments into a disjoint 55-point frontier, then calls the original complete reducer across all groups for the last three levels. It reconstructs each original weighted/total pair and applies the original group-offset combination. Initial and intermediate identities, doubling and opposite points retain the complete original handling; no speculative partial output escapes.

For a dense 2048-bucket window, shared inversions decrease from 64 to 43; for 4096, from 128 to 83. Ordinary point additions remain 4016/8032 respectively. Six scalar-tail pairs per group become full SIMD groups in the joint finish. Batch sizes also change which existing G2 prefix-storage branch runs. These are source-geometry counts, not dynamic instruction totals. The new implementation adds a per-window barrier, copies, frontier/frame storage, allocations and a more serial finish. Their complete cost is timed.

This is distinct from the [compact native weighted-group study](../2026-10-31-weighted-tree176/README.md), whose calibration gate remains failed. This trial uses the original host-limb representation and field code throughout. Earlier [flat reductions across whole G1 windows](../2026-10-22-universal-flat-window-reduction-sdk/README.md), [cross-query phases](../2026-10-24-joint-msm-phases/README.md) and [coarser groups](../2026-10-22-universal-coarse-weighted-sdk/README.md) remain separate rejected implementations. No table, key-point copy, transformed key, duplicate solver plan, pool, C/ASM backend or RAM cap is introduced.

Both private snapshots contain 177 files:174 accepted originals exact, two reversible diagnostic exports and one new child. The audit recovers all 176 accepted files twice. All 18 accepted field-crate members, including 16 source files, remain exact in each snapshot. No field arithmetic changes or fresh field tests are claimed; accepted qualification is inherited. Generator consistency passes.

All **128 native core tests** and **two independent integer-model tests** pass. Three new core tests cover complete weighted outputs across empty/small/group/tail sizes through 4096, full-width random points, identities, doubling, opposites, a late-cancellation family, input immutability and repeated 1/4/16/32-worker 512 KiB-stack waves. The integer model imports no field/native implementation and checks every coefficient, group offset and frontier slice bound. No new portable, full-prover or pressure qualification follows.

The separate original diagnostic build and candidate OFF/ON share source and driver bytes. The control omits the new cfg; candidate OFF/ON share one executable with a runtime selector. Explicit noinline baseline wrappers have matching sizes, instruction-site counts, IFMA sites and RSP-operand counts across builds. That is not whole-program machine-code, layout or timing equivalence: the frozen G1 arithmetic is inlined into weighted_sum, while candidate code also retains a separate reduce_layer specialization. All final links use actual optimization 3, native CPU, fat LTO and one codegen unit. Normal executables are frozen before separate instrumented final links overwrite Cargo's top-level files.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Twelve contexts cover G1/G2, 2048/4096 buckets and 1/16/32 workers. Simultaneous windows are 21 for G1/2048, 20 for G1/4096 and 11 for G2. These are representative multiplicities, not captured proof scheduling. Workers pin individually to CPU 2, 0–15 or 0–31 with 512 KiB stacks. Each fixture is a prebuilt generator walk with independent full-width random offset and step; input slices alternate between two shifted families. This is a synthetic dense caller distribution, not a fresh witness stream.

Three checked prime passes of two waves precede 32 timed waves. All weighted-window work, allocation/drop, nested Rayon work and outer dispatch/wait are included. Pool/input construction and independent final checks are excluded. All 624 caller processes agree with independent generator-weighted oracles and output digests across arms. Every prime and both final input families are checked; intermediate timed outputs are not each independently checked.

|Context|Original / OFF / ON ms per wave|ON/OFF ratio [bounds], wins|ON/original ratio [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|G1-2048-w1|3.262114 / 3.302583 / 2.952523|0.894381 [0.892869, 0.896257], 15/15|0.905999 [0.904557, 0.907026], 15/15|1.012128 [1.011422, 1.013625], 0/15|
|G1-2048-w16|0.253844 / 0.254701 / 0.304979|1.193357 [1.184714, 1.214358], 0/15|1.201267 [1.179283, 1.210091], 0/15|1.004550 [0.992124, 1.011067], 6/15|
|G1-2048-w32|0.208290 / 0.210727 / 0.277750|1.320002 [1.302881, 1.336048], 0/15|1.316934 [1.275851, 1.359351], 0/15|1.006684 [0.969139, 1.026991], 6/15|
|G1-4096-w1|6.185494 / 6.270428 / 5.541395|0.883438 [0.882657, 0.884953], 15/15|0.896327 [0.895354, 0.897663], 15/15|1.014586 [1.013030, 1.016631], 0/15|
|G1-4096-w16|0.473688 / 0.478747 / 0.551774|1.149606 [1.143650, 1.161011], 0/15|1.164848 [1.156360, 1.178353], 0/15|1.013258 [1.007229, 1.020941], 2/15|
|G1-4096-w32|0.380319 / 0.382699 / 0.492868|1.278805 [1.259117, 1.309548], 0/15|1.287247 [1.235296, 1.333672], 0/15|1.014775 [0.982971, 1.026495], 6/15|
|G2-2048-w1|3.207870 / 3.252861 / 2.904334|0.893763 [0.888711, 0.898893], 15/15|0.906126 [0.902883, 0.908544], 15/15|1.014511 [1.008845, 1.019596], 1/15|
|G2-2048-w16|0.278378 / 0.280448 / 0.349907|1.240575 [1.223814, 1.256620], 0/15|1.241624 [1.228082, 1.276755], 0/15|1.013271 [0.987744, 1.023057], 4/15|
|G2-2048-w32|0.251451 / 0.253030 / 0.337063|1.332479 [1.295308, 1.379165], 0/15|1.327889 [1.262588, 1.367164], 0/15|0.985494 [0.948839, 1.008040], 9/15|
|G2-4096-w1|6.399760 / 6.482131 / 5.817098|0.899395 [0.896378, 0.901383], 15/15|0.908902 [0.907416, 0.911300], 15/15|1.012204 [1.009595, 1.013514], 0/15|
|G2-4096-w16|0.525116 / 0.531310 / 0.653780|1.237275 [1.206182, 1.247539], 0/15|1.245022 [1.234303, 1.258503], 0/15|1.009147 [0.999240, 1.020925], 5/15|
|G2-4096-w32|0.458626 / 0.458371 / 0.644389|1.399782 [1.363759, 1.416131], 0/15|1.409045 [1.377753, 1.438292], 0/15|1.005624 [0.990131, 1.022247], 6/15|


Fifteen rotated/reversed blocks are fixed before capture. Each curve's equal-log two-size 32 worker aggregate requires upper rank 12/15<1 and>=10 wins against both controls. Every context ON/control upper must be<=1.03; every OFF/original interval must fit factor 1.03. Rank 4–12/15 gives nominal 96.484375% marginal coverage under independent exchangeable blocks, with no familywise guarantee. There is no gain floor, exclusion, extension, favorable subset, threshold rescue, parameter/layout sweep or unchanged reroll.

The **38 failed checks** comprise two calibrations, 16 nonregressions, 16 resource comparisons and four aggregate gains. Failed checks: `["G1-2048-w16/ON/frozen/nonregression","G1-2048-w16/ON/OFF/nonregression","G1-2048-w16/requested_heap/ON/frozen","G1-2048-w16/requested_heap/ON/OFF","G1-2048-w32/OFF/frozen/calibration","G1-2048-w32/ON/frozen/nonregression","G1-2048-w32/ON/OFF/nonregression","G1-2048-w32/requested_heap/ON/frozen","G1-2048-w32/requested_heap/ON/OFF","G1-4096-w16/ON/frozen/nonregression","G1-4096-w16/ON/OFF/nonregression","G1-4096-w16/requested_heap/ON/frozen","G1-4096-w16/requested_heap/ON/OFF","G1-4096-w32/ON/frozen/nonregression","G1-4096-w32/ON/OFF/nonregression","G1-4096-w32/requested_heap/ON/frozen","G1-4096-w32/requested_heap/ON/OFF","G2-2048-w16/ON/frozen/nonregression","G2-2048-w16/ON/OFF/nonregression","G2-2048-w16/requested_heap/ON/frozen","G2-2048-w16/requested_heap/ON/OFF","G2-2048-w32/OFF/frozen/calibration","G2-2048-w32/ON/frozen/nonregression","G2-2048-w32/ON/OFF/nonregression","G2-2048-w32/requested_heap/ON/frozen","G2-2048-w32/requested_heap/ON/OFF","G2-4096-w16/ON/frozen/nonregression","G2-4096-w16/ON/OFF/nonregression","G2-4096-w16/requested_heap/ON/frozen","G2-4096-w16/requested_heap/ON/OFF","G2-4096-w32/ON/frozen/nonregression","G2-4096-w32/ON/OFF/nonregression","G2-4096-w32/requested_heap/ON/frozen","G2-4096-w32/requested_heap/ON/OFF","G1-32aggregate/ON/frozen/gain","G1-32aggregate/ON/OFF/gain","G2-32aggregate/ON/frozen/gain","G2-32aggregate/ON/OFF/gain"]`.

|Context|Original / OFF / ON CPU ms per wave|Original / OFF / ON median CPU-ms/wall-ms|
|---|---:|---:|
|G1-2048-w1|3.262188 / 3.302625 / 2.952594|1.000 / 1.000 / 1.000|
|G1-2048-w16|4.028875 / 4.043906 / 4.022500|15.873 / 15.881 / 13.189|
|G1-2048-w32|6.629594 / 6.714625 / 7.641625|31.819 / 31.823 / 27.513|
|G1-4096-w1|6.185594 / 6.270281 / 5.541500|1.000 / 1.000 / 1.000|
|G1-4096-w16|7.470875 / 7.545562 / 6.859469|15.777 / 15.770 / 12.418|
|G1-4096-w32|11.950375 / 12.049250 / 12.135406|31.655 / 31.595 / 24.675|
|G2-2048-w1|3.207938 / 3.252969 / 2.904437|1.000 / 1.000 / 1.000|
|G2-2048-w16|4.218250 / 4.247094 / 3.888437|15.099 / 15.101 / 11.181|
|G2-2048-w32|7.679875 / 7.733063 / 7.370219|30.444 / 30.643 / 21.712|
|G2-4096-w1|6.399781 / 6.482156 / 5.817156|1.000 / 1.000 / 1.000|
|G2-4096-w16|7.809437 / 7.911562 / 7.043344|14.898 / 14.926 / 10.760|
|G2-4096-w32|13.708813 / 13.654344 / 12.819094|29.857 / 29.832 / 19.806|


Lower aggregate work does not imply lower single-proof latency. In G2/4096/32, median CPU service decreases from 13.654 to 12.819 ms per wave, while wall time increases from 0.458 to 0.644 ms. The median per-process CPU-ms/wall-ms ratio drops from about 29.8 to 19.8 logical-thread equivalents. This is average process CPU concurrency including scheduling, not physical-core utilization. A barrier and joint finishing alter the critical path and available parallelism. These observations are consistent with that tradeoff; they do not isolate an exclusive cache, scheduler or stall cause. G1/4096/32 CPU service is nearly unchanged despite its much worse wall time.

After all latency/profiles, 36 separate resource processes cover all 12 contexts and three arms with three prime passes plus four observed waves. They use separately linked requested-heap instrumentation compiled out of latency binaries. It tracks requested Rust global-allocator bytes from startup, resets the high-water mark after construction/primes, and reports the absolute peak and increment above that baseline. It excludes allocator metadata/internal realloc overlap, non-Rust/libc allocations, stacks and mapped memory. It is neither RSS nor minimum RAM. Times from these instrumented runs do not enter latency statistics; their allocation counts are perturbed resource diagnostics.

|Context|Original / OFF / ON absolute requested-heap peak MiB|Original / OFF / ON incremental peak KiB|ON/OFF absolute ratio|Original / OFF / ON allocation calls, four waves|
|---|---:|---:|---:|---:|
|G1-2048-w1|4.0242 / 4.0242 / 4.0511|71.492 / 71.492 / 99.055|1.006688|1432 / 1432 / 2356|
|G1-2048-w16|5.0151 / 5.0163 / 5.5220|961.805 / 963.055 / 1480.930|1.100818|5464 / 5464 / 6388|
|G1-2048-w32|6.0541 / 6.0541 / 6.5909|1892.555 / 1892.555 / 2442.266|1.088671|5464 / 5464 / 6388|
|G1-4096-w1|7.5872 / 7.5872 / 7.6638|72.156 / 72.156 / 150.594|1.010096|1364 / 1364 / 2324|
|G1-4096-w16|8.6013 / 8.5988 / 9.7714|986.219 / 983.719 / 2184.469|1.136368|10324 / 10324 / 11284|
|G1-4096-w32|9.6391 / 9.6391 / 10.7157|1915.719 / 1915.719 / 3018.219|1.111697|10324 / 10324 / 11284|
|G2-2048-w1|3.5436 / 3.5436 / 3.5879|92.258 / 92.258 / 137.633|1.012504|840 / 840 / 1236|
|G2-2048-w16|4.9074 / 4.9074 / 5.3561|1364.445 / 1364.445 / 1823.883|1.091426|3216 / 3216 / 3348|
|G2-2048-w32|6.2906 / 6.2445 / 6.6561|2647.633 / 2600.336 / 3021.883|1.065925|3216 / 3216 / 3348|
|G2-4096-w1|6.9836 / 6.9836 / 7.0804|94.758 / 94.758 / 193.945|1.013870|840 / 840 / 1280|
|G2-4096-w16|8.3624 / 8.3693 / 9.3985|1382.297 / 1389.445 / 2443.320|1.122969|6384 / 6384 / 6208|
|G2-4096-w32|9.7963 / 9.7520 / 10.6985|2717.445 / 2672.109 / 3641.320|1.097056|6384 / 6384 / 6208|


Threaded ON/OFF absolute requested-heap peaks increase about 6.59–13.64%, failing every threaded 5%comparison. For G1/4096/32, the absolute counter rises from 10,107,314 to 11,236,273bytes; incremental workspace rise is 1,961,696 to 3,090,656 bytes. For G2/4096/32 it rises from 10,225,762 to 11,218,233bytes, with increments 2,736,240 to 3,728,712 bytes. Input and benchmark-output storage are included in those absolute values, so they are not full-prover measurements or standalone algorithm bounds.

All 36 resource runs return to their initial requested-live level plus exactly the two retained benchmark output vectors. Those enum point records occupy 200 bytes each in this driver; this does not describe production point storage. All temporary algorithm work is released. A release check does not make the larger transient peak acceptable. Recorded rusage maximum-RSS counters remain diagnostic because prior page-table audits found undercounts on this host; no fresh page-table, swap or pressure result is claimed.

All 540 latency processes finish before six grouped four-event counter panels and six separate fixed 16M-user-cycle profiles. Counter panels run 256 waves and profiles 1024 after FIFO-excluded prime passes. No owned build/helper/hash/archive/analyzer/second workload overlaps capture. Ownership changes follow recording and preserve hashes. All events run 100%.

|Counter panel|Cycles per wave, million|Instructions per wave, million|Generic cache misses per wave|Branch misses per wave|Profile samples|
|---|---:|---:|---:|---:|---:|
|stat-G1-4096-w32-OFF|55.425|93.724|47034.797|58206.383|3545|
|stat-G1-4096-w32-ON|53.212|85.737|72898.543|50583.211|3370|
|stat-G1-4096-w32-frozen|55.440|93.677|47466.227|59646.371|3518|
|stat-G2-4096-w32-OFF|61.050|97.950|63369.125|47562.312|3920|
|stat-G2-4096-w32-ON|53.008|89.864|75888.270|36869.812|3382|
|stat-G2-4096-w32-frozen|61.010|97.969|65058.703|45988.582|3915|


At 4096/32, ON retires about 8.52% fewer G1 instructions and 8.26% fewer G2 instructions than OFF. Cycles decrease about 3.99%/13.17% in those separate panels, while latency regresses in the paired series. These are single descriptive counter observations, not paired uncertainty bounds or removable wall-time shares. Generic cache misses increase but do not identify a cache level or prove bandwidth limitation.

The six profiles contain **21,650 samples**, zero lost records, at least 3370 samples per profile and unknown share at most 1.900%. Process, registered worker TID, allowed CPU, mmap/FORK and fixed-period evidence is checked. Flat symbols include inlined work; they do not isolate stall cycles or critical-path fractions. Linked-code artifacts cover the two baseline wrappers and named original reducer specializations; joint work is also inlined in Rayon/caller code, so the seven selected disassemblies are not a complete candidate code census. Static sites/RSP operands are not dynamic spill counts.

Independent review passes all 654 commands / 624 callers, raw output and command bindings, statistical/resource comparisons, symbol bins and gates. The first source audit stopped after source recovery at an incorrect field-source count; its preserved v2 correction changes only that metadata expectation and printed label. Native source/tests/builds/capture all passed on their first executions. [Planning notes](artifacts/planning-notes.md). No measured workload or successful native test is repeated.

No SDK request, proof export or Gnark verification occurs; the distinct-proof ledger remains **28,349**. Accepted source 176 / SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remains unchanged. The following accepted proof timings and supplied M5 CPU/Metal references are inherited, not measurements of this prototype. The accepted physical16 calibration engineering exception and false original full-performance gate remain documented.

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


The all-eight CPU 10× objective remains **active and unmet, 0/8**. Primary is 7.928660 ms against a 3.7 ms target. Historical merge54 is 161.883899 ms without a supplied M5 reference. No new AWS/Aeglos, mobile/ARM/TEE, huge 250/4999, concurrent-proof, pressure/swap or minimum-RAM result follows; no AWS resource was created.

This implementation is closed. Its solo savings support a reduction in serial work, while the server regression and increased transient heap reject this shared-frontier organization. The preceding compact weighted-group caller result remains a separate promising but unqualified result; this failure changes none of its observations. Another candidate needs to preserve useful parallelism while reducing work and memory, with a complete verified proof measurement before any adoption or new M5 speedup claim.

[Protocol](artifacts/design.md), [full results](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [independent review](artifacts/review.json), [work/resource interpretation](artifacts/interpretation.json), [profile symbols](artifacts/profile-symbols.json), [linked code](artifacts/codegen.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [publication audit](artifacts/publication.json). Archives: [source snapshots](artifacts/source.tar.gz), [Rust workflows](artifacts/workflow.tar.gz), [command receipts](artifacts/receipts.tar.gz), [counter panels](artifacts/counters.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Executables/libraries/private inputs/raw perf.data remain local with hashes. Root: `/home/ubuntu/src/zopro-perf-work/weighted-frontier176`.

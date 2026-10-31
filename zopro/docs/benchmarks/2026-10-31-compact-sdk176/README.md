# Compact indexed-tree reduction in complete SDK proofs

Complete-proof experiment gate: **false**. The candidate is **not adopted**; accepted source 176 / SDK `6d83…` is unchanged. A passing experiment would still require final integration, compatibility and resource qualification.

Original eight-shape equal-log ON/accepted ratio: **1.001113 [0.998473, 1.002597], 5/15**.
Original eight-shape equal-log ON/OFF ratio: **0.998461 [0.995289, 1.000118], 11/15**.

Failed checks: `["8x2-w32/OFF/accepted","8x2-w32/ON/accepted","1x2-w16/OFF/accepted","original8/ON/accepted/gain","original8/ON/OFF/gain","12x2-w32/sampled_RSS/ON/accepted","24x2-w32/sampled_RSS/ON/accepted"]`. Every observation and the original thresholds are retained.

Primary medians are 7.905321 ms accepted, 7.894729 ms OFF and 7.873751 ms ON: observed differences of 31.570 µs and 20.978 µs respectively. Paired primary ON/OFF is 0.998874 [0.995237, 1.000804], 10/15 wins; ON/accepted is 0.998669 [0.990674, 1.003005], 10/15. Both bounds include no improvement. This is no verified complete-proof breakthrough. The seven failed checks include both aggregate gains, two control calibrations, one ON/accepted non-regression and two sampled-memory comparisons.

The candidate keeps affine coordinates in one compact native SIMD buffer across an unweighted binary reduction tree. It computes norms and prefixes, stores inverse factors in a reverse pass, then compacts coordinates forward after reading their inputs. Arbitrary leaf lengths use masks and odd-point carries. The final small tree uses the existing complete reducer. This avoids repeated coordinate conversions and a second point buffer; it adds packing, masks and factor passes, whose costs are included. Field arithmetic and representation are unchanged.

This experiment integrates the previously tested component into the existing indexed-bucket leaves before the original point vector is allocated. An indexed getter avoids copying the host point array. Admission requires at least 16 points, the existing G1/G2 tile maxima of 2,048/1,024, and a checked requested vector layout contained within the original charged grouping forecast. Denial or exceptional points release the private work before the original fallback allocates. There is no lookup table, transformed key copy, duplicate solver plan, new pool or arbitrary RAM cap. The full memory envelope, including allocator and stack overhead, is not proved by a vector-payload bound.

The accepted SDK is the retained production executable. Fresh OFF and ON share one separately linked executable with actual optimized native/fat-LTO/one-codegen-unit final flags. The environment selector initializes once. Per-thread route counters update inside the timed work; a wrapper collects and prints them after the original SDK main returns. The original SDK timing body is byte-exact apart from its function name. Counter totals include loading/primes where applicable and are not asserted as per-request counts.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Nine circuits use 32 workers on CPUs 0–31; primary also uses 16 workers on CPUs 0–15. Fifteen fixed rotated/reversed blocks compare accepted/OFF/ON, with three primes then 151 primary or 31 other fresh requests per process. Timing includes witness solving, randomized proving, SDK verification and serialization; key loading is excluded. Latency is the median of the fifteen process medians. CPU time is summed user/system service per request.

The original-eight equal-log gain requires an upper ratio below one and at least 10/15 wins against both controls. Every context upper ON/control ratio must be at most 1.03; every OFF/accepted interval must fit factor 1.03. Rank-4–12 bounds have nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, excluded result, extended sample set, favorable subset, threshold rescue or unchanged reroll is used. The earlier component's failed calibration/counter-memory gate remains failed.

|Context|Accepted / OFF / ON ms|ON/accepted ratio [bounds], wins|ON/OFF ratio [bounds], wins|OFF/accepted ratio [bounds], wins|
|---|---:|---|---|---|
|1x2-w32|7.905321 / 7.894729 / 7.873751|0.998669 [0.990674, 1.003005], 10/15|0.998874 [0.995237, 1.000804], 10/15|0.997746 [0.991262, 1.005497], 8/15|
|2x2-w32|11.179024 / 11.209202 / 11.199501|1.001175 [0.996832, 1.009293], 7/15|0.997771 [0.989938, 1.010768], 8/15|1.005006 [0.991004, 1.008808], 5/15|
|4x2-w32|16.099784 / 16.123527 / 16.102281|0.998854 [0.983077, 1.006910], 9/15|0.996596 [0.977435, 1.015903], 8/15|0.991817 [0.988153, 1.005372], 9/15|
|8x2-w32|23.950579 / 24.564252 / 24.348211|1.015044 [0.997306, 1.033423], 4/15|0.997142 [0.975551, 1.008642], 8/15|1.022702 [1.010074, 1.032803], 1/15|
|12x2-w32|33.721187 / 33.894196 / 33.952307|1.002575 [0.997874, 1.013199], 5/15|1.004404 [0.992574, 1.008884], 6/15|1.002464 [0.996564, 1.010076], 5/15|
|16x2-w32|40.094175 / 40.165328 / 40.190876|1.003155 [0.989200, 1.009786], 6/15|0.999527 [0.979756, 1.003691], 8/15|1.006961 [0.993776, 1.018691], 4/15|
|24x2-w32|65.863345 / 66.047373 / 66.070529|0.997863 [0.992559, 1.006497], 8/15|0.997787 [0.989606, 1.005200], 8/15|0.997819 [0.991652, 1.014344], 8/15|
|merge-24x1-w32|65.055239 / 64.818190 / 64.567229|0.992322 [0.984702, 0.996755], 13/15|0.997959 [0.987926, 1.004628], 9/15|0.995132 [0.992238, 1.006183], 10/15|
|merge-54x1-w32|162.279977 / 161.924781 / 161.836911|0.992660 [0.988054, 0.997975], 12/15|0.998120 [0.988042, 1.002928], 8/15|0.998260 [0.991490, 1.009058], 9/15|
|1x2-w16|9.689143 / 9.367980 / 9.359010|0.962686 [0.929365, 1.008126], 11/15|0.997807 [0.986902, 1.015795], 9/15|0.967978 [0.917744, 1.016734], 10/15|

|Context|Accepted / OFF / ON CPU ms/request|Accepted / OFF / ON recorded maximum-RSS counter MiB|
|---|---:|---:|
|1x2-w32|179.418 / 179.000 / 176.054|132.50 / 131.50 / 135.00|
|2x2-w32|258.962 / 259.305 / 257.018|275.75 / 276.00 / 275.50|
|4x2-w32|420.978 / 425.489 / 424.193|313.25 / 322.25 / 315.00|
|8x2-w32|666.341 / 678.566 / 673.395|510.25 / 506.75 / 519.50|
|12x2-w32|937.932 / 939.266 / 939.795|707.75 / 710.25 / 736.25|
|16x2-w32|1125.969 / 1128.768 / 1127.082|833.50 / 811.75 / 824.50|
|24x2-w32|1894.470 / 1894.823 / 1893.307|1304.11 / 1299.72 / 1284.05|
|merge-24x1-w32|1801.884 / 1804.714 / 1797.131|1177.58 / 1163.94 / 1209.62|
|merge-54x1-w32|4452.345 / 4442.488 / 4427.286|1871.86 / 1804.80 / 1856.09|
|1x2-w16|115.543 / 116.915 / 114.706|88.50 / 90.50 / 88.25|

Maximum-RSS counters above are diagnostic, not accurate resident peaks or minimum RAM. The previous audit found these counters below precise page-table snapshots on this host; the exact kernel-internal cause was not isolated. A separately declared resource matrix below uses the exact SDK binaries, all ten contexts and three arms, with 151 primary/2x2 or 31 other requests plus three primes. Its timings are excluded from latency statistics. Samples come from `/proc/PID/smaps_rollup` every 50 ms after FIFO enable + 10 ms and before disable. Serialized phase/frame timestamps permit independent selection checks. These sampled maxima are lower bounds on lifetime peaks. The new prospective resource gate requires each ON/control sampled maximum ratio at most 1.05, together with native layout-containment checks; it does not retroactively change older gates. One process per resource cell does not establish confidence intervals, minimum RAM, concurrent throughput or behavior under memory pressure.

|Context|Accepted / OFF / ON sampled RSS max MiB|ON/accepted / ON/OFF ratio|Accepted / OFF / ON snapshots|
|---|---:|---:|---:|
|1x2-w32|135.582 / 139.910 / 136.461|1.006482 / 0.975347|23 / 23 / 23|
|2x2-w32|318.656 / 323.500 / 323.492|1.015176 / 0.999976|31 / 31 / 31|
|4x2-w32|314.820 / 322.750 / 300.012|0.952962 / 0.929548|9 / 9 / 9|
|8x2-w32|515.797 / 491.590 / 486.750|0.943685 / 0.990155|12 / 13 / 13|
|12x2-w32|676.984 / 712.855 / 710.934|1.050148 / 0.997304|17 / 17 / 17|
|16x2-w32|807.090 / 840.168 / 812.176|1.006302 / 0.966683|20 / 20 / 20|
|24x2-w32|1158.148 / 1250.762 / 1236.676|1.067804 / 0.988738|31 / 30 / 31|
|merge-24x1-w32|1119.109 / 1135.934 / 1123.789|1.004182 / 0.989309|31 / 31 / 31|
|merge-54x1-w32|1721.289 / 1718.703 / 1774.719|1.031041 / 1.032592|63 / 62 / 63|
|1x2-w16|90.648 / 88.902 / 90.203|0.995087 / 1.014632|35 / 31 / 27|

Recorded-counter warnings retained without gating: `[]`.

Route snapshots from the first latency block follow. Each row gives `[eligible, entered, fallback, largest leaf, largest requested vector bytes]` separately for G1 and G2. OFF snapshots are all zero. ON snapshots satisfy eligible = entered + fallback; primary enters the new path. Other shapes retain their actual routes, including zero where no indexed leaf qualifies.

|Context|G1 counts|G2 counts|
|---|---|---|
|1x2-w32|`[1540,1540,0,2048,213288]`|`[68684,68684,0,164,28840]`|
|2x2-w32|`[1564,1564,0,2048,213288]`|`[15198,15198,0,163,28840]`|
|4x2-w32|`[1904,1904,0,2048,213288]`|`[374,374,0,1024,172328]`|
|8x2-w32|`[3196,3196,0,2048,213288]`|`[442,442,0,1024,172328]`|
|12x2-w32|`[4250,4250,0,2048,213288]`|`[544,544,0,1024,172328]`|
|16x2-w32|`[5372,5372,0,2048,213288]`|`[612,612,0,1024,172328]`|
|24x2-w32|`[7548,7548,0,2048,213288]`|`[748,748,0,1024,172328]`|
|merge-24x1-w32|`[7344,7344,0,2048,213288]`|`[714,714,0,1024,172328]`|
|merge-54x1-w32|`[15164,15164,0,2048,213288]`|`[1292,1292,0,1024,172328]`|
|1x2-w16|`[1540,1540,0,2048,213288]`|`[68684,68684,0,164,28840]`|

Thirty preflight proofs passed independent Gnark verification before latency. All 450 latency processes preceded nine grouped CPU-counter panels and nine separate fixed-16M-user-cycle profiles on primary32, 4x2 and merge24, across all arms. Stat panels use 31 requests; profiles use 251 primary or 101 other requests after FIFO-excluded primes. No owned build, helper, hashing, archive, analyzer, independent verifier or second workload overlapped primary latency or profiles. The explicit memory observer runs only in the separate resource matrix. Raw faults/context switches remain in the receipts, without attributing a causal wall-time share.

|Counter panel|Cycles M/request|Instructions M/request|Cache misses M/request|Branch misses M/request|Profile samples|Unknown period share|
|---|---:|---:|---:|---:|---:|---:|
|stat-1x2-w32-OFF|808.310|1388.971|2.982165|0.912966|12387|0.01526|
|stat-1x2-w32-ON|796.799|1368.546|2.842789|0.890067|12010|0.01790|
|stat-1x2-w32-accepted|819.166|1390.313|2.958754|0.911402|12235|0.01577|
|stat-4x2-w32-OFF|1908.815|2367.687|7.176361|1.621716|11391|0.02651|
|stat-4x2-w32-ON|1885.896|2358.912|7.007398|1.631160|11453|0.02750|
|stat-4x2-w32-accepted|1907.838|2364.433|7.095976|1.633954|11436|0.02842|
|stat-merge-24x1-w32-OFF|8168.269|8726.380|35.683664|3.607040|52105|0.00677|
|stat-merge-24x1-w32-ON|8275.939|8710.847|35.075423|3.579077|52247|0.00593|
|stat-merge-24x1-w32-accepted|8310.014|8735.151|33.764108|3.645292|52553|0.00668|

Diagnostics qualified: **true**; 227817 sampled events. Independent review checks executable PID, FORK/task ownership, allowed CPUs, fixed period, mappings and zero-loss receipts. Qualification requires at least 99.9% counter running, 1,000 samples per profile, at most 5% unknown period and zero loss. Flat symbol samples include inlined work; they do not establish exclusive critical-path time, dynamic spills, local CPI, removable latency or absolute period coverage.

Primary G2 indexed leaves top out at 164 points, whereas the previous component matrix measured 255–1,024 points. All first-block eligible calls enter without fallback, so failure to reach the path does not explain the absent SDK gain. The primary ON profile attributes about 2.37% of sampled period to symbols recognized as compact-tree context; 4x2 and merge24 show about 0.23% and 0.10%. Inlining and flat attribution prevent treating these shares as exact removable-time bounds. The source, route geometry and complete-proof results support revisiting wider MSM dataflow rather than extrapolating the component gain or rerunning this unchanged recipe. Control calibration failures remain separate from any proposed causal explanation.

All 131 native core tests pass, including exhaustive n16..existing-tile checked forecast containment for both curves, indexed permutations and arbitrary lengths, unchanged inputs, exceptional cancellation/fallback, overflow/bounds, reuse and threaded pools. Both raw prover tests and all six cubic prover tests pass. Field/SIMD helpers are byte-identical to the earlier 37-test qualification; that suite was not rerun. The existing generator consistency check passes. Source has 179 files: 171 accepted originals remain exact, reversing five parent edits recovers all 176 originals, and three child modules are added. The compact large-level arithmetic body and original SDK main body recover exactly. Portable, phone/ARM, pressure/swap and final compatibility qualification remain outside this experiment.

The primary capture performs **27,912 fresh internally verified SDK requests** and exports **498 distinct 164-byte proofs**, each independently verified by Gnark with proof/public/image/VK bindings and freshness against 27,851 prior proof hashes. The ledger becomes **28,349**. The resource matrix performs another **2,100** internally verified fresh SDK requests without adding exports, for **30,012 total requests**. Independent review recomputes all paired statistics, aggregate gains, calibration, counter warnings and sampled-RSS gates, verifies all memory frames and every profile symbol bin, and checks 1,048 structured command receipts plus the direct 131-core-test log. No capture, proof or verifier is rerun by review.

Fresh candidate comparisons against the supplied historical M5 references follow. ON measurements are experimental and do not imply adoption.

|Shape|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|Target met|
|---|---:|---:|---:|---:|---:|---:|---|
|1x2|7.873751|37|4.699×|24|3.048×|3.7|false|
|2x2|11.199501|59|5.268×|30|2.679×|5.9|false|
|4x2|16.102281|109|6.769×|48|2.981×|10.9|false|
|8x2|24.348211|189|7.762×|74|3.039×|18.9|false|
|12x2|33.952307|284|8.365×|101|2.975×|28.4|false|
|16x2|40.190876|335|8.335×|124|3.085×|33.5|false|
|24x2|66.070529|528|7.991×|176|2.664×|52.8|false|
|merge-24x1|64.567229|507|7.852×|175|2.710×|50.7|false|

Candidate target count: **0/8**. The all-eight verified CPU 10× objective remains active and unproven. Merge54 has no supplied M5 reference. Historical AWS/Aeglos measurements predate accepted source176 and are not refreshed here. No AWS resource was created. No new AWS, TEE, huge250/4999, mobile, minimum-RAM or concurrent-proof result follows.

[Protocol](artifacts/design.md), [full analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source review](artifacts/source-review.json), [identities](artifacts/identities.json), [proof inventory](artifacts/proof-inventory.json), [profile symbols](artifacts/profile-symbols.json), [publication review](artifacts/publication.json). Archives: [private source snapshot](artifacts/source.tar.gz), [Rust workflows and protocol](artifacts/workflow.tar.gz), [all command receipts](artifacts/receipts.tar.gz), [proof/public exports](artifacts/proofs.tar.gz), [counter panels](artifacts/counters.tar.gz), [resource samples](artifacts/memory-audit.tar.gz). [Checksums](SHA256SUMS). Keys, witnesses, input scalars, executables, libraries and raw perf.data stay local with hashes. Local root: `/home/ubuntu/src/zopro-perf-work/compact-sdk176`.

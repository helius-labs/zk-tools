# Compact SIMD tree with forward compaction

The complete-tree callers improve while retaining less scratch: the four-size 32-worker aggregate is **5.06% faster for G1 and 11.98% faster for G2 against fresh OFF**, with all 15 aggregate blocks improving against both controls. All individual latency bounds are below parity. The full prospective gate remains **false** because four control-calibration checks and four recorded RSS checks fail. No production change is adopted and no full-proof speedup is established.

The [previous two-buffer experiment](../2026-10-31-packed-tree176/README.md) established a conversion-saving opportunity with a G2 memory cost. This distinct implementation keeps one compact coordinate buffer, prepares every inverse factor before writing coordinates, then compacts forward. Output group `j` is written after reading input groups `2j` and `2j+1`; it precedes all future unread source groups. Existing four-word compact storage preserves the numerical Montgomery values without canonicalization or a field-representation change. Existing affine arithmetic and one shared inversion per level remain.

Unlike the previous power-of-two prototype, this implementation handles arbitrary lengths above eight. Partial SIMD groups contribute multiplicative identities in inactive prefix lanes. Valid zero denominators request the original complete reducer; inactive lanes are excluded from that guard. A raw lane copy carries an unpaired point into its exact next-level position. Initial identities and exceptional intermediate pairs use the original complete path from the immutable source. Packed buffers are released before constructing that fallback; the next packed call releases the previous fallback's buffers. The final one to eight points use the original complete reducer.

There is no lookup table, cached transformed key, duplicate solver plan, new field arithmetic, assembly rewrite, fixed RAM cap or production dispatch change. New helpers perform permutations, raw compact lane copies and masked selection of the existing field identity. The additional factor pass, compact/expand operations, padding and carry handling are real costs included in timing.

The fixed protocol covers G1/G2, 255/256/1023/1024 points and 1/16/32 workers: 24 contexts, 15 rotated/reversed blocks and three arms. The separate original executable and candidate OFF/ON use verified release/native CPU/fat-LTO/one-codegen-unit final links. Each fixture has three eight-round primes, then 4,096 complete reductions, alternating two input slices with independently known generator sums. Initial packing, buffer operations, inversion, tail reduction and output are timed. The final output for both slices is checked after each prime and timed chain; intermediate iterations are not individually checked. All timed allocation counts are zero, capacities stay stable and measured ON calls have no fallback.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Workers use CPU 2, CPUs 0–15 or CPUs 0–31 with 512 KiB stacks. Fixtures are Rayon tasks, without per-round barriers or guaranteed fixture/core ownership. This measures complete tree-call throughput, not single-proof latency. Two private 178-file snapshots recover all 352 accepted original source files after removing five parent additions. Production source 176 remains unchanged.

The declared four-size 32-worker aggregate must have an upper ratio below one and at least 10/15 wins against both original controls, separately for each curve. Every individual ON/control upper bound must be at most 1.03, each OFF/original interval must fit factor 1.03, and each maximum recorded RSS ON/control ratio must be at most 1.05. Bounds use ranks 4–12 of 15, nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise coverage. No gain floor, excluded observation, favorable subset, extension, threshold rescue or unchanged reroll is used.

|Curve / 32-worker aggregate|ON/control ratio [bounds]|Wins|
|---|---:|---:|
|G1 / frozen|0.942221981 [0.938629881, 0.950312428]|15/15|
|G1 / OFF|0.949386249 [0.940821043, 0.955032907]|15/15|
|G2 / frozen|0.880478745 [0.876737591, 0.883143327]|15/15|
|G2 / OFF|0.880193350 [0.876313753, 0.880868100]|15/15|


|Curve / points / workers|Original / OFF / ON µs per tree wave|ON/OFF ratio [bounds], wins|ON/original ratio [bounds], wins|
|---|---:|---:|---:|
|G1-255-w1|11.756 / 11.764 / 11.087|0.942801 [0.931827, 0.954072], 15/15|0.940967 [0.932060, 0.953295], 15/15|
|G1-255-w16|12.981 / 12.936 / 12.159|0.940362 [0.936831, 0.941249], 15/15|0.937297 [0.932210, 0.938912], 15/15|
|G1-255-w32|18.260 / 18.144 / 16.822|0.922538 [0.915379, 0.937506], 15/15|0.919987 [0.909197, 0.937063], 15/15|
|G1-256-w1|11.208 / 11.248 / 11.070|0.985747 [0.977245, 0.992913], 15/15|0.984150 [0.979527, 0.992106], 15/15|
|G1-256-w16|12.383 / 12.384 / 12.145|0.981109 [0.977278, 0.984061], 15/15|0.976903 [0.976565, 0.980808], 15/15|
|G1-256-w32|17.275 / 17.192 / 16.747|0.981081 [0.961162, 0.999997], 12/15|0.972069 [0.956378, 0.996850], 13/15|
|G1-1023-w1|30.389 / 30.347 / 29.469|0.969563 [0.965383, 0.977027], 15/15|0.968126 [0.963195, 0.976041], 15/15|
|G1-1023-w16|34.319 / 34.220 / 32.928|0.963279 [0.957608, 0.966206], 15/15|0.958800 [0.957012, 0.961969], 15/15|
|G1-1023-w32|48.685 / 48.326 / 45.192|0.936625 [0.924966, 0.956925], 15/15|0.929956 [0.925766, 0.948364], 15/15|
|G1-1024-w1|29.920 / 29.884 / 29.523|0.989135 [0.978466, 0.996315], 15/15|0.984914 [0.977011, 0.994879], 15/15|
|G1-1024-w16|33.869 / 33.611 / 32.915|0.979501 [0.975757, 0.981203], 15/15|0.970617 [0.965644, 0.973722], 15/15|
|G1-1024-w32|47.940 / 47.874 / 45.104|0.942737 [0.938929, 0.956838], 15/15|0.943146 [0.939379, 0.949969], 15/15|
|G2-255-w1|19.986 / 20.756 / 17.423|0.841694 [0.833711, 0.845667], 15/15|0.869324 [0.865934, 0.878454], 15/15|
|G2-255-w16|22.721 / 23.630 / 19.756|0.836495 [0.831487, 0.841275], 15/15|0.867039 [0.865608, 0.875680], 15/15|
|G2-255-w32|32.984 / 32.789 / 28.388|0.866926 [0.855219, 0.871843], 15/15|0.862080 [0.857990, 0.865397], 15/15|
|G2-256-w1|18.705 / 19.126 / 17.444|0.916792 [0.906080, 0.920143], 15/15|0.932133 [0.924577, 0.940011], 15/15|
|G2-256-w16|21.243 / 21.760 / 19.812|0.912118 [0.904624, 0.913866], 15/15|0.933912 [0.927654, 0.938447], 15/15|
|G2-256-w32|31.174 / 31.306 / 28.558|0.912100 [0.904425, 0.922087], 15/15|0.916896 [0.901996, 0.929889], 15/15|
|G2-1023-w1|58.097 / 58.806 / 51.064|0.868710 [0.862509, 0.875813], 15/15|0.878619 [0.872192, 0.887206], 15/15|
|G2-1023-w16|66.725 / 67.757 / 58.519|0.864703 [0.858048, 0.869810], 15/15|0.877015 [0.872053, 0.881419], 15/15|
|G2-1023-w32|99.845 / 100.008 / 87.050|0.868520 [0.864339, 0.874533], 15/15|0.871576 [0.862651, 0.878212], 15/15|
|G2-1024-w1|56.793 / 57.141 / 51.139|0.894607 [0.887514, 0.902329], 15/15|0.900392 [0.893721, 0.909275], 15/15|
|G2-1024-w16|65.018 / 65.613 / 58.530|0.893113 [0.884286, 0.895036], 15/15|0.901309 [0.898511, 0.901574], 15/15|
|G2-1024-w32|99.570 / 99.910 / 87.037|0.872663 [0.868410, 0.875995], 15/15|0.874431 [0.869951, 0.882151], 15/15|


All 32-worker control calibrations pass. The four failures are limited to G2 at 255/256 points with one or sixteen workers:

|Context|OFF/original ratio [bounds]|
|---|---:|
|G2-255-w1|1.034336112 [1.028269700, 1.048652002]|
|G2-255-w16|1.039997230 [1.037264982, 1.045667490]|
|G2-256-w1|1.017027351 [1.014274683, 1.030404013]|
|G2-256-w16|1.027779458 [1.016808998, 1.035925241]|


The calibration failures and four original counter-based RSS failures remain; all latency gain and nonregression checks pass. Later memory diagnostics do not turn this original gate into a pass.

The source model reconciles every ordinary retained vector capacity, including every worker in all latency processes. The following capacities exclude inputs, inline/stack storage, allocator metadata and allocation size classes. They do not measure minimum RAM or a complete resource-admission envelope. Native tests separately check release across late fallback and normal reuse.

|Curve / points|Original / compact retained bytes per fixture|Change bytes|
|---|---:|---:|
|G1-255|41344 / 26656|-14688|
|G1-256|43008 / 26656|-16352|
|G1-1023|170368 / 106528|-63840|
|G1-1024|172032 / 106528|-65504|
|G2-255|49024 / 43040|-5984|
|G2-256|49792 / 43040|-6752|
|G2-1023|181120 / 172064|-9056|
|G2-1024|181888 / 172064|-9824|


At 1,024 points, G1 retains 106,528 rather than 172,032 bytes, and G2 retains 172,064 rather than 181,888 bytes. Relative to the previous two-buffer G2 prototype's 286,752 bytes, the new layout removes 114,688 bytes per fixture. Canonical coordinate new/get calls also decrease, but the model excludes the extra compact/expand, masks, carry insertion, factor pass and changed scalar-tail work. Those counts are not cycles or physical cache traffic.

Native qualification passes **37 field and 129 core tests**, including raw-word preservation for compact pair permutations, every source/destination carry lane and all 256 active masks in both fields/radices. Complete-tree tests cover 0/1/7/8/9/15/16/17/31/32/33/63/64/127/128/255/256/1023/1024/1025/2048/2049 points, identities, equal/opposite inputs, intermediate cancellation, immutable inputs, scratch reuse and 1/4/16/32-worker 512 KiB pools. A late-fallback test enters a normal packed level before encountering cancelling partial sums, checks the complete result, and verifies that the two buffer families are not retained together. The existing generator consistency check passes. Native suites/builds finish once without failure or rerun; no fresh portable, full-SDK, pressure or phone qualification is claimed.

All 1,080 latency processes precede six grouped PMU panels and six fixed-16M-user-cycle profiles. PMU running time is 100%. Independent review validates 11,559 samples, expected PID/TID/CPU membership, event configuration and zero lost records; every profile exceeds the 400-sample floor and has less than 5% unknown symbols. The smallest profile has 584 samples. Static function sizes and RSP operands do not measure dynamic spills; sampled shares do not establish critical-path cost or absolute cycle coverage.

|32-worker panel|OFF / ON instructions per wave|OFF / ON cycles per wave|OFF / ON cache-miss events per wave|
|---|---:|---:|---:|
|G1-255-w32|5323765 / 4670410|2599522 / 2385975|43.59 / 32.10|
|G2-1023-w32|24221162 / 20706383|13232402 / 11918923|3280.87 / 2664.03|


The two panels reduce retired instructions by about 12.3% and 14.5%, respectively. Their cache-miss counts also fall, but neither the counters nor the profiles isolate a cache level, bandwidth limit or causal wall-time share. All CPU service, minor faults, context switches and recorded RSS values are retained in the caller receipts.

The memory checks exposed an accounting limitation. The retained allocations alone made some small process high-water readings implausibly low. A separate, resource-only audit reused the exact binaries for 32-worker, 1,024-point callers, sampled `/proc/PID/smaps_rollup` and status within the existing FIFO-controlled loop interval, and excluded all of those elapsed times from latency decisions. It also sampled the previous two-buffer G2 binary. No timing observation was replaced or added to the latency aggregates.

|Resource-only case|Sampled page-table RSS max KiB|Reported loop-end high-water KiB|Snapshots|
|---|---:|---:|---:|
|G1-1024-w32-frozen|11068|4864|16|
|G1-1024-w32-OFF|11220|4864|15|
|G1-1024-w32-ON|9208|4352|15|
|G2-1024-w32-frozen|13852|7424|32|
|G2-1024-w32-OFF|13860|7424|33|
|G2-1024-w32-ON|13688|7424|29|
|G2-1024-w32-old-two-buffer|17268|14592|30|


In all seven audit processes, a sampled page-table RSS exceeds the loop-end `getrusage` high-water counter; Hugetlb accounting is zero. For example, the fresh G1 control reports 4,864 KiB while a selected snapshot reads 11,220 KiB. This is direct evidence that the counter cannot be treated as an exact resident-memory peak in these runs. The Linux documentation [describes asynchronous RSS accounting and page-table-based snapshots](https://www.kernel.org/doc/html/v6.12/filesystems/proc.html). The exact internal reason for this counter discrepancy was not isolated here.

The resource audit has one process per case and is not an RSS regression confidence study. A sampled maximum is a lower bound on a lifetime peak, not a minimum-RAM result. Snapshot filtering is implemented in the archived helper; the FIFO boundary timestamps were not separately serialized. The audit therefore supports an accounting limitation and a bounded working-set observation, not pressure qualification or retroactive passing of the four failed RSS guards. Future SDK resource qualification should retain the historical counter and add page-table observations with explicit sampling scope.

All **1,185 primary workflow commands** pass: 72 preflights, 1,080 latency processes, six stat panels, six profiles, three planned file-access steps and eighteen decoders. There are 1,164 primary caller processes plus seven separate resource-audit processes. Before/after hashes match for the six perf files whose ownership is changed for decoding. All 390 source/compiler/executable/log pins bracket primary capture. The independent reader recomputes every paired gate and aggregate and verifies source recovery, command exits, stream hashes, counters and profiles. The source model verifies every worker's retained capacity. Resource-audit data are checked separately against the exact binary hashes and raw snapshot fields. No helper, build, hash, archive or second owned workload overlaps primary latency/profiling. Executables, libraries, raw perf.data, private keys and witness inputs remain local.

The previously accepted local single-proof comparison follows. These are inherited accepted latencies and supplied M5 CPU/Metal references, not results for this component candidate. The original CPU 10× goal remains unmet: **0/8 shapes**. There are no new SDK/Gnark proofs, and the canonical independent proof ledger remains 27,851. No fresh AWS, Aeglos, TEE, phone, cold-start, huge-circuit or concurrent-proof result is added.

|Shape|Accepted CPU ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|10× CPU target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667×|24|3.027×|3.7|
|2x2|11.151932|59|5.291×|30|2.690×|5.9|
|4x2|16.086386|109|6.776×|48|2.984×|10.9|
|8x2|24.179453|189|7.817×|74|3.060×|18.9|
|12x2|33.884455|284|8.381×|101|2.981×|28.4|
|16x2|40.077381|335|8.359×|124|3.094×|33.5|
|24x2|65.850777|528|8.018×|176|2.673×|52.8|
|merge-24x1|65.060937|507|7.793×|175|2.690×|50.7|


Accepted primary latency remains 7.928660 ms. Its historical recorded `ru_maxrss` value is 132.75 MiB; the accounting finding above is a reason to avoid interpreting it as an exact resident peak or minimum RAM. No new SDK memory measurement occurred. Merge54 remains 161.883899 ms, without a supplied M5 reference. [Accepted qualification](../2026-10-30-g2-vector-sign-adoption176/README.md), including its physical-16 calibration exception, remains unchanged.

The measured caller gain and lower bounded scratch justify a distinct complete-query/SDK translation test. The current component gate stays false. Integration must use actual indexed/grouped/weighted geometry, charge optional workspace before allocation, preserve the original fallback and validate full fresh randomized proofs. A tree-call gain alone does not establish how much single-proof latency it can remove.

[Protocol](artifacts/declaration.json), [latency/CPU/RSS rows and raw caller receipts](artifacts/latency-analysis.json), [independent review and profile bins](artifacts/review.json), [retained-capacity model](artifacts/work-model.json), [workflow](artifacts/plan.json), [source/compiler pins](artifacts/pins.json), [original source pins](artifacts/original-pins.json), [source snapshots](artifacts/source.tar.gz), [Rust driver/helpers](artifacts/workflow.tar.gz), [primary receipts/build logs](artifacts/receipts.tar.gz), [counter files](artifacts/counters.tar.gz), [resource-audit data](artifacts/memory-audit.tar.gz), [publication audit](artifacts/publication.json), [host](artifacts/host.txt).

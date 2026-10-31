# Compact weighted-group reduction in complete SDK proofs

The candidate is not adopted. It improves the original-eight equal-log aggregate by about 0.42% against the same executable with the path disabled, but does not demonstrate an aggregate gain against the accepted executable. All twenty sampled-RSS comparisons pass. The complete gate fails three checks: physical-16 control calibration, physical-16 ON/accepted non-regression, and original-eight ON/accepted gain. The production implementation and accepted SDK remain unchanged.

Primary32 medians are 7.906865 ms accepted, 7.916479 ms OFF and 7.883521 ms ON. The differences of process-median medians are 23.344 µs versus accepted and 32.958 µs versus OFF. Paired ON/OFF is 0.995152 [0.989854, 1.000644], 11/15 wins; ON/accepted is 0.997215 [0.991812, 1.004468], 10/15. Both intervals include parity. Positive estimates are retained, but this is not a verified complete-proof breakthrough or an adoption.

The native weighted-group kernel retains the existing identity `W(B) = 2W(pair sums) - sum(even B)`. Each level saves its correction points, computes norms/prefixes and inverse factors, then compacts affine coordinates in one native SIMD buffer. The last three levels use the original complete reducer. At 256 buckets it still performs 502 affine additions and eight shared inversions. It removes repeated coordinate conversion and temporary representations, rather than changing field arithmetic or the mathematical addition count.

This integration preserves the original 256-bucket parallel groups, `map_init` scheduling and final group combination. Admission checks native features, supported geometry and requested-vector forecasts against the existing weighted workspace forecast. For primary256 groups the new/old forecasts are 43,736/59,544 bytes for G1 and 75,672/112,728 for G2. These include the existing forecast charges, but do not prove an allocator/stack/process RAM bound. The state enum owns either native work or original scratch, with an Empty transition dropping one before constructing the other. Immutable inputs permit complete original fallback after identity/zero-norm rejection. No lookup table, transformed key, duplicate solver plan, new pool or arbitrary RAM cap is introduced.

The earlier weighted-tree caller study retains its eight failed calibration checks. This is a distinct full-SDK integration and measurement boundary; its earlier gate is neither waived nor rerun. The rejected joint-frontier design is also separate: it pooled late group frontiers and lost threaded performance. This candidate keeps those groups independent.

Accepted source176 / SDK `6d83…` is the retained production control. OFF and ON use the same fresh SDK `74bb…`, linked with actual optimized native/fat-LTO/one-codegen-unit final flags. Its private environment selector initializes once. Route counters update inside work; a wrapper prints them after the original SDK main returns. The original timing body recovers byte-exact apart from its name. Route counts include primes and any setup use, so they are not asserted to be per-request counts.

The host is Ryzen 9950X, 16 physical cores / 32 logical CPUs. All nine shapes run with 32 workers on CPUs 0–31, and primary also uses 16 workers on CPUs 0–15. Fifteen fixed rotated/reversed blocks compare all three arms. Each process has three primes, then 151 timed primary32 requests, 301 primary16 requests or 31 other requests. Timing includes witness solving, randomized proving, SDK verification and serialization; loading/primes are excluded. Reported latency is the median of fifteen process medians. CPU service is summed user/system time per request. The longer physical16 series was declared before capture; its failed checks remain failed.

Original-eight gain requires an equal-log aggregate upper ratio below 1 and at least 10/15 wins against both controls. Each context ON/control upper bound must be at most 1.03; OFF/accepted bounds must fit factor 1.03. Rank 4–12 bounds have nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise coverage. Every observation is retained, with no sample extension, minimum gain floor, favorable subset or unchanged reroll.

Original-eight ON/accepted: **1.000440 [0.996849, 1.004546], 6/15**.

Original-eight ON/OFF: **0.995816 [0.991767, 0.998895], 12/15**.

|Context|Accepted / OFF / ON ms|ON/accepted [bounds], wins|ON/OFF [bounds], wins|OFF/accepted [bounds], wins|
|---|---:|---|---|---|
|1x2-w32|7.906865 / 7.916479 / 7.883521|0.997215 [0.991812, 1.004468], 10/15|0.995152 [0.989854, 1.000644], 11/15|1.001681 [0.994820, 1.011835], 6/15|
|2x2-w32|11.173721 / 11.198288 / 11.208484|0.995377 [0.991380, 1.005324], 9/15|1.001476 [0.988278, 1.006461], 7/15|0.997976 [0.991519, 1.007425], 9/15|
|4x2-w32|16.164933 / 16.239887 / 16.132789|1.000290 [0.991018, 1.006346], 7/15|0.992882 [0.990086, 1.000481], 10/15|1.007559 [0.990385, 1.011823], 6/15|
|8x2-w32|24.116948 / 24.535311 / 24.201576|1.005032 [0.994581, 1.014362], 5/15|0.992969 [0.971742, 0.994921], 13/15|1.020864 [1.007695, 1.029496], 1/15|
|12x2-w32|33.812317 / 33.836984 / 33.732836|0.998144 [0.993435, 1.004188], 8/15|0.999432 [0.985664, 1.007094], 8/15|0.999843 [0.990845, 1.013319], 8/15|
|16x2-w32|39.934409 / 40.310056 / 40.112368|1.004057 [0.992353, 1.009194], 6/15|0.993430 [0.983474, 1.005679], 11/15|1.004293 [0.998916, 1.014657], 5/15|
|24x2-w32|65.904931 / 66.444654 / 66.031410|1.002125 [0.995319, 1.007218], 6/15|0.993653 [0.987826, 1.003473], 9/15|1.007585 [1.002260, 1.011324], 1/15|
|merge-24x1-w32|65.064326 / 65.273788 / 65.468387|1.007079 [0.995026, 1.015956], 5/15|1.004258 [0.985164, 1.011951], 7/15|1.007016 [0.997067, 1.011295], 6/15|
|merge-54x1-w32|162.357536 / 162.541678 / 162.147828|0.998003 [0.989576, 1.003827], 9/15|0.998964 [0.989023, 1.005196], 8/15|1.000579 [0.992132, 1.009643], 6/15|
|1x2-w16|9.372356 / 9.582222 / 9.451925|1.009308 [0.976226, 1.045297], 7/15|0.992593 [0.967916, 1.022829], 9/15|1.007728 [0.998789, 1.034158], 5/15|

|Context|Accepted / OFF / ON CPU ms/request|Accepted / OFF / ON recorded max-RSS counter MiB (diagnostic)|
|---|---:|---:|
|1x2-w32|179.497 / 179.140 / 178.098|134.50 / 133.50 / 136.50|
|2x2-w32|259.402 / 260.912 / 258.616|270.00 / 276.75 / 270.25|
|4x2-w32|423.983 / 423.743 / 422.791|312.25 / 317.50 / 318.50|
|8x2-w32|665.773 / 677.643 / 670.616|507.00 / 504.50 / 509.75|
|12x2-w32|938.915 / 935.794 / 934.961|715.00 / 725.25 / 720.00|
|16x2-w32|1122.087 / 1131.716 / 1121.061|831.25 / 839.75 / 827.50|
|24x2-w32|1894.174 / 1900.773 / 1894.260|1301.84 / 1297.11 / 1283.75|
|merge-24x1-w32|1805.564 / 1809.238 / 1811.021|1166.61 / 1168.25 / 1172.50|
|merge-54x1-w32|4461.536 / 4450.448 / 4440.774|1899.37 / 1894.12 / 1864.13|
|1x2-w16|116.807 / 117.884 / 117.063|92.25 / 92.25 / 92.25|


The separate resource matrix uses one fresh process per arm/context, 151 primary/2x2 requests or 31 others, plus three primes. Its timings are excluded from latency. It samples `/proc/PID/smaps_rollup` and status every 50 ms, selecting complete frames after FIFO enable + 10 ms and before disable. All ON/control sampled-max ratios are at most 1.05. A sampled maximum is a lower bound on lifetime peak; this matrix does not establish minimum RAM, pressure/swap behavior or a confidence interval for memory. The original `ru_maxrss` counters are retained as diagnostics because previous audits found them below page-table snapshots on this host; their precise undercount cause remains unisolated.

|Context|Accepted / OFF / ON sampled RSS max MiB|ON/accepted / ON/OFF|Accepted / OFF / ON snapshots|
|---|---:|---:|---:|
|1x2-w32|136.516 / 133.508 / 135.688|0.993934 / 1.016326|23 / 23 / 23|
|2x2-w32|315.711 / 317.367 / 322.160|1.020428 / 1.015102|30 / 30 / 30|
|4x2-w32|292.973 / 303.555 / 299.004|1.020586 / 0.985008|8 / 9 / 8|
|8x2-w32|513.465 / 507.969 / 522.121|1.016859 / 1.027861|12 / 12 / 13|
|12x2-w32|693.824 / 699.938 / 696.414|1.003733 / 0.994966|17 / 17 / 17|
|16x2-w32|791.594 / 824.801 / 805.129|1.017099 / 0.976150|19 / 20 / 19|
|24x2-w32|1229.457 / 1270.441 / 1192.195|0.969693 / 0.938410|31 / 30 / 30|
|merge-24x1-w32|1102.324 / 1117.516 / 1126.434|1.021871 / 1.007980|31 / 30 / 30|
|merge-54x1-w32|1835.996 / 1691.316 / 1678.441|0.914186 / 0.992388|63 / 62 / 62|
|1x2-w16|91.914 / 89.852 / 88.789|0.966001 / 0.988175|27 / 29 / 27|

Recorded-counter warnings retained without gating: `[]`.


Route rows from the first latency block use `[eligible, entered, fallback, denied, max_group, max_new_bytes, max_original_bytes]` separately for G1/G2. OFF rows are zero. ON obeys eligible=entered+fallback, and the native path enters both primary curves. Fallbacks and denials are retained; their presence does not indicate an invalid proof.

|Context|G1 counters|G2 counters|
|---|---|---|
|1x2-w32|`[68992,66990,2002,154,256,43736,59544]`|`[12474,12320,154,0,256,75672,112728]`|
|2x2-w32|`[17136,17000,136,102,256,43736,59544]`|`[2754,2720,34,0,256,75672,112728]`|
|4x2-w32|`[22848,22100,748,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|8x2-w32|`[22848,22134,714,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|12x2-w32|`[22848,22134,714,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|16x2-w32|`[22848,22134,714,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|24x2-w32|`[28560,27132,1428,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|merge-24x1-w32|`[22848,22134,714,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|merge-54x1-w32|`[34272,32130,2142,102,256,43736,59544]`|`[6256,5474,782,34,256,75672,112728]`|
|1x2-w16|`[136192,132240,3952,304,256,43736,59544]`|`[24624,24320,304,0,256,75672,112728]`|


All 450 latency processes precede nine grouped CPU-counter panels and nine separate fixed-16M-user-cycle profiles for primary32, 4x2 and merge24 across all arms. Stat panels use 31 requests; profiles use 251 primary or 101 other requests, excluding primes via FIFO. No owned build, hash, helper, analyzer, archive, independent verifier or other workload overlaps latency/profiles. The memory observer runs only in its separate matrix. Every grouped PMU runs 100%. All 229,579 sampled events pass ownership/CPU/mmap/FORK/period/zero-loss review; the smallest profile has 11,446 events and the largest unknown share is 2.5803%.

|Counter panel|Cycles M/request|Instructions M/request|Cache misses M/request|Branch misses M/request|Profile samples|Unknown share|
|---|---:|---:|---:|---:|---:|---:|
|stat-1x2-w32-OFF|810.888|1386.984|2.812893|0.898868|12268|0.01565|
|stat-1x2-w32-ON|808.751|1372.728|2.772010|0.887262|12206|0.01868|
|stat-1x2-w32-accepted|814.933|1388.377|2.942454|0.917476|12245|0.01821|
|stat-4x2-w32-OFF|1884.928|2362.568|7.030178|1.581735|11528|0.02568|
|stat-4x2-w32-ON|1877.599|2342.598|7.057387|1.615790|11446|0.02394|
|stat-4x2-w32-accepted|1902.511|2363.434|7.143403|1.604976|11937|0.02580|
|stat-merge-24x1-w32-OFF|8259.874|8735.892|34.731609|3.545114|53434|0.00709|
|stat-merge-24x1-w32-ON|8179.321|8703.907|34.896544|3.517363|52343|0.00688|
|stat-merge-24x1-w32-accepted|8318.249|8737.498|33.942436|3.621678|52172|0.00728|

|ON profile|Named weighted-tree context share|General field-inverse share (shared callers)|
|---|---:|---:|
|cycles-1x2-w32-ON|5.014%|7.611%|
|cycles-4x2-w32-ON|3.643%|4.587%|
|cycles-merge-24x1-w32-ON|0.737%|2.656%|


Primary ON/OFF instruction count falls about 1.03%, while cycles fall about 0.26% in the separate descriptive counter panel. These are single diagnostic panels, not paired effect estimates. Native weighted-tree symbol context, including enclosing Rayon symbols, is only part of the complete proof; inverse symbols may be shared with other stages. Flat attribution and inlining prevent treating these shares as exclusive critical-path fractions or removable-latency bounds. The complete-proof data support retaining the small OFF/ON estimate while looking for larger reductions in dominant MSM work; they do not support an unchanged rerun of this recipe.

|Primary context / arm|Minor faults/request|Voluntary switches/request|Involuntary switches/request|System ms/request|CPU ms / loop wall ms|
|---|---:|---:|---:|---:|---:|
|1x2-w16/accepted|31.638|263.100|345.528|2.110|12.281|
|1x2-w16/OFF|32.691|265.870|494.053|2.120|11.961|
|1x2-w16/ON|32.841|263.625|324.475|2.054|12.252|
|1x2-w32/accepted|130.113|396.411|225.523|5.117|22.470|
|1x2-w32/OFF|128.457|394.457|266.464|4.915|22.327|
|1x2-w32/ON|131.874|395.093|236.331|5.032|22.364|


Minor faults and context switches above are medians of timed-loop counter deltas divided by requests. They measure events, not their latency cost. A minor fault is resolved without a disk page-in; the existing driver does not export major faults. CPU/wall is logical-thread equivalents, not physical CPU utilization. These counts do not isolate allocator, cache, memory-bandwidth or scheduler causality.

Native qualification passes 130 core tests, including complete weighted/total references, all supported geometry, exception/fallback and state transitions, measured retained capacities against forecasts, unchanged inputs and threaded pools. Both raw and all six cubic prover tests pass, as does generator consistency. Field/SIMD helper bodies inherit the earlier 37-test qualification byte-exact; the field suite is not rerun. Source has 179 files: 171 originals are exact; reversing five parent edits recovers all 176 originals; three children are added. Original SDK and qualified kernel bodies recover exactly. Final portable/compatibility and deployment qualification are not claimed.

All 30 preflight exports pass Gnark before latency. The run performs 34,662 primary SDK requests and 2,100 separate resource requests: 36,762 fresh internally verified proofs in total. It exports 498 distinct 164-byte proofs, each independently checked against pinned Gnark verification with proof/public/image bindings and freshness against 28,349 prior hashes. The canonical ledger advances to 28,847 only after those checks and review. No resource proof is separately exported or counted again. Independent review recomputes all timing/aggregate/calibration/memory gates, profile bins, source recovery, all 498 bindings and 1,049 structured command receipts: 1,048 successes and one failed read-only checker.

Two tooling failures are preserved. An initial diagnostic match-arm syntax error prevented core compilation; it was fixed before successful qualification, with the failed source/log retained. After 60 successful preflight/verifier commands, the inherited capture reader rejected the new seven-field counters because it expected five. No latency had run. A separate 983-command continuation corrected only that reader and receipt resolution, then executed the original 982 remaining commands in order. Original pins/plan/helper and failed receipt stay unchanged. No successful test, preflight, proof, verifier or measurement was repeated to repair tooling. [Continuation note](artifacts/continuation-note.md) records the split roots.

Fresh experimental ON observations against supplied historical references:

|Shape|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|Target met|
|---|---:|---:|---:|---:|---:|---:|---|
|1x2|7.883521|37|4.693×|24|3.044×|3.7|false|
|2x2|11.208484|59|5.264×|30|2.677×|5.9|false|
|4x2|16.132789|109|6.756×|48|2.975×|10.9|false|
|8x2|24.201576|189|7.809×|74|3.058×|18.9|false|
|12x2|33.732836|284|8.419×|101|2.994×|28.4|false|
|16x2|40.112368|335|8.352×|124|3.091×|33.5|false|
|24x2|66.031410|528|7.996×|176|2.665×|52.8|false|
|merge-24x1|65.468387|507|7.744×|175|2.673×|50.7|false|


Candidate CPU 10× target count: 0/8. All-eight verified CPU 10× remains unmet. Merge54 has no supplied M5 reference. These are local observations, with no refreshed AWS/Aeglos measurement. No AWS resource was created, and no huge 250/4999, TEE, phone/ARM, memory-pressure or concurrent-proof claim follows.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source review](artifacts/source-review.json), [interpretation](artifacts/interpretation.json), [identities](artifacts/identities.json), [proof inventory](artifacts/proof-inventory.json), [profile symbols](artifacts/profile-symbols.json), [publication audit](artifacts/publication.json). Archives: [source snapshot](artifacts/source.tar.gz), [Rust workflows and protocol](artifacts/workflow.tar.gz), [command receipts](artifacts/receipts.tar.gz), [proof/public exports](artifacts/proofs.tar.gz), [counter panels](artifacts/counters.tar.gz), [resource snapshots](artifacts/memory-audit.tar.gz). [Checksums](SHA256SUMS). Keys, witnesses, input scalars, executables, libraries and raw perf.data stay local, bound by hashes. Local root:`/home/ubuntu/src/zopro-perf-work/weighted-sdk176`.

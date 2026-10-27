# Current CPU provers on c7a.24xlarge

Current adopted Zopro takes **11.966 ms for 1×2 at 32 workers**; pinned Aeglos takes **7.990 ms at 96 workers**. Aeglos remains faster for the primary proof at its best tested count. For 54→1, Zopro takes **126.887 ms at 64 workers**, versus Aeglos's **172.322 ms at 96**. Best-worker choices are descriptive.

At the same 32 workers, current Zopro improves on previous Zopro by **3.31% paired geometrically, 3/3 winning blocks**, with a median paired saving of **389.993 µs**. Primary improvements do not extend to 96 workers: current/previous is **1.02603**, with 0/3 wins. The small-proof scaling issue remains. This comparison adopts no new source; the previously adopted 171-file implementation stays in place. The original all-eight supplied M5 CPU 10× goal remains active, achieved 0/8.

## Method

One dedicated c7a.24xlarge, AMD EPYC 9R14, 96 physical cores without SMT. All engines use compact first-N CPU masks at 32/64/96 workers, inside the same 30 GiB container with swap disabled. Three fixed rotated/reversed fresh-process blocks cover all nine shapes. Primary processes run 101 timed requests after three primes; other shapes run 17 after three. All 243 latency processes finish before six separate 199 Hz warm user-cycle profiles, primary32/96 for each engine, 251 requests after three primes.

The current adopted source is freshly built for `target-cpu=znver4`, with all other retained flags. Previous is the exact prior AWS-tested Zopro executable; Aeglos is the pinned `0290613` Sorted/prepared-key executable from earlier comparisons, not a claim about latest upstream. Existing setup and native-budget policies are preserved. Aeglos reports prepared-key cache hits for timed calls. SDK adapters preserve each engine's timer and normalize output afterward. Witness solving, fresh proof randomness, proving, internal verification and serialization are timed. Loading/setup/primes are excluded from latency and included in lifetime RSS.

Every SDK call verifies internally; final exports also pass independent Gnark checks. No build, helper, hash, archive or second benchmark overlaps timed capture. Independent verifiers run between completed processes. Three blocks provide point estimates and wins; no 95% confidence or universal guarantee is claimed. Nothing is excluded, extended or successfully recaptured. Same-worker ratios and cross-worker best-tested comparisons remain separate.

## Same-worker latency

Times are medians of three process medians in milliseconds. Ratios are geometric means of within-block pairs; differences between displayed medians need not agree. No minimum gain floor is applied.

|Shape|Workers|Previous Zopro ms|Current Zopro ms|Aeglos ms|Current/previous|Wins|Current/Aeglos|Wins|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
|1x2|32|12.356320|11.966327|11.905637|0.966892|3/3|1.009442|1/3|
|1x2|64|12.305676|12.349144|8.701182|1.007559|1/3|1.423340|0/3|
|1x2|96|13.904740|14.368089|7.990218|1.026030|0/3|1.770117|0/3|
|2x2|32|16.418641|15.845498|18.924602|0.967709|3/3|0.841977|3/3|
|2x2|64|17.691535|17.188797|13.135075|0.984038|2/3|1.287226|0/3|
|2x2|96|19.046248|18.591921|11.739256|0.983788|3/3|1.602331|0/3|
|4x2|32|20.221495|19.968069|30.646582|0.987695|3/3|0.649685|3/3|
|4x2|64|21.007651|20.646672|20.764759|0.966270|3/3|0.991629|1/3|
|4x2|96|22.991127|22.707631|18.157863|0.983518|3/3|1.227457|0/3|
|8x2|32|27.971046|27.488522|53.607948|0.984326|3/3|0.513360|3/3|
|8x2|64|27.232231|27.222022|35.324159|1.000792|1/3|0.767016|3/3|
|8x2|96|28.867240|28.843111|30.577102|0.996953|2/3|0.926970|3/3|
|12x2|32|36.839059|36.460906|87.731689|0.988405|2/3|0.420335|3/3|
|12x2|64|35.245560|35.012048|55.195889|0.986422|2/3|0.626198|3/3|
|12x2|96|37.933946|37.295891|48.326518|0.987453|2/3|0.775133|3/3|
|16x2|32|43.246762|42.075360|99.075804|0.970584|2/3|0.416315|3/3|
|16x2|64|39.017444|39.956122|64.514917|1.021759|0/3|0.618130|3/3|
|16x2|96|43.320973|42.571615|56.715699|0.978565|3/3|0.744890|3/3|
|24x2|32|62.078462|62.516715|173.639412|1.006801|2/3|0.361861|3/3|
|24x2|64|56.654236|57.136986|103.809544|1.003155|1/3|0.544068|3/3|
|24x2|96|61.903928|61.248831|86.518215|0.999422|1/3|0.710992|3/3|
|merge-24x1|32|64.457579|62.754034|168.733479|0.988294|2/3|0.380232|3/3|
|merge-24x1|64|59.801665|59.693847|100.582971|1.003801|2/3|0.594811|3/3|
|merge-24x1|96|66.673993|66.005994|84.043066|0.995539|2/3|0.786577|3/3|
|merge-54x1|32|141.166870|139.894343|343.540529|0.994608|3/3|0.404943|3/3|
|merge-54x1|64|127.201385|126.886513|206.654743|0.996691|2/3|0.614397|3/3|
|merge-54x1|96|132.488512|132.532158|172.321738|0.999252|1/3|0.767893|3/3|

At 32 workers, primary current/Aeglos is **1.009442**, with 1/3 wins: this run does not establish a same-worker primary win either. At 64 and 96 workers, current/Aeglos is 1.423340/1.770117, with 0/3 wins in each. The prospective primary win condition against both controls is unmet. Current/previous original-eight geometric aggregates are 0.982506/0.996596/0.993811 at 32/64/96 workers, with 3/3, 3/3 and 2/3 block wins. Positive small aggregate gains are retained alongside individual regressions, including 16×2 at 64 workers and primary96. This is a comparison, not a new implementation-adoption gate.

## Best tested configurations and resources

Each engine selects the lowest displayed median across the three tested counts. Whole-process RSS is the maximum across blocks for that selected configuration, including loading/setup/primes. CPU is summed warm user+system time per request. Neither is minimum RAM or concurrent throughput.

|Shape|Previous ms/workers|Current ms/workers|Aeglos ms/workers|Current CPU ms|Aeglos CPU ms|Current RSS MiB|Aeglos RSS MiB|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|12.306/64|11.966/32|7.990/96|262.473|508.382|120.30|474.48|
|2x2|16.419/32|15.845/32|11.739/96|350.739|822.449|215.78|810.58|
|4x2|20.221/32|19.968/32|18.158/96|511.282|1387.064|275.32|1344.77|
|8x2|27.232/64|27.222/64|30.577/96|1256.872|2575.633|448.95|2457.74|
|12x2|35.246/64|35.012/64|48.327/96|1627.868|4117.930|629.90|3699.67|
|16x2|39.017/64|39.956/64|56.716/96|1821.284|4862.702|764.59|4612.10|
|24x2|56.654/64|57.137/64|86.518/96|2644.389|7415.774|1088.98|6791.86|
|merge-24x1|59.802/64|59.694/64|84.043/96|2645.868|7230.186|1087.07|6612.10|
|merge-54x1|127.201/64|126.887/64|172.322/96|4928.043|14873.448|1794.83|13063.67|

At their best tested counts, Zopro has lower latency than Aeglos on 8×2 and larger cases here; Aeglos has lower latency on 1×2, 2×2 and 4×2. Primary RSS is 120.30 MiB Zopro versus 474.48 MiB Aeglos; 54→1 is 1794.83 MiB versus 13063.67 MiB. These ratios compare whole-process peaks, including setup. They do not imply that adding tables would fix Zopro's scaling, that RAM capacity is the only speed difference, or that either engine has a universal memory bound. Full per-core CPU/RSS values are retained below and in analysis.json.

|Shape/workers|CPU previous/current/Aeglos ms per request|RSS previous/current/Aeglos MiB|
|---|---:|---:|
|1x2/32|273.059/262.473/327.585|122.55/120.30/425.23|
|1x2/64|456.505/444.771/433.617|125.44/127.70/445.45|
|1x2/96|752.613/743.748/508.382|120.96/116.54/474.48|
|2x2/32|364.588/350.739/536.594|208.04/215.78/715.99|
|2x2/64|613.941/596.183/680.378|216.19/227.69/756.93|
|2x2/96|1009.670/980.660/822.449|210.23/208.00/810.58|
|4x2/32|517.761/511.282/911.450|276.85/275.32/1251.06|
|4x2/64|908.364/896.972/1144.214|268.23/268.91/1279.78|
|4x2/96|1432.548/1428.556/1387.064|264.15/266.54/1344.77|
|8x2/32|772.083/763.782/1650.781|466.71/462.79/2280.52|
|8x2/64|1309.220/1256.872/2068.357|464.74/448.95/2334.38|
|8x2/96|1952.976/1979.928/2575.633|477.72/436.25/2457.74|
|12x2/32|1026.809/1012.960/2654.434|653.55/641.44/3378.19|
|12x2/64|1638.091/1627.868/3273.230|653.62/629.90/3446.39|
|12x2/96|2519.759/2528.544/4117.930|649.51/619.23/3699.67|
|16x2/32|1216.218/1209.786/3144.119|781.96/771.34/4268.67|
|16x2/64|1849.047/1821.284/3851.666|780.00/764.59/4383.38|
|16x2/96|2868.551/2856.578/4862.702|803.25/767.26/4612.10|
|24x2/32|1825.327/1834.241/5190.841|1192.91/1190.69/6362.02|
|24x2/64|2700.525/2644.389/6118.580|1187.02/1088.98/6430.53|
|24x2/96|4045.362/4005.735/7415.774|1236.09/1191.85/6791.86|
|merge-24x1/32|1757.429/1752.283/5031.985|1106.53/1073.64/6166.33|
|merge-24x1/64|2703.179/2645.868/5930.854|1146.80/1087.07/6245.20|
|merge-24x1/96|4137.021/4064.343/7230.186|1127.01/1093.80/6612.10|
|merge-54x1/32|3522.960/3533.914/10647.302|1788.61/1759.02/12492.41|
|merge-54x1/64|4961.167/4928.043/12139.008|1914.32/1794.83/12573.66|
|merge-54x1/96|7035.381/6979.626/14873.448|1934.34/1876.35/13063.67|

## Warm profiles and interpretation

The six separate profiles contain **178,577 samples**, zero lost records and one sampled SDK process per capture. Sampled shares are not removable wall-time fractions or exclusive critical-path costs. The sampled process and observer differ from each unprofiled latency run. Average occupied CPUs is warm summed CPU time divided by warm loop wall time; it is not a fraction of useful arithmetic.

|Primary workers|Engine|Samples|CPU ms/request|Warm loop ms/request|Average occupied CPUs|Epoch %|Scheduler %|
|---|---|---:|---:|---:|---:|---:|---:|
|32|previous|15431|280.914|12.865|21.84|8.40|4.95|
|32|current|15420|270.695|12.605|21.48|8.03|5.26|
|32|aeglos|17376|333.924|12.206|27.36|—|—|
|96|previous|53334|810.337|15.018|53.96|48.77|12.44|
|96|current|49564|794.508|14.982|53.03|49.55|12.52|
|96|aeglos|27452|534.322|7.945|67.25|—|—|

The generic categories match Zopro symbols; Aeglos appears as 100% other in that matcher, which is a categorization limitation rather than missing samples or absent arithmetic. Its retained flat symbols show Pool::work at 17.48%/26.51% for 32/96 workers, PairBatch<Lane1>::finish at 12.17%/10.17%, load at 10.85%/9.02% and Lane1::invert_all at 10.37%/9.50%. Pool::work includes dispatch and waiting and is not directly comparable to the sum of Zopro's epoch/scheduler categories.

Current Zopro at 96 workers has 49.55% epoch and 12.52% scheduler weight, versus 8.03% and 5.26% at 32. Its average occupied CPUs rise 21.48→53.03 while latency worsens; Aeglos rises 27.36→67.25 while latency improves. This confirms a scaling difference with current source and motivates examining task readiness, long indivisible work and coordination. It does not prove that removing a sampled fraction would recover that wall time, or establish a cache, DRAM, fault, fence or syscall cause. Prior failed empty-victim, epoch-collection, guard-reuse, retained-deque and idle-policy experiments remain negative evidence; they are not revived unchanged by these profiles.

The memory cgroup records no high/max/OOM/kill/swap event. Host swap is absent and container swap.max is 0. Aggregate container peak is 13,793,710,080 bytes across the entire campaign, not a per-engine RSS measurement. Thus this run did not force a memory-capacity failure or reclaim threshold. Memory-access costs remain possible and were not isolated. The 96-CPU quota records 7 throttle periods/47,753 µs summed throttling over the container lifetime; no per-engine attribution is made and zero throttling is not claimed. The profiles and CPU occupancy do not support reducing the explanation to RAM capacity alone.

## Supplied M5 references

References are supplied historical M5 timings, not fresh Apple measurements. Ratios use current Zopro's best tested worker count. No54→1 reference was supplied.

|Shape|Current C7a ms/workers|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|11.966327/32|37|3.092×|24|2.006×|3.7|
|2x2|15.845498/32|59|3.723×|30|1.893×|5.9|
|4x2|19.968069/32|109|5.459×|48|2.404×|10.9|
|8x2|27.222022/64|189|6.943×|74|2.718×|18.9|
|12x2|35.012048/64|284|8.111×|101|2.885×|28.4|
|16x2|39.956122/64|335|8.384×|124|3.103×|33.5|
|24x2|57.136986/64|528|9.241×|176|3.080×|52.8|
|merge-24x1|59.693847/64|507|8.493×|175|2.932×|50.7|
|merge-54x1|126.886513/64|—|—|—|—|—|

**0/8 rows meet the supplied CPU10× objective.** The primary target is 3.7 ms, so a 3.31% improvement at 32 workers is useful progress and remains far from completion.

## Validation, identities and lifecycle

The current C7a build passes 33 unfiltered field tests, 116 core tests, two raw and six cubic prover tests, plus generated-field consistency. All 171 snapshot/production source bytes match the adopted manifest. Previous Zopro and Aeglos reuse exact inherited qualified artifacts. Current SDK SHA256 is 6b5bb9d6f540db07a3de1eb15ad180af8b9499166414d5f9a67e99f310a1789e; previous 019ca258641c0cde9a9915028ceaa6c686388dce736b96d154ece118fca8c7e7; pinned Aeglos driver eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b. The pinned pool source hash matches its retained build receipt; its source structure does not alone establish a performance cause.

Local exact-container preflight adds 108 SDK calls/27 exports. Cloud latency adds 7,128 calls/243 exports; profiles 1,524 calls/six exports. Total **8,760 SDK calls, 276 distinct independent Gnark exports and 552 proof/public members**, global ledger **9,904→10,180**. Every export is 164 bytes, matches its exact fixture public hash, and is fresh against all prior/new identities. All 276 are independently reverified locally after retrieval; repeat verification is counted once. The 6,425,870-byte archive matches remote SHA256 1cce916c833901654ce1e6aad9f50a83f3511e54367a1d22b8c8a6ef3d572cd2. Raw perf data stays local with hashes.

A local mount permission repair occurs before any SDK call. A packaging helper then creates an incorrect destination-account tag; publication fails before provisioning. Retagging the same immutable image fixes publication without rebuilding or repeating proofs. An offline analyzer ownership error is fixed before post-transport verification. Original failures and repairs are retained; none changes prover code, flags, successful tests or measured captures.

Exactly one owned c7a.24xlarge is launched, with a 90-minute self-termination timer and delete-on-termination volume. After retrieval it is terminated; the owned volume is absent, the unique ECR tag and private transfer object/bucket are removed, and private presigned/Docker credential files are deleted. Readback finds zero live TemporaryProverBenchmark instances in the region. No unowned or production resource is modified. This is a container CPU benchmark, not a Nitro-enclave, phone, ARM, forester 250/4999, minimum-RAM, sustained-throughput or concurrent-server result.

The next optimization needs to address the observed C7a small-proof parallelism/coordination gap while preserving low-memory and concurrent-call behavior. Review prior scheduling/queue failures and real task dependencies before choosing a distinct experiment; further isolated arithmetic gains must be judged in complete proofs. No source promotion or unchanged acceptance rerun follows from this comparison.

[Protocol](artifacts/design.md), [all cases and paired results](artifacts/analysis.json), [raw rows](artifacts/remote-results/output/measure/performance/rows.json), [native/source identity](artifacts/aws-accepted-manifest.json), [proof inventory](artifacts/proof-inventory.json), [post-transport verification](artifacts/post-transport-gnark.log), [cleanup readback](artifacts/cleanup-complete.json), [raw profile hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).

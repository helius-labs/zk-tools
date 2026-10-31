# Phi2 dispatch after grouping large G2 queries

Selecting Phi2 by the effective grouped size did not establish a full-proof speedup. The five-shape estimate is **0.08% slower than fresh OFF**, with bounds spanning parity, and **0.60% slower than accepted**, with bounds above parity. This rejects the fixed implementation without claiming every possible post-group dispatch must be slower.

Full SDK gate: **false**. The candidate is not adopted. Its unchanged recipe is closed under the declared gate. Production source 176 / SDK `6d83…` remains unchanged.

Five eligible shapes, ON/accepted: **1.005990328 [1.001882819, 1.011712675], 3/15**.
Five eligible shapes, ON/OFF: **1.000819471 [0.995747803, 1.005328561], 6/15**.

Failed checks: `["1x2-w16/ON/accepted","eligible5/ON/accepted/gain","eligible5/ON/OFF/gain"]`. All observations remain. All ten control calibrations and all maximum-RSS guards pass. The separate primary16 ON/accepted nonregression bound is 1.035058, above the declared 1.03; that is insufficient evidence for the guard, not an established 3.5% slowdown. The two aggregate gain failures independently prevent adoption.

The original G2 dispatch tests both the original query length and its grouped length against the 65,536-term cutoff. Several large circuits group to a small effective population but retain generic full-width MSM. This candidate selects the existing Phi2 route when the original population is at least 65,536 and the effective population is between 1,024 and 65,535. The five selected shapes were identified from existing source/count evidence before timing: 4x2, 8x2, 12x2, 16x2 and merge24. This does not enable Phi on large effective populations or change the failed wider-window candidate.

The existing group-affine reduction is reused. A bounded proof-local clone of canonical grouped words permits in-place decomposition while concurrent B1 readers retain their inputs; requested scalar payload is at most 2,097,120 bytes. Field arithmetic, grouping policy/order, scalar split, width12/two-part geometry, queues, indexed top, weighting, key, solver, FFT and K/H remain. There is no lookup table, copied transformed key, duplicate solver plan, new pool or fixed RAM cap. The grouped point array already exists in the original route. A checked optional reservation covers the complete native parent/top/worker-leaf envelope and grouped-point capacity, in addition to existing charges. Denial occurs before new point reduction or clone and preserves the original fallback. A single payload bound is not a total-memory claim.

The actual admitted dimensions and requested scalar payloads follow. Additional layout reservation is conservative accounting over existing forecasts, not measured extra heap or minimum RAM. [Machine-readable dimensions](artifacts/actual-routes.json).

|Shape|Original points|Effective terms|Scalar clone bytes|Additional typed reservation bytes|
|---|---:|---:|---:|---:|
|4x2|95537|30544|977408|54469672|
|8x2|183347|37850|1211200|55668912|
|12x2|270757|46062|1473984|57016736|
|16x2|358343|51597|1651104|57925272|
|merge-24x1|507895|62005|1984160|59633504|

The SDK timing body is byte-exact apart from its function name. A wrapper prints six diagnostic counters after the original main returns: eligible, entered, denied, last original size, last grouped size and requested layout bytes. All per-query counter overhead is timed. OFF/ON share one optimized executable; accepted is the retained qualified SDK. The actual compiler arguments include -O, native CPU, fat LTO and one codegen unit. Route checks cover every ON request and prime on the five eligible shapes, with zero denials; primary, 2x2 and merge54 stay on their original paths. The 24x2 route follows its recorded effective population. Counter snapshots are retained in the review.

Ryzen 9950X, 16 physical cores / 32 logical CPUs, shared host. Nine circuits use 32 workers on CPUs 0–31; primary also uses 16 on CPUs 0–15. Fifteen fixed rotated/reversed blocks compare accepted/OFF/ON. Each process runs three primes then 151 primary or 31 other requests. Timers include fresh witness solving, randomized proving, SDK verification and serialization; loading and preparation are excluded. Reported latency is the median of process medians, CPU is summed service per request, and RSS is lifetime high-water including loading/primes. RSS is not minimum RAM or incremental workspace.

The five-shape equal-log gain requires an upper ratio below one and at least 10/15 wins against both controls. All ten context upper ratios must be at most 1.03; every OFF/accepted interval must fit factor 1.03, and every maximum RSS ON/control ratio must be at most 1.05. Rank-4–12 bounds have nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise coverage. Unchanged shapes need not improve but must satisfy all regression/calibration/memory checks. No gain floor, excluded result, extension, favorable post-result subset, threshold change or unchanged reroll is used.

|Case|Accepted/OFF/ON ms|ON/accepted [bounds],wins|ON/OFF [bounds],wins|OFF/accepted [bounds],wins|
|---|---:|---|---|---|
|1x2-w32|7.900754/7.916343/7.925440|1.003294078 [0.995183694, 1.009562363], 4/15|1.003480728 [0.998073977, 1.006427836], 4/15|1.000780316 [0.992165800, 1.007812471], 7/15|
|2x2-w32|11.134741/11.165920/11.155811|1.007205196 [0.997810379, 1.009769568], 6/15|0.999473524 [0.995233711, 1.004696372], 8/15|1.003465300 [0.999055299, 1.007849057], 4/15|
|4x2-w32|15.976429/16.179536/16.013739|0.997200520 [0.993470396, 1.016922941], 9/15|0.990501810 [0.977939170, 1.009510329], 9/15|1.007327561 [0.997622617, 1.018841296], 4/15|
|8x2-w32|24.075699/24.524274/24.532500|1.019181382 [0.994731458, 1.028292293], 4/15|1.006486528 [0.992366823, 1.012849065], 6/15|1.011054580 [1.000312555, 1.025549055], 3/15|
|12x2-w32|33.744416/33.795964/33.962007|1.007448000 [0.999325584, 1.016607174], 5/15|1.007468871 [0.999924679, 1.013906002], 4/15|1.000864712 [0.992310887, 1.007523887], 6/15|
|16x2-w32|39.955796/40.139533/40.102621|1.006266247 [0.997823419, 1.009152924], 5/15|0.995397326 [0.988236782, 1.007746866], 8/15|1.005043332 [0.995163716, 1.016697903], 6/15|
|24x2-w32|66.240470/66.253173/66.196326|0.999459250 [0.991181439, 1.011101094], 8/15|0.999034622 [0.988755716, 1.004806880], 9/15|1.004342188 [0.994677952, 1.008639458], 6/15|
|merge-24x1-w32|64.968179/64.993331/65.328489|1.005545946 [0.995331635, 1.014930054], 4/15|1.003933940 [0.994010521, 1.011574851], 7/15|1.001329075 [0.999635182, 1.005331631], 5/15|
|merge-54x1-w32|161.905198/161.565697/162.145190|0.998891378 [0.989111103, 1.004670465], 8/15|0.997292740 [0.991222489, 1.005616290], 9/15|0.997768067 [0.989368631, 1.003778200], 10/15|
|1x2-w16|9.345228/9.465294/9.535256|1.016920542 [0.985501890, 1.035058123], 4/15|1.005635867 [0.990492534, 1.019876115], 6/15|1.012847841 [0.971576066, 1.023896724], 5/15|

|Case|Accepted/OFF/ON CPU ms/request|Accepted/OFF/ON peak process RSS MiB|
|---|---:|---:|
|1x2-w32|179.317/179.019/179.077|136.25/132.75/133.50|
|2x2-w32|259.302/259.409/259.870|276.00/270.25/282.50|
|4x2-w32|420.664/425.593/418.945|307.50/314.50/314.50|
|8x2-w32|667.743/667.919/672.056|508.25/521.00/512.50|
|12x2-w32|939.214/937.603/936.981|719.75/719.75/723.50|
|16x2-w32|1123.867/1128.031/1119.045|828.75/838.00/821.50|
|24x2-w32|1894.501/1900.206/1894.744|1266.54/1315.50/1297.00|
|merge-24x1-w32|1799.931/1803.083/1802.495|1190.41/1161.70/1163.26|
|merge-54x1-w32|4438.293/4432.313/4427.660|1883.87/1842.24/1869.17|
|1x2-w16|116.848/117.211/116.975|88.75/90.75/89.00|

Thirty preflight proofs independently pass Gnark before latency. All 450 latency processes precede nine grouped stat panels and nine separate fixed-16M-cycle profiles on primary32, 4x2 and merge24 across all arms. Stat panels measure 31 requests; profiles measure 251 primary or 101 other requests, after three FIFO-excluded primes. All captures precede the remaining independent verifications and decoding. No owned build, helper, hash, archive, analyzer, verifier or second benchmark overlaps capture. Raw faults/context switches are retained without assigning a causal share.

|Counter panel|Cycles M/request|Instructions M/request|Cache misses M/request|Branch misses M/request|Counter/profile qualified|Profile samples|
|---|---:|---:|---:|---:|---|---:|
|stat-1x2-w32-OFF|812.728|1387.979|2.721792|0.905780|true/true|12328|
|stat-1x2-w32-ON|807.022|1386.413|2.688561|0.912264|true/true|12277|
|stat-1x2-w32-accepted|796.915|1392.449|2.801701|0.906877|true/true|12384|
|stat-4x2-w32-OFF|1890.341|2362.658|7.024870|1.652033|true/true|11581|
|stat-4x2-w32-ON|1883.064|2389.767|7.209503|1.642432|true/true|11417|
|stat-4x2-w32-accepted|1892.157|2363.766|7.006558|1.617382|true/true|11450|
|stat-merge-24x1-w32-OFF|8353.132|8739.486|35.372696|3.609934|true/true|51415|
|stat-merge-24x1-w32-ON|8330.937|8757.095|35.686324|3.547867|true/true|51410|
|stat-merge-24x1-w32-accepted|8354.711|8729.489|34.468309|3.618635|true/true|52117|

Diagnostics qualified: **true**. All 226379 samples are independently matched to executable PID, task/FORK ownership, allowed CPUs, 16M period, mappings and loss receipts. Qualifying panels require at least 99.9% event running, at least 1,000 profile samples, at most 5% unknown period and zero losses. Symbol attribution includes inlined work and does not establish exclusive stalls, dynamic spills, local CPI or removable wall-time fractions. Absolute period coverage remains unvalidated.

The individual 4x2 estimate improves 0.950% versus OFF and 0.280% versus accepted, with 9/15 wins and both intervals spanning parity. The 16x2 OFF estimate improves 0.460%, also inconclusive. These positive estimates remain; no favorable shape is promoted independently. In the counter panels, 4x2 instructions rise from 2362.658M to 2389.767M/request and merge24 from 8739.486M to 8757.095M. This does not establish a reduction in overall arithmetic work or isolate a memory/scheduling cause.

Qualification retains 127 passed core cases from the original full 128-test run and one corrected filtered case. The initial failure was a synthetic fixture rejected by the existing grouping sample; no arithmetic mismatch occurred. Only the fixture's group stride changes in the corrected snapshot, and the child backend before cfg(test) is byte-exact. The failed case then passes with 127 other cases filtered; successful tests are not rerun. Twelve complete generator query cases cover original lengths 65,536/65,537/131,137, small grouped populations, zero/full-width scalars, exceptional points and 1/4-worker pools with 512 KiB stacks. Denial/bounds/overflow and concurrent immutable grouped readers are covered. Both raw and all six cubic prover tests pass; fields inherit byte-exact qualification. Portable and final resource behavior remain unqualified here. Both 177-file snapshots are retained; 172 originals stay exact and reversing four parent edits recovers all 176 accepted files.

The experiment performs **27,912 internally verified SDK requests** and exports **498 distinct 164-byte proofs**, each independently checked by Gnark with exact image/public/VK binding and freshness against 27,353 prior identities. The ledger becomes **27,851**. Independent review checks 1,049 command receipts including the original failed test run, source/library/executable pins, all paired latency/CPU/RSS statistics and gates, route counts, every profile symbol bin and proof/export binding. Archives reconstruct exact bytes; no query, proof or verifier is rerun by review.

Fresh candidate comparisons against supplied historical M5 references follow. These are ON results from this experiment; they do not imply adoption.

|Shape|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10x target ms|Target met|
|---|---:|---:|---:|---:|---:|---:|---|
|1x2|7.925440|37|4.669×|24|3.028×|3.7|false|
|2x2|11.155811|59|5.289×|30|2.689×|5.9|false|
|4x2|16.013739|109|6.807×|48|2.997×|10.9|false|
|8x2|24.532500|189|7.704×|74|3.016×|18.9|false|
|12x2|33.962007|284|8.362×|101|2.974×|28.4|false|
|16x2|40.102621|335|8.354×|124|3.092×|33.5|false|
|24x2|66.196326|528|7.976×|176|2.659×|52.8|false|
|merge-24x1|65.328489|507|7.761×|175|2.679×|50.7|false|

Candidate rows meeting the supplied CPU target: 0/8. The full all-eight objective remains unproven and active; production is unchanged. Merge54 has no supplied M5 reference. Historical AWS comparisons predate source176 and are not refreshed here. No AWS resources or new AWS/Aeglos, mobile/ARM/TEE, huge250/4999, pressure/swap, minimum-RAM or concurrent-throughput result follows.

[Design](artifacts/design.md), [all results](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/source-audit-fixed.json), [identities](artifacts/identities.json), [proof inventory](artifacts/proof-inventory.json), [profile symbols](artifacts/profile-symbols.json), [linked code](artifacts/codegen.json). Archives: [both source snapshots](artifacts/source.tar.gz), [workflow/helper sources](artifacts/workflow.tar.gz), [all command receipts](artifacts/receipts.tar.gz), [proof/public exports](artifacts/proofs.tar.gz), [disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS). Private keys/scalars/witnesses, executables/libraries and raw perf.data remain local with hashes. Local root: `/home/ubuntu/src/zopro-perf-work/postgroup-phi176`.

A [preparation supplement](artifacts/preparation-supplement.tar.gz) retains the successful preparation log and total-count correction. Before any plan or capture, the job-sum assertion corrected 26,562 to 27,912 requests: the difference is 450 latency processes × three excluded primes. Timed request counts, blocks, contexts, thresholds and 498 exports never changed. One read-only preflight route check was added before timing. Original logs, helpers, source, successful captures and the failed native fixture remain in the archives. This editorial/closure step changes no source or performance data.

# Shared unit-Z: complete-proof comparison

**The candidate is slower and is not adopted.** At32 workers, primary REF/OFF/ON measures **7.803149 /7.805372 /8.044304ms**. The paired ON/REF slowdown is **3.20%**, interval[3.02%,3.36%], with0/31wins. All11control-calibration checks pass. The candidate fails12of79declared checks, including primary gain/nonregression and both all-eight aggregate gains. The accepted bitmap-reset build and production source remain unchanged.

The [qualified shared method](../2026-11-09-unit-z-shared176/README.md) specializes initial projection and finalZ for unit-Z inputs while sharing the original arithmetic. It saves3multiplications+2squares in eligible scalar calls. Both coherent SDK arms retain adopted bitmap reset and the original allocator, fixed-placement driver, nativeO3/fatLTO/oneCGU and stacks. OFF is the literal original method; ON selects the qualified shared field. REF is the exact accepted bitmap-reset SDK. Only field curve.rs differs from accepted176source, with175exact originals and reversible recovery of the changed file. No counter/prototype wrapper, new buffer, table, key copy, plan, pool, forecast or memory policy is linked into the candidate.

Each sample includes solving, fresh randomized proving, internal verification and serialization. Loading, pool creation, three warm proofs and final export are outside latency. Case and arm orders rotate/reverse under the prospective protocol. Primary16/32use31blocks of151requests; other conditions use15blocks of31requests, except primarysolo11. No observations are excluded, repeated or extended.

|Case|REF ms|OFF ms|ON ms|ON/OFF paired ratio[95%]|Wins|ON/REF paired ratio[95%]|Wins|
|---|---:|---:|---:|---|---:|---|---:|
|1x2-w32|7.803149|7.805372|8.044304|1.032067 [1.030529, 1.033624]|0/31|1.031977 [1.030224, 1.033648]|0/31|
|2x2-w32|10.960964|10.930117|11.293129|1.035031 [1.029276, 1.041202]|0/15|1.030778 [1.026390, 1.035601]|0/15|
|4x2-w32|15.869955|15.825998|16.061432|1.017633 [1.010749, 1.024038]|2/15|1.011023 [1.005287, 1.016155]|2/15|
|8x2-w32|24.036033|24.344933|24.348632|1.006530 [0.998908, 1.014164]|4/15|1.014640 [1.004697, 1.025273]|4/15|
|12x2-w32|33.735567|34.070172|34.322645|1.003068 [0.998049, 1.008076]|7/15|1.015715 [1.009787, 1.021930]|1/15|
|16x2-w32|39.937712|40.038870|40.315674|1.006819 [1.001253, 1.012555]|5/15|1.008202 [0.999612, 1.016877]|5/15|
|24x2-w32|66.114430|66.333180|66.528130|1.001443 [0.995659, 1.007556]|8/15|1.005207 [1.001621, 1.009074]|5/15|
|merge-24x1-w32|65.391550|65.743868|65.852001|1.004059 [0.998927, 1.009554]|6/15|1.007272 [1.001972, 1.011865]|4/15|
|merge-54x1-w32|162.727253|162.779167|162.451177|0.998710 [0.995018, 1.002339]|8/15|0.999807 [0.994466, 1.004557]|5/15|
|1x2-w1|94.381246|94.183540|96.541204|1.024919 [1.022642, 1.027128]|0/15|1.021694 [1.019576, 1.023814]|0/15|
|1x2-w16|9.187282|9.158583|9.473263|1.034753 [1.032794, 1.036769]|0/31|1.032243 [1.030271, 1.034216]|0/31|

Displayed times are medians of process means of integer-rounded emitted nanosecond samples. Paired ratios use20000log-bootstrap resamples; dividing displayed marginal medians need not give the paired estimate. Intervals assume exchangeable blocks and have no familywise guarantee. All-eight aggregates independently resample within each shape with equal log weight;31primaryblocks and15others are not treated as synchronized cross-shape blocks.

Primary ON/OFF is1.032067[1.030529,1.033624],0/31wins; ON/REF is1.031977[1.030224,1.033648],0/31. All-eight ON/OFF is1.013254[1.011208,1.015308] and ON/REF1.015556[1.013357,1.017766]. The implementation is closed without adoption, unchanged recapture, curve/shape/worker subset selection, or compiler/layout/threshold rescue. Positive individual estimates remain in the table; they do not override the failed full gate. The direct duplicated-body variant remains unmeasured and is not selected for retry.

All11calibration,22RSS and18diagnostic checks pass. Failed checks are six nonregression bounds(primary16/32and2x2_32against both controls), four primary32gain/win requirements and both all-eight gain requirements:

- `1x2-w16/ON/OFF/nonregression`
- `1x2-w16/ON/REF/nonregression`
- `1x2-w32/ON/OFF/nonregression`
- `1x2-w32/ON/OFF/primary-gain`
- `1x2-w32/ON/OFF/primary-wins`
- `1x2-w32/ON/REF/nonregression`
- `1x2-w32/ON/REF/primary-gain`
- `1x2-w32/ON/REF/primary-wins`
- `2x2-w32/ON/OFF/nonregression`
- `2x2-w32/ON/REF/nonregression`
- `ALL8/ON/OFF/gain`
- `ALL8/ON/REF/gain`

Maximum observed whole-process RSS, including loading/warm proofs:

|Case|REF MiB|OFF MiB|ON MiB|
|---|---:|---:|---:|
|1x2-w32|132.75|132.75|135.25|
|2x2-w32|270.75|272.00|276.75|
|4x2-w32|315.50|302.50|311.25|
|8x2-w32|508.25|506.25|516.00|
|12x2-w32|715.00|725.25|712.25|
|16x2-w32|840.77|806.50|816.25|
|24x2-w32|1288.23|1292.36|1294.75|
|merge-24x1-w32|1168.17|1190.58|1174.75|
|merge-54x1-w32|1910.79|1952.77|1985.96|
|1x2-w1|34.25|34.50|34.50|
|1x2-w16|88.00|88.25|87.75|

Every declared5%RSSguard passes. Primary32ON135.25MiB versus132.75MiB for both controls is not an additional logical-buffer bound, minimum RAM, stack peak or pressure result. No new physical memory/phone feasibility claim follows.

All nine PMU panels meet99.9%running coverage. Nine fixed16Mcycles:u profiles qualify with119780samples, minimum8822, maximum unknown2.05947%, zeroLOST and checked actual executable/PID/TID/CPU ownership. All recording precedes offline decoding and proof verification. Absolute period coverage remains unvalidated; flat samples are not wall-time critical-path fractions.

The single diagnostic panels show that fewer instructions need not mean fewer cycles. These ratios have no replication confidence interval and do not isolate a cache, branch, bandwidth, frequency or dependency cause:

|Diagnostic case|Instructions ON/OFF|Cycles ON/OFF|Cache misses ON/OFF|Branch misses ON/OFF|
|---|---:|---:|---:|---:|
|1x2-w32|0.995073|1.023409|0.950386|0.999078|
|1x2-w16|0.993183|1.019479|0.965178|1.001812|
|2x2-w32|0.999613|1.023154|0.997134|1.008339|

Primary32instructions fall0.493%while cycles rise2.341%; cache and branch misses do not rise. This rules out using the removed multiplication count alone as a speed prediction. It does not prove a unique cause of regression or that all unit-Z specializations are slower. Any new caller/state mechanism needs its own complete qualification and fixed performance experiment; this global-method recipe remains closed.

Four prospective helper builds,52preflight commands and1263main commands complete on first execution in this SDK measurement. Earlier native code-sharing and SDK link-search-path repairs are inherited from the linked qualification report; no recording/analyzer/reader repair occurs here. The separate same-agent reader checks440capture pins,176-source recovery, coherent field/library identities, every raw mean, receipt, placement, proof binding and all79decisions. Bootstrap/aggregate statistics and PMU/profile parsers are shared unchanged, not an external audit.

Preflights add104SDKrequests/26fresh Gnark exports, then the main run44106requests/609exports. Total **44210SDKrequests and635fresh unique independently Gnark-verified exports**, advancing canonical inventory **41803→41829→42438**. Every proof is164bytes with the expected image/public binding and no prior/new hash duplication. Calls are not independent exported proofs. No new AWS/Aeglos, ARM/mobile/TEE, huge250/4999, pressure/swap/minimum-RAM or concurrent-throughput result is added.

The following fresh accepted REF observations compare the unchanged accepted build with supplied historical M5CPU/Metal figures on different hardware. Candidate ON remains rejected; its timings are also retained here. M5 was not remeasured.

|Shape|Accepted REF ms|Rejected ON ms|M5 CPU ms /REF speedup|M5 Metal ms /REF speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.803149|8.044304|37 /4.742x|24 /3.076x|3.7|
|2x2|10.960964|11.293129|59 /5.383x|30 /2.737x|5.9|
|4x2|15.869955|16.061432|109 /6.868x|48 /3.025x|10.9|
|8x2|24.036033|24.348632|189 /7.863x|74 /3.079x|18.9|
|12x2|33.735567|34.322645|284 /8.418x|101 /2.994x|28.4|
|16x2|39.937712|40.315674|335 /8.388x|124 /3.105x|33.5|
|24x2|66.114430|66.528130|528 /7.986x|176 /2.662x|52.8|
|merge-24x1|65.391550|65.852001|507 /7.753x|175 /2.676x|50.7|

The full verified all-eight CPU10x goal remains active and unmet, **0/8**. The accepted build was already below8ms;7.809034ms is its adoption observation,7.803149ms its fresh matched observation. Historical7.699867ms used a different campaign/statistic. This rejected candidate adds no speed milestone.

[Results](artifacts/analysis.json), [protocol](artifacts/protocol.md), [raw reader](artifacts/review-result.json), [prospective review](artifacts/prospective-review.json), [preflights](artifacts/preflight-review.json), [proof inventory](artifacts/proof-inventory.json), [diagnostics](artifacts/diagnostic-summary.json), [fresh CPU/Metal comparison](artifacts/fresh-comparison.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Binaries/libraries, unchanged full source snapshots, raw perf, key/image/witness/proof bytes and full historical proof ledgers are omitted. Large decoded logs are retained locally with byte/hash indexes.

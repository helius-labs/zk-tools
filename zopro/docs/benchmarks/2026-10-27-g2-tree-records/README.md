# G2 prefix records in existing tree layers

The experimental G2 tree kernel improves **54→1 from176.932 to163.652ms**, winning all nine confirmation blocks against both controls. Its paired median ratio against accepted is0.924943, with a confidence interval of[0.916213,0.934171]. For24×2, the corresponding ratio is0.981112, interval[0.967395,0.993373], with8/9 wins against accepted and9/9 against fresh. These positive results are retained.

**The global confirmation fails; no change is adopted.** Primary32 has a positive paired median saving of15.098µs against accepted, but its interval includes regression. Primary16 calibration, primary16 nonregression,8×2 nonregression and the original-eight aggregate against accepted also fail their declared interval checks. There is no minimum positive-gain cutoff.

The existing G2 affine-tree layer stores a prefix and norm pair instead of coordinates and scaled numerators. Reverse evaluation rereads each disjoint original pair and combines prefix with inverse before the existing Fq2 numerator formula, removing one elementary field product per full ordinary pair. The record payload falls from nine native vectors to two. This applies the accepted streaming arithmetic ordering to previously unchanged tree layers.

Tree topology, tile size, pair selection, scalar tails, exceptional-pair compaction, weighted/indexed/grouped callers, query dispatch and scheduling remain unchanged. G1 keeps its original arithmetic bodies behind a compile-time false branch. Portable and no-IFMA builds retain the original G2 path. Only `msm.rs` changes; reversing the edits recovers the original file, and169 other originals are exact. There is no new key table, copied input array, plan, pool or field primitive.

This differs from the previous G2 level/prefix-tree experiments, which replaced a streaming query with trees and left generic weighted/top reductions unchanged. It also excludes the G1 prefix-streaming variant that previously regressed. Source review first checked the proposed empty-inverse bypass against the historical operation census: the actual `m==0` counter found no primary empty tree batches. That bypass was not built, and the old counts are not relabeled as a fresh capture.

## Complete verified SDK comparison

Ryzen9950X, exact accepted SDK and reused pinned fresh baseline, candidate with identical compiler/native flags/fat LTO/one code-generation unit/standard SDK driver. Baseline source, library and consumer pins are rechecked; no new control build is claimed. Timed requests include witness construction, fresh blinded proving, SDK verification and serialization; loading and primes are excluded. Ratios and savings pair within each rotated/reversed fresh-process block. CPU includes coordination and lifetime RSS includes load/primes; neither isolates kernel service or minimum RAM.

|Case|Accepted ms|Fresh ms|Tree ms|Tree/accepted|Tree/fresh|Paired saving vs fresh µs|Wins vs fresh|Tree RSS MiB|Tree CPU ms/request|
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
|12x2-w32|34.421324|34.660325|34.100319|0.996900|0.988954|412.346|6|706.75|953.853|
|16x2-w32|40.799074|40.635426|40.410350|0.995678|0.992194|364.878|7|835.25|1131.773|
|1x2-w16|10.715802|10.491810|10.499122|0.971829|1.019389|5.419|5|91.00|129.333|
|1x2-w32|8.633414|8.678249|8.624067|0.995760|0.990754|126.308|7|113.50|197.743|
|24x2-w32|68.862085|68.939792|67.604363|0.979841|0.981704|1242.533|9|1300.47|1939.704|
|2x2-w32|11.738060|11.748330|11.743912|1.005064|1.004226|-6.983|4|270.50|272.926|
|4x2-w32|16.207468|16.216991|16.130545|0.995181|0.993662|43.201|5|316.75|421.482|
|8x2-w32|24.398451|24.918404|24.843198|1.005599|0.990315|109.186|6|505.50|685.794|
|merge-24x1-w32|67.480569|67.664167|66.903513|0.989078|0.984376|874.049|9|1165.79|1862.270|
|merge-54x1-w32|176.931927|178.821837|163.651863|0.924682|0.919932|14769.037|9|1818.89|4487.106|

The table uses the **nine-block confirmation**, not the short screen. Ratios shown are geometric means of block ratios; the interval checks below concern their medians. Displayed time medians, geometric ratios and paired median savings are distinct summaries.

## Screen failure and prospective extension

The original screen used three blocks at primary16/CPUs0–15 and primary32/CPUs0–31,17 timed requests after3 primes. It **failed**, with primary16 fresh/accepted ratios1.556461 and0.809495 outside its factor1.05 block calibration limit. Primary32 won3/3 against both controls, with33.203µs paired median saving against accepted and117.121µs against fresh. That positive observation and the failure remain unchanged in [screen analysis](artifacts/screen-analysis.json).

The [confirmation addendum](artifacts/confirmation.md) was declared after that result and before any additional capture. It documents a deviation from the original conditional stop: one additional full matrix using the same frozen source, executables, controls, fixtures and request boundary. It is informed by the screen and is not presented as independent discovery. No short-screen capture is repeated, excluded or reclassified. The original conditional seven-block stage is not executed.

Confirmation uses nine blocks over all nine shapes at32workers plus primary16:31 timed requests after3 primes at32,151 after3 primes at16. Three separate warm-gated199Hz primary32 profiles follow **all** latency captures. The old screen and new matrix are analyzed separately; pooled primary ratios are retained only as sensitivity analysis because the request lengths differ.

The new rule uses distribution-free, two-sided median intervals with at least95% coverage under independent/exchangeable block assumptions. For nine observations, endpoints are the second-smallest and second-largest, giving96.09375% nominal coverage under those assumptions. Every value remains in the data and determines the order statistics; no outlier is deleted. These are individual intervals, not simultaneous guarantees across all comparisons.

Calibration requires the fresh/accepted interval wholly within factor1.03. Candidate nonregression requires an upper bound≤1.02 against both controls in every case. Primary32 improvement requires an upper bound<1 against both. The original-eight within-block geometric aggregate requires an upper bound≤1 against both. These prospectively declared checks do not rewrite the failed screen.

|Failed confirmation check|Observed interval or upper bound|Required|
|---|---:|---:|
|Primary16 control calibration|[0.876845,1.004542]|within[0.970874,1.03]|
|Primary32/accepted improvement|upper1.007150|<1|
|Primary32/fresh improvement|upper1.008242|<1|
|Primary16/accepted nonregression|upper1.035755|≤1.02|
|Primary16/fresh nonregression|upper1.061152|≤1.02|
|8×2/accepted nonregression|upper1.026323|≤1.02|
|Original-eight/accepted aggregate|upper1.002214|≤1|

The original-eight/fresh aggregate passes its individual check: median0.991250, interval[0.987170,0.994387]. This does not waive the accepted-control failure. [Full analysis](artifacts/performance-analysis.json) retains every block ratio, interval, failure and pooled sensitivity result. Interval failures express insufficient support for the declared bounds; they do not by themselves establish a causal slowdown.

## Supplied M5 comparisons

Historical supplied CPU/Metal values; no new M5 measurement or54 reference.

|Shape|Tree ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|
|---|---:|---:|---:|---:|---:|
|12x2|34.100319|284|8.328×|101|2.962×|
|16x2|40.410350|335|8.290×|124|3.069×|
|1x2|8.624067|37|4.290×|24|2.783×|
|24x2|67.604363|528|7.810×|176|2.603×|
|2x2|11.743912|59|5.024×|30|2.555×|
|4x2|16.130545|109|6.757×|48|2.976×|
|8x2|24.843198|189|7.608×|74|2.979×|
|merge-24x1|66.903513|507|7.578×|175|2.616×|
|merge-54x1|163.651863|—|—|—|—|

**0/8** supplied shapes reaches the CPU10× threshold. These are experimental timings; accepted production is unchanged.

## Profiles and memory observations

Three separate primary32 captures contain28,742 cycle samples: accepted9,505, fresh9,393 and candidate9,844. Each samples one process with zero lost records. Candidate category shares are G1 arithmetic41.20%, G2 arithmetic26.94%, inverse6.90%, epoch9.39% and FFT4.40%. These exclusive sampled-cycle categories do not give removable wall-time fractions or isolate the cause of a latency difference. Decoded profile records and capture receipts are archived; raw recordings remain local.

At54→1, measured whole-process peak RSS is1,932.20MiB for accepted,1,881.02MiB for fresh and1,818.89MiB for the candidate. CPU consumption is4,851.15/4,898.41/4,487.11ms per request respectively. These observations support retaining the large-shape result, without establishing minimum RAM or a universal memory benefit. Primary32 candidate peak RSS is113.50MiB versus112.75MiB for accepted.

## Correctness, memory and limits

All110 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. The new oracle performs240 full-buffer layer comparisons:24 ascending/descending lengths through2048, five ordinary/identity/doubling/opposite/mixed patterns, two inversion modes and seven empty/odd/even buckets. Expected points use complete scalar curve formulas; every output count and live point is checked, with scratch reuse and unused old-record-buffer assertions. Existing indexed/weighted/generator oracles remain. Field source and its accepted qualification are inherited unchanged.

The prefix Vec replaces the allocated native G2 Group Vec. Its payload is two320-byte vectors per eight ordinary pairs versus nine previously. One additional empty Vec header exists in Scratch (24bytes on this target); struct padding and allocator behavior are additional considerations. Lower source record bytes are not measured DRAM traffic, whole-process savings or a minimum-RAM claim. Typed workspace forecasts still use the larger record bound; formal metadata/resource/pressure qualification remains necessary before adoption.

The first setup helper stopped on an exact-match assertion before writing a candidate file: the same reserve expression appeared in tree and streaming code. A tree-specific anchor repaired setup before any native build or SDK call. The original helper and correction note are retained. No arithmetic, protocol or threshold changed in that repair.

The confirmation helper initially failed compilation on two malformed float literals, repaired before preparation or capture. Pre-capture review also replaced a pin on the mutable combined-inventory output with the immutable screen-inventory snapshot. Both superseded helper/plan versions and correction notes remain archived; no capture used them. The compiled confidence helper's rank/coverage self-check passes.

Preflight covers all nine candidate shapes at four workers, one timed request plus three primes:36 SDK calls/nine exports. All subsequent exported proofs pass independent Gnark verification,164-byte length, pinned public binding and freshness against7,260 prior identities and new exports. Helpers compile before captures; no helper/build/hash/archive/verifier overlaps timed processes.

Total **13578 SDK requests**, **300 distinct independently verified exports**, **600 proof/public members**; proof ledger **7,260 → 7560**. All170 production files and the accepted SDK remain unchanged. The all-eight CPU10× objective remains active and unproven.

No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed.

The next kernel question is whether layer size changes the relative cost of reloading coordinates and retaining records. An isolated complete-buffer caller can measure that tradeoff before any new structural dispatch rule is selected. The present results do not justify a posthoc circuit/core selector, repeated confirmation, or assuming that source bytes explain measured cache behavior. The24×2 and54→1 positives remain useful evidence; the current candidate is retained without global adoption.

reused fresh executable: `/tmp/zopro-poseidon-fused-dot/native/fresh-sdk`; SHA-256 `d847ae9fd2ae52c6c74172a95004780eaa5481ca2a5f8889d7c9d0063dcd1eaa`.

tree executable: `/tmp/zopro-g2-tree-records/native/tree-sdk`; SHA-256 `d8dc7ed028c9112f4c20a8b1847c23f27f8593097e12f450bfa5d4d53a22514a`.

[Design](artifacts/design.md), [source audit](artifacts/implementation-audit.json), [edits](artifacts/edits.json), [context audit](artifacts/context-audit.json), [screen analysis](artifacts/screen-analysis.json), [native tests](artifacts/native/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).

# Bounded G2 prefix-tree SDK route on C7a

The declared whole-SDK gate **fails**. No candidate is adopted; production remains unchanged. This tests whether the [positive high-core component result](../2026-10-26-g2-tree-c7a/README.md) improves complete verified proofs, including comparison against the accepted 32-worker primary configuration.

For 1x2, candidate ON at 64 workers takes 12.698 ms versus accepted Zopro's 12.327 ms at 32 workers. The paired comparison is 3.06% slower, with zero of three winning blocks. ON improves the 96-worker result against OFF by 7.56%, but its 13.331 ms still exceeds the accepted 32-worker result. Aeglos's best tested primary result is 8.121 ms at 96 workers. The component improvement did not establish a better best-tested primary prover latency.

Seven control calibration checks fail. All observations remain included; fine-grained general speedup claims are withheld. Differences on larger, tree-ineligible queries cannot be attributed to tree arithmetic from this experiment.

The optional route reuses the tested tree math and prefix-addition kernel unchanged. It requires explicit enablement, a native IFMA build with current prefix records, direct additions, at least 64 pool workers and an effective G2 query size from 1,024 through 65,535. Only exclusively owned scalar buffers can enter. Existing scalar grouping runs first if selected. Small pools, unsupported configurations or sizes, and denied admission retain the current stream. Full typed temporary capacity, including the original raw-query floor and all 22 potential parent jobs, is reserved before tree allocation or scalar mutation. The query-local RAII token releases after temporaries finish. The existing shared policy accounts for active proof forecasts and optional charges. There is no new pool, permanent table or universal memory ceiling. The conservative duplicate raw floor can favor fallback under pressure.

## Complete verified request latency

Dedicated c7a.24xlarge, 96 physical cores without SMT, 30 GiB container; compact first-N CPU masks. Three rotated/reversed fresh-process blocks, with 31 timed calls for primary and 17 for other shapes after three primes. Loading and prepared-key setup are excluded; witness solving, fresh randomness, proving, SDK verification and serialization are timed. Accepted and candidate target `znver4`; OFF/ON share one executable. Aeglos uses the pinned Sorted/prepared-key policy. No owned build, profile or archive overlaps latency. All 324 latency processes precede eight profiles. Times below are medians of process medians; paired ratios use within-block geometric means.

|Circuit|Workers|Accepted ms|OFF ms|ON ms|Aeglos ms|ON/OFF|ON/accepted|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|32|12.327467|12.335157|12.397067|11.990992|1.001742|1.002285|
|1x2|64|12.784721|12.784619|12.698251|9.187491|0.992000|0.995431|
|1x2|96|14.177135|14.445377|13.331162|8.120965|0.924366|0.932630|
|2x2|32|16.534106|16.188914|16.563336|18.721998|1.009379|1.006316|
|2x2|64|17.224198|17.562511|17.045589|13.098818|0.962255|0.975904|
|2x2|96|19.205998|18.466825|18.363565|11.869292|0.979893|0.946726|
|4x2|32|19.946742|20.091795|20.270543|30.746909|1.006030|1.012987|
|4x2|64|20.883218|20.319313|20.690867|20.869596|1.014653|0.993275|
|4x2|96|22.449804|22.754109|23.019650|18.539535|1.002144|1.009913|
|8x2|32|27.853547|27.867449|28.228771|55.286648|1.003230|1.013697|
|8x2|64|27.431097|27.259586|27.188162|36.449721|0.994240|0.996988|
|8x2|96|29.868039|28.706485|29.134145|31.021124|1.012284|0.978288|
|12x2|32|37.242905|36.982966|36.754592|86.630655|0.994915|0.990456|
|12x2|64|35.494189|35.230074|35.303779|56.664661|1.000611|0.994100|
|12x2|96|37.523517|38.800447|38.112562|48.735144|0.991986|1.021557|
|16x2|32|42.744848|43.556689|43.072630|105.079264|0.988296|1.007427|
|16x2|64|40.432734|39.357317|40.327742|65.967291|1.017317|1.002744|
|16x2|96|43.334487|43.707403|43.605492|56.998002|0.997841|1.007180|
|24x2|32|61.634814|63.597548|63.836444|169.591625|0.988911|1.024037|
|24x2|64|56.843918|57.211370|58.205996|106.261838|1.020415|1.025747|
|24x2|96|62.497396|62.727985|62.495504|87.172692|0.996204|1.003685|
|merge-24x1|32|64.636749|64.829236|64.067799|164.559917|0.981871|0.991688|
|merge-24x1|64|60.221501|60.323569|60.429720|102.909038|1.002220|1.002887|
|merge-24x1|96|65.735140|66.162081|67.212245|85.133963|1.016017|1.014503|
|merge-54x1|32|139.500912|139.196067|140.780146|343.497238|1.010128|1.009381|
|merge-54x1|64|127.514558|127.299441|125.965640|209.332636|0.994504|0.993959|
|merge-54x1|96|134.377393|135.663391|133.870393|174.936214|0.989452|1.001171|

The predeclared primary ON64/accepted32 paired geometric ratio is **1.030560**, with 0/3 block wins. This cross-worker requirement prevents qualification based only on a degraded high-core stream. It is an experimental comparison, not an adopted worker-selection policy.

## Best tested configurations and supplied M5 references

Best-tested choices are descriptive. Whole-process RSS includes loading/setup/primes and is not minimum RAM or incremental scratch. Summed CPU includes coordination and is not an arithmetic operation count. Supplied M5 values are historical references; these comparisons do not establish adoption.

|Circuit|Accepted ms/workers|ON ms/workers|Aeglos ms/workers|Accepted/ON/Aeglos RSS MiB|Supplied CPU/Metal ms|ON CPU/Metal speedup|
|---|---:|---:|---:|---:|---:|---:|
|1x2|12.327/32|12.397/32|8.121/96|101.5/97.6/460.7|37/24|2.98×/1.94×|
|2x2|16.534/32|16.563/32|11.869/96|231.4/223.9/806.0|59/30|3.56×/1.81×|
|4x2|19.947/32|20.271/32|18.540/96|278.6/270.5/1354.0|109/48|5.38×/2.37×|
|8x2|27.431/64|27.188/64|31.021/96|464.1/471.5/2452.7|189/74|6.95×/2.72×|
|12x2|35.494/64|35.304/64|48.735/96|622.8/664.1/3652.1|284/101|8.04×/2.86×|
|16x2|40.433/64|40.328/64|56.998/96|767.4/797.7/4612.0|335/124|8.31×/3.07×|
|24x2|56.844/64|58.206/64|87.173/96|1205.0/1251.1/6763.4|528/176|9.07×/3.02×|
|merge-24x1|60.222/64|60.430/64|85.134/96|1144.5/1141.3/6544.7|507/175|8.39×/2.90×|
|merge-54x1|127.515/64|125.966/64|174.936/96|1904.0/1911.2/13007.1|not supplied|—|

0/8 best-tested candidate rows reach the supplied CPU 10× threshold. The original all-eight objective remains unproven and active.

Primary summed CPU consumption also does not show less total CPU work. At 96 workers, accepted Zopro consumes 739.233 CPU ms/request and ON consumes 776.850 CPU ms/request. Their whole-process peak RSS values are 105.980 and 114.262 MiB, respectively. These include allocator and coordination behavior; they do not isolate arithmetic counts or minimum RAM.

## Frozen qualification

OFF/accepted must stay within factor 1.05 each block and 1.03 paired median for all 27 cases. Primary at 64 and 96 workers must improve against both controls with at least 2/3 wins each, and primary ON64 must improve against accepted32 with at least 2/3 wins. Every other case, including primary32, must remain within 1.02 against both controls. Original-eight aggregates must be <=1.00 at each worker count against both controls. There is no minimum gain floor, posthoc core selection, exclusion or repeated successful capture. All failures follow.

calibration_failures:

- 2x2-w96 b2 OFF/accepted=0.9309025324877809
- 2x2-w96 paired OFF/accepted=0.9672648432588695
- 8x2-w96 paired OFF/accepted=0.9601370438121086
- 12x2-w96 paired OFF/accepted=1.0308142427450842
- 24x2-w32 b2 OFF/accepted=1.051687608290127
- 24x2-w32 paired OFF/accepted=1.0336849801910852
- merge-24x1-w32 b0 OFF/accepted=1.0508269832187724

primary_failures:

- primaryON64/accepted32=1.0305599854357161,wins0/3

regressions:

- 12x2-w96 ON/accepted=1.021557398274257
- 24x2-w32 ON/accepted=1.0240369184073264
- 24x2-w64 ON/OFF=1.0204152462079106
- 24x2-w64 ON/accepted=1.0257471857756735

aggregate_failures:

- w32 ON/OFF=0.9967546664885241, ON/accepted=1.0060560701723074
- w64 ON/OFF=1.0003085623198342, ON/accepted=0.998300525749443

## Validation and profiles

All 117 unfiltered native core tests, raw2/cubic6 and generated-IFMA checks pass. Tests cover complete-curve signed/odd/skewed bucket cases, independent generator sums including 30,089 points, checked cost overflow, route boundaries, denial without scalar mutation and successful 64-worker route charge release. Portable x86-64 and native-noIFMA core/prover/zolana consumer compilation passes. Field math is unchanged; the 33-test target qualification is inherited. Compilation is not phone performance evidence.

Exact-container preflight covers all nine shapes and four engines at four workers, OFF/ON for 1x2/2x2 at 64 workers, and one primary ON64 with explicit optional-budget ceiling zero. The 64-worker local checks oversubscribe this 32-logical-CPU host and are correctness-only. All 41 exports independently verify; route traces confirm selection only on eligible ON64 cases and fallback elsewhere. This budget check does not establish a minimum RAM requirement or giant-circuit memory-pressure guarantee.

Combined 7,580 SDK requests produce 373 distinct Gnark-verified exports and 746 proof/public members. Every export verifies again locally after transport across nine pinned keys. Reverification is counted once. Fixture public hashes, 164-byte length and freshness against 5,958 prior identities pass; the new prior set is 6,331. All source/image/input/executable hashes and route receipts are retained.

Eight primary32/96 profiles, one per engine/count, follow all latencies. Each has 51 timed calls after three primes, cycles:u at 199 Hz, gated after preparation through direct pinned perf with `--no-buildid-cache`. There are 68,699 samples, zero lost records and one sampled process per profile. Categories may contain inlined arithmetic and are not exclusive-stage wall time. The displayed symbol categories target Rust paths; dashes for Aeglos mean these categories do not apply. They do not imply an absence of coordination or grouping work. Raw profiles stay local with hashes; decoded reports are archived.

|Profile|Samples|Lost|Epoch+scheduler %|Grouping %|Tree %|
|---|---:|---:|---:|---:|---:|
|1x2-w32/accepted|5239|0|12.291|0.662|0.000|
|1x2-w32/aeglos|3670|0|—|—|—|
|1x2-w32/off|5591|0|13.227|0.556|0.000|
|1x2-w32/on|3474|0|13.344|0.575|0.000|
|1x2-w96/accepted|11890|0|61.717|0.301|0.000|
|1x2-w96/aeglos|5907|0|—|—|—|
|1x2-w96/off|16558|0|62.888|0.175|0.000|
|1x2-w96/on|16370|0|59.833|0.158|6.758|

Topology/CPU flags and cgroup CPU, memory and swap snapshots are retained before the workflow, after all latency and after profiles. The after-latency snapshot is written by the first profile adapter before SDK preparation and before the disabled profiler is enabled. The latency stage records six CPU-throttling events totaling 49,731 microseconds; no further events occur in the profile stage. Memory limit/high/OOM events and container swap use remain zero. These stage counters include complete processes and verification; they cannot identify a cause for an individual timing or calibration failure. No concurrent-throughput, phone, TEE, giant or swap qualification is claimed.

This closes the tree integration experiment. Exposing more parallel G2 work did not reduce best-tested primary latency or summed CPU consumption. Further tree-width or idle-worker tuning is not justified by this result. The next work should target arithmetic work or the remaining preparation/critical path, with the accepted best worker configuration kept as a control. These profiles do not establish DRAM bandwidth saturation or a single cause for Aeglos's latency advantage.

## Cleanup and artifacts

The owned instance is terminated; its EBS volume, temporary ECR tag, private S3 object/bucket are absent, and the presigned parameter file is deleted. No live temporary benchmark instance remains in the region. Production source, networking and IAM remain unchanged.

[Design](artifacts/design.md), [implementation audit](artifacts/implementation-audit.json), [full analysis](artifacts/analysis.json), [all request/profile rows](artifacts/remote-results/output/measure/performance/rows.json), [route checks](artifacts/local-output/preflight/route-analysis.json), [proof inventory](artifacts/proof-inventory.json), [posttransport Gnark log](artifacts/post-transport-gnark.log), [cleanup](artifacts/cleanup-complete.json), and [artifact index](artifacts/artifact-index.json) retain the evidence. Keys, executables and raw profiles remain local with hashes. Secret transport files are excluded.

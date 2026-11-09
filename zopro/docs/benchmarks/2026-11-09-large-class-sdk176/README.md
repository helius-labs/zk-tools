# Oversized scalar-class splitting: complete-proof comparison

**The candidate is slower overall and is not adopted.** Across all eight shapes, ON/OFF is **1.003144 [1.001097, 1.005262]** and ON/accepted REF is **1.003705 [1.001757, 1.005717]**: increases of 0.314% and 0.371%. Both required aggregate improvement checks fail. All other 85 declared checks pass, including every control calibration, nonregression tolerance, RSS and diagnostic-quality check. Production remains unchanged.

The [native-qualified implementation](../2026-11-09-large-class-native176/README.md) splits scalar classes larger than the existing 2,048-point G1 / 1,024-point G2 tile into independent unchanged affine trees, then combines partial roots in the original 256-class output organization. Short classes keep their original inputs. Checked optional admission covers all new metadata/partials and first-pass scratch; denial retains the original reducer. There is no retained key table, expanded point representation, new pool, changed digit geometry or artificial architectural memory cap. Extra tasks, partial roots and a second reduction pass remain costs.

Exact qualified OFF/ON cores are reused without rebuilding. Six downstream commands link coherent prover/zolana/SDK artifacts with original field1200, adopted bitmap reset and the unchanged fixed-placement driver, native/O3/fat-LTO/one-CGU. REF is the exact accepted SDK. All 176 production-source hashes recover; 175 private originals are exact, one parent is reversible and two children are new. Explicit LARGE_CLASS_TILES=1 is set in every arm; only ON compiles the reducer. Native correctness/generic checks are inherited, not rerun.

The protocol and four compiled readers were frozen before any preflight. The experiment prospectively uses all-eight aggregate gains rather than primary-only gain checks because retained primary grouping declines. Primary remains subject to both nonregression/RSS checks and control calibration, and participates in the all-eight aggregate. This changes the experiment's selection test, not the full goal: final success still requires verified CPU10× on every supplied shape. No observation informed this rule.

All-eight plus merge54 at 32 workers, with primary/merge54 at one and 16 workers, pass 26 functional processes and 26 independent Gnark checks before timing. The 104 functional requests establish correctness only. The timing matrix has 11 conditions: all-eight plus merge54 at32, plus primary solo/16. Primary16/32 use 31 blocks of151 requests; other conditions use15 blocks of31, except solo11. Arm and case order rotate and reverse. Each request solves, proves with fresh randomness, verifies internally and serializes; loading, pool creation, three primes and final export are outside timing. Worker affinity is CPU2 solo,0–15 at16,0–31 at32, with before/after worker index/TID/CPU and N+1-task bindings.

Times below are medians of process means. Ratios pair blocks and need not equal quotients of displayed medians. Bounds are the frozen20,000-resample paired bootstrap95% intervals; aggregate bounds resample shapes independently with equal log weight. They have no simultaneous familywise guarantee. All observations remain.

|Case|REF ms|OFF ms|ON ms|ON/OFF [95% bounds]; wins|ON/REF [95% bounds]; wins|
|---|---:|---:|---:|---|---|
|1x2-w32|7.829286|7.843064|7.841611|1.000152 [0.998564, 1.001786]; 16/31|1.002990 [1.001441, 1.004656]; 10/31|
|2x2-w32|10.979335|10.970872|10.974760|0.998885 [0.993649, 1.004357]; 8/15|0.997186 [0.993957, 1.000417]; 9/15|
|4x2-w32|15.803969|15.726226|15.806487|1.008809 [1.004751, 1.013134]; 2/15|1.001263 [0.993444, 1.008835]; 6/15|
|8x2-w32|23.944017|23.987995|24.053065|1.004695 [0.995936, 1.013970]; 5/15|1.008522 [1.000992, 1.017138]; 4/15|
|12x2-w32|33.703579|33.848617|33.923575|1.000785 [0.994315, 1.006820]; 7/15|1.003832 [0.998119, 1.009439]; 5/15|
|16x2-w32|39.858478|39.797191|39.894964|1.000390 [0.996659, 1.004300]; 9/15|1.001309 [0.996900, 1.006230]; 8/15|
|24x2-w32|66.186709|66.121120|66.304409|1.004425 [0.997982, 1.011380]; 6/15|1.005489 [0.999999, 1.011455]; 4/15|
|merge-24x1-w32|65.489902|65.550756|66.135175|1.007059 [0.999920, 1.014400]; 4/15|1.009104 [1.004160, 1.014225]; 3/15|
|merge-54x1-w32|162.541995|162.796535|164.239251|1.011183 [1.005569, 1.016578]; 3/15|1.010871 [1.006060, 1.015516]; 2/15|
|1x2-w1|94.650410|94.933429|94.632998|0.995996 [0.993820, 0.998250]; 13/15|0.999123 [0.997234, 1.001145]; 9/15|
|1x2-w16|9.192237|9.221424|9.225498|0.998785 [0.997408, 1.000184]; 18/31|1.004158 [1.002705, 1.005638]; 5/31|

Primary32 is essentially unchanged against fresh OFF, while its ON/REF estimate is 0.299% slower. The solo ON/OFF gain of0.400% is retained, but its ON/REF interval crosses parity. No candidate-namespace samples occur in the primary profiles; the retained grouping population and source dispatch explain why primary can be a negative control. These observations do not attribute the solo difference to class-splitting arithmetic.

The all-eight slowdown intervals are wholly above parity. The4×2 ON/OFF estimate is0.881% slower, and merge54 is1.118% slower. Passing the3% nonregression tolerance does not assert equality or improvement. Only the two all-eight gain checks fail; no control-calibration failure explains this result. Close this fixed implementation without a size/worker/curve/layout sweep or favorable-subset promotion.

|Case|REF /OFF /ON maximum RSS MiB|REF /OFF /ON median CPU ms per proof|
|---|---:|---:|
|1x2-w32|133.50 /133.25 /133.75|178.446 /178.013 /178.003|
|2x2-w32|282.00 /271.00 /272.75|258.413 /257.621 /258.465|
|4x2-w32|312.00 /307.50 /307.00|419.955 /419.584 /420.226|
|8x2-w32|499.25 /508.50 /497.50|674.267 /673.444 /674.460|
|12x2-w32|712.25 /718.75 /717.75|946.615 /948.718 /949.482|
|16x2-w32|821.75 /811.75 /821.25|1135.428 /1132.850 /1133.663|
|24x2-w32|1285.25 /1291.56 /1305.00|1926.487 /1927.504 /1931.647|
|merge-24x1-w32|1179.25 /1159.96 /1149.68|1847.303 /1848.588 /1860.720|
|merge-54x1-w32|1899.15 /1925.27 /1921.99|4526.882 /4523.287 /4566.036|
|1x2-w1|34.25 /34.00 /34.25|94.648 /94.932 /94.632|
|1x2-w16|88.75 /89.50 /88.75|117.730 /117.962 /117.497|

RSS includes key loading, primes and allocator retention. It is not incremental workspace, minimum RAM or pressure qualification. All22 declared ON/control RSS guards pass. The existing admission/fallback checks do not replace a complete proof under memory pressure.

All591 latency processes finish before15 warm four-event PMU panels and15 separate fixed16M cycles:u profiles. Diagnostics cover primary16/32 and2×2/4×2/24×2 at32. No owned build/helper/hash/decode/analysis/report/push overlaps recording, including preflight. All621 recording processes finish before75 offline decode/ownership/event/loss commands and621 Gnark checks.

|Diagnostic case|ON/OFF instructions|ON/OFF cycles|ON/OFF cache misses|ON/OFF branch misses|ON candidate-namespace samples|
|---|---:|---:|---:|---:|---:|
|1x2-w32|0.999589|0.998316|1.020025|0.993146|0|
|1x2-w16|1.000322|1.000402|0.965926|1.058650|0|
|2x2-w32|1.000444|1.004434|1.010035|1.001478|278|
|4x2-w32|1.004008|0.968936|0.964670|0.999751|1044|
|24x2-w32|1.002391|1.000619|0.997850|0.984485|4202|

All15 PMU panels satisfy the99.9% running gate. All15 profiles qualify: **625414 samples**, minimum8821, maximum unknown share2.902%, zero LOST, with actual binary/PID/TID/CPU and fixed-period bindings. Candidate-specific samples at2×2/4×2/24×2 establish sampled execution there; absence in primary is not proof of no execution. The separate4×2 PMU panel shows fewer cycles despite slower repeated latency, while instructions rise slightly. These single descriptive panels and flat symbol bins do not isolate the critical path, a cache level, exact inverse counts, memory bandwidth or a unique cause.

The87-check gate requires11 OFF/REF calibration intervals within factor1.03,22 ON/control nonregression upper bounds≤1.03,22 ON/control peak-RSS ratios≤1.05, both all-eight gain upper bounds<1, and30 diagnostic-quality checks. Source, receipt, placement, input, public, proof and freshness bindings are additionally mandatory. No gain floor, excluded block, extension, changed threshold or reroll is used.

The frozen separate reader reconstructs591 raw process means, every paired/aggregate ratio, all87 decisions and621 fresh main exports. It shares the inherited statistical/PMU/profile parsers, not native arithmetic. Across preflight and main, **45,938 internally verified SDK requests and647 distinct independently Gnark-verified final exports** advance the canonical inventory43,083→43,109→43,730. These are fixed input fixtures with fresh randomized proofs, not45,938distinct witnesses.

All1,379 workflow commands complete successfully once: six downstream builds, four helper builds,52preflight and1,317main commands. An initial standalone linker-review preparation failed because its local audit copy was missing; supplying the exact already-qualified audit fixes that dependency. No scientific code, protocol, successful build, test or measurement is replayed. Setup, link/analysis/read/publication helper invocations are outside the workflow-command count.

The full CPU10× goal remains active and unmet, **0/8**. The following fresh accepted-build comparison uses supplied M5 CPU/Metal references on different hardware; they were not remeasured. Cross-machine ratios do not establish equal hardware efficiency.

|Shape|Accepted CPU ms|Candidate CPU ms|M5 CPU ms /accepted speedup|M5 Metal ms /accepted speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|7.829286|7.841611|37.0 /4.726×|24.0 /3.065×|3.7|
|2x2|10.979335|10.974760|59.0 /5.374×|30.0 /2.732×|5.9|
|4x2|15.803969|15.806487|109.0 /6.897×|48.0 /3.037×|10.9|
|8x2|23.944017|24.053065|189.0 /7.893×|74.0 /3.091×|18.9|
|12x2|33.703579|33.923575|284.0 /8.426×|101.0 /2.997×|28.4|
|16x2|39.858478|39.894964|335.0 /8.405×|124.0 /3.111×|33.5|
|24x2|66.186709|66.304409|528.0 /7.977×|176.0 /2.659×|52.8|
|merge-24x1|65.489902|66.135175|507.0 /7.742×|175.0 /2.672×|50.7|

Accepted primary peak RSS is133.50MiB in this campaign; merge54 is162.541995ms without a supplied M5 reference. The previous7.803404ms primary observation belongs to the same accepted artifact; no slower replacement was adopted. There is no new AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-proof throughput result. Next research should remove useful bulk arithmetic or dependencies; changing large-class task organization alone did not improve complete proofs.

[Full analysis](artifacts/analysis.json), [independent review](artifacts/review-result.json), [CPU/Metal comparisons](artifacts/fresh-comparison.json), [CPU diagnostics](artifacts/diagnostic-summary.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [prospective review](artifacts/prospective-review.json), [source recovery](artifacts/source-audit.json), [coherent links](artifacts/link-review.json), [preflight](artifacts/preflight-review.json), [main export inventory](artifacts/proof-inventory.json), [preparation note](artifacts/preparation-note.md), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).

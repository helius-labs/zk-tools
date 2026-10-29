# Cached-hash grouping with affine collision-queue draining

29 October 2026. A complete-request experiment combining the exact two earlier candidates. Production remains unchanged. The earlier parent gates remain failed. This report does not assume additive gains or causal independence.

Declared full-request performance gate: **false**. Failed checks: `["8x2-w32/ON/accepted","1x2-w16/OFF/accepted"]`.

**The combined candidate proves local 1×2 in 7.846378 ms with 87.00 MiB peak RSS**, versus 8.037538 ms and 135.25 MiB for the fresh accepted control. The paired latency gain is **2.471% versus accepted** and **2.210% versus OFF**, with 15/15 wins against each; observed primary RSS falls **35.675% versus accepted**. Both speed and memory improvements survive in the same complete-request executable.

The original-eight equal-log latency aggregate improves **1.213% versus accepted** and **1.060% versus OFF**, both with 15/15 wins. On 54→1, ON measures **157.582356 ms**, a paired **3.079% gain versus accepted** and **2.170% versus OFF**, again 15/15 wins. Maximum process RSS is lower on all nine circuits versus accepted. Relative RSS guards pass against both controls for all ten cases; this is not a minimum-RAM or memory-pressure qualification.

Two checks keep the candidate unadopted. The 8×2 ON/accepted ratio is 1.006386443 [0.994091917, 1.023994351], exceeding the 1.02 nonregression upper limit. It spans parity and does not establish an actual 2.4% slowdown. Physical16 ON/control comparisons pass nonregression, with upper ratios below 1, but its OFF/accepted calibration interval is [0.971699951, 1.125981046], outside factor 1.03. That is uncertainty in the fresh-control comparison, not a measured 12.6% candidate regression. Every other declared check passes. No threshold, included observation or acceptance decision changes after measurement.

Single diagnostic panels record about 3.3% fewer retired instructions versus OFF at both worker counts; every counter event has 100% running coverage. Six profiles pass their attribution and zero-loss checks. Known absolute-period accounting limits remain, so sampled shares cannot establish an exclusive cost or causal explanation. There is no new AWS measurement of this candidate.

## Implementation and protocol

The constructor uses compact representative rows/counts and caches hashes only while building a proof-local table, releasing the table and hash vector before scalar materialization. The borrowed G1/G2 queue drain admits suitable SIMD-width distinct destinations to the original affine batch before retaining the original projective fallback. These are the exact earlier implementations, without additional data structures. They add no persistent per-key table, point-array copy, execution plan or worker pool. Original conservative grouping and MSM workspace forecasts remain. No minimum-RAM or memory-pressure claim follows. [Original grouping experiment](../2026-10-29-cached-hash-sdk175/README.md), [original queue experiment](../2026-10-29-queue-drain175/README.md).

Measurements use a shared Ryzen 9 9950X with 16 physical cores and 32 logical CPUs. All nine circuits use 32 workers on CPUs 0–31; primary also uses 16 workers on CPUs 0–15. Fifteen fixed blocks rotate/reverse case and variant order. Accepted is the exact production executable; OFF and ON are coherent fresh builds, and ON enables both changes. Each process times 151 primary or 31 other requests after three primes. Timers include fresh solving, randomized proving, SDK verification and serialization. Loading and preparation are outside latency and inside lifetime RSS. Latency is the median of process medians; CPU is median summed service per request; RSS is maximum lifetime process high-water, not minimum RAM.

Thirty preflights independently verify with Gnark before latency. All 450 latency captures precede six grouped PMU panels and six fixed-period profiles on primary at 16/32 workers. All captures precede remaining independent verification and analysis. No owned compiler, hash/archive/analyzer helper or independent workload overlaps capture; every original helper is compiled and pinned beforehand.

## Complete requests

|Case|Variant|Median ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|
|12x2-w32|OFF|33.921189|942.494|708.50|
|12x2-w32|ON|33.447728|931.622|673.00|
|12x2-w32|accepted|33.836735|938.988|703.75|
|16x2-w32|OFF|40.157244|1124.433|815.25|
|16x2-w32|ON|39.836068|1120.136|816.25|
|16x2-w32|accepted|40.185346|1126.523|822.50|
|1x2-w16|OFF|9.672253|119.439|90.75|
|1x2-w16|ON|9.423573|115.956|66.00|
|1x2-w16|accepted|9.768923|120.382|91.00|
|1x2-w32|OFF|8.017230|181.573|134.75|
|1x2-w32|ON|7.846378|176.910|87.00|
|1x2-w32|accepted|8.037538|182.492|135.25|
|24x2-w32|OFF|65.884075|1885.131|1316.10|
|24x2-w32|ON|65.198557|1876.907|1211.75|
|24x2-w32|accepted|66.410932|1889.302|1290.75|
|2x2-w32|OFF|11.270167|260.854|273.00|
|2x2-w32|ON|11.123419|258.085|220.00|
|2x2-w32|accepted|11.282220|262.576|269.25|
|4x2-w32|OFF|15.960534|419.590|317.50|
|4x2-w32|ON|15.701353|412.383|292.00|
|4x2-w32|accepted|15.987681|419.663|310.75|
|8x2-w32|OFF|24.522912|676.229|502.25|
|8x2-w32|ON|24.323188|666.033|489.50|
|8x2-w32|accepted|24.108389|667.596|515.00|
|merge-24x1-w32|OFF|65.128227|1810.142|1186.81|
|merge-24x1-w32|ON|64.705300|1795.924|1142.52|
|merge-24x1-w32|accepted|65.249697|1809.574|1190.25|
|merge-54x1-w32|OFF|160.922362|4427.465|1852.27|
|merge-54x1-w32|ON|157.582356|4360.069|1837.85|
|merge-54x1-w32|accepted|162.698435|4458.645|1911.56|

|Paired comparison|Median ratio|Bounds|Wins /15|Pass|
|---|---:|---|---:|---|
|12x2-w32/OFF/accepted|1.002268819|[0.991840497, 1.013943052]|7|true|
|12x2-w32/ON/OFF|0.983108651|[0.975111687, 0.997116904]|13|true|
|12x2-w32/ON/accepted|0.988641238|[0.981349183, 0.991924336]|13|true|
|16x2-w32/OFF/accepted|0.997960902|[0.990673342, 1.006987827]|8|true|
|16x2-w32/ON/OFF|0.992002041|[0.977065675, 1.006901716]|11|true|
|16x2-w32/ON/accepted|0.991426771|[0.980653557, 1.006444673]|10|true|
|1x2-w16/OFF/accepted|0.997035126|[0.971699951, 1.125981046]|8|false|
|1x2-w16/ON/OFF|0.974035365|[0.875818941, 0.993295733]|12|true|
|1x2-w16/ON/accepted|0.965185415|[0.923258988, 0.989901102]|12|true|
|1x2-w32/OFF/accepted|0.998356998|[0.992266840, 1.000558333]|10|true|
|1x2-w32/ON/OFF|0.977900373|[0.973262215, 0.981483259]|15|true|
|1x2-w32/ON/accepted|0.975288972|[0.969703434, 0.980109857]|15|true|
|24x2-w32/OFF/accepted|0.995349643|[0.986131176, 1.003496666]|10|true|
|24x2-w32/ON/OFF|0.990584140|[0.983630296, 0.996704524]|13|true|
|24x2-w32/ON/accepted|0.986572586|[0.977057245, 0.993263679]|14|true|
|2x2-w32/OFF/accepted|0.994179656|[0.991395425, 1.000179740]|11|true|
|2x2-w32/ON/OFF|0.987499494|[0.979348759, 1.000211790]|11|true|
|2x2-w32/ON/accepted|0.984427622|[0.980019499, 0.992038189]|13|true|
|4x2-w32/OFF/accepted|0.995083333|[0.987578937, 1.001095181]|11|true|
|4x2-w32/ON/OFF|0.984730876|[0.982361482, 0.991594828]|13|true|
|4x2-w32/ON/accepted|0.985090141|[0.974186944, 0.992587963]|15|true|
|8x2-w32/OFF/accepted|1.011284061|[0.993395536, 1.029280683]|5|true|
|8x2-w32/ON/OFF|0.997615277|[0.980698085, 1.010944697]|8|true|
|8x2-w32/ON/accepted|1.006386443|[0.994091917, 1.023994351]|6|false|
|merge-24x1-w32/OFF/accepted|0.996244471|[0.989322647, 1.003136505]|9|true|
|merge-24x1-w32/ON/OFF|0.994409997|[0.984333407, 0.997241965]|13|true|
|merge-24x1-w32/ON/accepted|0.989315975|[0.986809682, 0.991656712]|14|true|
|merge-54x1-w32/OFF/accepted|0.992713503|[0.984036318, 1.001458189]|11|true|
|merge-54x1-w32/ON/OFF|0.978302938|[0.971835919, 0.983846404]|15|true|
|merge-54x1-w32/ON/accepted|0.969214350|[0.961708352, 0.979014826]|15|true|

|Original-eight aggregate versus|Ratio|Bounds|Wins /15|Pass|
|---|---:|---|---:|---|
|OFF|0.989400003|[0.985701749, 0.991341406]|15|true|
|accepted|0.987871629|[0.984986003, 0.990222215]|15|true|

Primary at 32 workers and the equal-log original-eight aggregate require upper ON/control ratios below 1 and at least 10/15 wins against both controls. Every other case, including primary at 16 workers and 54→1, requires upper ratios ≤1.02. All ten OFF/accepted calibration intervals must fit factor 1.03; every maximum RSS ON/control ratio must be ≤1.05. Paired ranks 4–12 of 15 give conditional nominal 96.484375% individual median coverage under independent/exchangeable blocks, without familywise coverage. No gain floor, excluded observation, extension, successful rerun, changed threshold or subset adoption applies. A passing screen still needs portable, resource and final-artifact qualification. [Frozen design](artifacts/design.md).

## Counters and profiles

Each counter panel is a single diagnostic observation, not an isolated cause or repeated-trial confidence interval. All four events require≥99.9% running coverage.

|Panel|Instructions/request million|Cycles/request million|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|1380.628|582.739|true|
|stat-1x2-w16-ON|1335.095|565.441|true|
|stat-1x2-w16-accepted|1381.999|588.620|true|
|stat-1x2-w32-OFF|1419.783|825.309|true|
|stat-1x2-w32-ON|1373.154|803.751|true|
|stat-1x2-w32-accepted|1419.095|825.632|true|

Profiles sample `cycles:u` at a fixed period of 16 million, recording CPU IDs, timestamps, period and executable mappings. Attribution checks require one PID, zero lost records, correct affinity/event/mapping, at least 1,000 samples and at most 5% unknown period. Known absolute-period undercoverage remains unresolved: these are qualitative flat shares, not exclusive costs, local CPI or removable wall fractions. LLVM can inline the queue drain; absent method-name period does not measure absent execution.

|Profile|Samples|Attribution qualified|Lost|
|---|---:|---|---:|
|cycles-1x2-w16-OFF|8886|true|0|
|cycles-1x2-w16-ON|8614|true|0|
|cycles-1x2-w16-accepted|8931|true|0|
|cycles-1x2-w32-OFF|12608|true|0|
|cycles-1x2-w32-ON|12081|true|0|
|cycles-1x2-w32-accepted|12631|true|0|

## Supplied CPU and Metal references

These are supplied historical M5 references, not new matched-hardware measurements. ON is experimental. There is no M5 reference for 54→1.

|Circuit|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× achieved|
|---|---:|---:|---:|---:|---:|---|
|12x2-w32|33.447728|284.0|8.491×|101.0|3.020×|false|
|16x2-w32|39.836068|335.0|8.409×|124.0|3.113×|false|
|1x2-w32|7.846378|37.0|4.716×|24.0|3.059×|false|
|24x2-w32|65.198557|528.0|8.098×|176.0|2.699×|false|
|2x2-w32|11.123419|59.0|5.304×|30.0|2.697×|false|
|4x2-w32|15.701353|109.0|6.942×|48.0|3.057×|false|
|8x2-w32|24.323188|189.0|7.770×|74.0|3.042×|false|
|merge-24x1-w32|64.705300|507.0|7.836×|175.0|2.705×|false|

## Correctness and provenance

OFF/ON pass 127/131 unfiltered native core tests and eight prover tests each. The seven added tests retain both parent suites: four constructor tests cover independent oracles, collisions, admission, hash counts and forecasts; three queue tests cover guards and independent G1/G2 bucket sums. They include compiled-route assertions, signed/repeated/identity/equal/opposite inputs and reuse. The 33 field tests are inherited without rerunning because all 17 field source files are exact. The 177 private files retain 169/175 originals byte-exact; independent inverse patches recover all 175 originals. Field arithmetic, workspace forecasts and Window fields remain exact. Immediate native freezing preserves coherent rlibs, proc macros, flags/features and the original SDK consumer.

All 1,014 workflow steps complete; 27,948 SDK requests verify internally. All 492 fresh 164-byte exported proofs independently pass Gnark, with exact public/image bindings and a common verification-key identity per shape. They are unique against 22,498 prior identities. Ledger 22,990 persists, with 984 proof/public archive members. This run adds no AWS/Aeglos, phone/ARM/TEE, forester 250/4999, swap/pressure/minimum-RAM or concurrent-throughput qualification.

[Complete source](artifacts/source.json.gz), [independent source review](artifacts/final-source-review.json), [all observations](artifacts/rows.json), [full analysis](artifacts/analysis.json), [capture plan](artifacts/plan.json), [all receipts](artifacts/receipts.json.gz), [proof bytes](artifacts/proofs.json.gz), [inventory](artifacts/proof-inventory.json), [archive review](artifacts/publication-review.json). ELF/perf stay local, hash-pinned.

The accepted 175-file source and all accepted local/AWS results remain unchanged. Accepted local 1×2 is 8.022165 ms with 134 MiB peak RSS; 54→1 is 162.250464 ms. Latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. No AWS claim transfers from this local candidate. The all-eight CPU 10× objective remains unmet on all eight shapes.

Before capture, two helper checks were corrected: unchanged binary fixtures are compared as bytes, and the expected request total includes 120 preflight requests (27,948 total). The original design accounting and failed helper sources remain archived. No capture had begun and no successful native work was repeated. [Corrections](artifacts/pre-capture-corrections.md).

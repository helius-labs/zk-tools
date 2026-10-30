# Full SDK qualification of bounded G2 delta multiplication

30 October 2026. Complete verified requests across all nine circuits and both primary worker counts. The candidate follows the [passed G2 caller gate](../2026-10-30-loose-delta175/README.md); its caller gain is not assumed to transfer to proving. Production remains unchanged.

The complete-request gate fails and the candidate is not adopted. At 32 workers, ON measures **8.168629 ms / 136.50 MiB peak RSS**, versus OFF’s **8.045739 ms / 134.75 MiB** and accepted’s **8.044768 ms / 134.75 MiB**. Paired latency is **1.61% slower versus OFF** and **1.52% slower versus accepted**. The original-eight aggregate is **0.34% slower versus OFF** and **0.45% slower versus accepted**. The passed caller improvement did not transfer to complete proofs.

Six checks fail: primary ON/accepted and ON/OFF gain; original-eight ON/accepted and ON/OFF gain; physical-16 OFF/accepted calibration; and physical-16 ON/accepted nonregression. All other nonregression/calibration checks and all relative RSS guards pass. Physical-16 candidate bounds span parity; they do not establish a slowdown equal to the upper endpoint. The 54→1 ON/OFF interval also spans parity. No samples were removed and no thresholds changed.

## Implementation and qualification

The final native G2 Y product uses a normalized local delta `D = 2p + anchor - X`, bounded by 3p for canonical anchor and X below 2p. The unchanged positive fused complex products are bounded by 12p². Since 4p < R256, REDC is below 4p; its original conditional subtraction of 2p restores the established output bound below 2p. Uniform R260 has enough headroom for the original uncorrected result. No loose operand escapes this helper. Two general modular-subtraction preparations become fixed-offset carry preparation; four raw products, two REDCs and 300 IFMA sites remain. Norms, inversion, slopes, square, X/final-Y subtraction, scalar tails and exceptional handling stay unchanged.

The SDK integration removes every common caller diagnostic adapter/export and adds an explicit ifma8 feature condition to all three native target-selection guards. Portable fallback evaluates the original expression. It uses accepted grouping, queues, task geometry and field operations; the separate unadopted grouping/queue-drain candidate is not included. No heap buffer, persistent key table, point copy, plan, pool, readiness dependency or workspace forecast is added. Stack frames and minimum RAM are not claimed identical.

Native OFF/ON each pass 124 unfiltered core and 8 prover tests. Portable ON passes 101 core and 8 prover tests with generic x86-64/no_ifma targeting. All 17 field parent files and the helper child are byte-exact to the qualified caller and inherit its 35 passed field tests and 103,424-case/206,848-output integer model. Removing the diagnostic caller cfg is the only field-build flag difference; that cfg is absent from every field file. These field tests/model runs are inherited, not repeated. Reversal recovers all 175 originals; 170 remain byte-exact and the private source has 176 files. [Source review](artifacts/final-source-review.json), [design](artifacts/design.md), [history](artifacts/history-audit.md).

## Measurement scope

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, shared host. All nine circuits use 32 workers on CPUs 0–31; primary also uses 16 on CPUs 0–15. Thirty-one fixed blocks rotate/reverse case and variant order. Accepted is the retained production executable; fresh OFF is byte-identical to it, and ON enables the candidate. Each process measures 151 primary or 31 other requests after three primes. Timers include fresh solving, randomized proving, SDK verification and serialization. Loading/preparation are outside latency and inside lifetime process RSS. Latency is the median of process medians; CPU is median summed service per request; RSS is the maximum process high-water mark, not minimum RAM.

Thirty preflights independently pass Gnark before latency. All 930 latency captures precede six grouped counter panels and six fixed-period profiles at 16/32 workers across all variants. All captures finish before remaining independent verification, owner transfer, decoding and analysis. Whole-process taskset also constrains the profiling coordinator. No owned compiler, report generator, analyzer, hash/archive helper or independent workload overlaps capture. The original helpers compile and pin beforehand.

|Case|Variant|Median ms|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|---:|
|12x2-w32|OFF|33.890120|940.412|732.25|
|12x2-w32|ON|33.948705|937.350|725.75|
|12x2-w32|accepted|33.805355|936.867|729.50|
|16x2-w32|OFF|40.088084|1123.436|832.25|
|16x2-w32|ON|40.171462|1121.515|842.75|
|16x2-w32|accepted|40.238070|1125.846|831.50|
|1x2-w16|OFF|10.008021|120.760|92.00|
|1x2-w16|ON|9.855924|119.258|88.00|
|1x2-w16|accepted|9.746099|120.226|89.75|
|1x2-w32|OFF|8.045739|183.053|134.75|
|1x2-w32|ON|8.168629|182.265|136.50|
|1x2-w32|accepted|8.044768|182.864|134.75|
|24x2-w32|OFF|65.739156|1890.352|1307.29|
|24x2-w32|ON|66.012439|1890.469|1291.35|
|24x2-w32|accepted|66.208601|1902.109|1313.36|
|2x2-w32|OFF|11.374222|264.874|274.25|
|2x2-w32|ON|11.436584|261.552|276.00|
|2x2-w32|accepted|11.320763|263.192|276.25|
|4x2-w32|OFF|16.011003|421.639|321.75|
|4x2-w32|ON|16.168818|419.231|316.00|
|4x2-w32|accepted|16.044175|420.265|314.75|
|8x2-w32|OFF|24.189134|667.932|515.00|
|8x2-w32|ON|24.407347|667.334|510.50|
|8x2-w32|accepted|24.185235|669.922|511.25|
|merge-24x1-w32|OFF|65.217525|1812.543|1191.57|
|merge-24x1-w32|ON|65.263799|1806.288|1182.53|
|merge-24x1-w32|accepted|65.291607|1809.516|1153.07|
|merge-54x1-w32|OFF|161.599743|4450.988|1866.85|
|merge-54x1-w32|ON|161.457567|4434.755|1911.90|
|merge-54x1-w32|accepted|162.340349|4463.248|1886.72|

|Paired comparison|Median ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|12x2-w32/OFF/accepted|1.004081231|[0.996931937, 1.007640434]|12|true|
|12x2-w32/ON/OFF|1.001031746|[0.996574804, 1.006573494]|14|true|
|12x2-w32/ON/accepted|1.004407861|[1.000038391, 1.009625640]|9|true|
|16x2-w32/OFF/accepted|0.996929864|[0.993579016, 1.002036119]|20|true|
|16x2-w32/ON/OFF|1.004436495|[0.998955757, 1.008699594]|12|true|
|16x2-w32/ON/accepted|0.998101922|[0.993833784, 1.004470795]|17|true|
|1x2-w16/OFF/accepted|1.015302275|[0.990754883, 1.080533438]|11|false|
|1x2-w16/ON/OFF|0.988440709|[0.920373909, 1.019923978]|16|true|
|1x2-w16/ON/accepted|0.998033789|[0.982784881, 1.027961848]|16|false|
|1x2-w32/OFF/accepted|1.001012778|[0.997804478, 1.004808643]|15|true|
|1x2-w32/ON/OFF|1.016085852|[1.009616275, 1.017233682]|6|false|
|1x2-w32/ON/accepted|1.015216655|[1.012471686, 1.017814211]|0|false|
|24x2-w32/OFF/accepted|0.991593810|[0.987788990, 0.996916157]|26|true|
|24x2-w32/ON/OFF|1.000209984|[0.994256029, 1.010710155]|14|true|
|24x2-w32/ON/accepted|0.994187498|[0.990798965, 0.998526196]|24|true|
|2x2-w32/OFF/accepted|1.007658891|[0.999759609, 1.012851255]|10|true|
|2x2-w32/ON/OFF|1.005668508|[0.994565252, 1.010480216]|12|true|
|2x2-w32/ON/accepted|1.009098699|[1.004717664, 1.017050359]|3|true|
|4x2-w32/OFF/accepted|0.993059569|[0.985759817, 1.002628314]|20|true|
|4x2-w32/ON/OFF|1.009639696|[1.002824183, 1.018853222]|5|true|
|4x2-w32/ON/accepted|1.003849264|[0.994880513, 1.017367429]|11|true|
|8x2-w32/OFF/accepted|0.998480075|[0.988864217, 1.007499484]|16|true|
|8x2-w32/ON/OFF|1.009021117|[1.001258104, 1.018117594]|9|true|
|8x2-w32/ON/accepted|1.009402193|[1.000434735, 1.015996215]|9|true|
|merge-24x1-w32/OFF/accepted|0.999200306|[0.996340881, 1.005217704]|18|true|
|merge-24x1-w32/ON/OFF|1.000853698|[0.996809559, 1.003217802]|14|true|
|merge-24x1-w32/ON/accepted|0.999159861|[0.995832721, 1.003672908]|17|true|
|merge-54x1-w32/OFF/accepted|0.994853706|[0.992008724, 0.998575866]|23|true|
|merge-54x1-w32/ON/OFF|0.999794186|[0.991671541, 1.003057578]|16|true|
|merge-54x1-w32/ON/accepted|0.994066533|[0.989496660, 0.998700861]|23|true|

|Original-eight aggregate versus|Median ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|OFF|1.003391385|[1.001798341, 1.006934357]|5|false|
|accepted|1.004518245|[1.003020133, 1.008699376]|4|false|

Primary 32-worker and equal-log original-eight aggregate checks require upper ON/control ratios below 1 and at least 21/31 wins against both controls. Every other case, including primary 16 and 54→1, requires upper ratios ≤1.02. All ten OFF/accepted calibration intervals must fit factor 1.03. Every maximum RSS ON/control ratio must be ≤1.05. Sorted ranks 10–22 of 31 provide nominal 97.05506265163422% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extensions, threshold changes, successful capture repeats or subset adoption were used. A passing performance screen would still need resource/admission and final-artifact qualification.

## Counters and automatic profiles

Grouped counter panels measure 31 requests after three primes; all four events require at least 99.9% running coverage. Every panel passes. These are single diagnostic observations, not repeated-trial intervals or isolated causes. Retired instructions fall about 0.32% versus OFF at 32 workers and 0.21% at 16, while the full-request gate fails. Those counts do not rescue latency or establish the cause of regression.

|Panel|Instructions/request million|Cycles/request million|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|1380.387|584.929|true|
|stat-1x2-w16-ON|1377.531|578.872|true|
|stat-1x2-w16-accepted|1380.401|584.247|true|
|stat-1x2-w32-OFF|1421.230|835.341|true|
|stat-1x2-w32-ON|1416.612|831.389|true|
|stat-1x2-w32-accepted|1421.163|839.263|true|

Profiles use cycles:u with a 16-million period, 251 measured requests after three primes, CPU IDs, timestamps, periods and executable/task mappings. All six pass attribution checks: one PID, zero lost records, correct affinity/event/mapping, at least 1,000 samples and at most 5% unknown period. Known absolute-period undercoverage remains unresolved. Shares are qualitative, not exclusive costs, local CPI or removable wall-time fractions. Zero helper-name context does not establish zero execution because the helper can be inlined.

|Profile|Samples|Helper-name context period|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-w16-OFF|9006|0|true|0|
|cycles-1x2-w16-ON|8989|0|true|0|
|cycles-1x2-w16-accepted|9054|0|true|0|
|cycles-1x2-w32-OFF|12639|0|true|0|
|cycles-1x2-w32-ON|12504|0|true|0|
|cycles-1x2-w32-accepted|12660|0|true|0|

## Supplied CPU and Metal comparisons

M5 references are supplied historical numbers, not new matched-hardware measurements. ON is experimental. There is no supplied M5 reference for 54→1.

|Circuit|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× achieved|
|---|---:|---:|---:|---:|---:|---|
|12x2-w32|33.948705|284.0|8.366×|101.0|2.975×|false|
|16x2-w32|40.171462|335.0|8.339×|124.0|3.087×|false|
|1x2-w32|8.168629|37.0|4.530×|24.0|2.938×|false|
|24x2-w32|66.012439|528.0|7.998×|176.0|2.666×|false|
|2x2-w32|11.436584|59.0|5.159×|30.0|2.623×|false|
|4x2-w32|16.168818|109.0|6.741×|48.0|2.969×|false|
|8x2-w32|24.407347|189.0|7.744×|74.0|3.032×|false|
|merge-24x1-w32|65.263799|507.0|7.768×|175.0|2.681×|false|

## Proof verification and records

All 1,974 workflow steps complete. All 55,788 SDK requests verify internally. All 972 fresh 164-byte exports independently pass Gnark, with exact public/image bindings, common verification-key identities per shape and uniqueness against 22,990 prior proof identities. The union ledger of 23,962 is persisted, with 1,944 proof/public archive members. Original source/helper/binary/input pins close. Source, receipts, proofs and symbol archives round-trip byte-exactly; raw perf/ELF artifacts stay local with hashes.

[Complete source](artifacts/source.json.gz), [all observations](artifacts/rows.json), [full analysis](artifacts/analysis.json), [capture plan](artifacts/plan.json), [all receipts](artifacts/receipts.json.gz), [proof bytes](artifacts/proofs.json.gz), [verification inventory](artifacts/proof-inventory.json), [archive review](artifacts/publication-review.json).

The candidate is closed under its failed gate. Production remains unchanged. There is no new AWS/Aeglos, ARM/mobile/TEE, forester 250/4999, pressure/swap/minimum-RAM or concurrent-throughput qualification. Accepted local 1×2 remains 8.022165 ms / 134 MiB and 54→1 remains 162.250464 ms. Latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. Accepted tables and the historical physical-16 adoption exception remain unchanged. All eight CPU 10× targets remain unmet.

The requested [consolidated benchmark report](../2026-10-30-cpu-benchmark-report/README.md) combines these results with accepted local/AWS comparisons. [Offline report notes](artifacts/offline-report-notes.json) describe prose and metadata changes after capture; original pinned helpers and measurement data are unchanged.

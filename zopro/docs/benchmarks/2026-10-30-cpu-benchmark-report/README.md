# CPU prover benchmark report

30 October 2026. Snapshot of accepted local and AWS benchmarks, including the completed bounded G2 delta SDK experiment. This report consolidates existing captures and adds no benchmark samples.

The accepted Zopro build measures **8.022 ms for 1×2 locally**, with **134 MiB observed peak process RSS**. On AWS c7a.24xlarge, Zopro measures **11.916 ms versus Aeglos Sorted’s 7.959 ms**, using each prover’s fastest tested worker count. Zopro has lower best-count latency on six of nine AWS circuits and lower observed peak RSS on all nine. Aeglos leads the three smallest circuits. The requested 10× speedup against the supplied M5 CPU references remains **unmet on all eight shapes**.

The latest full-proof candidate measures **8.168629 ms for 1×2**, with **136.50 MiB peak RSS**. Its paired ON/OFF latency ratio is **1.016085852**, with bounds **[1.009616275, 1.017233682]** and **6/31 wins**. Its declared full-request performance gate is **false**; production is unchanged. Kernel improvements and proof improvements are reported separately.

This is a negative full-proof result: **1×2 is 1.61% slower versus OFF and 1.52% slower versus accepted** in paired comparisons. The original-eight aggregate is 0.34% slower versus OFF and 0.45% slower versus accepted. The isolated caller gain did not transfer to this SDK integration.

## Measurement scope

Local: Ryzen 9 9950X, 16 physical cores, 32 logical CPUs, 128 GiB RAM, shared host. The accepted table uses 32 workers. AWS: one c7a.24xlarge, 96 physical cores without SMT, testing 32/64/96 workers in a network-disabled container with a 96-CPU quota, 30 GiB memory limit and no swap. That memory limit is a benchmark setting.

Accepted results are medians of 15 process medians after three priming proofs. Local processes measure 151 requests for 1×2 and 31 for other circuits; AWS processes measure 101 and 17 respectively. Each timed Zopro request includes fresh witness solving, fresh randomized proving, SDK verification and serialization. Loading and preparation are outside latency and inside lifetime process RSS. Peak RSS is the largest process high-water mark, not minimum required RAM or incremental workspace.

AWS Zopro uses the accepted 175-file source compiled for Zen 4; the local executable targets the Ryzen host. Aeglos uses the Sorted engine at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked. These results describe the tested versions and configurations. M5 CPU and Metal numbers are supplied historical references, not new matched-hardware measurements.

## Accepted local results

Speedup is reference latency divided by Zopro latency.

|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.336|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.946|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.845|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.932|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273|1175.50|507|7.767×|175|2.681×|50.7|
|merge-54x1|162.250|1831.87|—|—|—|—|—|

No M5 reference was supplied for 54→1. The accepted local 1×2 result needs approximately 54% lower latency to reach 3.7 ms.

The accepted build’s original full performance gate remains **false** because the physical-16 fresh-control calibration interval exceeded its declared range. Adoption used an explicit engineering exception; candidate gain/nonregression checks and a separate final-artifact gate passed. Later experiments do not inherit this exception. [Adoption decision](../2026-10-29-balanced-kh-adoption/README.md), [final-artifact qualification](../2026-10-29-balanced-kh-production/README.md).

## AWS comparison with Aeglos

Each displayed latency is that prover’s fastest median among 32/64/96 workers; RSS comes from the selected count. This descriptive selection does not establish a universal worker policy. The linked source retains the complete matrix and paired comparisons at equal worker counts.

|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro peak RSS MiB|Aeglos peak RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|11.916|32|7.959|96|120.97|480.14|
|2x2|15.895|32|11.822|96|230.36|811.11|
|4x2|19.993|32|18.492|96|276.21|1351.90|
|8x2|26.898|64|31.531|96|450.82|2491.09|
|12x2|34.862|64|48.580|96|648.76|3714.61|
|16x2|38.983|64|57.136|96|766.20|4612.36|
|24x2|54.034|64|86.667|96|1225.00|6758.14|
|merge-24x1|57.133|64|84.350|96|1109.38|6608.05|
|merge-54x1|122.397|64|173.236|96|1877.03|13115.29|

For 54→1, Zopro measures 122.397 ms and 1.83 GiB RSS versus Aeglos’s 173.236 ms and 12.81 GiB. These are observations of the tested configurations, not an explanation of the latency difference. [Complete AWS matrix and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).

The 1×2 scaling gap is visible at equal worker counts:

|Workers|Zopro ms|Aeglos ms|Zopro summed CPU ms/request|Aeglos summed CPU ms/request|
|---|---:|---:|---:|---:|
|32|11.916|11.868|256.128|328.248|
|64|11.995|8.774|420.399|430.446|
|96|13.778|7.959|704.247|513.394|

Aeglos gains latency as workers increase; Zopro spends more CPU without reducing latency in this sweep. The evidence supports investigating useful parallel work, critical paths and coordination. It does not establish RAM capacity as the sole cause. Local 32-worker runs use SMT; AWS does not, so local topology effects cannot be transferred directly.

## Latest full-proof experiment

The bounded G2 delta candidate replaces two general modular-subtraction preparations with fixed-offset carry preparation inside the final G2 Y product. It preserves four raw products, two Montgomery reductions, field output bounds, exceptional handling, queues and worker policy. It adds no heap buffer, point-array copy, persistent key table or duplicate plan. The earlier caller experiment passed its gate with a **0.405% aggregate gain versus OFF**; that is a caller result, not a proof result. [Caller model, tests and measurements](../2026-10-30-loose-delta175/README.md).

The SDK run uses 31 fixed blocks across all nine circuits at 32 workers and primary 1×2 at 16 workers. Case and variant order rotate/reverse. Accepted is the retained production executable; fresh OFF is byte-identical to it; ON enables the candidate. Primary processes measure 151 requests and others 31, after three primes, with the same full-request timing scope.

|Case|Accepted ms|OFF ms|ON ms|Accepted peak RSS MiB|ON peak RSS MiB|
|---|---:|---:|---:|---:|---:|
|1x2-w32|8.044768|8.045739|8.168629|134.75|136.50|
|1x2-w16|9.746099|10.008021|9.855924|89.75|88.00|
|2x2-w32|11.320763|11.374222|11.436584|276.25|276.00|
|4x2-w32|16.044175|16.011003|16.168818|314.75|316.00|
|8x2-w32|24.185235|24.189134|24.407347|511.25|510.50|
|12x2-w32|33.805355|33.890120|33.948705|729.50|725.75|
|16x2-w32|40.238070|40.088084|40.171462|831.50|842.75|
|24x2-w32|66.208601|65.739156|66.012439|1313.36|1291.35|
|merge-24x1-w32|65.291607|65.217525|65.263799|1153.07|1182.53|
|merge-54x1-w32|162.340349|161.599743|161.457567|1886.72|1911.90|

Ratios pair medians within blocks and need not equal ratios of displayed medians. Lower is faster.

|Paired comparison|Median ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|1x2-w32/ON/accepted|1.015216655|[1.012471686, 1.017814211]|0|false|
|1x2-w32/ON/OFF|1.016085852|[1.009616275, 1.017233682]|6|false|
|1x2-w16/ON/accepted|0.998033789|[0.982784881, 1.027961848]|16|false|
|1x2-w16/ON/OFF|0.988440709|[0.920373909, 1.019923978]|16|true|
|merge-54x1-w32/ON/accepted|0.994066533|[0.989496660, 0.998700861]|23|true|
|merge-54x1-w32/ON/OFF|0.999794186|[0.991671541, 1.003057578]|16|true|
|Original eight ON/accepted|1.004518245|[1.003020133, 1.008699376]|4|false|
|Original eight ON/OFF|1.003391385|[1.001798341, 1.006934357]|5|false|

Declared gate: **false**. Failed checks: `["1x2-w32/ON/accepted","1x2-w32/ON/OFF","1x2-w16/OFF/accepted","1x2-w16/ON/accepted","original8/ON/accepted/gain","original8/ON/OFF/gain"]`. No adoption or accepted performance change follows from this experiment.

The six failures comprise four gain checks (primary and original-eight aggregate, each against both controls), physical-16 OFF/accepted calibration, and physical-16 ON/accepted nonregression. The physical-16 ON/accepted interval spans parity; its 1.028 upper endpoint is not an established 2.8% slowdown. All other nonregression/calibration checks and all relative RSS guards pass. At 54→1, the ON/OFF interval spans parity, so the smaller displayed median does not establish a gain versus OFF.

Primary 32-worker and equal-log original-eight aggregate checks require upper ON/control ratios below 1 and at least 21/31 wins against both controls. Other cases require upper ratios ≤1.02, every OFF/accepted calibration interval must fit factor 1.03, and maximum ON RSS must be ≤1.05 times each control. The relative RSS guard is not an architecture cap. Sorted ranks 10–22 of 31 give nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No observations were excluded and no thresholds changed.

All **55,788 SDK requests** verify internally. All **972 fresh exported proofs** independently pass Gnark with public/image/key identity checks and uniqueness checks, bringing the retained distinct-proof ledger from 22,990 to 23,962. Native OFF/ON each pass 124 core and 8 prover tests; portable ON passes 101 core and 8 prover tests. The byte-exact field sources inherit 35 passed field tests and the caller’s integer model. All six counter panels meet running-coverage checks and all six profiles pass attribution/loss checks. [Full SDK report, all checks, profiles, source and proof receipts](../2026-10-30-loose-delta-sdk175/README.md).

## Other findings and limits

The earlier combined cached-hash/queue-drain candidate measured 7.846 ms for local 1×2 and 87 MiB peak RSS, with a 2.47% paired gain versus accepted and 35.68% lower RSS. It remains unadopted: its 8×2 nonregression interval exceeded 1.02 and physical-16 OFF/accepted calibration failed. An interval spanning parity does not establish a slowdown equal to its upper endpoint. This candidate is separate from bounded G2 delta. [Complete combined experiment](../2026-10-29-cached-drain175/README.md).

Local hardware counters show much larger growth in summed CPU time than in retired instructions as worker count rises. That supports investigating stalls, contention and coordination, but does not isolate a cause. Absolute sample-period accounting failed 21/24 panels in a separate diagnostic. Profile shares remain qualitative; they do not measure exclusive stage costs, local CPI or removable wall time. [Worker counters](../2026-10-29-worker-work175/README.md), [sampling accounting](../2026-10-29-sample-accounting175/README.md).

There is no new AWS measurement of the latest candidate. These local runs do not qualify minimum RAM, swap, memory-pressure behavior, ARM/mobile/TEE, forester 250/4999, or concurrent proof throughput. Loading and preparation remain excluded from proof latency.

The next decision must follow complete-request evidence. A candidate that passes performance still needs resource/admission and final-artifact qualification before adoption. Further work should remove useful arithmetic or coordination cost in complete MSM callers while preserving bounded memory. This report does not forecast a 2× breakthrough.

## Records

[Machine-readable snapshot](data.json), [source hashes](source-hashes.json), [offline report generator](refresh.rs), [checksums](SHA256SUMS). The previous [29 October report](../2026-10-29-cpu-benchmark-report/README.md) remains a dated snapshot.

# Phase-specific worker count for small C7a proofs

On 1×2 at 96 physical cores, limiting the canonical witness solve, scalar gathering and FFT to 32 existing workers produced a **7.45% median paired latency reduction** against the same executable with the cap disabled. Median process means were **11.32 ms ON**, **12.35 ms OFF** and **12.64 ms for accepted Zopro**. ON beat both controls in all three blocks. The paired CPU reduction against OFF was **22.5%**. Peak RSS across the measured processes was 120.26 MiB ON and 120.95 MiB OFF. The accepted build still ran faster at 32 cores, at **10.83 ms**. The previous matched-engine 96-core Aeglos result was **7.73 ms**. This campaign measured Zopro directly. [Matched engine results](../2026-11-18-c7a-phase251/README.md)

On 2×2 the same policy reduced paired 96-core latency by **4.24%** and CPU by **20.4%** against OFF. The other seven shapes took the original general solver route, including all merge54 requests. All three full blocks completed. A broader prototype previously overflowed a thread stack on merge54. The precise stack of the aborted process was unavailable. This follow-up confines the restriction to a scope that returns and drains before MSM begins. It qualifies these two small-shape gains for further work on an exclusive executor. It does not establish concurrent-server or mobile behavior. [Earlier full-width trial](../2026-11-18-phase-width253/README.md)

## Verified proof latency and resources

Times are medians of three warm process means with independent executable backings. Paired percentages divide the measurements within each block first. CPU is summed user and system time per request. RSS is the largest whole-process high-water mark, including preflight and any profiling runs. The count column includes only ON latency samples. At 32 cores the width setting admits the whole pool.
| Shape | Cores | Capped requests | REF ms | OFF ms | ON ms | CPU ms, OFF / ON | Peak MiB, REF / ON | M5 CPU / ON | M5 Metal / ON |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 0/453 | 10.83 | 10.90 | 10.87 | 230 / 230 | 120 / 123 | 3.40× | 2.21× |
| 1x2 | 96 | 453/453 | 12.64 | 12.35 | 11.32 | 627 / 483 | 117 / 120 | 3.27× | 2.12× |
| 2x2 | 32 | 0/153 | 14.58 | 14.50 | 14.49 | 316 / 315 | 273 / 279 | 4.07× | 2.07× |
| 2x2 | 96 | 153/153 | 16.58 | 16.14 | 15.49 | 886 / 704 | 263 / 280 | 3.81× | 1.94× |
| 4x2 | 32 | 0/51 | 19.59 | 19.59 | 19.34 | 493 / 491 | 265 / 265 | 5.64× | 2.48× |
| 4x2 | 96 | 0/51 | 20.93 | 20.43 | 20.32 | 1322 / 1330 | 261 / 255 | 5.37× | 2.36× |
| 8x2 | 32 | 0/51 | 26.90 | 26.93 | 26.80 | 736 / 734 | 468 / 447 | 7.05× | 2.76× |
| 8x2 | 96 | 0/51 | 28.23 | 27.31 | 27.62 | 1923 / 1931 | 454 / 449 | 6.84× | 2.68× |
| 12x2 | 32 | 0/51 | 35.64 | 35.65 | 35.59 | 986 / 988 | 671 / 689 | 7.98× | 2.84× |
| 12x2 | 96 | 0/51 | 35.68 | 35.38 | 35.55 | 2453 / 2470 | 643 / 657 | 7.99× | 2.84× |
| 16x2 | 32 | 0/51 | 41.54 | 41.33 | 41.42 | 1170 / 1164 | 816 / 744 | 8.09× | 2.99× |
| 16x2 | 96 | 0/51 | 41.55 | 41.20 | 41.17 | 2749 / 2745 | 764 / 785 | 8.14× | 3.01× |
| 24x2 | 32 | 0/51 | 60.06 | 60.43 | 61.12 | 1764 / 1756 | 1206 / 1181 | 8.64× | 2.88× |
| 24x2 | 96 | 0/51 | 57.88 | 56.89 | 56.37 | 3637 / 3610 | 1229 / 1201 | 9.37× | 3.12× |
| merge-24x1 | 32 | 0/51 | 63.57 | 61.61 | 63.68 | 1696 / 1705 | 1064 / 1069 | 7.96× | 2.75× |
| merge-24x1 | 96 | 0/51 | 62.41 | 61.33 | 61.65 | 3758 / 3769 | 1098 / 1107 | 8.22× | 2.84× |
| merge-54x1 | 32 | 0/51 | 145.69 | 139.76 | 143.02 | 3561 / 3632 | 2077 / 2158 | — | — |
| merge-54x1 | 96 | 0/51 | 125.04 | 124.20 | 122.29 | 6285 / 6240 | 2013 / 2009 | — | — |

No shape attained the supplied 10× M5 CPU target. The 24×2 result at 96 cores came closest, at 9.37×, on a route where the cap was inactive. All 18 latency and RSS guard checks passed. The paired OFF/REF calibration was within the declared factor of 1.03 for 17 of 18 contexts, including both active cases. Merge54 at 32 cores missed calibration. Its small absolute differences do not support attributing a benefit to this policy.

## What changed at 96 cores

The diagnostic parks surplus workers in a scoped broadcast, includes both transitions in proof time and retains the original Rayon registry. It creates no extra pool or key table. The tested 32-worker prefix normally spans four 8-core L3 groups on this host. An initiating worker outside that prefix replaces its last member. Worker participation and cache placement therefore change together. A fixed-width comparison with 32 workers distributed across the 12 L3 groups could isolate the placement effect. The unchanged G2 code starts almost immediately after the A query, then spends longer inside its callback. Its partition and weighting boundaries are a separate target for tracing. [Cache-sharing groups](artifacts/topology.json) · [G2 timing analysis](../2026-11-18-g2-entry254/README.md)

The grouped hardware-counter observer passed the 5% overhead check on both arms. Median paired per-proof ratios ON/OFF were 0.753 for user cycles and 0.957 for retired user instructions. Both flat sampling panels exceeded the 5% overhead bound, at 6.1% OFF and 6.6% ON. Their symbol shares are descriptive. They cannot attribute saved wall time to a particular scheduler instruction. [Profile details](artifacts/profiles.json.gz)

The first-stage route check and three threading tests passed. All 234 remote exports passed server and host Gnark verification. The remote campaign made 11,052 SDK calls, including 10,350 timed requests. Six local exports and 234 remote exports raised the unique inventory to 64,804. The C7a instance terminated and its tagged volumes and temporary bucket were removed. Production remains at the accepted build. [Block results](artifacts/metrics.json) · [All process observations](artifacts/processes.json.gz) · [Prototype patch](artifacts/prototype.patch.gz) · [Verification receipt](artifacts/review.json)

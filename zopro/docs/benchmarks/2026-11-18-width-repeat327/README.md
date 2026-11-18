# G2 partitioning across C7a core counts

All configurations use seven randomized blocks of verified SDK requests. Timings are medians of process means. The production build has the caller-owned frontend pool. The experimental binary changes the G2 geometry.

| Physical cores | Two ranges, ms | Four ranges, ms | Paired change | Four-range wins |
|---:|---:|---:|---:|---:|
| 64 | 8.410 | 9.071 | +8.68% | 0/7 |
| 72 | 8.945 | 8.780 | -1.84% | 6/7 |
| 80 | 9.634 | 8.764 | -9.09% | 7/7 |
| 88 | 10.125 | 8.809 | -12.84% | 7/7 |
| 96 | 10.002 | 9.092 | -9.22% | 7/7 |

The prespecified sign reversal across 64 and 96 cores replicated. Intermediate widths are exploratory.

| Shape | Cores | Setting | Median ms | CPU ms | Peak MiB | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|---:|
| 16x2 | 48 | h19 | 36.082 | 1373 | 803 | 9.28× / 3.44× |
| 16x2 | 48 | hoff | 36.828 | 1431 | 767 | 9.10× / 3.37× |
| 16x2 | 48 | pool0 | 37.201 | 1436 | 774 | 9.01× / 3.33× |
| 16x2 | 64 | h19 | 36.723 | 1736 | 777 | 9.12× / 3.38× |
| 16x2 | 64 | hoff | 37.697 | 1802 | 779 | 8.89× / 3.29× |
| 16x2 | 64 | pool0 | 37.604 | 1800 | 799 | 8.91× / 3.30× |
| 1x2 | 64 | aeg64 | 8.913 | 426 | 448 | 4.15× / 2.69× |
| 1x2 | 64 | aeg96 | 8.914 | 447 | 485 | 4.15× / 2.69× |
| 1x2 | 64 | integral-both | 9.071 | 296 | 57 | 4.08× / 2.65× |
| 1x2 | 64 | integral-four | 9.760 | 390 | 68 | 3.79× / 2.46× |
| 1x2 | 64 | integral-front | 8.410 | 272 | 54 | 4.40× / 2.85× |
| 1x2 | 64 | integral-off | 9.443 | 374 | 64 | 3.92× / 2.54× |
| 1x2 | 64 | pool16 | 8.453 | 275 | 54 | 4.38× / 2.84× |
| 1x2 | 72 | integral-both | 8.780 | 315 | 51 | 4.21× / 2.73× |
| 1x2 | 72 | integral-front | 8.945 | 301 | 53 | 4.14× / 2.68× |
| 1x2 | 80 | integral-both | 8.764 | 343 | 54 | 4.22× / 2.74× |
| 1x2 | 80 | integral-front | 9.634 | 335 | 53 | 3.84× / 2.49× |
| 1x2 | 88 | integral-both | 8.809 | 375 | 51 | 4.20× / 2.72× |
| 1x2 | 88 | integral-front | 10.125 | 373 | 52 | 3.65× / 2.37× |
| 1x2 | 96 | aeg96 | 7.875 | 515 | 476 | 4.70× / 3.05× |
| 1x2 | 96 | integral-both | 9.092 | 411 | 52 | 4.07× / 2.64× |
| 1x2 | 96 | integral-four | 9.764 | 561 | 67 | 3.79× / 2.46× |
| 1x2 | 96 | integral-front | 10.002 | 400 | 53 | 3.70× / 2.40× |
| 1x2 | 96 | integral-off | 10.732 | 619 | 64 | 3.45× / 2.24× |
| 1x2 | 96 | pool16 | 10.000 | 403 | 55 | 3.70× / 2.40× |
| 24x2 | 48 | h19 | 51.649 | 2021 | 1226 | 10.22× / 3.41× |
| 24x2 | 48 | hoff | 53.162 | 2087 | 1221 | 9.93× / 3.31× |
| 24x2 | 48 | pool0 | 53.781 | 2107 | 1250 | 9.82× / 3.27× |
| 24x2 | 64 | h19 | 50.624 | 2380 | 1240 | 10.43× / 3.48× |
| 24x2 | 64 | hoff | 52.250 | 2488 | 1225 | 10.11× / 3.37× |
| 24x2 | 64 | pool0 | 52.917 | 2488 | 1257 | 9.98× / 3.33× |
| merge-24x1 | 48 | h19 | 54.663 | 1985 | 1102 | 9.28× / 3.20× |
| merge-24x1 | 48 | hoff | 55.480 | 2078 | 1122 | 9.14× / 3.15× |
| merge-24x1 | 48 | pool0 | 56.015 | 2074 | 1081 | 9.05× / 3.12× |
| merge-24x1 | 64 | h19 | 53.826 | 2378 | 1143 | 9.42× / 3.25× |
| merge-24x1 | 64 | hoff | 56.031 | 2476 | 1109 | 9.05× / 3.12× |
| merge-24x1 | 64 | pool0 | 55.909 | 2476 | 1143 | 9.07× / 3.13× |

[Measurement protocol](artifacts/protocol.md) · [Process and request data](artifacts/processes.json.gz) · [Proof verification](artifacts/verification.json)

## Current comparison with the supplied M5 timings

The table gives the integrated 64-core configuration for 1×2, the isolated H candidate at 64 cores for its three qualifying large shapes, and prior C7a measurements for shapes absent from this run. It combines named benchmark cells from different hosts. RSS is the maximum process high-water mark in that cell. Each median comes from seven fresh processes.

| Shape | C7a experiment, cores, setting | Zopro ms | Peak MiB | M5 CPU / Metal ms | Speedup vs CPU / Metal | 10× CPU threshold ms |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | 327, 64, integrated frontend | 8.453 | 54 | 37.0 / 24.0 | 4.38× / 2.84× | 3.7 |
| 2x2 | 322, 48, integrated frontend | 14.089 | 101 | 59.0 / 30.0 | 4.19× / 2.13× | 5.9 |
| 4x2 | 316, 48, retained frontend | 17.851 | 265 | 109.0 / 48.0 | 6.11× / 2.69× | 10.9 |
| 8x2 | 316, 48, retained frontend | 24.363 | 465 | 189.0 / 74.0 | 7.76× / 3.04× | 18.9 |
| 12x2 | 316, 48, ordinary frontend | 32.336 | 658 | 284.0 / 101.0 | 8.78× / 3.12× | 28.4 |
| 16x2 | 327, 64, isolated H19 | 36.723 | 777 | 335.0 / 124.0 | 9.12× / 3.38× | 33.5 |
| 24x2 | 327, 64, isolated H19 | 50.624 | 1240 | 528.0 / 176.0 | 10.43× / 3.48× | 52.8 |
| merge-24x1 | 327, 64, isolated H19 | 53.826 | 1143 | 507.0 / 175.0 | 9.42× / 3.25× | 50.7 |

At 64 cores the isolated H19 candidate reduced paired latency by 3.89% on merge24 (6/7 wins), 3.45% on 24×2 (7/7) and 2.34% on 16×2 (7/7). On 48 cores its paired reductions were 1.73%, 1.71% and 2.29% respectively. The only independently confirmed 10× M5 CPU crossing remains 24×2. The eight-shape target uses the supplied M5 CPU reference for each circuit. Metal ratios are reported separately.

[Earlier full panel](../2026-11-18-frontend-confirm316/README.md) · [Matched-worker comparison](../2026-11-18-worker-match322/README.md)

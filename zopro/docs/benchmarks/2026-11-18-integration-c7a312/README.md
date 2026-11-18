# C7a qualification of the integrated grouping option and worker pools

On 96 physical cores the integrated production source took 10.689 ms with the grouping check enabled and 12.259 ms disabled, a paired change of -12.11% with 7/7 wins. The previously measured enabled binary took 10.656 ms. The integration screen passed.

The small shape uses 101 sequential requests per process and the large shapes use 21. All seven randomized blocks have three warmups per process. Each export passed external verification on the instance and after retrieval. The independent frontend pool is an experimental host configuration. The integrated grouping option builds no additional pool.

| Shape | Physical cores | Setting | Median ms | Median CPU ms | Peak RSS MiB | M5 CPU / Metal speedup | Blocks ≤10× CPU target | 1×2 blocks ≤9 ms |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 16x2 | 40 | combo-large | 37.277 | 1243 | 769 | 8.99× / 3.33× | 0/7 | — |
| 16x2 | 48 | accepted | 36.899 | 1436 | 776 | 9.08× / 3.36× | 0/7 | — |
| 16x2 | 48 | combo-large | 36.054 | 1389 | 766 | 9.29× / 3.44× | 0/7 | — |
| 16x2 | 48 | fft-on | 36.356 | 1401 | 766 | 9.21× / 3.41× | 0/7 | — |
| 16x2 | 56 | combo-large | 36.346 | 1577 | 806 | 9.22× / 3.41× | 0/7 | — |
| 1x2 | 48 | old-on | 9.790 | 301 | 63 | 3.78× / 2.45× | 0/7 | 0/7 |
| 1x2 | 48 | repo-off | 10.566 | 295 | 118 | 3.50× / 2.27× | 0/7 | 0/7 |
| 1x2 | 48 | repo-on | 9.832 | 302 | 63 | 3.76× / 2.44× | 0/7 | 0/7 |
| 1x2 | 56 | old-on | 9.595 | 333 | 62 | 3.86× / 2.50× | 0/7 | 0/7 |
| 1x2 | 56 | repo-off | 10.729 | 343 | 115 | 3.45× / 2.24× | 0/7 | 0/7 |
| 1x2 | 56 | repo-on | 9.536 | 331 | 63 | 3.88× / 2.52× | 0/7 | 0/7 |
| 1x2 | 64 | accepted | 10.923 | 398 | 118 | 3.39× / 2.20× | 0/7 | 0/7 |
| 1x2 | 64 | front-off | 9.412 | 375 | 67 | 3.93× / 2.55× | 0/7 | 0/7 |
| 1x2 | 64 | front16 | 8.492 | 274 | 53 | 4.36× / 2.83× | 0/7 | 7/7 |
| 1x2 | 64 | front32 | 8.561 | 293 | 54 | 4.32× / 2.80× | 0/7 | 7/7 |
| 1x2 | 64 | old-on | 9.420 | 374 | 69 | 3.93× / 2.55× | 0/7 | 0/7 |
| 1x2 | 64 | repo-off | 10.985 | 385 | 119 | 3.37× / 2.18× | 0/7 | 0/7 |
| 1x2 | 64 | repo-on | 9.414 | 375 | 67 | 3.93× / 2.55× | 0/7 | 0/7 |
| 1x2 | 72 | old-on | 9.704 | 426 | 63 | 3.81× / 2.47× | 0/7 | 0/7 |
| 1x2 | 72 | repo-off | 11.219 | 432 | 113 | 3.30× / 2.14× | 0/7 | 0/7 |
| 1x2 | 72 | repo-on | 9.713 | 425 | 67 | 3.81× / 2.47× | 0/7 | 0/7 |
| 1x2 | 96 | accepted | 12.477 | 689 | 111 | 2.97× / 1.92× | 0/7 | 0/7 |
| 1x2 | 96 | aeglos | 8.072 | 502 | 479 | 4.58× / 2.97× | 0/7 | 7/7 |
| 1x2 | 96 | front-off | 10.799 | 625 | 66 | 3.43× / 2.22× | 0/7 | 0/7 |
| 1x2 | 96 | front16 | 10.019 | 402 | 52 | 3.69× / 2.40× | 0/7 | 0/7 |
| 1x2 | 96 | front32 | 10.254 | 421 | 56 | 3.61× / 2.34× | 0/7 | 0/7 |
| 1x2 | 96 | old-on | 10.656 | 619 | 57 | 3.47× / 2.25× | 0/7 | 0/7 |
| 1x2 | 96 | repo-off | 12.259 | 625 | 111 | 3.02× / 1.96× | 0/7 | 0/7 |
| 1x2 | 96 | repo-on | 10.689 | 624 | 59 | 3.46× / 2.25× | 0/7 | 0/7 |
| 24x2 | 40 | combo-large | 55.527 | 1894 | 1234 | 9.51× / 3.17× | 0/7 | — |
| 24x2 | 48 | accepted | 53.783 | 2121 | 1211 | 9.82× / 3.27× | 1/7 | — |
| 24x2 | 48 | combo-large | 52.053 | 2030 | 1258 | 10.14× / 3.38× | 6/7 | — |
| 24x2 | 48 | fft-on | 51.907 | 2046 | 1253 | 10.17× / 3.39× | 7/7 | — |
| 24x2 | 56 | combo-large | 52.225 | 2232 | 1244 | 10.11× / 3.37× | 7/7 | — |
| merge-24x1 | 40 | combo-large | 57.804 | 1847 | 1115 | 8.77× / 3.03× | 0/7 | — |
| merge-24x1 | 48 | accepted | 55.175 | 2047 | 1124 | 9.19× / 3.17× | 0/7 | — |
| merge-24x1 | 48 | combo-large | 54.220 | 2002 | 1110 | 9.35× / 3.23× | 0/7 | — |
| merge-24x1 | 48 | fft-on | 54.288 | 2011 | 1139 | 9.34× / 3.22× | 0/7 | — |
| merge-24x1 | 56 | combo-large | 54.089 | 2182 | 1133 | 9.37× / 3.24× | 0/7 | — |

[Block and request data](artifacts/rows.json.gz) · [Checks](artifacts/checks.json) · [Scope and thresholds](artifacts/protocol.md) · [Settings](artifacts/settings.json) · [Verification](artifacts/verification.json) · [Latest all-shape comparison](../2026-11-18-combined-c7a307/README.md)

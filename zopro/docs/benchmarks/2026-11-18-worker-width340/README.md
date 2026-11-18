# C7a physical core scaling and integrated G2 ranges

Seven randomized blocks on one c7a.24xlarge. The eight-shape scorecard uses the integrated executable, with two ranges on 1×2 and 2×2. Times are medians of process means. Each 1×2 cell timed 707 SDK requests, after three primes per process.

| Shape | Cores | Time ms | Peak MiB | M5 CPU / Metal ms | Speedup CPU / Metal | Blocks within 10× CPU |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 8.441 | 55 | 37 / 24 | 4.38× / 2.84× | 0/7 |
| 2x2 | 48 | 14.131 | 101 | 59 / 30 | 4.18× / 2.12× | 0/7 |
| 4x2 | 48 | 18.090 | 280 | 109 / 48 | 6.03× / 2.65× | 0/7 |
| 8x2 | 48 | 24.572 | 468 | 189 / 74 | 7.69× / 3.01× | 0/7 |
| 12x2 | 64 | 32.964 | 656 | 284 / 101 | 8.62× / 3.06× | 0/7 |
| 16x2 | 64 | 36.618 | 769 | 335 / 124 | 9.15× / 3.39× | 0/7 |
| 24x2 | 64 | 50.340 | 1215 | 528 / 176 | 10.49× / 3.50× | 6/7 |
| merge-24x1 | 64 | 54.340 | 1145 | 507 / 175 | 9.33× / 3.22× | 0/7 |

On 1×2, 64 cores gave the lowest latency among the three tested widths. At 96 cores, the four-range override reduced Zopro latency from 9.997 to 9.039 ms. Aeglos measured 7.912 ms at that width. Only 24×2 reached the supplied 10× CPU goal in the median.

[Worker timing by request duration](workers.md) · [CPU sample attribution](samples.md) · [Follow-up bucket merge timings](pair-merges.md)

## 1×2 on matched worker counts

All Zopro configurations use 16 frontend workers on the same affinity mask. The Aeglos native and Go worker limits match the number of physical cores. RSS is the largest process peak in the cell. Request quantiles pool all seven blocks.

| Cores | Setting | Time ms | Request p50 / p95 ms | CPU-ms | Peak MiB |
|---:|---|---:|---:|---:|---:|
| 32 | current2 | 10.786 | 10.736 / 11.284 | 221.3 | 69 |
| 32 | current4 | 11.128 | 11.128 / 11.554 | 231.9 | 69 |
| 32 | aeg32 | 11.829 | 11.819 / 12.356 | 326.9 | 421 |
| 32 | observed2 | 10.801 | 10.743 / 11.329 | 221.0 | 67 |
| 32 | observed4 | 11.147 | 11.127 / 11.553 | 231.5 | 68 |
| 64 | current2 | 8.441 | 8.335 / 8.685 | 274.5 | 55 |
| 64 | current4 | 8.988 | 8.799 / 9.835 | 295.2 | 56 |
| 64 | aeg64 | 8.789 | 8.810 / 9.452 | 426.4 | 443 |
| 64 | observed2 | 8.425 | 8.335 / 8.644 | 274.0 | 56 |
| 64 | observed4 | 9.091 | 8.857 / 9.832 | 295.4 | 55 |
| 64 | prior2 | 8.410 | 8.333 / 8.621 | 273.9 | 56 |
| 96 | current2 | 9.997 | 8.911 / 12.323 | 400.6 | 53 |
| 96 | current4 | 9.039 | 8.507 / 11.051 | 409.6 | 49 |
| 96 | aeg96 | 7.912 | 7.850 / 8.404 | 512.8 | 473 |
| 96 | observed2 | 10.111 | 8.899 / 12.255 | 404.6 | 55 |
| 96 | observed4 | 9.157 | 8.516 / 11.034 | 412.4 | 52 |
| 96 | prior2 | 9.925 | 8.860 / 12.227 | 399.9 | 54 |

[Protocol](artifacts/protocol.md) · [Per-request data](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

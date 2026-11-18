# Width scaling across the eight reference shapes

For each shape the same integrated binary, key and request run with fixed algorithm settings at all three widths. The primary 1×2 row uses two ranges with parallel decomposition in seven blocks of 101 proofs per process. The other rows use five independent randomized blocks of eleven proofs per process. Paired changes compare 96 with 64 in the same block. CPU time sums user and system time for every thread.

| Shape | 32 cores ms | 64 cores ms | 96 cores ms | 96 vs 64 paired | 96 faster blocks | CPU-ms 32 / 64 / 96 | Peak MiB 32 / 64 / 96 |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 10.819 | 8.451 | 9.918 | +17.20% | 0/7 | 221.8 / 274.3 / 403.3 | 66 / 54 / 56 |
| 2x2 | 15.407 | 14.270 | 14.541 | +1.90% | 0/5 | 305.1 / 417.8 / 593.3 | 99 / 91 / 89 |
| 4x2 | 19.426 | 19.374 | 20.229 | +6.40% | 0/5 | 492.6 / 859.1 / 1336.2 | 243 / 247 / 246 |
| 8x2 | 27.264 | 26.361 | 28.224 | +6.92% | 0/5 | 742.9 / 1295.4 / 1958.3 | 434 / 415 / 419 |
| 12x2 | 35.520 | 33.740 | 35.491 | +7.23% | 0/5 | 955.9 / 1567.7 / 2382.5 | 600 / 588 / 591 |
| 16x2 | 40.939 | 36.911 | 40.022 | +9.01% | 0/5 | 1116.7 / 1743.4 / 2655.4 | 717 / 727 / 740 |
| 24x2 | 60.132 | 50.845 | 54.421 | +6.91% | 0/5 | 1648.3 / 2389.8 / 3503.0 | 1088 / 1119 / 1148 |
| merge-24x1 | 61.111 | 54.757 | 59.474 | +7.51% | 0/5 | 1603.3 / 2387.1 / 3608.1 | 1036 / 1023 / 1036 |

At fixed settings 64 cores gave the lowest process median of the three widths on all eight shapes. On 4×2 and 8×2, 96 also exceeded the 32-core median.

[Scaling protocol](artifacts/scaling-protocol.md) · [Primary report](README.md)

## 1×2 with the width-specific G2 change

Each row pairs the 96-core four-range serial setting with the 64-core ordinary two-range setting from the same block. The earlier host used the experimental serial implementation and its integrated two-range control. This host uses one integrated executable for both settings. The span shows every observed paired latency change.

| Host and build | 64 ms | 96 ms | Paired wall change | Range across blocks | 96 faster | Paired CPU change |
|---|---:|---:|---:|---:|---:|---:|
| Earlier candidate | 8.409 | 8.485 | +1.59% | -0.36% to +2.25% | 1/7 | +41.5% |
| Integrated release | 8.451 | 8.512 | +1.06% | -0.35% to +1.57% | 1/7 | +42.5% |

## CPU use of the main and frontend pools combined

Effective busy cores are the ratio of aggregate user plus system CPU time to elapsed time for the timed requests. The ratio includes useful arithmetic, worker coordination and any other on-CPU work. Each cell is the median of per-process ratios.

| Shape | Busy cores at 32 | Busy cores at 64 | Busy cores at 96 | Extra CPU-ms/proof, 96 vs 64 |
|---|---:|---:|---:|---:|
| 1x2 | 20.5 | 32.5 | 40.4 | +129.1 |
| 2x2 | 19.8 | 29.3 | 41.0 | +180.3 |
| 4x2 | 25.3 | 44.3 | 65.3 | +490.3 |
| 8x2 | 27.4 | 49.2 | 69.4 | +658.4 |
| 12x2 | 26.9 | 46.6 | 66.9 | +804.4 |
| 16x2 | 27.4 | 47.3 | 66.1 | +909.1 |
| 24x2 | 27.5 | 46.9 | 64.4 | +1113.2 |
| merge-24x1 | 26.3 | 43.8 | 61.0 | +1193.3 |

## Request distribution at 64 and 96 cores

Percentiles pool timed requests from the fixed-settings comparison. Each 1×2 cell contains 707 requests. Other shapes have 55 requests per cell.

| Shape | 64-core p50 / p95 ms | 96-core p50 / p95 ms |
|---|---:|---:|
| 1x2 | 8.347 / 8.647 | 8.894 / 12.257 |
| 2x2 | 14.050 / 15.533 | 14.497 / 15.741 |
| 4x2 | 19.258 / 20.136 | 20.433 / 22.564 |
| 8x2 | 26.077 / 27.628 | 27.983 / 30.347 |
| 12x2 | 33.578 / 36.315 | 35.404 / 37.873 |
| 16x2 | 36.422 / 39.510 | 39.650 / 43.090 |
| 24x2 | 50.467 / 55.076 | 54.123 / 57.277 |
| merge-24x1 | 54.547 / 56.935 | 58.828 / 61.700 |

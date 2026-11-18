# Solver epochs and G2 splitting on C7a

With four G2 ranges and serial scalar decomposition on 1×2 at 96 cores, four solver epochs measured 8.206 ms versus 8.510 ms at sixteen. The paired change was -3.57%, with 7/7 faster processes. Aeglos with 96 native workers and GOMAXPROCS 96 on the same host measured 8.001 ms. All three cells use seven fresh process means with 71 timed requests each.

## Prespecified all-shape scorecard

Each row uses the exact integrated release in five independently randomized blocks. Times are medians of process means. The M5 CPU and Metal references were supplied for comparison across machines.

| Shape | Cores | Epochs | G2 ranges | SDK ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 96 | 4 | 4 | 8.171 | 46 | 37 / 24 | 4.53× / 2.94× | 0/5 |
| 2x2 | 48 | 4 | 2 | 13.132 | 116 | 59 / 30 | 4.49× / 2.28× | 0/5 |
| 4x2 | 48 | 8 | 2 | 17.213 | 257 | 109 / 48 | 6.33× / 2.79× | 0/5 |
| 8x2 | 48 | 8 | 2 | 23.849 | 448 | 189 / 74 | 7.92× / 3.10× | 0/5 |
| 12x2 | 48 | 8 | 2 | 30.575 | 640 | 284 / 101 | 9.29× / 3.30× | 0/5 |
| 16x2 | 64 | 6 | 2 | 34.678 | 743 | 335 / 124 | 9.66× / 3.58× | 0/5 |
| 24x2 | 64 | 8 | 2 | 46.323 | 1165 | 528 / 176 | 11.40× / 3.80× | 5/5 |
| merge-24x1 | 64 | 8 | 2 | 47.796 | 993 | 507 / 175 | 10.61× / 3.66× | 5/5 |

## Held-out shape selections

The selected setting minimizes the median of five screen processes. The following results use seven subsequently randomized processes. Control settings are 64 cores and 16 epochs with two ranges for 1×2, 48 cores and 16 epochs with two ranges for 2×2, and 64 cores and six epochs for 16×2.

| Shape | Selected cores / epochs / ranges | Screen ms | Holdout ms | Control ms | Paired change | Faster blocks | CPU / Metal speedup | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 / 6 / 2 | 8.069 | 8.169 | 8.449 | -3.13% | 6/7 | 4.53× / 2.94× | 0/7 |
| 2x2 | 48 / 4 / 4 | 12.935 | 12.912 | 14.026 | -8.37% | 7/7 | 4.57× / 2.32× | 0/7 |
| 16x2 | 68 / 6 / 2 | 33.615 | 34.001 | 34.593 | -1.33% | 6/7 | 9.85× / 3.65× | 2/7 |

## Interaction at 96 cores on 1×2

| Epochs | Two ranges ms | Four serial ranges ms | Paired four vs two | Faster blocks | Peak MiB two / four |
|---:|---:|---:|---:|---:|---:|
| 4 | 9.652 | 8.206 | -14.76% | 7/7 | 53 / 50 |
| 16 | 9.911 | 8.510 | -13.90% | 7/7 | 53 / 49 |

[Full screening panel](screen.md) · [Large-proof phase profiles](stages.md) · [Protocol](artifacts/protocol.md) · [All individual timings](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

[Frontend-width confirmation against Aeglos](frontend.md) · [H merge/weight experiment](tiles.md) · [Selected 1×2 critical path](micro.md)

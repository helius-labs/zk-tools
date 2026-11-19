# Verified 12×2 proof across eight Ryzen placements

The same original request ran on 30 hardware threads. Every placement used all 16 physical cores. Each arm has 32 timing processes and eight separate profiles. Each process primed three requests and timed fifteen more. All 160 exported proofs verified on the original key. The 651 arm selected before this run was **fused**. The last two columns count process means and within-placement medians meeting 28.4 ms.

Mapped representatives beat the indexed configuration in 28/32 paired blocks. Their median process mean was 28.527 ms, 0.127 ms above the 10× target. Six processes met the target and one of eight placements met it by median. The historical 28.375 ms result remains valid for its earlier panel.

| Build and mode | Median ms | Paired vs indexed | Peak MiB | M5 CPU / Metal | Processes at target | Placements at target |
|---|---:|---:|---:|---:|---:|---:|
| dense | 28.689 | -0.88% (25/32) | 617.9 | 9.90× / 3.52× | 4/32 | 0/8 |
| mapped | 28.527 | -1.02% (28/32) | 567.3 | 9.96× / 3.54× | 6/32 | 1/8 |
| indexed | 28.810 | +0.00% (0/32) | 609.7 | 9.86× / 3.51× | 0/32 | 0/8 |
| fused | 28.756 | -0.63% (22/32) | 570.0 | 9.88× / 3.51× | 2/32 | 0/8 |

| Omitted SMT siblings | Dense ms | Mapped ms | Indexed ms | fused ms |
|---|---:|---:|---:|---:|
| 22, 30 | 28.705 | 28.559 | 28.787 | 28.715 |
| 23, 31 | 28.994 | 28.460 | 29.168 | 28.954 |
| 16, 24 | 28.529 | 28.471 | 28.888 | 28.719 |
| 17, 25 | 28.748 | 28.380 | 29.049 | 28.618 |
| 18, 26 | 28.577 | 28.696 | 28.841 | 28.700 |
| 19, 27 | 28.555 | 28.596 | 28.737 | 28.918 |
| 20, 28 | 28.604 | 28.565 | 28.995 | 28.632 |
| 21, 29 | 28.653 | 28.574 | 28.705 | 28.891 |

[Protocol](protocol.md) · [Pinned config](artifacts/config.json.gz) · [All samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

## Individual warm requests

The primary table uses the mean of all fifteen requests in each process before taking the median across processes. The following distribution gives each individual warm request equal weight. P95 uses the nearest rank. The final column summarizes the process medians shown by the benchmark runner.

| Arm | Individual median ms | Individual P95 ms | Individual requests at target | Median of process medians ms |
|---|---:|---:|---:|---:|
| dense | 28.476 | 30.442 | 219/480 | 28.464 |
| mapped | 28.374 | 30.142 | 250/480 | 28.375 |
| indexed | 28.886 | 34.676 | 128/480 | 28.805 |
| fused | 28.634 | 30.089 | 180/480 | 28.655 |

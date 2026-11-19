# Evaluating adjacent Poseidon partial rounds together

For two consecutive width-three partial rounds the second dot product can be expanded against the original two side-state values and the first S-box output. This permits two side-state updates to share a Montgomery reduction. Both candidate schedules use the same 28 public field coefficients, totaling 896 bytes per process. The existing code writes every original wire and constraint row.

Nine fresh randomized blocks confirmed the selected mode on 1×2 and 16×2. The other shapes have three fresh blocks each. Times are medians of per-process mean verified SDK latencies. Every row uses the integrated 379 binary as its release control. All arms enable task claims on 1×2 and 2×2.

| Shape | Mode | Release ms | Disabled ms | Selected ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Peak MiB release / selected | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | ahead | 7.342 | 7.345 | 7.357 | +0.02% | 4/9 | +0.07% | 4/9 | 67.2 / 68.8 | 5.03× / 3.26× |
| 2x2 | product | 10.471 | 10.275 | 10.321 | -1.16% | 2/3 | +0.45% | 1/3 | 112.2 / 117.0 | 5.72× / 2.91× |
| 4x2 | product | 16.917 | 16.972 | 17.049 | +0.17% | 1/3 | +0.46% | 1/3 | 204.0 / 210.5 | 6.39× / 2.82× |
| 8x2 | product | 26.532 | 26.679 | 26.676 | +0.55% | 1/3 | -0.10% | 2/3 | 343.2 / 356.2 | 7.08× / 2.77× |
| 12x2 | product | 41.491 | 38.791 | 38.643 | -6.86% | 2/3 | -0.51% | 2/3 | 491.8 / 507.8 | 7.35× / 2.61× |
| 16x2 | product | 45.389 | 45.309 | 45.755 | +0.67% | 3/9 | +0.55% | 0/9 | 629.8 / 638.2 | 7.32× / 2.71× |
| 24x2 | product | 73.562 | 73.826 | 73.755 | +1.60% | 1/3 | -0.24% | 2/3 | 952.0 / 891.4 | 7.16× / 2.39× |
| merge-24x1 | product | 71.432 | 70.553 | 70.994 | -1.05% | 2/3 | +0.73% | 1/3 | 864.6 / 891.8 | 7.14× / 2.46× |

The two-round recurrence reached the intended specialized spans and preserved the reference outputs. In the 16×2 holdout the selected product variant took 45.755 ms, versus 45.309 ms disabled and 45.389 ms for the integrated release. It lost all nine pairs against disabled. On 1×2 the selected ahead variant took 7.357 ms versus 7.342 ms release and won four of nine. The release keeps the original partial rounds.

## Coverage at preparation

| Shape | Selected | Width-three spans | Round pairs per proof |
|---|---|---:|---:|
| 12x2 | product | 720 | 20160 |
| 16x2 | product | 1007 | 28196 |
| 1x2 | ahead | 66 | 1848 |
| 24x2 | product | 1446 | 40488 |
| 2x2 | product | 128 | 3584 |
| 4x2 | product | 242 | 6776 |
| 8x2 | product | 475 | 13300 |
| merge-24x1 | product | 1413 | 39564 |

The zero optional-preparation allowance process produced an independently verified proof and recorded no specialized route.

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz)

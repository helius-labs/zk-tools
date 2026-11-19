# Encoding Poseidon output addresses in existing solver tasks

After full coefficient validation, eligible S-boxes can store their output locations in the three original task words. One word holds the input slot and multiplication orientation. Each of the next two words holds a first row or wire and two nonnegative displacements. Unsupported addresses retain the interpreter metadata. No separate address array is retained.

Nine fresh randomized blocks confirmed the selected mode on 1×2 and 16×2. The other shapes have three fresh blocks each. Times are medians of per-process mean verified SDK latencies. Every row uses the integrated 379 binary as its release control. All arms enable task claims on 1×2 and 2×2.

| Shape | Mode | Release ms | Disabled ms | Selected ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Peak MiB release / selected | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | partial | 7.359 | 7.452 | 7.397 | +0.54% | 4/9 | -0.73% | 6/9 | 70.0 / 72.8 | 5.00× / 3.24× |
| 2x2 | all | 10.235 | 10.230 | 10.351 | +1.12% | 1/3 | +1.55% | 0/3 | 113.2 / 111.8 | 5.70× / 2.90× |
| 4x2 | all | 16.891 | 16.799 | 16.667 | -1.27% | 3/3 | -0.91% | 3/3 | 198.2 / 201.5 | 6.54× / 2.88× |
| 8x2 | all | 26.710 | 26.552 | 26.310 | -1.59% | 2/3 | -0.14% | 3/3 | 342.0 / 356.0 | 7.18× / 2.81× |
| 12x2 | all | 39.101 | 38.792 | 38.642 | -1.79% | 2/3 | -0.74% | 2/3 | 506.5 / 504.5 | 7.35× / 2.61× |
| 16x2 | all | 45.292 | 45.487 | 44.989 | -1.18% | 9/9 | -1.27% | 9/9 | 641.2 / 647.0 | 7.45× / 2.76× |
| 24x2 | all | 73.689 | 73.959 | 72.275 | -1.92% | 3/3 | -2.12% | 3/3 | 929.0 / 922.8 | 7.31× / 2.44× |
| merge-24x1 | all | 71.605 | 71.538 | 71.049 | -0.26% | 3/3 | -0.68% | 2/3 | 862.8 / 837.4 | 7.14× / 2.46× |

On 16×2 the all-round candidate encoded every one of the 76,675 eligible S-boxes. It beat the integrated release and disabled control in all nine holdout blocks. Its median paired change was −1.18% versus release, with peak RSS of 647.0 MiB versus 641.2 MiB. The 1×2 holdout favored the release over the selected partial-round configuration in five of nine pairs. A clean all-round implementation is entering independent qualification with the default disabled.

## Coverage at preparation

| Shape | Mode | Eligible S-boxes | Encoded S-boxes | Coverage |
|---|---|---:|---:|---:|
| 12x2 | all | 55150 | 55150 | 100.0% |
| 16x2 | partial | 58171 | 58171 | 100.0% |
| 16x2 | all | 76675 | 76675 | 100.0% |
| 1x2 | partial | 4222 | 4222 | 100.0% |
| 1x2 | all | 5620 | 5620 | 100.0% |
| 24x2 | all | 110410 | 110410 | 100.0% |
| 2x2 | all | 10450 | 10450 | 100.0% |
| 4x2 | all | 18940 | 18940 | 100.0% |
| 8x2 | all | 36415 | 36415 | 100.0% |
| merge-24x1 | all | 106987 | 106987 | 100.0% |

The zero optional-preparation allowance process produced an independently verified proof and recorded no specialized route.

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz)

# Frontend worker pool and curve-query interaction on Ryzen

Small transfers use 32 logical workers and four G2 ranges for the backend option. Large shapes use 16 physical workers and 19 mixed H windows for the backend option. The optional frontend pool has eight workers within the same affinity mask. Five randomized blocks compare the integrated build and both retained single-optimization binaries. The retained frontend binary predates the direct FFT and H13 changes on large shapes.

| Shape | Setting | Median ms | Median CPU ms | Peak MiB |
|---|---|---:|---:|---:|
| 1x2 | old-front | 7.700 | 152 | 64 |
| 1x2 | old-msm | 9.687 | 174 | 88 |
| 1x2 | off | 7.346 | 166 | 74 |
| 1x2 | front | 7.788 | 151 | 66 |
| 1x2 | msm | 7.538 | 175 | 66 |
| 1x2 | both | 7.902 | 158 | 67 |
| 2x2 | old-front | 10.821 | 202 | 120 |
| 2x2 | old-msm | 10.617 | 247 | 274 |
| 2x2 | off | 10.518 | 240 | 262 |
| 2x2 | front | 10.838 | 203 | 114 |
| 2x2 | msm | 10.650 | 250 | 268 |
| 2x2 | both | 10.972 | 212 | 124 |
| 16x2 | old-front | 46.514 | 688 | 589 |
| 16x2 | old-msm | 44.451 | 645 | 579 |
| 16x2 | off | 45.218 | 672 | 624 |
| 16x2 | front | 45.326 | 675 | 595 |
| 16x2 | msm | 44.269 | 647 | 599 |
| 16x2 | both | 44.536 | 649 | 582 |
| merge-24x1 | old-front | 74.640 | 1087 | 885 |
| merge-24x1 | old-msm | 69.093 | 999 | 883 |
| merge-24x1 | off | 70.172 | 1041 | 857 |
| merge-24x1 | front | 70.244 | 1035 | 913 |
| merge-24x1 | msm | 68.253 | 1003 | 887 |
| merge-24x1 | both | 68.824 | 1001 | 898 |

| Shape | Both vs frontend | Wins / 5 | Both vs backend | Wins / 5 | Interaction screen |
|---|---:|---:|---:|---:|---|
| 1x2 | +0.46% | 2 | +4.96% | 1 | False |
| 2x2 | +1.87% | 2 | +2.02% | 1 | False |
| 16x2 | -1.74% | 4 | +1.31% | 1 | False |
| merge-24x1 | -1.97% | 5 | +0.00% | 2 | False |

All 120 fresh-process proof exports passed the external verifier. The patch applies to experiment 315.

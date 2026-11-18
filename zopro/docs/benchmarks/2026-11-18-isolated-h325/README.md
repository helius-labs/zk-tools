# Isolating the 19-window H query on the integrated prover

The candidate adds one opt-in layout to the source with the caller-owned pool. It keeps the ordinary FFT and curve code for the other queries. The new kernel reserves any extra scratch over the ordinary raw H geometry. Measurements use 16 physical Ryzen cores, five randomized blocks and 11 timed full SDK proofs per fresh process. The retained 315 executable also used the ordinary FFT for this comparison.

| Shape | Setting | Median ms | Paired vs off | Wins / 5 | CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| merge-24x1 | integrated | 73.759 | -0.05% | 3 | 1085 | 868 |
| merge-24x1 | old19 | 71.985 | -2.94% | 4 | 1044 | 869 |
| merge-24x1 | off | 74.163 | +0.00% | — | 1087 | 885 |
| merge-24x1 | on | 71.492 | -2.80% | 5 | 1030 | 869 |
| 12x2 | integrated | 40.352 | -1.97% | 3 | 577 | 504 |
| 12x2 | old19 | 38.055 | -5.47% | 4 | 556 | 526 |
| 12x2 | off | 40.256 | +0.00% | — | 591 | 514 |
| 12x2 | on | 38.484 | -1.98% | 3 | 564 | 522 |
| 16x2 | integrated | 46.785 | +0.69% | 2 | 690 | 600 |
| 16x2 | old19 | 45.999 | -0.47% | 3 | 663 | 600 |
| 16x2 | off | 46.464 | +0.00% | — | 692 | 606 |
| 16x2 | on | 44.845 | -3.72% | 5 | 654 | 579 |
| 24x2 | integrated | 75.735 | +1.20% | 2 | 1132 | 943 |
| 24x2 | old19 | 74.533 | -3.34% | 4 | 1082 | 961 |
| 24x2 | off | 75.176 | +0.00% | — | 1139 | 979 |
| 24x2 | on | 72.317 | -2.76% | 4 | 1078 | 956 |
| 8x2 | integrated | 27.329 | +1.30% | 2 | 398 | 364 |
| 8x2 | old19 | 27.421 | +1.03% | 1 | 403 | 366 |
| 8x2 | off | 26.942 | +0.00% | — | 396 | 353 |
| 8x2 | on | 26.913 | -1.03% | 4 | 393 | 358 |

The G1 route probe fired on enabled 24×2 and stayed idle on disabled 24×2, ineligible 8×2 and zero optional memory. All 104 exports passed independent verification. The patch applies to source commit 49ddb51d04.

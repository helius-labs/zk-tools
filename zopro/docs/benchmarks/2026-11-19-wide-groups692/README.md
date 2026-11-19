# Variable-coefficient groups on four wide-output shapes

For each key, eight public training requests identified positions that shared a varying wide scalar on every training input. Groups of at least three positions were mapped into B2, A and private K. The grouped evaluator retains a key-dependent point sum for each group and computes exact corrections for differing proof scalars or falls back to its ordinary path. Both settings in this comparison used the same binary and small fixed-coefficient file. Group preparation is an opt-in addition to the measured process RSS.

Four balanced blocks on a Ryzen 9950X with 32 logical workers compared eleven warm proofs per process on the original catalog input and four held-out transactions per shape. Median process means and within-block change are shown. Peak RSS includes preparation. The original keys and expected public inputs independently verified all 160 exports.

| Shape | Request | Groups off / on ms | Paired change (wins) | Peak MiB off / on |
|---|---|---:|---:|---:|
| 1x16 | catalog | 13.357 / 13.083 | -2.04% (3/4) | 158.5 / 136.2 |
| 1x16 | 57 | 13.160 / 13.205 | +0.14% (2/4) | 131.6 / 148.1 |
| 1x16 | 58 | 13.104 / 13.230 | +0.86% (2/4) | 127.2 / 134.9 |
| 1x16 | 59 | 13.330 / 13.155 | -0.75% (3/4) | 125.5 / 147.6 |
| 1x16 | 60 | 13.076 / 13.035 | +0.49% (2/4) | 123.0 / 150.9 |
| 2x16 | catalog | 16.872 / 17.783 | +5.80% (0/4) | 250.8 / 319.0 |
| 2x16 | 57 | 16.916 / 17.380 | +3.54% (0/4) | 245.6 / 313.0 |
| 2x16 | 58 | 16.940 / 17.962 | +6.06% (0/4) | 244.4 / 316.8 |
| 2x16 | 59 | 16.938 / 17.644 | +3.93% (0/4) | 259.8 / 314.6 |
| 2x16 | 60 | 17.091 / 17.767 | +4.20% (1/4) | 252.6 / 321.4 |
| 1x8 | catalog | 10.316 / 10.245 | -1.45% (3/4) | 106.0 / 114.8 |
| 1x8 | 57 | 10.264 / 10.365 | +0.98% (1/4) | 106.0 / 122.0 |
| 1x8 | 58 | 10.360 / 10.554 | +2.39% (1/4) | 102.5 / 115.5 |
| 1x8 | 59 | 10.392 / 10.423 | +0.01% (2/4) | 103.0 / 112.5 |
| 1x8 | 60 | 10.335 / 10.510 | +2.40% (1/4) | 101.2 / 118.8 |
| 1x4 | catalog | 8.877 / 8.789 | -0.31% (2/4) | 85.8 / 101.5 |
| 1x4 | 57 | 8.833 / 8.883 | +2.19% (1/4) | 94.5 / 126.8 |
| 1x4 | 58 | 8.864 / 8.821 | -1.10% (3/4) | 89.2 / 97.5 |
| 1x4 | 59 | 8.781 / 8.773 | -0.24% (2/4) | 94.2 / 94.8 |
| 1x4 | 60 | 8.840 / 8.883 | +0.15% (2/4) | 91.5 / 95.5 |

At 1×16 the grouped B2 layout removed 351 of 57,943 positions. The original catalog median improved, but three of four held-out paired medians worsened. At 2×16 the grouped setting lost 19/20 blocks and raised peak RSS on all five inputs. None of these four grouped profiles advanced to the AWS screen.

[Public coefficient study](../2026-11-19-wide-coefficients691/README.md) · [Group dimensions](artifacts/layouts.json) · [Measurements](artifacts/rows.json.gz) · [Proof verification](artifacts/verification.json)

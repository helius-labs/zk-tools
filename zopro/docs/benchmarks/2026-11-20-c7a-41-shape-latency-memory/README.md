# Zopro and Aeglos on 96-core C7a

Zopro had lower paired median latency on both requests for **41 of 41** circuit shapes, lower raw medians on both for **40 of 41**, and won every block on both for **34 of 41**. Peak process RSS was lower on both requests for **41 of 41** and was 4.3–10.9× lower across the 82 request cells.

The table consolidates completed, independently verified runs on the AWS c7a.24xlarge. Each row uses matched input, key, host and timing blocks within the linked panel. The selection uses distinct SDK snapshots and runtime profiles across shapes. Fresh same-build full-catalog validation is outstanding.

Latency is the median of warm block means in milliseconds. Memory is the highest whole-process RSS across timing blocks, including initialization. Each latency and memory pair is Zopro / Aeglos. The paired column shows the median percent change and wins within each request. The baseline uses four blocks per request, and the follow-ups use eight or ten. The machine-readable results retain memory peaks per request.

| Shape | Catalog ms | Held-out ms | Catalog / held-out paired difference | Peak MiB, catalog | Peak MiB, held-out | Matched panel |
|---|---:|---:|---|---:|---:|---|
| 1×2 | 7.765 / 8.190 | 7.685 / 8.162 | -5.18% (8/8) / -5.52% (8/8) | 45.2 / 458.9 | 48.2 / 458.8 | [48+48, two G2 ranges](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 1×4 | 8.716 / 9.732 | 8.718 / 9.609 | -10.63% (8/8) / -8.89% (8/8) | 54.2 / 584.3 | 57.3 / 584.0 | [48+32, fused K/H](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 1×8 | 10.374 / 10.616 | 10.326 / 10.681 | -2.29% (7/8) / -3.17% (7/8) | 138.2 / 690.7 | 139.1 / 690.2 | [32+64, concurrent scalar preparation](../2026-11-20-g2-joint-placement-c7a1016/README.md) |
| 1×16 | 11.702 / 12.394 | 11.668 / 12.392 | -5.62% (8/8) / -5.71% (8/8) | 91.0 / 825.7 | 89.5 / 831.0 | [40+40, G1 width 13, fused K/H](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 2×2 | 10.291 / 11.775 | 10.310 / 11.853 | -12.61% (8/8) / -12.76% (8/8) | 78.9 / 814.5 | 74.5 / 812.5 | [32+32](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 2×4 | 11.161 / 12.325 | 11.186 / 12.335 | -9.92% (8/8) / -8.89% (8/8) | 186.1 / 821.6 | 191.5 / 828.4 | [64+32, fused leaf and verifier](../2026-11-20-scoped-sdk-transfer-c7a1007/README.md) |
| 2×8 | 12.934 / 15.268 | 12.923 / 15.208 | -15.04% (8/8) / -14.88% (8/8) | 183.4 / 1026.9 | 180.2 / 1026.6 | [48+48, fused leaf and verifier](../2026-11-20-scoped-sdk-transfer-c7a1007/README.md) |
| 2×16 | 15.164 / 16.729 | 14.890 / 16.780 | -10.45% (8/8) / -11.16% (8/8) | 238.6 / 1227.3 | 241.6 / 1235.5 | [64+32, tiled](../2026-11-20-medium-profile-transfer-c7a985/README.md) |
| 3×2 | 13.385 / 16.011 | 13.364 / 15.968 | -16.66% (8/8) / -16.01% (8/8) | 183.1 / 1177.8 | 181.7 / 1183.9 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 3×4 | 13.823 / 16.543 | 13.831 / 16.361 | -16.61% (8/8) / -15.31% (8/8) | 213.1 / 1212.9 | 195.1 / 1204.8 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 3×8 | 14.726 / 17.347 | 14.554 / 17.251 | -14.47% (8/8) / -16.11% (8/8) | 218.5 / 1284.4 | 220.7 / 1280.2 | [64+32, tiled](../2026-11-20-medium-profile-transfer-c7a985/README.md) |
| 4×2 | 13.909 / 17.951 | 13.883 / 18.151 | -22.49% (8/8) / -23.37% (8/8) | 211.7 / 1346.7 | 217.6 / 1363.0 | [72+24, solver 16](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 4×4 | 14.589 / 18.798 | 14.647 / 18.741 | -22.90% (8/8) / -21.96% (8/8) | 216.2 / 1371.8 | 218.4 / 1367.3 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 4×8 | 15.541 / 19.638 | 15.519 / 19.484 | -21.35% (8/8) / -20.28% (8/8) | 241.0 / 1471.2 | 236.4 / 1461.1 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 4×16 | 17.551 / 20.990 | 17.466 / 20.969 | -16.65% (8/8) / -16.79% (8/8) | 263.4 / 1609.1 | 281.6 / 1591.4 | [64+32, tiled](../2026-11-20-medium-profile-transfer-c7a985/README.md) |
| 5×2 | 19.969 / 19.827 | 19.879 / 20.481 | -0.59% (2/4) / -2.09% (3/4) | 220.6 / 1527.9 | 218.4 / 1538.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 5×4 | 20.236 / 20.487 | 20.093 / 20.832 | -1.97% (2/4) / -3.31% (4/4) | 256.0 / 1568.8 | 243.2 / 1561.6 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 5×8 | 24.729 / 25.494 | 24.500 / 25.439 | -3.44% (3/4) / -4.51% (4/4) | 309.4 / 1918.7 | 304.0 / 1951.3 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 5×16 | 26.297 / 26.546 | 25.910 / 26.459 | -0.94% (3/4) / -2.12% (4/4) | 327.1 / 2022.4 | 327.2 / 2017.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 6×2 | 24.730 / 25.980 | 24.849 / 26.749 | -5.70% (4/4) / -7.99% (4/4) | 298.7 / 1993.7 | 311.0 / 1993.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 6×4 | 25.629 / 26.452 | 24.839 / 26.214 | -2.88% (4/4) / -5.23% (4/4) | 320.7 / 1999.4 | 314.5 / 2006.6 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 6×8 | 26.391 / 27.054 | 26.507 / 26.725 | -1.82% (4/4) / -0.91% (3/4) | 351.5 / 2151.2 | 334.9 / 2197.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 8×2 | 18.646 / 30.906 | 18.866 / 30.600 | -39.59% (10/10) / -38.47% (10/10) | 360.3 / 2491.6 | 371.4 / 2476.5 | [72+24, solver 20, balanced LLC](../2026-11-20-grouped-solver-epochs-c7a1011/README.md) |
| 8×4 | 27.868 / 30.332 | 27.385 / 30.920 | -8.47% (4/4) / -10.98% (4/4) | 377.7 / 2466.5 | 371.8 / 2506.8 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 8×8 | 28.869 / 31.323 | 29.422 / 31.127 | -7.58% (4/4) / -5.93% (4/4) | 436.4 / 2551.2 | 399.0 / 2550.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 8×16 | 30.840 / 33.219 | 32.034 / 33.249 | -7.66% (4/4) / -4.85% (4/4) | 460.6 / 2763.1 | 444.0 / 2719.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 12×2 | 27.636 / 47.810 | 28.126 / 46.838 | -41.57% (4/4) / -40.48% (4/4) | 453.9 / 3678.9 | 492.4 / 3682.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 12×4 | 35.018 / 47.033 | 35.859 / 46.775 | -26.13% (4/4) / -23.27% (4/4) | 548.5 / 3703.2 | 528.5 / 3657.6 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 12×8 | 35.751 / 48.104 | 36.411 / 48.317 | -25.62% (4/4) / -24.32% (4/4) | 574.1 / 3730.9 | 559.2 / 3726.9 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 / 56.033 | 31.110 / 55.906 | -44.23% (4/4) / -44.40% (4/4) | 610.8 / 4586.2 | 615.3 / 4590.4 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×4 | 41.304 / 56.713 | 41.418 / 56.573 | -27.04% (4/4) / -26.81% (4/4) | 681.0 / 4635.1 | 689.3 / 4615.3 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×8 | 42.256 / 57.014 | 41.408 / 56.887 | -25.80% (4/4) / -27.15% (4/4) | 701.0 / 4688.7 | 698.5 / 4713.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 / 88.357 | 45.346 / 88.020 | -47.39% (4/4) / -48.99% (4/4) | 921.0 / 6728.6 | 933.2 / 6739.3 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×4 | 56.421 / 87.218 | 56.560 / 87.769 | -36.00% (4/4) / -35.41% (4/4) | 1039.7 / 6700.4 | 1066.4 / 6672.4 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 32×2 | 66.845 / 103.314 | 66.314 / 100.672 | -35.29% (4/4) / -33.88% (4/4) | 1335.7 / 8346.6 | 1327.6 / 8345.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 40×2 | 79.139 / 115.465 | 77.747 / 115.609 | -31.75% (4/4) / -32.87% (4/4) | 1636.1 / 9719.4 | 1532.8 / 9722.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 48×2 | 103.252 / 163.108 | 103.181 / 161.872 | -36.80% (4/4) / -36.67% (4/4) | 1771.0 / 12319.0 | 1814.2 / 12374.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 49×2 | 104.097 / 164.080 | 102.933 / 162.579 | -35.72% (4/4) / -36.69% (4/4) | 1598.0 / 12546.7 | 1685.3 / 12465.3 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-8×1 | 28.506 / 28.516 | 28.693 / 28.898 | -0.04% (2/4) / -0.93% (3/4) | 358.4 / 2355.1 | 361.5 / 2350.5 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 / 86.446 | 42.878 / 87.182 | -46.44% (4/4) / -50.82% (4/4) | 975.3 / 6566.1 | 944.5 / 6584.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-54×1 | 99.494 / 167.786 | 94.024 / 168.535 | -40.70% (4/4) / -44.08% (4/4) | 1846.0 / 12901.7 | 1801.6 / 12934.6 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |

The original 41-shape panel remains available as a single-run baseline. Its paired score was 28 of 41. For 5×2 catalog, the paired difference was −0.59% with two wins in four blocks, while the raw median was 19.969 ms for Zopro and 19.827 ms for Aeglos. This is the narrowest catalog evidence in the consolidated comparison. An earlier coefficient profile won all eight catalog blocks and all four held-out blocks, with 19.754 / 20.692 ms and 19.918 / 20.841 ms respectively. [Earlier 5×2 confirmation](../2026-11-19-equal-core-scorecard707/README.md) [Full-catalog baseline](../2026-11-20-all-shapes-c7a979/README.md)

[All eight M5 reference comparisons](../2026-11-20-geometry-placement-c7a1027/README.md#all-eight-supplied-m5-references) · [Scorecard JSON](artifacts/scorecard.json) · [Request-level CSV](artifacts/scorecard.csv)

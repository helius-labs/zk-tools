# Zopro vs Aeglos on AWS C7a

**Zopro had lower median proof latency on 40 of 41 shapes for the primary request and on all 41 shapes for the held-out request.** Zopro used less peak memory on every shape for both requests.

Start with the first table. Each row is one circuit shape tested on a 96-core AWS c7a.24xlarge. **Proof time** is the warm single-proof latency in milliseconds. **Peak memory** is the highest resident process memory in MiB, including setup. Lower is better for both.

The primary request is the original benchmark input. The held-out request uses a different input for the same shape. Each comparison uses the same input, proving key, host, and timing blocks for both provers. The Zopro build and worker settings vary by shape and are documented in the linked runs.

## Primary request

| Circuit shape | Zopro proof ms | Aeglos proof ms | Faster prover | Zopro peak MiB | Aeglos peak MiB | Benchmark run |
|---|---:|---:|---|---:|---:|---|
| 1×2 | 7.765 | 8.190 | Zopro | 45.2 | 458.9 | [48+48, two G2 ranges](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 1×4 | 8.716 | 9.732 | Zopro | 54.2 | 584.3 | [48+32, fused K/H](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 1×8 | 10.012 | 10.748 | Zopro | 78.3 | 691.0 | [32+64, fused K/H](../2026-11-20-one-input-kh-fusion-c7a1036/README.md) |
| 1×16 | 11.702 | 12.394 | Zopro | 91.0 | 825.7 | [40+40, G1 width 13, fused K/H](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 2×2 | 10.291 | 11.775 | Zopro | 78.9 | 814.5 | [32+32](../2026-11-20-geometry-placement-c7a1027/README.md) |
| 2×4 | 11.161 | 12.325 | Zopro | 186.1 | 821.6 | [64+32, fused leaf and verifier](../2026-11-20-scoped-sdk-transfer-c7a1007/README.md) |
| 2×8 | 12.934 | 15.268 | Zopro | 183.4 | 1026.9 | [48+48, fused leaf and verifier](../2026-11-20-scoped-sdk-transfer-c7a1007/README.md) |
| 2×16 | 15.164 | 16.729 | Zopro | 238.6 | 1227.3 | [64+32, tiled](../2026-11-20-medium-profile-transfer-c7a985/README.md) |
| 3×2 | 13.385 | 16.011 | Zopro | 183.1 | 1177.8 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 3×4 | 13.823 | 16.543 | Zopro | 213.1 | 1212.9 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 3×8 | 14.726 | 17.347 | Zopro | 218.5 | 1284.4 | [64+32, tiled](../2026-11-20-medium-profile-transfer-c7a985/README.md) |
| 4×2 | 13.909 | 17.951 | Zopro | 211.7 | 1346.7 | [72+24, solver 16](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 4×4 | 14.589 | 18.798 | Zopro | 216.2 | 1371.8 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 4×8 | 15.541 | 19.638 | Zopro | 241.0 | 1471.2 | [64+32, tiled](../2026-11-20-grouped-transfers-c7a988/README.md) |
| 4×16 | 17.551 | 20.990 | Zopro | 263.4 | 1609.1 | [64+32, tiled](../2026-11-20-medium-profile-transfer-c7a985/README.md) |
| 5×2 | 19.969 | 19.827 | Aeglos | 220.6 | 1527.9 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 5×4 | 20.236 | 20.487 | Zopro | 256.0 | 1568.8 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 5×8 | 24.729 | 25.494 | Zopro | 309.4 | 1918.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 5×16 | 26.297 | 26.546 | Zopro | 327.1 | 2022.4 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 6×2 | 24.730 | 25.980 | Zopro | 298.7 | 1993.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 6×4 | 25.629 | 26.452 | Zopro | 320.7 | 1999.4 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 6×8 | 26.391 | 27.054 | Zopro | 351.5 | 2151.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 8×2 | 18.646 | 30.906 | Zopro | 360.3 | 2491.6 | [72+24, solver 20, balanced LLC](../2026-11-20-grouped-solver-epochs-c7a1011/README.md) |
| 8×4 | 27.868 | 30.332 | Zopro | 377.7 | 2466.5 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 8×8 | 28.869 | 31.323 | Zopro | 436.4 | 2551.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 8×16 | 30.840 | 33.219 | Zopro | 460.6 | 2763.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 12×2 | 27.636 | 47.810 | Zopro | 453.9 | 3678.9 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 12×4 | 35.018 | 47.033 | Zopro | 548.5 | 3703.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 12×8 | 35.751 | 48.104 | Zopro | 574.1 | 3730.9 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 56.033 | Zopro | 610.8 | 4586.2 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×4 | 41.304 | 56.713 | Zopro | 681.0 | 4635.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×8 | 42.256 | 57.014 | Zopro | 701.0 | 4688.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 88.357 | Zopro | 921.0 | 6728.6 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×4 | 56.421 | 87.218 | Zopro | 1039.7 | 6700.4 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 32×2 | 66.845 | 103.314 | Zopro | 1335.7 | 8346.6 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 40×2 | 79.139 | 115.465 | Zopro | 1636.1 | 9719.4 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 48×2 | 103.252 | 163.108 | Zopro | 1771.0 | 12319.0 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 49×2 | 104.097 | 164.080 | Zopro | 1598.0 | 12546.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-8×1 | 28.506 | 28.516 | Zopro | 358.4 | 2355.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 86.446 | Zopro | 975.3 | 6566.1 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-54×1 | 99.494 | 167.786 | Zopro | 1846.0 | 12901.7 | [Pinned catalog](../2026-11-20-all-shapes-c7a979/README.md) |

## Held-out request

| Circuit shape | Zopro proof ms | Aeglos proof ms | Faster prover | Zopro peak MiB | Aeglos peak MiB |
|---|---:|---:|---|---:|---:|
| 1×2 | 7.685 | 8.162 | Zopro | 48.2 | 458.8 |
| 1×4 | 8.718 | 9.609 | Zopro | 57.3 | 584.0 |
| 1×8 | 9.984 | 10.715 | Zopro | 74.6 | 689.5 |
| 1×16 | 11.668 | 12.392 | Zopro | 89.5 | 831.0 |
| 2×2 | 10.310 | 11.853 | Zopro | 74.5 | 812.5 |
| 2×4 | 11.186 | 12.335 | Zopro | 191.5 | 828.4 |
| 2×8 | 12.923 | 15.208 | Zopro | 180.2 | 1026.6 |
| 2×16 | 14.890 | 16.780 | Zopro | 241.6 | 1235.5 |
| 3×2 | 13.364 | 15.968 | Zopro | 181.7 | 1183.9 |
| 3×4 | 13.831 | 16.361 | Zopro | 195.1 | 1204.8 |
| 3×8 | 14.554 | 17.251 | Zopro | 220.7 | 1280.2 |
| 4×2 | 13.883 | 18.151 | Zopro | 217.6 | 1363.0 |
| 4×4 | 14.647 | 18.741 | Zopro | 218.4 | 1367.3 |
| 4×8 | 15.519 | 19.484 | Zopro | 236.4 | 1461.1 |
| 4×16 | 17.466 | 20.969 | Zopro | 281.6 | 1591.4 |
| 5×2 | 19.879 | 20.481 | Zopro | 218.4 | 1538.2 |
| 5×4 | 20.093 | 20.832 | Zopro | 243.2 | 1561.6 |
| 5×8 | 24.500 | 25.439 | Zopro | 304.0 | 1951.3 |
| 5×16 | 25.910 | 26.459 | Zopro | 327.2 | 2017.1 |
| 6×2 | 24.849 | 26.749 | Zopro | 311.0 | 1993.2 |
| 6×4 | 24.839 | 26.214 | Zopro | 314.5 | 2006.6 |
| 6×8 | 26.507 | 26.725 | Zopro | 334.9 | 2197.1 |
| 8×2 | 18.866 | 30.600 | Zopro | 371.4 | 2476.5 |
| 8×4 | 27.385 | 30.920 | Zopro | 371.8 | 2506.8 |
| 8×8 | 29.422 | 31.127 | Zopro | 399.0 | 2550.2 |
| 8×16 | 32.034 | 33.249 | Zopro | 444.0 | 2719.7 |
| 12×2 | 28.126 | 46.838 | Zopro | 492.4 | 3682.1 |
| 12×4 | 35.859 | 46.775 | Zopro | 528.5 | 3657.6 |
| 12×8 | 36.411 | 48.317 | Zopro | 559.2 | 3726.9 |
| 16×2 | 31.110 | 55.906 | Zopro | 615.3 | 4590.4 |
| 16×4 | 41.418 | 56.573 | Zopro | 689.3 | 4615.3 |
| 16×8 | 41.408 | 56.887 | Zopro | 698.5 | 4713.2 |
| 24×2 | 45.346 | 88.020 | Zopro | 933.2 | 6739.3 |
| 24×4 | 56.560 | 87.769 | Zopro | 1066.4 | 6672.4 |
| 32×2 | 66.314 | 100.672 | Zopro | 1327.6 | 8345.7 |
| 40×2 | 77.747 | 115.609 | Zopro | 1532.8 | 9722.1 |
| 48×2 | 103.181 | 161.872 | Zopro | 1814.2 | 12374.2 |
| 49×2 | 102.933 | 162.579 | Zopro | 1685.3 | 12465.3 |
| merge-8×1 | 28.693 | 28.898 | Zopro | 361.5 | 2350.5 |
| merge-24×1 | 42.878 | 87.182 | Zopro | 944.5 | 6584.7 |
| merge-54×1 | 94.024 | 168.535 | Zopro | 1801.6 | 12934.6 |

## Timing details

The faster prover column compares the two displayed medians of warm timing blocks. On the 5×2 primary request, Aeglos led by 0.142 ms and each prover won two of the four blocks. Across the full scorecard, comparing the block-by-block ratios gave Zopro a lower paired median on both requests for all 41 shapes. Zopro won every timed block on both requests for 34 shapes.

The earlier 1×8 profile peaked at 138.2 MiB. With the same worker layout, K/H fusion brought the new 1×8 result to 78.3 MiB. K/H fusion also brought the 1×16 primary peak from 165.2 MiB to 91.0 MiB at fixed placement. [Matched 1×8 measurements](../2026-11-20-one-input-kh-fusion-c7a1036/README.md) · [Matched 1×16 measurements](../2026-11-20-geometry-placement-c7a1027/README.md)

The original 41-shape run and the later follow-ups used different Zopro profiles. The CSV and JSON include paired percentages and win counts. The benchmark runs linked in the table include the raw timing blocks. [Original full-catalog baseline](../2026-11-20-all-shapes-c7a979/README.md) · [Earlier 5×2 confirmation](../2026-11-19-equal-core-scorecard707/README.md)

[Download CSV](artifacts/scorecard.csv) · [Scorecard JSON](artifacts/scorecard.json) · [Concurrent proof benchmark](../2026-11-20-c7a-concurrent-proofs/README.md) · [K/H fusion analysis for all 41 shapes](../2026-11-20-kh-fusion-geometry-all-shapes/README.md) · [M5 reference comparisons](../2026-11-20-geometry-placement-c7a1027/README.md#all-eight-supplied-m5-references)

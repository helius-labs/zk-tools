# Equal-core confirmation on all 41 C7a shapes

With 96 physical cores per prover, the selected Zopro configuration had a lower median warm single-proof latency on **28/41** shapes. It won all eight paired blocks on **26/41**. Aeglos matched or led the median on **13/41**. The selected configuration at least halved the Zopro release latency measured with the established per-shape catalog settings on this instance on **0/41**. Selection and timing used the same original request per shape.

The first comparison used six paired blocks with the catalog release configuration and prepared Aeglos. Where that comparison left a loss or a mixed result, a four-block screen compared smaller frontend pools and alternative FFT and G2 configurations with the main pool fixed at 96. The 1×2 screen additionally included its measured server profile. Eligible shapes also tested the small public coefficient files from experiments 691 and 693, with and without packed FFT. Additional arms compared idle search limits on the 96-worker main pool, pairwise G2 bucket merging, and balanced trees of unshifted G2 points on shapes with a verified local dispatch. Disabled controls used the same candidate executable as their respective enabled arms. The best median Zopro screen result for each screened shape advanced. The remaining shapes used the qualified catalog configuration. Every shape then ran eight fresh confirmation blocks, reversing order within consecutive pairs. Each process had three untimed warmups and eleven timed proofs. The table shows medians of process means and within-block percentage changes. Peak RSS covers the entire process, including setup. Every timed Aeglos proof hit the prepared cache. All 1660 screen and confirmation process exports passed remote and independent local verification.

| Shape | Selected configuration | Zopro ms | Aeglos ms | Paired change (Zopro wins) | Peak MiB Zopro / Aeglos | Warm CPU-ms Zopro / Aeglos | Speed vs catalog release |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | pair-on | 8.001 | 8.384 | -4.57% (8/8) | 45.2 / 457.6 | 369.9 / 514.5 | 1.18× |
| 1x4 | bands-tree2 | 11.808 | 9.761 | +21.34% (0/8) | 111.9 / 587.7 | 602.2 / 661.3 | 1.31× |
| 1x8 | wide32 | 13.810 | 10.741 | +28.30% (0/8) | 78.9 / 687.2 | 668.3 / 734.6 | 1.21× |
| 1x16 | runtime-off | 16.741 | 12.343 | +35.12% (0/8) | 104.4 / 818.7 | 727.6 / 881.3 | 1.20× |
| 2x2 | bands-tree2 | 12.619 | 11.761 | +6.90% (0/8) | 132.2 / 809.9 | 647.3 / 834.1 | 1.10× |
| 2x4 | bands-tree2 | 13.896 | 12.357 | +11.93% (0/8) | 144.1 / 824.2 | 691.8 / 862.7 | 1.23× |
| 2x8 | wide32 | 15.585 | 15.297 | +2.12% (1/8) | 124.7 / 1018.1 | 799.3 / 1137.4 | 1.26× |
| 2x16 | coeff-packed | 22.060 | 16.985 | +30.47% (0/8) | 261.2 / 1228.2 | 1201.6 / 1257.0 | 1.03× |
| 3x2 | runtime-off | 17.549 | 16.136 | +8.44% (0/8) | 190.7 / 1166.1 | 1079.2 / 1208.5 | 1.07× |
| 3x4 | coeff | 17.835 | 16.644 | +7.83% (0/8) | 200.3 / 1213.6 | 1111.9 / 1236.3 | 1.11× |
| 3x8 | runtime-off | 19.974 | 17.519 | +13.88% (0/8) | 236.5 / 1287.8 | 1200.0 / 1331.3 | 1.05× |
| 4x2 | coeff | 18.289 | 18.632 | -0.94% (4/8) | 234.9 / 1351.3 | 1130.5 / 1388.1 | 1.05× |
| 4x4 | coeff | 19.220 | 18.902 | +2.14% (3/8) | 221.5 / 1364.6 | 1174.2 / 1396.2 | 1.13× |
| 4x8 | front16 | 21.476 | 19.861 | +8.28% (0/8) | 262.8 / 1456.7 | 1353.3 / 1483.6 | 1.04× |
| 4x16 | front16 | 23.566 | 21.463 | +9.02% (0/8) | 322.8 / 1581.9 | 1497.2 / 1582.0 | 1.06× |
| 5x2 | coeff | 19.754 | 20.692 | -4.92% (8/8) | 249.2 / 1546.4 | 1204.0 / 1571.3 | 1.11× |
| 5x4 | coeff-packed | 20.994 | 21.003 | -2.00% (6/8) | 263.4 / 1591.2 | 1160.2 / 1586.6 | 1.07× |
| 5x8 | release | 24.895 | 26.261 | -5.02% (8/8) | 344.6 / 1933.8 | 1687.3 / 2125.0 | 1.00× |
| 5x16 | coeff-packed | 26.466 | 27.639 | -4.59% (8/8) | 336.9 / 2064.5 | 1480.4 / 2257.3 | 1.03× |
| 6x2 | release | 24.168 | 27.277 | -11.30% (8/8) | 354.3 / 2006.6 | 1686.0 / 2220.4 | 1.02× |
| 6x4 | release | 25.074 | 27.469 | -8.68% (8/8) | 359.6 / 2007.3 | 1713.1 / 2221.1 | 1.01× |
| 6x8 | release | 25.992 | 28.091 | -7.45% (8/8) | 373.9 / 2206.1 | 1798.1 / 2326.4 | 1.02× |
| 8x2 | release | 25.794 | 30.747 | -16.56% (8/8) | 415.5 / 2452.9 | 1754.6 / 2568.8 | 1.00× |
| 8x4 | release | 27.552 | 31.238 | -11.63% (8/8) | 425.0 / 2493.7 | 1950.4 / 2612.6 | 1.02× |
| 8x8 | release | 28.629 | 32.375 | -12.25% (8/8) | 451.5 / 2569.6 | 2009.3 / 2678.8 | 1.01× |
| 8x16 | release | 30.909 | 34.431 | -10.23% (8/8) | 497.3 / 2737.0 | 2159.8 / 2819.8 | 1.02× |
| 12x2 | release | 32.603 | 48.611 | -32.93% (8/8) | 546.9 / 3697.1 | 2130.7 / 4117.5 | 1.00× |
| 12x4 | release | 35.924 | 48.765 | -26.22% (8/8) | 576.0 / 3729.2 | 2501.1 / 4148.6 | 0.97× |
| 12x8 | release | 36.099 | 49.532 | -26.33% (8/8) | 606.4 / 3732.7 | 2553.1 / 4204.4 | 1.01× |
| 16x2 | release | 34.278 | 56.964 | -40.03% (8/8) | 651.9 / 4610.4 | 2341.0 / 4868.3 | 1.01× |
| 16x4 | release | 41.742 | 57.549 | -27.75% (8/8) | 745.5 / 4638.2 | 2806.3 / 4914.4 | 1.01× |
| 16x8 | release | 42.113 | 58.327 | -27.71% (8/8) | 760.6 / 4728.3 | 2850.3 / 4980.0 | 1.01× |
| 24x2 | release | 47.046 | 87.628 | -46.95% (8/8) | 991.6 / 6749.8 | 3253.1 / 7434.1 | 1.01× |
| 24x4 | release | 57.174 | 88.177 | -34.98% (8/8) | 1159.9 / 6737.0 | 3665.2 / 7478.5 | 1.00× |
| 32x2 | release | 67.100 | 102.224 | -34.08% (8/8) | 1435.2 / 8355.4 | 4087.2 / 8681.7 | 1.02× |
| 40x2 | release | 78.764 | 117.909 | -33.55% (8/8) | 1673.9 / 9767.6 | 4469.8 / 9996.8 | 1.03× |
| 48x2 | release | 103.680 | 166.687 | -37.83% (8/8) | 1871.8 / 12451.1 | 5781.8 / 14257.7 | 1.01× |
| 49x2 | release | 106.354 | 168.649 | -36.90% (8/8) | 1799.9 / 12585.3 | 5852.5 / 14441.4 | 1.00× |
| merge-8x1 | release | 28.546 | 29.924 | -4.79% (8/8) | 362.0 / 2367.3 | 1985.3 / 2493.5 | 1.01× |
| merge-24x1 | release | 51.427 | 85.223 | -39.64% (8/8) | 1039.5 / 6573.9 | 3344.3 / 7265.6 | 1.00× |
| merge-54x1 | release | 102.684 | 173.714 | -41.01% (8/8) | 1877.1 / 13040.1 | 5807.7 / 14938.3 | 1.00× |

## Supplied M5 references

| Shape | Selected C7a ms | M5 CPU / Metal ms | Speed vs CPU / Metal | Blocks at 10× M5 CPU |
|---|---:|---:|---:|---:|
| 1x2 | 8.001 | 37.0 / 24.0 | 4.62× / 3.00× | 0/8 |
| 2x2 | 12.619 | 59.0 / 30.0 | 4.68× / 2.38× | 0/8 |
| 4x2 | 18.289 | 109.0 / 48.0 | 5.96× / 2.62× | 0/8 |
| 8x2 | 25.794 | 189.0 / 74.0 | 7.33× / 2.87× | 0/8 |
| 12x2 | 32.603 | 284.0 / 101.0 | 8.71× / 3.10× | 0/8 |
| 16x2 | 34.278 | 335.0 / 124.0 | 9.77× / 3.62× | 1/8 |
| 24x2 | 47.046 | 528.0 / 176.0 | 11.22× / 3.74× | 8/8 |
| merge-24x1 | 51.427 | 507.0 / 175.0 | 9.86× / 3.40× | 1/8 |

[Same-host catalog](../2026-11-19-c7a-catalog687/README.md) · [Configuration selection](artifacts/selection.json) · [Fresh paired runs](artifacts/loss-confirm-rows.json.gz) · [Independent verification](artifacts/verification.json)

## Reproducing a selected configuration

The [per-key configuration manifest](artifacts/profiles.json) lists the exact SDK, image and optional file hashes plus environment settings for every shape. Each entry inherits the common environment. The frontend CPU list can be narrower than the 96-worker main mask. Supply the host’s key and Lagrange-cache directories and use the matching SDK implementation. The [complete command workflow](artifacts/loss-confirm-workflow.json.gz) includes affinity, verifier arguments and artifact paths on the benchmark host.

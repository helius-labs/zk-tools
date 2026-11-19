# Which proving stages scale from 32 to 96 C7a cores?

On this c7a.24xlarge, moving from 32 to 96 cores cut prepared Aeglos’s full proof time on 41/41 shapes in the six-block catalog. The median across those shapes of its MSM timing gain divided by the full-proof timing gain was 89%. The ratio is descriptive and can exceed 100% when time elsewhere grows. Both differences use paired blocks.

## Two shapes with additional Zopro instrumentation

Six ordinary processes per engine and width supplied end-to-end latency. Four additional processes per shape and width traced Zopro’s solver, FFT and curve intervals. Aeglos’s internal timers ran in its ordinary executable. Its MSM covers its complete MSM stage. Zopro’s curve union covers observed G1 and G2 calls. Source boundaries and instrumentation overhead differ. The last column counts elapsed time covered by only one of the two Zopro curves, taking the set difference within each request.

| Shape | Cores | Plain proof Zopro / Aeglos ms | Solver Zopro / Aeglos ms | FFT Zopro / Aeglos ms | Zopro curve union / Aeglos MSM ms | Zopro G1 only / G2 only ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x16 | 32 | 17.335 / 19.863 | 2.885 / 1.367 | 1.543 / 1.654 | 11.994 / 15.141 | 0.163 / 1.671 |
| 1x16 | 96 | 19.569 / 12.455 | 4.176 / 1.786 | 2.079 / 1.314 | 12.841 / 7.468 | 0.065 / 3.738 |
| 5x16 | 32 | 27.762 / 47.659 | 4.738 / 2.041 | 3.831 / 6.673 | 17.597 / 36.915 | 5.795 / 0.443 |
| 5x16 | 96 | 27.360 / 27.478 | 6.303 / 1.886 | 3.660 / 3.583 | 14.294 / 19.177 | 6.923 / 0.648 |

The 1×16 widths show opposite trends in the two curve implementations. The 96-core Zopro trace also spends longer in its solver. At 5×16, more cores shorten Zopro's G2 while G1 occupies most of its curve interval. These observations motivate separate experiments for the two cases. Removing a G2 operation can free resources for G1, so its exclusive elapsed time describes the current schedule and does not bound a redesigned prover.

## Full catalog

Each cell shows 32-core → 96-core median milliseconds. Each endpoint is the median of six process means, each with seven timed proofs following three warmups. Both engines used the same physical core mask in each pair.

| Shape | Zopro full | Aeglos full | Aeglos MSM | Aeglos witness | Aeglos FFT | MSM share of Aeglos full gain |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 10.14 → 9.46 | 12.25 → 8.38 | 8.60 → 4.30 | 1.28 → 1.55 | 0.90 → 0.92 | 109% |
| 1x4 | 13.38 → 15.43 | 15.43 → 9.85 | 10.90 → 5.32 | 1.24 → 1.62 | 1.75 → 1.31 | 99% |
| 1x8 | 14.66 → 16.68 | 16.91 → 10.81 | 12.43 → 6.08 | 1.28 → 1.66 | 1.72 → 1.35 | 104% |
| 1x16 | 17.44 → 20.16 | 20.07 → 12.75 | 15.21 → 7.80 | 1.33 → 1.73 | 1.71 → 1.34 | 103% |
| 2x2 | 13.80 → 13.94 | 19.44 → 11.82 | 14.69 → 6.85 | 1.34 → 1.76 | 1.76 → 1.29 | 100% |
| 2x4 | 15.38 → 17.03 | 19.45 → 12.48 | 14.78 → 7.42 | 1.39 → 1.79 | 1.69 → 1.30 | 107% |
| 2x8 | 17.74 → 19.70 | 25.26 → 15.35 | 18.92 → 9.64 | 1.26 → 1.75 | 3.36 → 2.06 | 94% |
| 2x16 | 21.65 → 22.68 | 28.41 → 16.72 | 21.82 → 10.63 | 1.27 → 1.92 | 3.39 → 2.04 | 97% |
| 3x2 | 18.01 → 18.76 | 26.59 → 16.00 | 20.41 → 10.11 | 1.32 → 1.86 | 3.24 → 2.03 | 98% |
| 3x4 | 18.68 → 19.85 | 27.71 → 17.15 | 21.29 → 11.02 | 1.36 → 1.83 | 3.35 → 2.04 | 98% |
| 3x8 | 20.15 → 20.88 | 28.94 → 17.50 | 22.58 → 11.37 | 1.32 → 1.92 | 3.28 → 2.04 | 97% |
| 4x2 | 18.81 → 19.29 | 31.07 → 18.33 | 24.69 → 12.29 | 1.30 → 2.04 | 3.37 → 2.09 | 98% |
| 4x4 | 20.54 → 21.73 | 31.49 → 19.04 | 24.85 → 12.49 | 1.42 → 2.36 | 3.35 → 2.03 | 100% |
| 4x8 | 21.82 → 22.32 | 33.11 → 19.99 | 26.10 → 13.40 | 1.80 → 2.21 | 3.27 → 1.99 | 98% |
| 4x16 | 24.78 → 24.99 | 36.18 → 21.48 | 28.94 → 14.39 | 2.02 → 2.35 | 3.28 → 1.92 | 98% |
| 5x2 | 21.46 → 21.96 | 35.56 → 20.75 | 28.50 → 13.94 | 1.98 → 2.34 | 3.31 → 1.92 | 99% |
| 5x4 | 22.05 → 22.55 | 35.76 → 21.24 | 28.75 → 14.26 | 1.73 → 2.28 | 3.31 → 1.93 | 100% |
| 5x8 | 24.91 → 24.87 | 44.72 → 26.24 | 34.46 → 17.91 | 1.66 → 1.91 | 6.54 → 3.62 | 91% |
| 5x16 | 27.77 → 27.14 | 48.13 → 27.66 | 37.13 → 19.47 | 2.09 → 1.78 | 6.68 → 3.70 | 87% |
| 6x2 | 24.47 → 24.73 | 47.67 → 27.13 | 36.85 → 18.78 | 1.95 → 1.81 | 6.69 → 3.59 | 89% |
| 6x4 | 24.98 → 25.41 | 46.88 → 27.24 | 36.40 → 19.14 | 1.77 → 1.87 | 6.68 → 3.62 | 88% |
| 6x8 | 26.18 → 26.42 | 48.55 → 28.24 | 37.73 → 20.04 | 1.97 → 1.73 | 6.62 → 3.64 | 87% |
| 8x2 | 26.51 → 25.92 | 54.52 → 30.91 | 43.26 → 22.54 | 2.32 → 2.01 | 6.64 → 3.73 | 87% |
| 8x4 | 27.87 → 27.99 | 55.08 → 31.29 | 43.85 → 22.72 | 2.65 → 2.03 | 6.66 → 3.71 | 88% |
| 8x8 | 29.44 → 28.95 | 56.89 → 32.44 | 45.27 → 23.72 | 2.71 → 2.18 | 6.66 → 3.71 | 88% |
| 8x16 | 32.15 → 31.67 | 59.85 → 34.61 | 47.75 → 25.26 | 3.08 → 2.24 | 6.66 → 3.60 | 89% |
| 12x2 | 33.41 → 32.70 | 86.96 → 48.53 | 66.89 → 34.94 | 3.52 → 2.60 | 13.65 → 7.57 | 85% |
| 12x4 | 35.98 → 34.87 | 86.23 → 48.23 | 67.06 → 34.72 | 3.22 → 2.64 | 13.42 → 7.55 | 85% |
| 12x8 | 37.07 → 36.48 | 89.54 → 49.97 | 69.13 → 35.90 | 4.20 → 2.82 | 13.75 → 7.65 | 84% |
| 16x2 | 40.15 → 34.69 | 103.09 → 57.08 | 81.79 → 42.11 | 4.71 → 3.83 | 13.60 → 7.70 | 87% |
| 16x4 | 42.00 → 42.16 | 104.74 → 56.99 | 82.51 → 42.17 | 5.27 → 3.45 | 13.80 → 7.70 | 86% |
| 16x8 | 43.52 → 42.51 | 106.96 → 58.94 | 84.45 → 43.30 | 5.67 → 3.99 | 13.87 → 7.85 | 86% |
| 24x2 | 58.06 → 47.41 | 173.52 → 87.27 | 131.27 → 63.62 | 8.72 → 4.52 | 29.80 → 14.96 | 79% |
| 24x4 | 61.15 → 57.34 | 171.81 → 87.53 | 130.55 → 63.45 | 8.86 → 5.29 | 29.44 → 14.80 | 80% |
| 32x2 | 72.89 → 68.58 | 200.98 → 100.39 | 157.13 → 74.96 | 9.95 → 5.97 | 29.66 → 14.54 | 82% |
| 40x2 | 84.68 → 81.43 | 233.66 → 117.50 | 185.23 → 89.67 | 13.36 → 7.51 | 30.08 → 14.67 | 83% |
| 48x2 | 130.39 → 104.60 | 333.49 → 165.36 | 251.84 → 119.73 | 16.40 → 8.93 | 60.47 → 30.34 | 79% |
| 49x2 | 139.39 → 105.86 | 334.09 → 168.22 | 255.33 → 121.46 | 15.37 → 10.59 | 58.23 → 30.36 | 81% |
| merge-8x1 | 27.21 → 28.89 | 51.88 → 30.26 | 41.25 → 22.01 | 2.05 → 1.90 | 6.64 → 3.80 | 89% |
| merge-24x1 | 58.54 → 51.48 | 164.69 → 85.69 | 126.26 → 61.49 | 6.71 → 5.25 | 28.83 → 14.80 | 82% |
| merge-54x1 | 134.89 → 102.49 | 343.94 → 173.05 | 266.05 → 126.88 | 14.12 → 9.60 | 57.90 → 30.51 | 82% |

This analysis reuses independently verified process exports from the [width pilot](../2026-11-19-aws-width686/README.md), [phase traces](../2026-11-19-phase-c7a690/README.md) and [full catalog](../2026-11-19-c7a-catalog687/README.md). [Pilot measurements](artifacts/pilot.json) · [Per-shape results](artifacts/catalog.json)

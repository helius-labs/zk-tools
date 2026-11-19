# Placing individual CPU curve queries on disjoint C7a cores

The candidate uses the current SDK's general folded-CPU route and its opt-in FFT output fusion. It can place the G1 A, K or H evaluation on the existing frontend pool. It uses the same arithmetic and borrows the existing key and proof buffers. The receiving pool's scratch estimate must pass the existing memory admission check. The control uses the same executable and placement with all curves on the main pool. Other arms use the existing frontend FFT option alone or with H. The H runs at 56+40 and 52+44 cores have matched controls at those widths.

Six randomized blocks on each of two transactions per shape prime three requests and time thirteen. The separate diagnostic panel interleaves clock-on and clock-off variants, with nine timed requests per process. Intervals cover the named call sites. A, K, H and B2 may overlap, and elapsed times can include scheduler delays. The B2 interval can include grouping or the omitted B1 branch. Timed speedups use clock-off processes. The zero-byte optional preparation check verified the fallback. All 364 C7a exports and 14 Ryzen exports passed independent verification. Temporary AWS resources were removed.

| Shape | Input | Mode | Proof ms | Paired vs same-layout off | Paired vs current SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|---:|---:|
| 4x2 | catalog | a | 16.330 | off, -2.66% (6/6) | -2.03% (6/6) | -11.03% (6/6) | 224.4 | 6.67× / 2.94× |
| 4x2 | catalog | aeglos | 18.376 | off, +9.86% (0/6) | +10.24% (0/6) | +0.00% (0/6) | 1347.2 | 5.93× / 2.61× |
| 4x2 | catalog | fft-front | 17.997 | off, +7.92% (0/6) | +8.09% (0/6) | -2.14% (4/6) | 169.0 | 6.06× / 2.67× |
| 4x2 | catalog | h | 16.906 | off, +1.33% (1/6) | +1.35% (0/6) | -7.81% (6/6) | 229.0 | 6.45× / 2.84× |
| 4x2 | catalog | hfft | 17.069 | off, +2.77% (0/6) | +3.01% (0/6) | -5.45% (6/6) | 179.5 | 6.39× / 2.81× |
| 4x2 | catalog | k | 15.326 | off, -7.67% (6/6) | -7.17% (6/6) | -15.96% (6/6) | 205.0 | 7.11× / 3.13× |
| 4x2 | catalog | off | 16.662 | off, +0.00% (0/6) | +0.78% (2/6) | -8.97% (6/6) | 231.2 | 6.54× / 2.88× |
| 4x2 | catalog | release | 16.648 | off, -0.77% (4/6) | +0.00% (0/6) | -9.28% (6/6) | 222.9 | 6.55× / 2.88× |
| 4x2 | heldout | a | 16.495 | off, -1.61% (5/6) | -0.67% (5/6) | -9.68% (6/6) | 229.7 | — |
| 4x2 | heldout | aeglos | 18.184 | off, +8.88% (0/6) | +9.37% (0/6) | +0.00% (0/6) | 1339.7 | — |
| 4x2 | heldout | fft-front | 17.972 | off, +7.39% (0/6) | +8.29% (0/6) | -1.41% (5/6) | 158.5 | — |
| 4x2 | heldout | h | 16.991 | off, +1.49% (0/6) | +2.33% (1/6) | -6.81% (6/6) | 228.1 | — |
| 4x2 | heldout | hfft | 17.079 | off, +1.21% (0/6) | +2.75% (1/6) | -6.57% (6/6) | 171.1 | — |
| 4x2 | heldout | k | 15.518 | off, -6.77% (6/6) | -6.53% (6/6) | -14.91% (6/6) | 209.4 | — |
| 4x2 | heldout | off | 16.806 | off, +0.00% (0/6) | +0.09% (3/6) | -8.15% (6/6) | 234.9 | — |
| 4x2 | heldout | release | 16.637 | off, -0.09% (3/6) | +0.00% (0/6) | -8.57% (6/6) | 220.7 | — |
| 8x2 | catalog | a | 22.121 | off, -2.33% (6/6) | -1.66% (6/6) | -27.46% (6/6) | 418.2 | 8.54× / 3.35× |
| 8x2 | catalog | aeglos | 30.480 | off, +34.42% (0/6) | +35.56% (0/6) | +0.00% (0/6) | 2457.0 | 6.20× / 2.43× |
| 8x2 | catalog | fft-front | 24.141 | off, +7.09% (0/6) | +7.45% (0/6) | -20.75% (6/6) | 370.4 | 7.83× / 3.07× |
| 8x2 | catalog | h | 23.439 | off, +3.45% (0/6) | +3.64% (0/6) | -23.25% (6/6) | 432.6 | 8.06× / 3.16× |
| 8x2 | catalog | h52 | 22.510 | off52, -0.69% (5/6) | -0.13% (4/6) | -26.24% (6/6) | 440.7 | 8.40× / 3.29× |
| 8x2 | catalog | h56 | 22.995 | off56, +2.21% (1/6) | +1.77% (0/6) | -24.47% (6/6) | 436.7 | 8.22× / 3.22× |
| 8x2 | catalog | hfft | 24.001 | off, +5.80% (0/6) | +6.04% (0/6) | -21.63% (6/6) | 364.9 | 7.87× / 3.08× |
| 8x2 | catalog | k | 21.283 | off, -6.36% (6/6) | -6.15% (6/6) | -30.48% (6/6) | 406.2 | 8.88× / 3.48× |
| 8x2 | catalog | off | 22.598 | off, +0.00% (0/6) | +0.20% (2/6) | -25.61% (6/6) | 442.3 | 8.36× / 3.27× |
| 8x2 | catalog | off52 | 22.648 | off52, +0.00% (0/6) | +0.18% (3/6) | -25.54% (6/6) | 439.3 | 8.35× / 3.27× |
| 8x2 | catalog | off56 | 22.504 | off56, +0.00% (0/6) | +0.05% (3/6) | -26.19% (6/6) | 443.5 | 8.40× / 3.29× |
| 8x2 | catalog | release | 22.539 | off, -0.20% (4/6) | +0.00% (0/6) | -26.23% (6/6) | 442.4 | 8.39× / 3.28× |
| 8x2 | heldout | a | 22.498 | off, -1.21% (6/6) | -1.43% (6/6) | -26.62% (6/6) | 432.3 | — |
| 8x2 | heldout | aeglos | 30.702 | off, +35.02% (0/6) | +34.70% (0/6) | +0.00% (0/6) | 2496.2 | — |
| 8x2 | heldout | fft-front | 24.407 | off, +7.66% (0/6) | +6.98% (0/6) | -20.58% (6/6) | 359.0 | — |
| 8x2 | heldout | h | 23.803 | off, +4.76% (0/6) | +4.04% (0/6) | -22.76% (6/6) | 441.7 | — |
| 8x2 | heldout | h52 | 22.765 | off52, -0.70% (6/6) | -0.09% (4/6) | -25.77% (6/6) | 440.0 | — |
| 8x2 | heldout | h56 | 23.388 | off56, +2.48% (0/6) | +2.29% (0/6) | -23.94% (6/6) | 440.1 | — |
| 8x2 | heldout | hfft | 24.217 | off, +6.64% (0/6) | +6.32% (0/6) | -20.91% (6/6) | 355.5 | — |
| 8x2 | heldout | k | 21.414 | off, -5.89% (6/6) | -6.14% (6/6) | -30.26% (6/6) | 411.6 | — |
| 8x2 | heldout | off | 22.760 | off, +0.00% (0/6) | -0.40% (4/6) | -25.94% (6/6) | 449.3 | — |
| 8x2 | heldout | off52 | 22.902 | off52, +0.00% (0/6) | +0.64% (2/6) | -24.78% (6/6) | 437.4 | — |
| 8x2 | heldout | off56 | 22.894 | off56, +0.00% (0/6) | -0.11% (3/6) | -25.68% (6/6) | 454.4 | — |
| 8x2 | heldout | release | 22.823 | off, +0.41% (2/6) | +0.00% (0/6) | -25.76% (6/6) | 440.3 | — |

| Shape | Input | Mode | Clock overhead | Solve / gather / FFT ms | A / K / H / B2 ms | Last curve return A / K / H / B2 (out of 27) |
|---|---|---|---:|---:|---:|---:|
| 4x2 | catalog | off | -0.12% | 3.615 / 0.761 / 1.955 | 6.453 / 8.111 / 6.685 / 8.264 | 3 / 10 / 0 / 14 |
| 4x2 | catalog | a | +0.85% | 3.638 / 0.794 / 1.998 | 4.536 / 8.222 / 5.955 / 7.742 | 0 / 11 / 1 / 15 |
| 4x2 | catalog | k | -0.47% | 3.618 / 0.790 / 2.044 | 6.113 / 6.220 / 5.859 / 7.276 | 2 / 12 / 0 / 13 |
| 4x2 | catalog | h | +1.32% | 3.555 / 0.807 / 2.015 | 6.831 / 8.366 / 6.061 / 8.481 | 2 / 8 / 0 / 17 |
| 4x2 | heldout | off | -2.64% | 3.586 / 0.773 / 1.995 | 5.464 / 7.750 / 6.286 / 8.259 | 3 / 6 / 2 / 16 |
| 4x2 | heldout | a | +0.29% | 3.611 / 0.812 / 2.013 | 4.538 / 7.962 / 5.596 / 8.128 | 0 / 11 / 0 / 16 |
| 4x2 | heldout | k | +0.51% | 3.617 / 0.801 / 2.029 | 4.846 / 6.543 / 5.413 / 7.267 | 2 / 10 / 2 / 13 |
| 4x2 | heldout | h | -0.29% | 3.652 / 0.805 / 2.010 | 6.019 / 7.763 / 6.067 / 8.370 | 1 / 5 / 0 / 21 |
| 8x2 | catalog | off | +1.82% | 5.138 / 0.930 / 2.786 | 9.206 / 10.731 / 9.913 / 11.247 | 3 / 9 / 1 / 14 |
| 8x2 | catalog | a | +0.51% | 5.177 / 0.928 / 2.770 | 6.228 / 9.794 / 10.292 / 10.919 | 0 / 11 / 1 / 15 |
| 8x2 | catalog | k | +1.47% | 5.146 / 0.913 / 2.825 | 7.502 / 7.709 / 8.266 / 10.445 | 2 / 0 / 4 / 21 |
| 8x2 | catalog | h | +0.59% | 5.095 / 0.878 / 2.798 | 9.153 / 10.372 / 11.455 / 11.649 | 2 / 5 / 16 / 4 |
| 8x2 | heldout | off | -0.95% | 5.125 / 0.963 / 2.828 | 9.175 / 10.886 / 10.284 / 11.548 | 2 / 7 / 3 / 15 |
| 8x2 | heldout | a | -1.22% | 5.081 / 0.949 / 2.808 | 6.465 / 11.087 / 10.613 / 11.091 | 0 / 8 / 3 / 16 |
| 8x2 | heldout | k | -1.17% | 5.247 / 0.975 / 2.810 | 7.949 / 8.367 / 8.660 / 10.440 | 2 / 0 / 4 / 21 |
| 8x2 | heldout | h | +0.27% | 5.034 / 0.920 / 2.750 | 8.407 / 10.795 / 11.236 / 11.931 | 0 / 7 / 10 / 10 |

[Samples and clocks](artifacts/rows.json.gz) · [Exact workflow](artifacts/workflow.json.gz) · [Source patch](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
## Run qualification

A first C7a attempt stopped when the SDK rejected a frontend pool as wide as the main pool. The published run substitutes 52+44 for that arm. Timings and proofs from the stopped attempt are excluded. Both instances were terminated and both temporary buckets removed.

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured C7a profiles. [Previous scorecard](../2026-11-19-middle-groups-c7a751/README.md)

| Shape | C7a profile | C7a ms | M5 CPU / Metal ms | Speedup CPU / Metal | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 753 k | 15.326 | 109 / 48 | 7.11× / 3.13× | +4.426 |
| 8×2 | 753 k | 21.283 | 189 / 74 | 8.88× / 3.48× | +2.383 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

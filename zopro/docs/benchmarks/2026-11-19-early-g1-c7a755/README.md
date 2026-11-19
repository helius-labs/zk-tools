# Overlapping G1 A and K with the quotient FFT

The trial starts A, K or both on the existing 32-core frontend while the 64-core main pool evaluates the packed, fused quotient FFT. As soon as the FFT returns, the main pool can compute the remaining curve queries. This uses the existing G1 arithmetic, immutable key and proof-local scalars. A skipped query has its result supplied by the frontend. Workspace admission is computed using the selected frontend width. The route declines if the available scratch is insufficient or the frontend is requested for the FFT. Diagnostic exports cover both fallbacks.

The late variants place the same A, K or H work on the frontend after the FFT. Timing uses six matched blocks per input, with three warmups and thirteen measured calls per process. A separate route panel checks pool use and whether the early query overlaps the FFT. Its intervals are individual diagnostics, separate from the timing panel. All 240 C7a and 12 Ryzen exports independently verified. Temporary resources were removed.

| Shape | Input | Configuration | Proof ms | Paired vs off | Paired vs named control | Paired vs SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4x2 | catalog | aeglos | 18.258 | +8.96% (0/6) | off, +8.96% (0/6) | +9.32% (0/6) | +0.00% (0/6) | 1361.6 | 5.97× / 2.63× |
| 4x2 | catalog | early-a | 17.823 | +6.48% (0/6) | late-a, +9.49% (0/6) | +6.63% (0/6) | -2.16% (6/6) | 201.9 | 6.12× / 2.69× |
| 4x2 | catalog | early-both | 16.267 | -2.92% (6/6) | off, -2.92% (6/6) | -3.32% (6/6) | -11.23% (6/6) | 196.8 | 6.70× / 2.95× |
| 4x2 | catalog | early-k | 15.766 | -5.20% (6/6) | late-k, +2.68% (1/6) | -5.10% (6/6) | -13.45% (6/6) | 198.9 | 6.91× / 3.04× |
| 4x2 | catalog | late-a | 16.332 | -3.23% (6/6) | off, -3.23% (6/6) | -2.85% (6/6) | -11.32% (6/6) | 220.8 | 6.67× / 2.94× |
| 4x2 | catalog | late-h | 17.134 | +1.57% (0/6) | off, +1.57% (0/6) | +1.54% (1/6) | -6.54% (6/6) | 224.4 | 6.36× / 2.80× |
| 4x2 | catalog | late-k | 15.380 | -7.88% (6/6) | off, -7.88% (6/6) | -7.52% (6/6) | -15.71% (6/6) | 206.5 | 7.09× / 3.12× |
| 4x2 | catalog | off | 16.723 | +0.00% (0/6) | off, +0.00% (0/6) | +0.38% (2/6) | -8.22% (6/6) | 232.8 | 6.52× / 2.87× |
| 4x2 | catalog | release | 16.779 | -0.38% (4/6) | off, -0.38% (4/6) | +0.00% (0/6) | -8.53% (6/6) | 231.1 | 6.50× / 2.86× |
| 4x2 | heldout | aeglos | 18.172 | +9.93% (0/6) | off, +9.93% (0/6) | +8.19% (0/6) | +0.00% (0/6) | 1347.3 | — |
| 4x2 | heldout | early-a | 17.905 | +7.67% (0/6) | late-a, +8.39% (0/6) | +6.55% (0/6) | -1.47% (5/6) | 194.5 | — |
| 4x2 | heldout | early-both | 16.454 | -0.96% (5/6) | off, -0.96% (5/6) | -1.87% (5/6) | -10.21% (6/6) | 208.6 | — |
| 4x2 | heldout | early-k | 15.990 | -4.11% (6/6) | late-k, +3.20% (0/6) | -5.15% (6/6) | -12.37% (6/6) | 208.0 | — |
| 4x2 | heldout | late-a | 16.535 | -0.98% (3/6) | off, -0.98% (3/6) | -1.67% (5/6) | -8.83% (6/6) | 232.6 | — |
| 4x2 | heldout | late-h | 16.752 | +1.29% (2/6) | off, +1.29% (2/6) | +0.03% (3/6) | -7.43% (6/6) | 234.2 | — |
| 4x2 | heldout | late-k | 15.465 | -7.22% (6/6) | off, -7.22% (6/6) | -8.66% (6/6) | -14.80% (6/6) | 209.4 | — |
| 4x2 | heldout | off | 16.677 | +0.00% (0/6) | off, +0.00% (0/6) | -0.60% (4/6) | -9.03% (6/6) | 219.1 | — |
| 4x2 | heldout | release | 16.884 | +0.60% (2/6) | off, +0.60% (2/6) | +0.00% (0/6) | -7.56% (6/6) | 235.7 | — |
| 8x2 | catalog | aeglos | 30.948 | +36.39% (0/6) | off, +36.39% (0/6) | +38.12% (0/6) | +0.00% (0/6) | 2492.2 | 6.11× / 2.39× |
| 8x2 | catalog | early-a | 22.987 | +1.34% (1/6) | late-a, +3.73% (0/6) | +1.76% (1/6) | -26.11% (6/6) | 394.2 | 8.22× / 3.22× |
| 8x2 | catalog | early-both | 21.408 | -5.38% (6/6) | off, -5.38% (6/6) | -5.53% (6/6) | -30.89% (6/6) | 388.3 | 8.83× / 3.46× |
| 8x2 | catalog | early-k | 22.074 | -2.85% (6/6) | late-k, +3.21% (0/6) | -2.69% (6/6) | -29.14% (6/6) | 404.6 | 8.56× / 3.35× |
| 8x2 | catalog | late-a | 22.290 | -1.49% (6/6) | off, -1.49% (6/6) | -1.13% (5/6) | -28.04% (6/6) | 428.0 | 8.48× / 3.32× |
| 8x2 | catalog | late-h | 23.603 | +3.63% (0/6) | off, +3.63% (0/6) | +4.26% (0/6) | -23.57% (6/6) | 431.7 | 8.01× / 3.14× |
| 8x2 | catalog | late-k | 21.340 | -6.49% (6/6) | off, -6.49% (6/6) | -5.82% (6/6) | -31.47% (6/6) | 411.6 | 8.86× / 3.47× |
| 8x2 | catalog | off | 22.726 | +0.00% (0/6) | off, +0.00% (0/6) | +0.59% (2/6) | -26.68% (6/6) | 423.3 | 8.32× / 3.26× |
| 8x2 | catalog | release | 22.600 | -0.59% (4/6) | off, -0.59% (4/6) | +0.00% (0/6) | -27.60% (6/6) | 435.7 | 8.36× / 3.27× |
| 8x2 | heldout | aeglos | 31.218 | +36.38% (0/6) | off, +36.38% (0/6) | +36.23% (0/6) | +0.00% (0/6) | 2454.3 | — |
| 8x2 | heldout | early-a | 23.221 | +2.48% (1/6) | late-a, +3.66% (0/6) | +1.93% (0/6) | -24.99% (6/6) | 396.6 | — |
| 8x2 | heldout | early-both | 21.543 | -6.13% (5/6) | off, -6.13% (5/6) | -6.11% (5/6) | -30.86% (6/6) | 399.4 | — |
| 8x2 | heldout | early-k | 22.422 | -1.25% (6/6) | late-k, +4.41% (0/6) | -1.80% (6/6) | -28.40% (6/6) | 395.1 | — |
| 8x2 | heldout | late-a | 22.512 | -1.43% (5/6) | off, -1.43% (5/6) | -2.13% (5/6) | -28.01% (6/6) | 420.4 | — |
| 8x2 | heldout | late-h | 23.728 | +4.26% (0/6) | off, +4.26% (0/6) | +3.99% (0/6) | -23.77% (6/6) | 436.5 | — |
| 8x2 | heldout | late-k | 21.512 | -5.40% (6/6) | off, -5.40% (6/6) | -5.82% (6/6) | -30.86% (6/6) | 419.1 | — |
| 8x2 | heldout | off | 22.791 | +0.00% (0/6) | off, +0.00% (0/6) | +0.00% (3/6) | -26.67% (6/6) | 439.6 | — |
| 8x2 | heldout | release | 22.867 | -0.00% (3/6) | off, -0.00% (3/6) | +0.00% (0/6) | -26.60% (6/6) | 444.0 | — |

| Shape | Input | Route | FFT begin–end ms | A begin–end ms | K begin–end ms |
|---|---|---|---:|---:|---:|
| 4x2 | catalog | early-a | 5.485–7.722 | 5.494–9.464 | 7.724–16.114 |
| 4x2 | catalog | early-k | 4.993–7.323 | 7.326–11.288 | 5.006–11.333 |
| 4x2 | catalog | early-both | 5.836–7.958 | 5.849–9.799 | 5.859–11.842 |
| 4x2 | heldout | early-a | 5.501–7.708 | 5.514–9.721 | 7.712–13.888 |
| 4x2 | heldout | early-k | 5.004–7.296 | 7.298–11.934 | 5.013–11.294 |
| 4x2 | heldout | early-both | 6.072–8.208 | 6.084–12.187 | 6.096–12.156 |
| 8x2 | catalog | early-a | 6.444–9.450 | 6.455–12.534 | 9.455–21.794 |
| 8x2 | catalog | early-k | 6.559–9.554 | 9.559–19.684 | 6.575–14.804 |
| 8x2 | catalog | early-both | 6.749–10.455 | 6.772–16.426 | 6.785–14.349 |
| 8x2 | heldout | early-a | 6.669–9.783 | 6.683–12.865 | 9.789–18.623 |
| 8x2 | heldout | early-k | 7.088–10.089 | 10.093–17.005 | 7.102–15.076 |
| 8x2 | heldout | early-both | 6.892–9.809 | 6.906–12.935 | 6.918–16.686 |

[Proof samples and intervals](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source changes](artifacts/source.patch) · [Previous curve-placement changes](../2026-11-19-curve-placement-c7a753/artifacts/source.patch) · [Verification](artifacts/verification.json)
## Result against the disabled schedule

| Shape | Configuration | Original / second ms | Paired change vs off |
|---|---|---:|---:|
| 4x2 | early-a | 17.823/17.905 | +6.48% / +7.67% |
| 4x2 | early-k | 15.766/15.990 | -5.20% / -4.11% |
| 4x2 | early-both | 16.267/16.454 | -2.92% / -0.96% |
| 8x2 | early-a | 22.987/23.221 | +1.34% / +2.48% |
| 8x2 | early-k | 22.074/22.422 | -2.85% / -1.25% |
| 8x2 | early-both | 21.408/21.543 | -5.38% / -6.13% |

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured C7a profiles. [Previous scorecard](../2026-11-19-curve-placement-c7a753/README.md)

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

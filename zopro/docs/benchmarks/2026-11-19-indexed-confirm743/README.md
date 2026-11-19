# Confirming the indexed 12×2 profile and testing combined 2×2 placement

The fresh C7a.24xlarge confirmation ran the fixed indexed 12×2 profile with 64 main cores and 32 disjoint frontend cores. The original request took **28.365 ms** with **3/6** process means at or below 28.4 ms. Its supplied M5 comparison is **10.01× CPU** and **3.56× Metal**. The fixed profile measured 28.365 / 28.338 / 39.636 / 28.383 ms across the original, second, v139 and v58 requests. The released SDK measured 29.704 / 29.616 / 40.900 / 29.734 ms on those same inputs.

The 12×2 experiment also measured eliminating a derivative-buffer pass with the [fused Euler output](../2026-11-19-euler-output742/README.md). The 2×2 experiment combines the existing G2 bucket-band evaluator and the dedicated A route. Its strongest Zopro arm by mean relative median across both inputs was 72-band-group, at 12.188 / 12.269 ms, against 11.679 / 11.862 ms for prepared Aeglos.

Each process warmed three requests and timed thirteen, in six matched blocks for 12×2 and five for 2×2. Aeglos used 96 cores and reported cache hits throughout the timed requests. Twelve additional exports checked route activation. All 270 exports independently verified against the original verifying keys and inputs. The instance and temporary storage were removed.

| Shape | Input | Configuration | Full proof ms | Paired vs previous SDK or archived 2×2 | Paired vs named control | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 12x2 | catalog | 64-sdk | 29.704 | +0.00% (0/6) | 64-sdk, +0.00% (0/6) | -38.06% (6/6) | 527.5 | 9.56× / 3.40× |
| 12x2 | catalog | aeglos | 47.882 | +61.46% (0/6) | 64-sdk, +61.46% (0/6) | +0.00% (0/6) | 3673.9 | 5.93× / 2.11× |
| 12x2 | catalog | emit-off | 29.834 | +0.99% (2/6) | emit-off, +0.00% (0/6) | -37.68% (6/6) | 529.7 | 9.52× / 3.39× |
| 12x2 | catalog | emit-pair | 29.538 | -0.70% (4/6) | emit-off, -0.96% (5/6) | -38.32% (6/6) | 546.9 | 9.61× / 3.42× |
| 12x2 | catalog | emit-quad | 29.391 | -0.97% (5/6) | emit-off, -1.54% (5/6) | -38.84% (6/6) | 523.8 | 9.66× / 3.44× |
| 12x2 | catalog | index-fixed-split-64 | 28.365 | -4.18% (6/6) | index-off-split-64, -0.13% (3/6) | -40.65% (6/6) | 529.0 | 10.01× / 3.56× |
| 12x2 | catalog | index-off-split-64 | 28.126 | -5.22% (6/6) | 64-sdk, -5.22% (6/6) | -41.11% (6/6) | 520.9 | 10.10× / 3.59× |
| 12x2 | heldout | 64-sdk | 29.616 | +0.00% (0/6) | 64-sdk, +0.00% (0/6) | -38.60% (6/6) | 528.3 | — |
| 12x2 | heldout | aeglos | 48.333 | +62.88% (0/6) | 64-sdk, +62.88% (0/6) | +0.00% (0/6) | 3720.5 | — |
| 12x2 | heldout | emit-off | 29.773 | +0.63% (1/6) | emit-off, +0.00% (0/6) | -38.40% (6/6) | 559.0 | — |
| 12x2 | heldout | emit-pair | 29.378 | -0.12% (3/6) | emit-off, -2.02% (4/6) | -38.42% (6/6) | 542.5 | — |
| 12x2 | heldout | emit-quad | 29.367 | -0.67% (4/6) | emit-off, -1.54% (6/6) | -39.26% (6/6) | 540.2 | — |
| 12x2 | heldout | index-fixed-split-64 | 28.338 | -4.80% (6/6) | index-off-split-64, +1.38% (2/6) | -40.96% (6/6) | 529.7 | — |
| 12x2 | heldout | index-off-split-64 | 28.067 | -5.33% (6/6) | 64-sdk, -5.33% (6/6) | -42.09% (6/6) | 523.0 | — |
| 12x2 | v139 | 64-sdk | 40.900 | +0.00% (0/6) | 64-sdk, +0.00% (0/6) | -15.66% (6/6) | 815.5 | — |
| 12x2 | v139 | aeglos | 48.464 | +18.58% (0/6) | 64-sdk, +18.58% (0/6) | +0.00% (0/6) | 3704.8 | — |
| 12x2 | v139 | emit-off | 40.915 | -0.27% (3/6) | emit-off, +0.00% (0/6) | -15.94% (6/6) | 849.3 | — |
| 12x2 | v139 | emit-pair | 39.946 | -1.74% (4/6) | emit-off, -2.09% (5/6) | -17.77% (6/6) | 854.0 | — |
| 12x2 | v139 | emit-quad | 40.811 | -0.49% (4/6) | emit-off, +0.58% (3/6) | -15.18% (6/6) | 847.6 | — |
| 12x2 | v139 | index-fixed-split-64 | 39.636 | -2.55% (6/6) | index-off-split-64, -0.86% (6/6) | -17.98% (6/6) | 760.0 | — |
| 12x2 | v139 | index-off-split-64 | 40.038 | -1.23% (4/6) | 64-sdk, -1.23% (4/6) | -17.29% (6/6) | 741.9 | — |
| 12x2 | v58 | 64-sdk | 29.734 | +0.00% (0/6) | 64-sdk, +0.00% (0/6) | -37.51% (6/6) | 539.4 | — |
| 12x2 | v58 | aeglos | 47.793 | +60.04% (0/6) | 64-sdk, +60.04% (0/6) | +0.00% (0/6) | 3704.4 | — |
| 12x2 | v58 | emit-off | 30.052 | +1.28% (2/6) | emit-off, +0.00% (0/6) | -36.96% (6/6) | 538.0 | — |
| 12x2 | v58 | emit-pair | 29.543 | -1.11% (4/6) | emit-off, -1.90% (6/6) | -38.42% (6/6) | 557.8 | — |
| 12x2 | v58 | emit-quad | 29.608 | -0.46% (4/6) | emit-off, -1.20% (5/6) | -37.93% (6/6) | 542.7 | — |
| 12x2 | v58 | index-fixed-split-64 | 28.383 | -5.00% (6/6) | index-off-split-64, +0.54% (2/6) | -40.57% (6/6) | 520.7 | — |
| 12x2 | v58 | index-off-split-64 | 28.359 | -5.00% (6/6) | 64-sdk, -5.00% (6/6) | -40.61% (6/6) | 523.0 | — |
| 2x2 | catalog | 64-band-group | 12.350 | -0.47% (3/5) | band96, -2.29% (5/5) | +5.21% (0/5) | 111.9 | 4.78× / 2.43× |
| 2x2 | catalog | 72-band-group | 12.188 | -2.03% (4/5) | 72-band-off, -2.68% (5/5) | +5.30% (0/5) | 112.6 | 4.84× / 2.46× |
| 2x2 | catalog | 72-band-group-four | 12.569 | +1.05% (0/5) | band96, -0.60% (4/5) | +7.62% (0/5) | 112.7 | 4.69× / 2.39× |
| 2x2 | catalog | 72-band-off | 12.659 | +0.20% (2/5) | band96, +0.11% (2/5) | +8.23% (0/5) | 115.0 | 4.66× / 2.37× |
| 2x2 | catalog | 80-band-group | 12.316 | -2.41% (4/5) | 80-band-off, -3.72% (5/5) | +4.41% (0/5) | 117.2 | 4.79× / 2.44× |
| 2x2 | catalog | 80-band-off | 12.774 | +1.94% (1/5) | band96, +0.11% (2/5) | +10.16% (0/5) | 118.7 | 4.62× / 2.35× |
| 2x2 | catalog | aeglos | 11.679 | -6.53% (5/5) | archived, -6.53% (5/5) | +0.00% (0/5) | 805.1 | 5.05× / 2.57× |
| 2x2 | catalog | archived | 12.514 | +0.00% (0/5) | archived, +0.00% (0/5) | +6.99% (0/5) | 132.2 | 4.71× / 2.40× |
| 2x2 | catalog | band96 | 12.645 | +1.98% (1/5) | band96, +0.00% (0/5) | +7.79% (0/5) | 133.0 | 4.67× / 2.37× |
| 2x2 | heldout | 64-band-group | 12.401 | -1.14% (3/5) | band96, -0.52% (4/5) | +5.12% (0/5) | 116.4 | — |
| 2x2 | heldout | 72-band-group | 12.269 | -1.88% (5/5) | 72-band-off, -4.82% (5/5) | +3.47% (0/5) | 120.3 | — |
| 2x2 | heldout | 72-band-group-four | 12.640 | +2.01% (1/5) | band96, +1.37% (1/5) | +7.07% (0/5) | 107.4 | — |
| 2x2 | heldout | 72-band-off | 12.807 | +3.35% (0/5) | band96, +1.99% (0/5) | +7.78% (0/5) | 119.3 | — |
| 2x2 | heldout | 80-band-group | 12.309 | -2.16% (4/5) | 80-band-off, -3.73% (5/5) | +5.55% (0/5) | 122.3 | — |
| 2x2 | heldout | 80-band-off | 12.636 | +1.04% (1/5) | band96, +0.98% (1/5) | +9.81% (0/5) | 115.7 | — |
| 2x2 | heldout | aeglos | 11.862 | -4.99% (5/5) | archived, -4.99% (5/5) | +0.00% (0/5) | 804.4 | — |
| 2x2 | heldout | archived | 12.475 | +0.00% (0/5) | archived, +0.00% (0/5) | +5.25% (0/5) | 132.2 | — |
| 2x2 | heldout | band96 | 12.535 | +0.63% (1/5) | band96, +0.00% (0/5) | +5.67% (0/5) | 135.1 | — |

| Shape | Input | Configuration | Route entries |
|---|---|---|---:|
| 2x2 | catalog | 80-band-group | 4/4 |
| 2x2 | catalog | 72-band-group | 4/4 |
| 2x2 | catalog | 64-band-group | 4/4 |
| 2x2 | catalog | 72-band-group-four | 4/4 |
| 12x2 | catalog | emit-quad | 4/4 |
| 12x2 | catalog | emit-pair | 4/4 |
| 2x2 | heldout | 80-band-group | 4/4 |
| 2x2 | heldout | 72-band-group | 4/4 |
| 2x2 | heldout | 64-band-group | 4/4 |
| 2x2 | heldout | 72-band-group-four | 4/4 |
| 12x2 | heldout | emit-quad | 4/4 |
| 12x2 | heldout | emit-pair | 4/4 |

## Eight supplied M5 reference timings

Across the measured C7a profiles, **4/8** supplied original requests reach 10× CPU speed. The indexed 12×2 result uses the experimental mapped image and prepared point groups. The slower 12×2 transactions appear in the full table above. The first comparison uses the reference's original request. The other seven rows come from the [previous reference panel](../2026-11-19-m5-width-confirm740/README.md).

| Shape | Profile | C7a ms | M5 CPU / Metal ms | Speedup CPU / Metal |
|---|---|---:|---:|---:|
| 1×2 | 737 catalog | 7.953 | 37 / 24 | 4.65× / 3.02× |
| 2×2 | 707 catalog | 12.513 | 59 / 30 | 4.72× / 2.40× |
| 4×2 | 714 catalog | 17.210 | 109 / 48 | 6.33× / 2.79× |
| 8×2 | 707 catalog | 25.794 | 189 / 74 | 7.33× / 2.87× |
| 12×2 | 743 indexed 64+32 | 28.365 | 284 / 101 | 10.01× / 3.56× |
| 16×2 | 740 64+32 | 31.200 | 335 / 124 | 10.74× / 3.97× |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 740 64+32 | 46.118 | 507 / 175 | 10.99× / 3.79× |

[Exact workflow](artifacts/workflow.json.gz) · [Measurements](artifacts/rows.json.gz) · [Proof verification](artifacts/verification.json) · [2×2 placement source](artifacts/band-placement.patch)

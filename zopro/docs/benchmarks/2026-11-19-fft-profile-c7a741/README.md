# Comparing 12×2 FFT profiles and 2×2 G1 placement on C7a

The 12×2 panel compares the released SDK at three main-pool sizes with the archived indexed profile that previously reached 28.4 ms. The indexed arms share the mapped key image and prepared point groups. Fixed enables the archived specialized FFT stages. Main pins the original 64-worker setup without a frontend. Split places the frontend on the remaining physical cores. For 2×2 the split arms move canonical A to a disjoint pool or keep it on the main pool with the same CPU placement. Raw skips the ordinary A grouping decision.

The fastest 12×2 original was index-fixed-split-64, at 27.995 / 28.422 ms on original and second inputs, against 29.748 / 29.574 ms for the released 64+32 layout. The strongest candidate by mean relative median across the two inputs was index-fixed-split-64, at 27.995 / 28.422 ms. The 2×2 winner by the same criterion was archived, at 12.453 / 12.510 ms, with Aeglos at 11.572 / 11.681 ms.

Four paired blocks cover 12×2, and five cover 2×2. Each timing process warmed three requests and timed thirteen. Aeglos used 96 cores and reported a cache hit on every timed request. Measurements use the median process mean. Independent verification passed all 204 exports, with both 12×2 images checked against the original verifying key. The instance and temporary resources were removed.

| Shape | Input | Profile | Median ms | Paired vs SDK control | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 2x2 | catalog | 64-group | 12.842 | -6.31% (5/5) | +10.09% (0/5) | 90.9 | 4.59× / 2.34× |
| 2x2 | catalog | 64-off | 13.199 | -3.25% (5/5) | +12.87% (0/5) | 91.6 | 4.47× / 2.27× |
| 2x2 | catalog | 72-group | 12.770 | -5.73% (5/5) | +11.50% (0/5) | 92.4 | 4.62× / 2.35× |
| 2x2 | catalog | 72-off | 13.513 | -1.59% (5/5) | +17.34% (0/5) | 87.9 | 4.37× / 2.22× |
| 2x2 | catalog | 72-raw | 12.927 | -5.18% (5/5) | +10.27% (0/5) | 86.4 | 4.56× / 2.32× |
| 2x2 | catalog | 80-group | 12.920 | -6.28% (5/5) | +11.53% (0/5) | 90.1 | 4.57× / 2.32× |
| 2x2 | catalog | 80-off | 13.154 | -2.06% (4/5) | +16.49% (0/5) | 86.4 | 4.49× / 2.28× |
| 2x2 | catalog | 80-raw | 13.211 | -3.64% (5/5) | +13.22% (0/5) | 84.2 | 4.47× / 2.27× |
| 2x2 | catalog | aeglos | 11.572 | -15.53% (5/5) | +0.00% (0/5) | 801.9 | 5.10× / 2.59× |
| 2x2 | catalog | archived | 12.453 | -8.12% (5/5) | +6.48% (0/5) | 130.7 | 4.74× / 2.41× |
| 2x2 | catalog | off | 13.634 | +0.00% (0/5) | +18.39% (0/5) | 88.7 | 4.33× / 2.20× |
| 2x2 | heldout | 64-group | 12.818 | -4.74% (5/5) | +11.41% (0/5) | 91.7 | — |
| 2x2 | heldout | 64-off | 13.343 | -1.71% (4/5) | +13.34% (0/5) | 92.4 | — |
| 2x2 | heldout | 72-group | 12.893 | -4.32% (5/5) | +10.11% (0/5) | 86.4 | — |
| 2x2 | heldout | 72-off | 13.385 | -1.02% (5/5) | +13.04% (0/5) | 88.7 | — |
| 2x2 | heldout | 72-raw | 12.892 | -3.14% (5/5) | +10.95% (0/5) | 91.7 | — |
| 2x2 | heldout | 80-group | 13.165 | -3.22% (4/5) | +13.19% (0/5) | 91.8 | — |
| 2x2 | heldout | 80-off | 13.486 | +0.51% (2/5) | +15.41% (0/5) | 84.9 | — |
| 2x2 | heldout | 80-raw | 13.121 | -3.25% (5/5) | +13.15% (0/5) | 85.7 | — |
| 2x2 | heldout | aeglos | 11.681 | -12.69% (5/5) | +0.00% (0/5) | 810.3 | — |
| 2x2 | heldout | archived | 12.510 | -7.13% (5/5) | +6.38% (0/5) | 129.3 | — |
| 2x2 | heldout | off | 13.566 | +0.00% (0/5) | +14.54% (0/5) | 92.4 | — |
| 12x2 | catalog | 56-sdk | 29.888 | -0.08% (3/4) | -37.83% (4/4) | 511.0 | 9.50× / 3.38× |
| 12x2 | catalog | 64-flat | 30.006 | +0.47% (1/4) | -37.52% (4/4) | 555.9 | 9.46× / 3.37× |
| 12x2 | catalog | 64-sdk | 29.748 | +0.00% (0/4) | -37.78% (4/4) | 536.5 | 9.55× / 3.40× |
| 12x2 | catalog | 72-sdk | 30.177 | +1.02% (0/4) | -37.01% (4/4) | 538.7 | 9.41× / 3.35× |
| 12x2 | catalog | aeglos | 47.788 | +60.72% (0/4) | +0.00% (0/4) | 3657.0 | 5.94× / 2.11× |
| 12x2 | catalog | index-fixed-main-64 | 28.664 | -3.88% (4/4) | -40.06% (4/4) | 513.1 | 9.91× / 3.52× |
| 12x2 | catalog | index-fixed-split-56 | 28.562 | -3.75% (4/4) | -39.96% (4/4) | 511.0 | 9.94× / 3.54× |
| 12x2 | catalog | index-fixed-split-64 | 27.995 | -6.22% (4/4) | -41.68% (4/4) | 508.6 | 10.14× / 3.61× |
| 12x2 | catalog | index-fixed-split-72 | 28.534 | -4.57% (4/4) | -40.49% (4/4) | 500.5 | 9.95× / 3.54× |
| 12x2 | catalog | index-off-main-64 | 28.446 | -4.38% (4/4) | -40.49% (4/4) | 525.3 | 9.98× / 3.55× |
| 12x2 | catalog | index-off-split-64 | 28.306 | -4.85% (4/4) | -40.70% (4/4) | 524.7 | 10.03× / 3.57× |
| 12x2 | heldout | 56-sdk | 30.062 | +1.00% (0/4) | -36.84% (4/4) | 536.9 | — |
| 12x2 | heldout | 64-flat | 29.723 | +0.63% (1/4) | -37.82% (4/4) | 552.2 | — |
| 12x2 | heldout | 64-sdk | 29.574 | +0.00% (0/4) | -38.29% (4/4) | 548.6 | — |
| 12x2 | heldout | 72-sdk | 30.369 | +2.54% (0/4) | -36.45% (4/4) | 562.6 | — |
| 12x2 | heldout | aeglos | 47.740 | +62.05% (0/4) | +0.00% (0/4) | 3695.3 | — |
| 12x2 | heldout | index-fixed-main-64 | 28.776 | -3.19% (3/4) | -38.99% (4/4) | 524.4 | — |
| 12x2 | heldout | index-fixed-split-56 | 28.878 | -3.04% (3/4) | -39.38% (4/4) | 499.4 | — |
| 12x2 | heldout | index-fixed-split-64 | 28.422 | -4.21% (4/4) | -40.46% (4/4) | 526.7 | — |
| 12x2 | heldout | index-fixed-split-72 | 28.577 | -3.31% (4/4) | -39.81% (4/4) | 508.0 | — |
| 12x2 | heldout | index-off-main-64 | 28.375 | -3.95% (4/4) | -40.39% (4/4) | 508.1 | — |
| 12x2 | heldout | index-off-split-64 | 28.117 | -4.83% (4/4) | -40.94% (4/4) | 524.6 | — |

Separate 2×2 checks timed one request after three warm-ups and counted invocations of the dedicated A route.

| Input | Profile | Route entries |
|---|---|---:|
| catalog | 80-group | 4/4 |
| catalog | 72-group | 4/4 |
| catalog | 80-raw | 4/4 |
| heldout | 80-group | 4/4 |
| heldout | 72-group | 4/4 |
| heldout | 80-raw | 4/4 |

[Measurements](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Reference comparison](../2026-11-19-m5-width-confirm740/README.md) · [Dedicated A implementation](../2026-11-19-wide-a-pool738/README.md)

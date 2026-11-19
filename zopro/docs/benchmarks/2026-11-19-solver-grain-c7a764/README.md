# Pacing packed solver work at 64+32 C7a cores

The experiment varies the cap for packing independent solver components and the number of already packed tasks reserved by one atomic claim. It changes the scheduling of existing work without changing dependencies or field arithmetic. The 8×2 released and trial controls use the earlier 128-operation packing cap and one task per claim. The 4×2 released control keeps its previous frontend solver without packed claiming. The 4×2 trial controls use packed claiming with the default cap and claim size. All Zopro variants retain the established frontend K and fused FFT settings.

Six matched blocks on each primary transaction and three on additional transactions use three priming and thirteen timed requests per process. Separate route checks verify that the bounded solver schedule is constructed and that a zero optional-preparation allowance can execute the fallback. Focused native solver tests pass with claim batches of two, four and eight. All 258 C7a and six Ryzen exports passed independent verification. The instance and temporary storage were removed.

| Shape | Input | Profile | Proof ms | Paired vs default packing and claims | Paired vs named control | Paired vs released SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---|---|---|---:|---:|
| 4x2 | catalog | aeglos | 18.765 | +15.49% (0/6) | off, +15.49% (0/6) | +19.29% (0/6) | +0.00% (0/6) | 1350.7 | 5.81× / 2.56× |
| 4x2 | catalog | claim4 | 19.434 | +20.65% (0/6) | off, +20.65% (0/6) | +22.98% (0/6) | +3.27% (0/6) | 206.4 | 5.61× / 2.47× |
| 4x2 | catalog | off | 16.146 | +0.00% (0/6) | off, +0.00% (0/6) | +2.02% (0/6) | -13.41% (6/6) | 204.9 | 6.75× / 2.97× |
| 4x2 | catalog | pack256 | 15.759 | -2.20% (6/6) | off, -2.20% (6/6) | -0.39% (4/6) | -15.69% (6/6) | 210.9 | 6.92× / 3.05× |
| 4x2 | catalog | pack256-claim4 | 19.467 | +20.55% (0/6) | pack256, +23.41% (0/6) | +23.13% (0/6) | +3.42% (0/6) | 204.3 | 5.60× / 2.47× |
| 4x2 | catalog | release | 15.775 | -1.98% (6/6) | off, -1.98% (6/6) | +0.00% (0/6) | -16.17% (6/6) | 203.5 | 6.91× / 3.04× |
| 4x2 | heldout | aeglos | 18.575 | +15.22% (0/6) | off, +15.22% (0/6) | +19.40% (0/6) | +0.00% (0/6) | 1346.1 | — |
| 4x2 | heldout | claim4 | 19.514 | +20.63% (0/6) | off, +20.63% (0/6) | +25.11% (0/6) | +5.47% (0/6) | 199.7 | — |
| 4x2 | heldout | off | 16.157 | +0.00% (0/6) | off, +0.00% (0/6) | +2.80% (0/6) | -13.21% (6/6) | 194.5 | — |
| 4x2 | heldout | pack256 | 15.964 | -1.21% (6/6) | off, -1.21% (6/6) | +1.77% (0/6) | -14.66% (6/6) | 199.0 | — |
| 4x2 | heldout | pack256-claim4 | 19.621 | +21.34% (0/6) | pack256, +22.86% (0/6) | +25.19% (0/6) | +4.95% (0/6) | 202.7 | — |
| 4x2 | heldout | release | 15.676 | -2.72% (6/6) | off, -2.72% (6/6) | +0.00% (0/6) | -16.25% (6/6) | 202.8 | — |
| 4x2 | v153 | off | 16.138 | +0.00% (0/3) | off, +0.00% (0/3) | +2.55% (0/3) | — | 199.7 | — |
| 4x2 | v153 | pack256 | 15.833 | -1.75% (3/3) | off, -1.75% (3/3) | +0.69% (0/3) | — | 206.4 | — |
| 4x2 | v153 | release | 15.724 | -2.49% (3/3) | off, -2.49% (3/3) | +0.00% (0/3) | — | 208.7 | — |
| 4x2 | v156 | off | 16.126 | +0.00% (0/3) | off, +0.00% (0/3) | +2.69% (0/3) | — | 204.9 | — |
| 4x2 | v156 | pack256 | 15.838 | -1.84% (3/3) | off, -1.84% (3/3) | +0.21% (0/3) | — | 196.0 | — |
| 4x2 | v156 | release | 15.795 | -2.62% (3/3) | off, -2.62% (3/3) | +0.00% (0/3) | — | 214.7 | — |
| 8x2 | catalog | aeglos | 31.877 | +51.57% (0/6) | off, +51.57% (0/6) | +51.02% (0/6) | +0.00% (0/6) | 2459.4 | 5.93× / 2.32× |
| 8x2 | catalog | claim2 | 22.439 | +5.97% (0/6) | off, +5.97% (0/6) | +5.86% (0/6) | -29.67% (6/6) | 410.0 | 8.42× / 3.30× |
| 8x2 | catalog | claim4 | 24.970 | +18.25% (0/6) | off, +18.25% (0/6) | +18.27% (0/6) | -21.62% (6/6) | 389.6 | 7.57× / 2.96× |
| 8x2 | catalog | claim8 | 29.948 | +41.04% (0/6) | off, +41.04% (0/6) | +41.39% (0/6) | -5.91% (6/6) | 390.6 | 6.31× / 2.47× |
| 8x2 | catalog | off | 21.119 | +0.00% (0/6) | off, +0.00% (0/6) | +0.43% (2/6) | -34.02% (6/6) | 407.8 | 8.95× / 3.50× |
| 8x2 | catalog | pack256 | 21.254 | +0.80% (1/6) | off, +0.80% (1/6) | +1.54% (2/6) | -33.44% (6/6) | 417.5 | 8.89× / 3.48× |
| 8x2 | catalog | pack256-claim4 | 25.139 | +18.85% (0/6) | pack256, +17.80% (0/6) | +18.67% (0/6) | -21.10% (6/6) | 386.8 | 7.52× / 2.94× |
| 8x2 | catalog | pack512 | 21.106 | -0.00% (3/6) | off, -0.00% (3/6) | -0.15% (3/6) | -33.57% (6/6) | 411.6 | 8.95× / 3.51× |
| 8x2 | catalog | pack512-claim4 | 25.440 | +19.87% (0/6) | pack512, +20.45% (0/6) | +20.27% (0/6) | -20.17% (6/6) | 389.0 | 7.43× / 2.91× |
| 8x2 | catalog | pack64 | 21.059 | -0.10% (3/6) | off, -0.10% (3/6) | -0.25% (3/6) | -34.09% (6/6) | 405.6 | 8.97× / 3.51× |
| 8x2 | catalog | release | 21.098 | -0.43% (4/6) | off, -0.43% (4/6) | +0.00% (0/6) | -33.78% (6/6) | 409.2 | 8.96× / 3.51× |
| 8x2 | heldout | aeglos | 31.466 | +48.29% (0/6) | off, +48.29% (0/6) | +48.23% (0/6) | +0.00% (0/6) | 2495.6 | — |
| 8x2 | heldout | claim2 | 22.519 | +6.12% (0/6) | off, +6.12% (0/6) | +5.73% (0/6) | -28.27% (6/6) | 399.0 | — |
| 8x2 | heldout | claim4 | 25.163 | +18.97% (0/6) | off, +18.97% (0/6) | +18.50% (0/6) | -19.67% (6/6) | 395.0 | — |
| 8x2 | heldout | claim8 | 30.159 | +42.14% (0/6) | off, +42.14% (0/6) | +41.99% (0/6) | -4.15% (6/6) | 383.1 | — |
| 8x2 | heldout | off | 21.176 | +0.00% (0/6) | off, +0.00% (0/6) | -0.37% (5/6) | -32.56% (6/6) | 414.8 | — |
| 8x2 | heldout | pack256 | 21.593 | +2.06% (0/6) | off, +2.06% (0/6) | +1.78% (0/6) | -31.78% (6/6) | 408.6 | — |
| 8x2 | heldout | pack256-claim4 | 25.304 | +19.36% (0/6) | pack256, +17.19% (0/6) | +18.78% (0/6) | -19.50% (6/6) | 420.0 | — |
| 8x2 | heldout | pack512 | 21.473 | +1.43% (1/6) | off, +1.43% (1/6) | +0.94% (1/6) | -31.66% (6/6) | 404.8 | — |
| 8x2 | heldout | pack512-claim4 | 25.766 | +21.41% (0/6) | pack512, +20.01% (0/6) | +20.55% (0/6) | -17.67% (6/6) | 396.0 | — |
| 8x2 | heldout | pack64 | 21.429 | +1.48% (1/6) | off, +1.48% (1/6) | +0.68% (1/6) | -31.53% (6/6) | 405.6 | — |
| 8x2 | heldout | release | 21.250 | +0.38% (1/6) | off, +0.38% (1/6) | +0.00% (0/6) | -32.54% (6/6) | 411.6 | — |
| 8x2 | v153 | off | 21.126 | +0.00% (0/3) | off, +0.00% (0/3) | -0.97% (2/3) | — | 398.0 | — |
| 8x2 | v153 | pack256 | 21.535 | +2.30% (0/3) | off, +2.30% (0/3) | +1.31% (0/3) | — | 400.2 | — |
| 8x2 | v153 | pack256-claim4 | 25.381 | +19.48% (0/3) | pack256, +17.86% (0/3) | +18.89% (0/3) | — | 389.1 | — |
| 8x2 | v153 | release | 21.333 | +0.98% (1/3) | off, +0.98% (1/3) | +0.00% (0/3) | — | 399.6 | — |
| 8x2 | v156 | off | 21.094 | +0.00% (0/3) | off, +0.00% (0/3) | -0.68% (3/3) | — | 392.0 | — |
| 8x2 | v156 | pack256 | 21.562 | +2.22% (0/3) | off, +2.22% (0/3) | +1.19% (0/3) | — | 401.8 | — |
| 8x2 | v156 | pack256-claim4 | 25.124 | +19.56% (0/3) | pack256, +16.79% (0/3) | +18.75% (0/3) | — | 388.9 | — |
| 8x2 | v156 | release | 21.158 | +0.68% (0/3) | off, +0.68% (0/3) | +0.00% (0/3) | — | 414.7 | — |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Solver tests](artifacts/tests.json) · [Verification](artifacts/verification.json)
## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles. [Previous comparison](../2026-11-19-b2-classification-c7a762/README.md)

| Shape | Configuration | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 758 packed | 20.818 | 189 / 74 | 9.08× / 3.55× | +1.918 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

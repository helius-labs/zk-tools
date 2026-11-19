# Starting large B2 queries during the quotient FFT

The trial starts the ordinary large B2 evaluator as soon as its coefficients are available in the folded CPU proof. It reuses the original key, coefficient slice and B2 arithmetic. The normal placement schedules the quotient and B2 on the main pool. The split placement runs the FFT on the attached frontend, where K can later run. The overlap reservation uses the existing FFT transient-memory estimate. Budget denial leaves the original sequencing.

Six paired blocks per primary input compare released SDK, disabled and early variants with the same settings for FFT placement and grouped Phi. Two additional inputs per shape use three blocks for the ordinary and split settings. Each timed process primes three requests and measures thirteen. Separate route checks confirm overlap and verify the zero-budget fallback. The table uses uninstrumented timing. The independent verifier checked 308 C7a and ten Ryzen exports. The temporary instance, volume and bucket were removed.

| Shape | Input | Profile | Proof ms | Paired vs named control | Paired vs released SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|---:|---:|
| 4x2 | catalog | aeglos | 18.569 | late, +20.09% (0/6) | +20.32% (0/6) | +0.00% (0/6) | 1346.0 | 5.87× / 2.58× |
| 4x2 | catalog | early | 15.422 | late, -0.07% (4/6) | +0.21% (3/6) | -16.26% (6/6) | 186.2 | 7.07× / 3.11× |
| 4x2 | catalog | early-phi | 15.921 | late-phi, -3.74% (6/6) | +3.70% (0/6) | -13.47% (6/6) | 196.8 | 6.85× / 3.01× |
| 4x2 | catalog | early-split | 17.244 | late-split, +11.24% (0/6) | +10.93% (0/6) | -7.23% (6/6) | 204.9 | 6.32× / 2.78× |
| 4x2 | catalog | early-split-phi | 18.676 | late-split-phi, +10.34% (0/6) | +20.74% (0/6) | -0.86% (3/6) | 181.7 | 5.84× / 2.57× |
| 4x2 | catalog | late | 15.482 | late, +0.00% (0/6) | +0.59% (2/6) | -16.73% (6/6) | 198.3 | 7.04× / 3.10× |
| 4x2 | catalog | late-phi | 16.484 | late, +6.31% (0/6) | +7.21% (0/6) | -10.39% (6/6) | 221.4 | 6.61× / 2.91× |
| 4x2 | catalog | late-split | 15.615 | late, +0.76% (1/6) | +1.01% (1/6) | -16.13% (6/6) | 203.4 | 6.98× / 3.07× |
| 4x2 | catalog | late-split-phi | 16.775 | late, +8.47% (0/6) | +8.95% (0/6) | -9.09% (6/6) | 192.2 | 6.50× / 2.86× |
| 4x2 | catalog | release | 15.390 | late, -0.58% (4/6) | +0.00% (0/6) | -16.89% (6/6) | 206.4 | 7.08× / 3.12× |
| 4x2 | heldout | aeglos | 18.332 | late, +17.30% (0/6) | +17.47% (0/6) | +0.00% (0/6) | 1342.8 | — |
| 4x2 | heldout | early | 15.381 | late, -1.14% (6/6) | -1.16% (5/6) | -15.85% (6/6) | 186.3 | — |
| 4x2 | heldout | early-phi | 15.734 | late-phi, -4.55% (6/6) | +0.81% (1/6) | -13.97% (6/6) | 178.6 | — |
| 4x2 | heldout | early-split | 16.768 | late-split, +6.09% (0/6) | +7.66% (0/6) | -9.29% (6/6) | 203.4 | — |
| 4x2 | heldout | early-split-phi | 18.225 | late-split-phi, +6.84% (1/6) | +16.09% (0/6) | -0.60% (3/6) | 189.2 | — |
| 4x2 | heldout | late | 15.607 | late, +0.00% (0/6) | +0.14% (3/6) | -14.75% (6/6) | 205.0 | — |
| 4x2 | heldout | late-phi | 16.439 | late, +5.38% (0/6) | +5.75% (0/6) | -10.20% (6/6) | 204.2 | — |
| 4x2 | heldout | late-split | 15.778 | late, +1.13% (2/6) | +1.27% (2/6) | -14.92% (6/6) | 190.6 | — |
| 4x2 | heldout | late-split-phi | 16.991 | late, +9.42% (0/6) | +9.35% (0/6) | -7.37% (6/6) | 190.0 | — |
| 4x2 | heldout | release | 15.605 | late, -0.14% (3/6) | +0.00% (0/6) | -14.87% (6/6) | 210.8 | — |
| 4x2 | v153 | early | 15.279 | late, -2.17% (3/3) | — | — | 176.3 | — |
| 4x2 | v153 | early-split | 16.800 | late-split, +7.40% (0/3) | — | — | 190.7 | — |
| 4x2 | v153 | late | 15.618 | late, +0.00% (0/3) | — | — | 193.0 | — |
| 4x2 | v153 | late-split | 15.763 | late, +1.16% (1/3) | — | — | 192.8 | — |
| 4x2 | v156 | early | 15.378 | late, -1.25% (2/3) | — | — | 187.6 | — |
| 4x2 | v156 | early-split | 17.428 | late-split, +8.31% (0/3) | — | — | 193.7 | — |
| 4x2 | v156 | late | 15.564 | late, +0.00% (0/3) | — | — | 207.2 | — |
| 4x2 | v156 | late-split | 15.924 | late, +2.64% (0/3) | — | — | 188.4 | — |
| 8x2 | catalog | aeglos | 31.317 | late, +46.98% (0/6) | +45.03% (0/6) | +0.00% (0/6) | 2457.3 | 6.04× / 2.36× |
| 8x2 | catalog | early | 21.628 | late, +1.64% (1/6) | +1.22% (2/6) | -31.11% (6/6) | 493.1 | 8.74× / 3.42× |
| 8x2 | catalog | early-phi | 22.151 | late-phi, -2.97% (5/6) | +2.84% (1/6) | -29.16% (6/6) | 478.2 | 8.53× / 3.34× |
| 8x2 | catalog | early-split | 23.136 | late-split, +4.34% (0/6) | +8.30% (1/6) | -25.67% (6/6) | 504.5 | 8.17× / 3.20× |
| 8x2 | catalog | early-split-phi | 23.674 | late-split-phi, -1.05% (5/6) | +8.61% (0/6) | -24.78% (6/6) | 496.9 | 7.98× / 3.13× |
| 8x2 | catalog | late | 21.269 | late, +0.00% (0/6) | -1.28% (4/6) | -31.96% (6/6) | 419.3 | 8.89× / 3.48× |
| 8x2 | catalog | late-phi | 22.705 | late, +6.93% (0/6) | +5.41% (1/6) | -27.31% (6/6) | 412.8 | 8.32× / 3.26× |
| 8x2 | catalog | late-split | 22.313 | late, +4.63% (0/6) | +3.38% (1/6) | -28.93% (6/6) | 377.1 | 8.47× / 3.32× |
| 8x2 | catalog | late-split-phi | 23.874 | late, +11.88% (0/6) | +11.44% (0/6) | -23.78% (6/6) | 366.0 | 7.92× / 3.10× |
| 8x2 | catalog | release | 21.539 | late, +1.30% (2/6) | +0.00% (0/6) | -31.04% (6/6) | 393.8 | 8.77× / 3.44× |
| 8x2 | heldout | aeglos | 31.229 | late, +43.90% (0/6) | +43.19% (0/6) | +0.00% (0/6) | 2461.9 | — |
| 8x2 | heldout | early | 22.067 | late, +1.82% (0/6) | +0.86% (2/6) | -29.73% (6/6) | 502.9 | — |
| 8x2 | heldout | early-phi | 22.290 | late-phi, -3.99% (5/6) | +3.20% (1/6) | -27.87% (6/6) | 513.6 | — |
| 8x2 | heldout | early-split | 23.221 | late-split, +2.85% (0/6) | +7.12% (0/6) | -25.24% (6/6) | 525.3 | — |
| 8x2 | heldout | early-split-phi | 24.522 | late-split-phi, +1.60% (1/6) | +11.56% (0/6) | -21.35% (6/6) | 532.0 | — |
| 8x2 | heldout | late | 21.563 | late, +0.00% (0/6) | -1.27% (4/6) | -30.51% (6/6) | 400.3 | — |
| 8x2 | heldout | late-phi | 23.110 | late, +7.44% (0/6) | +6.28% (0/6) | -25.77% (6/6) | 409.2 | — |
| 8x2 | heldout | late-split | 22.578 | late, +4.80% (0/6) | +3.24% (0/6) | -27.39% (6/6) | 379.3 | — |
| 8x2 | heldout | late-split-phi | 24.105 | late, +11.71% (0/6) | +10.57% (0/6) | -22.86% (6/6) | 368.0 | — |
| 8x2 | heldout | release | 21.800 | late, +1.28% (2/6) | +0.00% (0/6) | -30.16% (6/6) | 413.7 | — |
| 8x2 | v153 | early | 21.998 | late, +1.56% (0/3) | — | — | 486.3 | — |
| 8x2 | v153 | early-split | 23.993 | late-split, +5.68% (1/3) | — | — | 501.2 | — |
| 8x2 | v153 | late | 21.641 | late, +0.00% (0/3) | — | — | 403.2 | — |
| 8x2 | v153 | late-split | 22.691 | late, +4.70% (0/3) | — | — | 369.4 | — |
| 8x2 | v156 | early | 21.833 | late, +0.48% (0/3) | — | — | 470.5 | — |
| 8x2 | v156 | early-split | 23.511 | late-split, +2.45% (0/3) | — | — | 471.5 | — |
| 8x2 | v156 | late | 21.578 | late, +0.00% (0/3) | — | — | 405.6 | — |
| 8x2 | v156 | late-split | 22.693 | late, +5.00% (0/3) | — | — | 373.1 | — |

| Shape | Input | Profile | FFT begin–end ms | B2 begin–end ms | Overlap ms |
|---|---|---|---:|---:|---:|
| 4x2 | catalog | early | 0.002–2.186 | 0.010–8.855 | 2.176 |
| 4x2 | catalog | early-split | 0.000–2.135 | 0.006–9.318 | 2.129 |
| 4x2 | catalog | early-split-phi | 0.003–2.213 | 0.016–10.149 | 2.197 |
| 4x2 | heldout | early | 0.000–2.196 | 0.009–7.737 | 2.188 |
| 4x2 | heldout | early-split | 0.003–2.216 | 0.013–8.765 | 2.202 |
| 4x2 | heldout | early-split-phi | 0.000–2.105 | 0.007–10.411 | 2.098 |
| 8x2 | catalog | early | 0.010–3.101 | 0.015–12.834 | 3.086 |
| 8x2 | catalog | early-split | 0.006–3.915 | 0.011–13.990 | 3.904 |
| 8x2 | catalog | early-split-phi | 0.006–3.870 | 0.011–16.909 | 3.860 |
| 8x2 | heldout | early | 0.009–3.389 | 0.016–13.642 | 3.373 |
| 8x2 | heldout | early-split | 0.013–8.874 | 0.020–8.984 | 8.854 |
| 8x2 | heldout | early-split-phi | 0.007–4.167 | 0.012–14.634 | 4.155 |

The intervals are from the single timed request in each diagnostic process.

[Samples and interval diagnostics](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles. [Previous comparison](../2026-11-19-phase-switches-c7a758/README.md)

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

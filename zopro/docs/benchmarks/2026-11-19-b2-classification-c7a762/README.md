# Classifying B2 scalars during the quotient FFT

The trial moves the existing scalar classifier into the FFT interval. The B2 point reduction and MSM wait for the quotient and the class result. Classification runs on the caller's frontend pool or the main pool. A third arm runs the FFT on the frontend and classifies on the main pool. The classifier's output replaces the ordinary B2 classification. Both successful and declined classifications are reused. Key points and curve arithmetic are shared with the normal proof path. The existing budget rejects overlap when the transient FFT capacity cannot be reserved.

For 8×2 the control and primary candidate retain the packed solver configuration from the switch comparison. The two unpacked variants isolate the effect of that setting on the original and second transactions. There are six paired blocks per primary input and three per additional input. Each timed process primes three proofs and times thirteen. Separate route checks confirm the measured overlap and the fallback when the budget or shared frontend declines the request. All 276 C7a and 12 Ryzen exports passed independent verification. The temporary resources were removed.

| Shape | Input | Profile | Proof ms | Paired vs named control | Paired vs released SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|---:|---:|
| 4x2 | catalog | aeglos | 18.473 | late, +19.61% (0/6) | +20.02% (0/6) | +0.00% (0/6) | 1344.9 | 5.90× / 2.60× |
| 4x2 | catalog | group-front | 16.707 | late, +7.52% (0/6) | +8.62% (0/6) | -10.17% (6/6) | 213.3 | 6.52× / 2.87× |
| 4x2 | catalog | group-main | 16.399 | late, +5.45% (0/6) | +5.77% (0/6) | -12.21% (6/6) | 202.0 | 6.65× / 2.93× |
| 4x2 | catalog | late | 15.454 | late, +0.00% (0/6) | -0.28% (4/6) | -16.40% (6/6) | 201.3 | 7.05× / 3.11× |
| 4x2 | catalog | release | 15.416 | late, +0.28% (2/6) | +0.00% (0/6) | -16.68% (6/6) | 208.7 | 7.07× / 3.11× |
| 4x2 | catalog | split | 15.584 | late, +1.20% (1/6) | +1.32% (0/6) | -15.63% (6/6) | 201.3 | 6.99× / 3.08× |
| 4x2 | catalog | split-group | 16.219 | split, +4.01% (1/6) | +5.37% (0/6) | -12.75% (6/6) | 183.9 | 6.72× / 2.96× |
| 4x2 | heldout | aeglos | 18.364 | late, +18.77% (0/6) | +17.99% (0/6) | +0.00% (0/6) | 1352.4 | — |
| 4x2 | heldout | group-front | 16.732 | late, +7.68% (0/6) | +8.28% (0/6) | -8.83% (6/6) | 192.2 | — |
| 4x2 | heldout | group-main | 16.560 | late, +6.34% (0/6) | +6.69% (0/6) | -9.32% (6/6) | 205.0 | — |
| 4x2 | heldout | late | 15.571 | late, +0.00% (0/6) | +0.15% (2/6) | -15.80% (6/6) | 230.5 | — |
| 4x2 | heldout | release | 15.547 | late, -0.15% (4/6) | +0.00% (0/6) | -15.25% (6/6) | 208.7 | — |
| 4x2 | heldout | split | 15.703 | late, +1.08% (1/6) | +1.57% (0/6) | -14.22% (6/6) | 206.4 | — |
| 4x2 | heldout | split-group | 16.536 | split, +4.80% (0/6) | +6.58% (0/6) | -10.70% (6/6) | 187.6 | — |
| 4x2 | v153 | group-front | 16.964 | late, +9.71% (0/3) | — | — | 197.6 | — |
| 4x2 | v153 | group-main | 16.739 | late, +8.46% (0/3) | — | — | 194.6 | — |
| 4x2 | v153 | late | 15.433 | late, +0.00% (0/3) | — | — | 214.0 | — |
| 4x2 | v153 | split | 15.900 | late, +2.18% (0/3) | — | — | 200.4 | — |
| 4x2 | v153 | split-group | 16.382 | split, +3.03% (0/3) | — | — | 194.5 | — |
| 4x2 | v156 | group-front | 17.006 | late, +9.88% (0/3) | — | — | 197.5 | — |
| 4x2 | v156 | group-main | 16.354 | late, +5.68% (0/3) | — | — | 193.8 | — |
| 4x2 | v156 | late | 15.476 | late, +0.00% (0/3) | — | — | 202.7 | — |
| 4x2 | v156 | split | 15.817 | late, +1.42% (0/3) | — | — | 198.2 | — |
| 4x2 | v156 | split-group | 16.600 | split, +5.97% (0/3) | — | — | 186.2 | — |
| 8x2 | catalog | aeglos | 30.970 | late, +48.98% (0/6) | +49.31% (0/6) | +0.00% (0/6) | 2462.4 | 6.10× / 2.39× |
| 8x2 | catalog | group-front | 22.817 | late, +9.77% (0/6) | +9.47% (0/6) | -26.49% (6/6) | 471.3 | 8.28× / 3.24× |
| 8x2 | catalog | group-front-unpacked | 23.730 | late-unpacked, +10.99% (0/6) | +13.35% (0/6) | -23.72% (6/6) | 493.1 | 7.96× / 3.12× |
| 8x2 | catalog | group-main | 22.494 | late, +8.12% (0/6) | +8.57% (0/6) | -27.65% (6/6) | 520.8 | 8.40× / 3.29× |
| 8x2 | catalog | late | 20.798 | late, +0.00% (0/6) | -0.29% (5/6) | -32.88% (6/6) | 418.5 | 9.09× / 3.56× |
| 8x2 | catalog | late-unpacked | 21.337 | late, +2.65% (0/6) | +2.12% (0/6) | -31.21% (6/6) | 401.0 | 8.86× / 3.47× |
| 8x2 | catalog | release | 20.846 | late, +0.29% (1/6) | +0.00% (0/6) | -33.02% (6/6) | 413.0 | 9.07× / 3.55× |
| 8x2 | catalog | split | 21.622 | late, +4.03% (0/6) | +3.74% (0/6) | -30.14% (6/6) | 374.1 | 8.74× / 3.42× |
| 8x2 | catalog | split-group | 22.826 | split, +5.07% (0/6) | +9.08% (0/6) | -26.30% (6/6) | 459.6 | 8.28× / 3.24× |
| 8x2 | heldout | aeglos | 31.200 | late, +50.40% (0/6) | +49.41% (0/6) | +0.00% (0/6) | 2491.1 | — |
| 8x2 | heldout | group-front | 23.386 | late, +12.30% (0/6) | +10.53% (0/6) | -25.66% (6/6) | 505.4 | — |
| 8x2 | heldout | group-front-unpacked | 23.951 | late-unpacked, +11.30% (0/6) | +14.06% (0/6) | -23.46% (6/6) | 521.0 | — |
| 8x2 | heldout | group-main | 22.769 | late, +9.17% (0/6) | +8.33% (0/6) | -27.57% (6/6) | 478.2 | — |
| 8x2 | heldout | late | 20.807 | late, +0.00% (0/6) | -1.12% (6/6) | -33.51% (6/6) | 409.3 | — |
| 8x2 | heldout | late-unpacked | 21.536 | late, +3.65% (0/6) | +2.48% (1/6) | -31.57% (6/6) | 404.1 | — |
| 8x2 | heldout | release | 20.958 | late, +1.14% (0/6) | +0.00% (0/6) | -33.07% (6/6) | 414.3 | — |
| 8x2 | heldout | split | 21.895 | late, +5.93% (0/6) | +4.35% (0/6) | -29.76% (6/6) | 388.2 | — |
| 8x2 | heldout | split-group | 23.087 | split, +4.96% (0/6) | +9.53% (0/6) | -26.32% (6/6) | 464.6 | — |
| 8x2 | v153 | group-front | 23.399 | late, +12.34% (0/3) | — | — | 405.9 | — |
| 8x2 | v153 | group-main | 23.143 | late, +12.12% (0/3) | — | — | 479.6 | — |
| 8x2 | v153 | late | 20.829 | late, +0.00% (0/3) | — | — | 389.9 | — |
| 8x2 | v153 | split | 24.248 | late, +16.30% (0/3) | — | — | 374.1 | — |
| 8x2 | v153 | split-group | 22.742 | split, -6.21% (2/3) | — | — | 460.0 | — |
| 8x2 | v156 | group-front | 23.221 | late, +11.94% (0/3) | — | — | 503.5 | — |
| 8x2 | v156 | group-main | 22.790 | late, +9.42% (0/3) | — | — | 428.5 | — |
| 8x2 | v156 | late | 20.744 | late, +0.00% (0/3) | — | — | 423.6 | — |
| 8x2 | v156 | split | 21.972 | late, +6.22% (0/3) | — | — | 364.3 | — |
| 8x2 | v156 | split-group | 22.820 | split, +3.83% (0/3) | — | — | 493.1 | — |

| Shape | Input | Profile | FFT begin–end ms | Classification begin–end ms | Overlap ms | Classes built |
|---|---|---|---:|---:|---:|---:|
| 4x2 | catalog | group-front | 0.003–2.261 | 0.027–3.715 | 2.234 | 1 |
| 4x2 | catalog | group-main | 0.003–2.254 | 0.014–3.290 | 2.240 | 1 |
| 4x2 | catalog | split-group | 0.000–2.132 | 0.009–3.677 | 2.123 | 1 |
| 4x2 | heldout | group-front | 0.003–2.131 | 0.041–3.262 | 2.090 | 1 |
| 4x2 | heldout | group-main | 0.000–2.112 | 0.011–3.340 | 2.101 | 1 |
| 4x2 | heldout | split-group | 0.002–2.099 | 0.010–3.785 | 2.089 | 1 |
| 8x2 | catalog | group-front | 0.014–2.923 | 0.044–4.679 | 2.879 | 1 |
| 8x2 | catalog | group-main | 0.006–3.454 | 0.011–5.053 | 3.443 | 1 |
| 8x2 | catalog | split-group | 0.006–3.949 | 0.012–5.767 | 3.937 | 1 |
| 8x2 | heldout | group-front | 0.009–2.889 | 0.042–4.644 | 2.847 | 1 |
| 8x2 | heldout | group-main | 0.013–3.081 | 0.017–5.389 | 3.065 | 1 |
| 8x2 | heldout | split-group | 0.010–3.908 | 0.015–5.043 | 3.893 | 1 |

The intervals are from the single timed request in each diagnostic process.

[Samples and interval diagnostics](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles. [Previous comparison](../2026-11-19-early-large-b2-c7a760/README.md)

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

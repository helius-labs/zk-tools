# Prepared point groups and fused FFT on 4×2 and 8×2

The experiment reuses the earlier compact point-group descriptions, the indexed executable tested on 12×2 and its optional final-column FFT fusion. The no-groups, group64-off and group64-on arms share an executable, image and placement. The no-groups arm enables fusion and omits the group files. Group64-off loads the groups and disables fusion. Group56-on changes the worker split. The SDK arm uses its ordinary image and the opt-in FFT fusion. The archived Zopro and prepared Aeglos controls use their recorded 96-core settings.

Six paired blocks covered each original and second transaction. Three blocks covered each of two other transactions per shape. Timed processes warmed three proofs and timed thirteen. Eight Ryzen and eight C7a route checks recorded group use in A, K and B2, and entry to the fused FFT. Independent verification passed all 212 C7a and eight Ryzen exports. The temporary instance, bucket and volumes were removed.

- 4x2 grouped and fused at 64+32 took **18.058 / 18.068 ms** on the original and second inputs, versus **16.939 / 17.226 ms** without groups. Paired changes were +6.69% / +5.45%. Prepared Aeglos took **18.106 / 18.267 ms**.
- 8x2 grouped and fused at 64+32 took **24.582 / 24.107 ms** on the original and second inputs, versus **22.478 / 22.810 ms** without groups. Paired changes were +8.99% / +6.43%. Prepared Aeglos took **30.416 / 30.380 ms**.

| Shape | Input | Configuration | Proof ms | Paired vs no groups | Paired vs grouped fusion off | Paired vs current SDK | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4x2 | catalog | aeglos | 18.106 | +7.02% (0/6) | -1.53% (5/6) | +9.19% (0/6) | +0.00% (0/6) | 1346.6 | 6.02× / 2.65× |
| 4x2 | catalog | archive | 18.450 | +8.71% (0/6) | -0.96% (4/6) | +11.85% (0/6) | +1.83% (1/6) | 231.2 | 5.91× / 2.60× |
| 4x2 | catalog | group56-on | 18.023 | +6.08% (0/6) | -2.11% (4/6) | +8.82% (0/6) | +0.12% (3/6) | 273.1 | 6.05× / 2.66× |
| 4x2 | catalog | group64-off | 18.511 | +8.27% (0/6) | +0.00% (0/6) | +10.39% (0/6) | +1.56% (1/6) | 280.7 | 5.89× / 2.59× |
| 4x2 | catalog | group64-on | 18.058 | +6.69% (0/6) | -2.18% (4/6) | +9.75% (0/6) | -0.42% (3/6) | 275.4 | 6.04× / 2.66× |
| 4x2 | catalog | no-groups | 16.939 | +0.00% (0/6) | -7.64% (6/6) | +2.33% (0/6) | -6.56% (6/6) | 233.5 | 6.43× / 2.83× |
| 4x2 | catalog | sdk64 | 16.549 | -2.27% (6/6) | -9.41% (6/6) | +0.00% (0/6) | -8.41% (6/6) | 223.7 | 6.59× / 2.90× |
| 4x2 | heldout | aeglos | 18.267 | +6.89% (0/6) | -1.03% (4/6) | +7.95% (0/6) | +0.00% (0/6) | 1360.2 | — |
| 4x2 | heldout | archive | 18.443 | +9.31% (0/6) | +0.39% (2/6) | +9.93% (0/6) | +1.66% (2/6) | 229.1 | — |
| 4x2 | heldout | group56-on | 17.857 | +5.92% (0/6) | -2.86% (5/6) | +6.64% (0/6) | -1.21% (5/6) | 276.4 | — |
| 4x2 | heldout | group64-off | 18.383 | +8.06% (0/6) | +0.00% (0/6) | +8.41% (0/6) | +1.04% (2/6) | 269.4 | — |
| 4x2 | heldout | group64-on | 18.068 | +5.45% (0/6) | -2.04% (5/6) | +6.03% (0/6) | -1.65% (5/6) | 282.2 | — |
| 4x2 | heldout | no-groups | 17.226 | +0.00% (0/6) | -7.46% (6/6) | +0.89% (2/6) | -6.45% (6/6) | 233.4 | — |
| 4x2 | heldout | sdk64 | 17.014 | -0.88% (4/6) | -7.75% (6/6) | +0.00% (0/6) | -7.36% (6/6) | 231.3 | — |
| 4x2 | v153 | group64-on | 17.927 | +3.88% (0/3) | — | +6.61% (0/3) | — | 264.9 | — |
| 4x2 | v153 | no-groups | 17.305 | +0.00% (0/3) | — | +2.63% (0/3) | — | 227.4 | — |
| 4x2 | v153 | sdk64 | 16.795 | -2.57% (3/3) | — | +0.00% (0/3) | — | 214.0 | — |
| 4x2 | v156 | group64-on | 18.077 | +5.87% (0/3) | — | +9.81% (0/3) | — | 281.4 | — |
| 4x2 | v156 | no-groups | 17.064 | +0.00% (0/3) | — | +3.72% (0/3) | — | 229.7 | — |
| 4x2 | v156 | sdk64 | 16.606 | -3.58% (3/3) | — | +0.00% (0/3) | — | 225.3 | — |
| 8x2 | catalog | aeglos | 30.416 | +36.56% (0/6) | +22.84% (0/6) | +34.63% (0/6) | +0.00% (0/6) | 2467.0 | 6.21× / 2.43× |
| 8x2 | catalog | archive | 25.056 | +12.09% (0/6) | +0.82% (2/6) | +11.25% (0/6) | -18.03% (6/6) | 422.3 | 7.54× / 2.95× |
| 8x2 | catalog | group56-on | 25.136 | +11.84% (0/6) | +1.39% (1/6) | +10.99% (0/6) | -17.57% (6/6) | 441.6 | 7.52× / 2.94× |
| 8x2 | catalog | group64-off | 24.639 | +10.44% (0/6) | +0.00% (0/6) | +9.03% (0/6) | -18.59% (6/6) | 445.3 | 7.67× / 3.00× |
| 8x2 | catalog | group64-on | 24.582 | +8.99% (0/6) | -0.17% (3/6) | +9.21% (0/6) | -19.51% (6/6) | 456.5 | 7.69× / 3.01× |
| 8x2 | catalog | no-groups | 22.478 | +0.00% (0/6) | -9.45% (6/6) | -0.40% (3/6) | -26.77% (6/6) | 423.7 | 8.41× / 3.29× |
| 8x2 | catalog | sdk64 | 22.631 | +0.41% (3/6) | -8.28% (6/6) | +0.00% (0/6) | -25.72% (6/6) | 428.4 | 8.35× / 3.27× |
| 8x2 | heldout | aeglos | 30.380 | +33.72% (0/6) | +25.45% (0/6) | +34.56% (0/6) | +0.00% (0/6) | 2468.8 | — |
| 8x2 | heldout | archive | 25.775 | +13.66% (0/6) | +5.88% (0/6) | +13.62% (0/6) | -15.35% (6/6) | 422.1 | — |
| 8x2 | heldout | group56-on | 24.501 | +7.09% (0/6) | +0.83% (3/6) | +7.83% (0/6) | -19.90% (6/6) | 459.6 | — |
| 8x2 | heldout | group64-off | 24.182 | +5.50% (0/6) | +0.00% (0/6) | +6.86% (0/6) | -20.29% (6/6) | 458.7 | — |
| 8x2 | heldout | group64-on | 24.107 | +6.43% (0/6) | -0.31% (3/6) | +6.91% (0/6) | -20.54% (6/6) | 463.4 | — |
| 8x2 | heldout | no-groups | 22.810 | +0.00% (0/6) | -5.20% (6/6) | +0.71% (2/6) | -25.22% (6/6) | 436.3 | — |
| 8x2 | heldout | sdk64 | 22.658 | -0.70% (4/6) | -6.42% (6/6) | +0.00% (0/6) | -25.68% (6/6) | 441.2 | — |
| 8x2 | v153 | group64-on | 23.784 | +3.95% (0/3) | — | +4.18% (0/3) | — | 456.7 | — |
| 8x2 | v153 | no-groups | 22.761 | +0.00% (0/3) | — | +0.07% (1/3) | — | 433.6 | — |
| 8x2 | v153 | sdk64 | 22.919 | -0.07% (2/3) | — | +0.00% (0/3) | — | 428.2 | — |
| 8x2 | v156 | group64-on | 24.162 | +5.41% (0/3) | — | +6.29% (0/3) | — | 470.1 | — |
| 8x2 | v156 | no-groups | 22.926 | +0.00% (0/3) | — | +0.83% (0/3) | — | 428.8 | — |
| 8x2 | v156 | sdk64 | 22.732 | -0.83% (3/3) | — | +0.00% (0/3) | — | 427.4 | — |

[Samples](artifacts/rows.json.gz) · [Exact workflow](artifacts/workflow.json.gz) · [Routes](artifacts/routes.json) · [Verification](artifacts/verification.json) · [Earlier eight-shape comparison](../2026-11-19-critical-path-c7a750/README.md)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured C7a profiles. The table retains earlier profiles where the two-input screen did not select a faster replacement. Some profiles use experimental mapped images and prepared point groups. [Previous scorecard](../2026-11-19-critical-path-c7a750/README.md)

| Shape | C7a profile | C7a ms | M5 CPU / Metal ms | Speedup CPU / Metal | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 751 sdk64 | 16.549 | 109 / 48 | 6.59× / 2.90× | +5.649 |
| 8×2 | 747 opt-in fusion 64+32 | 22.583 | 189 / 74 | 8.37× / 3.28× | +3.683 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

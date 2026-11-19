# Fused K/H windows and fresh Aeglos comparison on C7a

One c7a.24xlarge compares the [balanced window layouts](../2026-11-19-fused-kh-layout-ryzen804/README.md) with released Zopro, the same-build ordinary layout, and prepared Aeglos. Zopro uses 80 main and 16 frontend cores on 1×2 and 64 main with 32 frontend cores on 2×2. All Zopro 2×2 arms use the existing medium K/H fusion option. Prepared Aeglos uses all 96 cores. The experimental window counts derive balanced signed digits for each proof. Merge and separate modes consume K and H in separate buffers, joining their buckets or final sums respectively.

Five matched blocks on each original and held-out input time thirteen proofs after three priming calls. All Aeglos samples hit its preparation cache. All 192 exports verified against the pinned original keys. Normal route checks exercised memory admission and zero optional-memory fallback. The temporary instance and storage were removed.

- 1×2 `21-shared` took 7.973 / 7.886 ms across original and held-out, paired +1.38% / +1.23% from the same-build ordinary arm.
- 2×2 `18-shared` took 12.343 / 12.283 ms across original and held-out, paired +2.05% / +1.22% from the same-build ordinary arm.

None of the layouts improved both primary inputs consistently against both controls. Selected profiles remain experimental.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.858 | -0.22% (3/5) | +0.00% (0/5) | -2.16% (4/5) | 40.8 |
| 1×2 | catalog | ordinary | 7.861 | +0.00% (0/5) | +0.22% (2/5) | -1.94% (5/5) | 39.2 |
| 1×2 | catalog | 18-shared | 8.203 | +4.36% (0/5) | +3.76% (1/5) | +0.69% (2/5) | 66.3 |
| 1×2 | catalog | 19-shared | 7.998 | +1.99% (0/5) | +1.41% (1/5) | -0.85% (3/5) | 47.5 |
| 1×2 | catalog | 21-shared | 7.973 | +1.38% (0/5) | +1.47% (1/5) | -0.73% (4/5) | 40.8 |
| 1×2 | catalog | 22-shared | 8.021 | +1.81% (0/5) | +1.99% (1/5) | -1.29% (3/5) | 40.0 |
| 1×2 | catalog | 20-merge | 8.310 | +6.16% (0/5) | +6.14% (1/5) | +3.54% (0/5) | 45.2 |
| 1×2 | catalog | 20-separate | 8.656 | +10.04% (0/5) | +9.65% (0/5) | +7.19% (0/5) | 40.8 |
| 1×2 | catalog | aeglos | 8.032 | +1.98% (0/5) | +2.21% (1/5) | +0.00% (0/5) | 451.6 |
| 1×2 | heldout | release | 7.754 | -0.03% (3/5) | +0.00% (0/5) | -6.16% (5/5) | 39.3 |
| 1×2 | heldout | ordinary | 7.802 | +0.00% (0/5) | +0.03% (2/5) | -5.01% (5/5) | 39.4 |
| 1×2 | heldout | 18-shared | 8.028 | +3.49% (0/5) | +3.27% (0/5) | -2.33% (4/5) | 61.0 |
| 1×2 | heldout | 19-shared | 7.906 | +0.95% (0/5) | +1.69% (0/5) | -4.67% (4/5) | 46.8 |
| 1×2 | heldout | 21-shared | 7.886 | +1.23% (0/5) | +1.21% (0/5) | -3.14% (4/5) | 38.5 |
| 1×2 | heldout | 22-shared | 7.976 | +2.15% (0/5) | +3.45% (0/5) | -2.88% (4/5) | 37.8 |
| 1×2 | heldout | 20-merge | 8.263 | +5.99% (0/5) | +6.73% (0/5) | +0.93% (2/5) | 41.5 |
| 1×2 | heldout | 20-separate | 8.575 | +9.91% (0/5) | +10.33% (0/5) | +4.41% (0/5) | 44.6 |
| 1×2 | heldout | aeglos | 8.213 | +5.27% (0/5) | +6.56% (0/5) | +0.00% (0/5) | 457.0 |
| 2×2 | catalog | release | 12.029 | +0.85% (2/5) | +0.00% (0/5) | +2.39% (0/5) | 72.2 |
| 2×2 | catalog | ordinary | 12.025 | +0.00% (0/5) | -0.84% (3/5) | +2.72% (0/5) | 71.4 |
| 2×2 | catalog | 18-shared | 12.343 | +2.05% (0/5) | +2.64% (0/5) | +5.86% (0/5) | 89.7 |
| 2×2 | catalog | 19-shared | 12.208 | +2.24% (0/5) | +1.38% (0/5) | +5.08% (0/5) | 76.8 |
| 2×2 | catalog | 21-shared | 12.262 | +1.97% (0/5) | +1.63% (0/5) | +4.59% (0/5) | 69.4 |
| 2×2 | catalog | 22-shared | 12.225 | +1.53% (0/5) | +1.72% (1/5) | +4.29% (0/5) | 74.6 |
| 2×2 | catalog | 20-merge | 12.500 | +3.95% (0/5) | +3.34% (0/5) | +6.49% (0/5) | 78.2 |
| 2×2 | catalog | 20-separate | 12.330 | +2.91% (0/5) | +2.22% (0/5) | +6.37% (0/5) | 80.4 |
| 2×2 | catalog | aeglos | 11.737 | -2.64% (5/5) | -2.34% (5/5) | +0.00% (0/5) | 807.2 |
| 2×2 | heldout | release | 12.115 | -0.31% (4/5) | +0.00% (0/5) | +3.88% (0/5) | 74.7 |
| 2×2 | heldout | ordinary | 12.142 | +0.00% (0/5) | +0.31% (1/5) | +4.04% (0/5) | 73.8 |
| 2×2 | heldout | 18-shared | 12.283 | +1.22% (0/5) | +1.88% (1/5) | +5.83% (0/5) | 93.3 |
| 2×2 | heldout | 19-shared | 12.362 | +1.75% (0/5) | +1.88% (0/5) | +6.51% (0/5) | 81.8 |
| 2×2 | heldout | 21-shared | 12.340 | +1.76% (1/5) | +1.88% (0/5) | +6.27% (0/5) | 73.0 |
| 2×2 | heldout | 22-shared | 12.352 | +2.08% (1/5) | +1.99% (0/5) | +5.94% (0/5) | 72.2 |
| 2×2 | heldout | 20-merge | 12.474 | +2.11% (0/5) | +2.48% (0/5) | +7.47% (0/5) | 79.9 |
| 2×2 | heldout | 20-separate | 12.400 | +1.22% (0/5) | +2.33% (0/5) | +6.34% (0/5) | 76.7 |
| 2×2 | heldout | aeglos | 11.607 | -3.89% (5/5) | -3.73% (5/5) | +0.00% (0/5) | 808.9 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

The released controls improve the 1×2 and 2×2 reference times. The [independent C7a panel](../2026-11-19-g1-query-bound-c7a803/README.md) measured the same released controls at 7.809 and 12.051 ms on the original requests. The scorecard uses this panel’s same-host values alongside Aeglos.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

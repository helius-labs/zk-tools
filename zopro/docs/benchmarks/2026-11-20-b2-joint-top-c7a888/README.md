# Joint indexed leading window for 1×2 B2 on C7a

The [Ryzen trial](../2026-11-20-b2-joint-top-ryzen887/README.md) found a 1×2 benefit from processing both signed G2 components together in the leading window. This run uses the same executable on c7a.24xlarge. The joint modes count and scatter both components together and weight the merged leading window once. The `12x3` control retains separate component reductions with the same three-range geometry. The `11x4` control uses four 11-bit ranges.

Zopro uses 80 main and 16 frontend physical cores, and prepared Aeglos uses all 96. Each process primes three proofs and times thirteen. Each principal transaction has eight paired blocks and each additional transaction has four. A joint mode advances with a lower paired median and at least six wins per principal input and three per additional input against its Zopro controls. All 175 exports independently verified. Separate probes confirmed the selected routes and the fallback at zero optional workspace. The temporary instance and storage were removed.

The fastest joint mode on the original request was `12x3-joint` at 9.205 ms. It took 8.995 ms on the held-out request. Neither joint mode improved all matched Zopro controls across the four inputs.

| Input | Profile | Proof ms | Paired vs ordinary | Paired vs 12x3 | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 3.7 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 8.925 | +2.91% (3/8) | -1.89% (5/8) | +0.00% (0/8) | +9.95% (0/8) | 42.2 | 0/8 |
| catalog | ordinary | 8.700 | +0.00% (0/8) | -1.33% (5/8) | -2.79% (5/8) | +6.87% (0/8) | 40.7 | 0/8 |
| catalog | 11x4 | 8.913 | +4.63% (2/8) | +0.72% (4/8) | +2.48% (3/8) | +11.41% (0/8) | 43.0 | 0/8 |
| catalog | 12x3 | 8.929 | +1.36% (3/8) | +0.00% (0/8) | +1.93% (3/8) | +9.21% (0/8) | 40.7 | 0/8 |
| catalog | 12x2-joint | 9.586 | +8.60% (0/8) | +7.45% (1/8) | +11.43% (1/8) | +17.25% (0/8) | 39.9 | 0/8 |
| catalog | 12x3-joint | 9.205 | +3.97% (1/8) | +3.30% (3/8) | +5.19% (1/8) | +12.11% (0/8) | 41.5 | 0/8 |
| catalog | aeglos | 8.127 | -6.42% (8/8) | -8.43% (8/8) | -9.04% (8/8) | +0.00% (0/8) | 457.6 | 0/8 |
| heldout | release | 8.858 | -0.08% (5/8) | -3.69% (6/8) | +0.00% (0/8) | +7.06% (1/8) | 40.7 | 0/8 |
| heldout | ordinary | 8.922 | +0.00% (0/8) | -2.78% (4/8) | +0.08% (3/8) | +8.74% (0/8) | 43.0 | 0/8 |
| heldout | 11x4 | 9.071 | +1.03% (4/8) | +0.12% (4/8) | +2.16% (4/8) | +8.39% (0/8) | 39.3 | 0/8 |
| heldout | 12x3 | 9.155 | +2.97% (4/8) | +0.00% (0/8) | +3.83% (2/8) | +10.51% (0/8) | 42.2 | 0/8 |
| heldout | 12x2-joint | 9.530 | +5.03% (2/8) | +4.19% (2/8) | +5.40% (0/8) | +14.54% (0/8) | 40.8 | 0/8 |
| heldout | 12x3-joint | 8.995 | +1.19% (3/8) | -1.61% (5/8) | +1.60% (3/8) | +10.32% (0/8) | 41.6 | 0/8 |
| heldout | aeglos | 8.179 | -8.04% (8/8) | -9.51% (8/8) | -6.59% (7/8) | +0.00% (0/8) | 456.6 | 0/8 |
| v27 | release | 8.679 | -2.34% (2/4) | -5.25% (4/4) | +0.00% (0/4) | +4.90% (0/4) | 41.6 | 0/4 |
| v27 | ordinary | 8.898 | +0.00% (0/4) | -4.91% (3/4) | +2.54% (2/4) | +7.90% (0/4) | 41.6 | 0/4 |
| v27 | 11x4 | 9.137 | +5.22% (1/4) | +1.03% (1/4) | +6.08% (0/4) | +13.29% (0/4) | 37.8 | 0/4 |
| v27 | 12x3 | 9.134 | +5.17% (1/4) | +0.00% (0/4) | +5.56% (0/4) | +12.13% (0/4) | 39.9 | 0/4 |
| v27 | 12x2-joint | 9.817 | +10.32% (1/4) | +4.92% (1/4) | +7.17% (0/4) | +15.70% (0/4) | 42.9 | 0/4 |
| v27 | 12x3-joint | 9.412 | +7.69% (0/4) | +5.36% (1/4) | +9.93% (0/4) | +16.15% (0/4) | 40.0 | 0/4 |
| v27 | aeglos | 8.058 | -7.31% (4/4) | -10.82% (4/4) | -4.67% (4/4) | +0.00% (0/4) | 453.3 | 0/4 |
| v28 | release | 9.124 | -3.71% (3/4) | +1.01% (0/4) | +0.00% (0/4) | +6.02% (1/4) | 39.3 | 0/4 |
| v28 | ordinary | 9.431 | +0.00% (0/4) | +4.94% (1/4) | +3.88% (1/4) | +7.59% (0/4) | 42.2 | 0/4 |
| v28 | 11x4 | 9.002 | -2.80% (3/4) | +1.79% (1/4) | +0.78% (2/4) | +5.32% (0/4) | 40.0 | 0/4 |
| v28 | 12x3 | 8.999 | -4.67% (3/4) | +0.00% (0/4) | -1.00% (4/4) | +4.94% (1/4) | 41.4 | 0/4 |
| v28 | 12x2-joint | 9.294 | -0.75% (3/4) | +3.56% (0/4) | +3.10% (1/4) | +8.74% (0/4) | 41.4 | 0/4 |
| v28 | 12x3-joint | 9.148 | -3.17% (3/4) | +1.61% (2/4) | +0.58% (2/4) | +8.22% (0/4) | 39.4 | 0/4 |
| v28 | aeglos | 8.440 | -7.04% (4/4) | -4.62% (3/4) | -5.55% (3/4) | +0.00% (0/4) | 459.4 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Routes](artifacts/routes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

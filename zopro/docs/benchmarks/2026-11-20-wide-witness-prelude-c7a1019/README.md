# Witness execution on the G2 pool on 96-core C7a

Moving witness instructions to the G2 pool increased paired latency by 3.52–5.01% on 1×4, 1.65–5.57% on 1×16, and 2.76–3.01% on 2×2 against same-build controls at the same widths. Moving the full prelude increased latency by 4.33–8.60% on 1×16 and 4.88–5.46% on 2×2. The Ryzen gains did not transfer to these C7a placements.

Zopro has paired leads on 39 of 41 shapes across matched shape-specific C7a runs.

This panel uses the exact [Ryzen trial](../2026-11-20-wide-g2-witness-prelude-ryzen1018/README.md) and the scoped SDK on a c7a.24xlarge. The trial can run either the witness instructions or the complete witness prelude on the G2 pool before curve work starts. The main and G2 pools use disjoint physical cores. Prepared Aeglos uses 96 physical cores.

| Shape | Request | Selected main+G2 profile | Zopro / Aeglos ms | Paired vs exact SDK | Paired vs Aeglos | Peak Zopro / Aeglos MiB |
|---|---|---|---:|---:|---:|---:|
| 1×4 | catalog | 48+48, Trial control | 9.609 / 9.412 | -1.19% (6/8) | +2.01% (2/8) | 100.0 / 582.2 |
| 1×4 | heldout | 48+48, Trial control | 9.664 / 9.739 | -1.34% (5/8) | -1.55% (5/8) | 103.7 / 587.6 |
| 1×16 | catalog | 32+64, Trial control | 12.760 / 12.158 | +0.66% (2/8) | +4.95% (0/8) | 175.8 / 818.8 |
| 1×16 | heldout | 32+64, Trial control | 12.722 / 12.392 | -0.97% (6/8) | +2.60% (0/8) | 163.9 / 824.5 |
| 2×2 | catalog | 32+64, Trial control | 10.635 / 11.555 | +0.25% (4/8) | -8.09% (8/8) | 79.1 / 809.0 |
| 2×2 | heldout | 32+64, Trial control | 10.687 / 11.737 | -0.18% (5/8) | -8.19% (8/8) | 78.4 / 806.6 |

A pairing qualifies with a lower paired median and at least six wins in eight blocks on both requests. Selection first considers variants that qualify against both controls and Aeglos. It then considers Aeglos leads, variants with gains against both SDK controls, and remaining Zopro profiles in that order, using the lowest worse-request median in each case.

| Shape | Request | Main+G2 profile | Proof ms | Paired vs exact SDK | Paired vs Aeglos | Instructions / prelude ms | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×4 | catalog | 56+40, Exact SDK | 9.727 | +0.00% (0/8) | +4.39% (3/8) | — | 101.4 |
| 1×4 | catalog | 56+40, Trial control | 9.755 | +0.54% (3/8) | +3.20% (1/8) | 1.464 / 2.123 | 94.0 |
| 1×4 | catalog | 56+40, G2 pool for instructions | 10.220 | +5.47% (0/8) | +9.28% (0/8) | 1.747 / 2.589 | 100.7 |
| 1×4 | catalog | 48+48, Trial control | 9.609 | -1.19% (6/8) | +2.01% (2/8) | 1.513 / 2.190 | 100.0 |
| 1×4 | catalog | 48+48, G2 pool for instructions | 9.995 | +1.98% (1/8) | +5.67% (1/8) | 1.865 / 2.528 | 104.4 |
| 1×4 | catalog | 40+56, Trial control | 9.676 | -0.09% (4/8) | +2.94% (3/8) | 1.467 / 2.048 | 108.2 |
| 1×4 | catalog | 40+56, G2 pool for instructions | 10.015 | +2.55% (0/8) | +6.90% (0/8) | 1.824 / 2.533 | 108.1 |
| 1×4 | catalog | Prepared Aeglos | 9.412 | -4.20% (5/8) | +0.00% (0/8) | — | 582.2 |
| 1×4 | heldout | 56+40, Exact SDK | 9.805 | +0.00% (0/8) | +0.11% (4/8) | — | 99.2 |
| 1×4 | heldout | 56+40, Trial control | 9.717 | -0.42% (4/8) | -0.47% (5/8) | 1.543 / 2.177 | 95.4 |
| 1×4 | heldout | 56+40, G2 pool for instructions | 10.167 | +4.34% (0/8) | +4.54% (0/8) | 1.782 / 2.539 | 98.4 |
| 1×4 | heldout | 48+48, Trial control | 9.664 | -1.34% (5/8) | -1.55% (5/8) | 1.495 / 2.210 | 103.7 |
| 1×4 | heldout | 48+48, G2 pool for instructions | 9.952 | +1.78% (0/8) | +2.73% (1/8) | 1.897 / 2.623 | 106.0 |
| 1×4 | heldout | 40+56, Trial control | 9.646 | -1.36% (6/8) | -0.93% (6/8) | 1.514 / 2.050 | 106.6 |
| 1×4 | heldout | 40+56, G2 pool for instructions | 10.023 | +2.32% (0/8) | +3.24% (0/8) | 1.834 / 2.440 | 101.4 |
| 1×4 | heldout | Prepared Aeglos | 9.739 | -0.10% (4/8) | +0.00% (0/8) | — | 587.6 |
| 1×16 | catalog | 32+64, Exact SDK | 12.755 | +0.00% (0/8) | +4.41% (0/8) | — | 162.9 |
| 1×16 | catalog | 32+64, Trial control | 12.760 | +0.66% (2/8) | +4.95% (0/8) | 2.108 / 2.804 | 175.8 |
| 1×16 | catalog | 32+64, G2 pool for instructions | 13.046 | +2.58% (0/8) | +7.28% (0/8) | 2.338 / 3.087 | 168.2 |
| 1×16 | catalog | 32+64, G2 pool for prelude | 13.330 | +4.44% (0/8) | +9.35% (0/8) | 2.133 / 3.476 | 151.9 |
| 1×16 | catalog | 40+56, Trial control | 12.766 | +0.48% (3/8) | +4.71% (0/8) | 2.172 / 2.932 | 161.5 |
| 1×16 | catalog | 40+56, G2 pool for instructions | 13.469 | +6.10% (0/8) | +10.48% (0/8) | 2.691 / 3.567 | 167.5 |
| 1×16 | catalog | 40+56, G2 pool for prelude | 13.800 | +8.65% (0/8) | +13.43% (0/8) | 2.663 / 3.713 | 146.6 |
| 1×16 | catalog | Prepared Aeglos | 12.158 | -4.22% (8/8) | +0.00% (0/8) | — | 818.8 |
| 1×16 | heldout | 32+64, Exact SDK | 12.769 | +0.00% (0/8) | +3.70% (0/8) | — | 169.0 |
| 1×16 | heldout | 32+64, Trial control | 12.722 | -0.97% (6/8) | +2.60% (0/8) | 2.056 / 2.753 | 163.9 |
| 1×16 | heldout | 32+64, G2 pool for instructions | 13.103 | +2.27% (1/8) | +6.17% (0/8) | 2.306 / 3.064 | 163.9 |
| 1×16 | heldout | 32+64, G2 pool for prelude | 13.335 | +3.84% (0/8) | +7.98% (0/8) | 2.087 / 3.250 | 151.0 |
| 1×16 | heldout | 40+56, Trial control | 12.836 | +0.20% (2/8) | +4.31% (0/8) | 2.155 / 2.963 | 163.0 |
| 1×16 | heldout | 40+56, G2 pool for instructions | 13.517 | +5.03% (1/8) | +9.44% (0/8) | 2.797 / 3.698 | 165.9 |
| 1×16 | heldout | 40+56, G2 pool for prelude | 13.859 | +8.16% (0/8) | +12.73% (0/8) | 2.587 / 3.733 | 148.7 |
| 1×16 | heldout | Prepared Aeglos | 12.392 | -3.57% (8/8) | +0.00% (0/8) | — | 824.5 |
| 2×2 | catalog | 32+64, Exact SDK | 10.617 | +0.00% (0/8) | -7.52% (8/8) | — | 76.0 |
| 2×2 | catalog | 32+64, Trial control | 10.635 | +0.25% (4/8) | -8.09% (8/8) | 1.757 / 2.388 | 79.1 |
| 2×2 | catalog | 32+64, G2 pool for instructions | 10.951 | +3.27% (0/8) | -4.86% (8/8) | 2.070 / 2.768 | 75.9 |
| 2×2 | catalog | 32+64, G2 pool for prelude | 11.153 | +5.09% (0/8) | -3.38% (8/8) | 1.944 / 2.934 | 80.4 |
| 2×2 | catalog | Prepared Aeglos | 11.555 | +8.13% (0/8) | +0.00% (0/8) | — | 809.0 |
| 2×2 | heldout | 32+64, Exact SDK | 10.763 | +0.00% (0/8) | -8.13% (8/8) | — | 75.2 |
| 2×2 | heldout | 32+64, Trial control | 10.687 | -0.18% (5/8) | -8.19% (8/8) | 1.662 / 2.322 | 78.4 |
| 2×2 | heldout | 32+64, G2 pool for instructions | 11.014 | +2.72% (0/8) | -5.56% (8/8) | 2.065 / 2.809 | 81.9 |
| 2×2 | heldout | 32+64, G2 pool for prelude | 11.271 | +4.71% (0/8) | -3.66% (8/8) | 1.941 / 3.143 | 74.5 |
| 2×2 | heldout | Prepared Aeglos | 11.737 | +8.85% (0/8) | +0.00% (0/8) | — | 806.6 |

Eight timing blocks prime three proofs and time seventeen per process. Separate route blocks measure instruction and prelude duration and check the active pool. The Gnark verifier checked all 396 C7a exports independently. The temporary instance, storage and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.825 | 37 / 24 | 4.73× / 3.07× | [Previous C7a panel](../2026-11-20-exclusive-g2-coarse-split-c7a1017/README.md) |
| 2×2 | 10.621 | 59 / 30 | 5.56× / 2.82× | [Previous C7a panel](../2026-11-20-exclusive-g2-coarse-split-c7a1017/README.md) |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five catalog shapes meet 10× against M5 CPU. All eight Metal comparisons are below 10×.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

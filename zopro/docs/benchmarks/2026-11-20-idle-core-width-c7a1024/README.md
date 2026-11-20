# Narrower G2 pools and G1 window geometry on C7a

Zopro has paired leads on 40 of 41 shapes across matched shape-specific C7a runs.

The 1×4 lead reproduced with fused K/H. Fusion lowered paired latency by 8.45–8.55% at the same 48+32 placement and won every block against Aeglos. On 1×16, width 13 lowered paired latency by 2.11–2.74% at the same 40+40 placement. It won six catalog blocks and four held-out blocks against Aeglos. On 2×2, 32+32 improved both inputs by more than 3% against the 32+64 SDK control. None of the 4×2 placements passed the paired comparison against its SDK control.

The panel compares the exact SDK with narrower G2 pools that leave unassigned physical cores idle, without constructing another worker pool. The previous trial with 16 idle workers is retained as a control on 1×4, 1×16 and 2×2. The window-geometry variants use the same SDK and pinned key. The grouped 4×2 run uses the independent solver placement from its previous C7a panel.

| Shape | Request | Selected profile | Zopro / Aeglos ms | Paired vs exact SDK | Paired vs Aeglos | Peak Zopro / Aeglos MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | Pinned 1×2 | 7.779 / 8.078 | -2.97% (7/8) | -3.50% (8/8) | 40.9 / 459.5 |
| 1×2 | heldout | Pinned 1×2 | 7.771 / 8.204 | +0.50% (2/8) | -5.24% (8/8) | 45.4 / 461.4 |
| 1×4 | catalog | 48+32, fused K/H | 8.747 / 9.750 | -9.33% (8/8) | -10.48% (8/8) | 59.5 / 586.9 |
| 1×4 | heldout | 48+32, fused K/H | 8.751 / 9.664 | -9.60% (8/8) | -9.65% (8/8) | 61.0 / 584.1 |
| 1×16 | catalog | 40+40, G1 width 13 | 12.297 / 12.522 | -3.68% (8/8) | -1.29% (6/8) | 164.5 / 825.8 |
| 1×16 | heldout | 40+40, G1 width 13 | 12.310 / 12.309 | -3.62% (8/8) | -0.24% (4/8) | 173.5 / 823.0 |
| 2×2 | catalog | 32+32, SDK | 10.287 / 11.929 | -3.11% (8/8) | -13.62% (8/8) | 82.8 / 810.4 |
| 2×2 | heldout | 32+32, SDK | 10.295 / 11.762 | -3.27% (8/8) | -12.67% (8/8) | 77.5 / 811.6 |
| 4×2 | catalog | 64+16 / 12 solver, SDK | 13.870 / 18.324 | -0.53% (5/8) | -24.22% (8/8) | 216.9 / 1351.0 |
| 4×2 | heldout | 64+16 / 12 solver, SDK | 13.869 / 18.368 | -0.65% (5/8) | -24.51% (8/8) | 208.0 / 1350.4 |

A paired lead requires a lower paired median and six wins in eight blocks on both requests. A window change is compared with its same-width control in addition to the exact SDK. Selection first considers profiles improving both the SDK and Aeglos comparisons, then the paired Aeglos leads, then the profiles with an SDK improvement. Each group uses the lowest worse-request median.

| Shape | Request | Profile | Full proof ms | Paired vs exact SDK | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 48+48, SDK control | 8.018 | +0.00% (0/8) | +0.17% (3/8) | 52.2 |
| 1×2 | catalog | 48+32, SDK | 8.087 | +0.94% (2/8) | +0.78% (2/8) | 48.2 |
| 1×2 | catalog | 56+32, SDK | 8.140 | +1.56% (1/8) | +1.25% (2/8) | 47.6 |
| 1×2 | catalog | Pinned 1×2 | 7.779 | -2.97% (7/8) | -3.50% (8/8) | 40.9 |
| 1×2 | catalog | Prepared Aeglos | 8.078 | -0.17% (5/8) | +0.00% (0/8) | 459.5 |
| 1×2 | heldout | 48+48, SDK control | 7.730 | +0.00% (0/8) | -5.98% (8/8) | 54.3 |
| 1×2 | heldout | 48+32, SDK | 8.087 | +4.42% (0/8) | -1.30% (6/8) | 41.4 |
| 1×2 | heldout | 56+32, SDK | 8.157 | +5.40% (0/8) | -1.09% (5/8) | 41.5 |
| 1×2 | heldout | Pinned 1×2 | 7.771 | +0.50% (2/8) | -5.24% (8/8) | 45.4 |
| 1×2 | heldout | Prepared Aeglos | 8.204 | +6.37% (0/8) | +0.00% (0/8) | 461.4 |
| 1×4 | catalog | 48+48, SDK control | 9.634 | +0.00% (0/8) | -1.59% (5/8) | 106.0 |
| 1×4 | catalog | 48+32, SDK | 9.526 | -0.87% (6/8) | -2.44% (6/8) | 97.8 |
| 1×4 | catalog | 48+40, SDK | 9.603 | -0.09% (4/8) | -1.87% (5/8) | 95.5 |
| 1×4 | catalog | 56+32, SDK | 9.559 | -0.23% (4/8) | -1.56% (5/8) | 107.4 |
| 1×4 | catalog | 64+32, SDK | 9.707 | +0.87% (2/8) | +1.11% (3/8) | 108.9 |
| 1×4 | catalog | 48+32, trial with idle pool | 9.527 | -0.29% (5/8) | -2.13% (6/8) | 102.9 |
| 1×4 | catalog | 48+32, fused K/H | 8.747 | -9.33% (8/8) | -10.48% (8/8) | 59.5 |
| 1×4 | catalog | 48+32, fused K/H, A width 13 | 8.756 | -9.17% (8/8) | -10.19% (8/8) | 56.5 |
| 1×4 | catalog | 48+32, two G1 ranges | 9.400 | -2.87% (7/8) | -3.11% (7/8) | 100.6 |
| 1×4 | catalog | Prepared Aeglos | 9.750 | +1.61% (3/8) | +0.00% (0/8) | 586.9 |
| 1×4 | heldout | 48+48, SDK control | 9.678 | +0.00% (0/8) | +0.84% (3/8) | 109.0 |
| 1×4 | heldout | 48+32, SDK | 9.555 | -1.10% (8/8) | -1.15% (5/8) | 102.9 |
| 1×4 | heldout | 48+40, SDK | 9.535 | -1.61% (6/8) | -1.19% (7/8) | 96.2 |
| 1×4 | heldout | 56+32, SDK | 9.717 | +0.27% (3/8) | +0.07% (4/8) | 103.7 |
| 1×4 | heldout | 64+32, SDK | 9.731 | +0.16% (4/8) | +1.52% (2/8) | 107.4 |
| 1×4 | heldout | 48+32, trial with idle pool | 9.386 | -2.98% (7/8) | -2.12% (7/8) | 97.7 |
| 1×4 | heldout | 48+32, fused K/H | 8.751 | -9.60% (8/8) | -9.65% (8/8) | 61.0 |
| 1×4 | heldout | 48+32, fused K/H, A width 13 | 8.733 | -9.86% (8/8) | -9.46% (8/8) | 62.4 |
| 1×4 | heldout | 48+32, two G1 ranges | 9.414 | -2.83% (8/8) | -2.18% (8/8) | 103.7 |
| 1×4 | heldout | Prepared Aeglos | 9.664 | -0.83% (5/8) | +0.00% (0/8) | 584.1 |
| 1×16 | catalog | 32+64, SDK control | 12.766 | +0.00% (0/8) | +1.80% (2/8) | 165.2 |
| 1×16 | catalog | 32+40, SDK | 12.495 | -1.86% (8/8) | +0.11% (4/8) | 163.8 |
| 1×16 | catalog | 32+48, SDK | 12.531 | -1.32% (8/8) | +0.22% (3/8) | 172.1 |
| 1×16 | catalog | 40+40, SDK | 12.621 | -0.78% (5/8) | +0.36% (3/8) | 164.5 |
| 1×16 | catalog | 40+48, SDK | 12.660 | -0.66% (6/8) | +1.06% (3/8) | 169.8 |
| 1×16 | catalog | 32+48, trial with idle pool | 12.602 | -1.07% (7/8) | +1.57% (3/8) | 160.0 |
| 1×16 | catalog | 32+48, G1 width 13 | 12.756 | +0.21% (3/8) | +1.82% (2/8) | 157.8 |
| 1×16 | catalog | 40+40, G1 width 13 | 12.297 | -3.68% (8/8) | -1.29% (6/8) | 164.5 |
| 1×16 | catalog | 32+48, two G1 ranges | 12.795 | +0.28% (4/8) | +2.40% (1/8) | 160.7 |
| 1×16 | catalog | Prepared Aeglos | 12.522 | -1.77% (6/8) | +0.00% (0/8) | 825.8 |
| 1×16 | heldout | 32+64, SDK control | 12.770 | +0.00% (0/8) | +3.78% (0/8) | 165.9 |
| 1×16 | heldout | 32+40, SDK | 12.496 | -2.10% (8/8) | +1.44% (3/8) | 165.2 |
| 1×16 | heldout | 32+48, SDK | 12.581 | -1.36% (8/8) | +1.82% (2/8) | 162.2 |
| 1×16 | heldout | 40+40, SDK | 12.742 | -0.03% (4/8) | +2.37% (0/8) | 165.2 |
| 1×16 | heldout | 40+48, SDK | 12.800 | +0.00% (4/8) | +3.55% (1/8) | 167.6 |
| 1×16 | heldout | 32+48, trial with idle pool | 12.609 | -1.53% (8/8) | +1.11% (1/8) | 163.6 |
| 1×16 | heldout | 32+48, G1 width 13 | 12.611 | -1.69% (5/8) | +1.75% (1/8) | 163.7 |
| 1×16 | heldout | 40+40, G1 width 13 | 12.310 | -3.62% (8/8) | -0.24% (4/8) | 173.5 |
| 1×16 | heldout | 32+48, two G1 ranges | 12.770 | -0.09% (5/8) | +3.46% (1/8) | 160.7 |
| 1×16 | heldout | Prepared Aeglos | 12.309 | -3.65% (8/8) | +0.00% (0/8) | 823.0 |
| 2×2 | catalog | 32+64, SDK control | 10.611 | +0.00% (0/8) | -10.97% (8/8) | 84.2 |
| 2×2 | catalog | 32+32, SDK | 10.287 | -3.11% (8/8) | -13.62% (8/8) | 82.8 |
| 2×2 | catalog | 32+40, SDK | 10.348 | -2.49% (8/8) | -12.98% (8/8) | 81.2 |
| 2×2 | catalog | 32+48, SDK | 10.435 | -1.76% (8/8) | -12.72% (8/8) | 79.7 |
| 2×2 | catalog | 40+40, SDK | 10.685 | +0.56% (1/8) | -10.55% (8/8) | 74.5 |
| 2×2 | catalog | 32+48, trial with idle pool | 10.486 | -1.13% (8/8) | -12.10% (8/8) | 78.4 |
| 2×2 | catalog | Prepared Aeglos | 11.929 | +12.33% (0/8) | +0.00% (0/8) | 810.4 |
| 2×2 | heldout | 32+64, SDK control | 10.650 | +0.00% (0/8) | -9.67% (8/8) | 75.9 |
| 2×2 | heldout | 32+32, SDK | 10.295 | -3.27% (8/8) | -12.67% (8/8) | 77.5 |
| 2×2 | heldout | 32+40, SDK | 10.325 | -2.91% (8/8) | -12.16% (8/8) | 82.9 |
| 2×2 | heldout | 32+48, SDK | 10.447 | -2.07% (8/8) | -11.14% (8/8) | 76.0 |
| 2×2 | heldout | 40+40, SDK | 10.715 | +0.45% (0/8) | -8.59% (8/8) | 73.7 |
| 2×2 | heldout | 32+48, trial with idle pool | 10.474 | -1.34% (8/8) | -11.04% (8/8) | 78.3 |
| 2×2 | heldout | Prepared Aeglos | 11.762 | +10.71% (0/8) | +0.00% (0/8) | 811.6 |
| 4×2 | catalog | 72+24 / 16 solver, SDK control | 13.946 | +0.00% (0/8) | -24.77% (8/8) | 222.2 |
| 4×2 | catalog | 64+24 / 16 solver, SDK | 13.971 | +0.17% (4/8) | -23.78% (8/8) | 210.1 |
| 4×2 | catalog | 64+16 / 12 solver, SDK | 13.870 | -0.53% (5/8) | -24.22% (8/8) | 216.9 |
| 4×2 | catalog | 72+16 / 12 solver, SDK | 13.969 | -0.35% (5/8) | -23.43% (8/8) | 209.4 |
| 4×2 | catalog | 64+32 / 16 solver, SDK | 14.125 | +1.18% (1/8) | -23.10% (8/8) | 208.0 |
| 4×2 | catalog | Prepared Aeglos | 18.324 | +32.92% (0/8) | +0.00% (0/8) | 1351.0 |
| 4×2 | heldout | 72+24 / 16 solver, SDK control | 13.965 | +0.00% (0/8) | -24.17% (8/8) | 213.2 |
| 4×2 | heldout | 64+24 / 16 solver, SDK | 13.948 | +0.06% (4/8) | -23.86% (8/8) | 213.3 |
| 4×2 | heldout | 64+16 / 12 solver, SDK | 13.869 | -0.65% (5/8) | -24.51% (8/8) | 208.0 |
| 4×2 | heldout | 72+16 / 12 solver, SDK | 14.002 | -0.13% (5/8) | -23.96% (8/8) | 208.7 |
| 4×2 | heldout | 64+32 / 16 solver, SDK | 14.147 | +1.16% (0/8) | -22.89% (8/8) | 212.4 |
| 4×2 | heldout | Prepared Aeglos | 18.368 | +31.87% (0/8) | +0.00% (0/8) | 1350.4 |

Eight timing blocks prime three proofs and time seventeen. Four separate route processes confirm the fused K/H call. Gnark independently verified all 612 proof exports. The temporary C7a instance, storage and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.779 | 37 / 24 | 4.76× / 3.09× | This run |
| 2×2 | 10.287 | 59 / 30 | 5.74× / 2.92× | This run |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

5 of eight catalog shapes meet 10× against M5 CPU. 0 of eight meet 10× against Metal.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

# Exclusive G2 placement and preparation on C7a

The verified comparison is 39/41 across shape-specific C7a runs.

The remaining shapes without a Zopro paired lead are 1×4, 1×16.

This panel follows the [previous G2 width run](../2026-11-20-g2-frontend-width-c7a1012/README.md). Profiles partition 96 physical cores between the main work and the exclusive G2 pool. The preparation trial overlaps point reduction and scalar decomposition in the grouped G2 path. Prepared Aeglos uses all 96 cores.

| Shape | Selected main+G2 and mode | Catalog Zopro / Aeglos ms | Held-out Zopro / Aeglos ms | Paired catalog / held-out | Peak Zopro / Aeglos MiB |
|---|---|---:|---:|---:|---:|
| 1×4 | 56+40, Grouping off | 9.755 / 9.642 | 9.785 / 9.697 | +0.71% (3/8) / +1.39% (3/8) | 97.1 / 590.7 |
| 1×8 | 32+64, Concurrent scalar preparation | 10.374 / 10.616 | 10.326 / 10.681 | -2.29% (7/8) / -3.17% (7/8) | 139.1 / 690.7 |
| 1×16 | 32+64, Grouping off | 12.712 / 12.209 | 12.798 / 12.304 | +4.01% (0/8) / +3.60% (0/8) | 164.6 / 825.7 |
| 2×2 | 32+64, Grouping off | 10.674 / 11.711 | 10.662 / 11.705 | -8.83% (8/8) / -9.10% (8/8) | 81.3 / 809.7 |

A profile qualifies with a lower paired median and at least six paired wins on each request. Selection uses the lowest worse-request median among qualifying profiles, or across all Zopro profiles if none qualifies.

On 1×8, the 32+64 preparation trial won seven of eight paired blocks on each request against Aeglos. The same-width preparation control won four and five, respectively. On 1×16, bypassing grouping at 32+64 saved 0.848 and 0.779 ms against the SDK control, leaving gaps of 0.503 and 0.494 ms to Aeglos. Earlier SDK traces placed witness solving at about 3.0–3.1 ms and G2 completion about 0.23–0.27 ms after the last observed G1 call. This panel measures wall time on the new profiles. It lacks matched phase timings for the new profiles and Aeglos. [Earlier SDK phase diagnostics](../2026-11-20-scoped-sdk-transfer-c7a1007/README.md#elf-phase-diagnostics)

| Shape | Request | Main+G2 and mode | Proof ms | Paired vs SDK control | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×4 | catalog | 64+32, SDK control | 9.994 | +0.00% (0/8) | +3.71% (0/8) | 153.9 |
| 1×4 | catalog | 64+32, Grouping off | 9.797 | -2.42% (7/8) | +1.38% (1/8) | 107.5 |
| 1×4 | catalog | 56+40, Grouping off | 9.755 | -2.81% (7/8) | +0.71% (3/8) | 93.2 |
| 1×4 | catalog | 72+24, Grouping off | 10.296 | +3.02% (1/8) | +6.03% (0/8) | 102.9 |
| 1×4 | catalog | 56+40, Concurrent scalar preparation | 9.818 | -2.02% (7/8) | +1.82% (1/8) | 146.4 |
| 1×4 | catalog | Prepared Aeglos | 9.642 | -3.58% (8/8) | +0.00% (0/8) | 581.9 |
| 1×4 | heldout | 64+32, SDK control | 9.912 | +0.00% (0/8) | +1.66% (2/8) | 151.7 |
| 1×4 | heldout | 64+32, Grouping off | 9.781 | -1.58% (5/8) | +1.56% (2/8) | 100.7 |
| 1×4 | heldout | 56+40, Grouping off | 9.785 | -1.74% (7/8) | +1.39% (3/8) | 97.1 |
| 1×4 | heldout | 72+24, Grouping off | 10.330 | +4.20% (0/8) | +6.29% (1/8) | 105.9 |
| 1×4 | heldout | 56+40, Concurrent scalar preparation | 9.781 | -1.09% (7/8) | +0.80% (3/8) | 153.2 |
| 1×4 | heldout | Prepared Aeglos | 9.697 | -1.63% (6/8) | +0.00% (0/8) | 590.7 |
| 1×8 | catalog | 48+48, SDK control | 11.082 | +0.00% (0/8) | +4.50% (0/8) | 139.8 |
| 1×8 | catalog | 40+56, Preparation control | 10.748 | -3.45% (8/8) | +0.84% (3/8) | 134.5 |
| 1×8 | catalog | 32+64, Preparation control | 10.533 | -4.32% (8/8) | +0.11% (4/8) | 137.5 |
| 1×8 | catalog | 40+56, Concurrent scalar preparation | 10.608 | -4.20% (8/8) | +0.09% (4/8) | 151.8 |
| 1×8 | catalog | 32+64, Concurrent scalar preparation | 10.374 | -6.09% (8/8) | -2.29% (7/8) | 138.2 |
| 1×8 | catalog | Prepared Aeglos | 10.616 | -4.30% (8/8) | +0.00% (0/8) | 690.7 |
| 1×8 | heldout | 48+48, SDK control | 11.126 | +0.00% (0/8) | +4.61% (1/8) | 142.6 |
| 1×8 | heldout | 40+56, Preparation control | 10.872 | -3.07% (7/8) | +2.31% (1/8) | 140.5 |
| 1×8 | heldout | 32+64, Preparation control | 10.538 | -5.26% (7/8) | -1.15% (5/8) | 137.4 |
| 1×8 | heldout | 40+56, Concurrent scalar preparation | 10.561 | -4.82% (8/8) | -0.65% (6/8) | 139.8 |
| 1×8 | heldout | 32+64, Concurrent scalar preparation | 10.326 | -6.83% (8/8) | -3.17% (7/8) | 139.1 |
| 1×8 | heldout | Prepared Aeglos | 10.681 | -4.40% (7/8) | +0.00% (0/8) | 690.2 |
| 1×16 | catalog | 48+48, SDK control | 13.560 | +0.00% (0/8) | +11.77% (0/8) | 169.8 |
| 1×16 | catalog | 48+48, Grouping off | 12.721 | -7.15% (8/8) | +3.62% (0/8) | 167.5 |
| 1×16 | catalog | 40+56, Grouping off | 12.827 | -5.36% (8/8) | +4.93% (0/8) | 166.7 |
| 1×16 | catalog | 32+64, Grouping off | 12.712 | -6.05% (8/8) | +4.01% (0/8) | 164.6 |
| 1×16 | catalog | 40+56, Concurrent scalar preparation | 12.756 | -5.77% (8/8) | +4.32% (0/8) | 169.7 |
| 1×16 | catalog | Prepared Aeglos | 12.209 | -10.53% (8/8) | +0.00% (0/8) | 825.7 |
| 1×16 | heldout | 48+48, SDK control | 13.577 | +0.00% (0/8) | +10.08% (0/8) | 167.5 |
| 1×16 | heldout | 48+48, Grouping off | 12.851 | -5.24% (8/8) | +4.33% (0/8) | 170.4 |
| 1×16 | heldout | 40+56, Grouping off | 12.883 | -5.30% (8/8) | +4.24% (0/8) | 166.1 |
| 1×16 | heldout | 32+64, Grouping off | 12.798 | -5.85% (8/8) | +3.60% (0/8) | 162.1 |
| 1×16 | heldout | 40+56, Concurrent scalar preparation | 12.843 | -5.36% (8/8) | +3.74% (0/8) | 175.0 |
| 1×16 | heldout | Prepared Aeglos | 12.304 | -9.16% (8/8) | +0.00% (0/8) | 822.2 |
| 2×2 | catalog | 48+48, SDK control | 11.274 | +0.00% (0/8) | -3.95% (8/8) | 79.8 |
| 2×2 | catalog | 48+48, Grouping off | 11.061 | -1.71% (7/8) | -5.36% (7/8) | 79.7 |
| 2×2 | catalog | 40+56, Grouping off | 10.905 | -3.13% (8/8) | -7.31% (8/8) | 77.5 |
| 2×2 | catalog | 32+64, Grouping off | 10.674 | -5.33% (8/8) | -8.83% (8/8) | 81.3 |
| 2×2 | catalog | 40+56, Concurrent scalar preparation | 11.039 | -1.69% (8/8) | -5.23% (8/8) | 81.4 |
| 2×2 | catalog | Prepared Aeglos | 11.711 | +4.11% (0/8) | +0.00% (0/8) | 809.7 |
| 2×2 | heldout | 48+48, SDK control | 11.273 | +0.00% (0/8) | -3.25% (7/8) | 81.3 |
| 2×2 | heldout | 48+48, Grouping off | 11.038 | -1.86% (8/8) | -4.69% (8/8) | 79.1 |
| 2×2 | heldout | 40+56, Grouping off | 10.960 | -2.86% (8/8) | -6.61% (8/8) | 80.5 |
| 2×2 | heldout | 32+64, Grouping off | 10.662 | -5.31% (8/8) | -9.10% (8/8) | 76.7 |
| 2×2 | heldout | 40+56, Concurrent scalar preparation | 11.041 | -2.09% (8/8) | -4.57% (8/8) | 79.7 |
| 2×2 | heldout | Prepared Aeglos | 11.705 | +3.36% (1/8) | +0.00% (0/8) | 806.6 |

Each of eight counterbalanced blocks primes three proofs and times seventeen per process. The independent Gnark oracle verified all 384 proof exports with the pinned verification keys. The temporary instance, storage, and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.864 | 37 / 24 | 4.70× / 3.05× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 2×2 | 10.674 | 59 / 30 | 5.53× / 2.81× | This panel |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five catalog shapes meet 10× against M5 CPU. All eight Metal comparisons are below 10×.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

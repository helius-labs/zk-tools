# Wider exclusive G2 pools on C7a

Aeglos retained the paired lead on 1×4, 1×8, 1×16.

The previously measured scoped SDK kept 64 main and 32 G2 cores for 1×4, and 48 main and 48 G2 cores for the other shapes. The width arms move eight or sixteen main cores to G2. The grouping arm runs G2 without scalar grouping. The preparation arms use a separate binary that can overlap point reduction and scalar decomposition. They share a worker mask and differ only in whether the preparation overlap is enabled.

| Shape | Selected profile | Catalog Zopro / Aeglos ms | Held-out Zopro / Aeglos ms | Paired catalog / held-out | Peak Zopro / Aeglos MiB |
|---|---|---:|---:|---:|---:|
| 1×4 | Grouping off | 9.808 / 9.565 | 9.733 / 9.552 | +2.69% (2/8) / +1.50% (3/8) | 101.4 / 584.6 |
| 1×8 | Sixteen cores to G2 | 10.738 / 10.608 | 10.749 / 10.731 | +1.67% (1/8) / +0.83% (1/8) | 141.1 / 695.4 |
| 1×16 | Grouping off | 12.736 / 12.318 | 12.890 / 12.301 | +2.92% (2/8) / +4.64% (0/8) | 171.2 / 824.4 |
| 2×2 | Grouping off | 11.008 / 11.556 | 11.101 / 11.581 | -4.75% (8/8) / -4.42% (7/8) | 82.1 / 810.0 |

Selection uses the lowest worse-request median among profiles with at least six paired wins and lower paired latency on each request against Aeglos. Where no profile qualifies, selection uses the lowest worse-request median among the Zopro profiles.

| Shape | Request | Profile | Proof ms | Paired vs scoped SDK | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×4 | catalog | Pinned catalog | 11.793 | +19.75% (0/8) | +22.85% (0/8) | 124.0 |
| 1×4 | catalog | Scoped SDK | 9.869 | +0.00% (0/8) | +3.43% (1/8) | 153.2 |
| 1×4 | catalog | Eight cores to G2 | 9.861 | -0.03% (4/8) | +3.58% (1/8) | 157.7 |
| 1×4 | catalog | Sixteen cores to G2 | 10.366 | +4.15% (0/8) | +8.38% (0/8) | 108.9 |
| 1×4 | catalog | Grouping off | 9.808 | -0.54% (5/8) | +2.69% (2/8) | 101.4 |
| 1×4 | catalog | Concurrent scalar preparation | 9.879 | +0.03% (4/8) | +3.68% (1/8) | 152.4 |
| 1×4 | catalog | Preparation control | 9.872 | +0.27% (3/8) | +3.25% (1/8) | 159.1 |
| 1×4 | catalog | Prepared Aeglos | 9.565 | -3.31% (7/8) | +0.00% (0/8) | 582.8 |
| 1×4 | heldout | Pinned catalog | 11.925 | +19.92% (0/8) | +25.64% (0/8) | 119.5 |
| 1×4 | heldout | Scoped SDK | 9.888 | +0.00% (0/8) | +3.75% (3/8) | 161.4 |
| 1×4 | heldout | Eight cores to G2 | 9.848 | -1.50% (7/8) | +2.95% (3/8) | 162.4 |
| 1×4 | heldout | Sixteen cores to G2 | 10.339 | +4.54% (0/8) | +7.91% (0/8) | 111.3 |
| 1×4 | heldout | Grouping off | 9.733 | -1.61% (7/8) | +1.50% (3/8) | 99.2 |
| 1×4 | heldout | Concurrent scalar preparation | 9.908 | -0.22% (5/8) | +2.41% (2/8) | 148.6 |
| 1×4 | heldout | Preparation control | 9.950 | +0.52% (2/8) | +3.49% (2/8) | 159.9 |
| 1×4 | heldout | Prepared Aeglos | 9.552 | -3.61% (5/8) | +0.00% (0/8) | 584.6 |
| 1×8 | catalog | Pinned catalog | 13.161 | +18.15% (0/8) | +24.11% (0/8) | 127.7 |
| 1×8 | catalog | Scoped SDK | 11.049 | +0.00% (0/8) | +5.26% (0/8) | 136.8 |
| 1×8 | catalog | Eight cores to G2 | 10.718 | -3.00% (7/8) | +1.78% (2/8) | 141.2 |
| 1×8 | catalog | Sixteen cores to G2 | 10.738 | -3.65% (7/8) | +1.67% (1/8) | 141.1 |
| 1×8 | catalog | Grouping off | 11.407 | +2.40% (2/8) | +7.74% (0/8) | 136.0 |
| 1×8 | catalog | Concurrent scalar preparation | 10.838 | -2.25% (7/8) | +3.70% (1/8) | 139.0 |
| 1×8 | catalog | Preparation control | 11.135 | +0.51% (3/8) | +3.91% (0/8) | 136.1 |
| 1×8 | catalog | Prepared Aeglos | 10.608 | -4.99% (8/8) | +0.00% (0/8) | 695.4 |
| 1×8 | heldout | Pinned catalog | 13.127 | +18.78% (0/8) | +24.66% (0/8) | 135.3 |
| 1×8 | heldout | Scoped SDK | 11.127 | +0.00% (0/8) | +3.63% (0/8) | 138.2 |
| 1×8 | heldout | Eight cores to G2 | 10.753 | -2.89% (8/8) | +0.62% (3/8) | 143.5 |
| 1×8 | heldout | Sixteen cores to G2 | 10.749 | -3.65% (8/8) | +0.83% (1/8) | 139.7 |
| 1×8 | heldout | Grouping off | 11.311 | +2.11% (1/8) | +7.54% (0/8) | 132.1 |
| 1×8 | heldout | Concurrent scalar preparation | 10.869 | -2.47% (7/8) | +1.27% (2/8) | 149.5 |
| 1×8 | heldout | Preparation control | 11.013 | -1.04% (6/8) | +3.27% (0/8) | 140.5 |
| 1×8 | heldout | Prepared Aeglos | 10.731 | -3.50% (8/8) | +0.00% (0/8) | 689.4 |
| 1×16 | catalog | Pinned catalog | 15.662 | +15.51% (0/8) | +26.84% (0/8) | 165.3 |
| 1×16 | catalog | Scoped SDK | 13.553 | +0.00% (0/8) | +9.90% (0/8) | 163.9 |
| 1×16 | catalog | Eight cores to G2 | 13.186 | -2.38% (7/8) | +6.54% (0/8) | 172.7 |
| 1×16 | catalog | Sixteen cores to G2 | 12.969 | -4.08% (8/8) | +5.46% (0/8) | 164.5 |
| 1×16 | catalog | Grouping off | 12.736 | -5.71% (8/8) | +2.92% (2/8) | 171.2 |
| 1×16 | catalog | Concurrent scalar preparation | 12.845 | -5.50% (8/8) | +4.09% (0/8) | 164.4 |
| 1×16 | catalog | Preparation control | 13.470 | -0.41% (5/8) | +9.58% (0/8) | 178.0 |
| 1×16 | catalog | Prepared Aeglos | 12.318 | -9.01% (8/8) | +0.00% (0/8) | 821.9 |
| 1×16 | heldout | Pinned catalog | 15.566 | +15.63% (0/8) | +25.56% (0/8) | 160.0 |
| 1×16 | heldout | Scoped SDK | 13.526 | +0.00% (0/8) | +9.70% (0/8) | 171.2 |
| 1×16 | heldout | Eight cores to G2 | 13.258 | -2.48% (8/8) | +7.71% (0/8) | 170.5 |
| 1×16 | heldout | Sixteen cores to G2 | 13.153 | -4.22% (8/8) | +6.14% (0/8) | 165.2 |
| 1×16 | heldout | Grouping off | 12.890 | -5.20% (8/8) | +4.64% (0/8) | 165.2 |
| 1×16 | heldout | Concurrent scalar preparation | 12.959 | -4.72% (8/8) | +5.30% (0/8) | 177.9 |
| 1×16 | heldout | Preparation control | 13.509 | +0.36% (2/8) | +9.63% (0/8) | 168.2 |
| 1×16 | heldout | Prepared Aeglos | 12.301 | -8.84% (8/8) | +0.00% (0/8) | 824.4 |
| 2×2 | catalog | Pinned catalog | 11.935 | +6.14% (0/8) | +3.28% (1/8) | 76.8 |
| 2×2 | catalog | Scoped SDK | 11.244 | +0.00% (0/8) | -2.66% (7/8) | 77.6 |
| 2×2 | catalog | Eight cores to G2 | 11.319 | +0.42% (2/8) | -1.57% (7/8) | 75.3 |
| 2×2 | catalog | Sixteen cores to G2 | 11.261 | +0.43% (3/8) | -2.49% (6/8) | 81.4 |
| 2×2 | catalog | Grouping off | 11.008 | -2.10% (8/8) | -4.75% (8/8) | 75.4 |
| 2×2 | catalog | Concurrent scalar preparation | 11.130 | -0.82% (7/8) | -3.33% (8/8) | 85.7 |
| 2×2 | catalog | Preparation control | 11.243 | -0.31% (4/8) | -2.72% (7/8) | 79.7 |
| 2×2 | catalog | Prepared Aeglos | 11.556 | +2.73% (1/8) | +0.00% (0/8) | 805.9 |
| 2×2 | heldout | Pinned catalog | 12.110 | +7.02% (0/8) | +4.67% (2/8) | 76.8 |
| 2×2 | heldout | Scoped SDK | 11.290 | +0.00% (0/8) | -3.16% (8/8) | 78.4 |
| 2×2 | heldout | Eight cores to G2 | 11.293 | +0.19% (2/8) | -2.98% (8/8) | 76.7 |
| 2×2 | heldout | Sixteen cores to G2 | 11.329 | +0.81% (2/8) | -3.35% (8/8) | 81.4 |
| 2×2 | heldout | Grouping off | 11.101 | -1.42% (7/8) | -4.42% (7/8) | 82.1 |
| 2×2 | heldout | Concurrent scalar preparation | 11.083 | -1.74% (8/8) | -4.83% (8/8) | 76.0 |
| 2×2 | heldout | Preparation control | 11.206 | -0.74% (5/8) | -3.70% (8/8) | 84.2 |
| 2×2 | heldout | Prepared Aeglos | 11.581 | +3.27% (0/8) | +0.00% (0/8) | 810.0 |

Eight counterbalanced blocks prime three proofs and time seventeen per process. The setup exports and all 512 C7a exports were verified independently. The temporary instance, storage and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.864 | 37 / 24 | 4.70× / 3.05× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 2×2 | 11.008 | 59 / 30 | 5.36× / 2.73× | This panel |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five catalog shapes meet 10× against M5 CPU. All eight Metal comparisons are below 10×.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

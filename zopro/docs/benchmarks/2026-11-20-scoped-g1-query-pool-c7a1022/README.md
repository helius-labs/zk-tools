# Dedicated G1 query placement on 96-core C7a

Leaving 16 cores idle lowered paired latency by 0.90–1.21% on 1×16 and 2.17–2.32% on 2×2 against the exact 32+64 SDK. Assigning one G1 query to those cores increased latency on both shapes versus the idle control. The separate G1 pool fell short of the paired comparisons on every shape.

Zopro has paired leads on 40 of 41 shapes across matched shape-specific C7a runs.

On 1×4, the 48+32 profile with 16 idle cores won six and seven paired blocks against Aeglos. Giving those cores to the main pool also qualified, winning six blocks on each request. This is the first qualified 1×4 panel. Its Aeglos medians were higher than in the previous run, so these results apply to the matched comparison on this host. [Previous C7a panel](../2026-11-20-wide-witness-prelude-c7a1019/README.md)

This panel compares the exact [Ryzen trial](../2026-11-20-scoped-g1-query-pool-ryzen1020/README.md) with the scoped SDK and prepared Aeglos. The disjoint affinity masks split 96 physical cores among the main pool, the G2 pool, and an optional third pool for one G1 query. Placement controls give the same cores to the main pool or leave the third pool idle.

| Shape | Request | Selected profile | Zopro / Aeglos ms | Paired vs exact SDK | Paired vs Aeglos | Peak Zopro / Aeglos MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | Pinned 1×2 | 7.786 / 8.197 | -3.06% (8/8) | -4.79% (8/8) | 43.1 / 461.9 |
| 1×2 | heldout | Pinned 1×2 | 7.732 / 8.224 | +0.29% (3/8) | -5.02% (8/8) | 42.3 / 463.3 |
| 1×4 | catalog | 48+32+16, third pool idle | 9.560 / 9.836 | -2.05% (7/8) | -2.80% (6/8) | 101.5 / 586.2 |
| 1×4 | heldout | 48+32+16, third pool idle | 9.588 / 9.838 | -0.27% (4/8) | -2.28% (7/8) | 104.5 / 589.4 |
| 1×16 | catalog | 32+48+16, third pool idle | 12.583 / 12.545 | -0.90% (7/8) | +0.66% (3/8) | 163.7 / 820.4 |
| 1×16 | heldout | 32+48+16, third pool idle | 12.576 / 12.409 | -1.21% (7/8) | +1.66% (1/8) | 172.0 / 829.5 |
| 2×2 | catalog | 32+48+16, third pool idle | 10.452 / 11.712 | -2.17% (8/8) | -10.64% (8/8) | 75.3 / 807.5 |
| 2×2 | heldout | 32+48+16, third pool idle | 10.425 / 11.914 | -2.32% (8/8) | -12.09% (8/8) | 74.5 / 806.2 |

Profiles pass a paired comparison with a lower paired median and at least six of eight wins on both requests. The third pool must beat the exact SDK, the corresponding main-allocation and idle controls, and the pinned runner on 1×2. Selection considers qualifying Aeglos leads first and uses the lowest worse-request median among comparable profiles.

| Shape | Request | Profile | Proof ms | Paired vs exact SDK | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | Pinned 1×2 | 7.786 | -3.06% (8/8) | -4.79% (8/8) | 43.1 |
| 1×2 | catalog | 48+48, exact SDK | 8.046 | +0.00% (0/8) | -2.14% (6/8) | 51.4 |
| 1×2 | catalog | 64+32, trial control | 8.236 | +2.72% (0/8) | +0.20% (4/8) | 45.2 |
| 1×2 | catalog | 48+32+16, third pool idle | 8.079 | +0.80% (1/8) | -0.89% (5/8) | 49.9 |
| 1×2 | catalog | 48+32+16, A | 9.505 | +19.35% (0/8) | +15.91% (0/8) | 47.6 |
| 1×2 | catalog | Prepared Aeglos | 8.197 | +2.19% (2/8) | +0.00% (0/8) | 461.9 |
| 1×2 | heldout | Pinned 1×2 | 7.732 | +0.29% (3/8) | -5.02% (8/8) | 42.3 |
| 1×2 | heldout | 48+48, exact SDK | 7.728 | +0.00% (0/8) | -6.14% (8/8) | 51.2 |
| 1×2 | heldout | 64+32, trial control | 8.243 | +6.44% (0/8) | +0.09% (4/8) | 44.5 |
| 1×2 | heldout | 48+32+16, third pool idle | 8.060 | +4.31% (0/8) | -1.70% (7/8) | 40.7 |
| 1×2 | heldout | 48+32+16, A | 8.186 | +6.32% (0/8) | +0.26% (3/8) | 46.8 |
| 1×2 | heldout | Prepared Aeglos | 8.224 | +6.54% (0/8) | +0.00% (0/8) | 463.3 |
| 1×4 | catalog | 48+48, exact SDK | 9.763 | +0.00% (0/8) | -0.45% (5/8) | 108.1 |
| 1×4 | catalog | 48+48, trial control | 9.613 | -0.30% (5/8) | -1.96% (5/8) | 105.1 |
| 1×4 | catalog | 64+32, trial control | 9.560 | -1.35% (6/8) | -0.53% (6/8) | 102.1 |
| 1×4 | catalog | 56+40, trial control | 9.658 | -1.09% (5/8) | -1.53% (5/8) | 99.2 |
| 1×4 | catalog | 48+32+16, third pool idle | 9.560 | -2.05% (7/8) | -2.80% (6/8) | 101.5 |
| 1×4 | catalog | 48+40+8, third pool idle | 9.618 | -0.68% (4/8) | -2.56% (5/8) | 96.9 |
| 1×4 | catalog | 48+32+16, K | 11.568 | +18.67% (0/8) | +17.70% (0/8) | 54.2 |
| 1×4 | catalog | 48+32+16, H | 9.976 | +2.28% (0/8) | +1.63% (1/8) | 92.4 |
| 1×4 | catalog | 48+40+8, K | 11.995 | +22.78% (0/8) | +22.07% (0/8) | 54.3 |
| 1×4 | catalog | 48+40+8, H | 11.664 | +19.54% (0/8) | +18.69% (0/8) | 93.2 |
| 1×4 | catalog | Prepared Aeglos | 9.836 | +0.45% (3/8) | +0.00% (0/8) | 586.2 |
| 1×4 | heldout | 48+48, exact SDK | 9.682 | +0.00% (0/8) | -1.40% (7/8) | 107.4 |
| 1×4 | heldout | 48+48, trial control | 9.627 | -0.34% (5/8) | -1.75% (6/8) | 106.6 |
| 1×4 | heldout | 64+32, trial control | 9.764 | +1.18% (1/8) | -0.70% (6/8) | 100.1 |
| 1×4 | heldout | 56+40, trial control | 9.744 | +1.37% (1/8) | -0.44% (5/8) | 96.9 |
| 1×4 | heldout | 48+32+16, third pool idle | 9.588 | -0.27% (4/8) | -2.28% (7/8) | 104.5 |
| 1×4 | heldout | 48+40+8, third pool idle | 9.668 | +0.45% (3/8) | -1.27% (7/8) | 101.5 |
| 1×4 | heldout | 48+32+16, K | 11.653 | +22.01% (0/8) | +18.74% (0/8) | 59.4 |
| 1×4 | heldout | 48+32+16, H | 9.997 | +4.24% (0/8) | +1.62% (0/8) | 95.5 |
| 1×4 | heldout | 48+40+8, K | 12.078 | +25.79% (0/8) | +22.55% (0/8) | 51.2 |
| 1×4 | heldout | 48+40+8, H | 11.665 | +20.86% (0/8) | +18.96% (0/8) | 93.2 |
| 1×4 | heldout | Prepared Aeglos | 9.838 | +1.42% (1/8) | +0.00% (0/8) | 589.4 |
| 1×16 | catalog | 32+64, exact SDK | 12.698 | +0.00% (0/8) | +1.07% (2/8) | 166.7 |
| 1×16 | catalog | 32+64, trial control | 12.742 | +0.72% (3/8) | +1.75% (0/8) | 162.3 |
| 1×16 | catalog | 48+48, trial control | 12.799 | +0.60% (3/8) | +1.64% (2/8) | 171.9 |
| 1×16 | catalog | 40+56, trial control | 12.872 | +1.66% (2/8) | +2.87% (0/8) | 161.4 |
| 1×16 | catalog | 32+48+16, third pool idle | 12.583 | -0.90% (7/8) | +0.66% (3/8) | 163.7 |
| 1×16 | catalog | 32+56+8, third pool idle | 12.670 | -0.32% (5/8) | +1.14% (3/8) | 162.1 |
| 1×16 | catalog | 32+48+16, K | 15.562 | +22.29% (0/8) | +24.16% (0/8) | 94.8 |
| 1×16 | catalog | 32+48+16, H | 13.924 | +9.55% (0/8) | +10.94% (0/8) | 157.8 |
| 1×16 | catalog | 32+56+8, K | 16.970 | +33.53% (0/8) | +34.96% (0/8) | 91.7 |
| 1×16 | catalog | 32+56+8, H | 16.311 | +28.68% (0/8) | +29.81% (0/8) | 167.5 |
| 1×16 | catalog | Prepared Aeglos | 12.545 | -1.05% (6/8) | +0.00% (0/8) | 820.4 |
| 1×16 | heldout | 32+64, exact SDK | 12.747 | +0.00% (0/8) | +2.73% (0/8) | 166.0 |
| 1×16 | heldout | 32+64, trial control | 12.766 | +0.30% (4/8) | +3.38% (0/8) | 168.1 |
| 1×16 | heldout | 48+48, trial control | 12.657 | -0.65% (6/8) | +2.30% (1/8) | 168.3 |
| 1×16 | heldout | 40+56, trial control | 12.817 | +0.49% (3/8) | +3.71% (0/8) | 169.0 |
| 1×16 | heldout | 32+48+16, third pool idle | 12.576 | -1.21% (7/8) | +1.66% (1/8) | 172.0 |
| 1×16 | heldout | 32+56+8, third pool idle | 12.667 | -0.89% (7/8) | +2.17% (0/8) | 163.8 |
| 1×16 | heldout | 32+48+16, K | 15.447 | +20.98% (0/8) | +24.69% (0/8) | 93.2 |
| 1×16 | heldout | 32+48+16, H | 13.989 | +9.10% (0/8) | +12.67% (0/8) | 167.5 |
| 1×16 | heldout | 32+56+8, K | 16.945 | +32.55% (0/8) | +36.28% (0/8) | 92.5 |
| 1×16 | heldout | 32+56+8, H | 16.281 | +27.41% (0/8) | +31.68% (0/8) | 159.3 |
| 1×16 | heldout | Prepared Aeglos | 12.409 | -2.66% (8/8) | +0.00% (0/8) | 829.5 |
| 2×2 | catalog | 32+64, exact SDK | 10.695 | +0.00% (0/8) | -8.70% (8/8) | 77.5 |
| 2×2 | catalog | 48+48, trial control | 11.131 | +3.99% (0/8) | -4.86% (8/8) | 78.2 |
| 2×2 | catalog | 32+48+16, third pool idle | 10.452 | -2.17% (8/8) | -10.64% (8/8) | 75.3 |
| 2×2 | catalog | 32+48+16, A | 10.627 | -0.56% (7/8) | -9.13% (8/8) | 81.3 |
| 2×2 | catalog | Prepared Aeglos | 11.712 | +9.53% (0/8) | +0.00% (0/8) | 807.5 |
| 2×2 | heldout | 32+64, exact SDK | 10.674 | +0.00% (0/8) | -10.26% (8/8) | 81.4 |
| 2×2 | heldout | 48+48, trial control | 11.032 | +3.12% (0/8) | -7.12% (8/8) | 76.0 |
| 2×2 | heldout | 32+48+16, third pool idle | 10.425 | -2.32% (8/8) | -12.09% (8/8) | 74.5 |
| 2×2 | heldout | 32+48+16, A | 10.678 | +0.05% (3/8) | -10.22% (8/8) | 74.5 |
| 2×2 | heldout | Prepared Aeglos | 11.914 | +11.43% (0/8) | +0.00% (0/8) | 806.2 |

Eight timing blocks prime three proofs and time seventeen per process. A separate diagnostic block checks query dispatch and worker counts. The independent Gnark oracle verified all 576 exports. The temporary C7a instance, storage and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.786 | 37 / 24 | 4.75× / 3.08× | This panel |
| 2×2 | 10.452 | 59 / 30 | 5.64× / 2.87× | This panel |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five catalog shapes meet 10× against M5 CPU. All eight Metal comparisons are below 10×.

[Summary](artifacts/summary.json) · [Samples and query timings](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

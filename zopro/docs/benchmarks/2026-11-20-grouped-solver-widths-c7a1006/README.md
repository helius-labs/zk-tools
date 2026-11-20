# Solver widths for grouped tiling on C7a

The same grouped-tiling SDK executable ran on one c7a.24xlarge. It uses the two requests from the [earlier C7a panel](../2026-11-20-grouped-sdk-widths-c7a1000/README.md). The main and frontend pools have disjoint masks. The solver uses a subset of the frontend pool. 8×2 keeps Euler leaf fusion and packed final exponentiation on across all width arms.

| Shape | Request | Selected profile | Selected / control ms | Paired vs control | M5 CPU / Metal | 10× CPU blocks |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | 72+24, solver 16 | 13.966 / 13.966 | +0.00% (0/10) | 7.80× / 3.44× | 0/10 |
| 4×2 | heldout | 72+24, solver 16 | 13.942 / 13.942 | +0.00% (0/10) | 7.82× / 3.44× | 0/10 |
| 8×2 | catalog | 72+24, solver 20 | 18.810 / 18.926 | -1.14% (7/10) | 10.05× / 3.93× | 7/10 |
| 8×2 | heldout | 72+24, solver 20 | 18.977 / 19.272 | -1.53% (9/10) | 9.96× / 3.90× | 3/10 |

A new layout qualifies with a lower paired median and at least seven wins in ten blocks on each request. The selection minimizes the worse-request median among qualifying layouts. The 72+24 control with sixteen solver workers remains selected where no new layout qualifies.

| Shape | Request | Profile | Proof ms | Paired vs control | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Pinned | 14.258 | +2.08% (1/10) | -22.68% (10/10) | 209.4 |
| 4×2 | catalog | 72+24, solver 12 | 13.978 | +0.09% (4/10) | -24.08% (10/10) | 211.7 |
| 4×2 | catalog | 72+24, solver 16 | 13.966 | +0.00% (0/10) | -23.92% (10/10) | 213.3 |
| 4×2 | catalog | 72+24, solver 20 | 14.117 | +1.15% (1/10) | -23.49% (10/10) | 213.2 |
| 4×2 | catalog | 72+24, solver 22 | 14.225 | +1.86% (1/10) | -22.51% (10/10) | 215.3 |
| 4×2 | catalog | 64+32, solver 20 | 14.307 | +2.54% (0/10) | -22.19% (10/10) | 211.7 |
| 4×2 | catalog | 64+32, solver 24 | 14.416 | +2.90% (0/10) | -21.72% (10/10) | 213.9 |
| 4×2 | catalog | 80+16, solver 12 | 14.188 | +1.49% (1/10) | -22.87% (10/10) | 217.7 |
| 4×2 | catalog | 80+16, solver 14 | 14.139 | +1.87% (2/10) | -23.20% (10/10) | 217.7 |
| 4×2 | catalog | Prepared Aeglos | 18.367 | +31.44% (0/10) | +0.00% (0/10) | 1351.6 |
| 4×2 | heldout | Pinned | 14.161 | +1.87% (1/10) | -23.09% (10/10) | 211.7 |
| 4×2 | heldout | 72+24, solver 12 | 13.972 | -0.13% (5/10) | -24.39% (10/10) | 214.1 |
| 4×2 | heldout | 72+24, solver 16 | 13.942 | +0.00% (0/10) | -24.78% (10/10) | 220.0 |
| 4×2 | heldout | 72+24, solver 20 | 14.121 | +1.25% (2/10) | -23.76% (10/10) | 234.3 |
| 4×2 | heldout | 72+24, solver 22 | 14.135 | +1.18% (2/10) | -23.76% (10/10) | 219.2 |
| 4×2 | heldout | 64+32, solver 20 | 14.317 | +2.82% (1/10) | -22.52% (10/10) | 219.2 |
| 4×2 | heldout | 64+32, solver 24 | 14.435 | +3.25% (0/10) | -21.84% (10/10) | 218.4 |
| 4×2 | heldout | 80+16, solver 12 | 14.223 | +1.95% (0/10) | -23.03% (10/10) | 216.9 |
| 4×2 | heldout | 80+16, solver 14 | 14.144 | +1.32% (0/10) | -23.42% (10/10) | 217.0 |
| 4×2 | heldout | Prepared Aeglos | 18.395 | +32.94% (0/10) | +0.00% (0/10) | 1352.4 |
| 8×2 | catalog | Pinned | 19.246 | +1.78% (2/10) | -37.75% (10/10) | 382.1 |
| 8×2 | catalog | 72+24, solver 12 | 19.441 | +2.95% (0/10) | -36.85% (10/10) | 370.7 |
| 8×2 | catalog | 72+24, solver 16 | 18.926 | +0.00% (0/10) | -38.69% (10/10) | 367.4 |
| 8×2 | catalog | 72+24, solver 20 | 18.810 | -1.14% (7/10) | -39.27% (10/10) | 355.7 |
| 8×2 | catalog | 72+24, solver 22 | 18.790 | -0.73% (7/10) | -39.28% (10/10) | 374.8 |
| 8×2 | catalog | 64+32, solver 20 | 18.957 | -0.13% (7/10) | -38.70% (10/10) | 369.5 |
| 8×2 | catalog | 64+32, solver 24 | 19.084 | +0.58% (0/10) | -38.37% (10/10) | 370.8 |
| 8×2 | catalog | 80+16, solver 12 | 19.545 | +3.20% (0/10) | -36.85% (10/10) | 362.0 |
| 8×2 | catalog | 80+16, solver 14 | 19.514 | +2.99% (0/10) | -36.88% (10/10) | 369.7 |
| 8×2 | catalog | Prepared Aeglos | 31.007 | +63.11% (0/10) | +0.00% (0/10) | 2494.7 |
| 8×2 | heldout | Pinned | 19.434 | +0.77% (2/10) | -38.18% (10/10) | 406.3 |
| 8×2 | heldout | 72+24, solver 12 | 19.566 | +1.86% (1/10) | -37.52% (10/10) | 363.5 |
| 8×2 | heldout | 72+24, solver 16 | 19.272 | +0.00% (0/10) | -38.62% (10/10) | 374.0 |
| 8×2 | heldout | 72+24, solver 20 | 18.977 | -1.53% (9/10) | -39.27% (10/10) | 373.4 |
| 8×2 | heldout | 72+24, solver 22 | 19.061 | -0.90% (8/10) | -39.13% (10/10) | 389.0 |
| 8×2 | heldout | 64+32, solver 20 | 19.143 | -0.31% (7/10) | -38.87% (10/10) | 362.9 |
| 8×2 | heldout | 64+32, solver 24 | 19.272 | +0.08% (4/10) | -38.58% (10/10) | 367.5 |
| 8×2 | heldout | 80+16, solver 12 | 19.820 | +2.44% (0/10) | -36.71% (10/10) | 356.1 |
| 8×2 | heldout | 80+16, solver 14 | 19.708 | +2.03% (0/10) | -36.93% (10/10) | 369.9 |
| 8×2 | heldout | Prepared Aeglos | 31.394 | +62.91% (0/10) | +0.00% (0/10) | 2511.1 |

Ten counterbalanced blocks prime three proofs and time seventeen. Independent verification passed for all 400 timing exports. The first invocation hit the SDK guard for a solver mask equal to the frontend mask. The retry used proper subsets, and the two preceding exports from the first invocation also verified. Both temporary instances and storage were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.864 | 37 / 24 | 4.70× / 3.05× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 2×2 | 11.325 | 59 / 30 | 5.21× / 2.65× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.810 | 189 / 74 | 10.05× / 3.93× | This panel |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five of eight catalog shapes reach the 10× CPU target. All eight Metal comparisons remain below 10×. Each linked panel records the executable and pool layout.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [First-run receipt](artifacts/rejected-layout.json) · [Cleanup](artifacts/cleanup.json)

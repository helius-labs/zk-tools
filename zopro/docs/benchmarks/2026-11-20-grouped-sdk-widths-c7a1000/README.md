# Grouped SDK worker widths and M5 reference check

The selected 8×2 profile took 18.804 ms on the catalog request and 19.117 ms on the held-out request. The 10× M5 CPU threshold is 18.9 ms and the Metal threshold is 7.4 ms. The catalog speedup was 10.05× CPU and 3.94× Metal.

This panel reuses the [validated tiled SDK](../2026-11-20-grouped-tiles-sdk-c7a995/README.md) on one 96-core c7a.24xlarge. The main and frontend workers use 64+32 or 72+24 physical cores. The solver uses eight or sixteen frontend cores. The fused arms also enable Euler leaf fusion and packed final verification. The pinned catalog control retains its prior worker placement. Aeglos uses all 96 physical cores.

| Shape | Request | Configuration | Median proof ms | Paired vs pinned | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Pinned | 14.182 | +0.00% (0/8) | -21.09% (8/8) | 213.2 |
| 4×2 | catalog | 64+32, solver 16 | 14.038 | -0.73% (6/8) | -21.65% (8/8) | 219.2 |
| 4×2 | catalog | 64+32, solver 8 | 14.213 | +0.69% (3/8) | -21.05% (8/8) | 207.2 |
| 4×2 | catalog | 72+24, solver 16 (selected) | 13.909 | -1.40% (6/8) | -22.49% (8/8) | 211.7 |
| 4×2 | catalog | 72+24, solver 8 | 14.125 | -0.40% (5/8) | -21.02% (8/8) | 210.9 |
| 4×2 | catalog | 64+32, solver 16, fused | 14.016 | -1.24% (7/8) | -21.57% (8/8) | 209.5 |
| 4×2 | catalog | 72+24, solver 16, fused | 13.943 | -1.74% (8/8) | -22.52% (8/8) | 211.0 |
| 4×2 | catalog | Prepared Aeglos | 17.951 | +26.74% (0/8) | +0.00% (0/8) | 1346.7 |
| 4×2 | heldout | Pinned | 14.173 | +0.00% (0/8) | -22.00% (8/8) | 210.3 |
| 4×2 | heldout | 64+32, solver 16 | 14.202 | +0.17% (4/8) | -22.27% (8/8) | 211.7 |
| 4×2 | heldout | 64+32, solver 8 | 14.265 | +0.72% (2/8) | -21.46% (8/8) | 219.2 |
| 4×2 | heldout | 72+24, solver 16 (selected) | 13.883 | -1.67% (7/8) | -23.37% (8/8) | 217.6 |
| 4×2 | heldout | 72+24, solver 8 | 14.037 | -0.81% (6/8) | -22.55% (8/8) | 214.7 |
| 4×2 | heldout | 64+32, solver 16, fused | 14.112 | -0.19% (7/8) | -22.37% (8/8) | 206.3 |
| 4×2 | heldout | 72+24, solver 16, fused | 13.878 | -2.00% (8/8) | -23.64% (8/8) | 208.1 |
| 4×2 | heldout | Prepared Aeglos | 18.151 | +28.21% (0/8) | +0.00% (0/8) | 1363.0 |
| 8×2 | catalog | Pinned | 19.190 | +0.00% (0/8) | -36.66% (8/8) | 382.2 |
| 8×2 | catalog | 64+32, solver 16 | 19.118 | +0.38% (4/8) | -36.55% (8/8) | 365.2 |
| 8×2 | catalog | 64+32, solver 8 | 20.147 | +5.23% (0/8) | -33.15% (8/8) | 360.4 |
| 8×2 | catalog | 72+24, solver 16 | 18.929 | -1.10% (7/8) | -37.29% (8/8) | 365.7 |
| 8×2 | catalog | 72+24, solver 8 | 19.995 | +4.57% (0/8) | -33.69% (8/8) | 379.1 |
| 8×2 | catalog | 64+32, solver 16, fused | 19.078 | -0.85% (5/8) | -36.65% (8/8) | 372.7 |
| 8×2 | catalog | 72+24, solver 16, fused (selected) | 18.804 | -1.76% (7/8) | -37.52% (8/8) | 356.8 |
| 8×2 | catalog | Prepared Aeglos | 30.116 | +57.88% (0/8) | +0.00% (0/8) | 2498.3 |
| 8×2 | heldout | Pinned | 19.293 | +0.00% (0/8) | -36.72% (8/8) | 398.1 |
| 8×2 | heldout | 64+32, solver 16 | 19.477 | +0.53% (2/8) | -36.43% (8/8) | 385.4 |
| 8×2 | heldout | 64+32, solver 8 | 20.479 | +5.95% (0/8) | -33.33% (8/8) | 366.4 |
| 8×2 | heldout | 72+24, solver 16 | 19.212 | +0.02% (3/8) | -37.31% (8/8) | 370.2 |
| 8×2 | heldout | 72+24, solver 8 | 20.259 | +5.07% (0/8) | -34.06% (8/8) | 376.4 |
| 8×2 | heldout | 64+32, solver 16, fused | 19.218 | +0.02% (4/8) | -37.38% (8/8) | 362.9 |
| 8×2 | heldout | 72+24, solver 16, fused (selected) | 19.117 | -1.10% (7/8) | -37.76% (8/8) | 363.6 |
| 8×2 | heldout | Prepared Aeglos | 30.657 | +58.05% (0/8) | +0.00% (0/8) | 2464.9 |

Each process primes three proofs and times seventeen. A candidate needs at least six paired wins and a lower paired median versus the pinned control on both requests. Among the qualifying profiles, selection minimizes the longer of the two request medians. The pinned control supplies the selection when all candidate profiles fall short. All 256 exports passed independent verification. The temporary AWS resources were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.864 | 37 / 24 | 4.70× / 3.05× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 2×2 | 11.325 | 59 / 30 | 5.21× / 2.65× | [Scoped G2 transfer](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md) |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | This panel |
| 8×2 | 18.804 | 189 / 74 | 10.05× / 3.94× | This panel |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

The shape selections across these matched C7a panels reach 10× for 5/8 CPU references and 0/8 Metal references. Each panel records its executable and worker layout.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

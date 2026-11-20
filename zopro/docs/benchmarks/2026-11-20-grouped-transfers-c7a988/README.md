# Grouped point-tile transfer on four more C7a shapes

The tiled profile beat the pinned catalog profile and prepared Aeglos in all eight paired blocks on both inputs for 3×2, 3×4, 4×4, and 4×8. Its paired median latency was 15.31–22.90% lower than Aeglos and 16.62–21.28% lower than the pinned Zopro profile. Peak whole-process RSS across the two inputs was 183.1–241.0 MiB for the tiled profile and 1,183.9–1,471.2 MiB for Aeglos.

The four circuit keys use the same inputs and initial profiles as the [full-catalog screen](../2026-11-20-all-shapes-c7a979/README.md). The newer option uses 64 main and 32 frontend workers with a 16-core solver subset. Its tiled mode enables direct point reduction for K and B2 on all four shapes, and for A on 4×4 and 4×8. The [local route checks](artifacts/local-routes.json) used 24 main and eight frontend hardware threads. The C7a panel tests the route under its matched worker placement.

| Shape | Input | Mode | Proof ms | Paired vs pinned profile | Paired vs newer | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 3×2 | catalog | Pinned catalog profile | 16.214 | — | +17.71% (0/8) | +0.73% (2/8) | 217.1 |
| 3×2 | catalog | Newer profile | 13.781 | -15.05% (8/8) | — | -13.73% (8/8) | 189.1 |
| 3×2 | catalog | Newer profile with tiles | 13.385 | -17.14% (8/8) | -2.94% (8/8) | -16.66% (8/8) | 183.1 |
| 3×2 | catalog | Prepared Aeglos | 16.011 | -0.72% (6/8) | +15.92% (0/8) | — | 1177.8 |
| 3×2 | heldout | Pinned catalog profile | 16.090 | — | +15.68% (0/8) | +1.14% (3/8) | 217.6 |
| 3×2 | heldout | Newer profile | 13.941 | -13.56% (8/8) | — | -12.51% (8/8) | 225.1 |
| 3×2 | heldout | Newer profile with tiles | 13.364 | -16.62% (8/8) | -3.26% (8/8) | -16.01% (8/8) | 181.7 |
| 3×2 | heldout | Prepared Aeglos | 15.968 | -1.12% (5/8) | +14.31% (0/8) | — | 1183.9 |
| 3×4 | catalog | Pinned catalog profile | 16.829 | — | +16.19% (0/8) | +1.40% (3/8) | 237.2 |
| 3×4 | catalog | Newer profile | 14.553 | -13.93% (8/8) | — | -11.98% (8/8) | 206.4 |
| 3×4 | catalog | Newer profile with tiles | 13.823 | -18.06% (8/8) | -5.18% (8/8) | -16.61% (8/8) | 213.1 |
| 3×4 | catalog | Prepared Aeglos | 16.543 | -1.38% (5/8) | +13.61% (0/8) | — | 1212.9 |
| 3×4 | heldout | Pinned catalog profile | 16.909 | — | +16.79% (0/8) | +3.19% (2/8) | 229.0 |
| 3×4 | heldout | Newer profile | 14.414 | -14.38% (8/8) | — | -11.99% (8/8) | 202.6 |
| 3×4 | heldout | Newer profile with tiles | 13.831 | -18.15% (8/8) | -4.43% (8/8) | -15.31% (8/8) | 195.1 |
| 3×4 | heldout | Prepared Aeglos | 16.361 | -3.09% (6/8) | +13.63% (0/8) | — | 1204.8 |
| 4×4 | catalog | Pinned catalog profile | 18.469 | — | +20.20% (0/8) | -1.98% (8/8) | 270.2 |
| 4×4 | catalog | Newer profile | 15.293 | -16.78% (8/8) | — | -19.71% (8/8) | 224.4 |
| 4×4 | catalog | Newer profile with tiles | 14.589 | -21.28% (8/8) | -4.28% (8/8) | -22.90% (8/8) | 216.2 |
| 4×4 | catalog | Prepared Aeglos | 18.798 | +2.02% (0/8) | +24.55% (0/8) | — | 1371.8 |
| 4×4 | heldout | Pinned catalog profile | 18.481 | — | +21.51% (0/8) | -2.25% (6/8) | 258.9 |
| 4×4 | heldout | Newer profile | 15.206 | -17.70% (8/8) | — | -19.02% (8/8) | 231.8 |
| 4×4 | heldout | Newer profile with tiles | 14.647 | -21.14% (8/8) | -3.70% (8/8) | -21.96% (8/8) | 218.4 |
| 4×4 | heldout | Prepared Aeglos | 18.741 | +2.30% (2/8) | +23.49% (0/8) | — | 1367.3 |
| 4×8 | catalog | Pinned catalog profile | 19.360 | — | +21.15% (0/8) | -1.49% (7/8) | 297.2 |
| 4×8 | catalog | Newer profile | 16.153 | -17.46% (8/8) | — | -17.82% (8/8) | 252.1 |
| 4×8 | catalog | Newer profile with tiles | 15.541 | -19.60% (8/8) | -4.42% (7/8) | -21.35% (8/8) | 241.0 |
| 4×8 | catalog | Prepared Aeglos | 19.638 | +1.51% (1/8) | +21.69% (0/8) | — | 1471.2 |
| 4×8 | heldout | Pinned catalog profile | 19.204 | — | +19.50% (0/8) | -1.61% (6/8) | 289.8 |
| 4×8 | heldout | Newer profile | 16.048 | -16.32% (8/8) | — | -17.90% (8/8) | 246.8 |
| 4×8 | heldout | Newer profile with tiles | 15.519 | -19.15% (8/8) | -3.33% (8/8) | -20.28% (8/8) | 236.4 |
| 4×8 | heldout | Prepared Aeglos | 19.484 | +1.64% (2/8) | +21.80% (0/8) | — | 1461.1 |

| Shape | Input | Mode | Solve ms | Gather ms | Quotient ms | B2 ms |
|---|---|---|---:|---:|---:|---:|
| 3×2 | catalog | Newer profile | 2.524 | 0.977 | 2.035 | 6.671 |
| 3×2 | catalog | Newer profile with tiles | 2.636 | 1.023 | 2.075 | 5.860 |
| 3×2 | heldout | Newer profile | 2.599 | 0.943 | 2.038 | 6.382 |
| 3×2 | heldout | Newer profile with tiles | 2.578 | 0.954 | 2.071 | 6.003 |
| 3×4 | catalog | Newer profile | 2.533 | 1.020 | 2.026 | 6.987 |
| 3×4 | catalog | Newer profile with tiles | 2.612 | 0.999 | 2.017 | 6.080 |
| 3×4 | heldout | Newer profile | 2.547 | 0.950 | 2.027 | 6.926 |
| 3×4 | heldout | Newer profile with tiles | 2.575 | 1.016 | 2.049 | 6.102 |
| 4×4 | catalog | Newer profile | 2.705 | 1.098 | 2.070 | 7.633 |
| 4×4 | catalog | Newer profile with tiles | 2.728 | 1.097 | 2.086 | 7.107 |
| 4×4 | heldout | Newer profile | 2.687 | 1.075 | 2.080 | 7.533 |
| 4×4 | heldout | Newer profile with tiles | 2.754 | 1.112 | 2.094 | 6.953 |
| 4×8 | catalog | Newer profile | 2.717 | 1.108 | 2.108 | 8.423 |
| 4×8 | catalog | Newer profile with tiles | 2.766 | 1.161 | 2.077 | 7.576 |
| 4×8 | heldout | Newer profile | 2.847 | 1.088 | 2.050 | 8.648 |
| 4×8 | heldout | Newer profile with tiles | 2.820 | 1.194 | 2.100 | 7.553 |

Eight paired blocks per input prime three proofs and time seventeen. The separate clock panel has three blocks and shifts paired timing by up to 5.09%. Aeglos used all 96 physical cores with a prepared-key hit on every timed proof. Route probes checked active scalar memberships and curve points and selected the ordinary path when optional scratch was denied. All 320 C7a and 36 local proof exports independently verified. The temporary instance and storage were removed.

These shapes used a separate C7a instance from the full-catalog run and the previous [three-shape confirmation](../2026-11-20-medium-profile-transfer-c7a985/README.md). A full rerun with a consolidated SDK remains pending.

[Per-mode results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

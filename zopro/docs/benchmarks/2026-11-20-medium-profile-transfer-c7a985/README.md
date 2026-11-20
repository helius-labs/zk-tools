# Newer grouped proving profiles on 96-core C7a

The newer profile with tiles won all eight paired blocks against the pinned catalog profile and prepared Aeglos on both inputs for 2×16, 3×8, and 4×16. Paired median latency dropped by 18.76–22.70% versus the pinned Zopro profile and 10.45–16.79% versus Aeglos. Across the three shapes, peak process RSS was 220.7–281.6 MiB for the tiled profile and 1,235.5–1,609.1 MiB for Aeglos, taking the higher peak across the two inputs for each shape.

The three circuit keys use the same inputs and initial profiles as the [full-catalog screen](../2026-11-20-all-shapes-c7a979/README.md). The newer option uses 64 main and 32 frontend workers with a 16-core solver subset. Its tiled mode enables direct point reduction for K and B2 on all three shapes, and for A on 4×16. The earlier [Ryzen preflight](../2026-11-20-medium-profile-transfer-ryzen984/README.md) remapped every mode to 24 main and eight frontend hardware threads.

| Shape | Input | Mode | Proof ms | Paired vs pinned profile | Paired vs newer | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 2×16 | catalog | Pinned catalog profile | 19.568 | — | +16.56% (0/8) | +17.06% (0/8) | 301.7 |
| 2×16 | catalog | Newer profile | 16.876 | -14.21% (8/8) | — | +0.28% (4/8) | 248.4 |
| 2×16 | catalog | Newer profile with tiles | 15.164 | -22.70% (8/8) | -9.86% (8/8) | -10.45% (8/8) | 238.6 |
| 2×16 | catalog | Prepared Aeglos | 16.729 | -14.57% (8/8) | -0.28% (4/8) | — | 1227.3 |
| 2×16 | heldout | Pinned catalog profile | 19.483 | — | +16.34% (0/8) | +15.46% (0/8) | 288.3 |
| 2×16 | heldout | Newer profile | 16.717 | -14.05% (8/8) | — | -0.59% (5/8) | 249.1 |
| 2×16 | heldout | Newer profile with tiles | 14.890 | -22.70% (8/8) | -10.44% (8/8) | -11.16% (8/8) | 241.6 |
| 2×16 | heldout | Prepared Aeglos | 16.780 | -13.39% (8/8) | +0.60% (3/8) | — | 1235.5 |
| 3×8 | catalog | Pinned catalog profile | 18.075 | — | +16.55% (0/8) | +4.54% (1/8) | 264.2 |
| 3×8 | catalog | Newer profile | 15.499 | -14.19% (8/8) | — | -10.36% (8/8) | 228.1 |
| 3×8 | catalog | Newer profile with tiles | 14.726 | -18.76% (8/8) | -5.40% (8/8) | -14.47% (8/8) | 218.5 |
| 3×8 | catalog | Prepared Aeglos | 17.347 | -4.33% (7/8) | +11.55% (0/8) | — | 1284.4 |
| 3×8 | heldout | Pinned catalog profile | 18.179 | — | +17.75% (0/8) | +5.70% (0/8) | 266.4 |
| 3×8 | heldout | Newer profile | 15.342 | -15.07% (8/8) | — | -11.23% (8/8) | 234.1 |
| 3×8 | heldout | Newer profile with tiles | 14.554 | -19.92% (8/8) | -5.29% (8/8) | -16.11% (8/8) | 220.7 |
| 3×8 | heldout | Prepared Aeglos | 17.251 | -5.39% (8/8) | +12.66% (0/8) | — | 1280.2 |
| 4×16 | catalog | Pinned catalog profile | 22.319 | — | +20.27% (0/8) | +6.34% (0/8) | 341.5 |
| 4×16 | catalog | Newer profile | 18.486 | -16.85% (8/8) | — | -12.19% (8/8) | 289.8 |
| 4×16 | catalog | Newer profile with tiles | 17.551 | -21.26% (8/8) | -5.03% (8/8) | -16.65% (8/8) | 263.4 |
| 4×16 | catalog | Prepared Aeglos | 20.990 | -5.95% (8/8) | +13.89% (0/8) | — | 1609.1 |
| 4×16 | heldout | Pinned catalog profile | 22.088 | — | +19.72% (0/8) | +5.56% (0/8) | 351.1 |
| 4×16 | heldout | Newer profile | 18.522 | -16.47% (8/8) | — | -10.70% (8/8) | 285.1 |
| 4×16 | heldout | Newer profile with tiles | 17.466 | -20.98% (8/8) | -5.83% (8/8) | -16.79% (8/8) | 281.6 |
| 4×16 | heldout | Prepared Aeglos | 20.969 | -5.26% (8/8) | +11.98% (0/8) | — | 1591.4 |

| Shape | Input | Mode | Solve ms | Gather ms | Quotient ms | B2 ms |
|---|---|---|---:|---:|---:|---:|
| 2×16 | catalog | Newer profile | 3.037 | 1.017 | 2.067 | 8.875 |
| 2×16 | catalog | Newer profile with tiles | 3.108 | 1.064 | 2.043 | 7.233 |
| 2×16 | heldout | Newer profile | 2.896 | 1.026 | 2.048 | 9.089 |
| 2×16 | heldout | Newer profile with tiles | 3.057 | 1.058 | 2.053 | 7.150 |
| 3×8 | catalog | Newer profile | 2.560 | 1.038 | 2.050 | 8.143 |
| 3×8 | catalog | Newer profile with tiles | 2.628 | 1.099 | 2.072 | 6.967 |
| 3×8 | heldout | Newer profile | 2.609 | 1.035 | 2.070 | 7.739 |
| 3×8 | heldout | Newer profile with tiles | 2.693 | 1.109 | 2.051 | 6.480 |
| 4×16 | catalog | Newer profile | 3.013 | 1.293 | 2.096 | 9.618 |
| 4×16 | catalog | Newer profile with tiles | 3.085 | 1.319 | 2.086 | 9.116 |
| 4×16 | heldout | Newer profile | 3.001 | 1.311 | 2.091 | 9.873 |
| 4×16 | heldout | Newer profile with tiles | 3.033 | 1.355 | 2.114 | 8.852 |

Eight paired blocks per input prime three proofs and time seventeen. The separate clock panel has three blocks and shifts paired timing by up to 3.53%. Aeglos used all 96 physical cores with a prepared-key hit on every timed proof. Route probes checked active scalar memberships and curve points and selected the ordinary path when optional scratch was denied. All 240 proof exports independently verified. The temporary instance and storage were removed.

These three shapes were tested on a separate C7a instance from the full-catalog screen. The results qualify the research profile for these keys. A consolidated SDK and full-catalog rerun are still pending.

[Per-mode results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

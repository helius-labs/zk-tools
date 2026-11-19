# Combining G2 worker placement and window geometry

Across the full 41-shape catalog, the latest frozen Zopro profiles beat prepared Aeglos on both transactions for **29/41** shapes and in every pair on both inputs for **27/41**. Nine grouped shapes use this fresh six-block confirmation. The other 32 retain their [earlier two-input results](../2026-11-19-equal-core-scorecard707/README.md). For the nine grouped shapes alone, 4/9 led on both medians and 3/9 swept both inputs. The [full 41-shape CSV](artifacts/full-catalog.csv) gives both input medians, paired wins, and peak RSS for each prover. Zopro used less maximum peak RSS across the two inputs on 41/41 shapes and reached half the earlier catalog release latency on 0/41.

The earlier 96-core screen measured two interventions separately. This trial chooses the faster dedicated split and the faster grouped window geometry from that screen for each grouped shape and combines them. The chosen pool reserves the full workspace for its actual geometry. Sixteen local proof exports checked all three geometries and the live zero-budget fallback on two keys and two inputs. On the C7a, a separate probe checked the actual geometry for each key. Two new catalog screening blocks froze the combined profile only when it beat the earlier Zopro profile in both blocks. Six fresh blocks per transaction then ran the combined profile, its same-binary control with the former two-window-partition geometry, earlier Zopro and prepared Aeglos. Both engines had the same 96 physical cores. All 477 AWS exports independently reverified.

| Shape | Request | Split / geometry | Frozen profile | Combined / off / earlier / Aeglos ms | Combined vs off | Frozen vs Aeglos | Peak MiB frozen / Aeglos |
|---|---|---|---|---:|---:|---:|---:|
| 2x16 | catalog | split56 / 11x4 | previous | 19.715 / 19.948 / 19.791 / 16.981 | -0.99% (5/6) | +15.72% (0/6) | 251.5 / 1231.9 |
| 2x16 | heldout | split56 / 11x4 | previous | 19.932 / 19.529 / 19.439 / 16.836 | +0.93% (2/6) | +15.33% (0/6) | 262.7 / 1213.1 |
| 3x2 | catalog | split56 / 11x4 | previous | 16.461 / 15.942 / 15.924 / 16.119 | +2.74% (0/6) | -1.70% (5/6) | 199.0 / 1158.4 |
| 3x2 | heldout | split56 / 11x4 | previous | 16.504 / 16.197 / 16.227 / 16.121 | +1.55% (1/6) | -0.63% (4/6) | 199.2 / 1160.1 |
| 3x4 | catalog | split56 / 11x4 | previous | 17.089 / 16.992 / 16.839 / 16.638 | +0.29% (2/6) | +0.04% (3/6) | 204.3 / 1192.9 |
| 3x4 | heldout | split56 / 11x4 | previous | 17.002 / 16.567 / 16.961 / 16.642 | -0.22% (4/6) | +1.28% (1/6) | 209.4 / 1209.6 |
| 3x8 | catalog | split56 / 11x4 | previous | 18.644 / 18.387 / 17.766 / 17.342 | +1.34% (2/6) | +2.21% (0/6) | 230.4 / 1276.1 |
| 3x8 | heldout | split56 / 11x4 | previous | 18.390 / 18.109 / 17.993 / 17.434 | +1.56% (1/6) | +2.73% (1/6) | 243.9 / 1265.9 |
| 4x16 | catalog | split56 / 11x4 | previous | 22.553 / 21.886 / 22.102 / 21.343 | +3.28% (1/6) | +3.06% (1/6) | 313.1 / 1585.5 |
| 4x16 | heldout | split56 / 11x4 | previous | 22.391 / 22.275 / 22.131 / 21.473 | +1.11% (2/6) | +2.60% (1/6) | 311.5 / 1588.4 |
| 4x2 | catalog | split56 / 11x4 | previous | 17.860 / 17.402 / 17.210 / 18.694 | +2.98% (1/6) | -7.55% (6/6) | 228.3 / 1344.8 |
| 4x2 | heldout | split56 / 11x4 | previous | 17.971 / 17.661 / 17.604 / 18.216 | +0.84% (3/6) | -4.41% (6/6) | 224.5 / 1346.0 |
| 4x4 | catalog | split64 / 11x4 | previous | 19.006 / 18.258 / 18.111 / 18.922 | +3.63% (0/6) | -4.90% (6/6) | 237.1 / 1368.0 |
| 4x4 | heldout | split64 / 11x4 | previous | 18.563 / 18.388 / 18.453 / 19.087 | +0.86% (2/6) | -3.55% (6/6) | 223.6 / 1361.4 |
| 4x8 | catalog | split56 / 11x4 | previous | 19.713 / 19.244 / 19.603 / 20.004 | +2.30% (1/6) | -1.88% (4/6) | 267.9 / 1460.1 |
| 4x8 | heldout | split56 / 11x4 | previous | 19.941 / 19.373 / 19.393 / 19.933 | +2.74% (2/6) | -2.76% (6/6) | 262.8 / 1458.8 |
| 5x4 | catalog | split64 / 11x4 | previous | 20.480 / 19.627 / 19.853 / 20.974 | +4.48% (0/6) | -6.14% (6/6) | 274.7 / 1576.2 |
| 5x4 | heldout | split64 / 11x4 | previous | 20.578 / 20.286 / 19.879 / 21.366 | +1.56% (2/6) | -7.13% (6/6) | 267.9 / 1573.4 |

## Shapes still above Aeglos on either transaction

| Shape | Higher relative loss | Zopro / Aeglos ms | Deficit ms |
|---|---|---:|---:|
| 1x2 | heldout | 11.035 / 8.292 | 2.743 |
| 1x8 | heldout | 13.229 / 10.497 | 2.732 |
| 1x16 | catalog | 15.597 / 12.443 | 3.154 |
| 1x4 | heldout | 11.782 / 9.615 | 2.167 |
| 2x16 | catalog | 19.791 / 16.981 | 2.810 |
| 2x4 | heldout | 13.979 / 12.294 | 1.685 |
| 2x2 | catalog | 12.513 / 11.799 | 0.714 |
| 4x16 | catalog | 22.102 / 21.343 | 0.759 |
| 3x8 | heldout | 17.993 / 17.434 | 0.559 |
| 2x8 | heldout | 15.542 / 15.203 | 0.338 |
| 3x4 | heldout | 16.961 / 16.642 | 0.319 |
| 3x2 | heldout | 16.227 / 16.121 | 0.106 |

[Fresh confirmation](artifacts/combo714b-rows.json.gz) · [Initial choices](artifacts/source-choice.json) · [All-shape status](artifacts/catalog-status.json) · [Verification](artifacts/verification.json) · [Implementation](artifacts/source.patch)

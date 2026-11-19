# G2 partitions and dedicated worker pools on unresolved C7a shapes

The experiment included 6 unresolved small-query shapes with locally verified dispatch and 9 additional grouped-query shapes. Together these cover every catalog shape that had a loss or mixed result in the eight-block 96-core confirmation. The grouped keys were ordered by their Zopro-to-Aeglos latency ratio. Four new 96-core screening blocks compared the available geometries, their same-binary control, the previously selected Zopro configuration and prepared Aeglos. On 6 eligible small-query shapes, additional configurations split the 96 physical cores between the main and frontend pools (64+32 or 56+40, with intermediate 60+36 and near-even 52+44 splits on eligible 1×4, 1×8 and 1×16). The experimental path sends the exclusive G2 query to the frontend pool during curve work. On 9 larger shapes the same disjoint layouts moved only the final G2 windows after grouping. Where a Ryzen layout with an equal or wider G2 pool beat 10+6 in at least three of four pairs and on median for every tested input of that shape, its corresponding 48+48 or 40+56 layout was also eligible. These layouts run the solver and FFT on the main workers. On 0 eligible grouped-query shapes, a further comparison placed A or K alone on 40 auxiliary cores. Eligibility required local paired gains over the idle and integrated controls, including at least three of four wins against integrated on all tested transactions. On 0 shapes, an additional same-binary comparison classified exact B2 scalars in parallel shards. It advanced a layout from Ryzen only when enabled beat both off and integrated on paired median for every tested input of that key. The small-query classification prototype shared an entry point with G1. Its first route audit expected a single query per proof, so its timings were unavailable for this screen. The separate curve-specific trial compares both scopes. A matching idle control reserves exactly the same cores. CPU masks are checked for both pools in every split run. The first screening block also records G2 dispatch, and all six confirmation blocks run with diagnostics off. The lowest median among the new configurations for each shape advanced. Six fresh blocks tested it against the two existing engines on both the catalog input and a second independently generated transaction. Each process in confirmation used three warmups and thirteen measured proofs. All 1176 exports passed remote and independent local verification. Peak RSS includes preparation.

| Shape | Transaction | Selected approach | Candidate / previous Zopro / Aeglos ms | vs previous (wins) | vs Aeglos (wins) | Peak MiB candidate / previous / Aeglos |
|---|---|---|---:|---:|---:|---:|
| 1x16 | catalog | split48 | 15.597 / 16.492 / 12.443 | -6.22% (6/6) | +25.96% (0/6) | 153.2 / 108.8 / 820.0 |
| 1x16 | heldout | split48 | 15.610 / 16.748 / 12.652 | -6.73% (6/6) | +23.85% (0/6) | 150.9 / 110.4 / 824.0 |
| 1x4 | catalog | split48 | 12.594 / 11.791 / 9.670 | +6.36% (0/6) | +31.21% (0/6) | 88.7 / 116.4 / 585.3 |
| 1x4 | heldout | split48 | 12.505 / 11.782 / 9.615 | +6.84% (0/6) | +31.73% (0/6) | 96.2 / 114.9 / 582.8 |
| 1x8 | catalog | split48 | 13.170 / 13.548 / 10.858 | -2.39% (5/6) | +22.27% (0/6) | 118.1 / 81.2 / 691.3 |
| 1x8 | heldout | split48 | 13.229 / 13.165 / 10.497 | -0.23% (4/6) | +23.97% (0/6) | 117.3 / 78.2 / 686.5 |
| 2x16 | catalog | split56 | 19.762 / 21.882 / 17.186 | -8.64% (6/6) | +15.02% (0/6) | 260.4 / 264.9 / 1249.4 |
| 2x16 | heldout | split56 | 19.512 / 22.031 / 17.119 | -10.99% (6/6) | +13.43% (0/6) | 276.9 / 252.1 / 1229.1 |
| 2x2 | catalog | split56 | 13.614 / 12.513 / 11.799 | +8.12% (0/6) | +16.05% (0/6) | 81.3 / 134.5 / 811.9 |
| 2x2 | heldout | split56 | 13.654 / 12.473 / 11.945 | +8.98% (0/6) | +14.78% (0/6) | 81.9 / 133.7 / 805.2 |
| 2x4 | catalog | split56 | 14.582 / 13.948 / 12.395 | +3.61% (0/6) | +17.20% (0/6) | 84.2 / 138.9 / 820.0 |
| 2x4 | heldout | split56 | 14.371 / 13.979 / 12.294 | +2.71% (0/6) | +17.19% (0/6) | 87.2 / 141.2 / 828.2 |
| 2x8 | catalog | split64 | 16.924 / 15.485 / 15.465 | +8.70% (0/6) | +9.30% (0/6) | 113.4 / 130.7 / 1026.8 |
| 2x8 | heldout | split64 | 16.762 / 15.542 / 15.203 | +7.28% (0/6) | +9.18% (0/6) | 120.1 / 126.2 / 1017.3 |
| 3x2 | catalog | split56 | 16.143 / 17.567 / 16.261 | -8.39% (6/6) | -1.46% (5/6) | 202.0 / 190.6 / 1162.7 |
| 3x2 | heldout | split56 | 16.160 / 17.592 / 16.103 | -9.02% (6/6) | -0.43% (3/6) | 210.2 / 190.1 / 1161.5 |
| 3x4 | catalog | split56 | 16.928 / 18.014 / 16.756 | -5.79% (6/6) | +0.24% (2/6) | 219.1 / 213.3 / 1216.9 |
| 3x4 | heldout | split56 | 16.499 / 18.082 / 16.733 | -8.65% (6/6) | -0.58% (5/6) | 206.4 / 208.7 / 1206.1 |
| 3x8 | catalog | split56 | 18.109 / 19.836 / 17.452 | -8.82% (6/6) | +3.83% (0/6) | 249.2 / 244.0 / 1277.6 |
| 3x8 | heldout | split56 | 17.897 / 19.755 / 17.721 | -8.99% (6/6) | +1.03% (2/6) | 253.1 / 244.0 / 1270.0 |
| 4x16 | catalog | split56 | 22.174 / 23.499 / 21.411 | -5.06% (5/6) | +3.61% (1/6) | 321.2 / 328.0 / 1585.7 |
| 4x16 | heldout | split56 | 22.112 / 23.598 / 21.427 | -6.29% (6/6) | +2.86% (1/6) | 329.4 / 322.0 / 1585.3 |
| 4x2 | catalog | split56 | 17.333 / 18.730 / 18.318 | -6.36% (6/6) | -5.40% (6/6) | 232.8 / 225.9 / 1347.6 |
| 4x2 | heldout | split56 | 17.653 / 18.877 / 18.722 | -6.55% (6/6) | -5.82% (6/6) | 239.6 / 219.2 / 1350.5 |
| 4x4 | catalog | split64 | 18.535 / 19.269 / 19.027 | -3.91% (6/6) | -2.59% (6/6) | 251.3 / 243.1 / 1389.6 |
| 4x4 | heldout | split64 | 18.520 / 19.020 / 18.939 | -2.56% (5/6) | -2.12% (5/6) | 240.9 / 225.9 / 1366.6 |
| 4x8 | catalog | split56 | 19.339 / 21.224 / 19.753 | -8.88% (6/6) | -1.47% (6/6) | 271.2 / 271.9 / 1460.6 |
| 4x8 | heldout | split56 | 19.572 / 21.235 / 19.646 | -7.83% (6/6) | -1.15% (4/6) | 274.6 / 269.4 / 1459.4 |
| 5x4 | catalog | split64 | 19.906 / 20.916 / 21.015 | -4.40% (6/6) | -4.44% (6/6) | 283.6 / 273.2 / 1562.2 |
| 5x4 | heldout | split64 | 19.863 / 21.168 / 21.144 | -5.53% (6/6) | -7.55% (6/6) | 285.9 / 274.7 / 1595.6 |

[Implementation and memory qualification](../2026-11-19-g2-width702/README.md) · [Small-query placement](../2026-11-19-g2-frontend705/README.md) · [Equal and wider G2 pools](../2026-11-19-equal-pools708/README.md) · [Canonical B2 classes](../2026-11-19-small-b2-classes711/README.md) · [Exact B2 scalar classes](../2026-11-19-shard-b2-classes710/README.md) · [Individual G1 queries](../2026-11-19-group-g1-pool709/README.md) · [Grouped placement](../2026-11-19-group-frontend706/README.md) · [Grouped-query implementation](../2026-11-19-group-width704/README.md) · [Original full catalog confirmation](../2026-11-19-loss-routes689/README.md) · [Split vs idle](artifacts/placement.json) · [Screen](artifacts/tile-screen-rows.json.gz) · [Fresh confirmation](artifacts/tile-confirm-rows.json.gz) · [Verification](artifacts/verification.json)

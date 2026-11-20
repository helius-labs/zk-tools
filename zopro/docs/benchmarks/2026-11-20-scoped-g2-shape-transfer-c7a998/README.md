# Scoped G2 shape and placement transfer on 96-core C7a

Scoped G2 achieved paired gains against both Zopro controls on 2×2 (equal and narrow), 1×4 (equal and narrow), 2×4 (equal and narrow), 2×8 (equal and narrow). The early schedule beat prepared Aeglos on 1×2 (equal), 2×2 (equal), 2×4 (narrow), 2×8 (equal and narrow).

The [scoped G2 candidate](../2026-11-20-scoped-g2-narrow-frontend-ryzen997/README.md) starts G2 on a separate fixed worker pool while the main workers compute the quotient, then joins that pool after G1. The two layouts use 48+48 cores and a narrower frontend. The narrower layout uses 80+16 for 1×2 and 64+32 elsewhere. Each layout has its own same-build late control. The panel uses the exact catalog and held-out requests, pinned Zopro controls and prepared Aeglos on one c7a.24xlarge.

| Shape | Request | Pinned / Aeglos ms | 48+48 late / early ms | Early vs late | Narrow late / early ms | Early vs late | Peak early equal / narrow / Aeglos MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.864 / 8.361 | 9.099 / 7.921 | -12.35% (8/8) | 10.239 / 9.296 | -9.02% (8/8) | 53.5 / 44.6 / 460.7 |
| 1×2 | heldout | 7.829 / 8.320 | 8.945 / 7.946 | -11.21% (8/8) | 10.183 / 9.365 | -8.56% (8/8) | 51.2 / 40.8 / 460.1 |
| 2×2 | catalog | 12.027 / 11.702 | 12.847 / 11.325 | -12.14% (8/8) | 12.509 / 11.758 | -6.02% (8/8) | 85.8 / 146.4 / 810.4 |
| 2×2 | heldout | 12.094 / 11.621 | 12.895 / 11.435 | -11.23% (8/8) | 12.584 / 11.799 | -6.21% (8/8) | 80.6 / 148.0 / 804.6 |
| 1×4 | catalog | 11.794 / 9.684 | 12.439 / 10.498 | -15.90% (8/8) | 11.841 / 10.035 | -15.19% (8/8) | 106.7 / 157.6 / 585.3 |
| 1×4 | heldout | 11.876 / 9.562 | 12.388 / 10.558 | -14.75% (8/8) | 11.716 / 10.193 | -13.32% (8/8) | 108.9 / 152.4 / 586.0 |
| 2×4 | catalog | 13.987 / 12.075 | 14.193 / 12.173 | -14.34% (8/8) | 13.615 / 11.370 | -16.71% (8/8) | 140.5 / 189.9 / 825.4 |
| 2×4 | heldout | 13.965 / 12.183 | 13.932 / 12.038 | -13.53% (8/8) | 13.485 / 11.378 | -15.56% (8/8) | 137.5 / 192.2 / 827.7 |
| 2×8 | catalog | 15.613 / 14.784 | 15.802 / 13.321 | -15.63% (8/8) | 16.485 / 13.674 | -17.22% (8/8) | 181.9 / 245.4 / 1025.5 |
| 2×8 | heldout | 15.672 / 14.930 | 15.819 / 13.142 | -16.97% (8/8) | 16.505 / 13.797 | -16.98% (8/8) | 175.7 / 241.7 / 1031.3 |

| Shape | Request | Pinned CPU / Metal | Equal early CPU / Metal | Narrow early CPU / Metal |
|---|---|---:|---:|---:|
| 1×2 | catalog | 4.70× / 3.05× | 4.67× / 3.03× | 3.98× / 2.58× |
| 1×2 | heldout | 4.73× / 3.07× | 4.66× / 3.02× | 3.95× / 2.56× |
| 2×2 | catalog | 4.91× / 2.49× | 5.21× / 2.65× | 5.02× / 2.55× |
| 2×2 | heldout | 4.88× / 2.48× | 5.16× / 2.62× | 5.00× / 2.54× |

Eight paired timing blocks prime three proofs and time seventeen. The forty route checks exercise early dispatch and forced workspace denial at both widths. All 520 exports passed independent verification. The temporary AWS resources were removed.

[Full paired comparisons and stage data](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

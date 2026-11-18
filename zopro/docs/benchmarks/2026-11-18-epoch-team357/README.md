# Solver epoch and interpreter width on C7a

On the prespecified 16×2 comparison at 64 physical cores, four epochs took 35.699 ms and eight took 34.291 ms. The paired change was +3.57% with 0/5 faster blocks. Four epochs met the 33.5 ms CPU threshold in 0/5 blocks.

## Fixed-width comparisons on every reference shape

The integrated release and benchmark options are fixed within each shape. All eight rows report four epochs in the comparison with M5. The neighboring columns show eight and sixteen epochs from the same host. The scorecard uses five randomized processes per setting. The 1×2 processes time 41 SDK requests each, and the other processes time eleven, after three primes.

| Shape | Physical cores | Four epochs ms | Eight ms | Sixteen ms | Peak MiB, four | M5 CPU / Metal ms | Four-epoch CPU / Metal speedup | Blocks at 10× CPU, four |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 8.210 | 8.299 | 8.545 | 51 | 37 / 24 | 4.51× / 2.92× | 0/5 |
| 2x2 | 48 | 13.435 | 13.777 | 14.341 | 92 | 59 / 30 | 4.39× / 2.23× | 0/5 |
| 4x2 | 48 | 17.220 | 17.180 | 18.290 | 240 | 109 / 48 | 6.33× / 2.79× | 0/5 |
| 8x2 | 48 | 24.238 | 24.151 | 24.812 | 426 | 189 / 74 | 7.80× / 3.05× | 0/5 |
| 12x2 | 64 | 31.649 | 31.440 | 32.210 | 599 | 284 / 101 | 8.97× / 3.19× | 0/5 |
| 16x2 | 64 | 35.699 | 34.291 | 34.896 | 756 | 335 / 124 | 9.38× / 3.47× | 0/5 |
| 24x2 | 64 | 48.170 | 45.693 | 46.954 | 1159 | 528 / 176 | 10.96× / 3.65× | 5/5 |
| merge-24x1 | 64 | 50.100 | 49.852 | 51.086 | 1037 | 507 / 175 | 10.12× / 3.49× | 5/5 |

## Shape-specific exploration

Every candidate here uses the same integrated executable. Selection of the lowest observed median across these cells is exploratory.

| Shape | Cores | Epochs | Wall ms | Aggregate CPU-ms/proof | Peak MiB | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 48 | 4 | 31.824 | 1197.3 | 601 | 0/5 |
| 12x2 | 48 | 8 | 30.925 | 1218.0 | 605 | 0/5 |
| 12x2 | 56 | 4 | 31.569 | 1334.2 | 595 | 0/5 |
| 12x2 | 56 | 8 | 31.980 | 1405.1 | 591 | 0/5 |
| 12x2 | 64 | 16 | 32.210 | 1599.4 | 608 | 0/5 |
| 12x2 | 64 | 4 | 31.649 | 1469.0 | 599 | 0/5 |
| 12x2 | 64 | 8 | 31.440 | 1530.2 | 607 | 0/5 |
| 16x2 | 48 | 12 | 35.022 | 1410.6 | 717 | 0/5 |
| 16x2 | 48 | 2 | 37.377 | 1343.0 | 720 | 0/5 |
| 16x2 | 48 | 4 | 36.010 | 1359.1 | 699 | 0/5 |
| 16x2 | 48 | 6 | 34.296 | 1369.3 | 703 | 0/5 |
| 16x2 | 48 | 8 | 35.155 | 1394.6 | 727 | 0/5 |
| 16x2 | 56 | 12 | 34.830 | 1561.4 | 717 | 0/5 |
| 16x2 | 56 | 2 | 38.229 | 1515.4 | 724 | 0/5 |
| 16x2 | 56 | 4 | 35.828 | 1517.5 | 705 | 0/5 |
| 16x2 | 56 | 6 | 34.571 | 1524.6 | 738 | 0/5 |
| 16x2 | 56 | 8 | 34.624 | 1545.8 | 740 | 0/5 |
| 16x2 | 64 | 12 | 34.922 | 1706.0 | 725 | 0/5 |
| 16x2 | 64 | 16 | 34.896 | 1748.6 | 723 | 0/5 |
| 16x2 | 64 | 2 | 37.364 | 1632.4 | 706 | 0/5 |
| 16x2 | 64 | 4 | 35.699 | 1654.2 | 756 | 0/5 |
| 16x2 | 64 | 6 | 33.759 | 1656.0 | 736 | 2/5 |
| 16x2 | 64 | 8 | 34.291 | 1691.0 | 717 | 0/5 |
| 16x2 | 96 | 12 | 37.813 | 2607.3 | 735 | 0/5 |
| 16x2 | 96 | 2 | 38.950 | 2347.6 | 735 | 0/5 |
| 16x2 | 96 | 4 | 38.364 | 2407.2 | 712 | 0/5 |
| 16x2 | 96 | 6 | 37.139 | 2478.3 | 716 | 0/5 |
| 16x2 | 96 | 8 | 37.286 | 2528.9 | 729 | 0/5 |
| merge-24x1 | 48 | 4 | 51.127 | 1901.4 | 1067 | 1/5 |
| merge-24x1 | 48 | 8 | 50.597 | 1942.5 | 1009 | 3/5 |
| merge-24x1 | 56 | 4 | 49.499 | 2061.7 | 1072 | 5/5 |
| merge-24x1 | 56 | 8 | 48.494 | 2106.6 | 1052 | 5/5 |
| merge-24x1 | 64 | 16 | 51.086 | 2430.9 | 1047 | 2/5 |
| merge-24x1 | 64 | 4 | 50.100 | 2250.6 | 1037 | 5/5 |
| merge-24x1 | 64 | 8 | 49.852 | 2326.1 | 1050 | 5/5 |
| merge-24x1 | 96 | 8 | 52.962 | 3407.1 | 1030 | 0/5 |

All 467 process exports passed independent verification on AWS and again after retrieval. The instance terminated, its tagged volume count returned to zero and the temporary bucket was removed.

[Fresh holdout results](holdout.md) · [Interpreter worker comparison](team.md) · [Protocol](artifacts/protocol.md) · [Request timings](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

[96-core interpreter confirmation](confirmation.md)

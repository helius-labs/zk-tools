# Warm proof latency on 96 C7a cores, by circuit shape

Zopro had lower median latency than prepared Aeglos on both tested transactions for **29/41** shapes. It won every paired block on both inputs for **26/41**. Zopro led on 31/41 catalog transactions and 30/41 second transactions. Its maximum whole-process RSS across the two transactions was lower on 41/41 shapes. The selected catalog median at least halved the earlier Zopro catalog release median on 0/41 shapes on this host.

Both engines had access to the same 96 physical cores of a c7a.24xlarge. Each row uses one configuration frozen from catalog screening, as specified in the [selection protocol](protocol.md). Fresh confirmation on both inputs gave each configuration the same paired comparison against Aeglos. Every one of the 984 scored process exports belongs to the independently reverified proof ledger. Peak RSS includes setup. Values are medians of warm process means, in Zopro / Aeglos order. The per-shape data also gives warm CPU-ms, average busy cores and minor faults per proof. Busy cores use total user plus system CPU time divided by elapsed wall time in each timed loop, including worker spin. These are research profiles with the SDK identified in the machine-readable results.

| Shape | Profile | Catalog ms | Zopro wins | Second input ms | Zopro wins | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | pair-on | 8.001 / 8.384 | 8/8 | 11.035 / 8.292 | 0/4 | 45.2 / 457.6 |
| 1×4 | bands-tree2 | 11.791 / 9.670 | 0/6 | 11.782 / 9.615 | 0/6 | 116.4 / 585.3 |
| 1×8 | split48 | 13.170 / 10.858 | 0/6 | 13.229 / 10.497 | 0/6 | 118.1 / 691.3 |
| 1×16 | split48 | 15.597 / 12.443 | 0/6 | 15.610 / 12.652 | 0/6 | 153.2 / 824.0 |
| 2×2 | bands-tree2 | 12.513 / 11.799 | 0/6 | 12.473 / 11.945 | 0/6 | 134.5 / 811.9 |
| 2×4 | bands-tree2 | 13.948 / 12.395 | 0/6 | 13.979 / 12.294 | 0/6 | 141.2 / 828.2 |
| 2×8 | wide32 | 15.485 / 15.465 | 2/6 | 15.542 / 15.203 | 2/6 | 130.7 / 1026.8 |
| 2×16 | split56 | 19.762 / 17.186 | 0/6 | 19.512 / 17.119 | 0/6 | 276.9 / 1249.4 |
| 3×2 | split56 | 16.143 / 16.261 | 5/6 | 16.160 / 16.103 | 3/6 | 210.2 / 1162.7 |
| 3×4 | split56 | 16.928 / 16.756 | 2/6 | 16.499 / 16.733 | 5/6 | 219.1 / 1216.9 |
| 3×8 | split56 | 18.109 / 17.452 | 0/6 | 17.897 / 17.721 | 2/6 | 253.1 / 1277.6 |
| 4×2 | split56 | 17.333 / 18.318 | 6/6 | 17.653 / 18.722 | 6/6 | 239.6 / 1350.5 |
| 4×4 | split64 | 18.535 / 19.027 | 6/6 | 18.520 / 18.939 | 5/6 | 251.3 / 1389.6 |
| 4×8 | split56 | 19.339 / 19.753 | 6/6 | 19.572 / 19.646 | 4/6 | 274.6 / 1460.6 |
| 4×16 | split56 | 22.174 / 21.411 | 1/6 | 22.112 / 21.427 | 1/6 | 329.4 / 1585.7 |
| 5×2 | coeff | 19.754 / 20.692 | 8/8 | 19.918 / 20.841 | 4/4 | 249.2 / 1556.4 |
| 5×4 | split64 | 19.906 / 21.015 | 6/6 | 19.863 / 21.144 | 6/6 | 285.9 / 1595.6 |
| 5×8 | release | 24.895 / 26.261 | 8/8 | 24.811 / 26.316 | 4/4 | 344.6 / 1933.8 |
| 5×16 | coeff-packed | 26.466 / 27.639 | 8/8 | 26.062 / 27.489 | 4/4 | 336.9 / 2064.5 |
| 6×2 | release | 24.168 / 27.277 | 8/8 | 24.628 / 27.507 | 4/4 | 354.3 / 2006.6 |
| 6×4 | release | 25.074 / 27.469 | 8/8 | 25.183 / 27.424 | 4/4 | 359.6 / 2012.8 |
| 6×8 | release | 25.992 / 28.091 | 8/8 | 26.329 / 28.164 | 4/4 | 373.9 / 2206.1 |
| 8×2 | release | 25.794 / 30.747 | 8/8 | 26.192 / 30.997 | 4/4 | 415.5 / 2452.9 |
| 8×4 | release | 27.552 / 31.238 | 8/8 | 27.722 / 31.699 | 4/4 | 425.0 / 2493.7 |
| 8×8 | release | 28.629 / 32.375 | 8/8 | 28.954 / 32.213 | 4/4 | 451.5 / 2569.6 |
| 8×16 | release | 30.909 / 34.431 | 8/8 | 31.337 / 34.322 | 4/4 | 497.3 / 2737.0 |
| 12×2 | release | 32.603 / 48.611 | 8/8 | 32.494 / 49.294 | 4/4 | 546.9 / 3705.2 |
| 12×4 | release | 35.924 / 48.765 | 8/8 | 35.441 / 48.880 | 4/4 | 576.0 / 3729.2 |
| 12×8 | release | 36.099 / 49.532 | 8/8 | 36.523 / 50.280 | 4/4 | 606.4 / 3781.6 |
| 16×2 | release | 34.278 / 56.964 | 8/8 | 34.799 / 56.992 | 4/4 | 651.9 / 4610.4 |
| 16×4 | release | 41.742 / 57.549 | 8/8 | 42.218 / 57.466 | 4/4 | 745.5 / 4638.2 |
| 16×8 | release | 42.113 / 58.327 | 8/8 | 42.265 / 58.358 | 4/4 | 760.6 / 4728.3 |
| 24×2 | release | 47.046 / 87.628 | 8/8 | 46.962 / 88.524 | 4/4 | 991.6 / 6764.9 |
| 24×4 | release | 57.174 / 88.177 | 8/8 | 57.279 / 88.947 | 4/4 | 1159.9 / 6751.3 |
| 32×2 | release | 67.100 / 102.224 | 8/8 | 66.861 / 102.403 | 4/4 | 1435.2 / 8388.7 |
| 40×2 | release | 78.764 / 117.909 | 8/8 | 79.831 / 118.855 | 4/4 | 1673.9 / 9767.6 |
| 48×2 | release | 103.680 / 166.687 | 8/8 | 106.217 / 167.224 | 4/4 | 1871.8 / 12451.1 |
| 49×2 | release | 106.354 / 168.649 | 8/8 | 106.525 / 168.572 | 4/4 | 1799.9 / 12596.7 |
| merge-8×1 | release | 28.546 / 29.924 | 8/8 | 28.999 / 29.848 | 3/4 | 362.0 / 2367.3 |
| merge-24×1 | release | 51.427 / 85.223 | 8/8 | 48.707 / 85.197 | 4/4 | 1039.5 / 6587.8 |
| merge-54×1 | release | 102.684 / 173.714 | 8/8 | 95.739 / 173.307 | 4/4 | 1877.1 / 13040.1 |

## Remaining latency gaps

Each row gives the larger relative deficit across the two transactions. This list includes every shape that missed its target on either input.

| Shape | Input | Gap ms | Above Aeglos |
|---|---|---:|---:|
| 1×2 | heldout | 2.743 | +33.08% |
| 1×8 | heldout | 2.732 | +26.03% |
| 1×16 | catalog | 3.154 | +25.35% |
| 1×4 | heldout | 2.167 | +22.54% |
| 2×16 | catalog | 2.576 | +14.99% |
| 2×4 | heldout | 1.685 | +13.71% |
| 2×2 | catalog | 0.714 | +6.05% |
| 3×8 | catalog | 0.658 | +3.77% |
| 4×16 | catalog | 0.763 | +3.57% |
| 2×8 | heldout | 0.338 | +2.23% |
| 3×4 | catalog | 0.171 | +1.02% |
| 3×2 | heldout | 0.057 | +0.36% |

## Supplied Apple references

The 96-core catalog latency reached ten times the supplied M5 CPU speed on 1/8 references.

| Shape | C7a ms | M5 CPU / Metal ms | C7a speed vs CPU / Metal |
|---|---:|---:|---:|
| 1×2 | 8.001 | 37.0 / 24.0 | 4.62× / 3.00× |
| 2×2 | 12.513 | 59.0 / 30.0 | 4.72× / 2.40× |
| 4×2 | 17.333 | 109.0 / 48.0 | 6.29× / 2.77× |
| 8×2 | 25.794 | 189.0 / 74.0 | 7.33× / 2.87× |
| 12×2 | 32.603 | 284.0 / 101.0 | 8.71× / 3.10× |
| 16×2 | 34.278 | 335.0 / 124.0 | 9.77× / 3.62× |
| 24×2 | 47.046 | 528.0 / 176.0 | 11.22× / 3.74× |
| merge-24×1 | 51.427 | 507.0 / 175.0 | 9.86× / 3.40× |

[Per-shape results](artifacts/shapes.json) · [Initial confirmation](../2026-11-19-loss-routes689/README.md) · [Worker and geometry trials](../2026-11-19-tile-c7a703/README.md) · [Second transactions](../2026-11-19-catalog-holdout700/README.md) · [AWS resource cleanup](artifacts/resources.json)

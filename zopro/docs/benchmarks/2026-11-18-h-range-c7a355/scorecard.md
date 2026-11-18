# C7a results against the eight supplied M5 references

The first five rows use the preceding full-shape C7a panel with configurations declared before that run. The last three use the exact newer two-range H release on the fresh qualification host. All three use 64 physical cores. The 16×2 and merge24 rows include the exploratory eight-epoch solver setting. This is a scorecard across two hosts and builds. The 56-core merge24 result with the native 16-epoch setting provides a separate check of the CPU target on the newer host.

| Shape | Physical cores | Configuration | Verified SDK ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup | Blocks at 10× CPU |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 64 | G2 two ranges | 8.451 | 54 | 37 / 24 | 4.38× / 2.84× | 0/7 |
| 2x2 | 48 | G2 two ranges | 14.127 | 101 | 59 / 30 | 4.18× / 2.12× | 0/7 |
| 4x2 | 48 | ordinary | 17.984 | 278 | 109 / 48 | 6.06× / 2.67× | 0/7 |
| 8x2 | 48 | ordinary | 24.651 | 455 | 189 / 74 | 7.67× / 3.00× | 0/7 |
| 12x2 | 64 | H19 | 33.529 | 656 | 284 / 101 | 8.47× / 3.01× | 0/7 |
| 16x2 | 64 | H19 two ranges, eight epochs | 34.226 | 711 | 335 / 124 | 9.79× / 3.62× | 0/5 |
| 24x2 | 64 | H19 two ranges | 47.786 | 1127 | 528 / 176 | 11.05× / 3.68× | 5/5 |
| merge-24x1 | 64 | H19 two ranges, eight epochs | 49.256 | 1033 | 507 / 175 | 10.29× / 3.55× | 4/5 |

[Preceding full-shape C7a panel](../2026-11-18-release-panel351/README.md) · [Integrated H qualification](README.md) · [Solver epoch comparison](epochs.md)

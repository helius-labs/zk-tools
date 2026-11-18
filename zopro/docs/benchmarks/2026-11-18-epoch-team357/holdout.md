# Fresh verification of the selected configurations

Each configuration was selected using only the five primary process means. The table uses seven subsequently randomized processes of 21 complete requests on the same host. Paired changes compare the selected settings with the previously used reference at 64 cores.

| Shape | Selected cores / epochs | Screen ms | Holdout ms | Control ms | Paired change | Faster blocks | Peak MiB | M5 CPU / Metal speedup | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 12x2 | 48 / 8 | 30.925 | 30.823 | 32.144 | -4.42% | 7/7 | 659 | 9.21× / 3.28× | 0/7 |
| 16x2 | 64 / 6 | 33.759 | 33.654 | 34.416 | -1.67% | 5/7 | 826 | 9.95× / 3.68× | 1/7 |
| merge-24x1 | 56 / 8 | 48.494 | 49.009 | 49.303 | -1.51% | 5/7 | 1147 | 10.34× / 3.57× | 7/7 |

[Selection protocol](artifacts/holdout-protocol.md) · [All primary settings](README.md)

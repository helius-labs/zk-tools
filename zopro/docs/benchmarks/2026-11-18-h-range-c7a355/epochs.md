# Solver epochs at 64 and 96 physical cores

The earlier independent profile identified interpreter time increasing with core count. This follow-up varies the requested epoch count on the exact integrated release. Every setting uses two H ranges. Zero executes the solver in program order. Five randomized process blocks each contain eleven full SDK requests. CPU includes system and user time for all threads.

| Shape | Cores | Requested epochs | Median ms | Paired vs 16 | Faster blocks | CPU-ms/proof | Peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 0 | 96.778 | +165.30% | 0/5 | 1486.8 | 669 |
| 16x2 | 64 | 8 | 34.226 | -4.84% | 5/5 | 1685.8 | 711 |
| 16x2 | 64 | 16 | 36.062 | +0.00% | 0/5 | 1788.4 | 724 |
| 16x2 | 64 | 32 | 38.106 | +5.82% | 0/5 | 1933.6 | 751 |
| 16x2 | 96 | 0 | 96.418 | +149.04% | 0/5 | 2065.4 | 672 |
| 16x2 | 96 | 8 | 37.078 | -3.07% | 5/5 | 2537.9 | 740 |
| 16x2 | 96 | 16 | 38.876 | +0.00% | 0/5 | 2700.5 | 725 |
| 16x2 | 96 | 32 | 41.973 | +8.71% | 0/5 | 3033.6 | 727 |
| merge-24x1 | 64 | 0 | 130.882 | +155.36% | 0/5 | 2065.1 | 938 |
| merge-24x1 | 64 | 8 | 49.256 | -3.37% | 5/5 | 2323.8 | 1033 |
| merge-24x1 | 64 | 16 | 51.075 | +0.00% | 0/5 | 2449.2 | 1046 |
| merge-24x1 | 64 | 32 | 52.099 | +2.71% | 0/5 | 2558.1 | 1063 |
| merge-24x1 | 96 | 0 | 132.070 | +136.80% | 0/5 | 2804.0 | 963 |
| merge-24x1 | 96 | 8 | 53.188 | -4.79% | 4/5 | 3442.1 | 1081 |
| merge-24x1 | 96 | 16 | 55.866 | +0.00% | 0/5 | 3671.2 | 1052 |
| merge-24x1 | 96 | 32 | 57.783 | +4.04% | 0/5 | 3949.2 | 1038 |

[Prior stage attribution](../2026-11-18-h-scale353/stages.md) · [Protocol](artifacts/epochs-protocol.md) · [Integrated release report](README.md)

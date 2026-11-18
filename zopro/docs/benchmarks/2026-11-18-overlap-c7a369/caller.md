# Caller-thread verification during generation

Seven fresh processes per cell use a 16-worker frontend for 1×2, and the main pool for 16×2. Early and caller use the same executable. Prior uses the original proof-generation overlap prototype.

| Shape | Cores | Mode | Median ms | CPU-ms | Peak MiB | Paired vs release | Faster blocks | Paired vs early | Faster blocks |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | release | 8.187 | 274.8 | 53 | +0.00% | 0/7 | +1.73% | 0/7 |
| 1x2 | 64 | voff | 8.138 | 278.3 | 54 | +0.33% | 3/7 | +1.68% | 1/7 |
| 1x2 | 64 | prior | 8.053 | 273.1 | 54 | -2.00% | 5/7 | +0.02% | 3/7 |
| 1x2 | 64 | early | 8.028 | 277.0 | 51 | -1.70% | 7/7 | +0.00% | 0/7 |
| 1x2 | 64 | caller | 8.090 | 276.2 | 58 | -1.14% | 6/7 | +1.15% | 3/7 |
| 1x2 | 96 | release | 8.260 | 390.3 | 53 | +0.00% | 0/7 | +2.51% | 0/7 |
| 1x2 | 96 | voff | 8.208 | 408.1 | 50 | -0.25% | 5/7 | +1.86% | 0/7 |
| 1x2 | 96 | prior | 8.119 | 387.5 | 51 | -1.09% | 7/7 | +0.83% | 1/7 |
| 1x2 | 96 | early | 8.060 | 404.7 | 48 | -2.45% | 7/7 | +0.00% | 0/7 |
| 1x2 | 96 | caller | 8.153 | 405.2 | 54 | -1.12% | 6/7 | +1.17% | 0/7 |
| 16x2 | 64 | release | 33.918 | 1666.5 | 799 | +0.00% | 0/7 | -7.42% | 7/7 |
| 16x2 | 64 | voff | 34.324 | 1702.2 | 814 | +0.53% | 3/7 | -7.94% | 7/7 |
| 16x2 | 64 | early | 37.462 | 1729.3 | 768 | +8.01% | 0/7 | +0.00% | 0/7 |
| 16x2 | 64 | caller | 34.279 | 1689.7 | 810 | +0.19% | 2/7 | -8.75% | 7/7 |
| 16x2 | 96 | release | 36.245 | 2461.1 | 786 | +0.00% | 0/7 | -7.66% | 6/7 |
| 16x2 | 96 | voff | 36.527 | 2583.9 | 826 | +1.37% | 2/7 | -4.73% | 7/7 |
| 16x2 | 96 | early | 38.769 | 2598.3 | 786 | +8.29% | 1/7 | +0.00% | 0/7 |
| 16x2 | 96 | caller | 36.461 | 2593.7 | 822 | +0.36% | 3/7 | -6.52% | 6/7 |

[Individual timings](artifacts/processes.json.gz)

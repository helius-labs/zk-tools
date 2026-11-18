# Earlier G2 pair merges on the same C7a host

This panel followed all primary timings and traces. All four settings use four G2 ranges with 11-bit windows and 16 frontend workers. The pair setting waits for all four range tasks before running two independent merges. The early setting starts each pair merge when its two inputs are ready. Both perform the same number of group operations as the existing serial merge.

| Shape | Cores | Mode | Median ms | Paired vs serial | Faster blocks | CPU-ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 64 | retained | 9.247 | +0.17% | 3/7 | 299.9 | 53 |
| 1x2 | 64 | off | 9.371 | +0.00% | 0/7 | 308.2 | 54 |
| 1x2 | 64 | pair | 9.185 | -1.46% | 5/7 | 306.5 | 53 |
| 1x2 | 64 | early | 9.229 | -1.12% | 5/7 | 300.2 | 51 |
| 1x2 | 96 | retained | 9.105 | +1.11% | 3/7 | 416.2 | 53 |
| 1x2 | 96 | off | 8.972 | +0.00% | 0/7 | 430.7 | 48 |
| 1x2 | 96 | pair | 8.852 | -0.73% | 4/7 | 427.8 | 49 |
| 1x2 | 96 | early | 8.825 | -0.58% | 5/7 | 428.2 | 50 |
| 2x2 | 48 | retained | 13.801 | -1.19% | 4/7 | 353.0 | 101 |
| 2x2 | 48 | off | 13.864 | +0.00% | 0/7 | 356.7 | 101 |
| 2x2 | 48 | pair | 13.803 | -0.60% | 4/7 | 354.0 | 96 |
| 2x2 | 48 | early | 13.774 | -0.81% | 4/7 | 353.4 | 100 |

At 96 cores starting the pair merges early reduced paired latency by 0.58%, with five wins in seven blocks. This missed the prespecified 2% threshold. The source change remains experimental.

[Protocol](artifacts/secondary-protocol.md) · [Samples](artifacts/secondary-processes.json.gz) · [Verification](artifacts/secondary-verification.json)

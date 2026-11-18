# Single frontend handoff on C7a

One c7a.24xlarge, three warmups per process. Seven shuffled blocks of 51 requests for 1×2 on 96 cores and five blocks of 31 for each other cell. Memory is maximum process RSS. Percentage changes use within-block comparisons to the integrated release.

| Shape | Cores | Mode | SDK ms | Paired change | Wins / pairs | CPU-ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 64 | defer | 8.207 | -0.06% | 3/5 | 277.1 | 49.1 |
| 1x2 | 64 | handoff | 11.790 | +43.52% | 0/5 | 347.9 | 49.8 |
| 1x2 | 64 | handoff-caller | 11.793 | +43.61% | 0/5 | 349.7 | 58.9 |
| 1x2 | 64 | off | 8.257 | +0.30% | 1/5 | 278.7 | 50.0 |
| 1x2 | 64 | release | 8.212 | +0.00% | 0/5 | 278.2 | 52.0 |
| 1x2 | 96 | aeglos | 8.016 | -3.13% | 6/7 | 505.5 | 466.0 |
| 1x2 | 96 | defer | 8.256 | -0.02% | 4/7 | 384.2 | 51.3 |
| 1x2 | 96 | handoff | 11.779 | +42.90% | 0/7 | 482.6 | 54.3 |
| 1x2 | 96 | handoff-caller | 11.763 | +42.39% | 0/7 | 484.3 | 56.0 |
| 1x2 | 96 | handoff-early | 11.650 | +41.32% | 0/7 | 482.0 | 51.9 |
| 1x2 | 96 | off | 8.250 | +0.14% | 3/7 | 387.1 | 46.9 |
| 1x2 | 96 | release | 8.258 | +0.00% | 0/7 | 387.1 | 49.1 |
| 2x2 | 48 | defer | 13.018 | -0.05% | 3/5 | 344.3 | 107.5 |
| 2x2 | 48 | handoff | 16.647 | +28.09% | 0/5 | 368.0 | 136.0 |
| 2x2 | 48 | handoff-caller | 16.728 | +28.64% | 0/5 | 368.6 | 136.2 |
| 2x2 | 48 | off | 13.117 | +0.73% | 1/5 | 346.1 | 103.7 |
| 2x2 | 48 | release | 13.010 | +0.00% | 0/5 | 345.5 | 103.8 |

[Request timings](artifacts/c7a.json.gz)

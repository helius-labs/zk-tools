# Indexed 12×2 proof latency by FFT pool and output layout

Each process warmed three requests and timed fifteen complete SDK requests. All 114 exports verified, including the two dispatch probes. The latency column is the median of process means.

| Witness | Output and pool | Latency ms | Paired change vs main, wins | Peak MiB | M5 CPU / Metal | Blocks at or below 28.4 ms |
|---|---|---:|---:|---:|---:|---:|
| Reference | main | 28.628 | — | 562.8 | 9.92× / 3.53× | 0/12 |
| Reference | pool | 28.918 | +0.96% (2/12) | 455.0 | 9.82× / 3.49× | 0/12 |
| Reference | tile | 28.808 | +0.44% (4/12) | 569.5 | 9.86× / 3.51× | 0/12 |
| Reference | pool-tile | 28.767 | +0.35% (4/12) | 448.9 | 9.87× / 3.51× | 2/12 |
| 58 | main | 28.797 | — | 590.9 | — | — |
| 58 | pool | 28.715 | -0.28% (2/4) | 447.5 | — | — |
| 58 | tile | 28.801 | -0.24% (3/4) | 553.0 | — | — |
| 58 | pool-tile | 28.936 | +0.26% (1/4) | 434.3 | — | — |
| 61 | main | 28.848 | — | 563.3 | — | — |
| 61 | pool | 28.865 | +0.06% (2/4) | 443.1 | — | — |
| 61 | tile | 28.812 | -0.13% (2/4) | 552.5 | — | — |
| 61 | pool-tile | 28.624 | -0.41% (2/4) | 466.1 | — | — |
| 139 | main | 43.705 | — | 767.5 | — | — |
| 139 | pool | 43.132 | -0.96% (3/4) | 513.2 | — | — |
| 139 | tile | 43.363 | -1.07% (3/4) | 761.8 | — | — |
| 139 | pool-tile | 43.329 | -0.71% (4/4) | 510.8 | — | — |

[Protocol](protocol.md) · [All samples](artifacts/rows.json.gz) · [Dispatch probes](artifacts/probes.json) · [Proof verification](artifacts/verification.json)

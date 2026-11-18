# Frontend and curve worker placement on C7a

One c7a.24xlarge, with all 96 physical CPUs available to every process. The main workers stayed on the first 64 or 80 cores. The frontend used 16 cores. Five randomized blocks each timed 51 verified 1×2 SDK requests per setting. The driver checked the worker affinity masks.

| Main cores | G2 schedule | Shared ms | Disjoint ms | Paired change | Wins / 5 | CPU-ms shared / disjoint | Peak MiB shared / disjoint |
|---:|---|---:|---:|---:|---:|---:|---:|
| 64 | defer | 8.852 | 9.085 | +2.96% | 0 | 303.2 / 308.5 | 51 / 44 |
| 64 | handoff | 11.265 | 10.846 | -3.92% | 5 | 339.7 / 343.2 | 53 / 39 |
| 80 | defer | 8.584 | 8.993 | +4.23% | 0 | 353.2 / 356.6 | 51 / 42 |
| 80 | handoff | 11.507 | 11.132 | -3.16% | 5 | 419.9 / 426.7 | 51 / 40 |

Separating the two pools recovered 0.38–0.42 ms when G2 ran early. At both widths the wait control with disjoint workers still finished over 1.7 ms sooner than its early-G2 counterpart. The placement comparison includes changes in cache locality when the frontend moves.

## Ryzen cross-check

The 12-main/4-frontend setting uses sixteen physical cores. With 24 main workers and eight frontend workers, all 32 logical CPUs are available. All eight offset frontend workers in that setting still share physical cores with the main pool through SMT.

| Main / frontend | G2 schedule | Same logical CPUs, ms | Offset CPUs, ms | Paired change | Offset wins / 5 | Physical cores shared after offset |
|---|---|---:|---:|---:|---:|---:|
| 12 / 4 | defer | 11.016 | 10.843 | -0.73% | 4 | 0 |
| 12 / 4 | handoff | 11.265 | 10.496 | -6.59% | 4 | 0 |
| 24 / 8 | defer | 10.470 | 7.920 | -24.36% | 4 | 8 |
| 24 / 8 | handoff | 8.173 | 7.722 | -5.44% | 5 | 8 |

[Ryzen samples](artifacts/ryzen.json.gz) · [C7a process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md)

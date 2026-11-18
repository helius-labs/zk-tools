# Point ranges for wide H windows

Each cell used five randomized processes. Results are medians of process means. The change column compares matched blocks with the ordinary path compiled into the experimental executable.

| Shape | Setting | Time ms | Paired change | Faster blocks | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 16x2 | reference | 45.176 | -6.14% | 3/5 | 657.0 | 590 |
| 16x2 | off | 47.506 | +0.00% | 0/5 | 668.8 | 595 |
| 16x2 | wide | 46.151 | -1.72% | 3/5 | 672.0 | 590 |
| 16x2 | all | 46.316 | -0.88% | 3/5 | 680.9 | 602 |
| 24x2 | reference | 72.088 | -2.48% | 4/5 | 1080.9 | 935 |
| 24x2 | off | 73.424 | +0.00% | 0/5 | 1089.0 | 1004 |
| 24x2 | wide | 75.268 | +2.47% | 2/5 | 1104.5 | 952 |
| 24x2 | all | 73.842 | -0.62% | 3/5 | 1113.0 | 970 |
| merge-24x1 | reference | 70.777 | +0.40% | 2/5 | 1032.6 | 868 |
| merge-24x1 | off | 71.010 | +0.00% | 0/5 | 1040.5 | 873 |
| merge-24x1 | wide | 74.217 | +4.31% | 0/5 | 1058.5 | 882 |
| merge-24x1 | all | 72.988 | +2.59% | 0/5 | 1065.9 | 874 |

All 64 exported proofs verified. Separate timed probes checked activation and the zero-memory fallback.

[Protocol](artifacts/protocol.md) · [Timings](artifacts/processes.json.gz) · [Routing](artifacts/routing.json) · [Verification](artifacts/verification.json)

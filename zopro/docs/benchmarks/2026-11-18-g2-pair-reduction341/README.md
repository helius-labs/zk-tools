# Pairwise G2 bucket merging

Each cell used five randomized processes. Results are medians of process means. The change column compares matched blocks with the ordinary path compiled into the experimental executable.

| Shape | Setting | Time ms | Paired change | Faster blocks | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | retained | 7.874 | +1.16% | 2/5 | 157.9 | 65 |
| 1x2 | off | 7.784 | +0.00% | 0/5 | 158.1 | 70 |
| 1x2 | pair | 7.825 | -0.09% | 3/5 | 158.0 | 76 |
| 1x2 | early | 7.754 | -0.77% | 4/5 | 156.1 | 71 |
| 2x2 | retained | 10.777 | -1.19% | 5/5 | 210.4 | 113 |
| 2x2 | off | 10.983 | +0.00% | 0/5 | 212.3 | 121 |
| 2x2 | pair | 11.038 | -1.25% | 4/5 | 211.9 | 124 |
| 2x2 | early | 10.820 | -1.20% | 4/5 | 210.2 | 116 |

All 44 exported proofs verified. Separate timed probes checked activation and the zero-memory fallback.

[Protocol](artifacts/protocol.md) · [Timings](artifacts/processes.json.gz) · [Routing](artifacts/routing.json) · [Verification](artifacts/verification.json)

# CPU samples around the G2 return

The table pools userspace CPU-clock samples across all threads within each wall-time interval. Samples include workers doing independent tasks. Family names describe the sampled instruction. Symbols shared by G1 and G2 remain in the other-arithmetic group. A slow proof took at least 10 ms. Raw per-family counts and the most frequent demangled symbols are in the artifact.

| Cores | Geometry | Requests | G2 samples | G2 interval, top family | After samples | After G2, top family |
|---:|---|---|---:|---|---:|---|
| 32 | 12x2 | fast | 0 | — | 0 | — |
| 32 | 12x2 | slow | 13237 | G1 (46%) | 0 | — |
| 32 | 11x4 | fast | 0 | — | 0 | — |
| 32 | 11x4 | slow | 14136 | G1 (43%) | 2 | Crossbeam (50%) |
| 64 | 12x2 | fast | 16494 | G1 (40%) | 23 | Crossbeam (78%) |
| 64 | 12x2 | slow | 124 | G1 (31%) | 0 | — |
| 64 | 11x4 | fast | 16049 | G1 (37%) | 7 | Crossbeam (43%) |
| 64 | 11x4 | slow | 2090 | G1 (33%) | 0 | — |
| 96 | 12x2 | fast | 12861 | Crossbeam (35%) | 52 | Crossbeam (98%) |
| 96 | 12x2 | slow | 10586 | Crossbeam (47%) | 2021 | G1 (55%) |
| 96 | 11x4 | fast | 16517 | Crossbeam (31%) | 2310 | Crossbeam (70%) |
| 96 | 11x4 | slow | 6791 | Crossbeam (43%) | 213 | G1 (46%) |

[Sampling counts and symbols](artifacts/cpu-samples.json) · [Worker timing](workers.md)

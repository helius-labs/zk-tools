# Accepted full-proof latency and stage times at intermediate C7a widths

On a fresh c7a.24xlarge the lowest median among 48 and 64 workers was 36.681 ms for 16×2 and 55.731 ms for merge24. Their 10× M5 CPU targets are 33.5 and 50.7 ms. The control shape 24×2 measured 53.893 ms at its faster width. These rows use the accepted prover with fixed workers and a warmed key.

| Shape | Workers | Wall ms | CPU ms | Peak MiB | Target ms | Blocks at target | M5 CPU | M5 Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 48 | 36.681 | 1434 | 811 | 33.5 | 0/7 | 9.133× | 3.381× |
| 16x2 | 64 | 38.248 | 1846 | 831 | 33.5 | 0/7 | 8.759× | 3.242× |
| merge-24x1 | 48 | 55.731 | 2054 | 1178 | 50.7 | 0/7 | 9.097× | 3.140× |
| merge-24x1 | 64 | 56.721 | 2523 | 1196 | 50.7 | 0/7 | 8.938× | 3.085× |
| 24x2 | 48 | 54.221 | 2127 | 1289 | 52.8 | 0/7 | 9.738× | 3.246× |
| 24x2 | 64 | 53.893 | 2562 | 1308 | 52.8 | 0/7 | 9.797× | 3.266× |

## Sparse stage profile

| Shape | Workers | Solver ms | FFT ms | MSM ms | Remaining ms | Paired trace/plain | Raw G1 / G2 finished last |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 48 | 5.62 | 6.57 | 22.32 | 3.01 | 1.014 | 50 / 43 |
| 16x2 | 64 | 6.87 | 6.33 | 22.65 | 3.30 | 1.025 | 74 / 19 |
| merge-24x1 | 48 | 11.22 | 11.93 | 29.10 | 3.37 | 1.004 | 82 / 11 |
| merge-24x1 | 64 | 11.99 | 11.43 | 30.47 | 3.67 | 1.015 | 93 / 0 |

Stage entries are the mean of 93 traced proofs per row across three independently backed processes. The solver, FFT and enclosing MSM intervals are disjoint. The remaining time includes intervening preparation, finalization, and SDK work. A raw curve query returning last identifies the last probed raw call. Group preparation and assembly may also be in flight. 4/4 cells passed the prespecified 5% median trace/plain tolerance. End-to-end results use seven uninstrumented process means per cell. All 64 fresh process exports verified with Gnark on the instance and again on retrieval. The instance terminated and temporary storage was removed. [Block data and observer calibration](artifacts/analysis.json) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

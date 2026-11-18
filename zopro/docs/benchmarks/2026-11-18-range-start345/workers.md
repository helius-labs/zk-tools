# G1 and G2 timing by request duration

Traces cover the canonical core, raw G2 kernel, each G2 point range, and each of the forty G1 K/H feed calls. A negative last-feed time means all G1 feed calls returned before the G2 kernel returned. Request groups use the 10 ms threshold from the previous host. Rows pool three traced processes per setting. Observer shift compares traced and plain process means by block.

| Cores | Mode | Group | Count | Proof ms | Observer shift | G2 to first range ms | Start spread ms | Longest range ms | After G2 ms | Last G1 feed vs G2 end ms |
|---:|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | test2 | fast | 150 | 8.535 | +2.15% | 0.132 | 0.006 | 3.239 | 0.019 | -0.815 |
| 64 | test2 | slow | 3 | 11.960 | +2.15% | 3.533 | 0.172 | 3.061 | 0.024 | -4.141 |
| 64 | off | fast | 130 | 9.144 | +3.82% | 0.471 | 1.442 | 2.799 | 0.018 | -1.568 |
| 64 | off | slow | 23 | 10.176 | +3.82% | 1.803 | 1.420 | 2.819 | 0.019 | -2.405 |
| 64 | serial | fast | 148 | 9.219 | +2.18% | 0.717 | 1.500 | 2.248 | 0.019 | -1.533 |
| 64 | serial | slow | 5 | 10.200 | +2.18% | 0.948 | 1.604 | 3.606 | 0.019 | -1.769 |
| 64 | range | fast | 134 | 9.553 | +2.87% | 0.311 | 1.637 | 3.398 | 0.018 | -1.941 |
| 64 | range | slow | 19 | 10.270 | +2.87% | 0.319 | 1.689 | 3.839 | 0.019 | -2.258 |
| 96 | test2 | fast | 87 | 8.898 | +4.18% | 0.123 | 0.009 | 3.283 | 0.021 | -0.672 |
| 96 | test2 | slow | 66 | 12.114 | +4.18% | 0.130 | 0.008 | 3.347 | 0.141 | -0.274 |
| 96 | off | fast | 113 | 8.643 | +5.39% | 0.298 | 0.011 | 2.400 | 0.634 | 0.209 |
| 96 | off | slow | 40 | 11.091 | +5.39% | 3.628 | 0.013 | 2.313 | 0.019 | -3.050 |
| 96 | serial | fast | 150 | 8.649 | +2.55% | 0.719 | 0.249 | 2.372 | 0.132 | -0.325 |
| 96 | serial | slow | 3 | 10.171 | +2.55% | 0.935 | 0.352 | 2.638 | 0.518 | 0.038 |
| 96 | range | fast | 132 | 8.967 | +4.52% | 0.322 | 0.603 | 2.223 | 0.023 | -0.777 |
| 96 | range | slow | 21 | 11.426 | +4.52% | 0.324 | 0.553 | 2.117 | 0.020 | -0.691 |

At 96 cores, the four-range parallel control had 40 traced requests at or above 10 ms; their median first-range delay was 3.628 ms. The serial split had three such requests. The parallel-control observer shifted mean latency by 5.39%, so the primary untraced request percentiles provide the direct latency comparison.

[Primary timings](README.md) · [Individual traces](artifacts/worker-traces.json.gz) · [Verification](artifacts/worker-verification.json)

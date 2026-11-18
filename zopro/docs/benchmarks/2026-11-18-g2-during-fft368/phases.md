# Local critical-path profiles

Each setting uses three traced and three plain processes on the same CPU mask as the local confirmation. Times are medians across 93 traced requests. Overlap is computed from the actual intersection of the FFT and G2 call intervals within each SDK call.

| Shape | Setting | Tracing overhead | SDK ms | FFT ms | G2 ms | Overlap ms | Requests overlapping | After both ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | wait | +2.24% | 7.744 | 0.623 | 4.943 | 0.000 | 0/93 | 0.629 |
| 1x2 | g2 | +1.91% | 8.074 | 3.038 | 2.850 | 2.716 | 93/93 | 3.373 |
| 2x2 | wait | +0.43% | 10.825 | 1.283 | 7.160 | 0.000 | 0/93 | 0.675 |
| 2x2 | g2 | +7.74% | 12.090 | 3.790 | 4.632 | 3.233 | 93/93 | 5.105 |

[Samples and intervals](artifacts/phases.json.gz)

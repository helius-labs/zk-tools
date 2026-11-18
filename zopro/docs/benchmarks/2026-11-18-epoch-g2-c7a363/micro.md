# Critical path of the selected 1×2 configurations

The exact SDK was traced at the two selected widths in three fresh processes with 51 requests each. Each traced process has a paired plain run. Timings are medians across 153 traced requests. The three adjacent intervals run from SDK entry to FFT return, then to canonical core return, then to SDK return. Removing the middle interval entirely gives the last column.

| Main cores | Epochs / frontend | Trace overhead | SDK ms | Through FFT ms | FFT return to core return ms | After core ms | SDK excluding middle interval ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | 4 / 16 | +3.79% | 8.337 | 2.899 | 4.509 | 0.898 | 3.804 |
| 96 | 4 / 16 | +2.96% | 8.467 | 3.066 | 4.383 | 0.970 | 4.042 |

| Cores | Witness ms | General solve call ms | FFT ms | G2 MSM ms | G2 preparation ms | Miller loops union ms | Final exponentiation ms |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | 0.130 | 1.507 | 0.808 | 3.976 | 0.070 | 0.185 | 0.293 |
| 96 | 0.142 | 1.598 | 0.830 | 3.740 | 0.075 | 0.205 | 0.312 |

A zero duration means the selected symbol had no observed call in that request. Named stages can overlap. These measurements use diagnostic instrumentation. [Samples](artifacts/micro.json.gz)

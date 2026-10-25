# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.647865 | 1.000000 | — | — | — |
| 12x2-w32 | off | 34.798234 | 1.000186 | — | — | — |
| 12x2-w32 | on | 35.167746 | 1.015005 | — | — | — |
| 16x2-w32 | accepted | 41.071827 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.819686 | 0.994682 | — | — | — |
| 16x2-w32 | on | 41.761045 | 1.014811 | — | — | — |
| 1x2-w16 | accepted | 12.386867 | 1.000000 | — | — | — |
| 1x2-w16 | off | 11.357605 | 0.861424 | — | — | — |
| 1x2-w16 | on | 11.348449 | 0.899092 | — | — | — |
| 1x2-w32 | accepted | 8.627238 | 1.000000 | 199.044 | 8.716623 | 129.25 |
| 1x2-w32 | off | 8.661483 | 1.005377 | 199.517 | 8.767442 | 130.50 |
| 1x2-w32 | on | 9.090140 | 1.051684 | 208.073 | 9.221545 | 132.25 |
| 24x2-w32 | accepted | 68.254036 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.576518 | 1.003659 | — | — | — |
| 24x2-w32 | on | 70.260092 | 1.024239 | — | — | — |
| 2x2-w32 | accepted | 11.826572 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.786080 | 0.995936 | — | — | — |
| 2x2-w32 | on | 12.192210 | 1.032272 | — | — | — |
| 4x2-w32 | accepted | 16.231862 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.106292 | 1.001146 | — | — | — |
| 4x2-w32 | on | 16.584181 | 1.022741 | — | — | — |
| 8x2-w32 | accepted | 24.625740 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.644355 | 0.997303 | — | — | — |
| 8x2-w32 | on | 25.234214 | 1.020646 | — | — | — |
| merge-24x1-w32 | accepted | 66.982068 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.464317 | 1.012727 | — | — | — |
| merge-24x1-w32 | on | 69.023978 | 1.031941 | — | — | — |
| merge-54x1-w32 | accepted | 177.908327 | 1.000000 | 4917.308 | 178.929207 | 1701.40 |
| merge-54x1-w32 | off | 179.571562 | 1.003775 | 4984.378 | 180.501017 | 1664.79 |
| merge-54x1-w32 | on | 179.444866 | 1.010871 | 4885.316 | 177.618633 | 1655.12 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.089 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.278 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.901 | 0.000 |
| 1x2-w32 | accepted | fft | 4.611 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.622 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.114 | 0.000 |
| 1x2-w32 | accepted | other | 4.852 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.898 | 0.000 |
| 1x2-w32 | accepted | solver | 1.635 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.104 | 0.015 |
| 1x2-w32 | off | G2_arithmetic | 28.736 | -0.543 |
| 1x2-w32 | off | epoch | 9.692 | -0.209 |
| 1x2-w32 | off | fft | 4.750 | 0.138 |
| 1x2-w32 | off | grouping | 0.759 | 0.137 |
| 1x2-w32 | off | inverse | 7.290 | 0.176 |
| 1x2-w32 | off | other | 5.084 | 0.232 |
| 1x2-w32 | off | scheduler | 4.140 | 0.242 |
| 1x2-w32 | off | solver | 1.446 | -0.189 |
| 1x2-w32 | on | G1_arithmetic | 36.402 | -1.686 |
| 1x2-w32 | on | G2_arithmetic | 32.044 | 2.766 |
| 1x2-w32 | on | epoch | 9.873 | -0.028 |
| 1x2-w32 | on | fft | 4.612 | 0.000 |
| 1x2-w32 | on | grouping | 0.548 | -0.073 |
| 1x2-w32 | on | inverse | 6.651 | -0.462 |
| 1x2-w32 | on | other | 4.587 | -0.265 |
| 1x2-w32 | on | scheduler | 3.815 | -0.083 |
| 1x2-w32 | on | solver | 1.467 | -0.168 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.162 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 15.074 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.238 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.128 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.416 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.804 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.882 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.096 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.199 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 25.806 | -0.356 |
| merge-54x1-w32 | off | G2_arithmetic | 16.092 | 1.017 |
| merge-54x1-w32 | off | epoch | 2.204 | -0.034 |
| merge-54x1-w32 | off | fft | 41.917 | -0.211 |
| merge-54x1-w32 | off | grouping | 1.386 | -0.030 |
| merge-54x1-w32 | off | inverse | 1.662 | -0.142 |
| merge-54x1-w32 | off | other | 3.976 | 0.094 |
| merge-54x1-w32 | off | scheduler | 2.035 | -0.060 |
| merge-54x1-w32 | off | solver | 4.921 | -0.278 |
| merge-54x1-w32 | on | G1_arithmetic | 26.192 | 0.029 |
| merge-54x1-w32 | on | G2_arithmetic | 15.714 | 0.639 |
| merge-54x1-w32 | on | epoch | 2.158 | -0.080 |
| merge-54x1-w32 | on | fft | 41.448 | -0.680 |
| merge-54x1-w32 | on | grouping | 1.405 | -0.011 |
| merge-54x1-w32 | on | inverse | 1.778 | -0.026 |
| merge-54x1-w32 | on | other | 3.899 | 0.016 |
| merge-54x1-w32 | on | scheduler | 2.148 | 0.053 |
| merge-54x1-w32 | on | solver | 5.259 | 0.060 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.627238 | 37.000 | 4.289× | 24.000 | 2.782× | false |
| 1x2-w32 | off | 8.661483 | 37.000 | 4.272× | 24.000 | 2.771× | false |
| 1x2-w32 | on | 9.090140 | 37.000 | 4.070× | 24.000 | 2.640× | false |
| 2x2-w32 | accepted | 11.826572 | 59.000 | 4.989× | 30.000 | 2.537× | false |
| 2x2-w32 | off | 11.786080 | 59.000 | 5.006× | 30.000 | 2.545× | false |
| 2x2-w32 | on | 12.192210 | 59.000 | 4.839× | 30.000 | 2.461× | false |
| 4x2-w32 | accepted | 16.231862 | 109.000 | 6.715× | 48.000 | 2.957× | false |
| 4x2-w32 | off | 16.106292 | 109.000 | 6.768× | 48.000 | 2.980× | false |
| 4x2-w32 | on | 16.584181 | 109.000 | 6.573× | 48.000 | 2.894× | false |
| 8x2-w32 | accepted | 24.625740 | 189.000 | 7.675× | 74.000 | 3.005× | false |
| 8x2-w32 | off | 24.644355 | 189.000 | 7.669× | 74.000 | 3.003× | false |
| 8x2-w32 | on | 25.234214 | 189.000 | 7.490× | 74.000 | 2.933× | false |
| 12x2-w32 | accepted | 34.647865 | 284.000 | 8.197× | 101.000 | 2.915× | false |
| 12x2-w32 | off | 34.798234 | 284.000 | 8.161× | 101.000 | 2.902× | false |
| 12x2-w32 | on | 35.167746 | 284.000 | 8.076× | 101.000 | 2.872× | false |
| 16x2-w32 | accepted | 41.071827 | 335.000 | 8.156× | 124.000 | 3.019× | false |
| 16x2-w32 | off | 40.819686 | 335.000 | 8.207× | 124.000 | 3.038× | false |
| 16x2-w32 | on | 41.761045 | 335.000 | 8.022× | 124.000 | 2.969× | false |
| 24x2-w32 | accepted | 68.254036 | 528.000 | 7.736× | 176.000 | 2.579× | false |
| 24x2-w32 | off | 68.576518 | 528.000 | 7.699× | 176.000 | 2.566× | false |
| 24x2-w32 | on | 70.260092 | 528.000 | 7.515× | 176.000 | 2.505× | false |
| merge-24x1-w32 | accepted | 66.982068 | 507.000 | 7.569× | 175.000 | 2.613× | false |
| merge-24x1-w32 | off | 68.464317 | 507.000 | 7.405× | 175.000 | 2.556× | false |
| merge-24x1-w32 | on | 69.023978 | 507.000 | 7.345× | 175.000 | 2.535× | false |

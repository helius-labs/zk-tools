# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.442273 | 1.000000 | 958.578 | 34.302294 | 714.00 |
| 12x2-w32 | off | 34.977863 | 1.006953 | 954.486 | 34.754972 | 741.50 |
| 12x2-w32 | on | 34.610800 | 1.002633 | 955.243 | 34.572448 | 723.00 |
| 16x2-w32 | accepted | 41.031470 | 1.000000 | — | — | — |
| 16x2-w32 | off | 41.128801 | 1.000131 | — | — | — |
| 16x2-w32 | on | 41.501937 | 1.009021 | — | — | — |
| 1x2-w16 | accepted | 10.550553 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.566153 | 1.000509 | — | — | — |
| 1x2-w16 | on | 10.757221 | 1.029471 | — | — | — |
| 1x2-w32 | accepted | 8.656291 | 1.000000 | 200.170 | 8.828023 | 128.25 |
| 1x2-w32 | off | 8.572243 | 0.993745 | 200.218 | 8.778259 | 128.00 |
| 1x2-w32 | on | 8.670757 | 1.002669 | 202.074 | 8.847167 | 134.50 |
| 24x2-w32 | accepted | 68.402157 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.582964 | 1.000931 | — | — | — |
| 24x2-w32 | on | 68.668280 | 1.003081 | — | — | — |
| 2x2-w32 | accepted | 11.712734 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.702034 | 1.007953 | — | — | — |
| 2x2-w32 | on | 11.639867 | 1.000374 | — | — | — |
| 4x2-w32 | accepted | 16.109013 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.103748 | 0.997437 | — | — | — |
| 4x2-w32 | on | 16.234542 | 0.997617 | — | — | — |
| 8x2-w32 | accepted | 24.780247 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.391450 | 0.997468 | — | — | — |
| 8x2-w32 | on | 24.629511 | 0.979287 | — | — | — |
| merge-24x1-w32 | accepted | 67.369762 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 67.321777 | 0.994255 | — | — | — |
| merge-24x1-w32 | on | 68.015410 | 1.009809 | — | — | — |
| merge-54x1-w32 | accepted | 176.776680 | 1.000000 | 4896.832 | 176.704629 | 1706.74 |
| merge-54x1-w32 | off | 175.677290 | 0.992151 | 4801.496 | 175.140533 | 1656.22 |
| merge-54x1-w32 | on | 177.759006 | 1.010570 | 4764.291 | 172.939393 | 1724.96 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.083 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.542 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.120 | 0.000 |
| 12x2-w32 | accepted | fft | 28.155 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.840 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.483 | 0.000 |
| 12x2-w32 | accepted | other | 3.855 | 0.000 |
| 12x2-w32 | accepted | scheduler | 4.057 | 0.000 |
| 12x2-w32 | accepted | solver | 5.863 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 34.552 | 0.468 |
| 12x2-w32 | off | G2_arithmetic | 13.735 | 0.193 |
| 12x2-w32 | off | epoch | 5.435 | 0.314 |
| 12x2-w32 | off | fft | 26.874 | -1.282 |
| 12x2-w32 | off | grouping | 1.700 | -0.140 |
| 12x2-w32 | off | inverse | 3.346 | -0.138 |
| 12x2-w32 | off | other | 4.248 | 0.392 |
| 12x2-w32 | off | scheduler | 3.854 | -0.203 |
| 12x2-w32 | off | solver | 6.257 | 0.394 |
| 12x2-w32 | on | G1_arithmetic | 34.357 | 0.273 |
| 12x2-w32 | on | G2_arithmetic | 13.677 | 0.135 |
| 12x2-w32 | on | epoch | 5.267 | 0.147 |
| 12x2-w32 | on | fft | 27.683 | -0.472 |
| 12x2-w32 | on | grouping | 1.776 | -0.064 |
| 12x2-w32 | on | inverse | 3.365 | -0.118 |
| 12x2-w32 | on | other | 3.844 | -0.011 |
| 12x2-w32 | on | scheduler | 4.012 | -0.045 |
| 12x2-w32 | on | solver | 6.019 | 0.156 |
| 1x2-w32 | accepted | G1_arithmetic | 38.143 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.771 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.177 | 0.000 |
| 1x2-w32 | accepted | fft | 4.509 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.612 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.214 | 0.000 |
| 1x2-w32 | accepted | other | 5.127 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.906 | 0.000 |
| 1x2-w32 | accepted | solver | 1.541 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.898 | -0.244 |
| 1x2-w32 | off | G2_arithmetic | 29.254 | -0.517 |
| 1x2-w32 | off | epoch | 9.779 | 0.602 |
| 1x2-w32 | off | fft | 4.685 | 0.177 |
| 1x2-w32 | off | grouping | 0.809 | 0.197 |
| 1x2-w32 | off | inverse | 6.960 | -0.254 |
| 1x2-w32 | off | other | 5.174 | 0.047 |
| 1x2-w32 | off | scheduler | 4.018 | 0.113 |
| 1x2-w32 | off | solver | 1.421 | -0.121 |
| 1x2-w32 | on | G1_arithmetic | 38.209 | 0.066 |
| 1x2-w32 | on | G2_arithmetic | 28.533 | -1.238 |
| 1x2-w32 | on | epoch | 9.534 | 0.356 |
| 1x2-w32 | on | fft | 4.581 | 0.072 |
| 1x2-w32 | on | grouping | 1.034 | 0.422 |
| 1x2-w32 | on | inverse | 7.078 | -0.135 |
| 1x2-w32 | on | other | 4.983 | -0.145 |
| 1x2-w32 | on | scheduler | 4.458 | 0.553 |
| 1x2-w32 | on | solver | 1.590 | 0.049 |
| merge-54x1-w32 | accepted | G1_arithmetic | 24.822 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 16.089 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.318 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.268 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.321 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.887 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.802 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 1.876 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.619 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 26.935 | 2.114 |
| merge-54x1-w32 | off | G2_arithmetic | 12.868 | -3.221 |
| merge-54x1-w32 | off | epoch | 2.320 | 0.002 |
| merge-54x1-w32 | off | fft | 42.822 | 0.554 |
| merge-54x1-w32 | off | grouping | 1.424 | 0.104 |
| merge-54x1-w32 | off | inverse | 1.712 | -0.174 |
| merge-54x1-w32 | off | other | 4.080 | 0.278 |
| merge-54x1-w32 | off | scheduler | 2.276 | 0.400 |
| merge-54x1-w32 | off | solver | 5.563 | -0.056 |
| merge-54x1-w32 | on | G1_arithmetic | 27.137 | 2.316 |
| merge-54x1-w32 | on | G2_arithmetic | 12.926 | -3.163 |
| merge-54x1-w32 | on | epoch | 2.164 | -0.154 |
| merge-54x1-w32 | on | fft | 42.616 | 0.348 |
| merge-54x1-w32 | on | grouping | 1.394 | 0.073 |
| merge-54x1-w32 | on | inverse | 1.697 | -0.190 |
| merge-54x1-w32 | on | other | 3.982 | 0.180 |
| merge-54x1-w32 | on | scheduler | 2.348 | 0.473 |
| merge-54x1-w32 | on | solver | 5.736 | 0.117 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.656291 | 37.000 | 4.274× | 24.000 | 2.773× | false |
| 1x2-w32 | off | 8.572243 | 37.000 | 4.316× | 24.000 | 2.800× | false |
| 1x2-w32 | on | 8.670757 | 37.000 | 4.267× | 24.000 | 2.768× | false |
| 2x2-w32 | accepted | 11.712734 | 59.000 | 5.037× | 30.000 | 2.561× | false |
| 2x2-w32 | off | 11.702034 | 59.000 | 5.042× | 30.000 | 2.564× | false |
| 2x2-w32 | on | 11.639867 | 59.000 | 5.069× | 30.000 | 2.577× | false |
| 4x2-w32 | accepted | 16.109013 | 109.000 | 6.766× | 48.000 | 2.980× | false |
| 4x2-w32 | off | 16.103748 | 109.000 | 6.769× | 48.000 | 2.981× | false |
| 4x2-w32 | on | 16.234542 | 109.000 | 6.714× | 48.000 | 2.957× | false |
| 8x2-w32 | accepted | 24.780247 | 189.000 | 7.627× | 74.000 | 2.986× | false |
| 8x2-w32 | off | 24.391450 | 189.000 | 7.749× | 74.000 | 3.034× | false |
| 8x2-w32 | on | 24.629511 | 189.000 | 7.674× | 74.000 | 3.005× | false |
| 12x2-w32 | accepted | 34.442273 | 284.000 | 8.246× | 101.000 | 2.932× | false |
| 12x2-w32 | off | 34.977863 | 284.000 | 8.119× | 101.000 | 2.888× | false |
| 12x2-w32 | on | 34.610800 | 284.000 | 8.206× | 101.000 | 2.918× | false |
| 16x2-w32 | accepted | 41.031470 | 335.000 | 8.164× | 124.000 | 3.022× | false |
| 16x2-w32 | off | 41.128801 | 335.000 | 8.145× | 124.000 | 3.015× | false |
| 16x2-w32 | on | 41.501937 | 335.000 | 8.072× | 124.000 | 2.988× | false |
| 24x2-w32 | accepted | 68.402157 | 528.000 | 7.719× | 176.000 | 2.573× | false |
| 24x2-w32 | off | 68.582964 | 528.000 | 7.699× | 176.000 | 2.566× | false |
| 24x2-w32 | on | 68.668280 | 528.000 | 7.689× | 176.000 | 2.563× | false |
| merge-24x1-w32 | accepted | 67.369762 | 507.000 | 7.526× | 175.000 | 2.598× | false |
| merge-24x1-w32 | off | 67.321777 | 507.000 | 7.531× | 175.000 | 2.599× | false |
| merge-24x1-w32 | on | 68.015410 | 507.000 | 7.454× | 175.000 | 2.573× | false |

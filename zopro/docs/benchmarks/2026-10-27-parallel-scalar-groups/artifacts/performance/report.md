# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.007873 | 1.000000 | — | — | — |
| 12x2-w32 | candidate | 34.767947 | 1.027144 | — | — | — |
| 12x2-w32 | fresh | 33.665438 | 0.991883 | — | — | — |
| 16x2-w32 | accepted | 40.032346 | 1.000000 | — | — | — |
| 16x2-w32 | candidate | 41.847474 | 1.043493 | — | — | — |
| 16x2-w32 | fresh | 40.027940 | 1.000157 | — | — | — |
| 1x2-w16 | accepted | 10.228112 | 1.000000 | — | — | — |
| 1x2-w16 | candidate | 10.021421 | 0.994565 | — | — | — |
| 1x2-w16 | fresh | 10.085646 | 0.977381 | — | — | — |
| 1x2-w32 | accepted | 8.328096 | 1.000000 | 193.651 | 8.505861 | 137.75 |
| 1x2-w32 | candidate | 8.305753 | 0.998793 | 192.850 | 8.454587 | 83.75 |
| 1x2-w32 | fresh | 8.288431 | 0.995853 | 192.515 | 8.401561 | 132.25 |
| 24x2-w32 | accepted | 67.146308 | 1.000000 | — | — | — |
| 24x2-w32 | candidate | 71.086713 | 1.057226 | — | — | — |
| 24x2-w32 | fresh | 67.254070 | 0.999272 | — | — | — |
| 2x2-w32 | accepted | 11.498399 | 1.000000 | — | — | — |
| 2x2-w32 | candidate | 11.428048 | 0.995448 | — | — | — |
| 2x2-w32 | fresh | 11.387248 | 0.995098 | — | — | — |
| 4x2-w32 | accepted | 16.060417 | 1.000000 | 418.077 | 16.062573 | 371.00 |
| 4x2-w32 | candidate | 15.861382 | 0.987734 | 417.618 | 16.065383 | 334.25 |
| 4x2-w32 | fresh | 15.923239 | 0.992382 | 414.945 | 16.105884 | 366.75 |
| 8x2-w32 | accepted | 24.051948 | 1.000000 | — | — | — |
| 8x2-w32 | candidate | 24.602121 | 1.025162 | — | — | — |
| 8x2-w32 | fresh | 24.183456 | 1.001522 | — | — | — |
| merge-24x1-w32 | accepted | 65.828410 | 1.000000 | — | — | — |
| merge-24x1-w32 | candidate | 69.280526 | 1.054183 | — | — | — |
| merge-24x1-w32 | fresh | 66.010879 | 1.001313 | — | — | — |
| merge-54x1-w32 | accepted | 163.467660 | 1.000000 | — | — | — |
| merge-54x1-w32 | candidate | 176.044468 | 1.078056 | — | — | — |
| merge-54x1-w32 | fresh | 163.338347 | 1.001383 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 35.232 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 30.184 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.953 | 0.000 |
| 1x2-w32 | accepted | fft | 5.716 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.688 | 0.000 |
| 1x2-w32 | accepted | inverse | 8.006 | 0.000 |
| 1x2-w32 | accepted | other | 4.918 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.758 | 0.000 |
| 1x2-w32 | accepted | solver | 1.544 | 0.000 |
| 1x2-w32 | candidate | G1_arithmetic | 36.014 | 0.782 |
| 1x2-w32 | candidate | G2_arithmetic | 30.481 | 0.297 |
| 1x2-w32 | candidate | epoch | 10.456 | 0.503 |
| 1x2-w32 | candidate | fft | 4.166 | -1.550 |
| 1x2-w32 | candidate | grouping | 0.035 | -0.653 |
| 1x2-w32 | candidate | inverse | 6.789 | -1.217 |
| 1x2-w32 | candidate | other | 6.283 | 1.365 |
| 1x2-w32 | candidate | scheduler | 4.424 | 0.666 |
| 1x2-w32 | candidate | solver | 1.351 | -0.193 |
| 1x2-w32 | fresh | G1_arithmetic | 35.952 | 0.720 |
| 1x2-w32 | fresh | G2_arithmetic | 29.972 | -0.212 |
| 1x2-w32 | fresh | epoch | 9.538 | -0.415 |
| 1x2-w32 | fresh | fft | 5.237 | -0.479 |
| 1x2-w32 | fresh | grouping | 0.588 | -0.100 |
| 1x2-w32 | fresh | inverse | 7.874 | -0.132 |
| 1x2-w32 | fresh | other | 5.144 | 0.226 |
| 1x2-w32 | fresh | scheduler | 3.729 | -0.029 |
| 1x2-w32 | fresh | solver | 1.965 | 0.421 |
| 4x2-w32 | accepted | G1_arithmetic | 35.362 | 0.000 |
| 4x2-w32 | accepted | G2_arithmetic | 18.420 | 0.000 |
| 4x2-w32 | accepted | epoch | 10.499 | 0.000 |
| 4x2-w32 | accepted | fft | 14.444 | 0.000 |
| 4x2-w32 | accepted | grouping | 1.447 | 0.000 |
| 4x2-w32 | accepted | inverse | 4.925 | 0.000 |
| 4x2-w32 | accepted | other | 6.388 | 0.000 |
| 4x2-w32 | accepted | scheduler | 4.918 | 0.000 |
| 4x2-w32 | accepted | solver | 3.597 | 0.000 |
| 4x2-w32 | candidate | G1_arithmetic | 34.808 | -0.554 |
| 4x2-w32 | candidate | G2_arithmetic | 19.376 | 0.956 |
| 4x2-w32 | candidate | epoch | 10.725 | 0.226 |
| 4x2-w32 | candidate | fft | 14.323 | -0.121 |
| 4x2-w32 | candidate | grouping | 0.668 | -0.779 |
| 4x2-w32 | candidate | inverse | 4.092 | -0.833 |
| 4x2-w32 | candidate | other | 7.967 | 1.578 |
| 4x2-w32 | candidate | scheduler | 5.110 | 0.193 |
| 4x2-w32 | candidate | solver | 2.930 | -0.667 |
| 4x2-w32 | fresh | G1_arithmetic | 34.634 | -0.728 |
| 4x2-w32 | fresh | G2_arithmetic | 18.284 | -0.136 |
| 4x2-w32 | fresh | epoch | 10.579 | 0.079 |
| 4x2-w32 | fresh | fft | 14.478 | 0.034 |
| 4x2-w32 | fresh | grouping | 1.329 | -0.118 |
| 4x2-w32 | fresh | inverse | 5.043 | 0.119 |
| 4x2-w32 | fresh | other | 6.444 | 0.055 |
| 4x2-w32 | fresh | scheduler | 5.183 | 0.266 |
| 4x2-w32 | fresh | solver | 4.026 | 0.428 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.328096 | 37.000 | 4.443× | 24.000 | 2.882× | false |
| 1x2-w32 | fresh | 8.288431 | 37.000 | 4.464× | 24.000 | 2.896× | false |
| 1x2-w32 | candidate | 8.305753 | 37.000 | 4.455× | 24.000 | 2.890× | false |
| 2x2-w32 | accepted | 11.498399 | 59.000 | 5.131× | 30.000 | 2.609× | false |
| 2x2-w32 | fresh | 11.387248 | 59.000 | 5.181× | 30.000 | 2.635× | false |
| 2x2-w32 | candidate | 11.428048 | 59.000 | 5.163× | 30.000 | 2.625× | false |
| 4x2-w32 | accepted | 16.060417 | 109.000 | 6.787× | 48.000 | 2.989× | false |
| 4x2-w32 | fresh | 15.923239 | 109.000 | 6.845× | 48.000 | 3.014× | false |
| 4x2-w32 | candidate | 15.861382 | 109.000 | 6.872× | 48.000 | 3.026× | false |
| 8x2-w32 | accepted | 24.051948 | 189.000 | 7.858× | 74.000 | 3.077× | false |
| 8x2-w32 | fresh | 24.183456 | 189.000 | 7.815× | 74.000 | 3.060× | false |
| 8x2-w32 | candidate | 24.602121 | 189.000 | 7.682× | 74.000 | 3.008× | false |
| 12x2-w32 | accepted | 34.007873 | 284.000 | 8.351× | 101.000 | 2.970× | false |
| 12x2-w32 | fresh | 33.665438 | 284.000 | 8.436× | 101.000 | 3.000× | false |
| 12x2-w32 | candidate | 34.767947 | 284.000 | 8.168× | 101.000 | 2.905× | false |
| 16x2-w32 | accepted | 40.032346 | 335.000 | 8.368× | 124.000 | 3.097× | false |
| 16x2-w32 | fresh | 40.027940 | 335.000 | 8.369× | 124.000 | 3.098× | false |
| 16x2-w32 | candidate | 41.847474 | 335.000 | 8.005× | 124.000 | 2.963× | false |
| 24x2-w32 | accepted | 67.146308 | 528.000 | 7.863× | 176.000 | 2.621× | false |
| 24x2-w32 | fresh | 67.254070 | 528.000 | 7.851× | 176.000 | 2.617× | false |
| 24x2-w32 | candidate | 71.086713 | 528.000 | 7.428× | 176.000 | 2.476× | false |
| merge-24x1-w32 | accepted | 65.828410 | 507.000 | 7.702× | 175.000 | 2.658× | false |
| merge-24x1-w32 | fresh | 66.010879 | 507.000 | 7.681× | 175.000 | 2.651× | false |
| merge-24x1-w32 | candidate | 69.280526 | 507.000 | 7.318× | 175.000 | 2.526× | false |
| 1x2-w16 | accepted | 10.228112 | 37.000 | 3.617× | 24.000 | 2.346× | false |
| 1x2-w16 | fresh | 10.085646 | 37.000 | 3.669× | 24.000 | 2.380× | false |
| 1x2-w16 | candidate | 10.021421 | 37.000 | 3.692× | 24.000 | 2.395× | false |

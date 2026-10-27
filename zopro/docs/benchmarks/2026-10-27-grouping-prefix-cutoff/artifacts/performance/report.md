# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 33.784085 | 1.000000 | — | — | — |
| 12x2-w32 | candidate | 33.888183 | 1.001029 | — | — | — |
| 12x2-w32 | fresh | 34.454529 | 1.014515 | — | — | — |
| 16x2-w32 | accepted | 40.068185 | 1.000000 | — | — | — |
| 16x2-w32 | candidate | 40.035247 | 0.994969 | — | — | — |
| 16x2-w32 | fresh | 40.842221 | 1.022198 | — | — | — |
| 1x2-w16 | accepted | 10.289343 | 1.000000 | — | — | — |
| 1x2-w16 | candidate | 10.233569 | 1.007121 | — | — | — |
| 1x2-w16 | fresh | 10.301116 | 1.024121 | — | — | — |
| 1x2-w32 | accepted | 8.325779 | 1.000000 | 192.414 | 8.478925 | 133.50 |
| 1x2-w32 | candidate | 8.330548 | 0.999520 | 192.815 | 8.523107 | 129.50 |
| 1x2-w32 | fresh | 8.424946 | 1.012084 | 194.417 | 8.548357 | 133.00 |
| 24x2-w32 | accepted | 67.277087 | 1.000000 | — | — | — |
| 24x2-w32 | candidate | 67.000664 | 0.999353 | — | — | — |
| 24x2-w32 | fresh | 67.381993 | 1.005999 | — | — | — |
| 2x2-w32 | accepted | 11.404087 | 1.000000 | — | — | — |
| 2x2-w32 | candidate | 11.495481 | 1.002554 | — | — | — |
| 2x2-w32 | fresh | 11.520316 | 1.012596 | — | — | — |
| 4x2-w32 | accepted | 15.950434 | 1.000000 | 416.424 | 16.079124 | 367.00 |
| 4x2-w32 | candidate | 15.891183 | 0.998251 | 417.286 | 16.084560 | 381.25 |
| 4x2-w32 | fresh | 16.181622 | 1.015206 | 418.742 | 16.159381 | 365.00 |
| 8x2-w32 | accepted | 24.138594 | 1.000000 | — | — | — |
| 8x2-w32 | candidate | 24.023161 | 0.997362 | — | — | — |
| 8x2-w32 | fresh | 24.783831 | 1.030054 | — | — | — |
| merge-24x1-w32 | accepted | 66.225500 | 1.000000 | — | — | — |
| merge-24x1-w32 | candidate | 66.230441 | 0.996635 | — | — | — |
| merge-24x1-w32 | fresh | 66.288750 | 1.000497 | — | — | — |
| merge-54x1-w32 | accepted | 163.636011 | 1.000000 | — | — | — |
| merge-54x1-w32 | candidate | 163.267591 | 0.999352 | — | — | — |
| merge-54x1-w32 | fresh | 164.219983 | 1.002013 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 36.527 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.668 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.147 | 0.000 |
| 1x2-w32 | accepted | fft | 4.764 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.418 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.148 | 0.000 |
| 1x2-w32 | accepted | other | 6.763 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.839 | 0.000 |
| 1x2-w32 | accepted | solver | 1.727 | 0.000 |
| 1x2-w32 | candidate | G1_arithmetic | 36.958 | 0.431 |
| 1x2-w32 | candidate | G2_arithmetic | 30.656 | 1.988 |
| 1x2-w32 | candidate | epoch | 8.663 | -0.484 |
| 1x2-w32 | candidate | fft | 5.726 | 0.962 |
| 1x2-w32 | candidate | grouping | 0.592 | 0.174 |
| 1x2-w32 | candidate | inverse | 7.108 | -0.040 |
| 1x2-w32 | candidate | other | 5.571 | -1.191 |
| 1x2-w32 | candidate | scheduler | 3.493 | -1.346 |
| 1x2-w32 | candidate | solver | 1.232 | -0.495 |
| 1x2-w32 | fresh | G1_arithmetic | 37.156 | 0.629 |
| 1x2-w32 | fresh | G2_arithmetic | 30.973 | 2.305 |
| 1x2-w32 | fresh | epoch | 8.765 | -0.381 |
| 1x2-w32 | fresh | fft | 3.632 | -1.132 |
| 1x2-w32 | fresh | grouping | 0.806 | 0.388 |
| 1x2-w32 | fresh | inverse | 7.546 | 0.398 |
| 1x2-w32 | fresh | other | 5.214 | -1.549 |
| 1x2-w32 | fresh | scheduler | 4.498 | -0.341 |
| 1x2-w32 | fresh | solver | 1.409 | -0.317 |
| 4x2-w32 | accepted | G1_arithmetic | 34.973 | 0.000 |
| 4x2-w32 | accepted | G2_arithmetic | 19.049 | 0.000 |
| 4x2-w32 | accepted | epoch | 10.539 | 0.000 |
| 4x2-w32 | accepted | fft | 14.262 | 0.000 |
| 4x2-w32 | accepted | grouping | 1.525 | 0.000 |
| 4x2-w32 | accepted | inverse | 4.275 | 0.000 |
| 4x2-w32 | accepted | other | 6.369 | 0.000 |
| 4x2-w32 | accepted | scheduler | 5.359 | 0.000 |
| 4x2-w32 | accepted | solver | 3.649 | 0.000 |
| 4x2-w32 | candidate | G1_arithmetic | 34.766 | -0.207 |
| 4x2-w32 | candidate | G2_arithmetic | 19.004 | -0.045 |
| 4x2-w32 | candidate | epoch | 10.900 | 0.361 |
| 4x2-w32 | candidate | fft | 14.639 | 0.377 |
| 4x2-w32 | candidate | grouping | 1.485 | -0.040 |
| 4x2-w32 | candidate | inverse | 4.114 | -0.161 |
| 4x2-w32 | candidate | other | 6.336 | -0.032 |
| 4x2-w32 | candidate | scheduler | 5.219 | -0.140 |
| 4x2-w32 | candidate | solver | 3.537 | -0.112 |
| 4x2-w32 | fresh | G1_arithmetic | 33.820 | -1.153 |
| 4x2-w32 | fresh | G2_arithmetic | 19.618 | 0.569 |
| 4x2-w32 | fresh | epoch | 10.748 | 0.208 |
| 4x2-w32 | fresh | fft | 15.048 | 0.787 |
| 4x2-w32 | fresh | grouping | 1.568 | 0.043 |
| 4x2-w32 | fresh | inverse | 4.353 | 0.077 |
| 4x2-w32 | fresh | other | 6.524 | 0.155 |
| 4x2-w32 | fresh | scheduler | 4.955 | -0.404 |
| 4x2-w32 | fresh | solver | 3.366 | -0.283 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.325779 | 37.000 | 4.444× | 24.000 | 2.883× | false |
| 1x2-w32 | fresh | 8.424946 | 37.000 | 4.392× | 24.000 | 2.849× | false |
| 1x2-w32 | candidate | 8.330548 | 37.000 | 4.441× | 24.000 | 2.881× | false |
| 2x2-w32 | accepted | 11.404087 | 59.000 | 5.174× | 30.000 | 2.631× | false |
| 2x2-w32 | fresh | 11.520316 | 59.000 | 5.121× | 30.000 | 2.604× | false |
| 2x2-w32 | candidate | 11.495481 | 59.000 | 5.132× | 30.000 | 2.610× | false |
| 4x2-w32 | accepted | 15.950434 | 109.000 | 6.834× | 48.000 | 3.009× | false |
| 4x2-w32 | fresh | 16.181622 | 109.000 | 6.736× | 48.000 | 2.966× | false |
| 4x2-w32 | candidate | 15.891183 | 109.000 | 6.859× | 48.000 | 3.021× | false |
| 8x2-w32 | accepted | 24.138594 | 189.000 | 7.830× | 74.000 | 3.066× | false |
| 8x2-w32 | fresh | 24.783831 | 189.000 | 7.626× | 74.000 | 2.986× | false |
| 8x2-w32 | candidate | 24.023161 | 189.000 | 7.867× | 74.000 | 3.080× | false |
| 12x2-w32 | accepted | 33.784085 | 284.000 | 8.406× | 101.000 | 2.990× | false |
| 12x2-w32 | fresh | 34.454529 | 284.000 | 8.243× | 101.000 | 2.931× | false |
| 12x2-w32 | candidate | 33.888183 | 284.000 | 8.381× | 101.000 | 2.980× | false |
| 16x2-w32 | accepted | 40.068185 | 335.000 | 8.361× | 124.000 | 3.095× | false |
| 16x2-w32 | fresh | 40.842221 | 335.000 | 8.202× | 124.000 | 3.036× | false |
| 16x2-w32 | candidate | 40.035247 | 335.000 | 8.368× | 124.000 | 3.097× | false |
| 24x2-w32 | accepted | 67.277087 | 528.000 | 7.848× | 176.000 | 2.616× | false |
| 24x2-w32 | fresh | 67.381993 | 528.000 | 7.836× | 176.000 | 2.612× | false |
| 24x2-w32 | candidate | 67.000664 | 528.000 | 7.881× | 176.000 | 2.627× | false |
| merge-24x1-w32 | accepted | 66.225500 | 507.000 | 7.656× | 175.000 | 2.642× | false |
| merge-24x1-w32 | fresh | 66.288750 | 507.000 | 7.648× | 175.000 | 2.640× | false |
| merge-24x1-w32 | candidate | 66.230441 | 507.000 | 7.655× | 175.000 | 2.642× | false |
| 1x2-w16 | accepted | 10.289343 | 37.000 | 3.596× | 24.000 | 2.333× | false |
| 1x2-w16 | fresh | 10.301116 | 37.000 | 3.592× | 24.000 | 2.330× | false |
| 1x2-w16 | candidate | 10.233569 | 37.000 | 3.616× | 24.000 | 2.345× | false |

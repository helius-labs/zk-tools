# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w16 | accepted | 10.590246 | 1.000000 | — | — | — |
| 1x2-w16 | off | 12.129369 | 1.010403 | — | — | — |
| 1x2-w16 | on | 11.349478 | 1.071692 | — | — | — |
| 1x2-w32 | accepted | 8.598141 | 1.000000 | 199.768 | 8.691512 | 129.00 |
| 1x2-w32 | off | 8.642184 | 1.004761 | 198.946 | 8.730427 | 127.75 |
| 1x2-w32 | on | 8.544290 | 0.993840 | 198.325 | 8.689302 | 130.75 |
| 2x2-w32 | accepted | 11.593448 | 1.000000 | 273.002 | 11.750754 | 313.50 |
| 2x2-w32 | off | 11.708554 | 1.005480 | 272.491 | 11.733948 | 306.75 |
| 2x2-w32 | on | 11.606481 | 0.996478 | 268.470 | 11.717929 | 309.50 |
| merge-54x1-w32 | accepted | 177.372180 | 1.000000 | 4929.852 | 178.958347 | 1585.85 |
| merge-54x1-w32 | off | 177.145223 | 1.000577 | 4895.575 | 176.952850 | 1572.39 |
| merge-54x1-w32 | on | 176.377783 | 0.997651 | 4896.098 | 178.255192 | 1697.62 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.812 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.719 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.760 | 0.000 |
| 1x2-w32 | accepted | fft | 4.360 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.823 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.838 | 0.000 |
| 1x2-w32 | accepted | other | 4.900 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.417 | 0.000 |
| 1x2-w32 | accepted | solver | 1.371 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.205 | -0.607 |
| 1x2-w32 | off | G2_arithmetic | 29.154 | -0.565 |
| 1x2-w32 | off | epoch | 9.320 | -0.440 |
| 1x2-w32 | off | fft | 4.746 | 0.386 |
| 1x2-w32 | off | grouping | 0.765 | -0.058 |
| 1x2-w32 | off | inverse | 6.922 | 0.084 |
| 1x2-w32 | off | other | 4.880 | -0.020 |
| 1x2-w32 | off | scheduler | 4.491 | 1.074 |
| 1x2-w32 | off | solver | 1.517 | 0.146 |
| 1x2-w32 | on | G1_arithmetic | 38.726 | -0.086 |
| 1x2-w32 | on | G2_arithmetic | 28.687 | -1.032 |
| 1x2-w32 | on | epoch | 9.955 | 0.195 |
| 1x2-w32 | on | fft | 5.598 | 1.239 |
| 1x2-w32 | on | grouping | 0.584 | -0.239 |
| 1x2-w32 | on | inverse | 7.178 | 0.340 |
| 1x2-w32 | on | other | 5.358 | 0.459 |
| 1x2-w32 | on | scheduler | 2.296 | -1.121 |
| 1x2-w32 | on | solver | 1.617 | 0.246 |
| 2x2-w32 | accepted | G1_arithmetic | 33.402 | 0.000 |
| 2x2-w32 | accepted | G2_arithmetic | 24.367 | 0.000 |
| 2x2-w32 | accepted | epoch | 10.807 | 0.000 |
| 2x2-w32 | accepted | fft | 9.774 | 0.000 |
| 2x2-w32 | accepted | grouping | 1.510 | 0.000 |
| 2x2-w32 | accepted | inverse | 6.675 | 0.000 |
| 2x2-w32 | accepted | other | 6.157 | 0.000 |
| 2x2-w32 | accepted | scheduler | 4.816 | 0.000 |
| 2x2-w32 | accepted | solver | 2.492 | 0.000 |
| 2x2-w32 | off | G1_arithmetic | 35.148 | 1.746 |
| 2x2-w32 | off | G2_arithmetic | 24.150 | -0.216 |
| 2x2-w32 | off | epoch | 10.996 | 0.188 |
| 2x2-w32 | off | fft | 9.085 | -0.690 |
| 2x2-w32 | off | grouping | 1.372 | -0.138 |
| 2x2-w32 | off | inverse | 6.466 | -0.209 |
| 2x2-w32 | off | other | 5.591 | -0.566 |
| 2x2-w32 | off | scheduler | 4.750 | -0.066 |
| 2x2-w32 | off | solver | 2.442 | -0.050 |
| 2x2-w32 | on | G1_arithmetic | 34.746 | 1.344 |
| 2x2-w32 | on | G2_arithmetic | 24.014 | -0.352 |
| 2x2-w32 | on | epoch | 10.842 | 0.035 |
| 2x2-w32 | on | fft | 10.642 | 0.868 |
| 2x2-w32 | on | grouping | 1.429 | -0.080 |
| 2x2-w32 | on | inverse | 7.012 | 0.337 |
| 2x2-w32 | on | other | 5.713 | -0.444 |
| 2x2-w32 | on | scheduler | 3.133 | -1.683 |
| 2x2-w32 | on | solver | 2.468 | -0.024 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.492 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 15.932 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.200 | 0.000 |
| merge-54x1-w32 | accepted | fft | 41.284 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.420 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.860 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.598 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.004 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.211 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 25.674 | -0.817 |
| merge-54x1-w32 | off | G2_arithmetic | 15.580 | -0.352 |
| merge-54x1-w32 | off | epoch | 2.105 | -0.095 |
| merge-54x1-w32 | off | fft | 41.724 | 0.440 |
| merge-54x1-w32 | off | grouping | 1.406 | -0.014 |
| merge-54x1-w32 | off | inverse | 1.692 | -0.168 |
| merge-54x1-w32 | off | other | 3.870 | 0.272 |
| merge-54x1-w32 | off | scheduler | 2.052 | 0.048 |
| merge-54x1-w32 | off | solver | 5.897 | 0.687 |
| merge-54x1-w32 | on | G1_arithmetic | 25.122 | -1.370 |
| merge-54x1-w32 | on | G2_arithmetic | 16.666 | 0.734 |
| merge-54x1-w32 | on | epoch | 2.033 | -0.167 |
| merge-54x1-w32 | on | fft | 42.530 | 1.246 |
| merge-54x1-w32 | on | grouping | 1.363 | -0.057 |
| merge-54x1-w32 | on | inverse | 1.681 | -0.179 |
| merge-54x1-w32 | on | other | 4.109 | 0.510 |
| merge-54x1-w32 | on | scheduler | 1.621 | -0.383 |
| merge-54x1-w32 | on | solver | 4.876 | -0.334 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.598141 | 37.000 | 4.303× | 24.000 | 2.791× | false |
| 1x2-w32 | off | 8.642184 | 37.000 | 4.281× | 24.000 | 2.777× | false |
| 1x2-w32 | on | 8.544290 | 37.000 | 4.330× | 24.000 | 2.809× | false |
| 2x2-w32 | accepted | 11.593448 | 59.000 | 5.089× | 30.000 | 2.588× | false |
| 2x2-w32 | off | 11.708554 | 59.000 | 5.039× | 30.000 | 2.562× | false |
| 2x2-w32 | on | 11.606481 | 59.000 | 5.083× | 30.000 | 2.585× | false |

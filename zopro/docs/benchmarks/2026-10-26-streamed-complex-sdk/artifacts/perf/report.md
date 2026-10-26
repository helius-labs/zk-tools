# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.567235 | 1.000000 | 965.295 | 34.832441 | 728.25 |
| 12x2-w32 | off | 34.520765 | 0.997370 | 956.955 | 34.332338 | 709.75 |
| 12x2-w32 | on | 35.248192 | 1.023086 | 962.984 | 35.358033 | 730.00 |
| 16x2-w32 | accepted | 41.199613 | 1.000000 | — | — | — |
| 16x2-w32 | off | 41.085516 | 1.000826 | — | — | — |
| 16x2-w32 | on | 41.905536 | 1.027296 | — | — | — |
| 1x2-w16 | accepted | 10.769231 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.661376 | 0.989042 | — | — | — |
| 1x2-w16 | on | 11.242183 | 1.037427 | — | — | — |
| 1x2-w32 | accepted | 8.652529 | 1.000000 | 199.524 | 8.813370 | 126.00 |
| 1x2-w32 | off | 8.674601 | 0.993941 | 199.840 | 8.815417 | 128.75 |
| 1x2-w32 | on | 9.063745 | 1.047526 | 209.664 | 9.190066 | 130.00 |
| 24x2-w32 | accepted | 69.268377 | 1.000000 | — | — | — |
| 24x2-w32 | off | 69.022089 | 1.001890 | — | — | — |
| 24x2-w32 | on | 70.644302 | 1.029315 | — | — | — |
| 2x2-w32 | accepted | 11.808991 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.789546 | 1.003783 | — | — | — |
| 2x2-w32 | on | 12.040880 | 1.024628 | — | — | — |
| 4x2-w32 | accepted | 16.377227 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.176368 | 0.991247 | — | — | — |
| 4x2-w32 | on | 16.748528 | 1.025691 | — | — | — |
| 8x2-w32 | accepted | 24.731095 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.764446 | 1.010690 | — | — | — |
| 8x2-w32 | on | 25.820529 | 1.052615 | — | — | — |
| merge-24x1-w32 | accepted | 67.832392 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.665609 | 1.013990 | — | — | — |
| merge-24x1-w32 | on | 68.508132 | 1.020701 | — | — | — |
| merge-54x1-w32 | accepted | 177.767922 | 1.000000 | 4801.486 | 173.371087 | 1658.52 |
| merge-54x1-w32 | off | 175.332904 | 0.990907 | 4887.248 | 176.941780 | 1648.93 |
| merge-54x1-w32 | on | 178.935113 | 1.006566 | 4941.691 | 177.222872 | 1616.48 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.363 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.873 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.348 | 0.000 |
| 12x2-w32 | accepted | fft | 27.226 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.847 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.225 | 0.000 |
| 12x2-w32 | accepted | other | 4.049 | 0.000 |
| 12x2-w32 | accepted | scheduler | 3.846 | 0.000 |
| 12x2-w32 | accepted | solver | 6.222 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 34.294 | -0.069 |
| 12x2-w32 | off | G2_arithmetic | 13.912 | 0.039 |
| 12x2-w32 | off | epoch | 4.988 | -0.360 |
| 12x2-w32 | off | fft | 28.048 | 0.822 |
| 12x2-w32 | off | grouping | 1.912 | 0.065 |
| 12x2-w32 | off | inverse | 3.458 | 0.233 |
| 12x2-w32 | off | other | 3.880 | -0.170 |
| 12x2-w32 | off | scheduler | 3.773 | -0.073 |
| 12x2-w32 | off | solver | 5.737 | -0.485 |
| 12x2-w32 | on | G1_arithmetic | 34.710 | 0.347 |
| 12x2-w32 | on | G2_arithmetic | 14.823 | 0.950 |
| 12x2-w32 | on | epoch | 5.148 | -0.200 |
| 12x2-w32 | on | fft | 26.432 | -0.795 |
| 12x2-w32 | on | grouping | 1.887 | 0.040 |
| 12x2-w32 | on | inverse | 3.332 | 0.107 |
| 12x2-w32 | on | other | 3.901 | -0.148 |
| 12x2-w32 | on | scheduler | 3.848 | 0.002 |
| 12x2-w32 | on | solver | 5.919 | -0.304 |
| 1x2-w32 | accepted | G1_arithmetic | 39.016 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.939 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.901 | 0.000 |
| 1x2-w32 | accepted | fft | 4.417 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.551 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.512 | 0.000 |
| 1x2-w32 | accepted | other | 4.552 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.679 | 0.000 |
| 1x2-w32 | accepted | solver | 1.433 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.504 | -0.511 |
| 1x2-w32 | off | G2_arithmetic | 29.154 | -0.785 |
| 1x2-w32 | off | epoch | 10.178 | 0.277 |
| 1x2-w32 | off | fft | 4.040 | -0.378 |
| 1x2-w32 | off | grouping | 0.698 | 0.147 |
| 1x2-w32 | off | inverse | 7.250 | 0.738 |
| 1x2-w32 | off | other | 4.986 | 0.434 |
| 1x2-w32 | off | scheduler | 3.641 | -0.038 |
| 1x2-w32 | off | solver | 1.549 | 0.116 |
| 1x2-w32 | on | G1_arithmetic | 37.372 | -1.644 |
| 1x2-w32 | on | G2_arithmetic | 31.203 | 1.264 |
| 1x2-w32 | on | epoch | 9.648 | -0.253 |
| 1x2-w32 | on | fft | 4.559 | 0.142 |
| 1x2-w32 | on | grouping | 0.541 | -0.010 |
| 1x2-w32 | on | inverse | 6.561 | 0.049 |
| 1x2-w32 | on | other | 4.415 | -0.137 |
| 1x2-w32 | on | scheduler | 4.178 | 0.499 |
| 1x2-w32 | on | solver | 1.523 | 0.090 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.615 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 14.189 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 1.883 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.491 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.374 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.807 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.864 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.363 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.414 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 26.491 | -0.124 |
| merge-54x1-w32 | off | G2_arithmetic | 14.374 | 0.185 |
| merge-54x1-w32 | off | epoch | 1.909 | 0.026 |
| merge-54x1-w32 | off | fft | 41.971 | -0.520 |
| merge-54x1-w32 | off | grouping | 1.420 | 0.046 |
| merge-54x1-w32 | off | inverse | 1.675 | -0.131 |
| merge-54x1-w32 | off | other | 4.406 | 0.541 |
| merge-54x1-w32 | off | scheduler | 2.039 | -0.324 |
| merge-54x1-w32 | off | solver | 5.715 | 0.301 |
| merge-54x1-w32 | on | G1_arithmetic | 26.241 | -0.374 |
| merge-54x1-w32 | on | G2_arithmetic | 14.678 | 0.489 |
| merge-54x1-w32 | on | epoch | 2.194 | 0.311 |
| merge-54x1-w32 | on | fft | 42.083 | -0.408 |
| merge-54x1-w32 | on | grouping | 1.304 | -0.070 |
| merge-54x1-w32 | on | inverse | 1.654 | -0.153 |
| merge-54x1-w32 | on | other | 4.114 | 0.250 |
| merge-54x1-w32 | on | scheduler | 2.218 | -0.145 |
| merge-54x1-w32 | on | solver | 5.516 | 0.102 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.652529 | 37.000 | 4.276× | 24.000 | 2.774× | false |
| 1x2-w32 | off | 8.674601 | 37.000 | 4.265× | 24.000 | 2.767× | false |
| 1x2-w32 | on | 9.063745 | 37.000 | 4.082× | 24.000 | 2.648× | false |
| 2x2-w32 | accepted | 11.808991 | 59.000 | 4.996× | 30.000 | 2.540× | false |
| 2x2-w32 | off | 11.789546 | 59.000 | 5.004× | 30.000 | 2.545× | false |
| 2x2-w32 | on | 12.040880 | 59.000 | 4.900× | 30.000 | 2.492× | false |
| 4x2-w32 | accepted | 16.377227 | 109.000 | 6.656× | 48.000 | 2.931× | false |
| 4x2-w32 | off | 16.176368 | 109.000 | 6.738× | 48.000 | 2.967× | false |
| 4x2-w32 | on | 16.748528 | 109.000 | 6.508× | 48.000 | 2.866× | false |
| 8x2-w32 | accepted | 24.731095 | 189.000 | 7.642× | 74.000 | 2.992× | false |
| 8x2-w32 | off | 24.764446 | 189.000 | 7.632× | 74.000 | 2.988× | false |
| 8x2-w32 | on | 25.820529 | 189.000 | 7.320× | 74.000 | 2.866× | false |
| 12x2-w32 | accepted | 34.567235 | 284.000 | 8.216× | 101.000 | 2.922× | false |
| 12x2-w32 | off | 34.520765 | 284.000 | 8.227× | 101.000 | 2.926× | false |
| 12x2-w32 | on | 35.248192 | 284.000 | 8.057× | 101.000 | 2.865× | false |
| 16x2-w32 | accepted | 41.199613 | 335.000 | 8.131× | 124.000 | 3.010× | false |
| 16x2-w32 | off | 41.085516 | 335.000 | 8.154× | 124.000 | 3.018× | false |
| 16x2-w32 | on | 41.905536 | 335.000 | 7.994× | 124.000 | 2.959× | false |
| 24x2-w32 | accepted | 69.268377 | 528.000 | 7.623× | 176.000 | 2.541× | false |
| 24x2-w32 | off | 69.022089 | 528.000 | 7.650× | 176.000 | 2.550× | false |
| 24x2-w32 | on | 70.644302 | 528.000 | 7.474× | 176.000 | 2.491× | false |
| merge-24x1-w32 | accepted | 67.832392 | 507.000 | 7.474× | 175.000 | 2.580× | false |
| merge-24x1-w32 | off | 68.665609 | 507.000 | 7.384× | 175.000 | 2.549× | false |
| merge-24x1-w32 | on | 68.508132 | 507.000 | 7.401× | 175.000 | 2.554× | false |

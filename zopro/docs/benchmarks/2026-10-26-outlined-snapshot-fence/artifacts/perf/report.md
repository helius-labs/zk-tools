# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.556643 | 1.000000 | 949.953 | 34.592578 | 736.25 |
| 12x2-w32 | inlined | 35.060773 | 1.007256 | 946.383 | 34.891988 | 707.25 |
| 12x2-w32 | off | 34.558356 | 0.992766 | 958.810 | 34.450422 | 725.25 |
| 12x2-w32 | on | 34.297123 | 0.992716 | 977.825 | 35.824245 | 730.00 |
| 16x2-w32 | accepted | 40.949175 | 1.000000 | — | — | — |
| 16x2-w32 | inlined | 41.490004 | 1.016894 | — | — | — |
| 16x2-w32 | off | 41.389176 | 1.010745 | — | — | — |
| 16x2-w32 | on | 40.780466 | 0.997208 | — | — | — |
| 1x2-w16 | accepted | 11.063235 | 1.000000 | — | — | — |
| 1x2-w16 | inlined | 10.580904 | 0.973230 | — | — | — |
| 1x2-w16 | off | 10.888986 | 1.031438 | — | — | — |
| 1x2-w16 | on | 10.492307 | 0.956048 | — | — | — |
| 1x2-w32 | accepted | 8.622709 | 1.000000 | 199.168 | 8.912247 | 126.50 |
| 1x2-w32 | inlined | 8.601599 | 1.001149 | 194.289 | 8.801291 | 132.25 |
| 1x2-w32 | off | 8.663186 | 1.008003 | 199.888 | 8.709851 | 128.75 |
| 1x2-w32 | on | 8.638950 | 1.001884 | 193.304 | 8.664477 | 128.50 |
| 24x2-w32 | accepted | 68.358811 | 1.000000 | — | — | — |
| 24x2-w32 | inlined | 70.479302 | 1.025669 | — | — | — |
| 24x2-w32 | off | 68.537127 | 1.003719 | — | — | — |
| 24x2-w32 | on | 69.289319 | 1.013812 | — | — | — |
| 2x2-w32 | accepted | 11.769607 | 1.000000 | — | — | — |
| 2x2-w32 | inlined | 11.887239 | 1.014527 | — | — | — |
| 2x2-w32 | off | 11.669077 | 0.995309 | — | — | — |
| 2x2-w32 | on | 11.786359 | 1.001423 | — | — | — |
| 4x2-w32 | accepted | 16.411706 | 1.000000 | — | — | — |
| 4x2-w32 | inlined | 16.611514 | 1.014812 | — | — | — |
| 4x2-w32 | off | 16.100078 | 0.982788 | — | — | — |
| 4x2-w32 | on | 16.379525 | 1.000431 | — | — | — |
| 8x2-w32 | accepted | 24.476924 | 1.000000 | — | — | — |
| 8x2-w32 | inlined | 25.194868 | 1.018654 | — | — | — |
| 8x2-w32 | off | 24.942502 | 1.004659 | — | — | — |
| 8x2-w32 | on | 25.157607 | 1.012435 | — | — | — |
| merge-24x1-w32 | accepted | 67.243789 | 1.000000 | — | — | — |
| merge-24x1-w32 | inlined | 68.823048 | 1.016552 | — | — | — |
| merge-24x1-w32 | off | 67.866942 | 1.007620 | — | — | — |
| merge-24x1-w32 | on | 69.319626 | 1.030870 | — | — | — |
| merge-54x1-w32 | accepted | 176.364533 | 1.000000 | 4897.727 | 178.119553 | 1641.61 |
| merge-54x1-w32 | inlined | 178.977861 | 1.015762 | 4925.202 | 179.597281 | 1697.82 |
| merge-54x1-w32 | off | 176.496112 | 1.001168 | 4743.520 | 171.437211 | 1634.58 |
| merge-54x1-w32 | on | 179.274789 | 1.016793 | 4897.932 | 178.206438 | 1629.88 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.406 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.546 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.643 | 0.000 |
| 12x2-w32 | accepted | fft | 26.878 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.733 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.512 | 0.000 |
| 12x2-w32 | accepted | other | 4.009 | 0.000 |
| 12x2-w32 | accepted | scheduler | 3.994 | 0.000 |
| 12x2-w32 | accepted | solver | 6.280 | 0.000 |
| 12x2-w32 | inlined | G1_arithmetic | 34.878 | 0.472 |
| 12x2-w32 | inlined | G2_arithmetic | 13.479 | -0.067 |
| 12x2-w32 | inlined | epoch | 2.160 | -3.483 |
| 12x2-w32 | inlined | fft | 28.140 | 1.262 |
| 12x2-w32 | inlined | grouping | 1.964 | 0.232 |
| 12x2-w32 | inlined | inverse | 3.390 | -0.122 |
| 12x2-w32 | inlined | other | 4.192 | 0.183 |
| 12x2-w32 | inlined | scheduler | 5.638 | 1.644 |
| 12x2-w32 | inlined | solver | 6.158 | -0.122 |
| 12x2-w32 | off | G1_arithmetic | 34.492 | 0.087 |
| 12x2-w32 | off | G2_arithmetic | 13.686 | 0.140 |
| 12x2-w32 | off | epoch | 4.885 | -0.759 |
| 12x2-w32 | off | fft | 28.017 | 1.139 |
| 12x2-w32 | off | grouping | 1.842 | 0.110 |
| 12x2-w32 | off | inverse | 3.375 | -0.136 |
| 12x2-w32 | off | other | 3.734 | -0.275 |
| 12x2-w32 | off | scheduler | 3.880 | -0.114 |
| 12x2-w32 | off | solver | 6.088 | -0.192 |
| 12x2-w32 | on | G1_arithmetic | 34.212 | -0.194 |
| 12x2-w32 | on | G2_arithmetic | 14.854 | 1.308 |
| 12x2-w32 | on | epoch | 2.209 | -3.434 |
| 12x2-w32 | on | fft | 27.539 | 0.661 |
| 12x2-w32 | on | grouping | 1.899 | 0.167 |
| 12x2-w32 | on | inverse | 3.356 | -0.155 |
| 12x2-w32 | on | other | 4.367 | 0.358 |
| 12x2-w32 | on | scheduler | 5.342 | 1.349 |
| 12x2-w32 | on | solver | 6.220 | -0.060 |
| 1x2-w32 | accepted | G1_arithmetic | 38.388 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 30.442 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.274 | 0.000 |
| 1x2-w32 | accepted | fft | 5.114 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.886 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.236 | 0.000 |
| 1x2-w32 | accepted | other | 4.636 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.737 | 0.000 |
| 1x2-w32 | accepted | solver | 1.286 | 0.000 |
| 1x2-w32 | inlined | G1_arithmetic | 39.264 | 0.876 |
| 1x2-w32 | inlined | G2_arithmetic | 29.866 | -0.576 |
| 1x2-w32 | inlined | epoch | 3.906 | -5.368 |
| 1x2-w32 | inlined | fft | 4.998 | -0.116 |
| 1x2-w32 | inlined | grouping | 0.618 | -0.268 |
| 1x2-w32 | inlined | inverse | 7.029 | 0.793 |
| 1x2-w32 | inlined | other | 5.819 | 1.183 |
| 1x2-w32 | inlined | scheduler | 6.844 | 3.107 |
| 1x2-w32 | inlined | solver | 1.655 | 0.369 |
| 1x2-w32 | off | G1_arithmetic | 37.586 | -0.802 |
| 1x2-w32 | off | G2_arithmetic | 29.604 | -0.839 |
| 1x2-w32 | off | epoch | 9.604 | 0.330 |
| 1x2-w32 | off | fft | 4.967 | -0.147 |
| 1x2-w32 | off | grouping | 0.695 | -0.192 |
| 1x2-w32 | off | inverse | 7.087 | 0.852 |
| 1x2-w32 | off | other | 4.463 | -0.173 |
| 1x2-w32 | off | scheduler | 4.397 | 0.659 |
| 1x2-w32 | off | solver | 1.598 | 0.311 |
| 1x2-w32 | on | G1_arithmetic | 39.738 | 1.350 |
| 1x2-w32 | on | G2_arithmetic | 30.332 | -0.110 |
| 1x2-w32 | on | epoch | 3.385 | -5.889 |
| 1x2-w32 | on | fft | 5.127 | 0.013 |
| 1x2-w32 | on | grouping | 0.725 | -0.161 |
| 1x2-w32 | on | inverse | 7.821 | 1.586 |
| 1x2-w32 | on | other | 5.259 | 0.623 |
| 1x2-w32 | on | scheduler | 6.039 | 2.301 |
| 1x2-w32 | on | solver | 1.573 | 0.287 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.132 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 15.582 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.148 | 0.000 |
| merge-54x1-w32 | accepted | fft | 41.515 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.322 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.817 | 0.000 |
| merge-54x1-w32 | accepted | other | 4.004 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.119 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.360 | 0.000 |
| merge-54x1-w32 | inlined | G1_arithmetic | 27.218 | 1.086 |
| merge-54x1-w32 | inlined | G2_arithmetic | 14.493 | -1.089 |
| merge-54x1-w32 | inlined | epoch | 0.911 | -1.237 |
| merge-54x1-w32 | inlined | fft | 41.823 | 0.308 |
| merge-54x1-w32 | inlined | grouping | 1.545 | 0.223 |
| merge-54x1-w32 | inlined | inverse | 1.712 | -0.105 |
| merge-54x1-w32 | inlined | other | 4.284 | 0.279 |
| merge-54x1-w32 | inlined | scheduler | 2.816 | 0.697 |
| merge-54x1-w32 | inlined | solver | 5.198 | -0.162 |
| merge-54x1-w32 | off | G1_arithmetic | 27.355 | 1.223 |
| merge-54x1-w32 | off | G2_arithmetic | 12.194 | -3.388 |
| merge-54x1-w32 | off | epoch | 2.307 | 0.159 |
| merge-54x1-w32 | off | fft | 43.086 | 1.571 |
| merge-54x1-w32 | off | grouping | 1.461 | 0.139 |
| merge-54x1-w32 | off | inverse | 1.758 | -0.060 |
| merge-54x1-w32 | off | other | 4.195 | 0.190 |
| merge-54x1-w32 | off | scheduler | 2.016 | -0.103 |
| merge-54x1-w32 | off | solver | 5.629 | 0.268 |
| merge-54x1-w32 | on | G1_arithmetic | 26.346 | 0.214 |
| merge-54x1-w32 | on | G2_arithmetic | 15.082 | -0.499 |
| merge-54x1-w32 | on | epoch | 1.127 | -1.020 |
| merge-54x1-w32 | on | fft | 41.936 | 0.421 |
| merge-54x1-w32 | on | grouping | 1.404 | 0.082 |
| merge-54x1-w32 | on | inverse | 1.778 | -0.039 |
| merge-54x1-w32 | on | other | 4.197 | 0.192 |
| merge-54x1-w32 | on | scheduler | 2.969 | 0.850 |
| merge-54x1-w32 | on | solver | 5.160 | -0.201 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.622709 | 37.000 | 4.291× | 24.000 | 2.783× | false |
| 1x2-w32 | off | 8.663186 | 37.000 | 4.271× | 24.000 | 2.770× | false |
| 1x2-w32 | on | 8.638950 | 37.000 | 4.283× | 24.000 | 2.778× | false |
| 1x2-w32 | inlined | 8.601599 | 37.000 | 4.302× | 24.000 | 2.790× | false |
| 2x2-w32 | accepted | 11.769607 | 59.000 | 5.013× | 30.000 | 2.549× | false |
| 2x2-w32 | off | 11.669077 | 59.000 | 5.056× | 30.000 | 2.571× | false |
| 2x2-w32 | on | 11.786359 | 59.000 | 5.006× | 30.000 | 2.545× | false |
| 2x2-w32 | inlined | 11.887239 | 59.000 | 4.963× | 30.000 | 2.524× | false |
| 4x2-w32 | accepted | 16.411706 | 109.000 | 6.642× | 48.000 | 2.925× | false |
| 4x2-w32 | off | 16.100078 | 109.000 | 6.770× | 48.000 | 2.981× | false |
| 4x2-w32 | on | 16.379525 | 109.000 | 6.655× | 48.000 | 2.930× | false |
| 4x2-w32 | inlined | 16.611514 | 109.000 | 6.562× | 48.000 | 2.890× | false |
| 8x2-w32 | accepted | 24.476924 | 189.000 | 7.722× | 74.000 | 3.023× | false |
| 8x2-w32 | off | 24.942502 | 189.000 | 7.577× | 74.000 | 2.967× | false |
| 8x2-w32 | on | 25.157607 | 189.000 | 7.513× | 74.000 | 2.941× | false |
| 8x2-w32 | inlined | 25.194868 | 189.000 | 7.502× | 74.000 | 2.937× | false |
| 12x2-w32 | accepted | 34.556643 | 284.000 | 8.218× | 101.000 | 2.923× | false |
| 12x2-w32 | off | 34.558356 | 284.000 | 8.218× | 101.000 | 2.923× | false |
| 12x2-w32 | on | 34.297123 | 284.000 | 8.281× | 101.000 | 2.945× | false |
| 12x2-w32 | inlined | 35.060773 | 284.000 | 8.100× | 101.000 | 2.881× | false |
| 16x2-w32 | accepted | 40.949175 | 335.000 | 8.181× | 124.000 | 3.028× | false |
| 16x2-w32 | off | 41.389176 | 335.000 | 8.094× | 124.000 | 2.996× | false |
| 16x2-w32 | on | 40.780466 | 335.000 | 8.215× | 124.000 | 3.041× | false |
| 16x2-w32 | inlined | 41.490004 | 335.000 | 8.074× | 124.000 | 2.989× | false |
| 24x2-w32 | accepted | 68.358811 | 528.000 | 7.724× | 176.000 | 2.575× | false |
| 24x2-w32 | off | 68.537127 | 528.000 | 7.704× | 176.000 | 2.568× | false |
| 24x2-w32 | on | 69.289319 | 528.000 | 7.620× | 176.000 | 2.540× | false |
| 24x2-w32 | inlined | 70.479302 | 528.000 | 7.492× | 176.000 | 2.497× | false |
| merge-24x1-w32 | accepted | 67.243789 | 507.000 | 7.540× | 175.000 | 2.602× | false |
| merge-24x1-w32 | off | 67.866942 | 507.000 | 7.471× | 175.000 | 2.579× | false |
| merge-24x1-w32 | on | 69.319626 | 507.000 | 7.314× | 175.000 | 2.525× | false |
| merge-24x1-w32 | inlined | 68.823048 | 507.000 | 7.367× | 175.000 | 2.543× | false |

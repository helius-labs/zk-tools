# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.216192 | 1.000000 | 957.380 | 34.393117 | 739.00 |
| 12x2-w32 | off | 34.381042 | 0.999983 | 955.782 | 34.570673 | 719.25 |
| 12x2-w32 | on | 34.729938 | 1.011376 | 1001.108 | 36.283867 | 717.25 |
| 16x2-w32 | accepted | 40.802843 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.759301 | 0.994918 | — | — | — |
| 16x2-w32 | on | 41.219929 | 1.003865 | — | — | — |
| 1x2-w16 | accepted | 10.816439 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.512366 | 0.977062 | — | — | — |
| 1x2-w16 | on | 10.735637 | 0.980748 | — | — | — |
| 1x2-w32 | accepted | 8.577467 | 1.000000 | 200.561 | 8.842239 | 129.50 |
| 1x2-w32 | off | 8.646547 | 1.008054 | 199.508 | 8.724842 | 127.75 |
| 1x2-w32 | on | 8.752187 | 1.018920 | 199.633 | 8.821217 | 128.75 |
| 24x2-w32 | accepted | 68.618172 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.372968 | 0.997621 | — | — | — |
| 24x2-w32 | on | 69.292993 | 1.010063 | — | — | — |
| 2x2-w32 | accepted | 11.710566 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.754229 | 1.001808 | — | — | — |
| 2x2-w32 | on | 11.756693 | 1.003939 | — | — | — |
| 4x2-w32 | accepted | 16.177090 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.051282 | 0.992756 | — | — | — |
| 4x2-w32 | on | 16.295763 | 1.002242 | — | — | — |
| 8x2-w32 | accepted | 24.693000 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.417330 | 0.990201 | — | — | — |
| 8x2-w32 | on | 24.560720 | 1.003449 | — | — | — |
| merge-24x1-w32 | accepted | 67.988865 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.103993 | 0.998950 | — | — | — |
| merge-24x1-w32 | on | 68.025503 | 1.001939 | — | — | — |
| merge-54x1-w32 | accepted | 179.145661 | 1.000000 | 4957.436 | 178.737935 | 1632.54 |
| merge-54x1-w32 | off | 176.559591 | 0.983737 | 4811.801 | 173.297503 | 1641.55 |
| merge-54x1-w32 | on | 178.219631 | 0.995410 | 4944.871 | 179.441080 | 1591.37 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.685 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.735 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.118 | 0.000 |
| 12x2-w32 | accepted | fft | 27.646 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.854 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.292 | 0.000 |
| 12x2-w32 | accepted | other | 3.763 | 0.000 |
| 12x2-w32 | accepted | scheduler | 3.778 | 0.000 |
| 12x2-w32 | accepted | solver | 6.130 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 34.436 | -0.249 |
| 12x2-w32 | off | G2_arithmetic | 13.643 | -0.092 |
| 12x2-w32 | off | epoch | 5.446 | 0.328 |
| 12x2-w32 | off | fft | 27.086 | -0.559 |
| 12x2-w32 | off | grouping | 1.752 | -0.102 |
| 12x2-w32 | off | inverse | 3.411 | 0.119 |
| 12x2-w32 | off | other | 3.955 | 0.193 |
| 12x2-w32 | off | scheduler | 4.099 | 0.321 |
| 12x2-w32 | off | solver | 6.172 | 0.043 |
| 12x2-w32 | on | G1_arithmetic | 33.382 | -1.303 |
| 12x2-w32 | on | G2_arithmetic | 14.527 | 0.792 |
| 12x2-w32 | on | epoch | 5.028 | -0.090 |
| 12x2-w32 | on | fft | 28.101 | 0.455 |
| 12x2-w32 | on | grouping | 1.806 | -0.048 |
| 12x2-w32 | on | inverse | 3.142 | -0.150 |
| 12x2-w32 | on | other | 4.114 | 0.351 |
| 12x2-w32 | on | scheduler | 3.689 | -0.088 |
| 12x2-w32 | on | solver | 6.211 | 0.081 |
| 1x2-w32 | accepted | G1_arithmetic | 38.053 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.306 | 0.000 |
| 1x2-w32 | accepted | epoch | 10.291 | 0.000 |
| 1x2-w32 | accepted | fft | 5.167 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.418 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.906 | 0.000 |
| 1x2-w32 | accepted | other | 4.855 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.635 | 0.000 |
| 1x2-w32 | accepted | solver | 1.368 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.065 | 0.012 |
| 1x2-w32 | off | G2_arithmetic | 29.025 | -0.281 |
| 1x2-w32 | off | epoch | 9.648 | -0.643 |
| 1x2-w32 | off | fft | 4.729 | -0.437 |
| 1x2-w32 | off | grouping | 0.746 | 0.328 |
| 1x2-w32 | off | inverse | 6.886 | -0.021 |
| 1x2-w32 | off | other | 4.918 | 0.063 |
| 1x2-w32 | off | scheduler | 4.632 | 0.996 |
| 1x2-w32 | off | solver | 1.350 | -0.018 |
| 1x2-w32 | on | G1_arithmetic | 37.937 | -0.116 |
| 1x2-w32 | on | G2_arithmetic | 29.708 | 0.402 |
| 1x2-w32 | on | epoch | 9.429 | -0.862 |
| 1x2-w32 | on | fft | 4.582 | -0.585 |
| 1x2-w32 | on | grouping | 0.773 | 0.354 |
| 1x2-w32 | on | inverse | 7.170 | 0.263 |
| 1x2-w32 | on | other | 4.851 | -0.004 |
| 1x2-w32 | on | scheduler | 4.236 | 0.601 |
| 1x2-w32 | on | solver | 1.314 | -0.054 |
| merge-54x1-w32 | accepted | G1_arithmetic | 25.510 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 16.381 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.093 | 0.000 |
| merge-54x1-w32 | accepted | fft | 41.013 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.482 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.827 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.982 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.045 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.667 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 26.251 | 0.741 |
| merge-54x1-w32 | off | G2_arithmetic | 14.299 | -2.082 |
| merge-54x1-w32 | off | epoch | 2.178 | 0.085 |
| merge-54x1-w32 | off | fft | 42.654 | 1.641 |
| merge-54x1-w32 | off | grouping | 1.390 | -0.092 |
| merge-54x1-w32 | off | inverse | 1.729 | -0.098 |
| merge-54x1-w32 | off | other | 4.180 | 0.198 |
| merge-54x1-w32 | off | scheduler | 2.149 | 0.104 |
| merge-54x1-w32 | off | solver | 5.171 | -0.497 |
| merge-54x1-w32 | on | G1_arithmetic | 25.250 | -0.261 |
| merge-54x1-w32 | on | G2_arithmetic | 16.859 | 0.478 |
| merge-54x1-w32 | on | epoch | 2.106 | 0.014 |
| merge-54x1-w32 | on | fft | 41.174 | 0.161 |
| merge-54x1-w32 | on | grouping | 1.433 | -0.049 |
| merge-54x1-w32 | on | inverse | 1.671 | -0.156 |
| merge-54x1-w32 | on | other | 4.121 | 0.139 |
| merge-54x1-w32 | on | scheduler | 2.176 | 0.131 |
| merge-54x1-w32 | on | solver | 5.210 | -0.458 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.577467 | 37.000 | 4.314× | 24.000 | 2.798× | false |
| 1x2-w32 | off | 8.646547 | 37.000 | 4.279× | 24.000 | 2.776× | false |
| 1x2-w32 | on | 8.752187 | 37.000 | 4.228× | 24.000 | 2.742× | false |
| 2x2-w32 | accepted | 11.710566 | 59.000 | 5.038× | 30.000 | 2.562× | false |
| 2x2-w32 | off | 11.754229 | 59.000 | 5.019× | 30.000 | 2.552× | false |
| 2x2-w32 | on | 11.756693 | 59.000 | 5.018× | 30.000 | 2.552× | false |
| 4x2-w32 | accepted | 16.177090 | 109.000 | 6.738× | 48.000 | 2.967× | false |
| 4x2-w32 | off | 16.051282 | 109.000 | 6.791× | 48.000 | 2.990× | false |
| 4x2-w32 | on | 16.295763 | 109.000 | 6.689× | 48.000 | 2.946× | false |
| 8x2-w32 | accepted | 24.693000 | 189.000 | 7.654× | 74.000 | 2.997× | false |
| 8x2-w32 | off | 24.417330 | 189.000 | 7.740× | 74.000 | 3.031× | false |
| 8x2-w32 | on | 24.560720 | 189.000 | 7.695× | 74.000 | 3.013× | false |
| 12x2-w32 | accepted | 34.216192 | 284.000 | 8.300× | 101.000 | 2.952× | false |
| 12x2-w32 | off | 34.381042 | 284.000 | 8.260× | 101.000 | 2.938× | false |
| 12x2-w32 | on | 34.729938 | 284.000 | 8.177× | 101.000 | 2.908× | false |
| 16x2-w32 | accepted | 40.802843 | 335.000 | 8.210× | 124.000 | 3.039× | false |
| 16x2-w32 | off | 40.759301 | 335.000 | 8.219× | 124.000 | 3.042× | false |
| 16x2-w32 | on | 41.219929 | 335.000 | 8.127× | 124.000 | 3.008× | false |
| 24x2-w32 | accepted | 68.618172 | 528.000 | 7.695× | 176.000 | 2.565× | false |
| 24x2-w32 | off | 68.372968 | 528.000 | 7.722× | 176.000 | 2.574× | false |
| 24x2-w32 | on | 69.292993 | 528.000 | 7.620× | 176.000 | 2.540× | false |
| merge-24x1-w32 | accepted | 67.988865 | 507.000 | 7.457× | 175.000 | 2.574× | false |
| merge-24x1-w32 | off | 68.103993 | 507.000 | 7.444× | 175.000 | 2.570× | false |
| merge-24x1-w32 | on | 68.025503 | 507.000 | 7.453× | 175.000 | 2.573× | false |

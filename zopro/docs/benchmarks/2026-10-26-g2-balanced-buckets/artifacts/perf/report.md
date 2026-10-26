# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w16 | accepted | 10.755022 | 1.000000 | — | — | — |
| 1x2-w16 | off | 12.982152 | 1.016605 | — | — | — |
| 1x2-w16 | on | 11.819470 | 1.010572 | — | — | — |
| 1x2-w32 | accepted | 8.627621 | 1.000000 | 200.049 | 8.653001 | 129.25 |
| 1x2-w32 | off | 8.601741 | 1.000376 | 199.558 | 8.635194 | 130.50 |
| 1x2-w32 | on | 8.809072 | 1.025847 | 209.721 | 8.960815 | 126.75 |
| 2x2-w32 | accepted | 11.725111 | 1.000000 | 271.726 | 11.744850 | 312.25 |
| 2x2-w32 | off | 11.586149 | 0.991279 | 274.467 | 11.823061 | 305.75 |
| 2x2-w32 | on | 11.792859 | 1.011249 | 279.881 | 11.869032 | 297.00 |
| merge-54x1-w32 | accepted | 177.810898 | 1.000000 | 4841.639 | 174.789547 | 1659.39 |
| merge-54x1-w32 | off | 177.865187 | 1.004817 | 4971.324 | 179.611756 | 1567.41 |
| merge-54x1-w32 | on | 178.709082 | 1.006272 | 4910.918 | 177.677270 | 1653.82 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 39.844 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.862 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.241 | 0.000 |
| 1x2-w32 | accepted | fft | 4.362 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.737 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.218 | 0.000 |
| 1x2-w32 | accepted | other | 4.929 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.249 | 0.000 |
| 1x2-w32 | accepted | solver | 1.558 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.325 | -1.519 |
| 1x2-w32 | off | G2_arithmetic | 28.982 | 0.120 |
| 1x2-w32 | off | epoch | 9.414 | 0.173 |
| 1x2-w32 | off | fft | 4.343 | -0.019 |
| 1x2-w32 | off | grouping | 0.752 | 0.015 |
| 1x2-w32 | off | inverse | 7.322 | 1.104 |
| 1x2-w32 | off | other | 5.105 | 0.176 |
| 1x2-w32 | off | scheduler | 4.355 | 0.106 |
| 1x2-w32 | off | solver | 1.401 | -0.157 |
| 1x2-w32 | on | G1_arithmetic | 36.929 | -2.915 |
| 1x2-w32 | on | G2_arithmetic | 31.606 | 2.744 |
| 1x2-w32 | on | epoch | 9.581 | 0.340 |
| 1x2-w32 | on | fft | 4.741 | 0.378 |
| 1x2-w32 | on | grouping | 0.550 | -0.187 |
| 1x2-w32 | on | inverse | 7.100 | 0.882 |
| 1x2-w32 | on | other | 4.399 | -0.530 |
| 1x2-w32 | on | scheduler | 3.693 | -0.556 |
| 1x2-w32 | on | solver | 1.401 | -0.157 |
| 2x2-w32 | accepted | G1_arithmetic | 33.819 | 0.000 |
| 2x2-w32 | accepted | G2_arithmetic | 25.148 | 0.000 |
| 2x2-w32 | accepted | epoch | 10.181 | 0.000 |
| 2x2-w32 | accepted | fft | 9.417 | 0.000 |
| 2x2-w32 | accepted | grouping | 1.895 | 0.000 |
| 2x2-w32 | accepted | inverse | 6.671 | 0.000 |
| 2x2-w32 | accepted | other | 5.426 | 0.000 |
| 2x2-w32 | accepted | scheduler | 4.937 | 0.000 |
| 2x2-w32 | accepted | solver | 2.504 | 0.000 |
| 2x2-w32 | off | G1_arithmetic | 34.483 | 0.664 |
| 2x2-w32 | off | G2_arithmetic | 24.293 | -0.855 |
| 2x2-w32 | off | epoch | 10.689 | 0.508 |
| 2x2-w32 | off | fft | 9.670 | 0.252 |
| 2x2-w32 | off | grouping | 1.664 | -0.232 |
| 2x2-w32 | off | inverse | 6.453 | -0.218 |
| 2x2-w32 | off | other | 5.386 | -0.040 |
| 2x2-w32 | off | scheduler | 4.906 | -0.031 |
| 2x2-w32 | off | solver | 2.456 | -0.048 |
| 2x2-w32 | on | G1_arithmetic | 33.072 | -0.748 |
| 2x2-w32 | on | G2_arithmetic | 25.375 | 0.227 |
| 2x2-w32 | on | epoch | 10.669 | 0.488 |
| 2x2-w32 | on | fft | 9.324 | -0.093 |
| 2x2-w32 | on | grouping | 1.663 | -0.233 |
| 2x2-w32 | on | inverse | 6.852 | 0.181 |
| 2x2-w32 | on | other | 5.872 | 0.446 |
| 2x2-w32 | on | scheduler | 4.869 | -0.068 |
| 2x2-w32 | on | solver | 2.304 | -0.200 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.824 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 14.176 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.087 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.490 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.433 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.990 | 0.000 |
| merge-54x1-w32 | accepted | other | 4.247 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.120 | 0.000 |
| merge-54x1-w32 | accepted | solver | 4.634 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 25.690 | -1.134 |
| merge-54x1-w32 | off | G2_arithmetic | 16.405 | 2.229 |
| merge-54x1-w32 | off | epoch | 2.133 | 0.047 |
| merge-54x1-w32 | off | fft | 41.010 | -1.480 |
| merge-54x1-w32 | off | grouping | 1.437 | 0.004 |
| merge-54x1-w32 | off | inverse | 1.700 | -0.290 |
| merge-54x1-w32 | off | other | 3.848 | -0.400 |
| merge-54x1-w32 | off | scheduler | 2.216 | 0.096 |
| merge-54x1-w32 | off | solver | 5.561 | 0.927 |
| merge-54x1-w32 | on | G1_arithmetic | 25.318 | -1.506 |
| merge-54x1-w32 | on | G2_arithmetic | 15.952 | 1.776 |
| merge-54x1-w32 | on | epoch | 2.032 | -0.055 |
| merge-54x1-w32 | on | fft | 42.603 | 0.113 |
| merge-54x1-w32 | on | grouping | 1.381 | -0.052 |
| merge-54x1-w32 | on | inverse | 1.712 | -0.279 |
| merge-54x1-w32 | on | other | 3.861 | -0.387 |
| merge-54x1-w32 | on | scheduler | 2.019 | -0.101 |
| merge-54x1-w32 | on | solver | 5.123 | 0.488 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.627621 | 37.000 | 4.289× | 24.000 | 2.782× | false |
| 1x2-w32 | off | 8.601741 | 37.000 | 4.301× | 24.000 | 2.790× | false |
| 1x2-w32 | on | 8.809072 | 37.000 | 4.200× | 24.000 | 2.724× | false |
| 2x2-w32 | accepted | 11.725111 | 59.000 | 5.032× | 30.000 | 2.559× | false |
| 2x2-w32 | off | 11.586149 | 59.000 | 5.092× | 30.000 | 2.589× | false |
| 2x2-w32 | on | 11.792859 | 59.000 | 5.003× | 30.000 | 2.544× | false |

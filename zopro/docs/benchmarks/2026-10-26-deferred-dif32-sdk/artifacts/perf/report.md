# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.853869 | 1.000000 | 954.799 | 34.441057 | 692.25 |
| 12x2-w32 | off | 34.555339 | 0.994333 | 961.782 | 34.446442 | 733.00 |
| 12x2-w32 | on | 34.285069 | 0.988006 | 944.868 | 34.420589 | 725.50 |
| 16x2-w32 | accepted | 40.861940 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.893080 | 1.005415 | — | — | — |
| 16x2-w32 | on | 40.829698 | 1.006034 | — | — | — |
| 1x2-w16 | accepted | 11.032182 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.418992 | 0.943490 | — | — | — |
| 1x2-w16 | on | 10.476766 | 0.949655 | — | — | — |
| 1x2-w32 | accepted | 8.580045 | 1.000000 | 200.321 | 8.725250 | 131.75 |
| 1x2-w32 | off | 8.590735 | 0.998098 | 199.455 | 8.685387 | 125.75 |
| 1x2-w32 | on | 8.569553 | 0.995464 | 199.898 | 8.716811 | 126.50 |
| 24x2-w32 | accepted | 68.866157 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.939306 | 1.001713 | — | — | — |
| 24x2-w32 | on | 68.363464 | 0.990981 | — | — | — |
| 2x2-w32 | accepted | 11.726016 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.694908 | 0.998801 | — | — | — |
| 2x2-w32 | on | 11.679797 | 0.999340 | — | — | — |
| 4x2-w32 | accepted | 16.106504 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.229896 | 1.007661 | — | — | — |
| 4x2-w32 | on | 16.118517 | 1.002614 | — | — | — |
| 8x2-w32 | accepted | 24.491386 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.230481 | 0.994226 | — | — | — |
| 8x2-w32 | on | 24.808428 | 1.002907 | — | — | — |
| merge-24x1-w32 | accepted | 67.608589 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 67.542462 | 1.000583 | — | — | — |
| merge-24x1-w32 | on | 67.789058 | 0.995355 | — | — | — |
| merge-54x1-w32 | accepted | 176.381251 | 1.000000 | 4885.681 | 175.835328 | 1656.41 |
| merge-54x1-w32 | off | 177.750341 | 1.007253 | 4995.100 | 180.198857 | 1606.82 |
| merge-54x1-w32 | on | 176.980118 | 0.996509 | 4821.068 | 176.504537 | 1639.16 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.223 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.351 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.229 | 0.000 |
| 12x2-w32 | accepted | fft | 27.723 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.769 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.469 | 0.000 |
| 12x2-w32 | accepted | other | 4.173 | 0.000 |
| 12x2-w32 | accepted | scheduler | 3.844 | 0.000 |
| 12x2-w32 | accepted | solver | 6.220 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 33.969 | -0.254 |
| 12x2-w32 | off | G2_arithmetic | 13.989 | 0.638 |
| 12x2-w32 | off | epoch | 5.146 | -0.083 |
| 12x2-w32 | off | fft | 27.677 | -0.047 |
| 12x2-w32 | off | grouping | 1.757 | -0.012 |
| 12x2-w32 | off | inverse | 3.463 | -0.006 |
| 12x2-w32 | off | other | 4.106 | -0.066 |
| 12x2-w32 | off | scheduler | 3.925 | 0.081 |
| 12x2-w32 | off | solver | 5.969 | -0.251 |
| 12x2-w32 | on | G1_arithmetic | 34.865 | 0.643 |
| 12x2-w32 | on | G2_arithmetic | 13.548 | 0.197 |
| 12x2-w32 | on | epoch | 5.290 | 0.061 |
| 12x2-w32 | on | fft | 27.049 | -0.674 |
| 12x2-w32 | on | grouping | 1.918 | 0.149 |
| 12x2-w32 | on | inverse | 3.273 | -0.196 |
| 12x2-w32 | on | other | 4.092 | -0.081 |
| 12x2-w32 | on | scheduler | 3.786 | -0.058 |
| 12x2-w32 | on | solver | 6.180 | -0.041 |
| 1x2-w32 | accepted | G1_arithmetic | 38.160 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.413 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.367 | 0.000 |
| 1x2-w32 | accepted | fft | 4.821 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.642 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.015 | 0.000 |
| 1x2-w32 | accepted | other | 4.917 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.118 | 0.000 |
| 1x2-w32 | accepted | solver | 1.547 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.011 | -0.148 |
| 1x2-w32 | off | G2_arithmetic | 29.478 | 0.065 |
| 1x2-w32 | off | epoch | 9.488 | 0.121 |
| 1x2-w32 | off | fft | 4.687 | -0.134 |
| 1x2-w32 | off | grouping | 0.725 | 0.084 |
| 1x2-w32 | off | inverse | 7.517 | 0.502 |
| 1x2-w32 | off | other | 4.885 | -0.033 |
| 1x2-w32 | off | scheduler | 3.884 | -0.234 |
| 1x2-w32 | off | solver | 1.325 | -0.222 |
| 1x2-w32 | on | G1_arithmetic | 37.125 | -1.035 |
| 1x2-w32 | on | G2_arithmetic | 29.794 | 0.380 |
| 1x2-w32 | on | epoch | 9.656 | 0.289 |
| 1x2-w32 | on | fft | 5.043 | 0.222 |
| 1x2-w32 | on | grouping | 0.610 | -0.031 |
| 1x2-w32 | on | inverse | 7.141 | 0.126 |
| 1x2-w32 | on | other | 4.908 | -0.009 |
| 1x2-w32 | on | scheduler | 4.316 | 0.198 |
| 1x2-w32 | on | solver | 1.408 | -0.139 |
| merge-54x1-w32 | accepted | G1_arithmetic | 25.645 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 15.292 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 1.992 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.798 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.308 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.752 | 0.000 |
| merge-54x1-w32 | accepted | other | 4.087 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.021 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.104 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 24.754 | -0.890 |
| merge-54x1-w32 | off | G2_arithmetic | 17.828 | 2.536 |
| merge-54x1-w32 | off | epoch | 1.988 | -0.004 |
| merge-54x1-w32 | off | fft | 41.258 | -1.540 |
| merge-54x1-w32 | off | grouping | 1.391 | 0.082 |
| merge-54x1-w32 | off | inverse | 1.791 | 0.038 |
| merge-54x1-w32 | off | other | 4.040 | -0.047 |
| merge-54x1-w32 | off | scheduler | 2.044 | 0.022 |
| merge-54x1-w32 | off | solver | 4.907 | -0.197 |
| merge-54x1-w32 | on | G1_arithmetic | 25.814 | 0.169 |
| merge-54x1-w32 | on | G2_arithmetic | 14.037 | -1.255 |
| merge-54x1-w32 | on | epoch | 2.340 | 0.348 |
| merge-54x1-w32 | on | fft | 42.962 | 0.164 |
| merge-54x1-w32 | on | grouping | 1.398 | 0.089 |
| merge-54x1-w32 | on | inverse | 1.766 | 0.014 |
| merge-54x1-w32 | on | other | 4.233 | 0.146 |
| merge-54x1-w32 | on | scheduler | 2.120 | 0.098 |
| merge-54x1-w32 | on | solver | 5.330 | 0.227 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.580045 | 37.000 | 4.312× | 24.000 | 2.797× | false |
| 1x2-w32 | off | 8.590735 | 37.000 | 4.307× | 24.000 | 2.794× | false |
| 1x2-w32 | on | 8.569553 | 37.000 | 4.318× | 24.000 | 2.801× | false |
| 2x2-w32 | accepted | 11.726016 | 59.000 | 5.032× | 30.000 | 2.558× | false |
| 2x2-w32 | off | 11.694908 | 59.000 | 5.045× | 30.000 | 2.565× | false |
| 2x2-w32 | on | 11.679797 | 59.000 | 5.051× | 30.000 | 2.569× | false |
| 4x2-w32 | accepted | 16.106504 | 109.000 | 6.767× | 48.000 | 2.980× | false |
| 4x2-w32 | off | 16.229896 | 109.000 | 6.716× | 48.000 | 2.958× | false |
| 4x2-w32 | on | 16.118517 | 109.000 | 6.762× | 48.000 | 2.978× | false |
| 8x2-w32 | accepted | 24.491386 | 189.000 | 7.717× | 74.000 | 3.021× | false |
| 8x2-w32 | off | 24.230481 | 189.000 | 7.800× | 74.000 | 3.054× | false |
| 8x2-w32 | on | 24.808428 | 189.000 | 7.618× | 74.000 | 2.983× | false |
| 12x2-w32 | accepted | 34.853869 | 284.000 | 8.148× | 101.000 | 2.898× | false |
| 12x2-w32 | off | 34.555339 | 284.000 | 8.219× | 101.000 | 2.923× | false |
| 12x2-w32 | on | 34.285069 | 284.000 | 8.283× | 101.000 | 2.946× | false |
| 16x2-w32 | accepted | 40.861940 | 335.000 | 8.198× | 124.000 | 3.035× | false |
| 16x2-w32 | off | 40.893080 | 335.000 | 8.192× | 124.000 | 3.032× | false |
| 16x2-w32 | on | 40.829698 | 335.000 | 8.205× | 124.000 | 3.037× | false |
| 24x2-w32 | accepted | 68.866157 | 528.000 | 7.667× | 176.000 | 2.556× | false |
| 24x2-w32 | off | 68.939306 | 528.000 | 7.659× | 176.000 | 2.553× | false |
| 24x2-w32 | on | 68.363464 | 528.000 | 7.723× | 176.000 | 2.574× | false |
| merge-24x1-w32 | accepted | 67.608589 | 507.000 | 7.499× | 175.000 | 2.588× | false |
| merge-24x1-w32 | off | 67.542462 | 507.000 | 7.506× | 175.000 | 2.591× | false |
| merge-24x1-w32 | on | 67.789058 | 507.000 | 7.479× | 175.000 | 2.582× | false |

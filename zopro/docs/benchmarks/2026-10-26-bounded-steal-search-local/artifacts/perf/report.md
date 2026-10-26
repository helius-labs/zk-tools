# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w16 | accepted | 11.107698 | 1.000000 | — | — | — |
| 1x2-w16 | off | 11.323835 | 0.948334 | — | — | — |
| 1x2-w16 | on | 11.228726 | 1.006878 | — | — | — |
| 1x2-w32 | accepted | 8.651936 | 1.000000 | 199.710 | 8.698287 | 125.50 |
| 1x2-w32 | off | 8.580181 | 0.992282 | 199.504 | 8.748244 | 125.50 |
| 1x2-w32 | on | 8.573519 | 0.990936 | 202.346 | 8.701075 | 130.50 |
| 2x2-w32 | accepted | 11.746021 | 1.000000 | 273.577 | 11.743375 | 287.00 |
| 2x2-w32 | off | 11.727406 | 1.001665 | 271.707 | 11.814452 | 305.50 |
| 2x2-w32 | on | 11.660761 | 0.992741 | 276.063 | 11.756897 | 309.75 |
| merge-54x1-w32 | accepted | 176.465048 | 1.000000 | 4832.782 | 176.798005 | 1725.92 |
| merge-54x1-w32 | off | 179.715708 | 1.015507 | 4961.479 | 177.844025 | 1520.49 |
| merge-54x1-w32 | on | 179.972472 | 1.022119 | 5018.077 | 180.495746 | 1586.83 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.518 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.794 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.437 | 0.000 |
| 1x2-w32 | accepted | fft | 4.663 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.762 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.551 | 0.000 |
| 1x2-w32 | accepted | other | 4.776 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.928 | 0.000 |
| 1x2-w32 | accepted | solver | 1.570 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.053 | -1.465 |
| 1x2-w32 | off | G2_arithmetic | 30.044 | 0.250 |
| 1x2-w32 | off | epoch | 9.380 | -0.057 |
| 1x2-w32 | off | fft | 4.780 | 0.117 |
| 1x2-w32 | off | grouping | 0.798 | 0.036 |
| 1x2-w32 | off | inverse | 7.197 | 0.646 |
| 1x2-w32 | off | other | 5.008 | 0.232 |
| 1x2-w32 | off | scheduler | 4.353 | 0.425 |
| 1x2-w32 | off | solver | 1.387 | -0.183 |
| 1x2-w32 | on | G1_arithmetic | 37.644 | -0.874 |
| 1x2-w32 | on | G2_arithmetic | 28.872 | -0.922 |
| 1x2-w32 | on | epoch | 6.135 | -3.302 |
| 1x2-w32 | on | fft | 4.857 | 0.193 |
| 1x2-w32 | on | grouping | 0.725 | -0.037 |
| 1x2-w32 | on | inverse | 7.076 | 0.525 |
| 1x2-w32 | on | other | 4.938 | 0.162 |
| 1x2-w32 | on | scheduler | 8.141 | 4.213 |
| 1x2-w32 | on | solver | 1.612 | 0.042 |
| 2x2-w32 | accepted | G1_arithmetic | 34.570 | 0.000 |
| 2x2-w32 | accepted | G2_arithmetic | 23.803 | 0.000 |
| 2x2-w32 | accepted | epoch | 10.337 | 0.000 |
| 2x2-w32 | accepted | fft | 9.906 | 0.000 |
| 2x2-w32 | accepted | grouping | 1.778 | 0.000 |
| 2x2-w32 | accepted | inverse | 6.848 | 0.000 |
| 2x2-w32 | accepted | other | 5.541 | 0.000 |
| 2x2-w32 | accepted | scheduler | 5.049 | 0.000 |
| 2x2-w32 | accepted | solver | 2.167 | 0.000 |
| 2x2-w32 | off | G1_arithmetic | 34.937 | 0.367 |
| 2x2-w32 | off | G2_arithmetic | 23.803 | -0.000 |
| 2x2-w32 | off | epoch | 10.470 | 0.133 |
| 2x2-w32 | off | fft | 9.690 | -0.216 |
| 2x2-w32 | off | grouping | 1.568 | -0.210 |
| 2x2-w32 | off | inverse | 6.551 | -0.297 |
| 2x2-w32 | off | other | 5.524 | -0.018 |
| 2x2-w32 | off | scheduler | 5.051 | 0.002 |
| 2x2-w32 | off | solver | 2.406 | 0.239 |
| 2x2-w32 | on | G1_arithmetic | 34.300 | -0.270 |
| 2x2-w32 | on | G2_arithmetic | 24.268 | 0.465 |
| 2x2-w32 | on | epoch | 6.783 | -3.554 |
| 2x2-w32 | on | fft | 9.337 | -0.569 |
| 2x2-w32 | on | grouping | 1.661 | -0.117 |
| 2x2-w32 | on | inverse | 6.636 | -0.212 |
| 2x2-w32 | on | other | 5.331 | -0.211 |
| 2x2-w32 | on | scheduler | 9.422 | 4.373 |
| 2x2-w32 | on | solver | 2.263 | 0.095 |
| merge-54x1-w32 | accepted | G1_arithmetic | 27.509 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 13.515 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.326 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.525 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.411 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.912 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.356 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 1.963 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.482 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 25.825 | -1.683 |
| merge-54x1-w32 | off | G2_arithmetic | 16.378 | 2.863 |
| merge-54x1-w32 | off | epoch | 2.013 | -0.313 |
| merge-54x1-w32 | off | fft | 41.334 | -1.191 |
| merge-54x1-w32 | off | grouping | 1.413 | 0.002 |
| merge-54x1-w32 | off | inverse | 1.774 | -0.138 |
| merge-54x1-w32 | off | other | 4.215 | 0.859 |
| merge-54x1-w32 | off | scheduler | 2.248 | 0.285 |
| merge-54x1-w32 | off | solver | 4.798 | -0.684 |
| merge-54x1-w32 | on | G1_arithmetic | 25.557 | -1.951 |
| merge-54x1-w32 | on | G2_arithmetic | 16.343 | 2.828 |
| merge-54x1-w32 | on | epoch | 1.300 | -1.027 |
| merge-54x1-w32 | on | fft | 40.911 | -1.615 |
| merge-54x1-w32 | on | grouping | 1.466 | 0.055 |
| merge-54x1-w32 | on | inverse | 1.668 | -0.244 |
| merge-54x1-w32 | on | other | 4.432 | 1.076 |
| merge-54x1-w32 | on | scheduler | 3.259 | 1.295 |
| merge-54x1-w32 | on | solver | 5.064 | -0.418 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.651936 | 37.000 | 4.276× | 24.000 | 2.774× | false |
| 1x2-w32 | off | 8.580181 | 37.000 | 4.312× | 24.000 | 2.797× | false |
| 1x2-w32 | on | 8.573519 | 37.000 | 4.316× | 24.000 | 2.799× | false |
| 2x2-w32 | accepted | 11.746021 | 59.000 | 5.023× | 30.000 | 2.554× | false |
| 2x2-w32 | off | 11.727406 | 59.000 | 5.031× | 30.000 | 2.558× | false |
| 2x2-w32 | on | 11.660761 | 59.000 | 5.060× | 30.000 | 2.573× | false |

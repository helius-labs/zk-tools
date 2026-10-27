# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.808308 | 1.000000 | — | — | — |
| 12x2-w32 | fresh | 34.763122 | 1.001899 | — | — | — |
| 12x2-w32 | overlap | 34.730652 | 1.000522 | — | — | — |
| 16x2-w32 | accepted | 40.928505 | 1.000000 | — | — | — |
| 16x2-w32 | fresh | 40.950975 | 0.988056 | — | — | — |
| 16x2-w32 | overlap | 40.966982 | 0.992709 | — | — | — |
| 1x2-w16 | accepted | 10.609489 | 1.000000 | — | — | — |
| 1x2-w16 | fresh | 10.509091 | 0.996491 | — | — | — |
| 1x2-w16 | overlap | 10.526781 | 0.992048 | — | — | — |
| 1x2-w32 | accepted | 8.627601 | 1.000000 | 199.851 | 8.726258 | 130.75 |
| 1x2-w32 | fresh | 8.688925 | 1.008728 | 201.186 | 8.819757 | 136.25 |
| 1x2-w32 | overlap | 8.629174 | 1.000182 | 200.987 | 8.787397 | 134.25 |
| 24x2-w32 | accepted | 68.861844 | 1.000000 | — | — | — |
| 24x2-w32 | fresh | 68.750797 | 0.998579 | — | — | — |
| 24x2-w32 | overlap | 69.298346 | 1.000615 | — | — | — |
| 2x2-w32 | accepted | 11.690138 | 1.000000 | — | — | — |
| 2x2-w32 | fresh | 11.739374 | 1.003330 | — | — | — |
| 2x2-w32 | overlap | 11.733970 | 1.002730 | — | — | — |
| 4x2-w32 | accepted | 16.148044 | 1.000000 | 429.983 | 16.405072 | 369.00 |
| 4x2-w32 | fresh | 16.230559 | 1.004884 | 425.545 | 16.416777 | 380.50 |
| 4x2-w32 | overlap | 16.153785 | 1.005310 | 424.138 | 16.283934 | 383.25 |
| 8x2-w32 | accepted | 24.807932 | 1.000000 | — | — | — |
| 8x2-w32 | fresh | 25.040778 | 0.997905 | — | — | — |
| 8x2-w32 | overlap | 24.981203 | 1.017711 | — | — | — |
| merge-24x1-w32 | accepted | 67.293156 | 1.000000 | — | — | — |
| merge-24x1-w32 | fresh | 68.054612 | 1.012590 | — | — | — |
| merge-24x1-w32 | overlap | 67.410162 | 1.001493 | — | — | — |
| merge-54x1-w32 | accepted | 175.217487 | 1.000000 | — | — | — |
| merge-54x1-w32 | fresh | 176.972746 | 1.014293 | — | — | — |
| merge-54x1-w32 | overlap | 175.723065 | 1.007932 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 39.294 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 27.888 | 0.000 |
| 1x2-w32 | accepted | epoch | 8.823 | 0.000 |
| 1x2-w32 | accepted | fft | 4.608 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.364 | 0.000 |
| 1x2-w32 | accepted | inverse | 8.742 | 0.000 |
| 1x2-w32 | accepted | other | 4.725 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.281 | 0.000 |
| 1x2-w32 | accepted | solver | 1.276 | 0.000 |
| 1x2-w32 | fresh | G1_arithmetic | 38.838 | -0.456 |
| 1x2-w32 | fresh | G2_arithmetic | 27.473 | -0.415 |
| 1x2-w32 | fresh | epoch | 10.706 | 1.883 |
| 1x2-w32 | fresh | fft | 4.855 | 0.247 |
| 1x2-w32 | fresh | grouping | 0.739 | 0.375 |
| 1x2-w32 | fresh | inverse | 6.875 | -1.867 |
| 1x2-w32 | fresh | other | 5.041 | 0.317 |
| 1x2-w32 | fresh | scheduler | 3.921 | -0.360 |
| 1x2-w32 | fresh | solver | 1.552 | 0.276 |
| 1x2-w32 | overlap | G1_arithmetic | 39.582 | 0.288 |
| 1x2-w32 | overlap | G2_arithmetic | 27.538 | -0.350 |
| 1x2-w32 | overlap | epoch | 9.157 | 0.334 |
| 1x2-w32 | overlap | fft | 4.549 | -0.059 |
| 1x2-w32 | overlap | grouping | 0.540 | 0.176 |
| 1x2-w32 | overlap | inverse | 7.933 | -0.809 |
| 1x2-w32 | overlap | other | 5.104 | 0.379 |
| 1x2-w32 | overlap | scheduler | 4.276 | -0.004 |
| 1x2-w32 | overlap | solver | 1.321 | 0.045 |
| 4x2-w32 | accepted | G1_arithmetic | 34.981 | 0.000 |
| 4x2-w32 | accepted | G2_arithmetic | 19.135 | 0.000 |
| 4x2-w32 | accepted | epoch | 10.143 | 0.000 |
| 4x2-w32 | accepted | fft | 14.539 | 0.000 |
| 4x2-w32 | accepted | grouping | 1.599 | 0.000 |
| 4x2-w32 | accepted | inverse | 4.167 | 0.000 |
| 4x2-w32 | accepted | other | 6.650 | 0.000 |
| 4x2-w32 | accepted | scheduler | 5.270 | 0.000 |
| 4x2-w32 | accepted | solver | 3.516 | 0.000 |
| 4x2-w32 | fresh | G1_arithmetic | 34.195 | -0.787 |
| 4x2-w32 | fresh | G2_arithmetic | 19.413 | 0.278 |
| 4x2-w32 | fresh | epoch | 10.789 | 0.646 |
| 4x2-w32 | fresh | fft | 14.909 | 0.370 |
| 4x2-w32 | fresh | grouping | 1.820 | 0.221 |
| 4x2-w32 | fresh | inverse | 4.116 | -0.051 |
| 4x2-w32 | fresh | other | 6.095 | -0.555 |
| 4x2-w32 | fresh | scheduler | 4.672 | -0.598 |
| 4x2-w32 | fresh | solver | 3.991 | 0.475 |
| 4x2-w32 | overlap | G1_arithmetic | 35.556 | 0.575 |
| 4x2-w32 | overlap | G2_arithmetic | 19.088 | -0.047 |
| 4x2-w32 | overlap | epoch | 10.852 | 0.709 |
| 4x2-w32 | overlap | fft | 14.532 | -0.007 |
| 4x2-w32 | overlap | grouping | 1.330 | -0.269 |
| 4x2-w32 | overlap | inverse | 4.085 | -0.081 |
| 4x2-w32 | overlap | other | 6.162 | -0.488 |
| 4x2-w32 | overlap | scheduler | 4.697 | -0.573 |
| 4x2-w32 | overlap | solver | 3.698 | 0.182 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.627601 | 37.000 | 4.289× | 24.000 | 2.782× | false |
| 1x2-w32 | fresh | 8.688925 | 37.000 | 4.258× | 24.000 | 2.762× | false |
| 1x2-w32 | overlap | 8.629174 | 37.000 | 4.288× | 24.000 | 2.781× | false |
| 2x2-w32 | accepted | 11.690138 | 59.000 | 5.047× | 30.000 | 2.566× | false |
| 2x2-w32 | fresh | 11.739374 | 59.000 | 5.026× | 30.000 | 2.556× | false |
| 2x2-w32 | overlap | 11.733970 | 59.000 | 5.028× | 30.000 | 2.557× | false |
| 4x2-w32 | accepted | 16.148044 | 109.000 | 6.750× | 48.000 | 2.972× | false |
| 4x2-w32 | fresh | 16.230559 | 109.000 | 6.716× | 48.000 | 2.957× | false |
| 4x2-w32 | overlap | 16.153785 | 109.000 | 6.748× | 48.000 | 2.971× | false |
| 8x2-w32 | accepted | 24.807932 | 189.000 | 7.619× | 74.000 | 2.983× | false |
| 8x2-w32 | fresh | 25.040778 | 189.000 | 7.548× | 74.000 | 2.955× | false |
| 8x2-w32 | overlap | 24.981203 | 189.000 | 7.566× | 74.000 | 2.962× | false |
| 12x2-w32 | accepted | 34.808308 | 284.000 | 8.159× | 101.000 | 2.902× | false |
| 12x2-w32 | fresh | 34.763122 | 284.000 | 8.170× | 101.000 | 2.905× | false |
| 12x2-w32 | overlap | 34.730652 | 284.000 | 8.177× | 101.000 | 2.908× | false |
| 16x2-w32 | accepted | 40.928505 | 335.000 | 8.185× | 124.000 | 3.030× | false |
| 16x2-w32 | fresh | 40.950975 | 335.000 | 8.181× | 124.000 | 3.028× | false |
| 16x2-w32 | overlap | 40.966982 | 335.000 | 8.177× | 124.000 | 3.027× | false |
| 24x2-w32 | accepted | 68.861844 | 528.000 | 7.668× | 176.000 | 2.556× | false |
| 24x2-w32 | fresh | 68.750797 | 528.000 | 7.680× | 176.000 | 2.560× | false |
| 24x2-w32 | overlap | 69.298346 | 528.000 | 7.619× | 176.000 | 2.540× | false |
| merge-24x1-w32 | accepted | 67.293156 | 507.000 | 7.534× | 175.000 | 2.601× | false |
| merge-24x1-w32 | fresh | 68.054612 | 507.000 | 7.450× | 175.000 | 2.571× | false |
| merge-24x1-w32 | overlap | 67.410162 | 507.000 | 7.521× | 175.000 | 2.596× | false |
| 1x2-w16 | accepted | 10.609489 | 37.000 | 3.487× | 24.000 | 2.262× | false |
| 1x2-w16 | fresh | 10.509091 | 37.000 | 3.521× | 24.000 | 2.284× | false |
| 1x2-w16 | overlap | 10.526781 | 37.000 | 3.515× | 24.000 | 2.280× | false |

# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.637390 | 1.000000 | — | — | — |
| 12x2-w32 | composed | 33.931699 | 0.985598 | — | — | — |
| 12x2-w32 | fresh | 34.553623 | 0.996173 | — | — | — |
| 16x2-w32 | accepted | 40.812446 | 1.000000 | — | — | — |
| 16x2-w32 | composed | 40.287411 | 0.985305 | — | — | — |
| 16x2-w32 | fresh | 41.070628 | 1.003177 | — | — | — |
| 1x2-w16 | accepted | 10.617007 | 1.000000 | — | — | — |
| 1x2-w16 | composed | 10.070846 | 0.960044 | — | — | — |
| 1x2-w16 | fresh | 10.735592 | 1.019994 | — | — | — |
| 1x2-w32 | accepted | 8.653411 | 1.000000 | 199.652 | 8.802610 | 135.25 |
| 1x2-w32 | composed | 8.372140 | 0.968653 | 192.070 | 8.490148 | 133.25 |
| 1x2-w32 | fresh | 8.704628 | 1.002363 | 200.850 | 8.767990 | 132.00 |
| 24x2-w32 | accepted | 68.503602 | 1.000000 | — | — | — |
| 24x2-w32 | composed | 67.228752 | 0.982660 | — | — | — |
| 24x2-w32 | fresh | 68.624285 | 0.996821 | — | — | — |
| 2x2-w32 | accepted | 11.748332 | 1.000000 | — | — | — |
| 2x2-w32 | composed | 11.444740 | 0.975281 | — | — | — |
| 2x2-w32 | fresh | 11.787626 | 1.004568 | — | — | — |
| 4x2-w32 | accepted | 16.160088 | 1.000000 | 440.993 | 16.834227 | 381.75 |
| 4x2-w32 | composed | 16.088793 | 0.987697 | 428.600 | 16.327565 | 372.50 |
| 4x2-w32 | fresh | 16.210183 | 0.999875 | 425.840 | 16.325118 | 375.00 |
| 8x2-w32 | accepted | 24.714464 | 1.000000 | — | — | — |
| 8x2-w32 | composed | 23.986009 | 0.973851 | — | — | — |
| 8x2-w32 | fresh | 24.837886 | 1.004726 | — | — | — |
| merge-24x1-w32 | accepted | 67.641491 | 1.000000 | — | — | — |
| merge-24x1-w32 | composed | 66.259942 | 0.976213 | — | — | — |
| merge-24x1-w32 | fresh | 67.457441 | 0.999004 | — | — | — |
| merge-54x1-w32 | accepted | 175.446051 | 1.000000 | — | — | — |
| merge-54x1-w32 | composed | 163.533643 | 0.933918 | — | — | — |
| merge-54x1-w32 | fresh | 176.983299 | 1.009022 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 39.135 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 27.874 | 0.000 |
| 1x2-w32 | accepted | epoch | 10.253 | 0.000 |
| 1x2-w32 | accepted | fft | 4.595 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.740 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.796 | 0.000 |
| 1x2-w32 | accepted | other | 5.867 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.605 | 0.000 |
| 1x2-w32 | accepted | solver | 1.136 | 0.000 |
| 1x2-w32 | composed | G1_arithmetic | 36.701 | -2.434 |
| 1x2-w32 | composed | G2_arithmetic | 29.398 | 1.524 |
| 1x2-w32 | composed | epoch | 10.728 | 0.476 |
| 1x2-w32 | composed | fft | 4.984 | 0.389 |
| 1x2-w32 | composed | grouping | 0.638 | -0.102 |
| 1x2-w32 | composed | inverse | 7.813 | 1.016 |
| 1x2-w32 | composed | other | 4.477 | -1.390 |
| 1x2-w32 | composed | scheduler | 3.744 | 0.140 |
| 1x2-w32 | composed | solver | 1.516 | 0.380 |
| 1x2-w32 | fresh | G1_arithmetic | 38.388 | -0.747 |
| 1x2-w32 | fresh | G2_arithmetic | 29.096 | 1.222 |
| 1x2-w32 | fresh | epoch | 9.767 | -0.485 |
| 1x2-w32 | fresh | fft | 4.128 | -0.467 |
| 1x2-w32 | fresh | grouping | 0.727 | -0.013 |
| 1x2-w32 | fresh | inverse | 7.257 | 0.461 |
| 1x2-w32 | fresh | other | 5.597 | -0.270 |
| 1x2-w32 | fresh | scheduler | 3.532 | -0.072 |
| 1x2-w32 | fresh | solver | 1.507 | 0.371 |
| 4x2-w32 | accepted | G1_arithmetic | 35.126 | 0.000 |
| 4x2-w32 | accepted | G2_arithmetic | 18.261 | 0.000 |
| 4x2-w32 | accepted | epoch | 11.129 | 0.000 |
| 4x2-w32 | accepted | fft | 14.689 | 0.000 |
| 4x2-w32 | accepted | grouping | 1.531 | 0.000 |
| 4x2-w32 | accepted | inverse | 3.968 | 0.000 |
| 4x2-w32 | accepted | other | 6.577 | 0.000 |
| 4x2-w32 | accepted | scheduler | 4.978 | 0.000 |
| 4x2-w32 | accepted | solver | 3.742 | 0.000 |
| 4x2-w32 | composed | G1_arithmetic | 34.603 | -0.522 |
| 4x2-w32 | composed | G2_arithmetic | 18.629 | 0.368 |
| 4x2-w32 | composed | epoch | 10.622 | -0.507 |
| 4x2-w32 | composed | fft | 15.252 | 0.564 |
| 4x2-w32 | composed | grouping | 1.419 | -0.112 |
| 4x2-w32 | composed | inverse | 4.254 | 0.286 |
| 4x2-w32 | composed | other | 6.177 | -0.400 |
| 4x2-w32 | composed | scheduler | 5.421 | 0.443 |
| 4x2-w32 | composed | solver | 3.623 | -0.120 |
| 4x2-w32 | fresh | G1_arithmetic | 35.587 | 0.462 |
| 4x2-w32 | fresh | G2_arithmetic | 19.048 | 0.788 |
| 4x2-w32 | fresh | epoch | 10.480 | -0.648 |
| 4x2-w32 | fresh | fft | 14.188 | -0.501 |
| 4x2-w32 | fresh | grouping | 1.827 | 0.296 |
| 4x2-w32 | fresh | inverse | 4.049 | 0.082 |
| 4x2-w32 | fresh | other | 6.846 | 0.269 |
| 4x2-w32 | fresh | scheduler | 4.535 | -0.443 |
| 4x2-w32 | fresh | solver | 3.438 | -0.304 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.653411 | 37.000 | 4.276× | 24.000 | 2.773× | false |
| 1x2-w32 | fresh | 8.704628 | 37.000 | 4.251× | 24.000 | 2.757× | false |
| 1x2-w32 | composed | 8.372140 | 37.000 | 4.419× | 24.000 | 2.867× | false |
| 2x2-w32 | accepted | 11.748332 | 59.000 | 5.022× | 30.000 | 2.554× | false |
| 2x2-w32 | fresh | 11.787626 | 59.000 | 5.005× | 30.000 | 2.545× | false |
| 2x2-w32 | composed | 11.444740 | 59.000 | 5.155× | 30.000 | 2.621× | false |
| 4x2-w32 | accepted | 16.160088 | 109.000 | 6.745× | 48.000 | 2.970× | false |
| 4x2-w32 | fresh | 16.210183 | 109.000 | 6.724× | 48.000 | 2.961× | false |
| 4x2-w32 | composed | 16.088793 | 109.000 | 6.775× | 48.000 | 2.983× | false |
| 8x2-w32 | accepted | 24.714464 | 189.000 | 7.647× | 74.000 | 2.994× | false |
| 8x2-w32 | fresh | 24.837886 | 189.000 | 7.609× | 74.000 | 2.979× | false |
| 8x2-w32 | composed | 23.986009 | 189.000 | 7.880× | 74.000 | 3.085× | false |
| 12x2-w32 | accepted | 34.637390 | 284.000 | 8.199× | 101.000 | 2.916× | false |
| 12x2-w32 | fresh | 34.553623 | 284.000 | 8.219× | 101.000 | 2.923× | false |
| 12x2-w32 | composed | 33.931699 | 284.000 | 8.370× | 101.000 | 2.977× | false |
| 16x2-w32 | accepted | 40.812446 | 335.000 | 8.208× | 124.000 | 3.038× | false |
| 16x2-w32 | fresh | 41.070628 | 335.000 | 8.157× | 124.000 | 3.019× | false |
| 16x2-w32 | composed | 40.287411 | 335.000 | 8.315× | 124.000 | 3.078× | false |
| 24x2-w32 | accepted | 68.503602 | 528.000 | 7.708× | 176.000 | 2.569× | false |
| 24x2-w32 | fresh | 68.624285 | 528.000 | 7.694× | 176.000 | 2.565× | false |
| 24x2-w32 | composed | 67.228752 | 528.000 | 7.854× | 176.000 | 2.618× | false |
| merge-24x1-w32 | accepted | 67.641491 | 507.000 | 7.495× | 175.000 | 2.587× | false |
| merge-24x1-w32 | fresh | 67.457441 | 507.000 | 7.516× | 175.000 | 2.594× | false |
| merge-24x1-w32 | composed | 66.259942 | 507.000 | 7.652× | 175.000 | 2.641× | false |
| 1x2-w16 | accepted | 10.617007 | 37.000 | 3.485× | 24.000 | 2.261× | false |
| 1x2-w16 | fresh | 10.735592 | 37.000 | 3.446× | 24.000 | 2.236× | false |
| 1x2-w16 | composed | 10.070846 | 37.000 | 3.674× | 24.000 | 2.383× | false |

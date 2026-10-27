# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.713975 | 1.000000 | — | — | — |
| 12x2-w32 | fresh | 34.785423 | 1.008480 | — | — | — |
| 12x2-w32 | parent | 34.141535 | 0.987242 | — | — | — |
| 12x2-w32 | split | 34.209087 | 0.989153 | — | — | — |
| 16x2-w32 | accepted | 41.309212 | 1.000000 | — | — | — |
| 16x2-w32 | fresh | 40.667285 | 0.997684 | — | — | — |
| 16x2-w32 | parent | 40.222098 | 0.977439 | — | — | — |
| 16x2-w32 | split | 40.496994 | 0.991411 | — | — | — |
| 1x2-w16 | accepted | 10.907017 | 1.000000 | — | — | — |
| 1x2-w16 | fresh | 10.708150 | 0.976229 | — | — | — |
| 1x2-w16 | parent | 10.548617 | 0.981274 | — | — | — |
| 1x2-w16 | split | 10.581158 | 0.979879 | — | — | — |
| 1x2-w32 | accepted | 8.636699 | 1.000000 | 199.515 | 8.803091 | 135.25 |
| 1x2-w32 | fresh | 8.673857 | 1.011105 | 200.949 | 8.796271 | 139.50 |
| 1x2-w32 | parent | 8.606502 | 0.996504 | 200.131 | 8.720997 | 136.00 |
| 1x2-w32 | split | 8.601051 | 1.003789 | 199.480 | 8.737644 | 134.50 |
| 24x2-w32 | accepted | 68.458550 | 1.000000 | — | — | — |
| 24x2-w32 | fresh | 68.281446 | 0.997351 | — | — | — |
| 24x2-w32 | parent | 67.237206 | 0.980832 | — | — | — |
| 24x2-w32 | split | 66.936831 | 0.981399 | — | — | — |
| 2x2-w32 | accepted | 11.715993 | 1.000000 | — | — | — |
| 2x2-w32 | fresh | 11.787568 | 1.006429 | — | — | — |
| 2x2-w32 | parent | 11.781846 | 1.007754 | — | — | — |
| 2x2-w32 | split | 11.805784 | 1.005812 | — | — | — |
| 4x2-w32 | accepted | 16.108469 | 1.000000 | 435.082 | 16.636278 | 367.00 |
| 4x2-w32 | fresh | 16.194141 | 1.000632 | 437.155 | 16.777182 | 372.25 |
| 4x2-w32 | parent | 16.147230 | 1.004123 | 437.164 | 16.743109 | 372.00 |
| 4x2-w32 | split | 16.222611 | 0.999186 | 421.910 | 16.193178 | 368.25 |
| 8x2-w32 | accepted | 24.897283 | 1.000000 | — | — | — |
| 8x2-w32 | fresh | 24.641225 | 1.006140 | — | — | — |
| 8x2-w32 | parent | 24.240085 | 0.974757 | — | — | — |
| 8x2-w32 | split | 24.377798 | 0.967099 | — | — | — |
| merge-24x1-w32 | accepted | 67.302685 | 1.000000 | — | — | — |
| merge-24x1-w32 | fresh | 67.112920 | 0.997067 | — | — | — |
| merge-24x1-w32 | parent | 66.069487 | 0.984321 | — | — | — |
| merge-24x1-w32 | split | 66.354462 | 0.987127 | — | — | — |
| merge-54x1-w32 | accepted | 175.768723 | 1.000000 | — | — | — |
| merge-54x1-w32 | fresh | 176.039642 | 1.004428 | — | — | — |
| merge-54x1-w32 | parent | 163.055570 | 0.928994 | — | — | — |
| merge-54x1-w32 | split | 163.703940 | 0.927104 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.850 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.037 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.371 | 0.000 |
| 1x2-w32 | accepted | fft | 4.482 | 0.000 |
| 1x2-w32 | accepted | grouping | 1.089 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.984 | 0.000 |
| 1x2-w32 | accepted | other | 4.631 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.106 | 0.000 |
| 1x2-w32 | accepted | solver | 1.450 | 0.000 |
| 1x2-w32 | fresh | G1_arithmetic | 39.522 | 0.672 |
| 1x2-w32 | fresh | G2_arithmetic | 27.803 | -1.234 |
| 1x2-w32 | fresh | epoch | 9.419 | 0.048 |
| 1x2-w32 | fresh | fft | 5.502 | 1.020 |
| 1x2-w32 | fresh | grouping | 0.554 | -0.535 |
| 1x2-w32 | fresh | inverse | 6.535 | -0.449 |
| 1x2-w32 | fresh | other | 5.340 | 0.709 |
| 1x2-w32 | fresh | scheduler | 4.228 | 0.122 |
| 1x2-w32 | fresh | solver | 1.097 | -0.353 |
| 1x2-w32 | parent | G1_arithmetic | 38.814 | -0.036 |
| 1x2-w32 | parent | G2_arithmetic | 28.215 | -0.821 |
| 1x2-w32 | parent | epoch | 10.302 | 0.931 |
| 1x2-w32 | parent | fft | 5.092 | 0.609 |
| 1x2-w32 | parent | grouping | 0.542 | -0.547 |
| 1x2-w32 | parent | inverse | 6.748 | -0.236 |
| 1x2-w32 | parent | other | 4.734 | 0.103 |
| 1x2-w32 | parent | scheduler | 4.011 | -0.095 |
| 1x2-w32 | parent | solver | 1.542 | 0.092 |
| 1x2-w32 | split | G1_arithmetic | 39.276 | 0.426 |
| 1x2-w32 | split | G2_arithmetic | 28.533 | -0.504 |
| 1x2-w32 | split | epoch | 8.550 | -0.821 |
| 1x2-w32 | split | fft | 5.282 | 0.800 |
| 1x2-w32 | split | grouping | 0.683 | -0.406 |
| 1x2-w32 | split | inverse | 6.668 | -0.317 |
| 1x2-w32 | split | other | 4.819 | 0.188 |
| 1x2-w32 | split | scheduler | 4.803 | 0.698 |
| 1x2-w32 | split | solver | 1.386 | -0.064 |
| 4x2-w32 | accepted | G1_arithmetic | 34.808 | 0.000 |
| 4x2-w32 | accepted | G2_arithmetic | 18.503 | 0.000 |
| 4x2-w32 | accepted | epoch | 10.370 | 0.000 |
| 4x2-w32 | accepted | fft | 14.947 | 0.000 |
| 4x2-w32 | accepted | grouping | 1.473 | 0.000 |
| 4x2-w32 | accepted | inverse | 3.756 | 0.000 |
| 4x2-w32 | accepted | other | 6.738 | 0.000 |
| 4x2-w32 | accepted | scheduler | 5.531 | 0.000 |
| 4x2-w32 | accepted | solver | 3.873 | 0.000 |
| 4x2-w32 | fresh | G1_arithmetic | 34.563 | -0.244 |
| 4x2-w32 | fresh | G2_arithmetic | 18.924 | 0.421 |
| 4x2-w32 | fresh | epoch | 10.948 | 0.577 |
| 4x2-w32 | fresh | fft | 14.487 | -0.460 |
| 4x2-w32 | fresh | grouping | 1.466 | -0.007 |
| 4x2-w32 | fresh | inverse | 3.924 | 0.167 |
| 4x2-w32 | fresh | other | 6.210 | -0.528 |
| 4x2-w32 | fresh | scheduler | 5.568 | 0.037 |
| 4x2-w32 | fresh | solver | 3.911 | 0.037 |
| 4x2-w32 | parent | G1_arithmetic | 33.941 | -0.867 |
| 4x2-w32 | parent | G2_arithmetic | 18.407 | -0.096 |
| 4x2-w32 | parent | epoch | 10.441 | 0.070 |
| 4x2-w32 | parent | fft | 16.342 | 1.395 |
| 4x2-w32 | parent | grouping | 1.783 | 0.310 |
| 4x2-w32 | parent | inverse | 4.277 | 0.521 |
| 4x2-w32 | parent | other | 6.633 | -0.105 |
| 4x2-w32 | parent | scheduler | 4.922 | -0.609 |
| 4x2-w32 | parent | solver | 3.254 | -0.619 |
| 4x2-w32 | split | G1_arithmetic | 35.248 | 0.440 |
| 4x2-w32 | split | G2_arithmetic | 17.966 | -0.537 |
| 4x2-w32 | split | epoch | 10.293 | -0.077 |
| 4x2-w32 | split | fft | 15.001 | 0.055 |
| 4x2-w32 | split | grouping | 1.436 | -0.037 |
| 4x2-w32 | split | inverse | 4.360 | 0.604 |
| 4x2-w32 | split | other | 6.706 | -0.033 |
| 4x2-w32 | split | scheduler | 5.430 | -0.101 |
| 4x2-w32 | split | solver | 3.560 | -0.314 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.636699 | 37.000 | 4.284× | 24.000 | 2.779× | false |
| 1x2-w32 | fresh | 8.673857 | 37.000 | 4.266× | 24.000 | 2.767× | false |
| 1x2-w32 | parent | 8.606502 | 37.000 | 4.299× | 24.000 | 2.789× | false |
| 1x2-w32 | split | 8.601051 | 37.000 | 4.302× | 24.000 | 2.790× | false |
| 2x2-w32 | accepted | 11.715993 | 59.000 | 5.036× | 30.000 | 2.561× | false |
| 2x2-w32 | fresh | 11.787568 | 59.000 | 5.005× | 30.000 | 2.545× | false |
| 2x2-w32 | parent | 11.781846 | 59.000 | 5.008× | 30.000 | 2.546× | false |
| 2x2-w32 | split | 11.805784 | 59.000 | 4.998× | 30.000 | 2.541× | false |
| 4x2-w32 | accepted | 16.108469 | 109.000 | 6.767× | 48.000 | 2.980× | false |
| 4x2-w32 | fresh | 16.194141 | 109.000 | 6.731× | 48.000 | 2.964× | false |
| 4x2-w32 | parent | 16.147230 | 109.000 | 6.750× | 48.000 | 2.973× | false |
| 4x2-w32 | split | 16.222611 | 109.000 | 6.719× | 48.000 | 2.959× | false |
| 8x2-w32 | accepted | 24.897283 | 189.000 | 7.591× | 74.000 | 2.972× | false |
| 8x2-w32 | fresh | 24.641225 | 189.000 | 7.670× | 74.000 | 3.003× | false |
| 8x2-w32 | parent | 24.240085 | 189.000 | 7.797× | 74.000 | 3.053× | false |
| 8x2-w32 | split | 24.377798 | 189.000 | 7.753× | 74.000 | 3.036× | false |
| 12x2-w32 | accepted | 34.713975 | 284.000 | 8.181× | 101.000 | 2.909× | false |
| 12x2-w32 | fresh | 34.785423 | 284.000 | 8.164× | 101.000 | 2.904× | false |
| 12x2-w32 | parent | 34.141535 | 284.000 | 8.318× | 101.000 | 2.958× | false |
| 12x2-w32 | split | 34.209087 | 284.000 | 8.302× | 101.000 | 2.952× | false |
| 16x2-w32 | accepted | 41.309212 | 335.000 | 8.110× | 124.000 | 3.002× | false |
| 16x2-w32 | fresh | 40.667285 | 335.000 | 8.238× | 124.000 | 3.049× | false |
| 16x2-w32 | parent | 40.222098 | 335.000 | 8.329× | 124.000 | 3.083× | false |
| 16x2-w32 | split | 40.496994 | 335.000 | 8.272× | 124.000 | 3.062× | false |
| 24x2-w32 | accepted | 68.458550 | 528.000 | 7.713× | 176.000 | 2.571× | false |
| 24x2-w32 | fresh | 68.281446 | 528.000 | 7.733× | 176.000 | 2.578× | false |
| 24x2-w32 | parent | 67.237206 | 528.000 | 7.853× | 176.000 | 2.618× | false |
| 24x2-w32 | split | 66.936831 | 528.000 | 7.888× | 176.000 | 2.629× | false |
| merge-24x1-w32 | accepted | 67.302685 | 507.000 | 7.533× | 175.000 | 2.600× | false |
| merge-24x1-w32 | fresh | 67.112920 | 507.000 | 7.554× | 175.000 | 2.608× | false |
| merge-24x1-w32 | parent | 66.069487 | 507.000 | 7.674× | 175.000 | 2.649× | false |
| merge-24x1-w32 | split | 66.354462 | 507.000 | 7.641× | 175.000 | 2.637× | false |
| 1x2-w16 | accepted | 10.907017 | 37.000 | 3.392× | 24.000 | 2.200× | false |
| 1x2-w16 | fresh | 10.708150 | 37.000 | 3.455× | 24.000 | 2.241× | false |
| 1x2-w16 | parent | 10.548617 | 37.000 | 3.508× | 24.000 | 2.275× | false |
| 1x2-w16 | split | 10.581158 | 37.000 | 3.497× | 24.000 | 2.268× | false |

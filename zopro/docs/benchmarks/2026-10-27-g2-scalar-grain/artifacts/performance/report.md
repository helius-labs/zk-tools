# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.092441 | 1.000000 | — | — | — |
| 12x2-w32 | candidate | 34.125382 | 0.998894 | — | — | — |
| 12x2-w32 | fresh | 34.072393 | 0.999412 | — | — | — |
| 16x2-w32 | accepted | 40.263725 | 1.000000 | — | — | — |
| 16x2-w32 | candidate | 40.233292 | 0.997544 | — | — | — |
| 16x2-w32 | fresh | 40.061768 | 0.994454 | — | — | — |
| 1x2-w16 | accepted | 10.281682 | 1.000000 | — | — | — |
| 1x2-w16 | candidate | 10.063110 | 0.978742 | — | — | — |
| 1x2-w16 | fresh | 10.247887 | 0.995227 | — | — | — |
| 1x2-w32 | accepted | 8.332936 | 1.000000 | 192.511 | 8.527877 | 135.25 |
| 1x2-w32 | candidate | 8.321766 | 0.997331 | 192.535 | 8.435465 | 135.50 |
| 1x2-w32 | fresh | 8.361881 | 1.001548 | 193.093 | 8.511997 | 132.25 |
| 24x2-w32 | accepted | 67.238808 | 1.000000 | — | — | — |
| 24x2-w32 | candidate | 67.022116 | 0.998752 | — | — | — |
| 24x2-w32 | fresh | 67.054452 | 0.997352 | — | — | — |
| 2x2-w32 | accepted | 11.486945 | 1.000000 | 262.484 | 11.486720 | 321.00 |
| 2x2-w32 | candidate | 11.444545 | 0.993770 | 261.914 | 11.372698 | 319.50 |
| 2x2-w32 | fresh | 11.447269 | 0.992561 | 267.471 | 11.617256 | 325.25 |
| 4x2-w32 | accepted | 15.978927 | 1.000000 | — | — | — |
| 4x2-w32 | candidate | 15.803015 | 0.989193 | — | — | — |
| 4x2-w32 | fresh | 15.928882 | 1.002112 | — | — | — |
| 8x2-w32 | accepted | 24.088421 | 1.000000 | — | — | — |
| 8x2-w32 | candidate | 24.169805 | 0.992863 | — | — | — |
| 8x2-w32 | fresh | 24.039950 | 0.994239 | — | — | — |
| merge-24x1-w32 | accepted | 65.979323 | 1.000000 | — | — | — |
| merge-24x1-w32 | candidate | 66.105108 | 1.002320 | — | — | — |
| merge-24x1-w32 | fresh | 66.451962 | 1.010000 | — | — | — |
| merge-54x1-w32 | accepted | 164.222695 | 1.000000 | — | — | — |
| merge-54x1-w32 | candidate | 163.813092 | 0.996211 | — | — | — |
| merge-54x1-w32 | fresh | 164.043742 | 1.002698 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.805 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.448 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.847 | 0.000 |
| 1x2-w32 | accepted | fft | 4.976 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.754 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.825 | 0.000 |
| 1x2-w32 | accepted | other | 4.171 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.479 | 0.000 |
| 1x2-w32 | accepted | solver | 1.694 | 0.000 |
| 1x2-w32 | candidate | G1_arithmetic | 36.559 | -2.247 |
| 1x2-w32 | candidate | G2_arithmetic | 29.976 | 1.529 |
| 1x2-w32 | candidate | epoch | 10.853 | 1.006 |
| 1x2-w32 | candidate | fft | 4.878 | -0.097 |
| 1x2-w32 | candidate | grouping | 0.704 | -0.050 |
| 1x2-w32 | candidate | inverse | 7.466 | -0.359 |
| 1x2-w32 | candidate | other | 4.575 | 0.403 |
| 1x2-w32 | candidate | scheduler | 3.572 | 0.093 |
| 1x2-w32 | candidate | solver | 1.416 | -0.278 |
| 1x2-w32 | fresh | G1_arithmetic | 36.363 | -2.443 |
| 1x2-w32 | fresh | G2_arithmetic | 30.462 | 2.014 |
| 1x2-w32 | fresh | epoch | 9.407 | -0.439 |
| 1x2-w32 | fresh | fft | 5.039 | 0.064 |
| 1x2-w32 | fresh | grouping | 0.662 | -0.093 |
| 1x2-w32 | fresh | inverse | 7.312 | -0.513 |
| 1x2-w32 | fresh | other | 5.502 | 1.331 |
| 1x2-w32 | fresh | scheduler | 3.734 | 0.255 |
| 1x2-w32 | fresh | solver | 1.519 | -0.176 |
| 2x2-w32 | accepted | G1_arithmetic | 33.246 | 0.000 |
| 2x2-w32 | accepted | G2_arithmetic | 25.464 | 0.000 |
| 2x2-w32 | accepted | epoch | 11.133 | 0.000 |
| 2x2-w32 | accepted | fft | 9.439 | 0.000 |
| 2x2-w32 | accepted | grouping | 1.864 | 0.000 |
| 2x2-w32 | accepted | inverse | 6.386 | 0.000 |
| 2x2-w32 | accepted | other | 5.253 | 0.000 |
| 2x2-w32 | accepted | scheduler | 5.070 | 0.000 |
| 2x2-w32 | accepted | solver | 2.144 | 0.000 |
| 2x2-w32 | candidate | G1_arithmetic | 33.470 | 0.224 |
| 2x2-w32 | candidate | G2_arithmetic | 24.424 | -1.040 |
| 2x2-w32 | candidate | epoch | 10.926 | -0.207 |
| 2x2-w32 | candidate | fft | 9.791 | 0.352 |
| 2x2-w32 | candidate | grouping | 1.832 | -0.033 |
| 2x2-w32 | candidate | inverse | 6.805 | 0.419 |
| 2x2-w32 | candidate | other | 5.687 | 0.434 |
| 2x2-w32 | candidate | scheduler | 4.638 | -0.432 |
| 2x2-w32 | candidate | solver | 2.427 | 0.284 |
| 2x2-w32 | fresh | G1_arithmetic | 32.427 | -0.819 |
| 2x2-w32 | fresh | G2_arithmetic | 23.683 | -1.781 |
| 2x2-w32 | fresh | epoch | 11.948 | 0.815 |
| 2x2-w32 | fresh | fft | 9.745 | 0.306 |
| 2x2-w32 | fresh | grouping | 1.796 | -0.069 |
| 2x2-w32 | fresh | inverse | 7.554 | 1.168 |
| 2x2-w32 | fresh | other | 5.459 | 0.206 |
| 2x2-w32 | fresh | scheduler | 4.782 | -0.288 |
| 2x2-w32 | fresh | solver | 2.606 | 0.462 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.332936 | 37.000 | 4.440× | 24.000 | 2.880× | false |
| 1x2-w32 | fresh | 8.361881 | 37.000 | 4.425× | 24.000 | 2.870× | false |
| 1x2-w32 | candidate | 8.321766 | 37.000 | 4.446× | 24.000 | 2.884× | false |
| 2x2-w32 | accepted | 11.486945 | 59.000 | 5.136× | 30.000 | 2.612× | false |
| 2x2-w32 | fresh | 11.447269 | 59.000 | 5.154× | 30.000 | 2.621× | false |
| 2x2-w32 | candidate | 11.444545 | 59.000 | 5.155× | 30.000 | 2.621× | false |
| 4x2-w32 | accepted | 15.978927 | 109.000 | 6.821× | 48.000 | 3.004× | false |
| 4x2-w32 | fresh | 15.928882 | 109.000 | 6.843× | 48.000 | 3.013× | false |
| 4x2-w32 | candidate | 15.803015 | 109.000 | 6.897× | 48.000 | 3.037× | false |
| 8x2-w32 | accepted | 24.088421 | 189.000 | 7.846× | 74.000 | 3.072× | false |
| 8x2-w32 | fresh | 24.039950 | 189.000 | 7.862× | 74.000 | 3.078× | false |
| 8x2-w32 | candidate | 24.169805 | 189.000 | 7.820× | 74.000 | 3.062× | false |
| 12x2-w32 | accepted | 34.092441 | 284.000 | 8.330× | 101.000 | 2.963× | false |
| 12x2-w32 | fresh | 34.072393 | 284.000 | 8.335× | 101.000 | 2.964× | false |
| 12x2-w32 | candidate | 34.125382 | 284.000 | 8.322× | 101.000 | 2.960× | false |
| 16x2-w32 | accepted | 40.263725 | 335.000 | 8.320× | 124.000 | 3.080× | false |
| 16x2-w32 | fresh | 40.061768 | 335.000 | 8.362× | 124.000 | 3.095× | false |
| 16x2-w32 | candidate | 40.233292 | 335.000 | 8.326× | 124.000 | 3.082× | false |
| 24x2-w32 | accepted | 67.238808 | 528.000 | 7.853× | 176.000 | 2.618× | false |
| 24x2-w32 | fresh | 67.054452 | 528.000 | 7.874× | 176.000 | 2.625× | false |
| 24x2-w32 | candidate | 67.022116 | 528.000 | 7.878× | 176.000 | 2.626× | false |
| merge-24x1-w32 | accepted | 65.979323 | 507.000 | 7.684× | 175.000 | 2.652× | false |
| merge-24x1-w32 | fresh | 66.451962 | 507.000 | 7.630× | 175.000 | 2.633× | false |
| merge-24x1-w32 | candidate | 66.105108 | 507.000 | 7.670× | 175.000 | 2.647× | false |
| 1x2-w16 | accepted | 10.281682 | 37.000 | 3.599× | 24.000 | 2.334× | false |
| 1x2-w16 | fresh | 10.247887 | 37.000 | 3.611× | 24.000 | 2.342× | false |
| 1x2-w16 | candidate | 10.063110 | 37.000 | 3.677× | 24.000 | 2.385× | false |

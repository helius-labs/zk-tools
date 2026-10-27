# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.164589 | 1.000000 | — | — | — |
| 12x2-w32 | candidate | 33.693397 | 0.988859 | — | — | — |
| 12x2-w32 | fresh | 33.974509 | 0.990323 | — | — | — |
| 16x2-w32 | accepted | 40.154939 | 1.000000 | — | — | — |
| 16x2-w32 | candidate | 39.981636 | 0.994780 | — | — | — |
| 16x2-w32 | fresh | 40.002315 | 0.998678 | — | — | — |
| 1x2-w16 | accepted | 10.168078 | 1.000000 | — | — | — |
| 1x2-w16 | candidate | 10.036855 | 0.988927 | — | — | — |
| 1x2-w16 | fresh | 10.078075 | 0.993680 | — | — | — |
| 1x2-w32 | accepted | 8.372564 | 1.000000 | 192.887 | 8.462485 | 132.25 |
| 1x2-w32 | candidate | 8.324853 | 0.996750 | 191.139 | 8.458937 | 139.00 |
| 1x2-w32 | fresh | 8.296841 | 0.994290 | 191.939 | 8.412703 | 137.00 |
| 24x2-w32 | accepted | 66.972265 | 1.000000 | — | — | — |
| 24x2-w32 | candidate | 66.010670 | 0.985642 | — | — | — |
| 24x2-w32 | fresh | 67.208254 | 1.004138 | — | — | — |
| 2x2-w32 | accepted | 11.425210 | 1.000000 | — | — | — |
| 2x2-w32 | candidate | 11.460675 | 0.995482 | — | — | — |
| 2x2-w32 | fresh | 11.430436 | 0.999371 | — | — | — |
| 4x2-w32 | accepted | 16.102861 | 1.000000 | — | — | — |
| 4x2-w32 | candidate | 15.854365 | 0.992302 | — | — | — |
| 4x2-w32 | fresh | 15.950026 | 0.997693 | — | — | — |
| 8x2-w32 | accepted | 24.382666 | 1.000000 | — | — | — |
| 8x2-w32 | candidate | 24.084626 | 0.999405 | — | — | — |
| 8x2-w32 | fresh | 24.162810 | 0.995474 | — | — | — |
| merge-24x1-w32 | accepted | 66.125565 | 1.000000 | — | — | — |
| merge-24x1-w32 | candidate | 65.086624 | 0.985864 | — | — | — |
| merge-24x1-w32 | fresh | 66.112380 | 0.998340 | — | — | — |
| merge-54x1-w32 | accepted | 164.132958 | 1.000000 | 4484.495 | 164.299675 | 2121.95 |
| merge-54x1-w32 | candidate | 161.341835 | 0.986882 | 4433.613 | 162.208962 | 2174.23 |
| merge-54x1-w32 | fresh | 163.761572 | 0.997838 | 4475.470 | 164.020172 | 2162.60 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 39.153 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.434 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.196 | 0.000 |
| 1x2-w32 | accepted | fft | 3.826 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.740 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.029 | 0.000 |
| 1x2-w32 | accepted | other | 4.971 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.332 | 0.000 |
| 1x2-w32 | accepted | solver | 1.320 | 0.000 |
| 1x2-w32 | candidate | G1_arithmetic | 35.316 | -3.837 |
| 1x2-w32 | candidate | G2_arithmetic | 30.465 | 1.031 |
| 1x2-w32 | candidate | epoch | 11.791 | 2.595 |
| 1x2-w32 | candidate | fft | 4.510 | 0.685 |
| 1x2-w32 | candidate | grouping | 0.837 | 0.097 |
| 1x2-w32 | candidate | inverse | 6.808 | -0.221 |
| 1x2-w32 | candidate | other | 4.891 | -0.080 |
| 1x2-w32 | candidate | scheduler | 3.897 | -0.435 |
| 1x2-w32 | candidate | solver | 1.485 | 0.165 |
| 1x2-w32 | fresh | G1_arithmetic | 35.775 | -3.378 |
| 1x2-w32 | fresh | G2_arithmetic | 30.542 | 1.108 |
| 1x2-w32 | fresh | epoch | 9.455 | 0.259 |
| 1x2-w32 | fresh | fft | 5.012 | 1.187 |
| 1x2-w32 | fresh | grouping | 0.547 | -0.193 |
| 1x2-w32 | fresh | inverse | 7.756 | 0.726 |
| 1x2-w32 | fresh | other | 4.659 | -0.312 |
| 1x2-w32 | fresh | scheduler | 4.396 | 0.064 |
| 1x2-w32 | fresh | solver | 1.858 | 0.538 |
| merge-54x1-w32 | accepted | G1_arithmetic | 27.898 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 8.370 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.438 | 0.000 |
| merge-54x1-w32 | accepted | fft | 44.609 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.515 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.925 | 0.000 |
| merge-54x1-w32 | accepted | other | 4.350 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.307 | 0.000 |
| merge-54x1-w32 | accepted | solver | 6.589 | 0.000 |
| merge-54x1-w32 | candidate | G1_arithmetic | 28.292 | 0.394 |
| merge-54x1-w32 | candidate | G2_arithmetic | 8.669 | 0.299 |
| merge-54x1-w32 | candidate | epoch | 2.118 | -0.319 |
| merge-54x1-w32 | candidate | fft | 44.254 | -0.355 |
| merge-54x1-w32 | candidate | grouping | 1.538 | 0.023 |
| merge-54x1-w32 | candidate | inverse | 1.981 | 0.056 |
| merge-54x1-w32 | candidate | other | 4.347 | -0.003 |
| merge-54x1-w32 | candidate | scheduler | 2.034 | -0.272 |
| merge-54x1-w32 | candidate | solver | 6.767 | 0.178 |
| merge-54x1-w32 | fresh | G1_arithmetic | 27.681 | -0.217 |
| merge-54x1-w32 | fresh | G2_arithmetic | 8.588 | 0.218 |
| merge-54x1-w32 | fresh | epoch | 2.471 | 0.034 |
| merge-54x1-w32 | fresh | fft | 44.673 | 0.064 |
| merge-54x1-w32 | fresh | grouping | 1.522 | 0.008 |
| merge-54x1-w32 | fresh | inverse | 1.913 | -0.012 |
| merge-54x1-w32 | fresh | other | 4.300 | -0.049 |
| merge-54x1-w32 | fresh | scheduler | 2.204 | -0.103 |
| merge-54x1-w32 | fresh | solver | 6.647 | 0.058 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.372564 | 37.000 | 4.419× | 24.000 | 2.867× | false |
| 1x2-w32 | fresh | 8.296841 | 37.000 | 4.460× | 24.000 | 2.893× | false |
| 1x2-w32 | candidate | 8.324853 | 37.000 | 4.445× | 24.000 | 2.883× | false |
| 2x2-w32 | accepted | 11.425210 | 59.000 | 5.164× | 30.000 | 2.626× | false |
| 2x2-w32 | fresh | 11.430436 | 59.000 | 5.162× | 30.000 | 2.625× | false |
| 2x2-w32 | candidate | 11.460675 | 59.000 | 5.148× | 30.000 | 2.618× | false |
| 4x2-w32 | accepted | 16.102861 | 109.000 | 6.769× | 48.000 | 2.981× | false |
| 4x2-w32 | fresh | 15.950026 | 109.000 | 6.834× | 48.000 | 3.009× | false |
| 4x2-w32 | candidate | 15.854365 | 109.000 | 6.875× | 48.000 | 3.028× | false |
| 8x2-w32 | accepted | 24.382666 | 189.000 | 7.751× | 74.000 | 3.035× | false |
| 8x2-w32 | fresh | 24.162810 | 189.000 | 7.822× | 74.000 | 3.063× | false |
| 8x2-w32 | candidate | 24.084626 | 189.000 | 7.847× | 74.000 | 3.072× | false |
| 12x2-w32 | accepted | 34.164589 | 284.000 | 8.313× | 101.000 | 2.956× | false |
| 12x2-w32 | fresh | 33.974509 | 284.000 | 8.359× | 101.000 | 2.973× | false |
| 12x2-w32 | candidate | 33.693397 | 284.000 | 8.429× | 101.000 | 2.998× | false |
| 16x2-w32 | accepted | 40.154939 | 335.000 | 8.343× | 124.000 | 3.088× | false |
| 16x2-w32 | fresh | 40.002315 | 335.000 | 8.375× | 124.000 | 3.100× | false |
| 16x2-w32 | candidate | 39.981636 | 335.000 | 8.379× | 124.000 | 3.101× | false |
| 24x2-w32 | accepted | 66.972265 | 528.000 | 7.884× | 176.000 | 2.628× | false |
| 24x2-w32 | fresh | 67.208254 | 528.000 | 7.856× | 176.000 | 2.619× | false |
| 24x2-w32 | candidate | 66.010670 | 528.000 | 7.999× | 176.000 | 2.666× | false |
| merge-24x1-w32 | accepted | 66.125565 | 507.000 | 7.667× | 175.000 | 2.646× | false |
| merge-24x1-w32 | fresh | 66.112380 | 507.000 | 7.669× | 175.000 | 2.647× | false |
| merge-24x1-w32 | candidate | 65.086624 | 507.000 | 7.790× | 175.000 | 2.689× | false |
| 1x2-w16 | accepted | 10.168078 | 37.000 | 3.639× | 24.000 | 2.360× | false |
| 1x2-w16 | fresh | 10.078075 | 37.000 | 3.671× | 24.000 | 2.381× | false |
| 1x2-w16 | candidate | 10.036855 | 37.000 | 3.686× | 24.000 | 2.391× | false |

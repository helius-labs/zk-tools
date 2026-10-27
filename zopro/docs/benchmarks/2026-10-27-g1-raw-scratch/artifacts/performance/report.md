# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 33.649030 | 1.000000 | — | — | — |
| 12x2-w32 | candidate | 33.939158 | 1.009315 | — | — | — |
| 12x2-w32 | fresh | 33.917661 | 0.997851 | — | — | — |
| 16x2-w32 | accepted | 39.961403 | 1.000000 | — | — | — |
| 16x2-w32 | candidate | 40.539226 | 1.011542 | — | — | — |
| 16x2-w32 | fresh | 40.056113 | 1.001804 | — | — | — |
| 1x2-w16 | accepted | 10.097085 | 1.000000 | — | — | — |
| 1x2-w16 | candidate | 10.223082 | 1.012207 | — | — | — |
| 1x2-w16 | fresh | 10.070642 | 1.001818 | — | — | — |
| 1x2-w32 | accepted | 8.310633 | 1.000000 | 190.399 | 8.597807 | 136.75 |
| 1x2-w32 | candidate | 8.404229 | 1.010053 | 192.894 | 8.563659 | 132.75 |
| 1x2-w32 | fresh | 8.317877 | 0.999323 | 192.013 | 8.456636 | 130.50 |
| 24x2-w32 | accepted | 66.222937 | 1.000000 | — | — | — |
| 24x2-w32 | candidate | 66.523544 | 1.005774 | — | — | — |
| 24x2-w32 | fresh | 66.312657 | 1.001042 | — | — | — |
| 2x2-w32 | accepted | 11.444606 | 1.000000 | — | — | — |
| 2x2-w32 | candidate | 11.479884 | 1.004995 | — | — | — |
| 2x2-w32 | fresh | 11.429629 | 0.999436 | — | — | — |
| 4x2-w32 | accepted | 16.023665 | 1.000000 | — | — | — |
| 4x2-w32 | candidate | 16.045488 | 0.995622 | — | — | — |
| 4x2-w32 | fresh | 16.030242 | 0.991386 | — | — | — |
| 8x2-w32 | accepted | 24.256758 | 1.000000 | — | — | — |
| 8x2-w32 | candidate | 24.297092 | 1.008460 | — | — | — |
| 8x2-w32 | fresh | 24.227915 | 1.000562 | — | — | — |
| merge-24x1-w32 | accepted | 65.028733 | 1.000000 | — | — | — |
| merge-24x1-w32 | candidate | 65.382236 | 1.004516 | — | — | — |
| merge-24x1-w32 | fresh | 65.327417 | 1.004593 | — | — | — |
| merge-54x1-w32 | accepted | 162.039299 | 1.000000 | 4415.025 | 161.727320 | 2185.18 |
| merge-54x1-w32 | candidate | 161.446096 | 0.995044 | 4467.174 | 163.866144 | 2107.38 |
| merge-54x1-w32 | fresh | 161.523860 | 0.997539 | 4416.944 | 161.982947 | 2106.15 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.945 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.267 | 0.000 |
| 1x2-w32 | accepted | epoch | 10.231 | 0.000 |
| 1x2-w32 | accepted | fft | 4.512 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.780 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.052 | 0.000 |
| 1x2-w32 | accepted | other | 4.131 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.518 | 0.000 |
| 1x2-w32 | accepted | solver | 1.562 | 0.000 |
| 1x2-w32 | candidate | G1_arithmetic | 38.073 | -0.872 |
| 1x2-w32 | candidate | G2_arithmetic | 29.347 | 0.080 |
| 1x2-w32 | candidate | epoch | 8.817 | -1.414 |
| 1x2-w32 | candidate | fft | 5.396 | 0.884 |
| 1x2-w32 | candidate | grouping | 0.795 | 0.014 |
| 1x2-w32 | candidate | inverse | 7.054 | 0.002 |
| 1x2-w32 | candidate | other | 4.621 | 0.491 |
| 1x2-w32 | candidate | scheduler | 4.137 | 0.619 |
| 1x2-w32 | candidate | solver | 1.759 | 0.197 |
| 1x2-w32 | fresh | G1_arithmetic | 37.415 | -1.530 |
| 1x2-w32 | fresh | G2_arithmetic | 29.760 | 0.494 |
| 1x2-w32 | fresh | epoch | 9.655 | -0.577 |
| 1x2-w32 | fresh | fft | 4.738 | 0.226 |
| 1x2-w32 | fresh | grouping | 0.690 | -0.090 |
| 1x2-w32 | fresh | inverse | 7.495 | 0.443 |
| 1x2-w32 | fresh | other | 4.994 | 0.863 |
| 1x2-w32 | fresh | scheduler | 3.893 | 0.375 |
| 1x2-w32 | fresh | solver | 1.360 | -0.203 |
| merge-54x1-w32 | accepted | G1_arithmetic | 28.441 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 8.759 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.208 | 0.000 |
| merge-54x1-w32 | accepted | fft | 43.985 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.549 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.970 | 0.000 |
| merge-54x1-w32 | accepted | other | 4.293 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.057 | 0.000 |
| merge-54x1-w32 | accepted | solver | 6.738 | 0.000 |
| merge-54x1-w32 | candidate | G1_arithmetic | 28.946 | 0.505 |
| merge-54x1-w32 | candidate | G2_arithmetic | 9.115 | 0.356 |
| merge-54x1-w32 | candidate | epoch | 2.035 | -0.174 |
| merge-54x1-w32 | candidate | fft | 43.082 | -0.903 |
| merge-54x1-w32 | candidate | grouping | 1.608 | 0.059 |
| merge-54x1-w32 | candidate | inverse | 2.000 | 0.030 |
| merge-54x1-w32 | candidate | other | 4.463 | 0.169 |
| merge-54x1-w32 | candidate | scheduler | 2.030 | -0.027 |
| merge-54x1-w32 | candidate | solver | 6.722 | -0.016 |
| merge-54x1-w32 | fresh | G1_arithmetic | 28.183 | -0.258 |
| merge-54x1-w32 | fresh | G2_arithmetic | 8.831 | 0.072 |
| merge-54x1-w32 | fresh | epoch | 2.125 | -0.083 |
| merge-54x1-w32 | fresh | fft | 44.344 | 0.359 |
| merge-54x1-w32 | fresh | grouping | 1.533 | -0.016 |
| merge-54x1-w32 | fresh | inverse | 2.090 | 0.120 |
| merge-54x1-w32 | fresh | other | 4.376 | 0.083 |
| merge-54x1-w32 | fresh | scheduler | 2.029 | -0.028 |
| merge-54x1-w32 | fresh | solver | 6.489 | -0.249 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.310633 | 37.000 | 4.452× | 24.000 | 2.888× | false |
| 1x2-w32 | fresh | 8.317877 | 37.000 | 4.448× | 24.000 | 2.885× | false |
| 1x2-w32 | candidate | 8.404229 | 37.000 | 4.403× | 24.000 | 2.856× | false |
| 2x2-w32 | accepted | 11.444606 | 59.000 | 5.155× | 30.000 | 2.621× | false |
| 2x2-w32 | fresh | 11.429629 | 59.000 | 5.162× | 30.000 | 2.625× | false |
| 2x2-w32 | candidate | 11.479884 | 59.000 | 5.139× | 30.000 | 2.613× | false |
| 4x2-w32 | accepted | 16.023665 | 109.000 | 6.802× | 48.000 | 2.996× | false |
| 4x2-w32 | fresh | 16.030242 | 109.000 | 6.800× | 48.000 | 2.994× | false |
| 4x2-w32 | candidate | 16.045488 | 109.000 | 6.793× | 48.000 | 2.991× | false |
| 8x2-w32 | accepted | 24.256758 | 189.000 | 7.792× | 74.000 | 3.051× | false |
| 8x2-w32 | fresh | 24.227915 | 189.000 | 7.801× | 74.000 | 3.054× | false |
| 8x2-w32 | candidate | 24.297092 | 189.000 | 7.779× | 74.000 | 3.046× | false |
| 12x2-w32 | accepted | 33.649030 | 284.000 | 8.440× | 101.000 | 3.002× | false |
| 12x2-w32 | fresh | 33.917661 | 284.000 | 8.373× | 101.000 | 2.978× | false |
| 12x2-w32 | candidate | 33.939158 | 284.000 | 8.368× | 101.000 | 2.976× | false |
| 16x2-w32 | accepted | 39.961403 | 335.000 | 8.383× | 124.000 | 3.103× | false |
| 16x2-w32 | fresh | 40.056113 | 335.000 | 8.363× | 124.000 | 3.096× | false |
| 16x2-w32 | candidate | 40.539226 | 335.000 | 8.264× | 124.000 | 3.059× | false |
| 24x2-w32 | accepted | 66.222937 | 528.000 | 7.973× | 176.000 | 2.658× | false |
| 24x2-w32 | fresh | 66.312657 | 528.000 | 7.962× | 176.000 | 2.654× | false |
| 24x2-w32 | candidate | 66.523544 | 528.000 | 7.937× | 176.000 | 2.646× | false |
| merge-24x1-w32 | accepted | 65.028733 | 507.000 | 7.797× | 175.000 | 2.691× | false |
| merge-24x1-w32 | fresh | 65.327417 | 507.000 | 7.761× | 175.000 | 2.679× | false |
| merge-24x1-w32 | candidate | 65.382236 | 507.000 | 7.754× | 175.000 | 2.677× | false |
| 1x2-w16 | accepted | 10.097085 | 37.000 | 3.664× | 24.000 | 2.377× | false |
| 1x2-w16 | fresh | 10.070642 | 37.000 | 3.674× | 24.000 | 2.383× | false |
| 1x2-w16 | candidate | 10.223082 | 37.000 | 3.619× | 24.000 | 2.348× | false |

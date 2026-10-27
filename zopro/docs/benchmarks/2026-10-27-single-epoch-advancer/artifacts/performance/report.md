# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 33.924576 | 1.000000 | — | — | — |
| 12x2-w32 | off | 33.563598 | 0.993633 | — | — | — |
| 12x2-w32 | on | 33.868743 | 1.001514 | — | — | — |
| 16x2-w32 | accepted | 39.901053 | 1.000000 | — | — | — |
| 16x2-w32 | off | 39.808306 | 0.990882 | — | — | — |
| 16x2-w32 | on | 39.723730 | 0.993294 | — | — | — |
| 1x2-w16 | accepted | 9.970658 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.022468 | 1.001281 | — | — | — |
| 1x2-w16 | on | 10.270333 | 1.029167 | — | — | — |
| 1x2-w32 | accepted | 8.302345 | 1.000000 | 190.869 | 8.586194 | 138.50 |
| 1x2-w32 | off | 8.335117 | 1.003458 | 191.942 | 8.452058 | 132.75 |
| 1x2-w32 | on | 8.327132 | 1.000548 | 188.984 | 8.449549 | 137.00 |
| 24x2-w32 | accepted | 66.315940 | 1.000000 | — | — | — |
| 24x2-w32 | off | 66.540656 | 1.001763 | — | — | — |
| 24x2-w32 | on | 66.531005 | 1.003243 | — | — | — |
| 2x2-w32 | accepted | 11.412078 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.470458 | 1.005116 | — | — | — |
| 2x2-w32 | on | 11.392968 | 0.997128 | — | — | — |
| 4x2-w32 | accepted | 15.914125 | 1.000000 | — | — | — |
| 4x2-w32 | off | 15.934471 | 1.002911 | — | — | — |
| 4x2-w32 | on | 15.960166 | 0.998684 | — | — | — |
| 8x2-w32 | accepted | 24.009335 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.029863 | 1.000855 | — | — | — |
| 8x2-w32 | on | 23.918371 | 0.997680 | — | — | — |
| merge-24x1-w32 | accepted | 65.051242 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 65.444828 | 1.006187 | — | — | — |
| merge-24x1-w32 | on | 65.177690 | 1.003784 | — | — | — |
| merge-54x1-w32 | accepted | 162.670614 | 1.000000 | 4449.932 | 161.524035 | 1662.69 |
| merge-54x1-w32 | off | 160.943732 | 0.987221 | 4447.737 | 162.669781 | 1727.89 |
| merge-54x1-w32 | on | 162.066337 | 0.999593 | 4474.648 | 163.082511 | 1688.24 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 36.470 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.750 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.882 | 0.000 |
| 1x2-w32 | accepted | fft | 4.330 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.727 | 0.000 |
| 1x2-w32 | accepted | inverse | 8.350 | 0.000 |
| 1x2-w32 | accepted | other | 4.814 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.180 | 0.000 |
| 1x2-w32 | accepted | solver | 1.498 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.134 | 0.665 |
| 1x2-w32 | off | G2_arithmetic | 29.368 | -0.382 |
| 1x2-w32 | off | epoch | 10.334 | 0.452 |
| 1x2-w32 | off | fft | 3.794 | -0.536 |
| 1x2-w32 | off | grouping | 0.735 | 0.008 |
| 1x2-w32 | off | inverse | 7.985 | -0.366 |
| 1x2-w32 | off | other | 5.536 | 0.722 |
| 1x2-w32 | off | scheduler | 3.915 | -0.265 |
| 1x2-w32 | off | solver | 1.199 | -0.298 |
| 1x2-w32 | on | G1_arithmetic | 37.630 | 1.160 |
| 1x2-w32 | on | G2_arithmetic | 29.742 | -0.007 |
| 1x2-w32 | on | epoch | 8.590 | -1.293 |
| 1x2-w32 | on | fft | 4.297 | -0.034 |
| 1x2-w32 | on | grouping | 1.221 | 0.494 |
| 1x2-w32 | on | inverse | 7.681 | -0.669 |
| 1x2-w32 | on | other | 5.024 | 0.210 |
| 1x2-w32 | on | scheduler | 4.199 | 0.020 |
| 1x2-w32 | on | solver | 1.616 | 0.118 |
| merge-54x1-w32 | accepted | G1_arithmetic | 30.108 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 9.740 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.207 | 0.000 |
| merge-54x1-w32 | accepted | fft | 44.167 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.528 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 2.513 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.330 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 1.957 | 0.000 |
| merge-54x1-w32 | accepted | solver | 4.451 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 29.861 | -0.247 |
| merge-54x1-w32 | off | G2_arithmetic | 9.238 | -0.502 |
| merge-54x1-w32 | off | epoch | 1.838 | -0.368 |
| merge-54x1-w32 | off | fft | 45.081 | 0.914 |
| merge-54x1-w32 | off | grouping | 1.953 | 0.426 |
| merge-54x1-w32 | off | inverse | 2.094 | -0.419 |
| merge-54x1-w32 | off | other | 3.740 | 0.410 |
| merge-54x1-w32 | off | scheduler | 1.464 | -0.492 |
| merge-54x1-w32 | off | solver | 4.730 | 0.279 |
| merge-54x1-w32 | on | G1_arithmetic | 31.306 | 1.199 |
| merge-54x1-w32 | on | G2_arithmetic | 10.163 | 0.423 |
| merge-54x1-w32 | on | epoch | 1.431 | -0.776 |
| merge-54x1-w32 | on | fft | 43.332 | -0.835 |
| merge-54x1-w32 | on | grouping | 1.722 | 0.195 |
| merge-54x1-w32 | on | inverse | 2.074 | -0.439 |
| merge-54x1-w32 | on | other | 3.607 | 0.277 |
| merge-54x1-w32 | on | scheduler | 1.981 | 0.024 |
| merge-54x1-w32 | on | solver | 4.384 | -0.067 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.302345 | 37.000 | 4.457× | 24.000 | 2.891× | false |
| 1x2-w32 | off | 8.335117 | 37.000 | 4.439× | 24.000 | 2.879× | false |
| 1x2-w32 | on | 8.327132 | 37.000 | 4.443× | 24.000 | 2.882× | false |
| 2x2-w32 | accepted | 11.412078 | 59.000 | 5.170× | 30.000 | 2.629× | false |
| 2x2-w32 | off | 11.470458 | 59.000 | 5.144× | 30.000 | 2.615× | false |
| 2x2-w32 | on | 11.392968 | 59.000 | 5.179× | 30.000 | 2.633× | false |
| 4x2-w32 | accepted | 15.914125 | 109.000 | 6.849× | 48.000 | 3.016× | false |
| 4x2-w32 | off | 15.934471 | 109.000 | 6.841× | 48.000 | 3.012× | false |
| 4x2-w32 | on | 15.960166 | 109.000 | 6.830× | 48.000 | 3.007× | false |
| 8x2-w32 | accepted | 24.009335 | 189.000 | 7.872× | 74.000 | 3.082× | false |
| 8x2-w32 | off | 24.029863 | 189.000 | 7.865× | 74.000 | 3.080× | false |
| 8x2-w32 | on | 23.918371 | 189.000 | 7.902× | 74.000 | 3.094× | false |
| 12x2-w32 | accepted | 33.924576 | 284.000 | 8.372× | 101.000 | 2.977× | false |
| 12x2-w32 | off | 33.563598 | 284.000 | 8.462× | 101.000 | 3.009× | false |
| 12x2-w32 | on | 33.868743 | 284.000 | 8.385× | 101.000 | 2.982× | false |
| 16x2-w32 | accepted | 39.901053 | 335.000 | 8.396× | 124.000 | 3.108× | false |
| 16x2-w32 | off | 39.808306 | 335.000 | 8.415× | 124.000 | 3.115× | false |
| 16x2-w32 | on | 39.723730 | 335.000 | 8.433× | 124.000 | 3.122× | false |
| 24x2-w32 | accepted | 66.315940 | 528.000 | 7.962× | 176.000 | 2.654× | false |
| 24x2-w32 | off | 66.540656 | 528.000 | 7.935× | 176.000 | 2.645× | false |
| 24x2-w32 | on | 66.531005 | 528.000 | 7.936× | 176.000 | 2.645× | false |
| merge-24x1-w32 | accepted | 65.051242 | 507.000 | 7.794× | 175.000 | 2.690× | false |
| merge-24x1-w32 | off | 65.444828 | 507.000 | 7.747× | 175.000 | 2.674× | false |
| merge-24x1-w32 | on | 65.177690 | 507.000 | 7.779× | 175.000 | 2.685× | false |
| 1x2-w16 | accepted | 9.970658 | 37.000 | 3.711× | 24.000 | 2.407× | false |
| 1x2-w16 | off | 10.022468 | 37.000 | 3.692× | 24.000 | 2.395× | false |
| 1x2-w16 | on | 10.270333 | 37.000 | 3.603× | 24.000 | 2.337× | false |

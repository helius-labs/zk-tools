# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.732339 | 1.000000 | — | — | — |
| 12x2-w32 | off | 34.372129 | 0.987903 | — | — | — |
| 12x2-w32 | on | 34.574571 | 0.993722 | — | — | — |
| 16x2-w32 | accepted | 40.845490 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.754459 | 0.999217 | — | — | — |
| 16x2-w32 | on | 40.693394 | 0.995158 | — | — | — |
| 1x2-w32 | accepted | 8.556801 | 1.000000 | 201.670 | 8.796327 | 118.25 |
| 1x2-w32 | off | 8.649927 | 1.010883 | 202.893 | 8.800245 | 118.00 |
| 1x2-w32 | on | 8.790092 | 1.027264 | 180.658 | 8.911239 | 72.75 |
| 24x2-w32 | accepted | 68.758215 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.864465 | 0.991845 | — | — | — |
| 24x2-w32 | on | 68.645743 | 0.993714 | — | — | — |
| 2x2-w32 | accepted | 11.722253 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.711222 | 1.000932 | — | — | — |
| 2x2-w32 | on | 11.616533 | 0.990981 | — | — | — |
| 4x2-w32 | accepted | 16.100783 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.121251 | 0.998757 | — | — | — |
| 4x2-w32 | on | 15.976338 | 0.989780 | — | — | — |
| 8x2-w32 | accepted | 24.711576 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.467936 | 1.004428 | — | — | — |
| 8x2-w32 | on | 24.597189 | 1.011599 | — | — | — |
| merge-24x1-w32 | accepted | 67.509419 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.040402 | 1.007064 | — | — | — |
| merge-24x1-w32 | on | 68.974954 | 1.021709 | — | — | — |
| merge-54x1-w32 | accepted | 177.270822 | 1.000000 | — | — | — |
| merge-54x1-w32 | off | 178.099866 | 1.004677 | — | — | — |
| merge-54x1-w32 | on | 175.559284 | 0.986593 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 35.703 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.026 | 0.000 |
| 1x2-w32 | accepted | epoch | 8.663 | 0.000 |
| 1x2-w32 | accepted | fft | 5.315 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.170 | 0.000 |
| 1x2-w32 | accepted | inverse | 10.005 | 0.000 |
| 1x2-w32 | accepted | other | 5.242 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.439 | 0.000 |
| 1x2-w32 | accepted | solver | 2.437 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 35.611 | -0.091 |
| 1x2-w32 | off | G2_arithmetic | 36.028 | 8.002 |
| 1x2-w32 | off | epoch | 8.473 | -0.190 |
| 1x2-w32 | off | fft | 4.215 | -1.099 |
| 1x2-w32 | off | grouping | 0.110 | -0.060 |
| 1x2-w32 | off | inverse | 5.058 | -4.948 |
| 1x2-w32 | off | other | 3.680 | -1.562 |
| 1x2-w32 | off | scheduler | 3.733 | -0.706 |
| 1x2-w32 | off | solver | 3.091 | 0.654 |
| 1x2-w32 | on | G1_arithmetic | 51.203 | 15.500 |
| 1x2-w32 | on | G2_arithmetic | 32.835 | 4.809 |
| 1x2-w32 | on | epoch | 5.277 | -3.386 |
| 1x2-w32 | on | fft | 1.778 | -3.537 |
| 1x2-w32 | on | grouping | 0.350 | 0.180 |
| 1x2-w32 | on | inverse | 5.008 | -4.997 |
| 1x2-w32 | on | other | 2.002 | -3.240 |
| 1x2-w32 | on | scheduler | 0.333 | -4.106 |
| 1x2-w32 | on | solver | 1.215 | -1.222 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.556801 | 37.000 | 4.324× | 24.000 | 2.805× | false |
| 1x2-w32 | off | 8.649927 | 37.000 | 4.277× | 24.000 | 2.775× | false |
| 1x2-w32 | on | 8.790092 | 37.000 | 4.209× | 24.000 | 2.730× | false |
| 2x2-w32 | accepted | 11.722253 | 59.000 | 5.033× | 30.000 | 2.559× | false |
| 2x2-w32 | off | 11.711222 | 59.000 | 5.038× | 30.000 | 2.562× | false |
| 2x2-w32 | on | 11.616533 | 59.000 | 5.079× | 30.000 | 2.583× | false |
| 4x2-w32 | accepted | 16.100783 | 109.000 | 6.770× | 48.000 | 2.981× | false |
| 4x2-w32 | off | 16.121251 | 109.000 | 6.761× | 48.000 | 2.977× | false |
| 4x2-w32 | on | 15.976338 | 109.000 | 6.823× | 48.000 | 3.004× | false |
| 8x2-w32 | accepted | 24.711576 | 189.000 | 7.648× | 74.000 | 2.995× | false |
| 8x2-w32 | off | 24.467936 | 189.000 | 7.724× | 74.000 | 3.024× | false |
| 8x2-w32 | on | 24.597189 | 189.000 | 7.684× | 74.000 | 3.008× | false |
| 12x2-w32 | accepted | 34.732339 | 284.000 | 8.177× | 101.000 | 2.908× | false |
| 12x2-w32 | off | 34.372129 | 284.000 | 8.263× | 101.000 | 2.938× | false |
| 12x2-w32 | on | 34.574571 | 284.000 | 8.214× | 101.000 | 2.921× | false |
| 16x2-w32 | accepted | 40.845490 | 335.000 | 8.202× | 124.000 | 3.036× | false |
| 16x2-w32 | off | 40.754459 | 335.000 | 8.220× | 124.000 | 3.043× | false |
| 16x2-w32 | on | 40.693394 | 335.000 | 8.232× | 124.000 | 3.047× | false |
| 24x2-w32 | accepted | 68.758215 | 528.000 | 7.679× | 176.000 | 2.560× | false |
| 24x2-w32 | off | 68.864465 | 528.000 | 7.667× | 176.000 | 2.556× | false |
| 24x2-w32 | on | 68.645743 | 528.000 | 7.692× | 176.000 | 2.564× | false |
| merge-24x1-w32 | accepted | 67.509419 | 507.000 | 7.510× | 175.000 | 2.592× | false |
| merge-24x1-w32 | off | 68.040402 | 507.000 | 7.451× | 175.000 | 2.572× | false |
| merge-24x1-w32 | on | 68.974954 | 507.000 | 7.350× | 175.000 | 2.537× | false |

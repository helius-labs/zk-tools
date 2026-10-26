# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.464068 | 1.000000 | — | — | — |
| 12x2-w32 | off | 35.197182 | 1.002596 | — | — | — |
| 12x2-w32 | on | 34.475460 | 0.998304 | — | — | — |
| 16x2-w32 | accepted | 41.489780 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.418188 | 0.974172 | — | — | — |
| 16x2-w32 | on | 40.574112 | 0.977930 | — | — | — |
| 1x2-w32 | accepted | 8.630346 | 1.000000 | 200.515 | 8.798285 | 116.75 |
| 1x2-w32 | off | 8.618765 | 0.998658 | 200.735 | 8.795806 | 119.00 |
| 1x2-w32 | on | 8.648521 | 0.996119 | 200.085 | 8.800827 | 79.75 |
| 24x2-w32 | accepted | 68.818480 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.327324 | 0.992225 | — | — | — |
| 24x2-w32 | on | 68.699957 | 0.997697 | — | — | — |
| 2x2-w32 | accepted | 11.644130 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.741084 | 1.003189 | — | — | — |
| 2x2-w32 | on | 11.728430 | 1.010680 | — | — | — |
| 4x2-w32 | accepted | 16.049760 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.249636 | 1.004653 | — | — | — |
| 4x2-w32 | on | 16.411722 | 1.014674 | — | — | — |
| 8x2-w32 | accepted | 24.957800 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.624952 | 0.983774 | — | — | — |
| 8x2-w32 | on | 24.495017 | 1.010237 | — | — | — |
| merge-24x1-w32 | accepted | 67.536854 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.619765 | 1.014999 | — | — | — |
| merge-24x1-w32 | on | 67.305908 | 0.995490 | — | — | — |
| merge-54x1-w32 | accepted | 176.621746 | 1.000000 | — | — | — |
| merge-54x1-w32 | off | 176.714420 | 0.999083 | — | — | — |
| merge-54x1-w32 | on | 175.906976 | 0.991899 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 35.037 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 30.448 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.892 | 0.000 |
| 1x2-w32 | accepted | fft | 3.217 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.885 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.986 | 0.000 |
| 1x2-w32 | accepted | other | 5.083 | 0.000 |
| 1x2-w32 | accepted | scheduler | 5.733 | 0.000 |
| 1x2-w32 | accepted | solver | 2.720 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 44.546 | 9.509 |
| 1x2-w32 | off | G2_arithmetic | 25.415 | -5.033 |
| 1x2-w32 | off | epoch | 8.667 | -1.225 |
| 1x2-w32 | off | fft | 4.758 | 1.541 |
| 1x2-w32 | off | grouping | 0.360 | -0.525 |
| 1x2-w32 | off | inverse | 5.846 | -1.139 |
| 1x2-w32 | off | other | 4.385 | -0.698 |
| 1x2-w32 | off | scheduler | 4.190 | -1.543 |
| 1x2-w32 | off | solver | 1.833 | -0.887 |
| 1x2-w32 | on | G1_arithmetic | 37.733 | 2.696 |
| 1x2-w32 | on | G2_arithmetic | 29.747 | -0.701 |
| 1x2-w32 | on | epoch | 11.611 | 1.719 |
| 1x2-w32 | on | fft | 4.622 | 1.404 |
| 1x2-w32 | on | grouping | 0.194 | -0.691 |
| 1x2-w32 | on | inverse | 5.097 | -1.889 |
| 1x2-w32 | on | other | 4.518 | -0.565 |
| 1x2-w32 | on | scheduler | 4.847 | -0.885 |
| 1x2-w32 | on | solver | 1.632 | -1.088 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.630346 | 37.000 | 4.287× | 24.000 | 2.781× | false |
| 1x2-w32 | off | 8.618765 | 37.000 | 4.293× | 24.000 | 2.785× | false |
| 1x2-w32 | on | 8.648521 | 37.000 | 4.278× | 24.000 | 2.775× | false |
| 2x2-w32 | accepted | 11.644130 | 59.000 | 5.067× | 30.000 | 2.576× | false |
| 2x2-w32 | off | 11.741084 | 59.000 | 5.025× | 30.000 | 2.555× | false |
| 2x2-w32 | on | 11.728430 | 59.000 | 5.031× | 30.000 | 2.558× | false |
| 4x2-w32 | accepted | 16.049760 | 109.000 | 6.791× | 48.000 | 2.991× | false |
| 4x2-w32 | off | 16.249636 | 109.000 | 6.708× | 48.000 | 2.954× | false |
| 4x2-w32 | on | 16.411722 | 109.000 | 6.642× | 48.000 | 2.925× | false |
| 8x2-w32 | accepted | 24.957800 | 189.000 | 7.573× | 74.000 | 2.965× | false |
| 8x2-w32 | off | 24.624952 | 189.000 | 7.675× | 74.000 | 3.005× | false |
| 8x2-w32 | on | 24.495017 | 189.000 | 7.716× | 74.000 | 3.021× | false |
| 12x2-w32 | accepted | 34.464068 | 284.000 | 8.240× | 101.000 | 2.931× | false |
| 12x2-w32 | off | 35.197182 | 284.000 | 8.069× | 101.000 | 2.870× | false |
| 12x2-w32 | on | 34.475460 | 284.000 | 8.238× | 101.000 | 2.930× | false |
| 16x2-w32 | accepted | 41.489780 | 335.000 | 8.074× | 124.000 | 2.989× | false |
| 16x2-w32 | off | 40.418188 | 335.000 | 8.288× | 124.000 | 3.068× | false |
| 16x2-w32 | on | 40.574112 | 335.000 | 8.256× | 124.000 | 3.056× | false |
| 24x2-w32 | accepted | 68.818480 | 528.000 | 7.672× | 176.000 | 2.557× | false |
| 24x2-w32 | off | 68.327324 | 528.000 | 7.728× | 176.000 | 2.576× | false |
| 24x2-w32 | on | 68.699957 | 528.000 | 7.686× | 176.000 | 2.562× | false |
| merge-24x1-w32 | accepted | 67.536854 | 507.000 | 7.507× | 175.000 | 2.591× | false |
| merge-24x1-w32 | off | 68.619765 | 507.000 | 7.389× | 175.000 | 2.550× | false |
| merge-24x1-w32 | on | 67.305908 | 507.000 | 7.533× | 175.000 | 2.600× | false |

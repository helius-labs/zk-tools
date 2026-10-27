# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.421324 | 1.000000 | — | — | — |
| 12x2-w32 | fresh | 34.660325 | 1.002424 | — | — | — |
| 12x2-w32 | tree | 34.100319 | 0.993871 | — | — | — |
| 16x2-w32 | accepted | 40.799074 | 1.000000 | — | — | — |
| 16x2-w32 | fresh | 40.635426 | 0.996639 | — | — | — |
| 16x2-w32 | tree | 40.410350 | 0.994456 | — | — | — |
| 1x2-w16 | accepted | 10.715802 | 1.000000 | — | — | — |
| 1x2-w16 | fresh | 10.491810 | 0.976191 | — | — | — |
| 1x2-w16 | tree | 10.499122 | 0.965057 | — | — | — |
| 1x2-w32 | accepted | 8.633414 | 1.000000 | 199.079 | 8.966370 | 132.75 |
| 1x2-w32 | fresh | 8.678249 | 1.005493 | 200.074 | 8.853052 | 135.25 |
| 1x2-w32 | tree | 8.624067 | 0.998253 | 199.804 | 8.753469 | 135.50 |
| 24x2-w32 | accepted | 68.862085 | 1.000000 | — | — | — |
| 24x2-w32 | fresh | 68.939792 | 0.992393 | — | — | — |
| 24x2-w32 | tree | 67.604363 | 0.981112 | — | — | — |
| 2x2-w32 | accepted | 11.738060 | 1.000000 | — | — | — |
| 2x2-w32 | fresh | 11.748330 | 1.002819 | — | — | — |
| 2x2-w32 | tree | 11.743912 | 1.004273 | — | — | — |
| 4x2-w32 | accepted | 16.207468 | 1.000000 | — | — | — |
| 4x2-w32 | fresh | 16.216991 | 1.000210 | — | — | — |
| 4x2-w32 | tree | 16.130545 | 0.995069 | — | — | — |
| 8x2-w32 | accepted | 24.398451 | 1.000000 | — | — | — |
| 8x2-w32 | fresh | 24.918404 | 1.011269 | — | — | — |
| 8x2-w32 | tree | 24.843198 | 1.006760 | — | — | — |
| merge-24x1-w32 | accepted | 67.480569 | 1.000000 | — | — | — |
| merge-24x1-w32 | fresh | 67.664167 | 1.000472 | — | — | — |
| merge-24x1-w32 | tree | 66.903513 | 0.991449 | — | — | — |
| merge-54x1-w32 | accepted | 176.931927 | 1.000000 | — | — | — |
| merge-54x1-w32 | fresh | 178.821837 | 1.010970 | — | — | — |
| merge-54x1-w32 | tree | 163.651863 | 0.924943 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 37.566 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 27.608 | 0.000 |
| 1x2-w32 | accepted | epoch | 10.337 | 0.000 |
| 1x2-w32 | accepted | fft | 4.636 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.820 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.672 | 0.000 |
| 1x2-w32 | accepted | other | 5.323 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.924 | 0.000 |
| 1x2-w32 | accepted | solver | 2.114 | 0.000 |
| 1x2-w32 | fresh | G1_arithmetic | 40.150 | 2.584 |
| 1x2-w32 | fresh | G2_arithmetic | 27.809 | 0.201 |
| 1x2-w32 | fresh | epoch | 9.754 | -0.583 |
| 1x2-w32 | fresh | fft | 4.748 | 0.112 |
| 1x2-w32 | fresh | grouping | 0.797 | -0.023 |
| 1x2-w32 | fresh | inverse | 7.437 | -0.235 |
| 1x2-w32 | fresh | other | 3.629 | -1.694 |
| 1x2-w32 | fresh | scheduler | 4.048 | 0.125 |
| 1x2-w32 | fresh | solver | 1.629 | -0.485 |
| 1x2-w32 | tree | G1_arithmetic | 41.204 | 3.638 |
| 1x2-w32 | tree | G2_arithmetic | 26.936 | -0.672 |
| 1x2-w32 | tree | epoch | 9.387 | -0.950 |
| 1x2-w32 | tree | fft | 4.402 | -0.234 |
| 1x2-w32 | tree | grouping | 0.818 | -0.002 |
| 1x2-w32 | tree | inverse | 6.900 | -0.772 |
| 1x2-w32 | tree | other | 4.762 | -0.560 |
| 1x2-w32 | tree | scheduler | 4.326 | 0.402 |
| 1x2-w32 | tree | solver | 1.264 | -0.849 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.633414 | 37.000 | 4.286× | 24.000 | 2.780× | false |
| 1x2-w32 | fresh | 8.678249 | 37.000 | 4.264× | 24.000 | 2.766× | false |
| 1x2-w32 | tree | 8.624067 | 37.000 | 4.290× | 24.000 | 2.783× | false |
| 2x2-w32 | accepted | 11.738060 | 59.000 | 5.026× | 30.000 | 2.556× | false |
| 2x2-w32 | fresh | 11.748330 | 59.000 | 5.022× | 30.000 | 2.554× | false |
| 2x2-w32 | tree | 11.743912 | 59.000 | 5.024× | 30.000 | 2.555× | false |
| 4x2-w32 | accepted | 16.207468 | 109.000 | 6.725× | 48.000 | 2.962× | false |
| 4x2-w32 | fresh | 16.216991 | 109.000 | 6.721× | 48.000 | 2.960× | false |
| 4x2-w32 | tree | 16.130545 | 109.000 | 6.757× | 48.000 | 2.976× | false |
| 8x2-w32 | accepted | 24.398451 | 189.000 | 7.746× | 74.000 | 3.033× | false |
| 8x2-w32 | fresh | 24.918404 | 189.000 | 7.585× | 74.000 | 2.970× | false |
| 8x2-w32 | tree | 24.843198 | 189.000 | 7.608× | 74.000 | 2.979× | false |
| 12x2-w32 | accepted | 34.421324 | 284.000 | 8.251× | 101.000 | 2.934× | false |
| 12x2-w32 | fresh | 34.660325 | 284.000 | 8.194× | 101.000 | 2.914× | false |
| 12x2-w32 | tree | 34.100319 | 284.000 | 8.328× | 101.000 | 2.962× | false |
| 16x2-w32 | accepted | 40.799074 | 335.000 | 8.211× | 124.000 | 3.039× | false |
| 16x2-w32 | fresh | 40.635426 | 335.000 | 8.244× | 124.000 | 3.052× | false |
| 16x2-w32 | tree | 40.410350 | 335.000 | 8.290× | 124.000 | 3.069× | false |
| 24x2-w32 | accepted | 68.862085 | 528.000 | 7.667× | 176.000 | 2.556× | false |
| 24x2-w32 | fresh | 68.939792 | 528.000 | 7.659× | 176.000 | 2.553× | false |
| 24x2-w32 | tree | 67.604363 | 528.000 | 7.810× | 176.000 | 2.603× | false |
| merge-24x1-w32 | accepted | 67.480569 | 507.000 | 7.513× | 175.000 | 2.593× | false |
| merge-24x1-w32 | fresh | 67.664167 | 507.000 | 7.493× | 175.000 | 2.586× | false |
| merge-24x1-w32 | tree | 66.903513 | 507.000 | 7.578× | 175.000 | 2.616× | false |
| 1x2-w16 | accepted | 10.715802 | 37.000 | 3.453× | 24.000 | 2.240× | false |
| 1x2-w16 | fresh | 10.491810 | 37.000 | 3.527× | 24.000 | 2.287× | false |
| 1x2-w16 | tree | 10.499122 | 37.000 | 3.524× | 24.000 | 2.286× | false |

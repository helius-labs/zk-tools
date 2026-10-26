# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w16 | accepted | 15.633843 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.235559 | 0.656040 | — | — | — |
| 1x2-w16 | on | 10.124579 | 0.977706 | — | — | — |
| 1x2-w32 | accepted | 8.588341 | 1.000000 | 199.067 | 8.830347 | 114.75 |
| 1x2-w32 | off | 8.423921 | 0.979253 | 197.669 | 8.636959 | 117.75 |
| 1x2-w32 | on | 8.544789 | 0.994929 | 198.168 | 8.675087 | 123.25 |
| 8x2-w32 | accepted | 25.218353 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.672123 | 0.978340 | — | — | — |
| 8x2-w32 | on | 24.839258 | 0.973314 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 36.554 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 31.762 | 0.000 |
| 1x2-w32 | accepted | epoch | 10.623 | 0.000 |
| 1x2-w32 | accepted | fft | 4.059 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.353 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.585 | 0.000 |
| 1x2-w32 | accepted | other | 5.410 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.334 | 0.000 |
| 1x2-w32 | accepted | solver | 1.320 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.034 | 0.480 |
| 1x2-w32 | off | G2_arithmetic | 32.496 | 0.735 |
| 1x2-w32 | off | epoch | 9.005 | -1.618 |
| 1x2-w32 | off | fft | 4.859 | 0.800 |
| 1x2-w32 | off | grouping | 0.512 | 0.159 |
| 1x2-w32 | off | inverse | 6.206 | -0.379 |
| 1x2-w32 | off | other | 4.818 | -0.592 |
| 1x2-w32 | off | scheduler | 3.734 | 0.400 |
| 1x2-w32 | off | solver | 1.335 | 0.015 |
| 1x2-w32 | on | G1_arithmetic | 38.693 | 2.139 |
| 1x2-w32 | on | G2_arithmetic | 28.850 | -2.911 |
| 1x2-w32 | on | epoch | 9.222 | -1.401 |
| 1x2-w32 | on | fft | 4.640 | 0.581 |
| 1x2-w32 | on | grouping | 0.450 | 0.097 |
| 1x2-w32 | on | inverse | 7.316 | 0.731 |
| 1x2-w32 | on | other | 4.923 | -0.487 |
| 1x2-w32 | on | scheduler | 4.303 | 0.969 |
| 1x2-w32 | on | solver | 1.602 | 0.283 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.588341 | 37.000 | 4.308× | 24.000 | 2.794× | false |
| 1x2-w32 | off | 8.423921 | 37.000 | 4.392× | 24.000 | 2.849× | false |
| 1x2-w32 | on | 8.544789 | 37.000 | 4.330× | 24.000 | 2.809× | false |
| 8x2-w32 | accepted | 25.218353 | 189.000 | 7.495× | 74.000 | 2.934× | false |
| 8x2-w32 | off | 24.672123 | 189.000 | 7.660× | 74.000 | 2.999× | false |
| 8x2-w32 | on | 24.839258 | 189.000 | 7.609× | 74.000 | 2.979× | false |

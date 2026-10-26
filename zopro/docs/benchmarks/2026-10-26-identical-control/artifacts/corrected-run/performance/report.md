# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | a | 34.641517 | 1.000000 | — | — | — |
| 12x2-w32 | b | 33.899064 | 0.981255 | — | — | — |
| 16x2-w32 | a | 40.705957 | 1.000000 | — | — | — |
| 16x2-w32 | b | 40.785367 | 1.001951 | — | — | — |
| 1x2-w32 | a | 8.600057 | 1.000000 | 201.713 | 8.782604 | 117.50 |
| 1x2-w32 | b | 8.611779 | 0.998143 | 201.210 | 8.740355 | 118.00 |
| 24x2-w32 | a | 67.542436 | 1.000000 | — | — | — |
| 24x2-w32 | b | 68.388142 | 1.012521 | — | — | — |
| 2x2-w32 | a | 11.555575 | 1.000000 | — | — | — |
| 2x2-w32 | b | 11.675460 | 1.010375 | — | — | — |
| 4x2-w32 | a | 15.990403 | 1.000000 | — | — | — |
| 4x2-w32 | b | 16.217802 | 0.991422 | — | — | — |
| 8x2-w32 | a | 24.345417 | 1.000000 | — | — | — |
| 8x2-w32 | b | 24.317866 | 0.994042 | — | — | — |
| merge-24x1-w32 | a | 67.216960 | 1.000000 | — | — | — |
| merge-24x1-w32 | b | 67.190340 | 0.999604 | — | — | — |
| merge-54x1-w32 | a | 176.415503 | 1.000000 | — | — | — |
| merge-54x1-w32 | b | 175.910365 | 0.995556 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | a | G1_arithmetic | 39.492 | 0.000 |
| 1x2-w32 | a | G2_arithmetic | 30.345 | 0.000 |
| 1x2-w32 | a | epoch | 8.142 | 0.000 |
| 1x2-w32 | a | fft | 4.600 | 0.000 |
| 1x2-w32 | a | grouping | 0.550 | 0.000 |
| 1x2-w32 | a | inverse | 7.840 | 0.000 |
| 1x2-w32 | a | other | 2.833 | 0.000 |
| 1x2-w32 | a | scheduler | 4.629 | 0.000 |
| 1x2-w32 | a | solver | 1.569 | 0.000 |
| 1x2-w32 | b | G1_arithmetic | 34.962 | -4.529 |
| 1x2-w32 | b | G2_arithmetic | 33.449 | 3.104 |
| 1x2-w32 | b | epoch | 8.717 | 0.574 |
| 1x2-w32 | b | fft | 5.134 | 0.533 |
| 1x2-w32 | b | grouping | 1.233 | 0.683 |
| 1x2-w32 | b | inverse | 7.184 | -0.656 |
| 1x2-w32 | b | other | 4.085 | 1.251 |
| 1x2-w32 | b | scheduler | 4.191 | -0.438 |
| 1x2-w32 | b | solver | 1.046 | -0.523 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | a | 8.600057 | 37.000 | 4.302× | 24.000 | 2.791× | false |
| 1x2-w32 | b | 8.611779 | 37.000 | 4.296× | 24.000 | 2.787× | false |
| 2x2-w32 | a | 11.555575 | 59.000 | 5.106× | 30.000 | 2.596× | false |
| 2x2-w32 | b | 11.675460 | 59.000 | 5.053× | 30.000 | 2.569× | false |
| 4x2-w32 | a | 15.990403 | 109.000 | 6.817× | 48.000 | 3.002× | false |
| 4x2-w32 | b | 16.217802 | 109.000 | 6.721× | 48.000 | 2.960× | false |
| 8x2-w32 | a | 24.345417 | 189.000 | 7.763× | 74.000 | 3.040× | false |
| 8x2-w32 | b | 24.317866 | 189.000 | 7.772× | 74.000 | 3.043× | false |
| 12x2-w32 | a | 34.641517 | 284.000 | 8.198× | 101.000 | 2.916× | false |
| 12x2-w32 | b | 33.899064 | 284.000 | 8.378× | 101.000 | 2.979× | false |
| 16x2-w32 | a | 40.705957 | 335.000 | 8.230× | 124.000 | 3.046× | false |
| 16x2-w32 | b | 40.785367 | 335.000 | 8.214× | 124.000 | 3.040× | false |
| 24x2-w32 | a | 67.542436 | 528.000 | 7.817× | 176.000 | 2.606× | false |
| 24x2-w32 | b | 68.388142 | 528.000 | 7.721× | 176.000 | 2.574× | false |
| merge-24x1-w32 | a | 67.216960 | 507.000 | 7.543× | 175.000 | 2.604× | false |
| merge-24x1-w32 | b | 67.190340 | 507.000 | 7.546× | 175.000 | 2.605× | false |

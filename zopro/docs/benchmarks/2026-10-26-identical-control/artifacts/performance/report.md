# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | a | 34.167559 | 1.000000 | — | — | — |
| 12x2-w32 | b | 34.275095 | 0.993739 | — | — | — |
| 16x2-w32 | a | 40.581990 | 1.000000 | — | — | — |
| 16x2-w32 | b | 40.786122 | 1.001658 | — | — | — |
| 1x2-w32 | a | 8.614940 | 1.000000 | 198.008 | 8.993010 | 120.75 |
| 1x2-w32 | b | 8.621363 | 0.999538 | 199.340 | 8.688359 | 121.00 |
| 24x2-w32 | a | 68.383259 | 1.000000 | — | — | — |
| 24x2-w32 | b | 68.651877 | 1.013098 | — | — | — |
| 2x2-w32 | a | 11.708770 | 1.000000 | — | — | — |
| 2x2-w32 | b | 11.628068 | 0.995191 | — | — | — |
| 4x2-w32 | a | 16.384417 | 1.000000 | — | — | — |
| 4x2-w32 | b | 16.488764 | 1.008751 | — | — | — |
| 8x2-w32 | a | 25.574581 | 1.000000 | — | — | — |
| 8x2-w32 | b | 24.273699 | 0.960417 | — | — | — |
| merge-24x1-w32 | a | 67.906088 | 1.000000 | — | — | — |
| merge-24x1-w32 | b | 67.680897 | 0.999260 | — | — | — |
| merge-54x1-w32 | a | 177.603012 | 1.000000 | — | — | — |
| merge-54x1-w32 | b | 177.762454 | 1.002993 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | a | G1_arithmetic | 39.764 | 0.000 |
| 1x2-w32 | a | G2_arithmetic | 29.601 | 0.000 |
| 1x2-w32 | a | epoch | 7.634 | 0.000 |
| 1x2-w32 | a | fft | 1.810 | 0.000 |
| 1x2-w32 | a | grouping | 1.887 | 0.000 |
| 1x2-w32 | a | inverse | 7.376 | 0.000 |
| 1x2-w32 | a | other | 6.697 | 0.000 |
| 1x2-w32 | a | scheduler | 4.354 | 0.000 |
| 1x2-w32 | a | solver | 0.876 | 0.000 |
| 1x2-w32 | b | G1_arithmetic | 39.206 | -0.558 |
| 1x2-w32 | b | G2_arithmetic | 28.586 | -1.015 |
| 1x2-w32 | b | epoch | 14.146 | 6.512 |
| 1x2-w32 | b | fft | 2.040 | 0.230 |
| 1x2-w32 | b | grouping | 0.338 | -1.549 |
| 1x2-w32 | b | inverse | 6.908 | -0.468 |
| 1x2-w32 | b | other | 4.488 | -2.210 |
| 1x2-w32 | b | scheduler | 3.461 | -0.894 |
| 1x2-w32 | b | solver | 0.829 | -0.047 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | a | 8.614940 | 37.000 | 4.295× | 24.000 | 2.786× | false |
| 1x2-w32 | b | 8.621363 | 37.000 | 4.292× | 24.000 | 2.784× | false |
| 2x2-w32 | a | 11.708770 | 59.000 | 5.039× | 30.000 | 2.562× | false |
| 2x2-w32 | b | 11.628068 | 59.000 | 5.074× | 30.000 | 2.580× | false |
| 4x2-w32 | a | 16.384417 | 109.000 | 6.653× | 48.000 | 2.930× | false |
| 4x2-w32 | b | 16.488764 | 109.000 | 6.611× | 48.000 | 2.911× | false |
| 8x2-w32 | a | 25.574581 | 189.000 | 7.390× | 74.000 | 2.893× | false |
| 8x2-w32 | b | 24.273699 | 189.000 | 7.786× | 74.000 | 3.049× | false |
| 12x2-w32 | a | 34.167559 | 284.000 | 8.312× | 101.000 | 2.956× | false |
| 12x2-w32 | b | 34.275095 | 284.000 | 8.286× | 101.000 | 2.947× | false |
| 16x2-w32 | a | 40.581990 | 335.000 | 8.255× | 124.000 | 3.056× | false |
| 16x2-w32 | b | 40.786122 | 335.000 | 8.214× | 124.000 | 3.040× | false |
| 24x2-w32 | a | 68.383259 | 528.000 | 7.721× | 176.000 | 2.574× | false |
| 24x2-w32 | b | 68.651877 | 528.000 | 7.691× | 176.000 | 2.564× | false |
| merge-24x1-w32 | a | 67.906088 | 507.000 | 7.466× | 175.000 | 2.577× | false |
| merge-24x1-w32 | b | 67.680897 | 507.000 | 7.491× | 175.000 | 2.586× | false |

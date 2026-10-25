# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | accepted | 8.823098 | 1.000000 | 195.803 | 8.692375 | 126.00 |
| 1x2-w32 | diagnostic-on | 8.657005 | 0.948845 | 197.724 | 8.725864 | 129.50 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 37.732 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.774 | 0.000 |
| 1x2-w32 | accepted | epoch | 10.013 | 0.000 |
| 1x2-w32 | accepted | fft | 5.215 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.703 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.484 | 0.000 |
| 1x2-w32 | accepted | other | 4.974 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.375 | 0.000 |
| 1x2-w32 | accepted | solver | 1.729 | 0.000 |
| 1x2-w32 | diagnostic-on | G1_arithmetic | 38.727 | 0.995 |
| 1x2-w32 | diagnostic-on | G2_arithmetic | 28.972 | -0.803 |
| 1x2-w32 | diagnostic-on | epoch | 9.721 | -0.292 |
| 1x2-w32 | diagnostic-on | fft | 4.570 | -0.645 |
| 1x2-w32 | diagnostic-on | grouping | 0.527 | -0.177 |
| 1x2-w32 | diagnostic-on | inverse | 7.578 | 1.094 |
| 1x2-w32 | diagnostic-on | other | 4.757 | -0.217 |
| 1x2-w32 | diagnostic-on | scheduler | 3.555 | 0.180 |
| 1x2-w32 | diagnostic-on | solver | 1.595 | -0.134 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.823098 | 37.000 | 4.194× | 24.000 | 2.720× | false |
| 1x2-w32 | diagnostic-on | 8.657005 | 37.000 | 4.274× | 24.000 | 2.772× | false |

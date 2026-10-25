# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.928809 | 1.000000 | — | — | — |
| 12x2-w32 | refactor | 34.386319 | 0.981471 | — | — | — |
| 16x2-w32 | accepted | 41.694141 | 1.000000 | — | — | — |
| 16x2-w32 | refactor | 40.867383 | 0.981703 | — | — | — |
| 1x2-w32 | accepted | 8.624253 | 1.000000 | 200.411 | 8.763007 | 128.75 |
| 1x2-w32 | refactor | 8.594267 | 0.996336 | 199.471 | 8.835961 | 129.00 |
| 24x2-w32 | accepted | 69.786302 | 1.000000 | — | — | — |
| 24x2-w32 | refactor | 69.265160 | 0.986565 | — | — | — |
| 2x2-w32 | accepted | 11.884072 | 1.000000 | — | — | — |
| 2x2-w32 | refactor | 11.697247 | 0.984343 | — | — | — |
| 4x2-w32 | accepted | 16.255464 | 1.000000 | — | — | — |
| 4x2-w32 | refactor | 16.158262 | 0.997694 | — | — | — |
| 8x2-w32 | accepted | 24.899371 | 1.000000 | — | — | — |
| 8x2-w32 | refactor | 24.477005 | 0.997987 | — | — | — |
| merge-24x1-w32 | accepted | 68.377620 | 1.000000 | — | — | — |
| merge-24x1-w32 | refactor | 68.150148 | 0.990599 | — | — | — |
| merge-54x1-w32 | accepted | 177.480120 | 1.000000 | 4893.403 | 177.717594 | 1689.64 |
| merge-54x1-w32 | refactor | 178.891799 | 1.010771 | 4921.851 | 180.155861 | 1668.96 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_stream | 16.421 | 0.000 |
| 1x2-w32 | accepted | G2_stream | 15.314 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.844 | 0.000 |
| 1x2-w32 | accepted | fft | 4.608 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.691 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.023 | 0.000 |
| 1x2-w32 | accepted | other | 40.381 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.007 | 0.000 |
| 1x2-w32 | accepted | solver | 1.712 | 0.000 |
| 1x2-w32 | refactor | G1_stream | 16.082 | -0.339 |
| 1x2-w32 | refactor | G2_stream | 14.938 | -0.376 |
| 1x2-w32 | refactor | epoch | 9.910 | 0.066 |
| 1x2-w32 | refactor | fft | 4.327 | -0.281 |
| 1x2-w32 | refactor | grouping | 0.651 | -0.041 |
| 1x2-w32 | refactor | inverse | 7.237 | 0.214 |
| 1x2-w32 | refactor | other | 41.308 | 0.927 |
| 1x2-w32 | refactor | scheduler | 4.029 | 0.022 |
| 1x2-w32 | refactor | solver | 1.519 | -0.193 |
| merge-54x1-w32 | accepted | G2_stream | 2.833 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.294 | 0.000 |
| merge-54x1-w32 | accepted | fft | 41.806 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.453 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.794 | 0.000 |
| merge-54x1-w32 | accepted | other | 42.247 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.186 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.387 | 0.000 |
| merge-54x1-w32 | refactor | G2_stream | 2.804 | -0.029 |
| merge-54x1-w32 | refactor | epoch | 2.127 | -0.166 |
| merge-54x1-w32 | refactor | fft | 41.833 | 0.027 |
| merge-54x1-w32 | refactor | grouping | 1.418 | -0.035 |
| merge-54x1-w32 | refactor | inverse | 1.837 | 0.043 |
| merge-54x1-w32 | refactor | other | 42.683 | 0.437 |
| merge-54x1-w32 | refactor | scheduler | 2.109 | -0.077 |
| merge-54x1-w32 | refactor | solver | 5.188 | -0.199 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.624253 | 37.000 | 4.290× | 24.000 | 2.783× | false |
| 1x2-w32 | refactor | 8.594267 | 37.000 | 4.305× | 24.000 | 2.793× | false |
| 2x2-w32 | accepted | 11.884072 | 59.000 | 4.965× | 30.000 | 2.524× | false |
| 2x2-w32 | refactor | 11.697247 | 59.000 | 5.044× | 30.000 | 2.565× | false |
| 4x2-w32 | accepted | 16.255464 | 109.000 | 6.705× | 48.000 | 2.953× | false |
| 4x2-w32 | refactor | 16.158262 | 109.000 | 6.746× | 48.000 | 2.971× | false |
| 8x2-w32 | accepted | 24.899371 | 189.000 | 7.591× | 74.000 | 2.972× | false |
| 8x2-w32 | refactor | 24.477005 | 189.000 | 7.722× | 74.000 | 3.023× | false |
| 12x2-w32 | accepted | 34.928809 | 284.000 | 8.131× | 101.000 | 2.892× | false |
| 12x2-w32 | refactor | 34.386319 | 284.000 | 8.259× | 101.000 | 2.937× | false |
| 16x2-w32 | accepted | 41.694141 | 335.000 | 8.035× | 124.000 | 2.974× | false |
| 16x2-w32 | refactor | 40.867383 | 335.000 | 8.197× | 124.000 | 3.034× | false |
| 24x2-w32 | accepted | 69.786302 | 528.000 | 7.566× | 176.000 | 2.522× | false |
| 24x2-w32 | refactor | 69.265160 | 528.000 | 7.623× | 176.000 | 2.541× | false |
| merge-24x1-w32 | accepted | 68.377620 | 507.000 | 7.415× | 175.000 | 2.559× | false |
| merge-24x1-w32 | refactor | 68.150148 | 507.000 | 7.439× | 175.000 | 2.568× | false |

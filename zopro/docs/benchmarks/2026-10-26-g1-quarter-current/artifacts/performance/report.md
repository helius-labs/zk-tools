# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.467557 | 1.000000 | — | — | — |
| 12x2-w32 | off | 34.799252 | 0.997055 | — | — | — |
| 12x2-w32 | on | 34.356977 | 0.999131 | — | — | — |
| 16x2-w32 | accepted | 40.553543 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.985076 | 1.005821 | — | — | — |
| 16x2-w32 | on | 40.493993 | 0.996176 | — | — | — |
| 1x2-w16 | accepted | 11.489336 | 1.000000 | — | — | — |
| 1x2-w16 | off | 11.395629 | 1.011458 | — | — | — |
| 1x2-w16 | on | 11.033937 | 0.908839 | — | — | — |
| 1x2-w32 | accepted | 8.593896 | 1.000000 | 200.843 | 8.780007 | 120.75 |
| 1x2-w32 | off | 8.599606 | 1.004352 | 200.613 | 8.706561 | 116.75 |
| 1x2-w32 | on | 8.554091 | 0.995368 | 199.411 | 8.646354 | 118.25 |
| 24x2-w32 | accepted | 68.268380 | 1.000000 | — | — | — |
| 24x2-w32 | off | 69.036326 | 1.006833 | — | — | — |
| 24x2-w32 | on | 68.081392 | 0.999661 | — | — | — |
| 2x2-w32 | accepted | 11.606487 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.686168 | 1.001403 | — | — | — |
| 2x2-w32 | on | 11.668644 | 1.005355 | — | — | — |
| 4x2-w32 | accepted | 16.179431 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.081585 | 1.003176 | — | — | — |
| 4x2-w32 | on | 15.963834 | 0.988758 | — | — | — |
| 8x2-w32 | accepted | 24.772574 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.583470 | 0.984295 | — | — | — |
| 8x2-w32 | on | 24.807339 | 0.987200 | — | — | — |
| merge-24x1-w32 | accepted | 67.225732 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 67.291500 | 1.002774 | — | — | — |
| merge-24x1-w32 | on | 67.628824 | 1.004834 | — | — | — |
| merge-54x1-w32 | accepted | 176.423318 | 1.000000 | — | — | — |
| merge-54x1-w32 | off | 177.837778 | 1.012131 | — | — | — |
| merge-54x1-w32 | on | 178.548536 | 1.012046 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 39.895 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.873 | 0.000 |
| 1x2-w32 | accepted | epoch | 12.223 | 0.000 |
| 1x2-w32 | accepted | fft | 3.012 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.532 | 0.000 |
| 1x2-w32 | accepted | inverse | 5.692 | 0.000 |
| 1x2-w32 | accepted | other | 5.317 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.202 | 0.000 |
| 1x2-w32 | accepted | solver | 1.253 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 34.321 | -5.574 |
| 1x2-w32 | off | G2_arithmetic | 34.514 | 5.640 |
| 1x2-w32 | off | epoch | 11.657 | -0.566 |
| 1x2-w32 | off | fft | 4.598 | 1.586 |
| 1x2-w32 | off | grouping | 1.024 | 0.491 |
| 1x2-w32 | off | inverse | 6.354 | 0.662 |
| 1x2-w32 | off | other | 5.037 | -0.279 |
| 1x2-w32 | off | scheduler | 1.524 | -1.678 |
| 1x2-w32 | off | solver | 0.971 | -0.282 |
| 1x2-w32 | on | G1_arithmetic | 43.071 | 3.175 |
| 1x2-w32 | on | G2_arithmetic | 27.152 | -1.721 |
| 1x2-w32 | on | epoch | 10.124 | -2.099 |
| 1x2-w32 | on | fft | 4.254 | 1.242 |
| 1x2-w32 | on | g1_quarter | 0.451 | 0.451 |
| 1x2-w32 | on | grouping | 0.431 | -0.101 |
| 1x2-w32 | on | inverse | 5.289 | -0.402 |
| 1x2-w32 | on | other | 4.497 | -0.819 |
| 1x2-w32 | on | scheduler | 2.997 | -0.205 |
| 1x2-w32 | on | solver | 1.733 | 0.480 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.593896 | 37.000 | 4.305× | 24.000 | 2.793× | false |
| 1x2-w32 | off | 8.599606 | 37.000 | 4.303× | 24.000 | 2.791× | false |
| 1x2-w32 | on | 8.554091 | 37.000 | 4.325× | 24.000 | 2.806× | false |
| 2x2-w32 | accepted | 11.606487 | 59.000 | 5.083× | 30.000 | 2.585× | false |
| 2x2-w32 | off | 11.686168 | 59.000 | 5.049× | 30.000 | 2.567× | false |
| 2x2-w32 | on | 11.668644 | 59.000 | 5.056× | 30.000 | 2.571× | false |
| 4x2-w32 | accepted | 16.179431 | 109.000 | 6.737× | 48.000 | 2.967× | false |
| 4x2-w32 | off | 16.081585 | 109.000 | 6.778× | 48.000 | 2.985× | false |
| 4x2-w32 | on | 15.963834 | 109.000 | 6.828× | 48.000 | 3.007× | false |
| 8x2-w32 | accepted | 24.772574 | 189.000 | 7.629× | 74.000 | 2.987× | false |
| 8x2-w32 | off | 24.583470 | 189.000 | 7.688× | 74.000 | 3.010× | false |
| 8x2-w32 | on | 24.807339 | 189.000 | 7.619× | 74.000 | 2.983× | false |
| 12x2-w32 | accepted | 34.467557 | 284.000 | 8.240× | 101.000 | 2.930× | false |
| 12x2-w32 | off | 34.799252 | 284.000 | 8.161× | 101.000 | 2.902× | false |
| 12x2-w32 | on | 34.356977 | 284.000 | 8.266× | 101.000 | 2.940× | false |
| 16x2-w32 | accepted | 40.553543 | 335.000 | 8.261× | 124.000 | 3.058× | false |
| 16x2-w32 | off | 40.985076 | 335.000 | 8.174× | 124.000 | 3.025× | false |
| 16x2-w32 | on | 40.493993 | 335.000 | 8.273× | 124.000 | 3.062× | false |
| 24x2-w32 | accepted | 68.268380 | 528.000 | 7.734× | 176.000 | 2.578× | false |
| 24x2-w32 | off | 69.036326 | 528.000 | 7.648× | 176.000 | 2.549× | false |
| 24x2-w32 | on | 68.081392 | 528.000 | 7.755× | 176.000 | 2.585× | false |
| merge-24x1-w32 | accepted | 67.225732 | 507.000 | 7.542× | 175.000 | 2.603× | false |
| merge-24x1-w32 | off | 67.291500 | 507.000 | 7.534× | 175.000 | 2.601× | false |
| merge-24x1-w32 | on | 67.628824 | 507.000 | 7.497× | 175.000 | 2.588× | false |
| 1x2-w16 | accepted | 11.489336 | 37.000 | 3.220× | 24.000 | 2.089× | false |
| 1x2-w16 | off | 11.395629 | 37.000 | 3.247× | 24.000 | 2.106× | false |
| 1x2-w16 | on | 11.033937 | 37.000 | 3.353× | 24.000 | 2.175× | false |

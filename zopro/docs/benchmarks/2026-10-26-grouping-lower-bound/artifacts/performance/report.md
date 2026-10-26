# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.973165 | 1.000000 | — | — | — |
| 12x2-w32 | off | 34.781885 | 0.994984 | — | — | — |
| 12x2-w32 | on | 34.497167 | 0.986390 | — | — | — |
| 16x2-w32 | accepted | 40.529709 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.970010 | 1.010864 | — | — | — |
| 16x2-w32 | on | 40.791761 | 0.999097 | — | — | — |
| 1x2-w32 | accepted | 9.076810 | 1.000000 | 200.868 | 8.772115 | 121.75 |
| 1x2-w32 | off | 8.640367 | 0.952370 | 202.387 | 8.814303 | 122.25 |
| 1x2-w32 | on | 8.594861 | 0.946903 | 201.693 | 8.688230 | 77.75 |
| 24x2-w32 | accepted | 68.193555 | 1.000000 | — | — | — |
| 24x2-w32 | off | 69.053378 | 1.011560 | — | — | — |
| 24x2-w32 | on | 68.955805 | 1.006827 | — | — | — |
| 2x2-w32 | accepted | 11.638353 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.693535 | 1.005139 | — | — | — |
| 2x2-w32 | on | 11.777965 | 1.012667 | — | — | — |
| 4x2-w32 | accepted | 16.317358 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.409943 | 1.006602 | — | — | — |
| 4x2-w32 | on | 16.378142 | 1.001665 | — | — | — |
| 8x2-w32 | accepted | 24.329279 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.998121 | 1.010311 | — | — | — |
| 8x2-w32 | on | 25.008352 | 1.039871 | — | — | — |
| merge-24x1-w32 | accepted | 67.461142 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.021279 | 1.008303 | — | — | — |
| merge-24x1-w32 | on | 68.544458 | 1.010911 | — | — | — |
| merge-54x1-w32 | accepted | 175.645760 | 1.000000 | — | — | — |
| merge-54x1-w32 | off | 178.766112 | 1.002972 | — | — | — |
| merge-54x1-w32 | on | 176.242775 | 1.003399 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 36.980 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 31.346 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.113 | 0.000 |
| 1x2-w32 | accepted | fft | 4.563 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.226 | 0.000 |
| 1x2-w32 | accepted | inverse | 5.702 | 0.000 |
| 1x2-w32 | accepted | other | 5.015 | 0.000 |
| 1x2-w32 | accepted | scheduler | 4.961 | 0.000 |
| 1x2-w32 | accepted | solver | 2.092 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 40.297 | 3.316 |
| 1x2-w32 | off | G2_arithmetic | 29.391 | -1.954 |
| 1x2-w32 | off | epoch | 8.685 | -0.429 |
| 1x2-w32 | off | fft | 5.121 | 0.558 |
| 1x2-w32 | off | grouping | 0.008 | -0.219 |
| 1x2-w32 | off | inverse | 9.120 | 3.418 |
| 1x2-w32 | off | other | 3.281 | -1.734 |
| 1x2-w32 | off | scheduler | 3.301 | -1.660 |
| 1x2-w32 | off | solver | 0.796 | -1.296 |
| 1x2-w32 | on | G1_arithmetic | 41.740 | 4.760 |
| 1x2-w32 | on | G2_arithmetic | 31.805 | 0.459 |
| 1x2-w32 | on | epoch | 7.380 | -1.733 |
| 1x2-w32 | on | fft | 2.077 | -2.487 |
| 1x2-w32 | on | grouping | 0.001 | -0.226 |
| 1x2-w32 | on | inverse | 8.153 | 2.451 |
| 1x2-w32 | on | other | 5.736 | 0.721 |
| 1x2-w32 | on | scheduler | 1.899 | -3.062 |
| 1x2-w32 | on | solver | 1.208 | -0.884 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 9.076810 | 37.000 | 4.076× | 24.000 | 2.644× | false |
| 1x2-w32 | off | 8.640367 | 37.000 | 4.282× | 24.000 | 2.778× | false |
| 1x2-w32 | on | 8.594861 | 37.000 | 4.305× | 24.000 | 2.792× | false |
| 2x2-w32 | accepted | 11.638353 | 59.000 | 5.069× | 30.000 | 2.578× | false |
| 2x2-w32 | off | 11.693535 | 59.000 | 5.046× | 30.000 | 2.566× | false |
| 2x2-w32 | on | 11.777965 | 59.000 | 5.009× | 30.000 | 2.547× | false |
| 4x2-w32 | accepted | 16.317358 | 109.000 | 6.680× | 48.000 | 2.942× | false |
| 4x2-w32 | off | 16.409943 | 109.000 | 6.642× | 48.000 | 2.925× | false |
| 4x2-w32 | on | 16.378142 | 109.000 | 6.655× | 48.000 | 2.931× | false |
| 8x2-w32 | accepted | 24.329279 | 189.000 | 7.768× | 74.000 | 3.042× | false |
| 8x2-w32 | off | 24.998121 | 189.000 | 7.561× | 74.000 | 2.960× | false |
| 8x2-w32 | on | 25.008352 | 189.000 | 7.557× | 74.000 | 2.959× | false |
| 12x2-w32 | accepted | 34.973165 | 284.000 | 8.121× | 101.000 | 2.888× | false |
| 12x2-w32 | off | 34.781885 | 284.000 | 8.165× | 101.000 | 2.904× | false |
| 12x2-w32 | on | 34.497167 | 284.000 | 8.233× | 101.000 | 2.928× | false |
| 16x2-w32 | accepted | 40.529709 | 335.000 | 8.266× | 124.000 | 3.059× | false |
| 16x2-w32 | off | 40.970010 | 335.000 | 8.177× | 124.000 | 3.027× | false |
| 16x2-w32 | on | 40.791761 | 335.000 | 8.212× | 124.000 | 3.040× | false |
| 24x2-w32 | accepted | 68.193555 | 528.000 | 7.743× | 176.000 | 2.581× | false |
| 24x2-w32 | off | 69.053378 | 528.000 | 7.646× | 176.000 | 2.549× | false |
| 24x2-w32 | on | 68.955805 | 528.000 | 7.657× | 176.000 | 2.552× | false |
| merge-24x1-w32 | accepted | 67.461142 | 507.000 | 7.515× | 175.000 | 2.594× | false |
| merge-24x1-w32 | off | 68.021279 | 507.000 | 7.454× | 175.000 | 2.573× | false |
| merge-24x1-w32 | on | 68.544458 | 507.000 | 7.397× | 175.000 | 2.553× | false |

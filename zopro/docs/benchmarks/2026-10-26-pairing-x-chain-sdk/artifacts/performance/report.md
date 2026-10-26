# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.551946 | 1.000000 | — | — | — |
| 12x2-w32 | off | 34.643139 | 0.996436 | — | — | — |
| 12x2-w32 | on | 34.542959 | 1.005552 | — | — | — |
| 16x2-w32 | accepted | 40.814660 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.502662 | 0.995268 | — | — | — |
| 16x2-w32 | on | 40.649779 | 0.991719 | — | — | — |
| 1x2-w16 | accepted | 10.929908 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.379018 | 0.956767 | — | — | — |
| 1x2-w16 | on | 10.576250 | 0.969344 | — | — | — |
| 1x2-w32 | accepted | 8.642453 | 1.000000 | 199.921 | 8.752575 | 135.00 |
| 1x2-w32 | off | 8.619118 | 1.000013 | 200.359 | 8.782271 | 137.75 |
| 1x2-w32 | on | 8.618067 | 0.994698 | 199.030 | 8.898156 | 139.25 |
| 24x2-w32 | accepted | 68.811062 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.892877 | 1.001395 | — | — | — |
| 24x2-w32 | on | 68.535851 | 0.995346 | — | — | — |
| 2x2-w32 | accepted | 11.756497 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.642822 | 0.991300 | — | — | — |
| 2x2-w32 | on | 11.636600 | 0.989669 | — | — | — |
| 4x2-w32 | accepted | 16.230727 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.171866 | 0.990020 | — | — | — |
| 4x2-w32 | on | 16.027312 | 0.991636 | — | — | — |
| 8x2-w32 | accepted | 24.740679 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.556572 | 1.001513 | — | — | — |
| 8x2-w32 | on | 24.665167 | 1.005424 | — | — | — |
| merge-24x1-w32 | accepted | 67.064555 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 67.295080 | 1.005148 | — | — | — |
| merge-24x1-w32 | on | 67.312555 | 1.005786 | — | — | — |
| merge-54x1-w32 | accepted | 177.082271 | 1.000000 | — | — | — |
| merge-54x1-w32 | off | 174.691230 | 0.993353 | — | — | — |
| merge-54x1-w32 | on | 175.679550 | 0.997727 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 41.197 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 27.363 | 0.000 |
| 1x2-w32 | accepted | epoch | 8.545 | 0.000 |
| 1x2-w32 | accepted | fft | 5.058 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.882 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.458 | 0.000 |
| 1x2-w32 | accepted | other | 4.426 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.815 | 0.000 |
| 1x2-w32 | accepted | solver | 1.256 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 39.570 | -1.628 |
| 1x2-w32 | off | G2_arithmetic | 29.031 | 1.669 |
| 1x2-w32 | off | epoch | 9.629 | 1.083 |
| 1x2-w32 | off | fft | 4.321 | -0.737 |
| 1x2-w32 | off | grouping | 0.472 | -0.410 |
| 1x2-w32 | off | inverse | 7.505 | 0.048 |
| 1x2-w32 | off | other | 4.626 | 0.200 |
| 1x2-w32 | off | scheduler | 3.415 | -0.400 |
| 1x2-w32 | off | solver | 1.430 | 0.174 |
| 1x2-w32 | on | G1_arithmetic | 38.158 | -3.039 |
| 1x2-w32 | on | G2_arithmetic | 29.464 | 2.102 |
| 1x2-w32 | on | epoch | 9.878 | 1.332 |
| 1x2-w32 | on | fft | 5.076 | 0.018 |
| 1x2-w32 | on | grouping | 0.939 | 0.057 |
| 1x2-w32 | on | inverse | 5.757 | -1.701 |
| 1x2-w32 | on | other | 5.299 | 0.873 |
| 1x2-w32 | on | scheduler | 4.505 | 0.690 |
| 1x2-w32 | on | solver | 0.924 | -0.331 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.642453 | 37.000 | 4.281× | 24.000 | 2.777× | false |
| 1x2-w32 | off | 8.619118 | 37.000 | 4.293× | 24.000 | 2.785× | false |
| 1x2-w32 | on | 8.618067 | 37.000 | 4.293× | 24.000 | 2.785× | false |
| 2x2-w32 | accepted | 11.756497 | 59.000 | 5.019× | 30.000 | 2.552× | false |
| 2x2-w32 | off | 11.642822 | 59.000 | 5.067× | 30.000 | 2.577× | false |
| 2x2-w32 | on | 11.636600 | 59.000 | 5.070× | 30.000 | 2.578× | false |
| 4x2-w32 | accepted | 16.230727 | 109.000 | 6.716× | 48.000 | 2.957× | false |
| 4x2-w32 | off | 16.171866 | 109.000 | 6.740× | 48.000 | 2.968× | false |
| 4x2-w32 | on | 16.027312 | 109.000 | 6.801× | 48.000 | 2.995× | false |
| 8x2-w32 | accepted | 24.740679 | 189.000 | 7.639× | 74.000 | 2.991× | false |
| 8x2-w32 | off | 24.556572 | 189.000 | 7.697× | 74.000 | 3.013× | false |
| 8x2-w32 | on | 24.665167 | 189.000 | 7.663× | 74.000 | 3.000× | false |
| 12x2-w32 | accepted | 34.551946 | 284.000 | 8.220× | 101.000 | 2.923× | false |
| 12x2-w32 | off | 34.643139 | 284.000 | 8.198× | 101.000 | 2.915× | false |
| 12x2-w32 | on | 34.542959 | 284.000 | 8.222× | 101.000 | 2.924× | false |
| 16x2-w32 | accepted | 40.814660 | 335.000 | 8.208× | 124.000 | 3.038× | false |
| 16x2-w32 | off | 40.502662 | 335.000 | 8.271× | 124.000 | 3.062× | false |
| 16x2-w32 | on | 40.649779 | 335.000 | 8.241× | 124.000 | 3.050× | false |
| 24x2-w32 | accepted | 68.811062 | 528.000 | 7.673× | 176.000 | 2.558× | false |
| 24x2-w32 | off | 68.892877 | 528.000 | 7.664× | 176.000 | 2.555× | false |
| 24x2-w32 | on | 68.535851 | 528.000 | 7.704× | 176.000 | 2.568× | false |
| merge-24x1-w32 | accepted | 67.064555 | 507.000 | 7.560× | 175.000 | 2.609× | false |
| merge-24x1-w32 | off | 67.295080 | 507.000 | 7.534× | 175.000 | 2.600× | false |
| merge-24x1-w32 | on | 67.312555 | 507.000 | 7.532× | 175.000 | 2.600× | false |
| 1x2-w16 | accepted | 10.929908 | 37.000 | 3.385× | 24.000 | 2.196× | false |
| 1x2-w16 | off | 10.379018 | 37.000 | 3.565× | 24.000 | 2.312× | false |
| 1x2-w16 | on | 10.576250 | 37.000 | 3.498× | 24.000 | 2.269× | false |

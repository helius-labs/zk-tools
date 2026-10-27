# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.263909 | 1.000000 | — | — | — |
| 12x2-w32 | adaptive | 33.923376 | 0.983759 | — | — | — |
| 12x2-w32 | fresh | 34.511827 | 1.003801 | — | — | — |
| 12x2-w32 | tree | 34.296020 | 0.995475 | — | — | — |
| 16x2-w32 | accepted | 40.899741 | 1.000000 | — | — | — |
| 16x2-w32 | adaptive | 40.679926 | 0.994626 | — | — | — |
| 16x2-w32 | fresh | 41.173699 | 1.012427 | — | — | — |
| 16x2-w32 | tree | 40.370585 | 0.990210 | — | — | — |
| 1x2-w16 | accepted | 10.491323 | 1.000000 | — | — | — |
| 1x2-w16 | adaptive | 10.453992 | 0.992061 | — | — | — |
| 1x2-w16 | fresh | 10.597142 | 1.010086 | — | — | — |
| 1x2-w16 | tree | 10.376707 | 0.995458 | — | — | — |
| 1x2-w32 | accepted | 8.628200 | 1.000000 | 199.276 | 8.877643 | 135.25 |
| 1x2-w32 | adaptive | 8.602090 | 0.996297 | 200.283 | 8.726980 | 133.50 |
| 1x2-w32 | fresh | 8.669467 | 1.003799 | 201.656 | 8.811164 | 135.25 |
| 1x2-w32 | tree | 8.539252 | 0.990533 | 198.865 | 8.732221 | 133.25 |
| 24x2-w32 | accepted | 68.547968 | 1.000000 | — | — | — |
| 24x2-w32 | adaptive | 67.421261 | 0.977847 | — | — | — |
| 24x2-w32 | fresh | 68.877007 | 1.000907 | — | — | — |
| 24x2-w32 | tree | 67.421373 | 0.978128 | — | — | — |
| 2x2-w32 | accepted | 11.726902 | 1.000000 | — | — | — |
| 2x2-w32 | adaptive | 11.753543 | 1.001012 | — | — | — |
| 2x2-w32 | fresh | 11.792506 | 1.005719 | — | — | — |
| 2x2-w32 | tree | 11.825017 | 1.004934 | — | — | — |
| 4x2-w32 | accepted | 16.181883 | 1.000000 | — | — | — |
| 4x2-w32 | adaptive | 16.374466 | 1.010890 | — | — | — |
| 4x2-w32 | fresh | 16.228821 | 1.006533 | — | — | — |
| 4x2-w32 | tree | 16.219163 | 1.003817 | — | — | — |
| 8x2-w32 | accepted | 24.536646 | 1.000000 | — | — | — |
| 8x2-w32 | adaptive | 24.090376 | 0.977716 | — | — | — |
| 8x2-w32 | fresh | 24.766080 | 1.002794 | — | — | — |
| 8x2-w32 | tree | 24.155899 | 0.985635 | — | — | — |
| merge-24x1-w32 | accepted | 67.272049 | 1.000000 | — | — | — |
| merge-24x1-w32 | adaptive | 66.314153 | 0.980849 | — | — | — |
| merge-24x1-w32 | fresh | 67.610169 | 1.004961 | — | — | — |
| merge-24x1-w32 | tree | 66.892252 | 0.992726 | — | — | — |
| merge-54x1-w32 | accepted | 177.333562 | 1.000000 | — | — | — |
| merge-54x1-w32 | adaptive | 163.004334 | 0.919568 | — | — | — |
| merge-54x1-w32 | fresh | 178.373996 | 1.006347 | — | — | — |
| merge-54x1-w32 | tree | 163.038432 | 0.924184 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 39.583 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 27.314 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.147 | 0.000 |
| 1x2-w32 | accepted | fft | 3.980 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.699 | 0.000 |
| 1x2-w32 | accepted | inverse | 8.494 | 0.000 |
| 1x2-w32 | accepted | other | 5.533 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.406 | 0.000 |
| 1x2-w32 | accepted | solver | 1.845 | 0.000 |
| 1x2-w32 | adaptive | G1_arithmetic | 39.693 | 0.110 |
| 1x2-w32 | adaptive | G2_arithmetic | 27.605 | 0.291 |
| 1x2-w32 | adaptive | epoch | 9.124 | -0.022 |
| 1x2-w32 | adaptive | fft | 5.529 | 1.550 |
| 1x2-w32 | adaptive | grouping | 0.681 | -0.019 |
| 1x2-w32 | adaptive | inverse | 6.777 | -1.718 |
| 1x2-w32 | adaptive | other | 4.887 | -0.646 |
| 1x2-w32 | adaptive | scheduler | 4.366 | 0.960 |
| 1x2-w32 | adaptive | solver | 1.338 | -0.506 |
| 1x2-w32 | fresh | G1_arithmetic | 40.491 | 0.908 |
| 1x2-w32 | fresh | G2_arithmetic | 26.392 | -0.922 |
| 1x2-w32 | fresh | epoch | 9.901 | 0.755 |
| 1x2-w32 | fresh | fft | 4.330 | 0.351 |
| 1x2-w32 | fresh | grouping | 0.591 | -0.108 |
| 1x2-w32 | fresh | inverse | 6.845 | -1.650 |
| 1x2-w32 | fresh | other | 5.618 | 0.085 |
| 1x2-w32 | fresh | scheduler | 4.414 | 1.008 |
| 1x2-w32 | fresh | solver | 1.418 | -0.426 |
| 1x2-w32 | tree | G1_arithmetic | 41.075 | 1.492 |
| 1x2-w32 | tree | G2_arithmetic | 27.935 | 0.620 |
| 1x2-w32 | tree | epoch | 9.518 | 0.371 |
| 1x2-w32 | tree | fft | 4.314 | 0.335 |
| 1x2-w32 | tree | grouping | 0.727 | 0.028 |
| 1x2-w32 | tree | inverse | 6.134 | -2.360 |
| 1x2-w32 | tree | other | 5.161 | -0.372 |
| 1x2-w32 | tree | scheduler | 3.510 | 0.104 |
| 1x2-w32 | tree | solver | 1.626 | -0.218 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.628200 | 37.000 | 4.288× | 24.000 | 2.782× | false |
| 1x2-w32 | fresh | 8.669467 | 37.000 | 4.268× | 24.000 | 2.768× | false |
| 1x2-w32 | tree | 8.539252 | 37.000 | 4.333× | 24.000 | 2.811× | false |
| 1x2-w32 | adaptive | 8.602090 | 37.000 | 4.301× | 24.000 | 2.790× | false |
| 2x2-w32 | accepted | 11.726902 | 59.000 | 5.031× | 30.000 | 2.558× | false |
| 2x2-w32 | fresh | 11.792506 | 59.000 | 5.003× | 30.000 | 2.544× | false |
| 2x2-w32 | tree | 11.825017 | 59.000 | 4.989× | 30.000 | 2.537× | false |
| 2x2-w32 | adaptive | 11.753543 | 59.000 | 5.020× | 30.000 | 2.552× | false |
| 4x2-w32 | accepted | 16.181883 | 109.000 | 6.736× | 48.000 | 2.966× | false |
| 4x2-w32 | fresh | 16.228821 | 109.000 | 6.716× | 48.000 | 2.958× | false |
| 4x2-w32 | tree | 16.219163 | 109.000 | 6.720× | 48.000 | 2.959× | false |
| 4x2-w32 | adaptive | 16.374466 | 109.000 | 6.657× | 48.000 | 2.931× | false |
| 8x2-w32 | accepted | 24.536646 | 189.000 | 7.703× | 74.000 | 3.016× | false |
| 8x2-w32 | fresh | 24.766080 | 189.000 | 7.631× | 74.000 | 2.988× | false |
| 8x2-w32 | tree | 24.155899 | 189.000 | 7.824× | 74.000 | 3.063× | false |
| 8x2-w32 | adaptive | 24.090376 | 189.000 | 7.845× | 74.000 | 3.072× | false |
| 12x2-w32 | accepted | 34.263909 | 284.000 | 8.289× | 101.000 | 2.948× | false |
| 12x2-w32 | fresh | 34.511827 | 284.000 | 8.229× | 101.000 | 2.927× | false |
| 12x2-w32 | tree | 34.296020 | 284.000 | 8.281× | 101.000 | 2.945× | false |
| 12x2-w32 | adaptive | 33.923376 | 284.000 | 8.372× | 101.000 | 2.977× | false |
| 16x2-w32 | accepted | 40.899741 | 335.000 | 8.191× | 124.000 | 3.032× | false |
| 16x2-w32 | fresh | 41.173699 | 335.000 | 8.136× | 124.000 | 3.012× | false |
| 16x2-w32 | tree | 40.370585 | 335.000 | 8.298× | 124.000 | 3.072× | false |
| 16x2-w32 | adaptive | 40.679926 | 335.000 | 8.235× | 124.000 | 3.048× | false |
| 24x2-w32 | accepted | 68.547968 | 528.000 | 7.703× | 176.000 | 2.568× | false |
| 24x2-w32 | fresh | 68.877007 | 528.000 | 7.666× | 176.000 | 2.555× | false |
| 24x2-w32 | tree | 67.421373 | 528.000 | 7.831× | 176.000 | 2.610× | false |
| 24x2-w32 | adaptive | 67.421261 | 528.000 | 7.831× | 176.000 | 2.610× | false |
| merge-24x1-w32 | accepted | 67.272049 | 507.000 | 7.537× | 175.000 | 2.601× | false |
| merge-24x1-w32 | fresh | 67.610169 | 507.000 | 7.499× | 175.000 | 2.588× | false |
| merge-24x1-w32 | tree | 66.892252 | 507.000 | 7.579× | 175.000 | 2.616× | false |
| merge-24x1-w32 | adaptive | 66.314153 | 507.000 | 7.645× | 175.000 | 2.639× | false |
| 1x2-w16 | accepted | 10.491323 | 37.000 | 3.527× | 24.000 | 2.288× | false |
| 1x2-w16 | fresh | 10.597142 | 37.000 | 3.492× | 24.000 | 2.265× | false |
| 1x2-w16 | tree | 10.376707 | 37.000 | 3.566× | 24.000 | 2.313× | false |
| 1x2-w16 | adaptive | 10.453992 | 37.000 | 3.539× | 24.000 | 2.296× | false |

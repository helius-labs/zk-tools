# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.412207 | 1.000000 | 950.398 | 34.387550 | 723.75 |
| 12x2-w32 | off | 34.678349 | 1.009481 | 957.889 | 34.949072 | 719.50 |
| 12x2-w32 | on | 35.347171 | 1.023193 | 951.736 | 35.050452 | 708.75 |
| 16x2-w32 | accepted | 40.845555 | 1.000000 | — | — | — |
| 16x2-w32 | off | 41.163765 | 1.000161 | — | — | — |
| 16x2-w32 | on | 41.432863 | 1.002319 | — | — | — |
| 1x2-w16 | accepted | 10.739513 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.458873 | 0.982238 | — | — | — |
| 1x2-w16 | on | 10.515269 | 0.985665 | — | — | — |
| 1x2-w32 | accepted | 8.644460 | 1.000000 | 199.330 | 8.832872 | 131.00 |
| 1x2-w32 | off | 8.557766 | 0.989971 | 200.071 | 8.757252 | 129.75 |
| 1x2-w32 | on | 8.601689 | 1.000669 | 192.760 | 8.770565 | 128.50 |
| 24x2-w32 | accepted | 68.399372 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.324091 | 0.999447 | — | — | — |
| 24x2-w32 | on | 69.800836 | 1.017285 | — | — | — |
| 2x2-w32 | accepted | 11.688494 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.796527 | 1.007321 | — | — | — |
| 2x2-w32 | on | 11.924308 | 1.021176 | — | — | — |
| 4x2-w32 | accepted | 16.102491 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.227988 | 1.009531 | — | — | — |
| 4x2-w32 | on | 16.621601 | 1.031144 | — | — | — |
| 8x2-w32 | accepted | 25.046416 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.775105 | 0.984517 | — | — | — |
| 8x2-w32 | on | 24.715732 | 1.010002 | — | — | — |
| merge-24x1-w32 | accepted | 67.311198 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 68.276640 | 1.014343 | — | — | — |
| merge-24x1-w32 | on | 70.131049 | 1.046633 | — | — | — |
| merge-54x1-w32 | accepted | 176.331010 | 1.000000 | 4873.120 | 175.660188 | 1616.60 |
| merge-54x1-w32 | off | 180.022115 | 1.019861 | 4897.789 | 177.490877 | 1653.21 |
| merge-54x1-w32 | on | 181.316627 | 1.025179 | 4844.698 | 177.197131 | 1594.89 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.614 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.326 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.234 | 0.000 |
| 12x2-w32 | accepted | fft | 27.615 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.801 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.325 | 0.000 |
| 12x2-w32 | accepted | other | 3.834 | 0.000 |
| 12x2-w32 | accepted | scheduler | 3.799 | 0.000 |
| 12x2-w32 | accepted | solver | 6.452 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 34.646 | 0.032 |
| 12x2-w32 | off | G2_arithmetic | 13.051 | -0.275 |
| 12x2-w32 | off | epoch | 5.313 | 0.079 |
| 12x2-w32 | off | fft | 26.828 | -0.787 |
| 12x2-w32 | off | grouping | 1.888 | 0.087 |
| 12x2-w32 | off | inverse | 3.265 | -0.060 |
| 12x2-w32 | off | other | 4.189 | 0.355 |
| 12x2-w32 | off | scheduler | 4.215 | 0.416 |
| 12x2-w32 | off | solver | 6.605 | 0.153 |
| 12x2-w32 | on | G1_arithmetic | 35.447 | 0.833 |
| 12x2-w32 | on | G2_arithmetic | 13.517 | 0.191 |
| 12x2-w32 | on | epoch | 2.313 | -2.921 |
| 12x2-w32 | on | fft | 27.491 | -0.123 |
| 12x2-w32 | on | grouping | 1.925 | 0.125 |
| 12x2-w32 | on | inverse | 3.385 | 0.061 |
| 12x2-w32 | on | other | 4.491 | 0.657 |
| 12x2-w32 | on | scheduler | 5.473 | 1.673 |
| 12x2-w32 | on | solver | 5.957 | -0.495 |
| 1x2-w32 | accepted | G1_arithmetic | 38.344 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.741 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.398 | 0.000 |
| 1x2-w32 | accepted | fft | 5.082 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.720 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.597 | 0.000 |
| 1x2-w32 | accepted | other | 4.564 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.956 | 0.000 |
| 1x2-w32 | accepted | solver | 1.599 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.386 | -0.957 |
| 1x2-w32 | off | G2_arithmetic | 29.391 | 0.650 |
| 1x2-w32 | off | epoch | 10.008 | 0.610 |
| 1x2-w32 | off | fft | 4.788 | -0.294 |
| 1x2-w32 | off | grouping | 0.556 | -0.165 |
| 1x2-w32 | off | inverse | 7.326 | -0.271 |
| 1x2-w32 | off | other | 5.056 | 0.492 |
| 1x2-w32 | off | scheduler | 4.165 | 0.209 |
| 1x2-w32 | off | solver | 1.324 | -0.275 |
| 1x2-w32 | on | G1_arithmetic | 39.213 | 0.870 |
| 1x2-w32 | on | G2_arithmetic | 30.349 | 1.608 |
| 1x2-w32 | on | epoch | 3.821 | -5.577 |
| 1x2-w32 | on | fft | 5.205 | 0.123 |
| 1x2-w32 | on | grouping | 0.787 | 0.066 |
| 1x2-w32 | on | inverse | 7.954 | 0.358 |
| 1x2-w32 | on | other | 5.496 | 0.932 |
| 1x2-w32 | on | scheduler | 5.749 | 1.792 |
| 1x2-w32 | on | solver | 1.426 | -0.173 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.487 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 14.782 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 1.918 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.004 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.397 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.871 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.813 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.320 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.410 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 26.610 | 0.123 |
| merge-54x1-w32 | off | G2_arithmetic | 14.630 | -0.152 |
| merge-54x1-w32 | off | epoch | 2.180 | 0.262 |
| merge-54x1-w32 | off | fft | 41.543 | -0.461 |
| merge-54x1-w32 | off | grouping | 1.551 | 0.155 |
| merge-54x1-w32 | off | inverse | 1.898 | 0.027 |
| merge-54x1-w32 | off | other | 4.011 | 0.198 |
| merge-54x1-w32 | off | scheduler | 2.102 | -0.218 |
| merge-54x1-w32 | off | solver | 5.475 | 0.065 |
| merge-54x1-w32 | on | G1_arithmetic | 28.052 | 1.565 |
| merge-54x1-w32 | on | G2_arithmetic | 12.951 | -1.830 |
| merge-54x1-w32 | on | epoch | 1.061 | -0.857 |
| merge-54x1-w32 | on | fft | 42.819 | 0.815 |
| merge-54x1-w32 | on | grouping | 1.683 | 0.287 |
| merge-54x1-w32 | on | inverse | 1.853 | -0.018 |
| merge-54x1-w32 | on | other | 4.171 | 0.358 |
| merge-54x1-w32 | on | scheduler | 2.886 | 0.567 |
| merge-54x1-w32 | on | solver | 4.523 | -0.887 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.644460 | 37.000 | 4.280× | 24.000 | 2.776× | false |
| 1x2-w32 | off | 8.557766 | 37.000 | 4.324× | 24.000 | 2.804× | false |
| 1x2-w32 | on | 8.601689 | 37.000 | 4.301× | 24.000 | 2.790× | false |
| 2x2-w32 | accepted | 11.688494 | 59.000 | 5.048× | 30.000 | 2.567× | false |
| 2x2-w32 | off | 11.796527 | 59.000 | 5.001× | 30.000 | 2.543× | false |
| 2x2-w32 | on | 11.924308 | 59.000 | 4.948× | 30.000 | 2.516× | false |
| 4x2-w32 | accepted | 16.102491 | 109.000 | 6.769× | 48.000 | 2.981× | false |
| 4x2-w32 | off | 16.227988 | 109.000 | 6.717× | 48.000 | 2.958× | false |
| 4x2-w32 | on | 16.621601 | 109.000 | 6.558× | 48.000 | 2.888× | false |
| 8x2-w32 | accepted | 25.046416 | 189.000 | 7.546× | 74.000 | 2.955× | false |
| 8x2-w32 | off | 24.775105 | 189.000 | 7.629× | 74.000 | 2.987× | false |
| 8x2-w32 | on | 24.715732 | 189.000 | 7.647× | 74.000 | 2.994× | false |
| 12x2-w32 | accepted | 34.412207 | 284.000 | 8.253× | 101.000 | 2.935× | false |
| 12x2-w32 | off | 34.678349 | 284.000 | 8.190× | 101.000 | 2.912× | false |
| 12x2-w32 | on | 35.347171 | 284.000 | 8.035× | 101.000 | 2.857× | false |
| 16x2-w32 | accepted | 40.845555 | 335.000 | 8.202× | 124.000 | 3.036× | false |
| 16x2-w32 | off | 41.163765 | 335.000 | 8.138× | 124.000 | 3.012× | false |
| 16x2-w32 | on | 41.432863 | 335.000 | 8.085× | 124.000 | 2.993× | false |
| 24x2-w32 | accepted | 68.399372 | 528.000 | 7.719× | 176.000 | 2.573× | false |
| 24x2-w32 | off | 68.324091 | 528.000 | 7.728× | 176.000 | 2.576× | false |
| 24x2-w32 | on | 69.800836 | 528.000 | 7.564× | 176.000 | 2.521× | false |
| merge-24x1-w32 | accepted | 67.311198 | 507.000 | 7.532× | 175.000 | 2.600× | false |
| merge-24x1-w32 | off | 68.276640 | 507.000 | 7.426× | 175.000 | 2.563× | false |
| merge-24x1-w32 | on | 70.131049 | 507.000 | 7.229× | 175.000 | 2.495× | false |

# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.611273 | 1.000000 | 948.203 | 34.277324 | 738.25 |
| 12x2-w32 | off | 34.287845 | 0.995403 | 969.171 | 35.247690 | 723.25 |
| 12x2-w32 | on | 34.310896 | 0.998111 | 969.967 | 34.929606 | 731.25 |
| 16x2-w32 | accepted | 40.898107 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.851396 | 1.001176 | — | — | — |
| 16x2-w32 | on | 40.722918 | 0.998905 | — | — | — |
| 1x2-w16 | accepted | 11.018018 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.626684 | 0.961488 | — | — | — |
| 1x2-w16 | on | 10.545843 | 0.996463 | — | — | — |
| 1x2-w32 | accepted | 8.646072 | 1.000000 | 200.066 | 8.850684 | 129.25 |
| 1x2-w32 | off | 8.638903 | 0.996472 | 198.949 | 8.795941 | 125.00 |
| 1x2-w32 | on | 8.604017 | 0.988177 | 198.018 | 8.636432 | 128.25 |
| 24x2-w32 | accepted | 68.546921 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.387492 | 1.000052 | — | — | — |
| 24x2-w32 | on | 67.392952 | 0.981242 | — | — | — |
| 2x2-w32 | accepted | 11.695492 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.784155 | 1.003475 | — | — | — |
| 2x2-w32 | on | 11.684529 | 0.993532 | — | — | — |
| 4x2-w32 | accepted | 16.197786 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.045586 | 0.998801 | — | — | — |
| 4x2-w32 | on | 16.006727 | 0.997432 | — | — | — |
| 8x2-w32 | accepted | 24.908450 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.794680 | 1.006118 | — | — | — |
| 8x2-w32 | on | 24.750650 | 0.997198 | — | — | — |
| merge-24x1-w32 | accepted | 67.352853 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 67.292361 | 0.995729 | — | — | — |
| merge-24x1-w32 | on | 66.294877 | 0.984292 | — | — | — |
| merge-54x1-w32 | accepted | 177.089653 | 1.000000 | 4898.309 | 177.086673 | 1660.68 |
| merge-54x1-w32 | off | 176.442303 | 0.983578 | 4951.835 | 178.932480 | 1562.79 |
| merge-54x1-w32 | on | 173.395150 | 0.982445 | 4800.422 | 173.869623 | 1630.06 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.958 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.512 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.338 | 0.000 |
| 12x2-w32 | accepted | fft | 27.416 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.721 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.467 | 0.000 |
| 12x2-w32 | accepted | other | 4.031 | 0.000 |
| 12x2-w32 | accepted | scheduler | 3.589 | 0.000 |
| 12x2-w32 | accepted | solver | 5.968 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 33.929 | -1.029 |
| 12x2-w32 | off | G2_arithmetic | 14.189 | 0.677 |
| 12x2-w32 | off | epoch | 5.189 | -0.149 |
| 12x2-w32 | off | fft | 27.287 | -0.130 |
| 12x2-w32 | off | grouping | 1.874 | 0.154 |
| 12x2-w32 | off | inverse | 3.407 | -0.060 |
| 12x2-w32 | off | other | 4.094 | 0.063 |
| 12x2-w32 | off | scheduler | 3.898 | 0.309 |
| 12x2-w32 | off | solver | 6.133 | 0.165 |
| 12x2-w32 | on | G1_arithmetic | 34.290 | -0.669 |
| 12x2-w32 | on | G2_arithmetic | 14.713 | 1.201 |
| 12x2-w32 | on | epoch | 5.302 | -0.036 |
| 12x2-w32 | on | fft | 26.543 | -0.873 |
| 12x2-w32 | on | grouping | 1.848 | 0.128 |
| 12x2-w32 | on | inverse | 3.220 | -0.246 |
| 12x2-w32 | on | other | 4.071 | 0.040 |
| 12x2-w32 | on | scheduler | 3.805 | 0.216 |
| 12x2-w32 | on | solver | 6.207 | 0.239 |
| 1x2-w32 | accepted | G1_arithmetic | 37.208 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 29.800 | 0.000 |
| 1x2-w32 | accepted | epoch | 11.118 | 0.000 |
| 1x2-w32 | accepted | fft | 4.470 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.840 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.748 | 0.000 |
| 1x2-w32 | accepted | other | 4.901 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.414 | 0.000 |
| 1x2-w32 | accepted | solver | 1.501 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.836 | 0.628 |
| 1x2-w32 | off | G2_arithmetic | 29.512 | -0.288 |
| 1x2-w32 | off | epoch | 9.411 | -1.708 |
| 1x2-w32 | off | fft | 4.964 | 0.494 |
| 1x2-w32 | off | grouping | 0.634 | -0.206 |
| 1x2-w32 | off | inverse | 7.429 | 0.681 |
| 1x2-w32 | off | other | 5.327 | 0.426 |
| 1x2-w32 | off | scheduler | 3.522 | 0.109 |
| 1x2-w32 | off | solver | 1.365 | -0.136 |
| 1x2-w32 | on | G1_arithmetic | 37.818 | 0.610 |
| 1x2-w32 | on | G2_arithmetic | 30.583 | 0.783 |
| 1x2-w32 | on | epoch | 8.417 | -2.701 |
| 1x2-w32 | on | fft | 4.822 | 0.353 |
| 1x2-w32 | on | grouping | 0.724 | -0.116 |
| 1x2-w32 | on | inverse | 6.998 | 0.251 |
| 1x2-w32 | on | other | 4.928 | 0.027 |
| 1x2-w32 | on | scheduler | 3.946 | 0.532 |
| 1x2-w32 | on | solver | 1.762 | 0.261 |
| merge-54x1-w32 | accepted | G1_arithmetic | 25.522 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 14.828 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.367 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.514 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.377 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.934 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.966 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.149 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.344 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 25.990 | 0.467 |
| merge-54x1-w32 | off | G2_arithmetic | 15.419 | 0.591 |
| merge-54x1-w32 | off | epoch | 2.211 | -0.156 |
| merge-54x1-w32 | off | fft | 41.725 | -0.788 |
| merge-54x1-w32 | off | grouping | 1.446 | 0.068 |
| merge-54x1-w32 | off | inverse | 1.813 | -0.120 |
| merge-54x1-w32 | off | other | 4.279 | 0.313 |
| merge-54x1-w32 | off | scheduler | 2.175 | 0.026 |
| merge-54x1-w32 | off | solver | 4.942 | -0.402 |
| merge-54x1-w32 | on | G1_arithmetic | 26.792 | 1.269 |
| merge-54x1-w32 | on | G2_arithmetic | 15.265 | 0.437 |
| merge-54x1-w32 | on | epoch | 1.749 | -0.618 |
| merge-54x1-w32 | on | fft | 41.915 | -0.598 |
| merge-54x1-w32 | on | grouping | 1.420 | 0.043 |
| merge-54x1-w32 | on | inverse | 1.969 | 0.035 |
| merge-54x1-w32 | on | other | 3.819 | -0.147 |
| merge-54x1-w32 | on | scheduler | 2.040 | -0.109 |
| merge-54x1-w32 | on | solver | 5.032 | -0.312 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.646072 | 37.000 | 4.279× | 24.000 | 2.776× | false |
| 1x2-w32 | off | 8.638903 | 37.000 | 4.283× | 24.000 | 2.778× | false |
| 1x2-w32 | on | 8.604017 | 37.000 | 4.300× | 24.000 | 2.789× | false |
| 2x2-w32 | accepted | 11.695492 | 59.000 | 5.045× | 30.000 | 2.565× | false |
| 2x2-w32 | off | 11.784155 | 59.000 | 5.007× | 30.000 | 2.546× | false |
| 2x2-w32 | on | 11.684529 | 59.000 | 5.049× | 30.000 | 2.567× | false |
| 4x2-w32 | accepted | 16.197786 | 109.000 | 6.729× | 48.000 | 2.963× | false |
| 4x2-w32 | off | 16.045586 | 109.000 | 6.793× | 48.000 | 2.991× | false |
| 4x2-w32 | on | 16.006727 | 109.000 | 6.810× | 48.000 | 2.999× | false |
| 8x2-w32 | accepted | 24.908450 | 189.000 | 7.588× | 74.000 | 2.971× | false |
| 8x2-w32 | off | 24.794680 | 189.000 | 7.623× | 74.000 | 2.985× | false |
| 8x2-w32 | on | 24.750650 | 189.000 | 7.636× | 74.000 | 2.990× | false |
| 12x2-w32 | accepted | 34.611273 | 284.000 | 8.205× | 101.000 | 2.918× | false |
| 12x2-w32 | off | 34.287845 | 284.000 | 8.283× | 101.000 | 2.946× | false |
| 12x2-w32 | on | 34.310896 | 284.000 | 8.277× | 101.000 | 2.944× | false |
| 16x2-w32 | accepted | 40.898107 | 335.000 | 8.191× | 124.000 | 3.032× | false |
| 16x2-w32 | off | 40.851396 | 335.000 | 8.200× | 124.000 | 3.035× | false |
| 16x2-w32 | on | 40.722918 | 335.000 | 8.226× | 124.000 | 3.045× | false |
| 24x2-w32 | accepted | 68.546921 | 528.000 | 7.703× | 176.000 | 2.568× | false |
| 24x2-w32 | off | 68.387492 | 528.000 | 7.721× | 176.000 | 2.574× | false |
| 24x2-w32 | on | 67.392952 | 528.000 | 7.835× | 176.000 | 2.612× | false |
| merge-24x1-w32 | accepted | 67.352853 | 507.000 | 7.528× | 175.000 | 2.598× | false |
| merge-24x1-w32 | off | 67.292361 | 507.000 | 7.534× | 175.000 | 2.601× | false |
| merge-24x1-w32 | on | 66.294877 | 507.000 | 7.648× | 175.000 | 2.640× | false |

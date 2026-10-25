# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w16 | accepted | 10.518058 | 1.000000 | 129.716 | 11.056956 | 87.25 |
| 1x2-w16 | diagnostic-off | 10.512859 | 0.999506 | 130.282 | 10.684472 | 88.75 |
| 1x2-w16 | diagnostic-on | 10.730359 | 1.031025 | 129.140 | 11.267221 | 89.25 |
| 1x2-w32 | accepted | 8.558022 | 1.000000 | 198.534 | 8.702075 | 130.00 |
| 1x2-w32 | diagnostic-off | 8.461529 | 1.007565 | 198.707 | 8.739436 | 136.25 |
| 1x2-w32 | diagnostic-on | 8.554925 | 1.017901 | 198.788 | 8.752565 | 129.75 |
| merge-54x1-w32 | accepted | 179.161698 | 1.000000 | 4912.305 | 178.944041 | 1718.71 |
| merge-54x1-w32 | diagnostic-off | 176.190173 | 0.990548 | 4895.661 | 178.672084 | 1672.41 |
| merge-54x1-w32 | diagnostic-on | 176.023558 | 0.982484 | 4999.644 | 180.786660 | 1614.50 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w16 | accepted | G1_arithmetic | 43.867 | 0.000 |
| 1x2-w16 | accepted | G2_arithmetic | 31.397 | 0.000 |
| 1x2-w16 | accepted | epoch | 3.370 | 0.000 |
| 1x2-w16 | accepted | fft | 3.318 | 0.000 |
| 1x2-w16 | accepted | grouping | 0.688 | 0.000 |
| 1x2-w16 | accepted | inverse | 8.495 | 0.000 |
| 1x2-w16 | accepted | other | 4.746 | 0.000 |
| 1x2-w16 | accepted | scheduler | 2.214 | 0.000 |
| 1x2-w16 | accepted | solver | 1.904 | 0.000 |
| 1x2-w16 | diagnostic-off | G1_arithmetic | 43.961 | 0.094 |
| 1x2-w16 | diagnostic-off | G2_arithmetic | 32.234 | 0.838 |
| 1x2-w16 | diagnostic-off | epoch | 2.917 | -0.453 |
| 1x2-w16 | diagnostic-off | fft | 3.567 | 0.249 |
| 1x2-w16 | diagnostic-off | grouping | 0.646 | -0.042 |
| 1x2-w16 | diagnostic-off | inverse | 8.239 | -0.256 |
| 1x2-w16 | diagnostic-off | other | 4.742 | -0.004 |
| 1x2-w16 | diagnostic-off | scheduler | 1.854 | -0.360 |
| 1x2-w16 | diagnostic-off | solver | 1.839 | -0.066 |
| 1x2-w16 | diagnostic-on | G1_arithmetic | 43.933 | 0.067 |
| 1x2-w16 | diagnostic-on | G2_arithmetic | 32.506 | 1.109 |
| 1x2-w16 | diagnostic-on | epoch | 2.816 | -0.554 |
| 1x2-w16 | diagnostic-on | fft | 3.564 | 0.246 |
| 1x2-w16 | diagnostic-on | grouping | 0.653 | -0.035 |
| 1x2-w16 | diagnostic-on | inverse | 8.318 | -0.177 |
| 1x2-w16 | diagnostic-on | other | 4.573 | -0.173 |
| 1x2-w16 | diagnostic-on | scheduler | 1.743 | -0.471 |
| 1x2-w16 | diagnostic-on | solver | 1.892 | -0.012 |
| 1x2-w32 | accepted | G1_arithmetic | 38.895 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.766 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.457 | 0.000 |
| 1x2-w32 | accepted | fft | 4.838 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.688 | 0.000 |
| 1x2-w32 | accepted | inverse | 6.947 | 0.000 |
| 1x2-w32 | accepted | other | 5.003 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.742 | 0.000 |
| 1x2-w32 | accepted | solver | 1.664 | 0.000 |
| 1x2-w32 | diagnostic-off | G1_arithmetic | 38.399 | -0.496 |
| 1x2-w32 | diagnostic-off | G2_arithmetic | 28.747 | -0.019 |
| 1x2-w32 | diagnostic-off | epoch | 9.813 | 0.356 |
| 1x2-w32 | diagnostic-off | fft | 4.706 | -0.132 |
| 1x2-w32 | diagnostic-off | grouping | 0.712 | 0.024 |
| 1x2-w32 | diagnostic-off | inverse | 7.111 | 0.164 |
| 1x2-w32 | diagnostic-off | other | 4.992 | -0.011 |
| 1x2-w32 | diagnostic-off | scheduler | 4.091 | 0.349 |
| 1x2-w32 | diagnostic-off | solver | 1.429 | -0.235 |
| 1x2-w32 | diagnostic-on | G1_arithmetic | 38.189 | -0.707 |
| 1x2-w32 | diagnostic-on | G2_arithmetic | 28.669 | -0.097 |
| 1x2-w32 | diagnostic-on | epoch | 10.139 | 0.682 |
| 1x2-w32 | diagnostic-on | fft | 4.737 | -0.101 |
| 1x2-w32 | diagnostic-on | grouping | 0.687 | -0.001 |
| 1x2-w32 | diagnostic-on | inverse | 6.993 | 0.046 |
| 1x2-w32 | diagnostic-on | other | 5.114 | 0.111 |
| 1x2-w32 | diagnostic-on | scheduler | 3.968 | 0.226 |
| 1x2-w32 | diagnostic-on | solver | 1.504 | -0.161 |
| merge-54x1-w32 | accepted | G1_arithmetic | 26.084 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 15.960 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.196 | 0.000 |
| merge-54x1-w32 | accepted | fft | 41.548 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.418 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.827 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.856 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.114 | 0.000 |
| merge-54x1-w32 | accepted | solver | 4.997 | 0.000 |
| merge-54x1-w32 | diagnostic-off | G1_arithmetic | 26.318 | 0.234 |
| merge-54x1-w32 | diagnostic-off | G2_arithmetic | 15.061 | -0.899 |
| merge-54x1-w32 | diagnostic-off | epoch | 2.250 | 0.054 |
| merge-54x1-w32 | diagnostic-off | fft | 41.796 | 0.248 |
| merge-54x1-w32 | diagnostic-off | grouping | 1.450 | 0.032 |
| merge-54x1-w32 | diagnostic-off | inverse | 1.761 | -0.066 |
| merge-54x1-w32 | diagnostic-off | other | 4.107 | 0.250 |
| merge-54x1-w32 | diagnostic-off | scheduler | 2.127 | 0.013 |
| merge-54x1-w32 | diagnostic-off | solver | 5.131 | 0.134 |
| merge-54x1-w32 | diagnostic-on | G1_arithmetic | 25.519 | -0.565 |
| merge-54x1-w32 | diagnostic-on | G2_arithmetic | 15.760 | -0.200 |
| merge-54x1-w32 | diagnostic-on | epoch | 2.201 | 0.005 |
| merge-54x1-w32 | diagnostic-on | fft | 41.874 | 0.326 |
| merge-54x1-w32 | diagnostic-on | grouping | 1.435 | 0.017 |
| merge-54x1-w32 | diagnostic-on | inverse | 1.803 | -0.024 |
| merge-54x1-w32 | diagnostic-on | other | 3.938 | 0.082 |
| merge-54x1-w32 | diagnostic-on | scheduler | 2.228 | 0.113 |
| merge-54x1-w32 | diagnostic-on | solver | 5.243 | 0.246 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.558022 | 37.000 | 4.323× | 24.000 | 2.804× | false |
| 1x2-w32 | diagnostic-off | 8.461529 | 37.000 | 4.373× | 24.000 | 2.836× | false |
| 1x2-w32 | diagnostic-on | 8.554925 | 37.000 | 4.325× | 24.000 | 2.805× | false |

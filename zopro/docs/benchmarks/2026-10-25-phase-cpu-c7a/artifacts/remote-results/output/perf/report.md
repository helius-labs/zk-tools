# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | accepted | 12.632945 | 1.000000 | 292.439 | 13.195020 | 119.54 |
| 1x2-w32 | diagnostic-off | 12.636996 | 1.001109 | 293.128 | 13.285202 | 122.57 |
| 1x2-w32 | diagnostic-on | 12.744928 | 1.007823 | 291.996 | 13.290629 | 121.14 |
| 1x2-w64 | accepted | 12.554244 | 1.000000 | 490.961 | 13.363385 | 121.11 |
| 1x2-w64 | diagnostic-off | 12.572178 | 1.009155 | 487.351 | 13.299439 | 120.92 |
| 1x2-w64 | diagnostic-on | 12.722048 | 1.019245 | 486.760 | 13.161740 | 125.45 |
| 1x2-w96 | accepted | 14.177925 | 1.000000 | 812.163 | 15.068209 | 123.22 |
| 1x2-w96 | diagnostic-off | 14.199295 | 0.995904 | 821.738 | 15.195493 | 126.95 |
| 1x2-w96 | diagnostic-on | 14.280162 | 1.011557 | 831.514 | 15.696326 | 129.20 |
| merge-54x1-w32 | accepted | 139.348213 | 1.000000 | 3620.650 | 144.739281 | 1502.93 |
| merge-54x1-w32 | diagnostic-off | 139.725876 | 0.997836 | 3615.667 | 144.939510 | 1495.78 |
| merge-54x1-w32 | diagnostic-on | 140.290287 | 1.003576 | 3621.520 | 144.830341 | 1527.60 |
| merge-54x1-w64 | accepted | 129.293800 | 1.000000 | 5270.729 | 134.839403 | 1672.39 |
| merge-54x1-w64 | diagnostic-off | 127.676606 | 0.983678 | 5339.654 | 136.062830 | 1647.02 |
| merge-54x1-w64 | diagnostic-on | 128.351826 | 0.996480 | 5257.081 | 133.919039 | 1706.21 |
| merge-54x1-w96 | accepted | 132.123606 | 1.000000 | 7517.401 | 140.657788 | 1902.29 |
| merge-54x1-w96 | diagnostic-off | 131.486124 | 0.992324 | 7658.942 | 144.467022 | 2064.35 |
| merge-54x1-w96 | diagnostic-on | 131.408722 | 1.000388 | 7556.766 | 141.046264 | 1986.90 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.328 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.817 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.593 | 0.000 |
| 1x2-w32 | accepted | fft | 4.352 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.647 | 0.000 |
| 1x2-w32 | accepted | inverse | 5.589 | 0.000 |
| 1x2-w32 | accepted | other | 5.464 | 0.000 |
| 1x2-w32 | accepted | scheduler | 5.407 | 0.000 |
| 1x2-w32 | accepted | solver | 1.803 | 0.000 |
| 1x2-w32 | diagnostic-off | G1_arithmetic | 38.069 | -0.259 |
| 1x2-w32 | diagnostic-off | G2_arithmetic | 28.854 | 0.037 |
| 1x2-w32 | diagnostic-off | epoch | 9.629 | 0.035 |
| 1x2-w32 | diagnostic-off | fft | 4.409 | 0.057 |
| 1x2-w32 | diagnostic-off | grouping | 0.609 | -0.038 |
| 1x2-w32 | diagnostic-off | inverse | 5.464 | -0.125 |
| 1x2-w32 | diagnostic-off | other | 5.650 | 0.186 |
| 1x2-w32 | diagnostic-off | scheduler | 5.497 | 0.090 |
| 1x2-w32 | diagnostic-off | solver | 1.821 | 0.018 |
| 1x2-w32 | diagnostic-on | G1_arithmetic | 38.046 | -0.282 |
| 1x2-w32 | diagnostic-on | G2_arithmetic | 29.041 | 0.224 |
| 1x2-w32 | diagnostic-on | epoch | 9.831 | 0.237 |
| 1x2-w32 | diagnostic-on | fft | 4.257 | -0.094 |
| 1x2-w32 | diagnostic-on | grouping | 0.641 | -0.006 |
| 1x2-w32 | diagnostic-on | inverse | 5.603 | 0.014 |
| 1x2-w32 | diagnostic-on | other | 5.264 | -0.200 |
| 1x2-w32 | diagnostic-on | scheduler | 5.492 | 0.085 |
| 1x2-w32 | diagnostic-on | solver | 1.825 | 0.022 |
| 1x2-w64 | accepted | G1_arithmetic | 25.212 | 0.000 |
| 1x2-w64 | accepted | G2_arithmetic | 19.283 | 0.000 |
| 1x2-w64 | accepted | epoch | 30.857 | 0.000 |
| 1x2-w64 | accepted | fft | 3.718 | 0.000 |
| 1x2-w64 | accepted | grouping | 0.467 | 0.000 |
| 1x2-w64 | accepted | inverse | 3.421 | 0.000 |
| 1x2-w64 | accepted | other | 5.314 | 0.000 |
| 1x2-w64 | accepted | scheduler | 10.435 | 0.000 |
| 1x2-w64 | accepted | solver | 1.294 | 0.000 |
| 1x2-w64 | diagnostic-off | G1_arithmetic | 25.063 | -0.149 |
| 1x2-w64 | diagnostic-off | G2_arithmetic | 19.327 | 0.045 |
| 1x2-w64 | diagnostic-off | epoch | 31.085 | 0.228 |
| 1x2-w64 | diagnostic-off | fft | 3.711 | -0.007 |
| 1x2-w64 | diagnostic-off | grouping | 0.470 | 0.003 |
| 1x2-w64 | diagnostic-off | inverse | 3.460 | 0.039 |
| 1x2-w64 | diagnostic-off | other | 5.185 | -0.129 |
| 1x2-w64 | diagnostic-off | scheduler | 10.388 | -0.047 |
| 1x2-w64 | diagnostic-off | solver | 1.311 | 0.017 |
| 1x2-w64 | diagnostic-on | G1_arithmetic | 25.105 | -0.107 |
| 1x2-w64 | diagnostic-on | G2_arithmetic | 19.342 | 0.060 |
| 1x2-w64 | diagnostic-on | epoch | 31.025 | 0.167 |
| 1x2-w64 | diagnostic-on | fft | 3.673 | -0.045 |
| 1x2-w64 | diagnostic-on | grouping | 0.450 | -0.017 |
| 1x2-w64 | diagnostic-on | inverse | 3.476 | 0.055 |
| 1x2-w64 | diagnostic-on | other | 5.262 | -0.052 |
| 1x2-w64 | diagnostic-on | scheduler | 10.346 | -0.088 |
| 1x2-w64 | diagnostic-on | solver | 1.321 | 0.027 |
| 1x2-w96 | accepted | G1_arithmetic | 15.682 | 0.000 |
| 1x2-w96 | accepted | G2_arithmetic | 12.738 | 0.000 |
| 1x2-w96 | accepted | epoch | 48.525 | 0.000 |
| 1x2-w96 | accepted | fft | 2.588 | 0.000 |
| 1x2-w96 | accepted | grouping | 0.274 | 0.000 |
| 1x2-w96 | accepted | inverse | 2.109 | 0.000 |
| 1x2-w96 | accepted | other | 4.441 | 0.000 |
| 1x2-w96 | accepted | scheduler | 12.814 | 0.000 |
| 1x2-w96 | accepted | solver | 0.829 | 0.000 |
| 1x2-w96 | diagnostic-off | G1_arithmetic | 15.664 | -0.018 |
| 1x2-w96 | diagnostic-off | G2_arithmetic | 12.756 | 0.018 |
| 1x2-w96 | diagnostic-off | epoch | 48.442 | -0.083 |
| 1x2-w96 | diagnostic-off | fft | 2.616 | 0.028 |
| 1x2-w96 | diagnostic-off | grouping | 0.283 | 0.009 |
| 1x2-w96 | diagnostic-off | inverse | 2.115 | 0.006 |
| 1x2-w96 | diagnostic-off | other | 4.413 | -0.028 |
| 1x2-w96 | diagnostic-off | scheduler | 12.882 | 0.068 |
| 1x2-w96 | diagnostic-off | solver | 0.828 | -0.001 |
| 1x2-w96 | diagnostic-on | G1_arithmetic | 15.414 | -0.269 |
| 1x2-w96 | diagnostic-on | G2_arithmetic | 12.571 | -0.166 |
| 1x2-w96 | diagnostic-on | epoch | 48.822 | 0.297 |
| 1x2-w96 | diagnostic-on | fft | 2.596 | 0.008 |
| 1x2-w96 | diagnostic-on | grouping | 0.297 | 0.022 |
| 1x2-w96 | diagnostic-on | inverse | 2.096 | -0.012 |
| 1x2-w96 | diagnostic-on | other | 4.352 | -0.090 |
| 1x2-w96 | diagnostic-on | scheduler | 13.004 | 0.191 |
| 1x2-w96 | diagnostic-on | solver | 0.848 | 0.019 |
| merge-54x1-w32 | accepted | G1_arithmetic | 44.623 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 11.722 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 4.207 | 0.000 |
| merge-54x1-w32 | accepted | fft | 23.798 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.619 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 2.775 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.199 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 1.932 | 0.000 |
| merge-54x1-w32 | accepted | solver | 6.124 | 0.000 |
| merge-54x1-w32 | diagnostic-off | G1_arithmetic | 44.499 | -0.124 |
| merge-54x1-w32 | diagnostic-off | G2_arithmetic | 11.711 | -0.011 |
| merge-54x1-w32 | diagnostic-off | epoch | 4.419 | 0.213 |
| merge-54x1-w32 | diagnostic-off | fft | 23.753 | -0.045 |
| merge-54x1-w32 | diagnostic-off | grouping | 1.592 | -0.028 |
| merge-54x1-w32 | diagnostic-off | inverse | 2.858 | 0.083 |
| merge-54x1-w32 | diagnostic-off | other | 3.159 | -0.040 |
| merge-54x1-w32 | diagnostic-off | scheduler | 1.942 | 0.010 |
| merge-54x1-w32 | diagnostic-off | solver | 6.067 | -0.057 |
| merge-54x1-w32 | diagnostic-on | G1_arithmetic | 44.560 | -0.063 |
| merge-54x1-w32 | diagnostic-on | G2_arithmetic | 11.783 | 0.061 |
| merge-54x1-w32 | diagnostic-on | epoch | 4.215 | 0.008 |
| merge-54x1-w32 | diagnostic-on | fft | 23.904 | 0.105 |
| merge-54x1-w32 | diagnostic-on | grouping | 1.607 | -0.012 |
| merge-54x1-w32 | diagnostic-on | inverse | 2.723 | -0.052 |
| merge-54x1-w32 | diagnostic-on | other | 3.137 | -0.063 |
| merge-54x1-w32 | diagnostic-on | scheduler | 1.910 | -0.023 |
| merge-54x1-w32 | diagnostic-on | solver | 6.163 | 0.038 |
| merge-54x1-w64 | accepted | G1_arithmetic | 35.193 | 0.000 |
| merge-54x1-w64 | accepted | G2_arithmetic | 9.609 | 0.000 |
| merge-54x1-w64 | accepted | epoch | 14.701 | 0.000 |
| merge-54x1-w64 | accepted | fft | 22.698 | 0.000 |
| merge-54x1-w64 | accepted | grouping | 1.265 | 0.000 |
| merge-54x1-w64 | accepted | inverse | 2.153 | 0.000 |
| merge-54x1-w64 | accepted | other | 4.051 | 0.000 |
| merge-54x1-w64 | accepted | scheduler | 5.153 | 0.000 |
| merge-54x1-w64 | accepted | solver | 5.177 | 0.000 |
| merge-54x1-w64 | diagnostic-off | G1_arithmetic | 35.071 | -0.123 |
| merge-54x1-w64 | diagnostic-off | G2_arithmetic | 9.794 | 0.186 |
| merge-54x1-w64 | diagnostic-off | epoch | 14.486 | -0.215 |
| merge-54x1-w64 | diagnostic-off | fft | 23.052 | 0.354 |
| merge-54x1-w64 | diagnostic-off | grouping | 1.346 | 0.082 |
| merge-54x1-w64 | diagnostic-off | inverse | 2.183 | 0.030 |
| merge-54x1-w64 | diagnostic-off | other | 4.017 | -0.034 |
| merge-54x1-w64 | diagnostic-off | scheduler | 4.944 | -0.209 |
| merge-54x1-w64 | diagnostic-off | solver | 5.107 | -0.070 |
| merge-54x1-w64 | diagnostic-on | G1_arithmetic | 35.417 | 0.223 |
| merge-54x1-w64 | diagnostic-on | G2_arithmetic | 9.743 | 0.135 |
| merge-54x1-w64 | diagnostic-on | epoch | 14.389 | -0.312 |
| merge-54x1-w64 | diagnostic-on | fft | 22.661 | -0.037 |
| merge-54x1-w64 | diagnostic-on | grouping | 1.324 | 0.059 |
| merge-54x1-w64 | diagnostic-on | inverse | 2.167 | 0.015 |
| merge-54x1-w64 | diagnostic-on | other | 3.945 | -0.106 |
| merge-54x1-w64 | diagnostic-on | scheduler | 5.117 | -0.036 |
| merge-54x1-w64 | diagnostic-on | solver | 5.237 | 0.060 |
| merge-54x1-w96 | accepted | G1_arithmetic | 27.678 | 0.000 |
| merge-54x1-w96 | accepted | G2_arithmetic | 8.408 | 0.000 |
| merge-54x1-w96 | accepted | epoch | 25.200 | 0.000 |
| merge-54x1-w96 | accepted | fft | 20.671 | 0.000 |
| merge-54x1-w96 | accepted | grouping | 0.996 | 0.000 |
| merge-54x1-w96 | accepted | inverse | 1.635 | 0.000 |
| merge-54x1-w96 | accepted | other | 4.164 | 0.000 |
| merge-54x1-w96 | accepted | scheduler | 7.456 | 0.000 |
| merge-54x1-w96 | accepted | solver | 3.791 | 0.000 |
| merge-54x1-w96 | diagnostic-off | G1_arithmetic | 27.099 | -0.579 |
| merge-54x1-w96 | diagnostic-off | G2_arithmetic | 8.126 | -0.282 |
| merge-54x1-w96 | diagnostic-off | epoch | 26.326 | 1.126 |
| merge-54x1-w96 | diagnostic-off | fft | 19.903 | -0.768 |
| merge-54x1-w96 | diagnostic-off | grouping | 0.993 | -0.003 |
| merge-54x1-w96 | diagnostic-off | inverse | 1.607 | -0.029 |
| merge-54x1-w96 | diagnostic-off | other | 3.964 | -0.200 |
| merge-54x1-w96 | diagnostic-off | scheduler | 8.082 | 0.626 |
| merge-54x1-w96 | diagnostic-off | solver | 3.899 | 0.108 |
| merge-54x1-w96 | diagnostic-on | G1_arithmetic | 27.895 | 0.216 |
| merge-54x1-w96 | diagnostic-on | G2_arithmetic | 8.289 | -0.118 |
| merge-54x1-w96 | diagnostic-on | epoch | 25.088 | -0.113 |
| merge-54x1-w96 | diagnostic-on | fft | 20.608 | -0.063 |
| merge-54x1-w96 | diagnostic-on | grouping | 0.982 | -0.014 |
| merge-54x1-w96 | diagnostic-on | inverse | 1.587 | -0.049 |
| merge-54x1-w96 | diagnostic-on | other | 4.182 | 0.018 |
| merge-54x1-w96 | diagnostic-on | scheduler | 7.526 | 0.070 |
| merge-54x1-w96 | diagnostic-on | solver | 3.844 | 0.052 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 12.632945 | 37.000 | 2.929× | 24.000 | 1.900× | false |
| 1x2-w32 | diagnostic-off | 12.636996 | 37.000 | 2.928× | 24.000 | 1.899× | false |
| 1x2-w32 | diagnostic-on | 12.744928 | 37.000 | 2.903× | 24.000 | 1.883× | false |
| 1x2-w64 | accepted | 12.554244 | 37.000 | 2.947× | 24.000 | 1.912× | false |
| 1x2-w64 | diagnostic-off | 12.572178 | 37.000 | 2.943× | 24.000 | 1.909× | false |
| 1x2-w64 | diagnostic-on | 12.722048 | 37.000 | 2.908× | 24.000 | 1.886× | false |
| 1x2-w96 | accepted | 14.177925 | 37.000 | 2.610× | 24.000 | 1.693× | false |
| 1x2-w96 | diagnostic-off | 14.199295 | 37.000 | 2.606× | 24.000 | 1.690× | false |
| 1x2-w96 | diagnostic-on | 14.280162 | 37.000 | 2.591× | 24.000 | 1.681× | false |

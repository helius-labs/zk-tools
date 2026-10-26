# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32 | accepted | 12.664744 | 1.000000 | 292.883 | 13.259355 | 120.94 |
| 1x2-w32 | off | 12.769673 | 1.009110 | 292.842 | 13.331848 | 120.30 |
| 1x2-w32 | on | 12.748023 | 1.004521 | 300.720 | 13.223932 | 123.95 |
| 1x2-w64 | accepted | 12.751725 | 1.000000 | 490.887 | 13.402516 | 119.50 |
| 1x2-w64 | off | 12.807365 | 1.003213 | 489.469 | 13.175608 | 119.43 |
| 1x2-w64 | on | 12.518493 | 0.977372 | 507.826 | 12.910211 | 114.41 |
| 1x2-w96 | accepted | 14.300623 | 1.000000 | 815.444 | 15.209578 | 124.04 |
| 1x2-w96 | off | 14.497226 | 1.008057 | 813.768 | 15.027861 | 114.98 |
| 1x2-w96 | on | 14.001573 | 0.979995 | 853.550 | 15.074971 | 146.57 |
| merge-54x1-w32 | accepted | 140.952759 | 1.000000 | 3661.897 | 145.737439 | 1440.85 |
| merge-54x1-w32 | off | 140.384480 | 0.992032 | 3653.663 | 145.536545 | 1485.15 |
| merge-54x1-w32 | on | 139.366489 | 0.988746 | 3646.458 | 144.269398 | 1453.33 |
| merge-54x1-w64 | accepted | 133.196539 | 1.000000 | 5308.230 | 135.532380 | 1695.69 |
| merge-54x1-w64 | off | 127.968586 | 0.990080 | 5262.821 | 134.882784 | 1677.88 |
| merge-54x1-w64 | on | 127.297767 | 0.949676 | 5261.257 | 132.417402 | 1629.34 |
| merge-54x1-w96 | accepted | 131.813642 | 1.000000 | 7559.375 | 142.254680 | 1915.31 |
| merge-54x1-w96 | off | 131.928022 | 0.995553 | 7616.019 | 142.394965 | 1913.20 |
| merge-54x1-w96 | on | 129.318931 | 0.981074 | 7420.372 | 135.292193 | 2138.91 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 38.411 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.858 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.372 | 0.000 |
| 1x2-w32 | accepted | fft | 4.239 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.666 | 0.000 |
| 1x2-w32 | accepted | inverse | 5.574 | 0.000 |
| 1x2-w32 | accepted | other | 5.526 | 0.000 |
| 1x2-w32 | accepted | scheduler | 5.559 | 0.000 |
| 1x2-w32 | accepted | solver | 1.794 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 38.359 | -0.052 |
| 1x2-w32 | off | G2_arithmetic | 29.165 | 0.306 |
| 1x2-w32 | off | epoch | 9.739 | 0.367 |
| 1x2-w32 | off | fft | 4.296 | 0.057 |
| 1x2-w32 | off | grouping | 0.622 | -0.044 |
| 1x2-w32 | off | inverse | 5.409 | -0.166 |
| 1x2-w32 | off | other | 5.319 | -0.207 |
| 1x2-w32 | off | scheduler | 5.285 | -0.274 |
| 1x2-w32 | off | solver | 1.807 | 0.013 |
| 1x2-w32 | on | G1_arithmetic | 36.793 | -1.618 |
| 1x2-w32 | on | G2_arithmetic | 28.193 | -0.665 |
| 1x2-w32 | on | epoch | 5.533 | -3.839 |
| 1x2-w32 | on | fft | 4.262 | 0.023 |
| 1x2-w32 | on | grouping | 0.641 | -0.026 |
| 1x2-w32 | on | inverse | 5.341 | -0.234 |
| 1x2-w32 | on | other | 8.885 | 3.360 |
| 1x2-w32 | on | scheduler | 8.615 | 3.056 |
| 1x2-w32 | on | solver | 1.738 | -0.056 |
| 1x2-w64 | accepted | G1_arithmetic | 24.753 | 0.000 |
| 1x2-w64 | accepted | G2_arithmetic | 19.113 | 0.000 |
| 1x2-w64 | accepted | epoch | 31.027 | 0.000 |
| 1x2-w64 | accepted | fft | 3.735 | 0.000 |
| 1x2-w64 | accepted | grouping | 0.465 | 0.000 |
| 1x2-w64 | accepted | inverse | 3.496 | 0.000 |
| 1x2-w64 | accepted | other | 5.357 | 0.000 |
| 1x2-w64 | accepted | scheduler | 10.693 | 0.000 |
| 1x2-w64 | accepted | solver | 1.360 | 0.000 |
| 1x2-w64 | off | G1_arithmetic | 25.155 | 0.402 |
| 1x2-w64 | off | G2_arithmetic | 19.355 | 0.242 |
| 1x2-w64 | off | epoch | 31.293 | 0.266 |
| 1x2-w64 | off | fft | 3.873 | 0.139 |
| 1x2-w64 | off | grouping | 0.450 | -0.015 |
| 1x2-w64 | off | inverse | 3.408 | -0.088 |
| 1x2-w64 | off | other | 5.313 | -0.044 |
| 1x2-w64 | off | scheduler | 9.830 | -0.863 |
| 1x2-w64 | off | solver | 1.322 | -0.038 |
| 1x2-w64 | on | G1_arithmetic | 24.172 | -0.581 |
| 1x2-w64 | on | G2_arithmetic | 18.695 | -0.418 |
| 1x2-w64 | on | epoch | 18.944 | -12.083 |
| 1x2-w64 | on | fft | 3.428 | -0.306 |
| 1x2-w64 | on | grouping | 0.407 | -0.058 |
| 1x2-w64 | on | inverse | 3.200 | -0.296 |
| 1x2-w64 | on | other | 11.979 | 6.622 |
| 1x2-w64 | on | scheduler | 17.994 | 7.300 |
| 1x2-w64 | on | solver | 1.180 | -0.180 |
| 1x2-w96 | accepted | G1_arithmetic | 15.684 | 0.000 |
| 1x2-w96 | accepted | G2_arithmetic | 12.817 | 0.000 |
| 1x2-w96 | accepted | epoch | 48.317 | 0.000 |
| 1x2-w96 | accepted | fft | 2.665 | 0.000 |
| 1x2-w96 | accepted | grouping | 0.276 | 0.000 |
| 1x2-w96 | accepted | inverse | 2.131 | 0.000 |
| 1x2-w96 | accepted | other | 4.302 | 0.000 |
| 1x2-w96 | accepted | scheduler | 12.948 | 0.000 |
| 1x2-w96 | accepted | solver | 0.860 | 0.000 |
| 1x2-w96 | off | G1_arithmetic | 15.617 | -0.067 |
| 1x2-w96 | off | G2_arithmetic | 12.729 | -0.087 |
| 1x2-w96 | off | epoch | 48.666 | 0.348 |
| 1x2-w96 | off | fft | 2.638 | -0.026 |
| 1x2-w96 | off | grouping | 0.263 | -0.013 |
| 1x2-w96 | off | inverse | 2.229 | 0.098 |
| 1x2-w96 | off | other | 4.634 | 0.332 |
| 1x2-w96 | off | scheduler | 12.370 | -0.578 |
| 1x2-w96 | off | solver | 0.853 | -0.008 |
| 1x2-w96 | on | G1_arithmetic | 15.399 | -0.285 |
| 1x2-w96 | on | G2_arithmetic | 12.461 | -0.355 |
| 1x2-w96 | on | epoch | 28.440 | -19.877 |
| 1x2-w96 | on | fft | 2.396 | -0.269 |
| 1x2-w96 | on | grouping | 0.283 | 0.007 |
| 1x2-w96 | on | inverse | 2.010 | -0.121 |
| 1x2-w96 | on | other | 14.422 | 10.120 |
| 1x2-w96 | on | scheduler | 23.745 | 10.797 |
| 1x2-w96 | on | solver | 0.844 | -0.017 |
| merge-54x1-w32 | accepted | G1_arithmetic | 44.453 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 11.588 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 4.237 | 0.000 |
| merge-54x1-w32 | accepted | fft | 24.144 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.641 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 2.802 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.042 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 1.955 | 0.000 |
| merge-54x1-w32 | accepted | solver | 6.138 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 44.322 | -0.131 |
| merge-54x1-w32 | off | G2_arithmetic | 11.578 | -0.010 |
| merge-54x1-w32 | off | epoch | 4.424 | 0.187 |
| merge-54x1-w32 | off | fft | 24.090 | -0.054 |
| merge-54x1-w32 | off | grouping | 1.581 | -0.059 |
| merge-54x1-w32 | off | inverse | 2.831 | 0.029 |
| merge-54x1-w32 | off | other | 3.234 | 0.193 |
| merge-54x1-w32 | off | scheduler | 1.914 | -0.042 |
| merge-54x1-w32 | off | solver | 6.025 | -0.113 |
| merge-54x1-w32 | on | G1_arithmetic | 44.177 | -0.277 |
| merge-54x1-w32 | on | G2_arithmetic | 11.789 | 0.202 |
| merge-54x1-w32 | on | epoch | 2.185 | -2.053 |
| merge-54x1-w32 | on | fft | 23.558 | -0.586 |
| merge-54x1-w32 | on | grouping | 1.577 | -0.064 |
| merge-54x1-w32 | on | inverse | 2.827 | 0.025 |
| merge-54x1-w32 | on | other | 4.572 | 1.531 |
| merge-54x1-w32 | on | scheduler | 3.275 | 1.320 |
| merge-54x1-w32 | on | solver | 6.041 | -0.098 |
| merge-54x1-w64 | accepted | G1_arithmetic | 35.392 | 0.000 |
| merge-54x1-w64 | accepted | G2_arithmetic | 9.752 | 0.000 |
| merge-54x1-w64 | accepted | epoch | 14.061 | 0.000 |
| merge-54x1-w64 | accepted | fft | 22.860 | 0.000 |
| merge-54x1-w64 | accepted | grouping | 1.277 | 0.000 |
| merge-54x1-w64 | accepted | inverse | 2.051 | 0.000 |
| merge-54x1-w64 | accepted | other | 4.085 | 0.000 |
| merge-54x1-w64 | accepted | scheduler | 5.148 | 0.000 |
| merge-54x1-w64 | accepted | solver | 5.374 | 0.000 |
| merge-54x1-w64 | off | G1_arithmetic | 35.267 | -0.125 |
| merge-54x1-w64 | off | G2_arithmetic | 9.696 | -0.057 |
| merge-54x1-w64 | off | epoch | 14.810 | 0.749 |
| merge-54x1-w64 | off | fft | 22.555 | -0.305 |
| merge-54x1-w64 | off | grouping | 1.204 | -0.073 |
| merge-54x1-w64 | off | inverse | 2.194 | 0.143 |
| merge-54x1-w64 | off | other | 4.139 | 0.054 |
| merge-54x1-w64 | off | scheduler | 4.946 | -0.202 |
| merge-54x1-w64 | off | solver | 5.190 | -0.184 |
| merge-54x1-w64 | on | G1_arithmetic | 35.439 | 0.048 |
| merge-54x1-w64 | on | G2_arithmetic | 9.603 | -0.149 |
| merge-54x1-w64 | on | epoch | 8.417 | -5.644 |
| merge-54x1-w64 | on | fft | 22.011 | -0.849 |
| merge-54x1-w64 | on | grouping | 1.268 | -0.009 |
| merge-54x1-w64 | on | inverse | 2.244 | 0.193 |
| merge-54x1-w64 | on | other | 7.127 | 3.042 |
| merge-54x1-w64 | on | scheduler | 8.809 | 3.662 |
| merge-54x1-w64 | on | solver | 5.081 | -0.293 |
| merge-54x1-w96 | accepted | G1_arithmetic | 27.817 | 0.000 |
| merge-54x1-w96 | accepted | G2_arithmetic | 8.562 | 0.000 |
| merge-54x1-w96 | accepted | epoch | 25.200 | 0.000 |
| merge-54x1-w96 | accepted | fft | 20.334 | 0.000 |
| merge-54x1-w96 | accepted | grouping | 0.968 | 0.000 |
| merge-54x1-w96 | accepted | inverse | 1.658 | 0.000 |
| merge-54x1-w96 | accepted | other | 4.160 | 0.000 |
| merge-54x1-w96 | accepted | scheduler | 7.289 | 0.000 |
| merge-54x1-w96 | accepted | solver | 4.011 | 0.000 |
| merge-54x1-w96 | off | G1_arithmetic | 27.554 | -0.263 |
| merge-54x1-w96 | off | G2_arithmetic | 8.414 | -0.147 |
| merge-54x1-w96 | off | epoch | 25.435 | 0.234 |
| merge-54x1-w96 | off | fft | 20.618 | 0.284 |
| merge-54x1-w96 | off | grouping | 1.003 | 0.035 |
| merge-54x1-w96 | off | inverse | 1.564 | -0.094 |
| merge-54x1-w96 | off | other | 4.227 | 0.067 |
| merge-54x1-w96 | off | scheduler | 7.115 | -0.174 |
| merge-54x1-w96 | off | solver | 4.069 | 0.058 |
| merge-54x1-w96 | on | G1_arithmetic | 28.300 | 0.483 |
| merge-54x1-w96 | on | G2_arithmetic | 8.658 | 0.097 |
| merge-54x1-w96 | on | epoch | 13.429 | -11.771 |
| merge-54x1-w96 | on | fft | 20.475 | 0.141 |
| merge-54x1-w96 | on | grouping | 1.025 | 0.057 |
| merge-54x1-w96 | on | inverse | 1.628 | -0.030 |
| merge-54x1-w96 | on | other | 9.422 | 5.262 |
| merge-54x1-w96 | on | scheduler | 13.008 | 5.718 |
| merge-54x1-w96 | on | solver | 4.054 | 0.043 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 12.664744 | 37.000 | 2.921× | 24.000 | 1.895× | false |
| 1x2-w32 | off | 12.769673 | 37.000 | 2.897× | 24.000 | 1.879× | false |
| 1x2-w32 | on | 12.748023 | 37.000 | 2.902× | 24.000 | 1.883× | false |
| 1x2-w64 | accepted | 12.751725 | 37.000 | 2.902× | 24.000 | 1.882× | false |
| 1x2-w64 | off | 12.807365 | 37.000 | 2.889× | 24.000 | 1.874× | false |
| 1x2-w64 | on | 12.518493 | 37.000 | 2.956× | 24.000 | 1.917× | false |
| 1x2-w96 | accepted | 14.300623 | 37.000 | 2.587× | 24.000 | 1.678× | false |
| 1x2-w96 | off | 14.497226 | 37.000 | 2.552× | 24.000 | 1.655× | false |
| 1x2-w96 | on | 14.001573 | 37.000 | 2.643× | 24.000 | 1.714× | false |

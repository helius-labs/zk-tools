# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.108489 | 1.000000 | — | — | — |
| 12x2-w32 | candidate | 34.020013 | 0.996469 | — | — | — |
| 12x2-w32 | fresh | 33.882784 | 0.999866 | — | — | — |
| 16x2-w32 | accepted | 40.041681 | 1.000000 | — | — | — |
| 16x2-w32 | candidate | 40.102747 | 0.999819 | — | — | — |
| 16x2-w32 | fresh | 40.100563 | 1.001669 | — | — | — |
| 1x2-w16 | accepted | 10.303266 | 1.000000 | — | — | — |
| 1x2-w16 | candidate | 10.317271 | 0.984324 | — | — | — |
| 1x2-w16 | fresh | 10.122895 | 0.994572 | — | — | — |
| 1x2-w32 | accepted | 8.379486 | 1.000000 | 191.315 | 8.566918 | 135.75 |
| 1x2-w32 | candidate | 8.468905 | 1.007710 | 192.445 | 8.579738 | 130.25 |
| 1x2-w32 | fresh | 8.340873 | 1.006615 | 191.984 | 8.475954 | 134.25 |
| 24x2-w32 | accepted | 67.244175 | 1.000000 | — | — | — |
| 24x2-w32 | candidate | 67.245707 | 0.996920 | — | — | — |
| 24x2-w32 | fresh | 67.283719 | 1.000588 | — | — | — |
| 2x2-w32 | accepted | 11.514329 | 1.000000 | — | — | — |
| 2x2-w32 | candidate | 11.492008 | 1.001281 | — | — | — |
| 2x2-w32 | fresh | 11.465648 | 0.993996 | — | — | — |
| 4x2-w32 | accepted | 16.080934 | 1.000000 | 416.042 | 16.096073 | 374.25 |
| 4x2-w32 | candidate | 15.928726 | 0.991726 | 425.976 | 16.526945 | 372.50 |
| 4x2-w32 | fresh | 15.965606 | 0.993513 | 428.937 | 16.451203 | 374.50 |
| 8x2-w32 | accepted | 24.028846 | 1.000000 | — | — | — |
| 8x2-w32 | candidate | 24.369048 | 1.018775 | — | — | — |
| 8x2-w32 | fresh | 24.179791 | 1.003395 | — | — | — |
| merge-24x1-w32 | accepted | 66.183164 | 1.000000 | — | — | — |
| merge-24x1-w32 | candidate | 66.155943 | 0.999219 | — | — | — |
| merge-24x1-w32 | fresh | 65.766548 | 0.997034 | — | — | — |
| merge-54x1-w32 | accepted | 164.598097 | 1.000000 | — | — | — |
| merge-54x1-w32 | candidate | 164.192531 | 1.001591 | — | — | — |
| merge-54x1-w32 | fresh | 162.975977 | 0.995431 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 35.571 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 31.486 | 0.000 |
| 1x2-w32 | accepted | epoch | 8.615 | 0.000 |
| 1x2-w32 | accepted | fft | 4.832 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.926 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.164 | 0.000 |
| 1x2-w32 | accepted | other | 5.671 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.881 | 0.000 |
| 1x2-w32 | accepted | solver | 1.854 | 0.000 |
| 1x2-w32 | candidate | G1_arithmetic | 37.432 | 1.861 |
| 1x2-w32 | candidate | G2_arithmetic | 29.168 | -2.318 |
| 1x2-w32 | candidate | epoch | 10.105 | 1.490 |
| 1x2-w32 | candidate | fft | 5.394 | 0.562 |
| 1x2-w32 | candidate | grouping | 0.872 | -0.054 |
| 1x2-w32 | candidate | inverse | 6.629 | -0.536 |
| 1x2-w32 | candidate | other | 5.264 | -0.407 |
| 1x2-w32 | candidate | scheduler | 3.370 | -0.511 |
| 1x2-w32 | candidate | solver | 1.766 | -0.088 |
| 1x2-w32 | fresh | G1_arithmetic | 36.771 | 1.200 |
| 1x2-w32 | fresh | G2_arithmetic | 30.771 | -0.715 |
| 1x2-w32 | fresh | epoch | 8.745 | 0.130 |
| 1x2-w32 | fresh | fft | 5.250 | 0.418 |
| 1x2-w32 | fresh | grouping | 0.773 | -0.153 |
| 1x2-w32 | fresh | inverse | 6.896 | -0.268 |
| 1x2-w32 | fresh | other | 5.610 | -0.061 |
| 1x2-w32 | fresh | scheduler | 3.980 | 0.099 |
| 1x2-w32 | fresh | solver | 1.205 | -0.649 |
| 4x2-w32 | accepted | G1_arithmetic | 34.164 | 0.000 |
| 4x2-w32 | accepted | G2_arithmetic | 19.156 | 0.000 |
| 4x2-w32 | accepted | epoch | 10.190 | 0.000 |
| 4x2-w32 | accepted | fft | 15.401 | 0.000 |
| 4x2-w32 | accepted | grouping | 1.264 | 0.000 |
| 4x2-w32 | accepted | inverse | 4.535 | 0.000 |
| 4x2-w32 | accepted | other | 6.704 | 0.000 |
| 4x2-w32 | accepted | scheduler | 4.722 | 0.000 |
| 4x2-w32 | accepted | solver | 3.863 | 0.000 |
| 4x2-w32 | candidate | G1_arithmetic | 33.389 | -0.775 |
| 4x2-w32 | candidate | G2_arithmetic | 19.546 | 0.390 |
| 4x2-w32 | candidate | epoch | 10.973 | 0.783 |
| 4x2-w32 | candidate | fft | 15.291 | -0.111 |
| 4x2-w32 | candidate | grouping | 1.702 | 0.438 |
| 4x2-w32 | candidate | inverse | 3.893 | -0.642 |
| 4x2-w32 | candidate | other | 6.615 | -0.089 |
| 4x2-w32 | candidate | scheduler | 5.005 | 0.283 |
| 4x2-w32 | candidate | solver | 3.585 | -0.277 |
| 4x2-w32 | fresh | G1_arithmetic | 34.774 | 0.610 |
| 4x2-w32 | fresh | G2_arithmetic | 18.960 | -0.197 |
| 4x2-w32 | fresh | epoch | 10.285 | 0.095 |
| 4x2-w32 | fresh | fft | 14.455 | -0.946 |
| 4x2-w32 | fresh | grouping | 1.583 | 0.320 |
| 4x2-w32 | fresh | inverse | 4.041 | -0.494 |
| 4x2-w32 | fresh | other | 6.957 | 0.254 |
| 4x2-w32 | fresh | scheduler | 5.686 | 0.964 |
| 4x2-w32 | fresh | solver | 3.258 | -0.605 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.379486 | 37.000 | 4.416× | 24.000 | 2.864× | false |
| 1x2-w32 | fresh | 8.340873 | 37.000 | 4.436× | 24.000 | 2.877× | false |
| 1x2-w32 | candidate | 8.468905 | 37.000 | 4.369× | 24.000 | 2.834× | false |
| 2x2-w32 | accepted | 11.514329 | 59.000 | 5.124× | 30.000 | 2.605× | false |
| 2x2-w32 | fresh | 11.465648 | 59.000 | 5.146× | 30.000 | 2.617× | false |
| 2x2-w32 | candidate | 11.492008 | 59.000 | 5.134× | 30.000 | 2.611× | false |
| 4x2-w32 | accepted | 16.080934 | 109.000 | 6.778× | 48.000 | 2.985× | false |
| 4x2-w32 | fresh | 15.965606 | 109.000 | 6.827× | 48.000 | 3.006× | false |
| 4x2-w32 | candidate | 15.928726 | 109.000 | 6.843× | 48.000 | 3.013× | false |
| 8x2-w32 | accepted | 24.028846 | 189.000 | 7.866× | 74.000 | 3.080× | false |
| 8x2-w32 | fresh | 24.179791 | 189.000 | 7.816× | 74.000 | 3.060× | false |
| 8x2-w32 | candidate | 24.369048 | 189.000 | 7.756× | 74.000 | 3.037× | false |
| 12x2-w32 | accepted | 34.108489 | 284.000 | 8.326× | 101.000 | 2.961× | false |
| 12x2-w32 | fresh | 33.882784 | 284.000 | 8.382× | 101.000 | 2.981× | false |
| 12x2-w32 | candidate | 34.020013 | 284.000 | 8.348× | 101.000 | 2.969× | false |
| 16x2-w32 | accepted | 40.041681 | 335.000 | 8.366× | 124.000 | 3.097× | false |
| 16x2-w32 | fresh | 40.100563 | 335.000 | 8.354× | 124.000 | 3.092× | false |
| 16x2-w32 | candidate | 40.102747 | 335.000 | 8.354× | 124.000 | 3.092× | false |
| 24x2-w32 | accepted | 67.244175 | 528.000 | 7.852× | 176.000 | 2.617× | false |
| 24x2-w32 | fresh | 67.283719 | 528.000 | 7.847× | 176.000 | 2.616× | false |
| 24x2-w32 | candidate | 67.245707 | 528.000 | 7.852× | 176.000 | 2.617× | false |
| merge-24x1-w32 | accepted | 66.183164 | 507.000 | 7.661× | 175.000 | 2.644× | false |
| merge-24x1-w32 | fresh | 65.766548 | 507.000 | 7.709× | 175.000 | 2.661× | false |
| merge-24x1-w32 | candidate | 66.155943 | 507.000 | 7.664× | 175.000 | 2.645× | false |
| 1x2-w16 | accepted | 10.303266 | 37.000 | 3.591× | 24.000 | 2.329× | false |
| 1x2-w16 | fresh | 10.122895 | 37.000 | 3.655× | 24.000 | 2.371× | false |
| 1x2-w16 | candidate | 10.317271 | 37.000 | 3.586× | 24.000 | 2.326× | false |

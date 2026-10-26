# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w16 | accepted | 18.376383 | 1.000000 | — | — | — |
| 1x2-w16 | aeglos | 19.265104 | 1.046014 | — | — | — |
| 1x2-w16 | off | 18.161999 | 0.986311 | — | — | — |
| 1x2-w16 | on | 18.133082 | 0.979282 | — | — | — |
| 1x2-w32 | accepted | 12.383334 | 1.000000 | 292.923 | 13.818487 | 115.80 |
| 1x2-w32 | aeglos | 11.832472 | 0.955516 | 333.421 | 12.048499 | 406.92 |
| 1x2-w32 | off | 12.333426 | 0.995723 | 281.979 | 12.774403 | 116.40 |
| 1x2-w32 | on | 12.293644 | 0.992757 | 280.772 | 12.729687 | 116.52 |
| 1x2-w64 | accepted | 12.796556 | 1.000000 | — | — | — |
| 1x2-w64 | aeglos | 8.859061 | 0.702506 | — | — | — |
| 1x2-w64 | off | 12.871939 | 1.013262 | — | — | — |
| 1x2-w64 | on | 12.841626 | 1.003522 | — | — | — |
| 1x2-w96 | accepted | 14.166512 | 1.000000 | 800.426 | 14.870721 | 111.15 |
| 1x2-w96 | aeglos | 7.885125 | 0.557855 | 529.174 | 8.037337 | 458.99 |
| 1x2-w96 | off | 14.228332 | 1.005144 | 863.752 | 16.504254 | 120.98 |
| 1x2-w96 | on | 14.232280 | 1.002757 | 865.235 | 16.432658 | 120.86 |
| 8x2-w16 | accepted | 41.940992 | 1.000000 | — | — | — |
| 8x2-w16 | aeglos | 104.148266 | 2.442330 | — | — | — |
| 8x2-w16 | off | 42.286054 | 1.008609 | — | — | — |
| 8x2-w16 | on | 42.140192 | 1.005279 | — | — | — |
| 8x2-w32 | accepted | 28.005279 | 1.000000 | — | — | — |
| 8x2-w32 | aeglos | 54.395062 | 1.947560 | — | — | — |
| 8x2-w32 | off | 27.959784 | 0.997013 | — | — | — |
| 8x2-w32 | on | 27.974529 | 0.998902 | — | — | — |
| 8x2-w64 | accepted | 27.374031 | 1.000000 | — | — | — |
| 8x2-w64 | aeglos | 35.340691 | 1.294019 | — | — | — |
| 8x2-w64 | off | 27.096330 | 0.997907 | — | — | — |
| 8x2-w64 | on | 27.186686 | 1.013985 | — | — | — |
| 8x2-w96 | accepted | 29.212100 | 1.000000 | — | — | — |
| 8x2-w96 | aeglos | 31.089053 | 1.066133 | — | — | — |
| 8x2-w96 | off | 28.286679 | 0.964541 | — | — | — |
| 8x2-w96 | on | 29.635926 | 1.001474 | — | — | — |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 1x2-w32 | accepted | G1_arithmetic | 40.395 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.955 | 0.000 |
| 1x2-w32 | accepted | epoch | 8.586 | 0.000 |
| 1x2-w32 | accepted | fft | 3.868 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.847 | 0.000 |
| 1x2-w32 | accepted | inverse | 5.625 | 0.000 |
| 1x2-w32 | accepted | other | 5.215 | 0.000 |
| 1x2-w32 | accepted | scheduler | 5.158 | 0.000 |
| 1x2-w32 | accepted | solver | 1.350 | 0.000 |
| 1x2-w32 | aeglos | other | 100.000 | 94.785 |
| 1x2-w32 | off | G1_arithmetic | 40.000 | -0.395 |
| 1x2-w32 | off | G2_arithmetic | 28.479 | -0.476 |
| 1x2-w32 | off | epoch | 8.767 | 0.181 |
| 1x2-w32 | off | fft | 4.471 | 0.603 |
| 1x2-w32 | off | grouping | 0.664 | -0.184 |
| 1x2-w32 | off | inverse | 5.339 | -0.287 |
| 1x2-w32 | off | other | 5.918 | 0.703 |
| 1x2-w32 | off | scheduler | 4.946 | -0.212 |
| 1x2-w32 | off | solver | 1.416 | 0.065 |
| 1x2-w32 | on | G1_arithmetic | 41.438 | 1.043 |
| 1x2-w32 | on | G2_arithmetic | 27.552 | -1.402 |
| 1x2-w32 | on | epoch | 8.443 | -0.143 |
| 1x2-w32 | on | fft | 3.895 | 0.027 |
| 1x2-w32 | on | grouping | 0.537 | -0.310 |
| 1x2-w32 | on | inverse | 5.359 | -0.266 |
| 1x2-w32 | on | other | 5.335 | 0.120 |
| 1x2-w32 | on | scheduler | 5.658 | 0.500 |
| 1x2-w32 | on | solver | 1.782 | 0.432 |
| 1x2-w96 | accepted | G1_arithmetic | 16.267 | 0.000 |
| 1x2-w96 | accepted | G2_arithmetic | 12.077 | 0.000 |
| 1x2-w96 | accepted | epoch | 48.604 | 0.000 |
| 1x2-w96 | accepted | fft | 2.810 | 0.000 |
| 1x2-w96 | accepted | grouping | 0.198 | 0.000 |
| 1x2-w96 | accepted | inverse | 2.611 | 0.000 |
| 1x2-w96 | accepted | other | 4.626 | 0.000 |
| 1x2-w96 | accepted | scheduler | 12.043 | 0.000 |
| 1x2-w96 | accepted | solver | 0.765 | 0.000 |
| 1x2-w96 | aeglos | other | 100.000 | 95.374 |
| 1x2-w96 | off | G1_arithmetic | 14.054 | -2.213 |
| 1x2-w96 | off | G2_arithmetic | 13.124 | 1.047 |
| 1x2-w96 | off | epoch | 50.548 | 1.944 |
| 1x2-w96 | off | fft | 2.523 | -0.288 |
| 1x2-w96 | off | grouping | 0.282 | 0.084 |
| 1x2-w96 | off | inverse | 1.795 | -0.817 |
| 1x2-w96 | off | other | 4.211 | -0.414 |
| 1x2-w96 | off | scheduler | 12.648 | 0.605 |
| 1x2-w96 | off | solver | 0.815 | 0.051 |
| 1x2-w96 | on | G1_arithmetic | 15.117 | -1.149 |
| 1x2-w96 | on | G2_arithmetic | 12.887 | 0.809 |
| 1x2-w96 | on | epoch | 49.434 | 0.830 |
| 1x2-w96 | on | fft | 2.664 | -0.146 |
| 1x2-w96 | on | grouping | 0.317 | 0.120 |
| 1x2-w96 | on | inverse | 2.103 | -0.508 |
| 1x2-w96 | on | other | 4.483 | -0.142 |
| 1x2-w96 | on | scheduler | 12.367 | 0.324 |
| 1x2-w96 | on | solver | 0.627 | -0.138 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w16 | accepted | 18.376383 | 37.000 | 2.013× | 24.000 | 1.306× | false |
| 1x2-w16 | off | 18.161999 | 37.000 | 2.037× | 24.000 | 1.321× | false |
| 1x2-w16 | on | 18.133082 | 37.000 | 2.040× | 24.000 | 1.324× | false |
| 1x2-w16 | aeglos | 19.265104 | 37.000 | 1.921× | 24.000 | 1.246× | false |
| 1x2-w32 | accepted | 12.383334 | 37.000 | 2.988× | 24.000 | 1.938× | false |
| 1x2-w32 | off | 12.333426 | 37.000 | 3.000× | 24.000 | 1.946× | false |
| 1x2-w32 | on | 12.293644 | 37.000 | 3.010× | 24.000 | 1.952× | false |
| 1x2-w32 | aeglos | 11.832472 | 37.000 | 3.127× | 24.000 | 2.028× | false |
| 1x2-w64 | accepted | 12.796556 | 37.000 | 2.891× | 24.000 | 1.876× | false |
| 1x2-w64 | off | 12.871939 | 37.000 | 2.874× | 24.000 | 1.865× | false |
| 1x2-w64 | on | 12.841626 | 37.000 | 2.881× | 24.000 | 1.869× | false |
| 1x2-w64 | aeglos | 8.859061 | 37.000 | 4.177× | 24.000 | 2.709× | false |
| 1x2-w96 | accepted | 14.166512 | 37.000 | 2.612× | 24.000 | 1.694× | false |
| 1x2-w96 | off | 14.228332 | 37.000 | 2.600× | 24.000 | 1.687× | false |
| 1x2-w96 | on | 14.232280 | 37.000 | 2.600× | 24.000 | 1.686× | false |
| 1x2-w96 | aeglos | 7.885125 | 37.000 | 4.692× | 24.000 | 3.044× | false |
| 8x2-w16 | accepted | 41.940992 | 189.000 | 4.506× | 74.000 | 1.764× | false |
| 8x2-w16 | off | 42.286054 | 189.000 | 4.470× | 74.000 | 1.750× | false |
| 8x2-w16 | on | 42.140192 | 189.000 | 4.485× | 74.000 | 1.756× | false |
| 8x2-w16 | aeglos | 104.148266 | 189.000 | 1.815× | 74.000 | 0.711× | false |
| 8x2-w32 | accepted | 28.005279 | 189.000 | 6.749× | 74.000 | 2.642× | false |
| 8x2-w32 | off | 27.959784 | 189.000 | 6.760× | 74.000 | 2.647× | false |
| 8x2-w32 | on | 27.974529 | 189.000 | 6.756× | 74.000 | 2.645× | false |
| 8x2-w32 | aeglos | 54.395062 | 189.000 | 3.475× | 74.000 | 1.360× | false |
| 8x2-w64 | accepted | 27.374031 | 189.000 | 6.904× | 74.000 | 2.703× | false |
| 8x2-w64 | off | 27.096330 | 189.000 | 6.975× | 74.000 | 2.731× | false |
| 8x2-w64 | on | 27.186686 | 189.000 | 6.952× | 74.000 | 2.722× | false |
| 8x2-w64 | aeglos | 35.340691 | 189.000 | 5.348× | 74.000 | 2.094× | false |
| 8x2-w96 | accepted | 29.212100 | 189.000 | 6.470× | 74.000 | 2.533× | false |
| 8x2-w96 | off | 28.286679 | 189.000 | 6.682× | 74.000 | 2.616× | false |
| 8x2-w96 | on | 29.635926 | 189.000 | 6.377× | 74.000 | 2.497× | false |
| 8x2-w96 | aeglos | 31.089053 | 189.000 | 6.079× | 74.000 | 2.380× | false |

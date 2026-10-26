# CPU experiment results

Latency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.

| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 12x2-w32 | accepted | 34.479295 | 1.000000 | 960.032 | 34.932888 | 740.50 |
| 12x2-w32 | off | 34.194708 | 0.991746 | 959.788 | 34.897740 | 751.00 |
| 12x2-w32 | on | 34.748290 | 1.000074 | 951.055 | 35.154882 | 743.75 |
| 16x2-w32 | accepted | 41.279478 | 1.000000 | — | — | — |
| 16x2-w32 | off | 40.951905 | 0.995235 | — | — | — |
| 16x2-w32 | on | 40.939441 | 0.992713 | — | — | — |
| 1x2-w16 | accepted | 10.522266 | 1.000000 | — | — | — |
| 1x2-w16 | off | 10.494102 | 0.984941 | — | — | — |
| 1x2-w16 | on | 11.132547 | 1.042800 | — | — | — |
| 1x2-w32 | accepted | 8.592875 | 1.000000 | 199.876 | 8.749502 | 131.00 |
| 1x2-w32 | off | 8.651315 | 1.007794 | 200.738 | 8.704589 | 129.75 |
| 1x2-w32 | on | 8.606501 | 1.003733 | 195.493 | 8.721220 | 127.75 |
| 24x2-w32 | accepted | 68.300461 | 1.000000 | — | — | — |
| 24x2-w32 | off | 68.635936 | 1.003500 | — | — | — |
| 24x2-w32 | on | 69.981548 | 1.024418 | — | — | — |
| 2x2-w32 | accepted | 11.715638 | 1.000000 | — | — | — |
| 2x2-w32 | off | 11.721300 | 0.999920 | — | — | — |
| 2x2-w32 | on | 12.003772 | 1.030985 | — | — | — |
| 4x2-w32 | accepted | 16.020210 | 1.000000 | — | — | — |
| 4x2-w32 | off | 16.213257 | 1.011532 | — | — | — |
| 4x2-w32 | on | 16.683316 | 1.040544 | — | — | — |
| 8x2-w32 | accepted | 24.680698 | 1.000000 | — | — | — |
| 8x2-w32 | off | 24.424749 | 0.988621 | — | — | — |
| 8x2-w32 | on | 24.560268 | 0.996458 | — | — | — |
| merge-24x1-w32 | accepted | 67.342885 | 1.000000 | — | — | — |
| merge-24x1-w32 | off | 67.121031 | 1.000346 | — | — | — |
| merge-24x1-w32 | on | 69.397358 | 1.030771 | — | — | — |
| merge-54x1-w32 | accepted | 176.647389 | 1.000000 | 4786.932 | 174.092277 | 1725.04 |
| merge-54x1-w32 | off | 176.390000 | 0.994829 | 4926.849 | 179.854844 | 1603.56 |
| merge-54x1-w32 | on | 178.577997 | 1.010678 | 4995.006 | 180.834586 | 1602.17 |

Category shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.

| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |
| --- | --- | --- | ---: | ---: |
| 12x2-w32 | accepted | G1_arithmetic | 34.292 | 0.000 |
| 12x2-w32 | accepted | G2_arithmetic | 13.475 | 0.000 |
| 12x2-w32 | accepted | epoch | 5.699 | 0.000 |
| 12x2-w32 | accepted | fft | 26.976 | 0.000 |
| 12x2-w32 | accepted | grouping | 1.737 | 0.000 |
| 12x2-w32 | accepted | inverse | 3.560 | 0.000 |
| 12x2-w32 | accepted | other | 3.887 | 0.000 |
| 12x2-w32 | accepted | scheduler | 4.047 | 0.000 |
| 12x2-w32 | accepted | solver | 6.327 | 0.000 |
| 12x2-w32 | off | G1_arithmetic | 34.576 | 0.284 |
| 12x2-w32 | off | G2_arithmetic | 14.016 | 0.541 |
| 12x2-w32 | off | epoch | 5.193 | -0.507 |
| 12x2-w32 | off | fft | 26.546 | -0.430 |
| 12x2-w32 | off | grouping | 1.866 | 0.129 |
| 12x2-w32 | off | inverse | 3.294 | -0.266 |
| 12x2-w32 | off | other | 3.936 | 0.049 |
| 12x2-w32 | off | scheduler | 3.819 | -0.228 |
| 12x2-w32 | off | solver | 6.754 | 0.427 |
| 12x2-w32 | on | G1_arithmetic | 35.097 | 0.805 |
| 12x2-w32 | on | G2_arithmetic | 14.278 | 0.803 |
| 12x2-w32 | on | fft | 27.133 | 0.157 |
| 12x2-w32 | on | grouping | 1.753 | 0.017 |
| 12x2-w32 | on | inverse | 3.472 | -0.088 |
| 12x2-w32 | on | other | 4.645 | 0.758 |
| 12x2-w32 | on | scheduler | 7.537 | 3.490 |
| 12x2-w32 | on | solver | 6.085 | -0.242 |
| 1x2-w32 | accepted | G1_arithmetic | 38.009 | 0.000 |
| 1x2-w32 | accepted | G2_arithmetic | 28.817 | 0.000 |
| 1x2-w32 | accepted | epoch | 9.767 | 0.000 |
| 1x2-w32 | accepted | fft | 4.695 | 0.000 |
| 1x2-w32 | accepted | grouping | 0.739 | 0.000 |
| 1x2-w32 | accepted | inverse | 7.561 | 0.000 |
| 1x2-w32 | accepted | other | 5.034 | 0.000 |
| 1x2-w32 | accepted | scheduler | 3.829 | 0.000 |
| 1x2-w32 | accepted | solver | 1.548 | 0.000 |
| 1x2-w32 | off | G1_arithmetic | 37.576 | -0.433 |
| 1x2-w32 | off | G2_arithmetic | 30.202 | 1.386 |
| 1x2-w32 | off | epoch | 9.585 | -0.182 |
| 1x2-w32 | off | fft | 4.501 | -0.194 |
| 1x2-w32 | off | grouping | 0.570 | -0.170 |
| 1x2-w32 | off | inverse | 7.271 | -0.290 |
| 1x2-w32 | off | other | 4.894 | -0.140 |
| 1x2-w32 | off | scheduler | 3.931 | 0.102 |
| 1x2-w32 | off | solver | 1.470 | -0.078 |
| 1x2-w32 | on | G1_arithmetic | 39.927 | 1.918 |
| 1x2-w32 | on | G2_arithmetic | 30.072 | 1.255 |
| 1x2-w32 | on | fft | 4.969 | 0.274 |
| 1x2-w32 | on | grouping | 0.684 | -0.055 |
| 1x2-w32 | on | inverse | 7.471 | -0.090 |
| 1x2-w32 | on | other | 5.606 | 0.571 |
| 1x2-w32 | on | scheduler | 9.723 | 5.893 |
| 1x2-w32 | on | solver | 1.548 | -0.000 |
| merge-54x1-w32 | accepted | G1_arithmetic | 28.003 | 0.000 |
| merge-54x1-w32 | accepted | G2_arithmetic | 12.479 | 0.000 |
| merge-54x1-w32 | accepted | epoch | 2.214 | 0.000 |
| merge-54x1-w32 | accepted | fft | 42.507 | 0.000 |
| merge-54x1-w32 | accepted | grouping | 1.412 | 0.000 |
| merge-54x1-w32 | accepted | inverse | 1.801 | 0.000 |
| merge-54x1-w32 | accepted | other | 3.919 | 0.000 |
| merge-54x1-w32 | accepted | scheduler | 2.212 | 0.000 |
| merge-54x1-w32 | accepted | solver | 5.454 | 0.000 |
| merge-54x1-w32 | off | G1_arithmetic | 24.415 | -3.588 |
| merge-54x1-w32 | off | G2_arithmetic | 17.673 | 5.194 |
| merge-54x1-w32 | off | epoch | 2.061 | -0.153 |
| merge-54x1-w32 | off | fft | 41.223 | -1.284 |
| merge-54x1-w32 | off | grouping | 1.416 | 0.005 |
| merge-54x1-w32 | off | inverse | 1.756 | -0.046 |
| merge-54x1-w32 | off | other | 4.287 | 0.368 |
| merge-54x1-w32 | off | scheduler | 2.049 | -0.163 |
| merge-54x1-w32 | off | solver | 5.121 | -0.332 |
| merge-54x1-w32 | on | G1_arithmetic | 27.035 | -0.968 |
| merge-54x1-w32 | on | G2_arithmetic | 15.928 | 3.449 |
| merge-54x1-w32 | on | fft | 40.687 | -1.820 |
| merge-54x1-w32 | on | grouping | 1.446 | 0.034 |
| merge-54x1-w32 | on | inverse | 1.739 | -0.062 |
| merge-54x1-w32 | on | other | 4.361 | 0.442 |
| merge-54x1-w32 | on | scheduler | 3.751 | 1.539 |
| merge-54x1-w32 | on | solver | 5.054 | -0.399 |

All final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.

## Supplied reference comparison

CPU and Metal times below are supplied historical references, not new Apple measurements.

| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| 1x2-w32 | accepted | 8.592875 | 37.000 | 4.306× | 24.000 | 2.793× | false |
| 1x2-w32 | off | 8.651315 | 37.000 | 4.277× | 24.000 | 2.774× | false |
| 1x2-w32 | on | 8.606501 | 37.000 | 4.299× | 24.000 | 2.789× | false |
| 2x2-w32 | accepted | 11.715638 | 59.000 | 5.036× | 30.000 | 2.561× | false |
| 2x2-w32 | off | 11.721300 | 59.000 | 5.034× | 30.000 | 2.559× | false |
| 2x2-w32 | on | 12.003772 | 59.000 | 4.915× | 30.000 | 2.499× | false |
| 4x2-w32 | accepted | 16.020210 | 109.000 | 6.804× | 48.000 | 2.996× | false |
| 4x2-w32 | off | 16.213257 | 109.000 | 6.723× | 48.000 | 2.961× | false |
| 4x2-w32 | on | 16.683316 | 109.000 | 6.533× | 48.000 | 2.877× | false |
| 8x2-w32 | accepted | 24.680698 | 189.000 | 7.658× | 74.000 | 2.998× | false |
| 8x2-w32 | off | 24.424749 | 189.000 | 7.738× | 74.000 | 3.030× | false |
| 8x2-w32 | on | 24.560268 | 189.000 | 7.695× | 74.000 | 3.013× | false |
| 12x2-w32 | accepted | 34.479295 | 284.000 | 8.237× | 101.000 | 2.929× | false |
| 12x2-w32 | off | 34.194708 | 284.000 | 8.305× | 101.000 | 2.954× | false |
| 12x2-w32 | on | 34.748290 | 284.000 | 8.173× | 101.000 | 2.907× | false |
| 16x2-w32 | accepted | 41.279478 | 335.000 | 8.115× | 124.000 | 3.004× | false |
| 16x2-w32 | off | 40.951905 | 335.000 | 8.180× | 124.000 | 3.028× | false |
| 16x2-w32 | on | 40.939441 | 335.000 | 8.183× | 124.000 | 3.029× | false |
| 24x2-w32 | accepted | 68.300461 | 528.000 | 7.731× | 176.000 | 2.577× | false |
| 24x2-w32 | off | 68.635936 | 528.000 | 7.693× | 176.000 | 2.564× | false |
| 24x2-w32 | on | 69.981548 | 528.000 | 7.545× | 176.000 | 2.515× | false |
| merge-24x1-w32 | accepted | 67.342885 | 507.000 | 7.529× | 175.000 | 2.599× | false |
| merge-24x1-w32 | off | 67.121031 | 507.000 | 7.554× | 175.000 | 2.607× | false |
| merge-24x1-w32 | on | 69.397358 | 507.000 | 7.306× | 175.000 | 2.522× | false |

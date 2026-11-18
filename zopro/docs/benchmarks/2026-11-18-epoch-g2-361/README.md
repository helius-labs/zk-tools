# Solver epochs and G2 ranges on Ryzen

Fresh confirmation processes use the exact release SDK. Complete proof latency includes the SDK verification step. Each process exports its final proof and public witness for independent Gnark verification.

| Shape | Epochs / G2 ranges | Control ms | Selected ms | Paired change | Faster blocks |
|---|---:|---:|---:|---:|---:|
| 1x2 | 4 / 2 | 7.731 | 7.625 | -1.21% | 9/9 |
| 2x2 | 6 / 2 | 10.821 | 10.675 | -1.63% | 6/9 |

## All eight M5 references

The first two rows use the fresh confirmation measurements. The other six use the prespecified local control. The speedup is the historical M5 reference divided by the median of local process means.

| Shape | Ryzen ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup | CPU 10× target ms |
|---|---:|---:|---:|---:|---:|
| 1x2 | 7.625 | 65 | 37 / 24 | 4.85× / 3.15× | 3.7 |
| 2x2 | 10.675 | 120 | 59 / 30 | 5.53× / 2.81× | 5.9 |
| 4x2 | 16.963 | 214 | 109 / 48 | 6.43× / 2.83× | 10.9 |
| 8x2 | 26.887 | 358 | 189 / 74 | 7.03× / 2.75× | 18.9 |
| 12x2 | 39.113 | 528 | 284 / 101 | 7.26× / 2.58× | 28.4 |
| 16x2 | 45.273 | 598 | 335 / 124 | 7.40× / 2.74× | 33.5 |
| 24x2 | 74.196 | 977 | 528 / 176 | 7.12× / 2.37× | 52.8 |
| merge-24x1 | 71.443 | 869 | 507 / 175 | 7.10× / 2.45× | 50.7 |

## Interaction screen

| Shape | Epochs | Two ranges ms | Four ranges with serial split ms | Paired four vs two | Faster blocks |
|---|---:|---:|---:|---:|---:|
| 1x2 | 2 | 7.754 | 7.986 | +2.99% | 1/5 |
| 1x2 | 4 | 7.638 | 7.871 | +2.70% | 0/5 |
| 1x2 | 6 | 7.662 | 8.012 | +3.49% | 2/5 |
| 1x2 | 8 | 7.683 | 7.984 | +3.97% | 1/5 |
| 1x2 | 16 | 7.807 | 8.045 | +2.97% | 2/5 |
| 2x2 | 2 | 11.090 | 11.296 | +1.85% | 0/5 |
| 2x2 | 4 | 10.736 | 11.118 | +3.23% | 0/5 |
| 2x2 | 6 | 10.684 | 11.018 | +2.79% | 0/5 |
| 2x2 | 8 | 10.714 | 10.908 | +2.46% | 0/5 |
| 2x2 | 16 | 10.782 | 11.117 | +2.33% | 0/5 |

[Protocol](artifacts/protocol.md) · [Request timings](artifacts/timings.json.gz) · [Verification](artifacts/verification.json)

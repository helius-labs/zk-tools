# Verified Ryzen scorecard after FFT input packing

Five fresh matched blocks compare the new executable with its predecessor on the same Ryzen 9950X. Each small-shape process timed 51 requests after three warmups, and each other process timed eleven. Both executables used compact outputs on 12×2 and 16×2. Only 12×2 enabled FFT input packing on the new executable. At 16 workers 12×2 also used the single-range signed H layout for both executables. Other rows measure the effect of the complete rebuild with the new option disabled. M5 CPU and Metal values are the supplied cross-machine references.

| Shape | Threads | Previous ms | Current ms | Paired change | Wins / 5 | Current CPU-ms | Peak MiB previous / current | M5 CPU / Metal ms | CPU / Metal speedup | Blocks at 10× CPU / 5 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.240 | 7.253 | +0.19% | 2 | 151.6 | 68.8 / 63.8 | 37.0 / 24.0 | 5.10× / 3.31× | 0 |
| 2x2 | 32 | 10.100 | 10.110 | +0.50% | 1 | 201.6 | 120.2 / 119.0 | 59.0 / 30.0 | 5.84× / 2.97× | 0 |
| 4x2 | 32 | 15.533 | 15.619 | +0.75% | 1 | 407.8 | 260.5 / 247.2 | 109.0 / 48.0 | 6.98× / 3.07× | 0 |
| 8x2 | 32 | 24.159 | 24.037 | -0.11% | 3 | 666.4 | 431.0 / 433.5 | 189.0 / 74.0 | 7.86× / 3.08× | 0 |
| 12x2 | 32 | 33.124 | 32.622 | -2.47% | 5 | 900.7 | 625.2 / 626.0 | 284.0 / 101.0 | 8.71× / 3.10× | 0 |
| 12x2 | 16 | 37.018 | 36.439 | -1.68% | 5 | 537.2 | 504.2 / 503.2 | 284.0 / 101.0 | 7.79× / 2.77× | 0 |
| 16x2 | 32 | 40.985 | 40.554 | -0.36% | 3 | 1170.6 | 719.2 / 740.2 | 335.0 / 124.0 | 8.26× / 3.06× | 0 |
| 24x2 | 32 | 69.471 | 69.314 | +0.23% | 2 | 2014.9 | 1128.2 / 1141.5 | 528.0 / 176.0 | 7.62× / 2.54× | 0 |
| merge-24x1 | 32 | 66.994 | 66.954 | -0.16% | 3 | 1916.7 | 1051.5 / 1047.5 | 507.0 / 175.0 | 7.57× / 2.61× | 0 |

At 32 threads 0/8 shapes reached 10× M5 CPU in all five blocks, and 0/8 reached it by median. Each export passed independent Gnark verification.

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Varied 12×2 witnesses](../2026-11-19-varied-fft417/README.md) · [Earlier C7a scorecard](../2026-11-18-overlap-c7a369/README.md)

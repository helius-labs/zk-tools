# Verified Ryzen scorecard for the qualified FFT profiles

Five fresh paired blocks measured the original reference requests. Both profiles use input packing on 12×2. The current profile also uses it on 16×2, 24×2 and merge-24×1, as qualified in the preceding eight-shape experiment. The varied-input experiment qualified local Euler weighting for 12×2 at both 16 and 32 workers. The remaining settings are held fixed by shape. Each 1×2 or 2×2 process timed 51 requests and each other process timed eleven, after three warmups. The Apple references are the supplied CPU and Metal times.

| Shape | Threads | Previous ms | Current ms | Paired change | Wins / 5 | Current CPU-ms | Peak MiB previous / current | M5 CPU / Metal ms | CPU / Metal speedup | Blocks at 10× CPU / 5 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.260 | 7.257 | +0.22% | 2 | 151.0 | 65.0 / 64.5 | 37.0 / 24.0 | 5.10× / 3.31× | 0 |
| 2x2 | 32 | 10.050 | 10.162 | +1.16% | 0 | 202.8 | 118.5 / 116.0 | 59.0 / 30.0 | 5.81× / 2.95× | 0 |
| 4x2 | 32 | 15.348 | 15.573 | +1.47% | 1 | 414.7 | 255.8 / 262.2 | 109.0 / 48.0 | 7.00× / 3.08× | 0 |
| 8x2 | 32 | 24.321 | 24.071 | -0.11% | 3 | 661.8 | 444.0 / 442.0 | 189.0 / 74.0 | 7.85× / 3.07× | 0 |
| 12x2 | 32 | 32.951 | 32.005 | -3.00% | 5 | 884.5 | 604.2 / 602.0 | 284.0 / 101.0 | 8.87× / 3.16× | 0 |
| 12x2 | 16 | 36.502 | 36.512 | -0.36% | 3 | 533.2 | 522.5 / 503.5 | 284.0 / 101.0 | 7.78× / 2.77× | 0 |
| 16x2 | 32 | 40.822 | 39.950 | -1.17% | 4 | 1159.8 | 727.8 / 733.8 | 335.0 / 124.0 | 8.39× / 3.10× | 0 |
| 24x2 | 32 | 69.648 | 66.867 | -3.28% | 4 | 1955.4 | 1128.7 / 1123.1 | 528.0 / 176.0 | 7.90× / 2.63× | 0 |
| merge-24x1 | 32 | 66.364 | 65.766 | -1.73% | 3 | 1873.8 | 1040.5 / 1045.0 | 507.0 / 175.0 | 7.71× / 2.66× | 0 |

At 32 threads, 12×2 improved in all five paired blocks. The new executable was slower on 2×2 (+1.16%, slower in five of five) and 4×2 (+1.47%, slower in four of five). Both shapes ran with FFT input packing and local Euler weighting disabled in both executables. This comparison cannot isolate the cause.

At 32 threads 0/8 shapes reached 10× M5 CPU in all five blocks, and 0/8 reached it by median. Each export passed independent Gnark verification.

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Varied 12×2 witnesses](../2026-11-19-varied-column-center427/README.md) · [Earlier C7a scorecard](../2026-11-18-overlap-c7a369/README.md)

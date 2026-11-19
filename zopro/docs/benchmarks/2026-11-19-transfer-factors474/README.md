# Local Euler weighting and public wire offsets together on 24×2

Eight public training inputs determined a key-bound offset file. Eight separate requests and the original were each run through the same 32-worker Ryzen checkout with both options off, offsets alone, local FFT weighting alone and both together. Five randomized matched blocks per input and configuration each used three primes and eleven timed full SDK calls.

| Input | Neither / file / Euler / both ms | File vs neither | Both vs Euler | Both vs file |
|---:|---:|---:|---:|---:|
| 0 | 67.651 / 65.430 / 66.762 / 64.422 | -3.41% (4/5) | -3.44% (5/5) | -1.78% (5/5) |
| 97 | 67.542 / 65.576 / 65.744 / 62.480 | -2.91% (5/5) | -4.97% (5/5) | -4.72% (5/5) |
| 98 | 68.405 / 65.685 / 65.142 / 62.929 | -3.32% (5/5) | -4.05% (5/5) | -4.37% (5/5) |
| 99 | 68.470 / 65.564 / 65.883 / 63.763 | -4.01% (5/5) | -3.22% (5/5) | -3.34% (4/5) |
| 100 | 67.770 / 65.888 / 65.761 / 62.442 | -3.66% (4/5) | -4.05% (5/5) | -6.01% (5/5) |
| 101 | 67.463 / 65.022 / 65.449 / 62.800 | -3.57% (5/5) | -3.98% (5/5) | -3.42% (5/5) |
| 102 | 68.278 / 65.264 / 66.549 / 62.668 | -3.66% (5/5) | -4.53% (5/5) | -3.72% (5/5) |
| 103 | 67.449 / 65.252 / 65.086 / 62.917 | -3.26% (3/5) | -3.69% (5/5) | -3.94% (5/5) |
| 104 | 67.466 / 65.264 / 65.222 / 63.667 | -3.43% (5/5) | -1.81% (3/5) | -1.35% (4/5) |

Against the profile selected in the [varied Euler qualification](../2026-11-19-larger-varied473/README.md), the file changed paired latency by -4.01% across the held-out inputs, with 8/8 faster. The original measured 64.422 ms on that file-enabled profile. The combination passed the latency screen. All 196 exports passed independent Gnark verification. At preparation limits of 0, 1, 4 and 8 MiB, the prover selected ordinary proving, ordinary proving, ordinary proving, offsets, respectively. The four additional exports also passed Gnark verification. The file occupies 1,471,080 bytes and took 126.33 ms to prepare per key. [Public coefficients](artifacts/public.wo)

[Results](artifacts/score.json) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

## Current 32-worker Ryzen scorecard

Measurements use the same CPU executable. The [preceding scorecard](../2026-11-19-larger-varied473/README.md) supplies the other profiles.

| Shape | Warm ms | Peak MiB | M5 CPU / Metal ms | Speedup vs CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2 | 7.033 | 65.0 | 37.0 / 24.0 | 5.26× / 3.41× |
| 2x2 | 9.619 | 117.5 | 59.0 / 30.0 | 6.13× / 3.12× |
| 4x2 | 15.421 | 263.5 | 109.0 / 48.0 | 7.07× / 3.11× |
| 8x2 | 23.898 | 436.2 | 189.0 / 74.0 | 7.91× / 3.10× |
| 12x2 | 31.064 | 580.4 | 284.0 / 101.0 | 9.14× / 3.25× |
| 16x2 | 39.553 | 670.1 | 335.0 / 124.0 | 8.47× / 3.14× |
| 24x2 | 64.422 | 1038.9 | 528.0 / 176.0 | 8.20× / 2.73× |
| merge-24x1 | 63.216 | 1030.2 | 507.0 / 175.0 | 8.02× / 2.77× |

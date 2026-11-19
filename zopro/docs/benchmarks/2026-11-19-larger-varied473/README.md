# Local Euler weighting on varied 24×2 and merge-24×1 requests

The Ryzen 9950X ran eight varied requests and the original for each shape, at 32 pinned workers. Transfers included deposits, withdrawals, private splits and data-bearing outputs. Merge inputs vary the coins and Merkle tree. The same binary packed the first inverse columns in both arms. The tested option performs the Euler weighting inside the local transform. Five randomized matched blocks used three primes and eleven timed full SDK requests per process.

| Shape | Input | Ordinary / local ms | Paired change (wins) | Peak MiB ordinary / local |
|---|---:|---:|---:|---:|
| 24x2 | 0 | 67.190 / 66.214 | -2.16% (5/5) | 1116.8 / 1123.2 |
| 24x2 | 81 | 68.118 / 67.642 | -0.70% (3/5) | 1169.0 / 1146.1 |
| 24x2 | 82 | 68.342 / 66.918 | -2.52% (4/5) | 1125.8 / 1117.8 |
| 24x2 | 83 | 68.817 / 66.596 | -2.01% (4/5) | 1128.7 / 1137.8 |
| 24x2 | 84 | 68.366 / 67.244 | -1.34% (5/5) | 1131.8 / 1136.0 |
| 24x2 | 85 | 68.079 / 66.342 | -2.38% (5/5) | 1139.8 / 1144.1 |
| 24x2 | 86 | 68.389 / 66.825 | -0.59% (4/5) | 1118.9 / 1134.0 |
| 24x2 | 87 | 70.463 / 66.285 | -5.28% (5/5) | 1146.0 / 1110.2 |
| 24x2 | 88 | 68.491 / 66.114 | -2.04% (5/5) | 1108.2 / 1114.7 |
| merge-24x1 | 0 | 65.749 / 63.216 | -3.85% (4/5) | 1023.0 / 1030.2 |
| merge-24x1 | 1 | 62.295 / 59.520 | -4.75% (5/5) | 1001.8 / 992.8 |
| merge-24x1 | 2 | 62.186 / 60.030 | -2.58% (5/5) | 989.8 / 1004.2 |
| merge-24x1 | 3 | 61.982 / 60.137 | -3.25% (5/5) | 994.2 / 992.2 |
| merge-24x1 | 4 | 61.752 / 59.674 | -2.47% (5/5) | 1001.0 / 1017.5 |
| merge-24x1 | 5 | 62.071 / 59.740 | -4.16% (4/5) | 1022.8 / 1003.2 |
| merge-24x1 | 6 | 61.959 / 59.660 | -3.17% (5/5) | 992.8 / 1005.5 |
| merge-24x1 | 7 | 61.505 / 59.901 | -2.41% (5/5) | 1015.2 / 1020.8 |
| merge-24x1 | 8 | 61.788 / 60.280 | -3.59% (5/5) | 1016.3 / 1008.2 |

24x2 measured -2.02% median paired change over the eight varied requests, with 8/8 faster, and qualified. merge-24x1 measured -3.21% median paired change over the eight varied requests, with 8/8 faster, and qualified. All 180 exports verified independently. [Original screen and debugger checks](../2026-11-19-larger-center471/README.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

## Current 32-worker Ryzen scorecard

The table uses this run for newly qualified profiles, the [16×2 qualification](../2026-11-19-sixteen-wire472/README.md) where applicable, and the [earlier eight-shape measurements](../2026-11-19-integrated-wire467/README.md) for the remaining rows. All use the same production binary.

| Shape | Warm ms | Peak MiB | M5 CPU / Metal ms | Speedup vs CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2 | 7.033 | 65.0 | 37.0 / 24.0 | 5.26× / 3.41× |
| 2x2 | 9.619 | 117.5 | 59.0 / 30.0 | 6.13× / 3.12× |
| 4x2 | 15.421 | 263.5 | 109.0 / 48.0 | 7.07× / 3.11× |
| 8x2 | 23.898 | 436.2 | 189.0 / 74.0 | 7.91× / 3.10× |
| 12x2 | 31.064 | 580.4 | 284.0 / 101.0 | 9.14× / 3.25× |
| 16x2 | 39.553 | 670.1 | 335.0 / 124.0 | 8.47× / 3.14× |
| 24x2 | 66.214 | 1123.2 | 528.0 / 176.0 | 7.97× / 2.66× |
| merge-24x1 | 63.216 | 1030.2 | 507.0 / 175.0 | 8.02× / 2.77× |

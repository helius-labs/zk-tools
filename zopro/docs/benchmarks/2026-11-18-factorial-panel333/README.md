# C7a G2 factorial and integrated H scorecard

Each cell has seven randomized process means. Every 1×2 process timed 101 SDK requests, other processes timed 21, after three primes. The table uses the integrated release executable and the core counts specified before launch. The reference columns are the supplied M5 measurements.

| Shape | Cores | Setting | Zopro ms | Peak MiB | M5 CPU / Metal ms | Speedup CPU / Metal | At CPU target |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 64 | release-front | 8.444 | 56 | 37 / 24 | 4.38× / 2.84× | 0/7 |
| 2x2 | 48 | release-front | 14.212 | 105 | 59 / 30 | 4.15× / 2.11× | 0/7 |
| 4x2 | 48 | release-ordinary | 17.853 | 281 | 109 / 48 | 6.11× / 2.69× | 0/7 |
| 8x2 | 48 | release-ordinary | 24.555 | 450 | 189 / 74 | 7.70× / 3.01× | 0/7 |
| 12x2 | 64 | release-h19 | 33.623 | 643 | 284 / 101 | 8.45× / 3.00× | 0/7 |
| 16x2 | 64 | release-h19 | 37.214 | 755 | 335 / 124 | 9.00× / 3.33× | 0/7 |
| 24x2 | 64 | release-h19 | 51.368 | 1226 | 528 / 176 | 10.28× / 3.43× | 7/7 |
| merge-24x1 | 64 | release-h19 | 53.512 | 1155 | 507 / 175 | 9.47× / 3.27× | 0/7 |

## Width and partition effects on 1×2

Both parameters are changed within the same executable. The old executable provides independent controls for the 12-bit/two-part and 11-bit/four-part diagonals. Negative changes indicate lower latency.

| Cores | Change | Baseline ms | Changed ms | Paired change | Faster blocks |
|---:|---|---:|---:|---:|---:|
| 64 | 12x2 → 11x2 | 8.404 | 9.343 | +10.56% | 0/7 |
| 64 | 12x4 → 11x4 | 8.835 | 9.071 | +2.26% | 0/7 |
| 64 | 12x2 → 12x4 | 8.404 | 8.835 | +5.02% | 0/7 |
| 64 | 11x2 → 11x4 | 9.343 | 9.071 | -3.01% | 7/7 |
| 64 | old-normal → 12x2 | 8.353 | 8.404 | +0.28% | 1/7 |
| 64 | old-four → 11x4 | 9.043 | 9.071 | +0.41% | 3/7 |
| 80 | 12x2 → 11x2 | 9.532 | 9.660 | +1.45% | 2/7 |
| 80 | 12x4 → 11x4 | 8.821 | 8.718 | -1.17% | 4/7 |
| 80 | 12x2 → 12x4 | 9.532 | 8.821 | -7.01% | 7/7 |
| 80 | 11x2 → 11x4 | 9.660 | 8.718 | -9.25% | 7/7 |
| 80 | old-normal → 12x2 | 9.647 | 9.532 | -2.25% | 5/7 |
| 80 | old-four → 11x4 | 8.752 | 8.718 | -0.27% | 4/7 |
| 96 | 12x2 → 11x2 | 10.013 | 9.700 | -3.26% | 7/7 |
| 96 | 12x4 → 11x4 | 9.239 | 9.070 | -1.71% | 6/7 |
| 96 | 12x2 → 12x4 | 10.013 | 9.239 | -7.54% | 7/7 |
| 96 | 11x2 → 11x4 | 9.700 | 9.070 | -6.49% | 7/7 |
| 96 | old-normal → 12x2 | 10.173 | 10.013 | -1.40% | 6/7 |
| 96 | old-four → 11x4 | 9.059 | 9.070 | +0.56% | 2/7 |

## Full panel

| Shape | Cores | Mode | Median ms | CPU ms | Peak MiB |
|---|---:|---|---:|---:|---:|
| 12x2 | 48 | qualified-h19 | 32.812 | 1226 | 664 |
| 12x2 | 48 | release-h19 | 32.568 | 1222 | 656 |
| 12x2 | 48 | release-ordinary | 32.155 | 1247 | 642 |
| 12x2 | 64 | qualified-h19 | 33.703 | 1571 | 646 |
| 12x2 | 64 | release-h19 | 33.623 | 1563 | 643 |
| 12x2 | 64 | release-ordinary | 33.417 | 1608 | 654 |
| 16x2 | 48 | qualified-h19 | 36.049 | 1375 | 768 |
| 16x2 | 48 | release-h19 | 36.296 | 1375 | 786 |
| 16x2 | 48 | release-ordinary | 36.479 | 1420 | 770 |
| 16x2 | 64 | qualified-h19 | 36.980 | 1741 | 775 |
| 16x2 | 64 | release-h19 | 37.214 | 1749 | 755 |
| 16x2 | 64 | release-ordinary | 37.459 | 1796 | 771 |
| 1x2 | 64 | 11x2 | 9.343 | 285 | 53 |
| 1x2 | 64 | 11x4 | 9.071 | 295 | 55 |
| 1x2 | 64 | 12x2 | 8.404 | 274 | 53 |
| 1x2 | 64 | 12x4 | 8.835 | 296 | 63 |
| 1x2 | 64 | aeg64 | 8.753 | 428 | 443 |
| 1x2 | 64 | aeg96 | 8.913 | 447 | 482 |
| 1x2 | 64 | old-four | 9.043 | 294 | 54 |
| 1x2 | 64 | old-normal | 8.353 | 272 | 56 |
| 1x2 | 64 | pool16 | 8.410 | 273 | 53 |
| 1x2 | 64 | release-front | 8.444 | 274 | 56 |
| 1x2 | 80 | 11x2 | 9.660 | 339 | 51 |
| 1x2 | 80 | 11x4 | 8.718 | 343 | 50 |
| 1x2 | 80 | 12x2 | 9.532 | 333 | 54 |
| 1x2 | 80 | 12x4 | 8.821 | 350 | 57 |
| 1x2 | 80 | old-four | 8.752 | 343 | 51 |
| 1x2 | 80 | old-normal | 9.647 | 335 | 51 |
| 1x2 | 96 | 11x2 | 9.700 | 397 | 51 |
| 1x2 | 96 | 11x4 | 9.070 | 411 | 50 |
| 1x2 | 96 | 12x2 | 10.013 | 403 | 53 |
| 1x2 | 96 | 12x4 | 9.239 | 424 | 53 |
| 1x2 | 96 | aeg96 | 7.877 | 516 | 473 |
| 1x2 | 96 | old-four | 9.059 | 411 | 55 |
| 1x2 | 96 | old-normal | 10.173 | 407 | 51 |
| 1x2 | 96 | pool16 | 10.171 | 405 | 54 |
| 1x2 | 96 | release-front | 9.918 | 399 | 55 |
| 24x2 | 48 | qualified-h19 | 52.544 | 2042 | 1244 |
| 24x2 | 48 | release-h19 | 50.988 | 2001 | 1217 |
| 24x2 | 48 | release-ordinary | 52.122 | 2082 | 1223 |
| 24x2 | 64 | qualified-h19 | 51.733 | 2421 | 1247 |
| 24x2 | 64 | release-h19 | 51.368 | 2401 | 1226 |
| 24x2 | 64 | release-ordinary | 52.791 | 2500 | 1238 |
| 2x2 | 48 | 11x2 | 14.280 | 344 | 101 |
| 2x2 | 48 | 11x4 | 13.680 | 352 | 96 |
| 2x2 | 48 | 12x2 | 14.088 | 340 | 105 |
| 2x2 | 48 | 12x4 | 13.599 | 353 | 103 |
| 2x2 | 48 | pool16 | 14.045 | 341 | 102 |
| 2x2 | 48 | release-front | 14.212 | 345 | 105 |
| 4x2 | 48 | release-ordinary | 17.853 | 641 | 281 |
| 8x2 | 48 | release-ordinary | 24.555 | 972 | 450 |
| merge-24x1 | 48 | qualified-h19 | 54.592 | 1985 | 1114 |
| merge-24x1 | 48 | release-h19 | 54.219 | 1963 | 1114 |
| merge-24x1 | 48 | release-ordinary | 55.272 | 2070 | 1127 |
| merge-24x1 | 64 | qualified-h19 | 54.052 | 2382 | 1105 |
| merge-24x1 | 64 | release-h19 | 53.512 | 2367 | 1155 |
| merge-24x1 | 64 | release-ordinary | 55.539 | 2483 | 1127 |

[Protocol](artifacts/protocol.md) · [Configuration](artifacts/settings.json) · [Process and request data](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

# Zopro and Aeglos on the same C7a instance

The 1×2 rows use seven fresh processes and 101 warm, individually timed SDK requests per process. Median latency below means the median of the seven process means. Request percentiles pool the 707 individual requests. Peak RSS includes the full process.

| Physical cores | Implementation and settings | Median ms | Request p50 / p95 / p99 ms | Requests ≤9 ms | Max RSS MiB |
|---:|---|---:|---:|---:|---:|
| 64 | Zopro integrated, frontend 16 | 8.444 | 8.314 / 8.674 / 11.671 | 677/707 | 56 |
| 64 | Zopro experimental, 12-bit / 2 G2 ranges | 8.404 | 8.338 / 8.612 / 11.594 | 688/707 | 53 |
| 64 | Zopro experimental, 11-bit / 2 G2 ranges | 9.343 | 9.622 / 10.174 / 10.325 | 319/707 | 53 |
| 64 | Zopro experimental, 12-bit / 4 G2 ranges | 8.835 | 8.834 / 9.197 / 9.303 | 544/707 | 63 |
| 64 | Zopro experimental, 11-bit / 4 G2 ranges | 9.071 | 8.816 / 9.816 / 10.043 | 447/707 | 55 |
| 64 | Aeglos, native 64 / Go 64 | 8.753 | 8.770 / 9.420 / 9.758 | 521/707 | 443 |
| 64 | Aeglos, native 96 / Go 96 | 8.913 | 8.882 / 9.472 / 9.902 | 470/707 | 482 |
| 96 | Zopro integrated, frontend 16 | 9.918 | 8.864 / 12.256 / 12.454 | 414/707 | 55 |
| 96 | Zopro experimental, 12-bit / 2 G2 ranges | 10.013 | 8.880 / 12.237 / 12.461 | 392/707 | 53 |
| 96 | Zopro experimental, 11-bit / 2 G2 ranges | 9.700 | 8.999 / 12.213 / 12.442 | 356/707 | 51 |
| 96 | Zopro experimental, 12-bit / 4 G2 ranges | 9.239 | 8.571 / 11.277 / 11.559 | 499/707 | 53 |
| 96 | Zopro experimental, 11-bit / 4 G2 ranges | 9.070 | 8.520 / 11.041 / 11.294 | 516/707 | 50 |
| 96 | Aeglos, native 96 / Go 96 | 7.877 | 7.866 / 8.458 / 8.854 | 703/707 | 473 |

## Which G2 change helps at 96 cores?

At 64 cores, going from two to four ranges with the window fixed at 12 bits changed latency by +5.02% (0/7 faster). With four ranges, switching from 12-bit to 11-bit windows changed it by +2.26% (0/7 faster).

At 80 cores, going from two to four ranges with the window fixed at 12 bits changed latency by -7.01% (7/7 faster). With four ranges, switching from 12-bit to 11-bit windows changed it by -1.17% (4/7 faster).

At 96 cores, going from two to four ranges with the window fixed at 12 bits changed latency by -7.54% (7/7 faster). With four ranges, switching from 12-bit to 11-bit windows changed it by -1.71% (6/7 faster).

The factorial measures changes in full-proof latency. It does not isolate a hardware-level cause such as DRAM bandwidth or cache misses.

[Full eight-shape scorecard and all settings](README.md) · [Process and request data](artifacts/processes.json.gz)

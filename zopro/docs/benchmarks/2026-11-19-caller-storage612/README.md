# Calling-thread allocation for packed FFT scratch

The table compares allocating packed output inside product evaluation with allocating it on the calling thread before dispatch. Each setting uses the same-size buffer. Latencies are medians of five process means. Faults cover the timed loop and peak RSS includes setup.

| Shape | Request | Pool | Ordinary / caller ms | Paired change | Minor faults ordinary / caller | Peak MiB ordinary / caller | Caller vs M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | main | 28.879 / 28.619 | -0.71% (4/5) | 3,305 / 3,404 | 589.8 / 581.0 | 9.92× / 3.53× |
| 12x2 | Original | auxiliary | 28.812 / 29.252 | +1.22% (1/5) | 913 / 1,185 | 458.8 / 454.0 | 9.71× / 3.45× |
| 12x2 | 57 | main | 28.846 / 28.476 | +0.70% (2/5) | 3,396 / 3,121 | 579.4 / 574.4 | — |
| 12x2 | 57 | auxiliary | 28.718 / 28.866 | +0.53% (0/5) | 1,257 / 1,077 | 456.1 / 458.7 | — |
| 24x2 | Original | main | 59.114 / 58.588 | -0.89% (3/5) | 9,671 / 6,217 | 1097.3 / 1104.6 | 9.01× / 3.00× |
| 24x2 | Original | auxiliary | 60.971 / 59.559 | -3.99% (5/5) | 19,274 / 3,611 | 922.3 / 944.1 | 8.87× / 2.96× |
| 24x2 | 103 | main | 57.999 / 58.369 | +0.51% (2/5) | 5,695 / 6,033 | 1087.4 / 1119.6 | — |
| 24x2 | 103 | auxiliary | 59.356 / 57.154 | -3.70% (5/5) | 19,234 / 2,784 | 917.5 / 924.9 | — |

All 104 exports verified independently. [Protocol](protocol.md) · [Per-process timings, allocation receipts and frozen controls](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Fault profile](../2026-11-19-product-faults609/README.md)

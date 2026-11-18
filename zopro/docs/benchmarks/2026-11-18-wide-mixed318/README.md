# Trading another dense H scalar window for wider buckets

The 18-window candidate uses sixteen 14-bit and two 15-bit fields. The 17-window candidate uses sixteen 15-bit fields and one 14-bit field. Each uses the existing scratch reservation and affine-bucket implementation. Timing uses five randomized fresh-process blocks with three warmups and eleven SDK requests per process on 16 physical Ryzen cores. The primary comparison is merge24 against 19 windows.

| Shape | Windows | Median ms | Paired vs 19 | Wins / 5 | Median CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| merge-24x1 | reference19 | 69.378 | +1.74% | 0 | 1004 | 868 |
| merge-24x1 | 19 | 68.033 | +0.00% | — | 994 | 882 |
| merge-24x1 | uniform14 | 70.559 | +3.06% | 0 | 1040 | 874 |
| merge-24x1 | 18 | 68.890 | +1.42% | 1 | 1000 | 889 |
| merge-24x1 | 17 | 72.384 | +6.80% | 0 | 1040 | 906 |
| 16x2 | reference19 | 44.586 | -0.70% | 5 | 652 | 597 |
| 16x2 | 19 | 44.968 | +0.00% | — | 656 | 603 |
| 16x2 | uniform14 | 45.194 | +1.84% | 1 | 670 | 603 |
| 16x2 | 18 | 45.268 | +1.13% | 2 | 660 | 618 |
| 16x2 | 17 | 46.281 | +2.55% | 0 | 683 | 609 |
| 24x2 | reference19 | 71.061 | -0.06% | 3 | 1052 | 939 |
| 24x2 | 19 | 71.031 | +0.00% | — | 1051 | 987 |
| 24x2 | uniform14 | 72.817 | +2.51% | 0 | 1081 | 959 |
| 24x2 | 18 | 71.523 | +1.57% | 1 | 1067 | 960 |
| 24x2 | 17 | 72.298 | +3.21% | 0 | 1091 | 946 |
| 12x2 | reference19 | 37.523 | -2.78% | 4 | 546 | 511 |
| 12x2 | 19 | 38.369 | +0.00% | — | 549 | 508 |
| 12x2 | uniform14 | 38.222 | -2.00% | 3 | 566 | 523 |
| 12x2 | 18 | 37.404 | -2.89% | 5 | 554 | 511 |
| 12x2 | 17 | 38.937 | +2.76% | 1 | 576 | 528 |


All 100 process exports passed independent verification. The generator-sum unit test covers all four mixed layouts including high-bit scalars, zero scalars and identity bases. The primary 18-window screen did not pass. The patch applies to experiment 315.

# Smaller frontend pool for the general folded CPU path

The general folded path can dispatch witness solving and scalar gathering, the product FFT, or both to the same caller-owned pool. The benchmark uses eight frontend workers and 16 physical main-pool workers within the same affinity mask. The existing opt-in canonical path and the elliptic-curve stages retain their own scheduling.

| Shape | Setting | Median ms | Paired vs off | Wins / 5 | CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 8x2 | reference | 26.574 | +0.63% | 2 | 390 | 357 |
| 8x2 | off | 26.493 | +0.00% | — | 390 | 364 |
| 8x2 | solver | 27.341 | +3.26% | 1 | 367 | 385 |
| 8x2 | fft | 27.966 | +4.51% | 0 | 368 | 321 |
| 8x2 | both | 28.350 | +6.70% | 0 | 338 | 340 |
| 4x2 | reference | 17.121 | +1.17% | 1 | 238 | 212 |
| 4x2 | off | 16.956 | +0.00% | — | 237 | 214 |
| 4x2 | solver | 17.054 | +0.91% | 1 | 223 | 229 |
| 4x2 | fft | 17.455 | +3.27% | 1 | 224 | 185 |
| 4x2 | both | 17.453 | +2.40% | 0 | 210 | 197 |
| 12x2 | reference | 37.437 | +0.71% | 1 | 548 | 508 |
| 12x2 | off | 37.681 | +0.00% | — | 547 | 532 |
| 12x2 | solver | 38.912 | +4.04% | 1 | 516 | 538 |
| 12x2 | fft | 41.826 | +11.62% | 0 | 513 | 458 |
| 12x2 | both | 43.035 | +14.04% | 0 | 478 | 590 |
| 16x2 | reference | 45.098 | +1.29% | 1 | 648 | 586 |
| 16x2 | off | 44.481 | +0.00% | — | 646 | 597 |
| 16x2 | solver | 45.310 | +1.77% | 0 | 607 | 618 |
| 16x2 | fft | 48.308 | +9.04% | 0 | 609 | 547 |
| 16x2 | both | 50.699 | +13.37% | 0 | 564 | 645 |
| 24x2 | reference | 71.332 | +0.37% | 1 | 1051 | 974 |
| 24x2 | off | 70.796 | +0.00% | — | 1049 | 982 |
| 24x2 | solver | 72.837 | +2.96% | 0 | 991 | 978 |
| 24x2 | fft | 86.045 | +21.74% | 0 | 974 | 868 |
| 24x2 | both | 87.354 | +23.60% | 0 | 910 | 818 |
| merge-24x1 | reference | 68.391 | +0.51% | 0 | 997 | 865 |
| merge-24x1 | off | 68.043 | +0.00% | — | 996 | 901 |
| merge-24x1 | solver | 70.043 | +2.60% | 0 | 943 | 907 |
| merge-24x1 | fft | 84.483 | +24.27% | 0 | 926 | 799 |
| merge-24x1 | both | 84.894 | +24.07% | 0 | 865 | 757 |

The primary 8×2 dual-stage screen did not pass. All 150 timing process exports and eight routing exports passed independent verification. TID probes on 8×2 and 16×2 confirmed exactly the requested stages entered the frontend pool in each of the four settings. The patch applies to experiment 317.

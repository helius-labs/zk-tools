# Balanced SMT worker counts on Ryzen

Each configuration uses all 16 physical Ryzen 9950X cores. Additional workers use equal numbers of sibling threads in each L3 group. The sibling placements rotate across the five matched blocks. The 32-worker proof settings, public files and binary are fixed across widths. Each process primed three requests and timed eleven verified SDK calls on its original circuit input. The CPU-ms and software counters are per timed proof. Peak RSS covers the entire process.

| Shape | Workers | Warm ms | Paired vs 32 (wins) | CPU-ms | Minor faults | Context switches | Peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| 12x2 | 16 | 36.040 | +14.95% (0/5) | 531.4 | 2293 | 523 | 488.5 |
| 12x2 | 20 | 34.058 | +8.12% (0/5) | 623.1 | 2660 | 721 | 523.8 |
| 12x2 | 24 | 31.999 | +1.62% (1/5) | 703.2 | 3038 | 690 | 557.5 |
| 12x2 | 28 | 31.217 | -0.67% (4/5) | 778.4 | 3235 | 760 | 557.3 |
| 12x2 | 30 | 31.286 | -0.36% (3/5) | 823.8 | 3183 | 828 | 564.3 |
| 12x2 | 32 | 31.400 | +0.00% (0/5) | 867.6 | 3842 | 829 | 570.8 |
| 16x2 | 16 | 43.401 | +9.49% (0/5) | 639.7 | 2048 | 650 | 576.1 |
| 16x2 | 20 | 41.488 | +5.26% (1/5) | 756.5 | 2979 | 745 | 607.6 |
| 16x2 | 24 | 39.699 | -0.02% (3/5) | 864.8 | 2875 | 811 | 625.9 |
| 16x2 | 28 | 39.620 | -0.11% (3/5) | 1000.1 | 3178 | 961 | 653.5 |
| 16x2 | 30 | 39.576 | -0.16% (3/5) | 1069.9 | 3521 | 1024 | 654.3 |
| 16x2 | 32 | 39.660 | +0.00% (0/5) | 1148.0 | 3735 | 999 | 668.8 |
| 24x2 | 16 | 68.715 | +8.17% (0/5) | 1016.4 | 3208 | 808 | 885.8 |
| 24x2 | 20 | 67.140 | +5.67% (0/5) | 1224.7 | 8371 | 994 | 919.3 |
| 24x2 | 24 | 63.301 | -0.64% (4/5) | 1418.2 | 4308 | 1045 | 989.6 |
| 24x2 | 28 | 63.269 | -0.65% (4/5) | 1628.5 | 4805 | 1080 | 1012.9 |
| 24x2 | 30 | 63.109 | -1.13% (4/5) | 1734.8 | 5389 | 1184 | 1063.1 |
| 24x2 | 32 | 63.953 | +0.00% (0/5) | 1850.0 | 5549 | 1273 | 1009.8 |
| merge-24x1 | 16 | 68.047 | +7.26% (0/5) | 1007.1 | 3752 | 803 | 897.5 |
| merge-24x1 | 20 | 66.620 | +4.43% (0/5) | 1206.4 | 4974 | 982 | 925.9 |
| merge-24x1 | 24 | 63.144 | -0.61% (5/5) | 1390.9 | 4213 | 979 | 977.5 |
| merge-24x1 | 28 | 62.764 | -0.87% (5/5) | 1586.9 | 5481 | 1038 | 1020.8 |
| merge-24x1 | 30 | 62.486 | -1.77% (5/5) | 1688.8 | 5860 | 1128 | 1043.5 |
| merge-24x1 | 32 | 63.443 | +0.00% (0/5) | 1818.5 | 6848 | 1221 | 1053.8 |

3 of four shapes had a smaller pool win at least four of five pairs with a lower paired median. Width is selected per original request for subsequent independent confirmation. The reference row compares 32 workers with itself. All 120 exports verified independently. These results apply to this Ryzen SMT topology. [Settings and CPU masks](artifacts/settings.json) · [Samples](artifacts/processes.json.gz) · [Candidates](artifacts/confirmation_candidates.json) · [Verification](artifacts/verification.json)

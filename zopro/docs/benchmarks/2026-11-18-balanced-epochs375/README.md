# Balancing witness-solver dependency bands by operation count

The schedule builder reuses the union-find buffer for a histogram before it initializes parent indices. It groups operations of the same dependency depth together. Work uses the midpoint of cumulative operation count to assign a depth to a band. Blend averages relative depth and relative operation rank. Outer and inner apply that blended mapping at only the chosen level. Warm proofs use the ordinary task executor. The existing memory accounting charges the actual number and capacity of the resulting tasks.

| Stage | Shape | Selected | Release ms | Disabled ms | Selected ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Peak MiB | M5 CPU / Metal gain |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| confirm | 1x2 | outer | 7.652 | 7.598 | 7.417 | -2.89% | 7/7 | -2.40% | 7/7 | 65 | 4.99× / 3.24× |
| panel | 2x2 | inner | 10.823 | 10.675 | 10.818 | -0.13% | 2/3 | +2.50% | 1/3 | 119 | 5.45× / 2.77× |
| panel | 4x2 | inner | 16.959 | 16.883 | 17.187 | +0.99% | 1/3 | +1.56% | 0/3 | 207 | 6.34× / 2.79× |
| panel | 8x2 | inner | 26.783 | 26.691 | 26.821 | +0.11% | 1/3 | +0.49% | 1/3 | 348 | 7.05× / 2.76× |
| panel | 12x2 | inner | 39.285 | 38.733 | 39.805 | +1.17% | 1/3 | +2.83% | 0/3 | 498 | 7.13× / 2.54× |
| confirm | 16x2 | inner | 45.620 | 45.706 | 46.084 | +1.10% | 2/7 | +0.63% | 1/7 | 630 | 7.27× / 2.69× |
| panel | 24x2 | inner | 73.127 | 73.534 | 74.992 | +2.55% | 1/3 | +1.96% | 0/3 | 951 | 7.04× / 2.35× |
| panel | merge-24x1 | inner | 71.936 | 72.039 | 71.077 | -0.84% | 3/3 | -1.47% | 2/3 | 872 | 7.13× / 2.46× |

## Schedule at key loading

The span sums the longest serial task chain in each outer epoch, counting each operation equally. It is a structural estimate. Preparation and retained bytes come from the solver’s reservation.

| Shape | Policy | General ops | Span | Work / span | Leaves | Epochs | Retained KiB | Preparation peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | off | 31061 | 11500 | 2.70 | 97 | 4 | 182 | 0.72 |
| 1x2 | work | 31061 | 12760 | 2.43 | 91 | 4 | 149 | 0.69 |
| 1x2 | blend | 31061 | 11565 | 2.69 | 132 | 4 | 159 | 0.70 |
| 1x2 | outer | 31061 | 11418 | 2.72 | 124 | 4 | 158 | 0.70 |
| 1x2 | inner | 31061 | 11632 | 2.67 | 138 | 4 | 185 | 0.72 |
| 16x2 | off | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | work | 372731 | 11821 | 31.53 | 5837 | 6 | 2081 | 8.18 |
| 16x2 | blend | 372731 | 13885 | 26.84 | 9380 | 6 | 2494 | 8.59 |
| 16x2 | outer | 372731 | 14017 | 26.59 | 899 | 6 | 1765 | 7.88 |
| 16x2 | inner | 372731 | 13298 | 28.03 | 861 | 6 | 1846 | 7.95 |

The zero optional-preparation allowance process exported an independently verified 1×2 proof. The driver observed 0 balanced-schedule marker and 0 admitted schedule report.

The outer-band change won all seven 1×2 holdout pairs against both controls. On 16×2, the selected policy lost six of seven pairs against the disabled control. Dividing by operation count shortened the modeled 16×2 span by 9.7% but increased the number of leaf tasks from 810 to 5,837. The experiment leaves the release schedule in place.

[Samples](artifacts/processes.json.gz) · [Source](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.md)

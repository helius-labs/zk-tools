# Packing small independent witness-solver tasks

Within each dependency band, adjacent independent components can share a single task vector. The limit counts operations. Packed layouts retain the same order within each component. Five screening blocks selected the packing policy for each holdout shape. The unpacked column uses the same band boundaries as the selected policy. The disabled column uses the release boundaries. Times are medians of per-process mean warm, verified request latencies on a Ryzen 9950X. Paired changes use the independent confirmation or panel blocks.

| Shape | Policy | Release ms | Disabled ms | Unpacked ms | Packed ms | Paired vs unpacked | Wins / pairs | Paired vs release | Wins / pairs | Peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | outer-128 | 7.629 | 7.617 | 7.451 | 7.292 | -2.30% | 7/7 | -4.68% | 7/7 | 66 | 5.07× / 3.29× |
| 2x2 | outer-128 | 10.684 | 10.817 | 10.625 | 10.344 | -4.03% | 3/3 | -3.75% | 3/3 | 117 | 5.70× / 2.90× |
| 4x2 | outer-128 | 16.975 | 16.934 | 17.431 | 17.522 | +0.52% | 1/3 | +2.94% | 0/3 | 210 | 6.22× / 2.74× |
| 8x2 | outer-128 | 26.849 | 26.649 | 26.578 | 26.438 | -0.53% | 2/3 | -1.10% | 3/3 | 353 | 7.15× / 2.80× |
| 12x2 | outer-128 | 38.980 | 39.162 | 39.290 | 38.845 | -0.09% | 2/3 | +0.26% | 1/3 | 490 | 7.31× / 2.60× |
| 16x2 | outer-128 | 45.657 | 45.776 | 45.883 | 45.441 | -1.14% | 6/7 | -0.47% | 6/7 | 631 | 7.37× / 2.73× |
| 24x2 | outer-128 | 72.576 | 72.858 | 74.111 | 73.527 | +0.16% | 1/3 | +0.41% | 1/3 | 975 | 7.18× / 2.39× |
| merge-24x1 | outer-128 | 71.882 | 71.927 | 72.314 | 72.394 | -0.01% | 2/3 | +1.18% | 0/3 | 874 | 7.00× / 2.42× |

On 1×2, packing the blended outer bands reduced the leaf count from 124 to 69. It beat the same unpacked bands and the integrated release in all seven confirmation blocks. Across the holdout processes, peak RSS was 65.75 MiB for the selected schedule and 66.75 MiB for the release. On 16×2, it won six of seven comparisons against each control, with a median paired improvement of 0.47% over release. The 4×2 and merge-24×1 panels favored the release in every block. These Ryzen results support a targeted follow-up. This version has no C7a measurement.

## Schedule at key loading

Mode names combine the dependency band policy and maximum packing size in operations. Span counts each operation equally. The solver charges the actual capacities of the resulting schedule.

| Shape | Policy | Modeled span in operations | Leaf tasks | Retained KiB | Preparation peak MiB |
|---|---|---:|---:|---:|---:|
| 1x2 | off | 11500 | 97 | 182 | 0.72 |
| 1x2 | off-128 | 11500 | 58 | 178 | 0.72 |
| 1x2 | work | 12760 | 91 | 149 | 0.69 |
| 1x2 | work-32 | 12760 | 68 | 147 | 0.69 |
| 1x2 | work-128 | 12760 | 50 | 146 | 0.69 |
| 1x2 | work-512 | 12760 | 38 | 145 | 0.69 |
| 1x2 | outer | 11418 | 124 | 158 | 0.70 |
| 1x2 | outer-128 | 11419 | 69 | 153 | 0.69 |
| 16x2 | off | 13096 | 810 | 1841 | 7.95 |
| 16x2 | off-128 | 13096 | 551 | 1819 | 7.93 |
| 16x2 | work | 11821 | 5837 | 2081 | 8.18 |
| 16x2 | work-32 | 11821 | 883 | 1656 | 7.77 |
| 16x2 | work-128 | 11821 | 564 | 1628 | 7.74 |
| 16x2 | work-512 | 15822 | 357 | 1610 | 7.72 |
| 16x2 | outer | 14017 | 899 | 1765 | 7.88 |
| 16x2 | outer-128 | 14045 | 599 | 1740 | 7.85 |

The zero-preparation-budget process exercised the fallback and exported a verified proof.

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz)

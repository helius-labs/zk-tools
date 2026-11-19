# Distributing witness operations between earliest and latest legal depths

The schedule first computes earliest start depths. A reverse walk computes the latest legal depth without increasing the longest dependency chain. The candidate assigns a fixed fraction of that slack to each operation before building the normal task bands. The reverse pass uses the same buffer subsequently used for union-find. Modes late, mid and quarter use all, one half, or one quarter of the slack. Outer and inner apply the midpoint at just the named level. Packed is the exact packed outer-band build from experiment 377, measured in the same randomized blocks.

| Stage | Shape | Selected | Release ms | Disabled ms | Packed ms | Integrated claims ms | Selected ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Peak MiB | M5 CPU / Metal gain |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| confirm | 1x2 | inner | 7.661 | 7.606 | 7.279 | 7.332 | 7.786 | +2.60% | 1/7 | +2.21% | 1/7 | 72 | 4.75× / 3.08× |
| panel | 2x2 | inner | 10.748 | 10.792 | 10.285 | 10.233 | 11.026 | +3.44% | 0/3 | +3.56% | 0/3 | 118 | 5.35× / 2.72× |
| panel | 4x2 | inner | 16.952 | 16.846 | 17.354 | 16.905 | 17.497 | +3.37% | 0/3 | +3.83% | 0/3 | 200 | 6.23× / 2.74× |
| panel | 8x2 | inner | 26.756 | 26.707 | 26.296 | 26.686 | 27.781 | +3.34% | 0/3 | +4.10% | 0/3 | 348 | 6.80× / 2.66× |
| panel | 12x2 | inner | 38.502 | 39.639 | 38.263 | 38.594 | 40.993 | +6.49% | 0/3 | +4.83% | 0/3 | 490 | 6.93× / 2.46× |
| confirm | 16x2 | inner | 45.600 | 45.633 | 45.610 | 45.797 | 47.352 | +3.86% | 1/7 | +3.49% | 0/7 | 646 | 7.07× / 2.62× |
| panel | 24x2 | inner | 73.003 | 73.445 | 73.571 | 72.672 | 78.365 | +7.77% | 0/3 | +7.12% | 0/3 | 934 | 6.74× / 2.25× |
| panel | merge-24x1 | inner | 71.274 | 71.634 | 73.068 | 71.060 | 72.362 | +1.84% | 1/3 | +1.02% | 1/3 | 893 | 7.01× / 2.42× |

The lowest-latency slack policy in both screens shifted only inner bands. In the independent 16×2 holdout, it took 47.352 ms versus 45.633 ms disabled and lost all seven pairs. Its modeled span fell from 13,096 operations to 11,380, while leaf count rose from 810 to 13,525. On 1×2 it took 7.786 ms versus 7.332 ms for integrated task claiming. The release keeps the existing band assignment.

## Schedule at key loading

The span sums the longest serial task chain in each outer epoch, counting each operation equally. It is a structural estimate. Preparation and retained bytes come from the solver’s reservation.

| Shape | Policy | General ops | Span | Work / span | Leaves | Epochs | Retained KiB | Preparation peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | off | 31061 | 11500 | 2.70 | 97 | 4 | 182 | 0.72 |
| 1x2 | quarter | 31061 | 9922 | 3.13 | 1334 | 4 | 236 | 0.78 |
| 1x2 | mid | 31061 | 10217 | 3.04 | 1458 | 4 | 247 | 0.79 |
| 1x2 | late | 31061 | 11095 | 2.80 | 166 | 4 | 176 | 0.72 |
| 1x2 | outer | 31061 | 10217 | 3.04 | 1458 | 4 | 247 | 0.79 |
| 1x2 | inner | 31061 | 10746 | 2.89 | 642 | 4 | 228 | 0.77 |
| 1x2 | packed | 31061 | 11419 | 2.72 | 69 | 4 | 153 | 0.69 |
| 16x2 | off | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | quarter | 372731 | 12136 | 30.71 | 4875 | 6 | 2042 | 8.15 |
| 16x2 | mid | 372731 | 10198 | 36.55 | 16342 | 6 | 2878 | 8.96 |
| 16x2 | late | 372731 | 12755 | 29.22 | 19222 | 6 | 3314 | 9.39 |
| 16x2 | outer | 372731 | 10198 | 36.55 | 16341 | 6 | 2878 | 8.96 |
| 16x2 | inner | 372731 | 11380 | 32.75 | 13525 | 6 | 2934 | 9.02 |
| 16x2 | packed | 372731 | 14045 | 26.54 | 599 | 6 | 1740 | 7.85 |

The zero optional-preparation allowance process exported an independently verified 1×2 proof. The driver observed 0 shifted-schedule marker and 0 admitted schedule report.

[Samples](artifacts/processes.json.gz) · [Source](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.md)

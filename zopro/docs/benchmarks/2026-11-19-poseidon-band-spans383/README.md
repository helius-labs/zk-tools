# Keeping structural Poseidon spans inside solver bands

The existing coefficient checks still determine specialized execution. The preparation candidate treats structurally matching contiguous operation sequences as single units for determining band boundaries. Each unit gets a duration of one, half its length, or its full length. The depth assignment uses the center of the unit. Its external consumers use its finish depth. All original operations and dependency checks remain. The search borrows the current operation-order buffer and reserves its fixed span-recognition workspace. The latency table compares exact release, the same binary disabled, the previous packed build, and the selected variant.

| Stage | Shape | Selected | Release ms | Disabled ms | Packed ms | Integrated claims ms | Selected ms | Paired vs release | Wins / pairs | Paired vs disabled | Wins / pairs | Peak MiB | M5 CPU / Metal gain |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| confirm | 1x2 | both-half | 7.599 | 7.640 | 7.334 | 7.353 | 7.432 | -2.04% | 7/7 | -2.83% | 7/7 | 65 | 4.98× / 3.23× |
| panel | 2x2 | outer-length | 10.770 | 10.692 | 10.218 | 10.212 | 10.688 | -0.76% | 3/3 | +0.07% | 1/3 | 118 | 5.52× / 2.81× |
| panel | 4x2 | outer-length | 16.863 | 16.927 | 17.266 | 16.925 | 16.926 | +0.65% | 1/3 | -0.43% | 2/3 | 202 | 6.44× / 2.84× |
| panel | 8x2 | outer-length | 26.708 | 26.324 | 26.480 | 26.514 | 26.647 | +0.42% | 1/3 | +1.33% | 1/3 | 362 | 7.09× / 2.78× |
| panel | 12x2 | outer-length | 38.683 | 38.586 | 38.446 | 38.552 | 38.702 | +0.05% | 1/3 | -0.17% | 2/3 | 495 | 7.34× / 2.61× |
| confirm | 16x2 | outer-length | 45.401 | 45.662 | 45.351 | 45.572 | 45.799 | +0.75% | 1/7 | -0.04% | 4/7 | 634 | 7.31× / 2.71× |
| panel | 24x2 | outer-length | 73.725 | 72.734 | 72.622 | 73.563 | 73.846 | -0.82% | 2/3 | +1.53% | 1/3 | 954 | 7.15× / 2.38× |
| panel | merge-24x1 | outer-length | 72.020 | 71.036 | 72.304 | 70.967 | 71.303 | -0.15% | 2/3 | +0.38% | 1/3 | 890 | 7.11× / 2.45× |

On 1×2, the selected policy increased recognized width-three spans from 66 to 71 and took 7.432 ms. The integrated task-claim option measured 7.353 ms and the packed candidate measured 7.334 ms in the same seven blocks. On 16×2 the screen selected outer-length. It found zero contiguous structural spans before grouping, so its 45.799 ms result measures the ordinary warm schedule. Inner matching found only 285 candidate operations among 372,731. These results favor working directly with the many spans that the existing grouped schedule already recognizes.

## Schedule at key loading

The span sums the longest serial task chain in each outer epoch, counting each operation equally. It is a structural estimate. Preparation and retained bytes come from the solver’s reservation.

| Shape | Policy | General ops | Span | Work / span | Leaves | Epochs | Retained KiB | Preparation peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | off | 31061 | 11500 | 2.70 | 97 | 4 | 182 | 0.72 |
| 1x2 | outer-unit | 31061 | 11501 | 2.70 | 78 | 4 | 159 | 0.70 |
| 1x2 | outer-half | 31061 | 11556 | 2.69 | 97 | 4 | 180 | 0.72 |
| 1x2 | outer-length | 31061 | 11478 | 2.71 | 97 | 4 | 182 | 0.73 |
| 1x2 | inner-half | 31061 | 11758 | 2.64 | 104 | 4 | 182 | 0.73 |
| 1x2 | both-half | 31061 | 11423 | 2.72 | 106 | 4 | 181 | 0.72 |
| 1x2 | both-length | 31061 | 11326 | 2.74 | 96 | 4 | 182 | 0.73 |
| 1x2 | packed | 31061 | 11419 | 2.72 | 69 | 4 | 153 | 0.69 |
| 16x2 | off | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | outer-unit | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | outer-half | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | outer-length | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | inner-half | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | both-half | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | both-length | 372731 | 13096 | 28.46 | 810 | 6 | 1841 | 7.95 |
| 16x2 | packed | 372731 | 14045 | 26.54 | 599 | 6 | 1740 | 7.85 |

## Structural and fully recognized operations

Structural span operations are tallied independently at outer and inner preparation levels. The final column uses the full coefficient-checked execution tags.

| Shape | Mode | Outer structural ops | Inner structural ops | Tagged ops |
|---|---|---:|---:|---:|
| 1x2 | off | 0 | 0 | 17559 |
| 1x2 | outer-unit | 1404 | 0 | 18126 |
| 1x2 | outer-half | 1404 | 0 | 17610 |
| 1x2 | outer-length | 1404 | 0 | 17793 |
| 1x2 | inner-half | 0 | 3297 | 18309 |
| 1x2 | both-half | 1404 | 2778 | 18543 |
| 1x2 | both-length | 1404 | 3297 | 20388 |
| 1x2 | packed | 0 | 0 | 19866 |
| 16x2 | off | 0 | 0 | 239277 |
| 16x2 | outer-unit | 0 | 0 | 239277 |
| 16x2 | outer-half | 0 | 0 | 239277 |
| 16x2 | outer-length | 0 | 0 | 239277 |
| 16x2 | inner-half | 0 | 285 | 239562 |
| 16x2 | both-half | 0 | 285 | 239562 |
| 16x2 | both-length | 0 | 285 | 239562 |
| 16x2 | packed | 0 | 0 | 248340 |

The zero optional-preparation allowance process exported an independently verified 1×2 proof. The driver observed 0 span-schedule marker and 0 admitted schedule report.

[Samples](artifacts/processes.json.gz) · [Source](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.md)

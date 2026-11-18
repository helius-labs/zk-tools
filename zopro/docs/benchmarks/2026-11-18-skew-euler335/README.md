# Cache padding between Euler FFT stripes

On merge24 the unmodified path took 70.885 ms. One and three padding elements per stripe took 70.562 and 70.762 ms, with paired gains of 0.46% and 0.29%. Both missed the 2% threshold. On 16×2 both increased latency. The integrated FFT keeps its existing layout.

| Shape | Workers | Mode | Median ms | Paired vs off | Wins | CPU ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 8x2 | 16 | reference | 26.819 | +0.91% | 0/5 | 394.5 | 353 |
| 8x2 | 16 | off | 26.731 | +0.00% | 0/5 | 391.7 | 366 |
| 8x2 | 16 | 0 | 26.809 | +0.54% | 1/5 | 392.7 | 358 |
| 8x2 | 16 | 1 | 26.972 | +0.91% | 1/5 | 395.0 | 367 |
| 8x2 | 16 | 3 | 26.949 | +0.03% | 2/5 | 392.8 | 368 |
| 12x2 | 16 | reference | 37.997 | +1.00% | 2/5 | 557.0 | 513 |
| 12x2 | 16 | off | 38.127 | +0.00% | 0/5 | 557.4 | 506 |
| 12x2 | 16 | 0 | 38.252 | +1.14% | 1/5 | 558.1 | 508 |
| 12x2 | 16 | 1 | 38.107 | -0.35% | 3/5 | 555.9 | 508 |
| 12x2 | 16 | 3 | 38.889 | +2.00% | 1/5 | 558.7 | 512 |
| 16x2 | 16 | reference | 44.270 | -1.02% | 3/5 | 656.9 | 603 |
| 16x2 | 16 | off | 44.728 | +0.00% | 0/5 | 657.9 | 599 |
| 16x2 | 16 | 0 | 46.563 | +4.10% | 1/5 | 663.4 | 581 |
| 16x2 | 16 | 1 | 45.387 | +2.01% | 1/5 | 658.8 | 594 |
| 16x2 | 16 | 3 | 45.450 | +0.90% | 1/5 | 656.0 | 594 |
| 24x2 | 16 | reference | 72.540 | -0.23% | 3/5 | 1077.4 | 990 |
| 24x2 | 16 | off | 72.756 | +0.00% | 0/5 | 1082.0 | 953 |
| 24x2 | 16 | 0 | 73.559 | +2.29% | 1/5 | 1087.3 | 963 |
| 24x2 | 16 | 1 | 72.096 | -0.60% | 3/5 | 1077.3 | 964 |
| 24x2 | 16 | 3 | 72.078 | -0.35% | 4/5 | 1074.7 | 991 |
| merge-24x1 | 16 | reference | 72.399 | +2.76% | 1/5 | 1042.6 | 883 |
| merge-24x1 | 16 | off | 70.885 | +0.00% | 0/5 | 1030.6 | 892 |
| merge-24x1 | 16 | 0 | 70.996 | +0.85% | 2/5 | 1032.3 | 907 |
| merge-24x1 | 16 | 1 | 70.562 | -0.46% | 3/5 | 1031.1 | 888 |
| merge-24x1 | 16 | 3 | 70.762 | -0.29% | 3/5 | 1033.5 | 891 |

All 125 timed processes and five separate routing checks exported proofs that verified against the pinned public witness. Zero optional memory selected the existing fallback. Timings are medians of five process means. Peak memory is the largest process RSS in each cell.

[Protocol](artifacts/protocol.md) · [Process data](artifacts/processes.json.gz) · [Route counts](artifacts/routing.json) · [Verification](artifacts/verification.json)

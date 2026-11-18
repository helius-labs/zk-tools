# Isolating instruction execution at fixed main-pool width

All rows use the same experimental executable, eight solver epochs and the two-range H layout. The active and idle configurations within each pair have identical main and auxiliary worker counts. The instrumented evaluator printed its team width from inside its execution closure. The complete SDK still returns a verified proof.

| Shape | Main cores | Auxiliary workers | Idle ms | Active ms | Paired change | Faster blocks | Idle / active CPU-ms | Idle / active peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 32 | 33.992 | 34.638 | +1.66% | 0/5 | 1646 / 1529 | 737 / 692 |
| 16x2 | 64 | 48 | 35.172 | 35.179 | -0.17% | 3/5 | 1696 / 1581 | 726 / 670 |
| 16x2 | 96 | 48 | 36.666 | 37.382 | +2.72% | 2/5 | 2498 / 2227 | 729 / 704 |
| 16x2 | 96 | 64 | 36.858 | 36.268 | -2.76% | 5/5 | 2516 / 2308 | 754 / 679 |
| merge-24x1 | 64 | 32 | 49.823 | 48.564 | -2.76% | 5/5 | 2323 / 2110 | 1040 / 968 |
| merge-24x1 | 64 | 48 | 50.430 | 48.276 | -3.71% | 4/5 | 2329 / 2178 | 1042 / 928 |
| merge-24x1 | 96 | 48 | 53.524 | 50.144 | -6.31% | 5/5 | 3494 / 2962 | 1048 / 960 |
| merge-24x1 | 96 | 64 | 54.433 | 49.794 | -7.65% | 5/5 | 3519 / 3078 | 1068 / 953 |

## Controls without auxiliary workers

| Shape | Cores | Integrated release ms | Experiment disabled ms | Difference |
|---|---:|---:|---:|---:|
| 16x2 | 64 | 34.291 | 34.210 | -0.22% |
| 16x2 | 96 | 37.286 | 36.463 | -1.50% |
| merge-24x1 | 64 | 49.852 | 48.992 | -2.70% |
| merge-24x1 | 96 | 52.962 | 53.097 | +0.18% |

[Local route qualification](../2026-11-18-interpreter-team356/README.md) · [Primary panel](README.md)

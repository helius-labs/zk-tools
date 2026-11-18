# Claiming ready solver tasks within dependency waves

The experimental solver estimates the amount of parallel work available in a wave as its number of operations divided by its largest task, rounded up. This bounds the number of Rayon jobs that take successive tasks from one proof-local atomic counter. The existing dependency order between waves is preserved. The leaves setting keeps the original outer iteration for refined tasks, both applies the policy at both levels, and twice doubles the estimated parallelism at both levels. The comparison keeps the integrated SDK and the same binary with the option disabled.

| Stage | Shape | Mode | Release ms | Disabled ms | Candidate ms | Paired vs release | Wins / pairs | Paired vs disabled | Peak MiB | M5 CPU / Metal gain |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| confirm | 1x2 | twice | 7.616 | 7.616 | 7.374 | -4.18% | 6/7 | -3.40% | 92 | 5.02× / 3.25× |
| panel | 2x2 | twice | 10.748 | 10.711 | 10.679 | -0.65% | 2/3 | -2.62% | 117 | 5.53× / 2.81× |
| panel | 4x2 | twice | 16.958 | 16.920 | 16.942 | -0.09% | 2/3 | +0.05% | 211 | 6.43× / 2.83× |
| panel | 8x2 | twice | 26.701 | 26.634 | 27.170 | +1.76% | 1/3 | +1.14% | 355 | 6.96× / 2.72× |
| panel | 12x2 | twice | 39.220 | 39.040 | 38.526 | -1.93% | 2/3 | -0.15% | 494 | 7.37× / 2.62× |
| confirm | 16x2 | twice | 45.676 | 45.752 | 45.863 | +0.22% | 3/7 | +0.35% | 636 | 7.30× / 2.70× |
| panel | 24x2 | twice | 73.563 | 73.510 | 73.515 | -0.20% | 2/3 | +0.02% | 956 | 7.18× / 2.39× |
| panel | merge-24x1 | twice | 71.850 | 71.676 | 72.038 | -0.66% | 2/3 | +0.50% | 839 | 7.04× / 2.43× |

[Solver CPU profiles on C7a](quintiles.md) · [Raw process timings](artifacts/local.json.gz) · [Source change](artifacts/source.patch.gz)

[Matched C7a comparison](../2026-11-18-claims-c7a374/README.md)

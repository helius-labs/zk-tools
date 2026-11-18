# Scaling the H query and varying frontend workers on C7a

All 238 primary processes ran in seven randomized blocks on one c7a.24xlarge. The H-query experiment divides work within seven or nineteen windows while preserving the 19-window layout. All larger-shape cells have 11 timed requests per process. Peak memory is process-lifetime high-water RSS. CPU time is aggregate user and system time per proof. Negative paired changes mean lower latency.

On merge24 at 96 cores, splitting the seven wide H windows changed latency by -0.97% (6/7 faster blocks). On 1×2 at 96 cores, reducing the frontend from 16 to eight workers changed latency by +4.82% (0/7 faster blocks).

## H-query work at matched core counts

| Shape | Cores | Setting | Median ms | Paired vs fresh off | Faster blocks | CPU-ms/proof | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 16x2 | 64 | integrated | 36.651 | -1.21% | 5/7 | 1735.7 | 721 |
| 16x2 | 64 | off | 37.266 | +0.00% | 0/7 | 1772.6 | 735 |
| 16x2 | 64 | wide | 37.030 | -1.37% | 5/7 | 1807.0 | 736 |
| 16x2 | 64 | all | 35.752 | -4.27% | 7/7 | 1799.9 | 731 |
| 16x2 | 96 | integrated | 39.624 | -3.05% | 7/7 | 2644.6 | 722 |
| 16x2 | 96 | off | 41.065 | +0.00% | 0/7 | 2836.8 | 732 |
| 16x2 | 96 | wide | 40.387 | -1.82% | 6/7 | 2806.6 | 750 |
| 16x2 | 96 | all | 38.455 | -6.35% | 7/7 | 2782.5 | 723 |
| 24x2 | 64 | integrated | 51.285 | -0.30% | 4/7 | 2393.3 | 1148 |
| 24x2 | 64 | off | 51.717 | +0.00% | 0/7 | 2438.4 | 1160 |
| 24x2 | 64 | wide | 51.187 | +0.73% | 3/7 | 2455.0 | 1155 |
| 24x2 | 64 | all | 47.441 | -7.90% | 7/7 | 2451.3 | 1114 |
| 24x2 | 96 | integrated | 53.304 | -5.44% | 7/7 | 3482.2 | 1137 |
| 24x2 | 96 | off | 56.747 | +0.00% | 0/7 | 3735.3 | 1180 |
| 24x2 | 96 | wide | 54.670 | -3.24% | 7/7 | 3730.9 | 1148 |
| 24x2 | 96 | all | 52.564 | -7.37% | 7/7 | 3698.3 | 1192 |
| merge-24x1 | 64 | integrated | 53.877 | -2.31% | 7/7 | 2384.9 | 1036 |
| merge-24x1 | 64 | off | 55.015 | +0.00% | 0/7 | 2450.9 | 1037 |
| merge-24x1 | 64 | wide | 55.143 | +0.25% | 3/7 | 2488.0 | 1070 |
| merge-24x1 | 64 | all | 51.666 | -6.83% | 6/7 | 2478.5 | 1049 |
| merge-24x1 | 96 | integrated | 58.969 | -2.35% | 7/7 | 3579.0 | 1052 |
| merge-24x1 | 96 | off | 60.284 | +0.00% | 0/7 | 3813.0 | 1030 |
| merge-24x1 | 96 | wide | 59.325 | -0.97% | 6/7 | 3791.4 | 1077 |
| merge-24x1 | 96 | all | 55.648 | -7.08% | 6/7 | 3803.6 | 1087 |

## Two ranges in every H window against the integrated release

The comparison below uses the unchanged integrated executable as the baseline. The full table also gives the same-binary disabled control. The candidate and integrated control use different executable backings.

| Shape | Cores | Release ms | Candidate ms | Paired change | Faster blocks | Peak MiB release / candidate | M5 CPU / Metal speedup on candidate |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 36.651 | 35.752 | -2.64% | 5/7 | 721 / 731 | 9.37× / 3.47× |
| 16x2 | 96 | 39.624 | 38.455 | -3.20% | 6/7 | 722 / 723 | 8.71× / 3.22× |
| 24x2 | 64 | 51.285 | 47.441 | -7.96% | 7/7 | 1148 / 1114 | 11.13× / 3.71× |
| 24x2 | 96 | 53.304 | 52.564 | -2.16% | 7/7 | 1137 / 1192 | 10.04× / 3.35× |
| merge-24x1 | 64 | 53.877 | 51.666 | -3.70% | 6/7 | 1036 / 1049 | 9.81× / 3.39× |
| merge-24x1 | 96 | 58.969 | 55.648 | -5.50% | 6/7 | 1052 / 1087 | 9.11× / 3.14× |

## Frontend workers for 1×2

The 64-core setting uses two G2 ranges and parallel splitting. The 96-core setting uses four ranges and serial splitting. Each process times 51 proofs. The main pool covers the full physical mask. Any additional frontend workers are pinned inside it. Every process output from this repeat was archived and independently verified.

| Cores | Frontend workers | Median ms | Paired vs 16 | Faster blocks | Request p95 ms | CPU-ms/proof | Peak MiB |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | 0 | 9.507 | +13.24% | 0/7 | 9.822 | 376.7 | 63 |
| 64 | 8 | 8.890 | +6.10% | 0/7 | 9.081 | 265.7 | 49 |
| 64 | 16 | 8.379 | +0.00% | 0/7 | 8.602 | 273.5 | 49 |
| 64 | 24 | 8.510 | +1.64% | 1/7 | 8.776 | 284.9 | 50 |
| 64 | 32 | 8.549 | +2.27% | 0/7 | 8.907 | 293.9 | 51 |
| 96 | 0 | 10.052 | +18.18% | 0/7 | 10.407 | 551.6 | 56 |
| 96 | 8 | 8.907 | +4.82% | 0/7 | 9.184 | 392.3 | 43 |
| 96 | 16 | 8.509 | +0.00% | 0/7 | 8.837 | 388.6 | 50 |
| 96 | 24 | 8.611 | +1.61% | 1/7 | 9.022 | 400.5 | 49 |
| 96 | 32 | 8.670 | +1.88% | 0/7 | 8.973 | 405.9 | 54 |

[48 through 96 cores with split H windows](crossover.md) · [Stage profiles for 16×2 and merge24](stages.md) · [Eight-shape scaling comparison and Aeglos](../2026-11-18-release-panel351/README.md) · [Protocol](artifacts/protocol.md) · [Individual process and request data](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

All 358 timing exports and 24 profiling exports passed independent verification on the instance and again after retrieval. The archive listed every named timing output. The instance terminated, the tagged EBS volume count was zero and the temporary bucket was removed.

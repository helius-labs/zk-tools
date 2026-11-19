# Memory and CPU cost of optional prepared point groups

The prototype stores group member indices, end offsets, scalar-source indices and a compact point array for each curve query. For the public 12×2 plans their combined vector payload is 22.67 MiB. The conservatively charged lifetime preparation reservation totals 23.38 MiB. These quantities exclude the existing key, allocator overhead, temporary setup allocations and proof-local buffers. The charges cover one prepared prover.

| Query | Member indices MiB | Ends MiB | Sources MiB | Compact points MiB | Charged MiB |
|---|---:|---:|---:|---:|---:|
| a | 0.53 | 0.09 | 0.25 | 3.97 | 5.03 |
| k | 0.79 | 0.14 | 0.33 | 5.30 | 6.82 |
| b2 | 0.79 | 0.14 | 0.31 | 10.04 | 11.52 |

The operating-system rows below compare the enabled and disabled modes of the same binary in the same randomized block. Differences are first summarized by the median of three pairs per input, then by the median across inputs in each population. Peak RSS is the lifetime process maximum. CPU time, minor faults and context switches cover the timed loop and are divided by the number of proofs. These counters do not identify individual allocations or the cause of a latency change.

| Experiment | Population | Inputs | Wall ms | Peak RSS MiB | CPU-ms | Minor faults/proof | Voluntary switches/proof | Involuntary switches/proof |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| scattered-transfer485 | original | 1 | -1.04 | +45.61 | -16.79 | +577.36 | -4.64 | +2.82 |
| scattered-transfer485 | scattered | 8 | +0.80 | +10.90 | +21.09 | -381.45 | +74.55 | +2.50 |
| sampled-groups487 | original | 1 | -1.39 | +75.80 | -37.87 | +986.09 | -17.36 | +1.64 |
| sampled-groups487 | adjacent | 8 | -1.58 | +55.66 | -34.27 | +516.77 | -7.95 | +1.77 |
| sampled-groups487 | scattered | 8 | +0.18 | -0.08 | +6.96 | -500.95 | -3.00 | -1.95 |

The [full-validation trial](../2026-11-19-scattered-transfer485/README.md) and [sampled trial](../2026-11-19-sampled-groups487/README.md) used different binaries. Each row has its own paired control. All numbers come from the original verified processes. [Budget calculation](artifacts/budget.json) · [Paired outcomes](artifacts/paired.json) · [Process counters](artifacts/operating-system.json.gz)

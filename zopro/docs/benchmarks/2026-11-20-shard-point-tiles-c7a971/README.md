# Parallel shard point reduction on C7a 4×2

Tiling K and B2 together reduced 4×2 proof latency to **14.424 ms** on catalog and **14.350 ms** on held-out. It was 1.72% and 2.54% faster than ordinary in paired medians, winning 11/12 and 12/12 blocks. It was 2.22% and 2.21% faster than frozen, winning 11/12 on both inputs. The joint run met the full-proof repeatability criterion. The [Ryzen run](../2026-11-20-shard-point-tiles-ryzen970/README.md) shows a regression with the same code.

K and B2 classify scalars within sixteen shards. The point reducer schedules jobs of up to 256 classes across those shards. Each job writes to its own range of the final point and scalar arrays. The existing window MSM receives the compact rows in the same shard order as the prior serial compactor. The trial used 64 main, 32 frontend and 16 solver workers on c7a.24xlarge. [Source and preceding trial](../2026-11-20-shard-point-tiles-ryzen970/README.md)

| Input | Mode | Full proof ms | Paired vs ordinary | Paired vs frozen | Process peak MiB |
|---|---|---:|---:|---:|---:|
| Catalog | Frozen | 14.753 | — | — | 219.2 |
| Catalog | Ordinary | 14.710 | — | -0.38% (7/12) | 217.7 |
| Catalog | Prior serial B2 | 15.215 | +3.22% (0/12) | +3.28% (0/12) | 216.9 |
| Catalog | Tiled K | 14.846 | +0.96% (4/12) | +0.80% (4/12) | 226.6 |
| Catalog | Tiled B2 | 14.528 | -1.38% (11/12) | -1.56% (11/12) | 220.7 |
| Catalog | Tiled K and B2 | 14.424 | -1.72% (11/12) | -2.22% (11/12) | 212.4 |
| Held-out | Frozen | 14.613 | — | — | 216.2 |
| Held-out | Ordinary | 14.723 | — | +0.11% (5/12) | 208.6 |
| Held-out | Prior serial B2 | 15.302 | +4.23% (0/12) | +4.58% (1/12) | 216.2 |
| Held-out | Tiled K | 14.607 | -0.60% (9/12) | +0.62% (5/12) | 221.4 |
| Held-out | Tiled B2 | 14.582 | -1.02% (10/12) | -0.40% (7/12) | 209.5 |
| Held-out | Tiled K and B2 | 14.350 | -2.54% (12/12) | -2.21% (11/12) | 213.1 |

| Input | Query | Mode | Preparation and compaction ms | Windows ms | Total query ms |
|---|---|---|---:|---:|---:|
| Catalog | K | Ordinary | 4.178 | 2.314 | 7.328 |
| Catalog | K | Tiled K | 1.723 | 2.758 | 5.263 |
| Catalog | K | Tiled K and B2 | 1.748 | 3.032 | 5.564 |
| Catalog | B2 | Ordinary | 4.376 | 2.849 | 7.267 |
| Catalog | B2 | Prior serial B2 | 4.966 | 2.968 | 7.958 |
| Catalog | B2 | Tiled B2 | 2.989 | 3.339 | 6.324 |
| Catalog | B2 | Tiled K and B2 | 3.140 | 3.359 | 6.468 |
| Held-out | K | Ordinary | 4.177 | 2.317 | 7.253 |
| Held-out | K | Tiled K | 1.719 | 3.157 | 5.666 |
| Held-out | K | Tiled K and B2 | 1.781 | 2.878 | 5.424 |
| Held-out | B2 | Ordinary | 4.286 | 2.894 | 7.237 |
| Held-out | B2 | Prior serial B2 | 4.920 | 2.961 | 7.940 |
| Held-out | B2 | Tiled B2 | 2.960 | 3.476 | 6.450 |
| Held-out | B2 | Tiled K and B2 | 3.011 | 3.467 | 6.670 |

B2 preparation and compaction changed by -1.978 and -1.961 ms versus serial B2, using the medians of the instrumented block means. The additional scratch reservations were 35.21 MiB for K and 53.60 MiB for B2. The reservation ends before the window MSM. Ordinary calibrated within 0.38% of frozen. The largest paired proof timing drift with instrumentation was 2.80%.

Each input used 12 paired blocks of seventeen timed proofs per mode. A separate panel used 6 paired blocks of thirteen timed proofs per instrumented mode. Preflight compared every nonzero scalar class and the full query point, including the ordinary fallback at zero experimental scratch. All 216 proof exports independently verified. The temporary C7a instance and storage were removed.

The joint run reduced K preparation and compaction by 2.44 and 2.40 ms and B2 by 1.24 and 1.30 ms in paired medians versus ordinary. Windows took 0.54–0.64 ms longer. K finished last in 36 and 32 of 78 ordinary traces and zero joint traces. With both tiles enabled, B2 finished last in 32 and 36 traces, and A or H finished last in the remaining 46 and 42.

## Eight supplied M5 examples

Five of eight reach 10× the M5 CPU reference. The new 4×2 C7a median is 7.56× M5 CPU and 3.33× Metal. The other seven rows come from the [previous C7a scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples).

| Shape | Recorded C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.424 | 109 / 48 | 7.56× / 3.33× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)

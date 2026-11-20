# Parallel shard point reduction on Ryzen 4×2

Tiled B2 changed full-proof latency by -1.45% on catalog and -1.24% on held-out versus the prior serial B2 compactor. Against ordinary, the changes were +2.24% and +2.26%. None of the modes met the full-proof repeatability criterion on both inputs.

K and B2 classify scalars within sixteen shards. The point reducer schedules jobs of up to 256 classes across those shards. Each job writes to its own range of the final point and scalar arrays. The existing window MSM receives the compact rows in the same shard order as the prior serial compactor. The trial used 24 main, eight frontend and four solver workers on Ryzen. [Source and preceding trial](../2026-11-20-fused-shard-compaction-ryzen968/README.md)

| Input | Mode | Full proof ms | Paired vs ordinary | Paired vs frozen | Process peak MiB |
|---|---|---:|---:|---:|---:|
| Catalog | Frozen | 14.544 | — | — | 214.2 |
| Catalog | Ordinary | 14.602 | — | +0.52% (2/8) | 213.8 |
| Catalog | Prior serial B2 | 15.223 | +4.14% (0/8) | +4.41% (0/8) | 219.2 |
| Catalog | Tiled K | 14.587 | -0.13% (4/8) | +0.19% (2/8) | 220.0 |
| Catalog | Tiled B2 | 14.990 | +2.24% (0/8) | +2.78% (0/8) | 229.5 |
| Catalog | Tiled K and B2 | 14.999 | +2.59% (0/8) | +3.02% (0/8) | 239.8 |
| Held-out | Frozen | 14.572 | — | — | 225.5 |
| Held-out | Ordinary | 14.540 | — | -0.40% (7/8) | 211.0 |
| Held-out | Prior serial B2 | 15.068 | +3.58% (0/8) | +3.13% (0/8) | 225.5 |
| Held-out | Tiled K | 14.580 | +0.43% (3/8) | -0.29% (5/8) | 212.0 |
| Held-out | Tiled B2 | 14.836 | +2.26% (1/8) | +1.81% (1/8) | 229.0 |
| Held-out | Tiled K and B2 | 14.905 | +2.83% (0/8) | +1.96% (0/8) | 234.2 |

| Input | Query | Mode | Preparation and compaction ms | Windows ms | Total query ms |
|---|---|---|---:|---:|---:|
| Catalog | K | Ordinary | 2.421 | 4.859 | 7.910 |
| Catalog | K | Tiled K | 1.856 | 5.416 | 7.895 |
| Catalog | K | Tiled K and B2 | 1.820 | 5.074 | 7.492 |
| Catalog | B2 | Ordinary | 5.165 | 3.331 | 8.453 |
| Catalog | B2 | Prior serial B2 | 6.280 | 3.012 | 9.310 |
| Catalog | B2 | Tiled B2 | 5.863 | 3.105 | 8.992 |
| Catalog | B2 | Tiled K and B2 | 6.057 | 3.018 | 9.071 |
| Held-out | K | Ordinary | 2.468 | 5.019 | 8.059 |
| Held-out | K | Tiled K | 1.783 | 5.363 | 7.815 |
| Held-out | K | Tiled K and B2 | 1.789 | 5.272 | 7.709 |
| Held-out | B2 | Ordinary | 5.168 | 3.357 | 8.541 |
| Held-out | B2 | Prior serial B2 | 6.353 | 3.067 | 9.419 |
| Held-out | B2 | Tiled B2 | 5.945 | 3.007 | 8.977 |
| Held-out | B2 | Tiled K and B2 | 5.913 | 3.114 | 9.098 |

B2 preparation and compaction changed by -0.416 and -0.408 ms versus serial B2, using the medians of the instrumented block means. The additional scratch reservations were 35.21 MiB for K and 53.60 MiB for B2. The reservation ends before the window MSM. Ordinary calibrated within 0.52% of frozen. The largest paired proof timing drift with instrumentation was 2.63%.

Each input used 8 paired blocks of seventeen timed proofs per mode. A separate panel used 4 paired blocks of thirteen timed proofs per instrumented mode. Preflight compared every nonzero scalar class and the full query point, including the ordinary fallback at zero experimental scratch. All 148 proof exports independently verified. Two native tests passed.

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz) · [Eight-shape M5 scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

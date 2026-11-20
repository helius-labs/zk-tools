# First-occurrence order for compact class rows on Ryzen 4×2

Restoring first-occurrence order for K and B2 took **14.946 ms** on catalog and **14.824 ms** on held-out. Paired changes against same-build shard order were -0.22% (4/8) and -0.25% (6/8). Every layout missed the repeatability criterion across both inputs.

Sixteen shards still classify coefficients independently and preserve their members in input order. One bit per input position identifies the first member of each class. Ranks in the bitset map four-byte descriptors to the original global class order. Point reduction writes into disjoint contiguous output tiles in that order. The trial used 24 main, eight frontend and four solver workers. [Source and prior trial](../2026-11-20-shard-point-tiles-ryzen970/README.md)

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs 970 joint | Paired vs same-build joint | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| Catalog | Frozen | 14.491 | -0.51% (5/8) | -3.02% (8/8) | -3.48% (8/8) | 209.5 |
| Catalog | Ordinary | 14.541 | — | -2.73% (7/8) | -2.68% (7/8) | 215.5 |
| Catalog | 970 joint | 14.899 | +2.80% (1/8) | — | -0.17% (5/8) | 244.5 |
| Catalog | Joint shard order | 14.949 | +2.75% (1/8) | +0.17% (3/8) | — | 228.8 |
| Catalog | First occurrence K | 14.925 | +2.48% (1/8) | +0.17% (3/8) | -0.36% (6/8) | 220.0 |
| Catalog | First occurrence B2 | 14.832 | +2.29% (1/8) | -0.82% (5/8) | -0.72% (5/8) | 244.5 |
| Catalog | First occurrence K and B2 | 14.946 | +2.80% (1/8) | -0.26% (4/8) | -0.22% (4/8) | 227.5 |
| Held-out | Frozen | 14.431 | -0.62% (5/8) | -3.43% (8/8) | -3.17% (8/8) | 206.8 |
| Held-out | Ordinary | 14.509 | — | -2.98% (8/8) | -2.57% (8/8) | 223.8 |
| Held-out | 970 joint | 14.979 | +3.07% (0/8) | — | +0.46% (3/8) | 240.5 |
| Held-out | Joint shard order | 14.903 | +2.63% (0/8) | -0.45% (5/8) | — | 227.5 |
| Held-out | First occurrence K | 14.940 | +3.05% (0/8) | -0.26% (5/8) | +0.63% (2/8) | 232.8 |
| Held-out | First occurrence B2 | 14.888 | +2.90% (0/8) | -0.48% (5/8) | -0.43% (5/8) | 240.0 |
| Held-out | First occurrence K and B2 | 14.824 | +2.04% (0/8) | -0.96% (6/8) | -0.25% (6/8) | 233.0 |

| Input | Mode | Query | Ranking ms | Paired preparation and compaction change ms | Paired window change ms | Paired query change ms |
|---|---|---|---:|---:|---:|---:|
| Catalog | First occurrence K | K | 0.103 | -0.054 | +0.029 | -0.033 |
| Catalog | First occurrence B2 | B2 | 0.108 | -0.084 | +0.014 | -0.081 |
| Catalog | First occurrence K and B2 | K | 0.100 | -0.061 | +0.190 | +0.123 |
| Catalog | First occurrence K and B2 | B2 | 0.109 | -0.154 | -0.003 | -0.101 |
| Held-out | First occurrence K | K | 0.101 | -0.055 | -0.159 | -0.272 |
| Held-out | First occurrence B2 | B2 | 0.108 | +0.024 | -0.014 | +0.027 |
| Held-out | First occurrence K and B2 | K | 0.105 | -0.047 | -0.264 | -0.281 |
| Held-out | First occurrence K and B2 | B2 | 0.107 | +0.088 | -0.004 | -0.094 |

The phase changes are paired differences from same-build joint shard order. Ranking is inside the preparation interval. The worst-case incremental reservations are 0.393 MiB for K and 0.387 MiB for B2. Ordinary calibrated within 0.62% of frozen. Same-build joint calibrated within 0.45% of the 970 joint executable. The largest paired shift from phase instrumentation was 2.22%.

Each input used 8 paired blocks of seventeen proofs and 4 separate blocks of thirteen proofs with phase clocks. Route checks compared every membership and the complete query point. For reordered queries they also compared the complete compact scalar and affine point arrays to ordinary. Zero optional scratch selected ordinary. All 164 proof exports independently verified. Two native tests passed.

[Eight-shape C7a scorecard](../2026-11-20-shard-point-tiles-c7a971/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Experimental source](artifacts/source.patch.gz)

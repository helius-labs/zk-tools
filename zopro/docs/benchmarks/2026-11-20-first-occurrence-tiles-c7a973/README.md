# First-occurrence order for compact class rows on C7a 4×2

Restoring first-occurrence order for K and B2 took **14.482 ms** on catalog and **14.410 ms** on held-out. Paired changes against same-build shard order were +0.33% (3/12) and +1.16% (3/12). Every layout missed the repeatability criterion across both inputs. The exact 970 executable with shard order reproduced medians of 14.411 and 14.463 ms.

Sixteen shards still classify coefficients independently and preserve their members in input order. One bit per input position identifies the first member of each class. Ranks in the bitset map four-byte descriptors to the original global class order. Point reduction writes into disjoint contiguous output tiles in that order. The trial used 64 main, 32 frontend and 16 solver workers. [Source and prior trial](../2026-11-20-first-occurrence-tiles-ryzen972/README.md)

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs 970 joint | Paired vs same-build joint | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| Catalog | Frozen | 14.764 | -0.70% (7/12) | +2.51% (2/12) | +2.17% (0/12) | 219.1 |
| Catalog | Ordinary | 14.734 | — | +2.48% (1/12) | +3.30% (1/12) | 228.2 |
| Catalog | 970 joint | 14.411 | -2.42% (11/12) | — | +0.32% (5/12) | 216.3 |
| Catalog | Joint shard order | 14.402 | -3.19% (11/12) | -0.32% (7/12) | — | 213.9 |
| Catalog | First occurrence K | 14.428 | -2.12% (11/12) | +0.22% (5/12) | +0.79% (4/12) | 218.4 |
| Catalog | First occurrence B2 | 14.414 | -2.00% (12/12) | -0.27% (7/12) | +0.11% (5/12) | 215.4 |
| Catalog | First occurrence K and B2 | 14.482 | -1.45% (12/12) | +0.31% (5/12) | +0.33% (3/12) | 207.9 |
| Held-out | Frozen | 14.703 | -0.23% (6/12) | +2.00% (1/12) | +3.53% (0/12) | 214.7 |
| Held-out | Ordinary | 14.684 | — | +1.68% (1/12) | +2.73% (0/12) | 216.1 |
| Held-out | 970 joint | 14.463 | -1.65% (11/12) | — | +1.52% (1/12) | 209.4 |
| Held-out | Joint shard order | 14.312 | -2.66% (12/12) | -1.50% (11/12) | — | 213.2 |
| Held-out | First occurrence K | 14.323 | -2.67% (12/12) | -1.05% (11/12) | -0.02% (6/12) | 213.2 |
| Held-out | First occurrence B2 | 14.456 | -1.84% (12/12) | -0.01% (6/12) | +1.00% (3/12) | 217.0 |
| Held-out | First occurrence K and B2 | 14.410 | -2.00% (12/12) | -0.26% (7/12) | +1.16% (3/12) | 216.9 |

| Input | Mode | Query | Ranking ms | Paired preparation and compaction change ms | Paired window change ms | Paired query change ms |
|---|---|---|---:|---:|---:|---:|
| Catalog | First occurrence K | K | 0.119 | +0.087 | -0.001 | +0.083 |
| Catalog | First occurrence B2 | B2 | 0.127 | +0.101 | -0.112 | -0.184 |
| Catalog | First occurrence K and B2 | K | 0.125 | +0.183 | -0.019 | +0.141 |
| Catalog | First occurrence K and B2 | B2 | 0.124 | +0.126 | -0.095 | +0.063 |
| Held-out | First occurrence K | K | 0.123 | +0.115 | -0.028 | +0.104 |
| Held-out | First occurrence B2 | B2 | 0.132 | +0.227 | +0.067 | +0.235 |
| Held-out | First occurrence K and B2 | K | 0.126 | +0.143 | -0.040 | +0.076 |
| Held-out | First occurrence K and B2 | B2 | 0.131 | +0.202 | -0.139 | +0.047 |

With both queries reordered, K windows saved 0.019 and 0.040 ms. B2 windows saved 0.095 and 0.139 ms. Total preparation and compaction increased by 0.143–0.183 ms for K and 0.126–0.202 ms for B2. The phase changes are paired differences from same-build joint shard order. Ranking is inside the preparation interval. The worst-case incremental reservations are 0.393 MiB for K and 0.387 MiB for B2. Ordinary calibrated within 0.70% of frozen. Same-build joint calibrated within 1.50% of the 970 joint executable. The largest paired shift from phase instrumentation was 2.61%.

Each input used 12 paired blocks of seventeen proofs and 6 separate blocks of thirteen proofs with phase clocks. Route checks compared every membership and the complete query point. For reordered queries they also compared the complete compact scalar and affine point arrays to ordinary. Zero optional scratch selected ordinary. All 240 proof exports independently verified. The temporary C7a instance and storage were removed.

[Eight-shape C7a scorecard](../2026-11-20-shard-point-tiles-c7a971/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)

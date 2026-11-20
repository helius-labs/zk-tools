# Direct point tiles for A on Ryzen 4×2

A with 16 shards plus K and B2 took **15.103 ms** on catalog and **15.081 ms** on held-out. Paired latency changes against same-build k and b2 were +0.68% (1/8) and +0.66% (3/8). All candidate layouts fell short of the repeatability criterion.

A classifies its 67,154 scalar rows in independent shards. A global list of reduction tiles writes directly into disjoint ranges of the compact affine and scalar outputs. Each tile uses the previous point-reduction math and the existing window passes consume the rows in shard order. The trial used 24 main, eight frontend and four solver workers. [Source and previous trial](../2026-11-20-shard-point-tiles-c7a971/README.md)

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs exact 970 K and B2 | Paired vs same-build K and B2 | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| Catalog | Frozen | 14.495 | -0.42% (5/8) | -3.21% (8/8) | -3.47% (8/8) | 213.2 |
| Catalog | Ordinary | 14.611 | — | -2.51% (8/8) | -2.85% (8/8) | 217.5 |
| Catalog | 970 K and B2 | 14.966 | +2.58% (0/8) | — | -0.30% (5/8) | 231.5 |
| Catalog | Same-build K and B2 | 14.991 | +2.93% (0/8) | +0.30% (3/8) | — | 226.2 |
| Catalog | A with 16 shards | 14.752 | +1.09% (0/8) | -1.46% (8/8) | -1.94% (7/8) | 225.0 |
| Catalog | A with 16 shards plus K and B2 | 15.103 | +3.87% (0/8) | +0.92% (0/8) | +0.68% (1/8) | 234.2 |
| Catalog | A with 32 shards plus K and B2 | 15.069 | +3.69% (0/8) | +0.79% (1/8) | +0.50% (1/8) | 231.2 |
| Held-out | Frozen | 14.481 | -1.18% (7/8) | -3.16% (8/8) | -3.74% (8/8) | 221.5 |
| Held-out | Ordinary | 14.644 | — | -2.30% (7/8) | -2.48% (8/8) | 217.8 |
| Held-out | 970 K and B2 | 14.934 | +2.35% (1/8) | — | -0.56% (6/8) | 236.0 |
| Held-out | Same-build K and B2 | 15.011 | +2.54% (0/8) | +0.57% (2/8) | — | 232.2 |
| Held-out | A with 16 shards | 14.739 | +0.34% (3/8) | -2.28% (7/8) | -2.03% (8/8) | 216.0 |
| Held-out | A with 16 shards plus K and B2 | 15.081 | +3.37% (0/8) | +0.58% (3/8) | +0.66% (3/8) | 230.2 |
| Held-out | A with 32 shards plus K and B2 | 15.159 | +3.80% (1/8) | +1.41% (1/8) | +0.88% (1/8) | 231.2 |

| Input | Mode | A prep and compaction change ms | A window change ms | A query change ms | H query change ms | K query change ms | B2 query change ms |
|---|---|---:|---:|---:|---:|---:|---:|
| Catalog | A with 16 shards | +0.025 | -0.439 | +0.034 | +0.147 | +0.226 | +0.171 |
| Catalog | A with 16 shards plus K and B2 | +0.236 | -0.051 | +0.007 | +0.145 | -0.075 | +0.199 |
| Catalog | A with 32 shards plus K and B2 | +0.174 | +0.120 | +0.423 | +0.019 | +0.299 | +0.145 |
| Held-out | A with 16 shards | -0.128 | -0.021 | -0.109 | +0.085 | +0.105 | +0.131 |
| Held-out | A with 16 shards plus K and B2 | +0.487 | -0.232 | +0.306 | +0.265 | +0.142 | +0.126 |
| Held-out | A with 32 shards plus K and B2 | +0.781 | -0.100 | +0.688 | +0.077 | +0.137 | +0.022 |

| Input | Mode | Curve stage ms | A finished last | H finished last | K finished last | B2 finished last |
|---|---|---:|---:|---:|---:|---:|
| Catalog | Ordinary | 8.644 | 1 | 9 | 0 | 42 |
| Catalog | Same-build K and B2 | 9.159 | 1 | 4 | 0 | 47 |
| Catalog | A with 16 shards | 8.860 | 2 | 9 | 0 | 41 |
| Catalog | A with 16 shards plus K and B2 | 9.313 | 0 | 4 | 0 | 48 |
| Catalog | A with 32 shards plus K and B2 | 9.296 | 2 | 3 | 0 | 47 |
| Held-out | Ordinary | 8.624 | 3 | 14 | 1 | 34 |
| Held-out | Same-build K and B2 | 9.101 | 1 | 4 | 0 | 47 |
| Held-out | A with 16 shards | 8.787 | 3 | 18 | 0 | 31 |
| Held-out | A with 16 shards plus K and B2 | 9.208 | 1 | 4 | 0 | 47 |
| Held-out | A with 32 shards plus K and B2 | 9.166 | 3 | 5 | 0 | 44 |

Phase changes are paired against ordinary for A alone and against same-build K and B2 for the combined modes. The additional worst-case construction reservations for A are 24.36 MiB with 16 shards and 24.39 MiB with 32. Ordinary calibrated within 1.20% of frozen. Same-build K and B2 calibrated within 0.57% of the exact 970 executable. The largest paired shift with phase instrumentation was 1.77%.

Each input used 8 paired blocks of seventeen proofs and 4 separate blocks of thirteen proofs with clocks. Route checks compared every member of each scalar class and the complete query point. A separate probe denied optional scratch and exercised the ordinary paths. All 164 proof exports independently verified. Two native tests passed.

[Eight-shape C7a scorecard](../2026-11-20-shard-point-tiles-c7a971/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Experimental source](artifacts/source.patch.gz)

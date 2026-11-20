# B2 shard count with A/K/B2 tiles on C7a 4×2

Eight B2 shards reduced paired proof latency by 0.61% on catalog and 0.10% on held-out versus sixteen shards. It won eight and six of twelve blocks. Thirty-two shards increased paired latency by 0.33% and 0.45%. Both candidates fell short of nine wins per input.

The run uses the same [qualified executable and placement](../2026-11-20-a-class-tiles-c7a975/README.md) with 32 A shards, 16 K shards, 64 main workers, 32 frontend workers and 16 solver workers. Each B2 mode keeps direct point tiles and the same window arithmetic.

| Input | B2 shards | Proof ms | Paired vs 16 | Peak MiB | B2 finished last |
|---|---:|---:|---:|---:|---:|
| Catalog | 8 | 14.136 | -0.61% (8/12) | 219.2 | 45/78 |
| Catalog | 16 | 14.189 | — | 220.7 | 47/78 |
| Catalog | 32 | 14.138 | +0.33% (5/12) | 216.9 | 50/78 |
| Held-out | 8 | 14.092 | -0.10% (6/12) | 226.7 | 40/78 |
| Held-out | 16 | 14.078 | — | 215.4 | 50/78 |
| Held-out | 32 | 14.106 | +0.45% (4/12) | 211.7 | 51/78 |

| Input | B2 shards | B2 preparation and tiles change ms | B2 window change ms | B2 query change ms |
|---|---:|---:|---:|---:|
| Catalog | 8 | +0.146 | -0.052 | +0.052 |
| Catalog | 32 | +0.044 | -0.010 | +0.098 |
| Held-out | 8 | -0.262 | +0.262 | -0.075 |
| Held-out | 32 | +0.070 | +0.060 | +0.092 |

Changes are paired against 16 shards. The timed panel uses twelve paired blocks of seventeen proofs. The separate clock panel uses six paired blocks of thirteen proofs and shifts proof timing by up to 2.79%. The six route probes check every scalar class and its complete curve result against ordinary. All 114 proof exports independently verified. The temporary instance and storage were removed.

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json) · [Eight-shape M5 scorecard](../2026-11-20-a-class-tiles-c7a975/README.md#eight-supplied-m5-examples)

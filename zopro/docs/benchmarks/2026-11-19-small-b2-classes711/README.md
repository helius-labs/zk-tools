# Sharding the shared small-query scalar classifier

The entry point is shared by the small canonical G1 and G2 queries. The first route audit expected one invocation per proof. On 1×16 it recorded three distinct query lengths per proof. The audit now preserves every dispatch and checks its repetition across all four calls. The full-proof comparisons measure the combined change. This trial preserves the small-query sample and diversity filters. When a proof qualifies for grouping, eight or 32 shards independently classify equal canonical scalars. The points inside each class keep their order. The private order of the classes can follow the shards. Unlike grouped witnesses on larger keys, these scalars already use canonical limbs, so the shard builder passes their values through unchanged. The same charge and fallback cover the temporary arrays.

Four balanced Ryzen blocks compare the prior release, the new executable with sharding disabled, and both shard counts. Each process used 32 logical workers, three primes and eleven measured proofs. The 1×16 shape includes a second transaction. A zero-budget execution checks the original fallback. A generator-independent test compares the grouped membership in both scalar representations under deliberate shard collisions. All 143 exports verified.

At 1×16, 32 shards lowered the paired median against off by 1.1% on the catalog request and 3.3% on the second request. Eight shards at 1×8 gained 1.8% against off. The separate curve-specific follow-up distinguishes G1 and B2 contributions.

| Shape | Request | Shards | Release / off / enabled ms | vs off (wins) | vs release (wins) | Peak MiB release / enabled |
|---|---|---|---:|---:|---:|---:|
| 1x16 | catalog | 8 | 13.230 / 13.182 / 13.132 | -0.34% (2/4) | -0.30% (2/4) | 121.4 / 129.9 |
| 1x16 | catalog | 32 | 13.230 / 13.182 / 13.041 | -1.07% (4/4) | -1.70% (3/4) | 121.4 / 126.6 |
| 1x16 | 57 | 8 | 15.178 / 13.695 / 13.175 | -4.33% (3/4) | -11.78% (4/4) | 138.4 / 132.7 |
| 1x16 | 57 | 32 | 15.178 / 13.695 / 13.137 | -3.29% (3/4) | -11.43% (3/4) | 138.4 / 126.8 |
| 1x2 | catalog | 8 | 7.314 / 7.309 / 7.309 | -0.02% (3/4) | +0.24% (2/4) | 59.8 / 57.2 |
| 1x2 | catalog | 32 | 7.314 / 7.309 / 7.275 | -0.26% (3/4) | +0.13% (2/4) | 59.8 / 56.0 |
| 1x4 | catalog | 8 | 10.449 / 10.360 / 10.425 | +1.35% (1/4) | +0.82% (2/4) | 103.5 / 112.0 |
| 1x4 | catalog | 32 | 10.449 / 10.360 / 10.503 | +0.04% (2/4) | +1.64% (1/4) | 103.5 / 110.0 |
| 1x8 | catalog | 8 | 12.733 / 12.615 / 12.466 | -1.75% (3/4) | -0.47% (3/4) | 128.5 / 115.0 |
| 1x8 | catalog | 32 | 12.733 / 12.615 / 12.364 | +1.29% (2/4) | -2.98% (3/4) | 128.5 / 113.8 |
| 2x2 | catalog | 8 | 11.769 / 9.943 / 10.117 | +2.00% (0/4) | +1.59% (1/4) | 118.5 / 113.8 |
| 2x2 | catalog | 32 | 11.769 / 9.943 / 12.297 | +1.28% (0/4) | +0.88% (2/4) | 118.5 / 121.8 |
| 2x4 | catalog | 8 | 10.871 / 10.740 / 10.853 | +0.35% (2/4) | -0.83% (3/4) | 110.0 / 115.8 |
| 2x4 | catalog | 32 | 10.871 / 10.740 / 10.978 | +1.51% (0/4) | +0.32% (1/4) | 110.0 / 114.5 |
| 2x8 | catalog | 8 | 13.733 / 13.756 / 14.030 | +2.56% (1/4) | +1.86% (1/4) | 163.5 / 148.5 |
| 2x8 | catalog | 32 | 13.733 / 13.756 / 16.155 | +16.17% (0/4) | +1.65% (1/4) | 163.5 / 152.5 |

[Earlier C7a geometry comparison](../2026-11-18-wide-g2-c7a298/README.md) · [Source](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Sample timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

# Separating G1 and B2 scalar classification

The previous scalar-class experiment entered a shared G1 and G2 grouping routine. This version gives the exclusive B2 caller its own entry point and records the curve for each admitted query. One mode enables sharding only for B2. A second includes the G1 callers and reproduces the previous scope. Both use the same executable and budget policy. On 1×16 each uses 32 shards, on 1×8 and 2×2 each uses eight. A previous shared-sharding executable and the release run in every block for independent calibration.

Four fresh paired blocks time eleven proofs following three primes. The original, disabled, B2-only, shared and previous shared executable use identical keys, transactions and affinity. The 1×16 shape includes a second input. Separate probes verify the admitted curve and a 1×16 zero-budget fallback. All 87 process exports verified.

| Shape | Request | Shards | Release / off / enabled ms | vs off (wins) | vs release (wins) | Peak MiB release / enabled |
|---|---|---|---:|---:|---:|---:|
| 1x16 | catalog | b2 | 13.213 / 13.306 / 13.216 | -1.01% (4/4) | -0.07% (2/4) | 118.7 / 129.9 |
| 1x16 | catalog | shared | 13.213 / 13.306 / 13.021 | -2.57% (4/4) | -1.39% (4/4) | 118.7 / 124.4 |
| 1x16 | 57 | b2 | 13.287 / 13.280 / 13.385 | +0.97% (2/4) | -0.45% (2/4) | 142.9 / 133.2 |
| 1x16 | 57 | shared | 13.287 / 13.280 / 13.139 | -0.94% (2/4) | -1.22% (4/4) | 142.9 / 137.9 |
| 1x8 | catalog | b2 | 10.342 / 10.403 / 10.388 | -0.18% (2/4) | +0.45% (2/4) | 110.0 / 111.2 |
| 1x8 | catalog | shared | 10.342 / 10.403 / 10.546 | +1.30% (0/4) | +1.03% (2/4) | 110.0 / 104.2 |
| 2x2 | catalog | b2 | 9.807 / 9.940 / 10.039 | +0.46% (1/4) | +1.12% (1/4) | 104.5 / 106.2 |
| 2x2 | catalog | shared | 9.807 / 9.940 / 10.064 | +1.09% (1/4) | +1.78% (1/4) | 104.5 / 108.8 |

[Earlier C7a geometry comparison](../2026-11-18-wide-g2-c7a298/README.md) · [Source](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Sample timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

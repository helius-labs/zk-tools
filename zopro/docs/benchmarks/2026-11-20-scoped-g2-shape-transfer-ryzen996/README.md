# Scoped G2 route check on five more shapes

The [C7a scoped G2 candidate](../2026-11-20-scoped-g2-overlap-c7a993/README.md) activates on both requests for 1×2, 2×2, 1×4, 2×4 and 2×8. With eight physical Ryzen cores per pool, paired median changes versus the same-build late start ranged from −0.95% to +0.51%. These local checks qualify the route and its workspace fallback for server measurements.

Both timing arms use identical inputs and disjoint placement. The 2×8 wire-offset artifact is pinned to the same image hash as the catalog benchmark. Each process primes three proofs and times thirteen in four recurring blocks. The two separate instrumented proofs per case confirm the G2 start and its overlap with the quotient.

| Shape | Request | Late / early ms | Paired early change | Minimum quotient overlap ms | Peak late / early MiB |
|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 8.540 / 8.516 | -0.00% (2/4) | 0.452 | 53.0 / 49.2 |
| 1×2 | heldout | 8.188 / 8.210 | +0.51% (2/4) | 0.449 | 53.5 / 51.2 |
| 2×2 | catalog | 12.263 / 12.270 | -0.95% (2/4) | 0.999 | 88.5 / 88.5 |
| 2×2 | heldout | 12.411 / 12.313 | -0.59% (2/4) | 0.995 | 88.2 / 88.5 |
| 1×4 | catalog | 10.297 / 10.268 | -0.15% (2/4) | 0.957 | 83.5 / 85.5 |
| 1×4 | heldout | 10.291 / 10.205 | -0.82% (3/4) | 0.967 | 83.8 / 84.8 |
| 2×4 | catalog | 13.056 / 12.959 | -0.74% (3/4) | 1.076 | 104.2 / 104.8 |
| 2×4 | heldout | 13.047 / 12.985 | -0.79% (2/4) | 1.025 | 106.2 / 100.8 |
| 2×8 | catalog | 16.773 / 16.634 | -0.72% (3/4) | 2.316 | 134.8 / 132.0 |
| 2×8 | heldout | 16.758 / 16.752 | -0.10% (4/4) | 2.353 | 131.0 / 136.5 |

Every forced workspace denial selected the late path. All 110 process exports passed independent verification.

[Samples](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json)

# An invertible B2 point layout with the original point count

For each disjoint group, write its point sum once, followed later by the original points of all members except the first. The prefix contains the group sums and all ungrouped points. Its scalars are the representative scalars and ungrouped scalars. Each tail scalar is the difference from its representative. This identity holds for arbitrary scalars, including when every difference is nonzero. It preserves the number and size of curve points. Optional row-index metadata takes additional space.

A standalone Gnark-crypto checker reads the original native-layout BN254 points, checks that they lie on the curve, writes an alternative B2 section and reopens it. It inverts the representation and compares every recovered point with the original. It then compares the complete original and transformed B2 MSM results on two archived public scalar vectors and one deterministic independent full-width random vector per shape. The 12×2 vectors include a scattered request excluded from the adjacent plan's training.

| Shape | Original and transformed points | Prefix terms | Original and transformed B2 MiB | Nonzero tail terms, two fixtures / random |
|---|---:|---:|---:|---|
| 2x2 | 52,274 | 34,981 | 6.38 | 0 / 0 / 17,293 |
| 4x2 | 95,537 | 44,060 | 11.66 | 0 / 0 / 51,477 |
| 8x2 | 183,347 | 63,474 | 22.38 | 0 / 0 / 119,873 |
| 12x2 | 270,757 | 82,233 | 33.05 | 0 / 94,596 / 188,524 |
| 16x2 | 358,343 | 102,002 | 43.74 | 0 / 0 / 256,341 |
| 24x2 | 534,619 | 141,184 | 65.26 | 0 / 0 / 393,435 |
| merge-24x1 | 507,895 | 217,596 | 62.00 | 0 / 0 / 290,299 |

This experiment verifies storage and query arithmetic. Loading it directly in the SDK, proof latency, preparation peak RSS, fallback behavior and interaction with public offsets require separate full-proof measurements. The checker keeps both layouts in memory for differential verification. [Checker](artifacts/check.go) · [Point and input identities](artifacts/results.json)

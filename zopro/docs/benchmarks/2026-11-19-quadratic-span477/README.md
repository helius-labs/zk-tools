# Exact cancellation among nonlinear constraint rows

A valid witness satisfies each constraint A·B − C = 0 over BN254 Fr. A constant-coefficient linear combination of these constraints yields a linear wire identity if its homogeneous quadratic terms cancel. This screen computes the dimension of that cancellation space for all nonlinear rows in each supplied key. Existing [direct linear rows](../2026-11-19-linear-rows386/README.md) are excluded from the input.

A variable that appears in the homogeneous product of just one surviving row forces that row's multiplier to zero. The reader repeatedly removes those rows, then does the same for individual quadratic monomials after expanding the remaining products and combining coefficients modulo Fr. Sparse Gaussian elimination on the survivors carries the affine terms through every row operation. A second elimination counts linearly independent residual equations. A synthetic control checks a three-row cancellation with no proportional pair. Two additional controls check cascading removal and rows that must survive.

| Shape | Nonlinear rows | After wire removal | After monomial removal | Independent quadratic cancellations | Independent linear consequences | Upper bound on pivots among non-hint B2 wires |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 30,839 | 10,763 | 9,177 | 96 | 96 | 96 |
| 12x2 | 279,741 | 100,671 | 82,383 | 932 | 932 | 932 |

The rank of the residual equations sums to 1,028 across these two keys. The analysis covers constant-coefficient row combinations over the original wires. Consequences that require multiplying, dividing or otherwise using nonlinear constraints fall outside this calculation.

Each image digest was verified before the scan. These are algebraic counts from unmodified public keys. No latency or memory improvement is claimed. [Counts and image identities](artifacts/counts.json) · [Reader](artifacts/scan.py)

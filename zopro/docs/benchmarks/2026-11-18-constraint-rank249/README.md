# Query rank on satisfying witnesses

The 1×2 circuit has 31 additional A rows and 67 additional B rows that could be removed by using equations from constraints without assignments. These amount to 0.15% of the original A rows and 0.22% of the original B rows beyond the earlier proportional folding. The counts give little reason to prioritize this transformation for the 96-core latency gap.

| Circuit | Query | Original points | Proportional classes | Rank using constraint equations | Additional removable rows |
|---|---|---:|---:|---:|---:|
| 1×2 | A | 21,006 | 20,896 | 20,865 | 31 |
| 1×2 | B | 30,089 | 29,904 | 29,837 | 67 |
| 2×2 | A | 36,637 | 36,442 | 36,393 | 49 |
| 2×2 | B | 52,274 | 51,971 | 51,874 | 97 |

A depends on G1 points. B supplies scalars to both G1 and G2. The count measures algebraic dimension. Full-proof latency, point-addition savings and memory use would need separate measurements.

The offline reader uses the semantic forms from the existing [proportional-fold experiment](../2026-11-17-static-query-fold205/README.md). It multiplies the first two sides of each constraint without an assignment using the same product interning rules, then subtracts the third side. This expression vanishes on satisfying witnesses. Sparse Gaussian elimination over BN254 Fr measures the dimension added by the query rows after inserting those relations. The constant occupies a separate column. The 2,421 and 4,395 constraint rows give relation ranks of 619 and 1,157. Computing the query rank without these relations reproduces the earlier proportional-class count in all four cases.

A separate dense field-elimination routine checks the sparse quotient computation on 16 small matrices in two query orders. Both routines use the same field library. The dense oracle and both real-image analyses passed. Results apply to the modeled identities and pinned inputs. Other nonlinear identities, K and H queries, and the cost distribution among surviving rows remain open. This experiment changed no proving key or production code and made no proof calls.

[Method and inputs](artifacts/protocol.md) · [Sparse and dense algorithms](artifacts/rank.rs) · [1×2 counts](artifacts/1x2.json) · [2×2 counts](artifacts/2x2.json)

# Early grouped B2 with a disjoint worker pool

On Ryzen, starting grouped B2 before the quotient reduced paired full-proof latency by 4.29% to 10.26% for 8×2 and 1.32% to 2.29% for 4×2 against the same executable’s ordinary route. Early B2 won all four blocks for both requests and both placements, with lower peak RSS in every comparison.

The candidate runs grouped B2 on the frontend pool and keeps the quotient and G1 on the main pool. Both early and late scoped arms use the same B2 and G1 code. The late arm starts B2 after the quotient. The early arm starts B2 before it and reserves the additional scratch against the existing memory budget.

| Shape | Request | Main + frontend | Archived / ordinary ms | Late / early ms | Paired early vs ordinary | Peak ordinary / early MiB |
|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | 12+4 | 19.976 / 20.057 | 22.017 / 19.538 | -2.29% (4/4) | 177.8 / 167.0 |
| 4×2 | catalog | 24+8 | 15.045 / 14.980 | 16.404 / 14.799 | -1.32% (4/4) | 216.0 / 186.2 |
| 4×2 | heldout | 12+4 | 20.062 / 19.799 | 21.934 / 19.469 | -1.76% (4/4) | 177.5 / 156.8 |
| 4×2 | heldout | 24+8 | 15.111 / 14.939 | 16.456 / 14.630 | -2.09% (4/4) | 211.0 / 179.0 |
| 8×2 | catalog | 12+4 | 33.687 / 33.919 | 33.939 / 30.790 | -9.29% (4/4) | 330.2 / 298.0 |
| 8×2 | catalog | 24+8 | 24.600 / 24.760 | 25.276 / 23.303 | -5.60% (4/4) | 394.8 / 346.0 |
| 8×2 | heldout | 12+4 | 34.332 / 34.399 | 34.389 / 31.021 | -10.26% (4/4) | 337.0 / 311.2 |
| 8×2 | heldout | 24+8 | 24.884 / 24.600 | 25.532 / 23.527 | -4.29% (4/4) | 373.8 / 339.8 |

The 12+4 placement uses physical cores. The 24+8 placement includes SMT siblings. Each timing block primes three proofs and times thirteen. Separate two-proof diagnostics confirmed both scoped dispatches and the zero-budget fallback. All 152 exports passed independent verification. This experiment used Ryzen. The C7a comparison is pending.

[Summary](artifacts/summary.json) · [Samples and diagnostics](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz)

# Applying public offsets once across three query sums

This 1×2 prototype subtracts each public offset at its solved wire before the canonical gather. The key's index maps project 1168 B2 entries onto 818 A and 1167 private K entries. At setup it computes all three correction points from the actual proving key. The quotient uses the original constraint evaluations. Scalar subtraction and addition of the constant correction points preserve the result for arbitrary input values.

Seven randomized blocks measured 31 complete warm SDK proofs per process per input, using 32 Ryzen logical workers and eight frontend workers. The B2 comparison uses the same executable and the compact correction setup from experiment 457.

| Input | Release / disabled / B2 only / shared wire ms | vs release | vs disabled | vs B2 only | Peak RSS B2 only / shared MiB |
|---:|---:|---:|---:|---:|---:|
| 9 | 7.050 / 7.066 / 6.963 / 6.924 | -1.58% (7/7) | -1.76% (7/7) | -0.09% (4/7) | 63.0 / 62.5 |
| 10 | 7.112 / 7.074 / 6.969 / 6.945 | -2.01% (7/7) | -1.81% (7/7) | -0.50% (5/7) | 61.5 / 60.5 |
| 11 | 7.046 / 7.100 / 7.037 / 6.934 | -1.74% (7/7) | -2.19% (7/7) | -1.31% (7/7) | 66.2 / 61.2 |
| 12 | 7.056 / 7.073 / 6.979 / 6.973 | -1.41% (7/7) | -1.78% (7/7) | -0.32% (4/7) | 62.2 / 62.0 |
| 13 | 7.079 / 7.048 / 7.016 / 6.968 | -1.48% (7/7) | -0.99% (7/7) | -0.30% (4/7) | 62.8 / 61.0 |
| 14 | 7.066 / 7.040 / 6.991 / 6.935 | -1.64% (6/7) | -1.35% (6/7) | -0.39% (4/7) | 62.5 / 62.0 |
| 15 | 7.052 / 7.030 / 6.970 / 6.938 | -1.52% (7/7) | -1.17% (7/7) | -0.46% (5/7) | 60.5 / 62.5 |
| 16 | 7.043 / 7.048 / 7.001 / 6.950 | -1.79% (7/7) | -1.40% (7/7) | -0.88% (6/7) | 61.8 / 60.8 |

The median paired changes were -1.61% against release (8/8), -1.58% against disabled (8/8) and -0.43% against B2 only (8/8). The candidate met the six-witness criterion against all three controls. The median paired first-proof difference against B2 only was +3.45 ms. The additional G1 map and setup scratch reservations were 47,144 and 192,512 bytes. The shared route activated under a 1 MiB optional preparation budget. With zero budget the ordinary route produced a verified proof. All 229 exported proofs passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Incremental source](artifacts/changes.patch) · [Input identities](../2026-11-19-batched-offset456/artifacts/input-hashes.json)

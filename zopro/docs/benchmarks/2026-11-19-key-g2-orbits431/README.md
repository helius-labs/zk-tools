# G2 proving-key aliases across eight reference shapes

A B-query point can share an MSM term with an equal or opposite base. On the BN254 twist, two finite points whose y coordinates agree up to sign have the same x cubed. Counting those y classes also covers the cube-root endomorphism. This extends the earlier two-shape census to six larger keys and gives a bound on how many raw B-query terms these relations could eliminate before witness grouping.

| Shape | B-query points | Identity | Exact duplicate excess | Additional opposite | Additional cube-root | Removable raw terms |
|---|---:|---:|---:|---:|---:|
| 1x2 | 30,089 | 0 | 77 | 0 | 0 | 0.256% |
| 2x2 | 52,274 | 0 | 155 | 0 | 0 | 0.297% |
| 4x2 | 95,537 | 0 | 311 | 0 | 0 | 0.326% |
| 8x2 | 183,347 | 0 | 623 | 0 | 0 | 0.340% |
| 12x2 | 270,757 | 0 | 935 | 0 | 0 | 0.345% |
| 16x2 | 358,343 | 0 | 1,247 | 0 | 0 | 0.348% |
| 24x2 | 534,619 | 0 | 1,871 | 0 | 0 | 0.350% |
| merge-24x1 | 507,895 | 0 | 1,847 | 0 | 0 | 0.364% |

All eight native Euler images matched their pinned SHA-256. On 1×2 and 2×2 each point also matched its independently qualified Arkworks fixture after converting the big-endian canonical Fq2 coordinates to native Montgomery order, and all class counts reproduced the earlier census. Remaining shapes use the validated image layout and the same counter. A direct raw-byte hash comparison initially failed because the two formats use different coordinate encodings. The counts above use pointwise equality after explicit conversion. The October [duplicate-base census](../2026-10-21-ryzen-research/fixed-basis-collisions/README.md) already examined ordinary duplicates; the extension here covers the larger keys under the signed cube-root map. The census used only immutable key bytes and added no prover state. It measured no proof latency.

[Counts and input hashes](artifacts/counts.json) · [Earlier census and subgroup checks](../2026-11-06-key-point-orbits176/README.md)

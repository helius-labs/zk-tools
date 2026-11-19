# Affine relations between moving G2 query scalars

Five distinct 1×2 inputs determined the row groups. Each eligible row had at least four distinct field values across those inputs. Four additional inputs varied the coin, amounts and Merkle trees. The model kept every original point for a group whenever any relation in that group failed. A relation of the form sⱼ = a·sᵢ + b would allow a prover to combine Pᵢ + a·Pⱼ and add the fixed correction b·Pⱼ, provided it establishes that relation for the request.

25,374 of 30,089 scalar positions met the training variation requirement. The stable columns require every member of the group to match in all four held-out requests.

| Relation | Discovery groups | Stable groups | Stable groups moving in holdout | Stable removable rows | Edges with nonzero offset | Edges with slope outside ±1 | Median signed digit updates avoided |
|---|---:|---:|---:|---:|---:|---:|---:|
| signed | 29 | 29 | 29 | 45 | 0 | 0 | 0.161% |
| translated | 29 | 29 | 29 | 45 | 0 | 0 | 0.161% |
| affine | 29 | 29 | 29 | 45 | 0 | 0 | 0.161% |

An exploratory sensitivity check changes the minimum number of distinct training values. The entries count removable rows in groups that validated on every holdout.

| Minimum distinct values in five training requests | Signed | Signed plus offset | Arbitrary slope plus offset |
|---:|---:|---:|---:|
| 2 | 336 | 336 | 49 |
| 3 | 49 | 49 | 49 |
| 4 | 45 | 45 | 45 |
| 5 | 45 | 45 | 45 |

Allowing offsets and arbitrary slopes added zero stable rows at the primary threshold. With only two distinct training values, unrelated rows can share a fitted slope. A failed member rejects the entire group, which explains how the more general fit can retain fewer rows on the holdout.

The update estimate allows all groups that validate on the individual holdout input and counts the signed, nonzero digits of each removed row. It excludes group validation, point preparation, fixed corrections, bucket collisions, reduction and scheduling. The same logical circuit generated every input. The model supplies no latency measurement or general circuit identity. All nine original exports passed independent Gnark verification again. Numerical relations inferred from these private fixtures remain local.

[Counts for each held-out request](artifacts/counts.json) · [Sensitivity counts](artifacts/sensitivity.json) · [Protocol](artifacts/protocol.md) · [Proof verification](artifacts/verification.json) · [Signed-group discovery on one input](../2026-11-19-g2-tree-variation418/README.md)

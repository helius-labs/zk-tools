# Pair distinct scalars using their values on the current proof

Full scalar grouping leaves one base per distinct nonzero scalar. For two such scalars x and y and their bases P and Q, the prover could evaluate x(P+Q)+(y−x)Q. Either scalar can anchor the pair. When the residual has sparse signed digits, this trades one online point addition for fewer bucket updates. The ordering and matching would use only the current request, with no per-key lookup table.

This screen tries four orders: canonical scalar, lexicographic signed GLV coordinates in either order and interleaved (Morton) GLV coordinates. For each order, dynamic programming finds a maximum-saving set of disjoint adjacent pairs. The weight is the reduction in nonzero width-12 signed window digits after exact field subtraction and fresh GLV decomposition, less one addition per selected pair. Both anchor directions are scored. The baseline already contracts all equal nonzero scalars. The matched synthetic control draws the same number of distinct canonical values by deterministic rejection sampling. The reported best of four per witness is an optimistic selector within this family.

| Shape | Public input IDs | Distinct scalars | Highest-scoring orders | Net nominal saving | Matched synthetic control |
|---|---|---:|---|---:|---:|
| 1x2 | 1 / 8 | 26,801 / 26,795 | spatial / spatial | 1.38% / 1.36% | 1.37% / 1.37% |
| 2x2 | 33 / 40 | 29,211 / 29,208 | spatial / spatial | 1.41% / 1.43% | 1.42% / 1.42% |
| 4x2 | 145 / 152 | 32,146 / 32,136 | spatial / spatial | 1.48% / 1.46% | 1.47% / 1.47% |
| 8x2 | 145 / 152 | 39,653 / 39,643 | spatial / spatial | 1.58% / 1.57% | 1.57% / 1.57% |
| 12x2 | 49 / 56 | 46,364 / 46,357 | spatial / spatial | 1.65% / 1.64% | 1.64% / 1.64% |
| 16x2 | 65 / 72 | 53,806 / 53,799 | spatial / spatial | 1.71% / 1.71% | 1.71% / 1.71% |
| 24x2 | 89 / 96 | 68,129 / 68,124 | spatial / spatial | 1.80% / 1.82% | 1.80% / 1.80% |
| merge-24x1 | 105 / 112 | 42,380 / 80,342 | spatial / spatial | 1.60% / 1.87% | 1.60% / 1.87% |

These counts exclude sorting, normalization, memory traffic and the choice of partner. They treat a point addition and a bucket arrival as one operation each. Grouped curve execution may use a different window width or partition. Captures contain ordinary B2 input scalars before optional key transformations. The table measures neither elapsed time nor a general bound for pairing across other orders. Sixteen archived proof exports were independently reverified with their original keys. Native tests cover disjoint selection and field wraparound. An independent Python scalar-field and signed-digit implementation reproduced all four matching scores and counts on a synthetic vector containing field-boundary and near-neighbor pairs. [Rust model](artifacts/neighbors.rs) · [Per-order counts and input identities](artifacts/counts.json) · [Independent control results](artifacts/control.json)

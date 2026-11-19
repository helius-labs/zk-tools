# Can static linear elimination substantially shrink the wide G2 queries?

Two existing, witness-independent scanners inspect the original key images used in the 96-core comparison. The first derives equations from constraints with a provably constant multiplicative operand and bounds the number of independent pivots on queried wires outside Boolean hint outputs. The second computes the rank of the B-side coefficient columns over the scalar field. An independent Arkworks executable checks every resulting column relation against the actual G2 key points. The two counts describe different opportunities and can overlap.

| Shape | Original B2 points | Direct pivot ceiling (fraction of original) | B-column rank | Dependent columns (fraction of original) |
|---|---:|---:|---:|---:|
| 1x16 | 57,943 | 283 (0.49%) | 57,427 | 516 (0.89%) |
| 5x16 | 145,951 | 395 (0.27%) | 144,179 | 1,772 (1.21%) |
| 6x2 | 139,420 | 213 (0.15%) | 137,481 | 1,939 (1.39%) |

The 5×16 scan found 1,772 dependent columns among 145,951 B2 points. The separate direct-constraint screen bounded removable pivots at 395. On 1×16 the corresponding counts were 516 and 283. The small structural shares give little reason to expect a twofold whole-proof gain from rewriting these key sections alone. All 4,227 B-column relations across the three keys passed independent curve checks.

A removed point and a saved millisecond are distinct quantities. The percentage denominator is the original query, before the prover groups equal witness scalars. The audit measures no new proof latency. It covers these two classes of exact identities only.

[Direct row scanner](../2026-11-19-linear-rows386/README.md) · [Column elimination and curve oracle](../2026-11-19-b-column-rank552/README.md) · [Counts and curve checks](artifacts/counts.json) · [Input and tool digests](artifacts/pins.json)

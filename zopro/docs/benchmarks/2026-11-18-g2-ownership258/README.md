# Work split between the two G2 point ranges

The second G2 partition had **7.15% more nonzero digit arrivals** than the first on the retained 1×2 scalar fixture. At 96 cores its mean elapsed time across ordinary windows was **8.0% higher**. In the window that returned last, the second partition also returned later in **405 of 453 proofs**. This gives a reason to test how the prover divides a point array whose entries cost different amounts to process.

| Ordinary G2 window measurement | First half | Second half |
|---|---:|---:|
| Rows | 15,045 | 15,044 |
| Nonzero digit arrivals across ten ordinary windows | 269,666 | 288,948 |
| Mean partition elapsed at 32 cores, ms | 2.490 | 2.650 |
| Mean partition elapsed at 96 cores, ms | 2.877 | 3.107 |

The first ten windows use the ordinary borrowed fill. The eleventh uses a specialized top-digit reduction. For each of the ten ordinary windows the second half had more arrivals. At 96 cores it took longer in 80.7% of individual within-window comparisons. In window zero the first half took slightly longer on average. Counts alone do not account for all execution time.

The fixture supplies the complete original 30,089 subgroup points and canonical scalars. The reader checks both input digests, applies the accepted signed endomorphism decomposition and Booth digit rule, and excludes zero digits and identity points. The earlier runtime audit found byte-identical grouping inputs for this fixed 1×2 request. The trace reader reconstructs 22 partition intervals per request from the six verified C7a streams and pairs each interval with its actual window and half. This analysis adds no proof calls. [Runtime input audit](../2026-11-17-group-input212/README.md) · [Matched C7a traces](../2026-11-18-g2-interior256/README.md)

For this fixture, dividing ordinary digit arrivals evenly would place the boundary after row 15,532. A reusable implementation would estimate a boundary from the current proof and include the estimator in query timing. Even perfect balance between two parts would leave most of the G2 cost. [Per-window arrival counts](artifacts/counts.json) · [Per-block timing](artifacts/blocks.json)

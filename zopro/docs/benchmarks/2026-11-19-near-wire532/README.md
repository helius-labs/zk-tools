# Short residuals between varying public-fixture scalars

The discovery pass searches for pairs of rows whose scalar difference stays narrow across training inputs after a sign, optional BN254 endomorphism and fixed translation. A match predicts y = slope·x + offset + residual modulo the scalar field. A prepared key could fold the slope into a point sum and the offset into a correction point, leaving one representative and an exact online residual. This screen scores only scalar digits. It excludes group preparation, extra memory, grouping already performed by the prover and scheduler effects.

For each shape, whole-row offsets fixed on all training witnesses are subtracted first. Each eligible row must have at least three distinct training values. Training differences against the first witness are binned at 48, 96 and 128 bits with two grid offsets. Each bin retains at most six representative targets. Exact affine pairs are excluded because an earlier screen covered them. Among overlapping edges a greedy pass chooses disjoint pairs with positive training savings. The bounded search can miss candidates, including at bin boundaries. The frozen pairs are then evaluated on four separate requests for each shape with exact scalar reconstruction. A synthetic control tests a near-endomorphism pair with varying nonzero residuals and excludes an exactly affine pair.

| Shape | Held-out input | Eligible varying rows | Profitable edges / selected pairs | Ordinary / paired modeled digits | Change | Narrow / zero residuals |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 5 | 25,390 | 0 / 0 | 592,802 / 592,802 | +0.000% | 0 / 0 |
| 1x2 | 6 | 25,390 | 0 / 0 | 592,815 / 592,815 | +0.000% | 0 / 0 |
| 1x2 | 7 | 25,390 | 0 / 0 | 592,903 / 592,903 | +0.000% | 0 / 0 |
| 1x2 | 8 | 25,390 | 0 / 0 | 592,707 / 592,707 | +0.000% | 0 / 0 |
| 2x2 | 34 | 44,555 | 2 / 1 | 979,269 / 979,249 | -0.002% | 1 / 0 |
| 2x2 | 36 | 44,555 | 2 / 1 | 979,407 / 979,387 | -0.002% | 1 / 0 |
| 2x2 | 38 | 44,555 | 2 / 1 | 979,495 / 979,475 | -0.002% | 1 / 0 |
| 2x2 | 40 | 44,555 | 2 / 1 | 979,501 / 979,481 | -0.002% | 1 / 0 |

All 17 archived proof exports were independently verified again. [Exact counts and fixture hashes](artifacts/counts.json) · [Bounded search](artifacts/screen.py) · [Signed digit model](artifacts/zp_glv513.py)

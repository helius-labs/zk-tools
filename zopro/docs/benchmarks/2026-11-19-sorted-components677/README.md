# Sorted point suffix sums in the two BN254 scalar components

For sorted positive coefficients in a block, each scalar is the sum of the first coefficient and all adjacent gaps leading up to it. This rewrites a weighted point sum as gaps multiplied by point suffix sums. One experiment sorts coefficients after exact and sign grouping and splits each gap with the original endomorphism. The other sorts the two signed GLV components independently, folding each sign into its corresponding point. The high component's final sum uses the endomorphism. Both transformations were checked modulo the BN254 scalar modulus on every input using independent model point weights. The scanner reconstructs each signed component from its Booth digits.

The work model counts nonzero signed 12-bit digit insertions, subtracts one seed for each occupied absolute-digit bucket in each window, and adds one point addition per constructed nontrivial suffix. It assumes one bucket set per window and excludes sorting, memory traffic, reduction of weighted buckets, exceptional cases, field operation costs and parallel span. These are operation counts on public witness captures. The test reverified seventeen archived proofs and produced no new latency measurement. The previous fixed-index difference experiment changed the point basis in input order. Here each proof determines the coefficient order.

The last columns select the lowest operation count across block sizes 2, 8, 32 and the full list for each witness. Such per-input hindsight is favorable to each candidate. The short-gap column counts neighboring signed-magnitude scalar classes separated by a difference of at most 126 bits.

| Shape | Fixture | Signed classes | Short gaps | Ordinary modeled adds | Best scalar gaps vs ordinary (block) | Best component gaps vs ordinary (block) |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 0 | 26,139 | 3 | 553,780 | +2.37% (2) | -0.36% (8) |
| 1x2 | 1 | 26,793 | 16 | 567,961 | +2.36% (2) | -0.39% (8) |
| 1x2 | 2 | 26,789 | 16 | 567,839 | +2.37% (2) | -0.36% (8) |
| 1x2 | 3 | 26,795 | 16 | 568,025 | +2.35% (2) | -0.40% (8) |
| 1x2 | 4 | 26,789 | 16 | 567,837 | +2.35% (2) | -0.38% (8) |
| 1x2 | 5 | 26,795 | 16 | 568,001 | +2.36% (2) | -0.38% (8) |
| 1x2 | 6 | 26,791 | 16 | 567,969 | +2.35% (2) | -0.39% (8) |
| 1x2 | 7 | 26,797 | 16 | 568,058 | +2.37% (2) | -0.37% (8) |
| 1x2 | 8 | 26,788 | 16 | 567,848 | +2.36% (2) | -0.39% (8) |
| 2x2 | 33 | 29,199 | 18 | 620,760 | +2.35% (2) | -0.44% (8) |
| 2x2 | 34 | 29,205 | 19 | 620,885 | +2.36% (2) | -0.42% (8) |
| 2x2 | 35 | 29,211 | 19 | 621,059 | +2.34% (2) | -0.42% (8) |
| 2x2 | 36 | 29,198 | 18 | 620,784 | +2.35% (2) | -0.43% (8) |
| 2x2 | 37 | 29,201 | 18 | 620,811 | +2.35% (2) | -0.43% (8) |
| 2x2 | 38 | 29,207 | 19 | 620,949 | +2.35% (2) | -0.43% (8) |
| 2x2 | 39 | 29,213 | 19 | 621,091 | +2.35% (2) | -0.43% (8) |
| 2x2 | 40 | 29,197 | 18 | 620,757 | +2.34% (2) | -0.42% (8) |

[Scalar decomposition and Booth digits](../2026-11-19-trained-digits541/artifacts/zp_glv513.py) · [Scanner](artifacts/screen.py) · [Per-witness work counts and proof hashes](artifacts/counts.json) · [Fixed-order difference study](../2026-10-26-local-difference-basis/README.md)

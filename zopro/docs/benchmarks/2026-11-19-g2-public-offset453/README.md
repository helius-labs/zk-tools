# Subtracting fixed public offsets from B-query scalars

A key could retain a correction point equal to the sum of each selected B2 base times a fixed public coefficient. For every proof, subtract the coefficient from the corresponding scalar modulo the BN254 scalar field, evaluate the residual query and add the correction once. This identity holds even when a new scalar differs from its reference. The experiment uses only public synthetic fixtures to choose coefficients. It calculates scalar work and storage before building a new proving path.

Five training requests selected indices with the same nonzero coefficient in every request. Four separate inputs vary amounts, blindings and Merkle trees. A comparison uses all nonzero coefficients from the first training request. Counts use the existing width-12 signed decomposition and measure nonzero digit updates only.

| Coefficients | Input | Selected rows | Exact matches | Original / residual updates on selected rows | All original updates | Fraction removed |
|---|---:|---:|---:|---:|---:|---:|
| Stable on five | 0 | 1,168 | 1,168 | 21,575 / 0 | 613,891 | 3.51% |
| Stable on five | 5 | 1,168 | 1,154 | 21,561 / 14 | 614,349 | 3.51% |
| Stable on five | 6 | 1,168 | 1,155 | 21,562 / 13 | 614,364 | 3.51% |
| Stable on five | 7 | 1,168 | 1,155 | 21,562 / 13 | 614,452 | 3.51% |
| Stable on five | 8 | 1,168 | 1,158 | 21,569 / 94 | 614,182 | 3.50% |
| First fixture | 0 | 28,489 | 28,489 | 613,891 / 0 | 613,891 | 100.00% |
| First fixture | 5 | 28,489 | 1,348 | 613,556 / 592,090 | 614,349 | 3.49% |
| First fixture | 6 | 28,489 | 1,325 | 613,588 / 592,164 | 614,364 | 3.49% |
| First fixture | 7 | 28,489 | 1,345 | 613,569 / 592,222 | 614,452 | 3.47% |
| First fixture | 8 | 28,489 | 1,365 | 613,564 / 591,929 | 614,182 | 3.52% |

The stable selection removes a median 3.51% of nominal digit updates across four held-out inputs. Storing a 32-bit index and a 32-byte field element for each selected row plus one 128-byte affine G2 point would use 42,176 payload bytes. This excludes container overhead, setup, comparisons and modular subtraction. No proving code or key table was changed. All nine original proof exports were independently reverified.

[Counts](artifacts/counts.json) · [Capture identities](artifacts/sources.json) · [Related moving-row analysis](../2026-11-19-g2-affine-relations429/README.md)

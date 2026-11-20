# AVX batches of independent wire products on Ryzen

The packet trial did not produce a repeatable full-proof improvement across both inputs of a shape. With four lanes required, 4×2 reached 2,857 vectorized rows in 432 batches per proof and showed a 0.48–0.54% reduction in paired medians with four wins out of six on each input.

The trial scans consecutive instructions in each solver task. It batches unit-coefficient product assignments when their operands are available before the batch and preserves every original output wire and row write. It uses temporary arrays for eight field values per input and no persistent metadata. The first route check exposed the first-instruction marker in a tagged Poseidon task. The corrected build removes the marker before looking up an operation and keeps the original boundaries of specialized Poseidon spans.

| Shape | Request | Profile | Proof ms | Paired vs scalar | Vectorized rows / batches | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | Scalar control | 7.652 | +0.00% (0/6) | 0 / 0 | 58.2 |
| 1×2 | catalog | AVX batches of at least four | 7.544 | -1.32% (5/6) | 1,445 / 263 | 58.2 |
| 1×2 | catalog | AVX batches of eight | 7.625 | -0.00% (3/6) | 680 / 85 | 58.2 |
| 1×2 | heldout | Scalar control | 7.453 | +0.00% (0/6) | 0 / 0 | 60.0 |
| 1×2 | heldout | AVX batches of at least four | 7.492 | +0.74% (2/6) | 1,445 / 263 | 58.8 |
| 1×2 | heldout | AVX batches of eight | 7.444 | +0.10% (3/6) | 680 / 85 | 58.5 |
| 2×2 | catalog | Scalar control | 11.257 | +0.00% (0/6) | 0 / 0 | 87.5 |
| 2×2 | catalog | AVX batches of at least four | 11.259 | +0.11% (3/6) | 2,058 / 343 | 90.8 |
| 2×2 | catalog | AVX batches of eight | 11.245 | -0.20% (4/6) | 912 / 114 | 89.0 |
| 2×2 | heldout | Scalar control | 11.295 | +0.00% (0/6) | 0 / 0 | 86.5 |
| 2×2 | heldout | AVX batches of at least four | 11.300 | -0.30% (4/6) | 2,058 / 343 | 95.0 |
| 2×2 | heldout | AVX batches of eight | 11.272 | -0.26% (4/6) | 912 / 114 | 85.8 |
| 4×2 | catalog | Scalar control | 15.073 | +0.00% (0/6) | 0 / 0 | 214.8 |
| 4×2 | catalog | AVX batches of at least four | 15.058 | -0.48% (4/6) | 2,857 / 432 | 216.2 |
| 4×2 | catalog | AVX batches of eight | 15.088 | -0.20% (3/6) | 1,272 / 159 | 219.8 |
| 4×2 | heldout | Scalar control | 15.079 | +0.00% (0/6) | 0 / 0 | 222.0 |
| 4×2 | heldout | AVX batches of at least four | 15.043 | -0.54% (4/6) | 2,857 / 432 | 218.2 |
| 4×2 | heldout | AVX batches of eight | 15.102 | +0.02% (3/6) | 1,272 / 159 | 213.8 |
| 8×2 | catalog | Scalar control | 24.846 | +0.00% (0/6) | 0 / 0 | 372.8 |
| 8×2 | catalog | AVX batches of at least four | 24.668 | -0.76% (5/6) | 4,819 / 790 | 370.5 |
| 8×2 | catalog | AVX batches of eight | 24.659 | -0.42% (4/6) | 2,000 / 250 | 367.2 |
| 8×2 | heldout | Scalar control | 25.061 | +0.00% (0/6) | 0 / 0 | 371.0 |
| 8×2 | heldout | AVX batches of at least four | 24.965 | +0.28% (3/6) | 4,819 / 790 | 386.8 |
| 8×2 | heldout | AVX batches of eight | 24.840 | -0.13% (3/6) | 2,000 / 250 | 385.5 |

All profiles use 24 main and eight frontend Ryzen logical workers. Six paired timing blocks prime three proofs and time thirteen. A separate diagnostic block checks dispatch. The corrected arithmetic test passed. Gnark independently verified all 168 exports and the single completed control export from the initial trial.

[Summary](artifacts/summary.json) · [Samples and batch counts](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)

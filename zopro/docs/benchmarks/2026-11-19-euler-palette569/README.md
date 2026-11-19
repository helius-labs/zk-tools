# A compact public origin for the Euler H query

A bounded Misra–Gries pass retains at most 511 candidate public field pairs from the previously frozen modal reference. A second pass counts those candidates exactly. A palette retains up to 1, 15 or 255 nonzero pairs, with a zero fallback for all remaining rows. Each row costs one bit, four bits or one byte. The same public reference is used for every evaluation input of a shape.

Each encoded origin is decoded and consumed by the native Euler scalar observer. Counts use complete field calculations on the original and independent held-out requests. The SDK exports the ordinary proof, which is independently verified. The reported file size includes the reference dictionary and the packed row identifiers. The observer expands these files before evaluation, so these are storage and sparsity measurements.

| Shape | Request | Removed nonzero H terms with 1-bit / 4-bit / 8-bit / full modal origin | Packed KiB for 1 / 4 / 8 bits |
|---|---:|---:|---:|
| 1x2 | Original | 3.92% / 5.56% / 7.88% / 8.51% | 3.9 / 16.0 / 46.1 |
| 1x2 | 29 | 3.84% / 5.52% / 7.88% / 12.69% | 3.9 / 16.0 / 46.1 |
| 1x2 | 32 | 3.86% / 5.54% / 7.89% / 12.71% | 3.9 / 16.0 / 46.1 |
| 2x2 | Original | 3.80% / 5.61% / 7.86% / 12.80% | 6.7 / 27.3 / 68.6 |
| 2x2 | 37 | 3.79% / 5.61% / 7.86% / 12.79% | 6.7 / 27.3 / 68.6 |
| 2x2 | 40 | 3.74% / 5.56% / 7.79% / 12.73% | 6.7 / 27.3 / 68.6 |
| 4x2 | Original | 3.84% / 5.64% / 8.08% / 9.26% | 12.1 / 49.1 / 112.3 |
| 4x2 | 37 | 3.82% / 5.69% / 8.14% / 12.37% | 12.1 / 49.1 / 112.3 |
| 4x2 | 40 | 3.80% / 5.69% / 8.13% / 12.37% | 12.1 / 49.1 / 112.3 |
| 8x2 | Original | 3.81% / 5.64% / 7.95% / 9.72% | 23.2 / 93.5 / 201.1 |
| 8x2 | 37 | 3.84% / 5.81% / 8.12% / 12.51% | 23.2 / 93.5 / 201.1 |
| 8x2 | 40 | 3.79% / 5.73% / 8.04% / 12.43% | 23.2 / 93.5 / 201.1 |
| 12x2 | Original | 3.77% / 5.79% / 8.05% / 12.42% | 34.3 / 137.8 / 289.6 |
| 12x2 | 53 | 3.71% / 5.74% / 7.99% / 12.35% | 34.3 / 137.8 / 289.6 |
| 12x2 | 56 | 3.70% / 5.73% / 7.99% / 12.34% | 34.3 / 137.8 / 289.6 |
| 12x2 | 141 | 2.44% / 3.57% / 5.79% / 9.99% | 34.3 / 137.8 / 289.6 |
| 12x2 | 142 | 2.41% / 3.54% / 5.77% / 9.97% | 34.3 / 137.8 / 289.6 |
| 16x2 | Original | 3.82% / 5.73% / 7.96% / 9.94% | 45.4 / 182.2 / 378.4 |
| 16x2 | 69 | 3.80% / 5.85% / 8.08% / 12.46% | 45.4 / 182.2 / 378.4 |
| 16x2 | 72 | 3.79% / 5.86% / 8.09% / 12.46% | 45.4 / 182.2 / 378.4 |
| 24x2 | Original | 3.72% / 5.66% / 7.86% / 10.01% | 67.7 / 271.6 / 557.2 |
| 24x2 | 93 | 3.69% / 5.83% / 8.03% / 12.48% | 67.7 / 271.6 / 557.2 |
| 24x2 | 96 | 3.69% / 5.82% / 8.02% / 12.47% | 67.7 / 271.6 / 557.2 |
| merge-24x1 | Original | 3.91% / 5.62% / 6.77% / 50.85% | 64.3 / 257.7 / 529.4 |
| merge-24x1 | 109 | 3.92% / 5.89% / 7.58% / 70.93% | 64.3 / 257.7 / 529.4 |
| merge-24x1 | 112 | 3.89% / 5.58% / 6.73% / 50.88% | 64.3 / 257.7 / 529.4 |
| merge-24x1 | 122 | 2.59% / 3.84% / 5.83% / 38.13% | 64.3 / 257.7 / 529.4 |
| merge-24x1 | 124 | 2.58% / 3.70% / 5.51% / 31.46% | 64.3 / 257.7 / 529.4 |

All 84 original proof exports verified. [Rust encoder and decoder](artifacts/palette.rs) · [Counts](artifacts/rows.json) · [Origin metadata](artifacts/palettes.json) · [Verification](artifacts/verification.json)

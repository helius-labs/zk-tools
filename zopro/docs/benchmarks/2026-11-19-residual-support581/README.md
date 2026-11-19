# Whole-pack FFT support after subtracting public Euler row pairs

The reader uses the four archived, independently verified generated row captures for each supplied shape. In each leave-one-out fold, the held-out A/B row pair is compared with the majority pair from the other three fixtures. If all three differ, the reference is zero. This three-fixture reference differs from the four-fixture production candidate used in the mapped-image timings. Every input byte and capture digest was checked.

A four-row SIMD pack can be skipped by the final pointwise product when all four centered row pairs vanish. Domain padding is excluded from the first percentage. For the five outer radix-32 forward layers, the reader propagates the required output packs backwards across each real butterfly pair. Neither output required permits omitting the entire butterfly, including its root multiplication. One output required still needs the root product. This second percentage includes domain padding in both arms. Counts are structural opportunities. Realized savings depend on traversal costs and the active key layout.

| Shape | Empty four-row packs, ordinary / centered | Entire outer butterflies unnecessary, ordinary / centered | Distinct centered 32-stripe masks (median) | Most common mask's column share |
|---|---:|---:|---:|---:|
| 12x2 | 1.06% / 11.38% | 0.21% / 1.95% | 126 | 11.8% |
| 16x2 | 1.06% / 11.51% | 0.13% / 1.15% | 218 | 4.5% |
| 1x2 | 0.80% / 11.63% | 0.00% / 0.31% | 48 | 9.0% |
| 24x2 | 1.06% / 11.50% | 0.22% / 1.89% | 226 | 11.0% |
| 2x2 | 0.91% / 11.70% | 0.00% / 0.75% | 67 | 12.4% |
| 4x2 | 1.00% / 11.33% | 0.18% / 1.45% | 96 | 14.0% |
| 8x2 | 1.04% / 11.42% | 0.13% / 1.26% | 156 | 11.1% |
| merge-24x1 | 1.15% / 82.98% | 0.23% / 40.06% | 110 | 6.4% |

The values summarize four folds per shape. The 32 underlying proof identities were previously verified in the [Euler origin study](../2026-11-19-euler-origin566/README.md). This analysis runs no new prover timings. [Raw level counts and identities](artifacts/counts.json) · [Streaming reader](artifacts/read.rs)

# Original-key verification after folding the Euler cross terms

The experimental CPU SDK reads a fixed public origin, subtracts its two evaluation vectors before the Euler product, and folds the resulting linear terms into the actual public and private query coefficients. A group-valued transposed derivative includes the domain padding. The converter retains additional solver expression slots when their combined key coefficient is nonzero and refreshes the public fixed-base window. The existing public wire offset setup runs against the transformed coefficients.

A native test compares the group-valued transpose with the native field Euler computation on three domains, two generators and both full and padded input lengths. The solver metadata test reconstructs left, right and output rows on the original and reloaded solver plans. The zero-origin control and every original or held-out SDK proof are verified using the original verification key and expected public-input digest. Every transformed run rebuilds the key before warmup. Setup uses full-sized group scratch and the current prototype is intended for a converter on a workstation. These single process pairs are a functional measurement with a fixed order.

| Shape | Evaluation request | Ordinary / transformed ms | Peak MiB ordinary / transformed, including setup | Key conversion seconds | Additional K slots |
|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 6.974 / 7.519 | 62.2 / 96.5 | 2.695 | 9,842 |
| 1x2 | 29 | 6.876 / 7.419 | 61.8 / 98.8 | 2.718 | 9,842 |
| 1x2 | 32 | 6.821 / 7.413 | 65.0 / 91.5 | 2.726 | 9,842 |
| 2x2 | Original | 9.564 / 11.268 | 116.5 / 155.5 | 5.804 | 17,266 |
| 2x2 | 37 | 9.604 / 11.154 | 113.0 / 146.5 | 5.834 | 17,266 |
| 2x2 | 40 | 9.553 / 11.425 | 110.2 / 152.5 | 5.861 | 17,266 |

All 13 exports verified independently. [Proofs and timings](artifacts/processes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

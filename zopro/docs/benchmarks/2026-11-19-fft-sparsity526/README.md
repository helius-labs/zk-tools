# Whole-vector equality in the first five inverse FFT layers

The SDK observed each original shape and three varied public requests. A scalar replay used the plan's actual inverse twiddles on the padded A and B evaluations through the first five decimation-in-frequency layers. It counted complete eight-lane butterflies in three layouts: the current four A plus four B lanes, separate eight-lane A and B vectors, and separate eight-lane A and B−A vectors. Linearity permits computing the third layout's inputs by subtraction after each stage. Equal operands give a zero difference that could skip a vector twiddle multiplication. An all-zero side can simplify additions. The two measures overlap.

These are operation counts over the five input layers. They include the public padding and exclude branching, conversion, recombination, inner FFT layers and all curve queries. They do not measure a latency improvement. Each process observed only its first proof, retained aggregate counts and exported a proof that verified independently.

| Shape | Request | Domain | Equal operands, current / separate / B−A | Any zero side, current / separate / B−A |
|---|---|---:|---:|---:|
| 1x2 | Original | 32,768 | 0.00% / 0.02% / 0.00% | 2.55% / 3.12% / 2.67% |
| 2x2 | Original | 65,536 | 0.02% / 0.02% / 0.01% | 7.44% / 7.90% / 7.54% |
| 4x2 | Original | 131,072 | 0.20% / 0.37% / 0.25% | 10.40% / 10.98% / 10.56% |
| 8x2 | Original | 262,144 | 0.14% / 0.28% / 0.18% | 11.51% / 12.05% / 11.67% |
| 12x2 | Original | 524,288 | 0.21% / 0.45% / 0.29% | 19.06% / 19.60% / 19.22% |
| 16x2 | Original | 524,288 | 0.14% / 0.28% / 0.18% | 12.13% / 12.67% / 12.29% |
| 24x2 | Original | 1,048,576 | 0.23% / 0.47% / 0.31% | 19.32% / 19.86% / 19.48% |
| merge-24x1 | Original | 1,048,576 | 0.23% / 0.51% / 0.31% | 20.41% / 20.96% / 20.56% |
| 1x2 | 31 | 32,768 | 0.00% / 0.01% / 0.00% | 2.54% / 3.04% / 2.69% |
| 12x2 | 142 | 524,288 | 0.02% / 0.14% / 0.12% | 18.65% / 18.94% / 18.88% |
| 24x2 | 103 | 1,048,576 | 0.22% / 0.43% / 0.30% | 19.30% / 19.78% / 19.46% |

[Stage counts](artifacts/inputs-and-counts.json) · [Counting code](artifacts/sparsity.rs) · [Verification](artifacts/verification.json)

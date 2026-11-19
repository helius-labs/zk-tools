# Class reductions bounded by input point count

The ordinary B2 point reducer batches up to 256 scalar classes per task. The experimental reducer uses the same capacity for class descriptors and also limits the number of input points per task. It cuts the existing ordered class interval into disjoint tiles using the cumulative point counts. A class larger than the target stays intact. The output order, affine additions, and following MSM are unchanged. The trial uses the normal tile geometry when the workspace policy declines the task list allocation.

Ryzen uses 24 main and eight frontend logical CPUs. The released SDK, the same-build control, and three limits use five matched blocks per input. 8×2 enables Poseidon origins and serial class conversion throughout. Each process primes three requests and measures thirteen. All 108 exported proofs independently verified, including zero optional memory on the two primary requests per circuit.

None of the point limits beat both controls consistently on either shape. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.661 | +1.04% (0/5) | +0.00% (0/5) | 201.5 |
| 4×2 | catalog | ordinary | 14.563 | +0.00% (0/5) | -1.03% (5/5) | 206.8 |
| 4×2 | catalog | points-384 | 14.701 | +1.05% (0/5) | +0.01% (2/5) | 212.5 |
| 4×2 | catalog | points-768 | 14.418 | -0.47% (5/5) | -1.37% (5/5) | 202.5 |
| 4×2 | catalog | points-1536 | 14.537 | -0.17% (4/5) | -0.75% (5/5) | 217.0 |
| 4×2 | heldout | release | 14.444 | +0.64% (2/5) | +0.00% (0/5) | 194.0 |
| 4×2 | heldout | ordinary | 14.433 | +0.00% (0/5) | -0.64% (3/5) | 212.5 |
| 4×2 | heldout | points-384 | 14.381 | -0.10% (3/5) | -0.34% (5/5) | 202.8 |
| 4×2 | heldout | points-768 | 14.545 | +0.77% (2/5) | -0.23% (3/5) | 238.8 |
| 4×2 | heldout | points-1536 | 14.413 | -0.13% (3/5) | -0.76% (3/5) | 205.5 |
| 8×2 | catalog | release | 22.735 | +0.54% (2/5) | +0.00% (0/5) | 405.2 |
| 8×2 | catalog | ordinary | 22.613 | +0.00% (0/5) | -0.53% (3/5) | 417.8 |
| 8×2 | catalog | points-384 | 22.524 | -0.39% (3/5) | -0.96% (3/5) | 411.2 |
| 8×2 | catalog | points-768 | 22.588 | -0.69% (5/5) | -1.22% (4/5) | 407.0 |
| 8×2 | catalog | points-1536 | 22.471 | -0.64% (4/5) | -1.17% (4/5) | 400.2 |
| 8×2 | heldout | release | 22.637 | -1.88% (3/5) | +0.00% (0/5) | 404.5 |
| 8×2 | heldout | ordinary | 22.940 | +0.00% (0/5) | +1.91% (2/5) | 416.0 |
| 8×2 | heldout | points-384 | 22.871 | -2.12% (3/5) | -0.02% (3/5) | 411.8 |
| 8×2 | heldout | points-768 | 22.612 | -0.84% (3/5) | -0.11% (3/5) | 400.8 |
| 8×2 | heldout | points-1536 | 22.700 | -1.29% (3/5) | +0.70% (2/5) | 394.5 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

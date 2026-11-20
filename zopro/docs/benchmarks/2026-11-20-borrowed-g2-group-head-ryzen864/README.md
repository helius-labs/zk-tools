# Borrowing first-layer operands in grouped G2 on Ryzen

The trial gathers only the first point from each ordinary pair into the grouped reducer's first working array. The existing borrowed affine addition reads the second point from the original key. Singleton and completed two-point classes go straight to the output, while remaining classes use the existing limb-based reduction. Oversized groups of 256 classes and declined optional workspace retain the original reducer. The native test compares complete group sums with independent generator multiples and covers empty and oversized classes, inverses and identities.

There are five matched blocks per input on 24 main and eight frontend Ryzen logical CPUs. Each process primes three proofs and times thirteen. All 144 exports independently verified. Route probes confirmed that 2×2 used the borrowed first layer and retained the original route at zero optional workspace. The 4×2 and 8×2 probes exposed a coverage gap. On those shapes, the grouped G2 dispatcher went through its generic branch and bypassed the experimental reducer. Their retained timing rows provide no estimate of its speed on larger inputs.

The borrowed-first-layer variant did not improve both controls consistently across both inputs on any tested shape.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 2×2 | catalog | release | 9.535 | -0.41% (3/5) | +0.00% (0/5) | 94.0 |
| 2×2 | catalog | ordinary | 9.511 | +0.00% (0/5) | +0.41% (2/5) | 96.2 |
| 2×2 | catalog | borrow | 9.470 | -0.71% (3/5) | -0.42% (5/5) | 93.5 |
| 2×2 | heldout | release | 9.522 | +0.57% (1/5) | +0.00% (0/5) | 99.0 |
| 2×2 | heldout | ordinary | 9.503 | +0.00% (0/5) | -0.57% (4/5) | 92.5 |
| 2×2 | heldout | borrow | 9.509 | +0.10% (2/5) | -0.10% (4/5) | 99.2 |

## Group point reduction timing

Three matched blocks separately time B2 scalar-class point reduction in the same executable. Clocked proofs have unclocked companions. The effect columns describe the full proof after adding the clock.

| Shape | Input | Ordinary ms | Borrowed first layer ms | Paired reduction change | Full-proof clock effect ordinary / borrowed |
|---|---|---:|---:|---:|---:|
| 2×2 | catalog | 0.850 | 0.784 | -6.04% (2/3) | +0.51% / +0.32% |
| 2×2 | heldout | 0.860 | 0.800 | -3.75% (2/3) | -0.05% / +0.67% |

[Source](artifacts/source.patch) · [Design](artifacts/design.md) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

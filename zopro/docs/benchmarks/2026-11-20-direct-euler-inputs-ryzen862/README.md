# Loading borrowed rows in the fused Euler FFT on Ryzen

This follow-up to the [mixed-radix screen](../2026-11-20-mixed-radix-euler-ryzen861/README.md) loads complete rows directly from the original solver arrays. Each inverse butterfly receives two borrowed groups of four scalars. Each final multiplication borrows eight contiguous values when available and uses zero padding for a short row. Both mixed-radix builds perform the same field arithmetic. The new build changes the load sites and retains the original proving key and scratch.

There are five matched blocks per input on 24 main and eight frontend Ryzen logical CPUs. Each process primes three proofs and times thirteen. The retained mixed-radix build and released SDK are separate controls. All 168 proof exports independently verified, and all 20 native FFT tests passed.

The direct-load variant did not improve all three controls consistently on any tested shape.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs prior mixed | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 2×2 | catalog | release | 9.504 | +0.33% (2/5) | +0.76% (0/5) | +0.00% (0/5) | 94.0 |
| 2×2 | catalog | ordinary | 9.480 | +0.00% (0/5) | +0.59% (1/5) | -0.32% (3/5) | 93.2 |
| 2×2 | catalog | previous | 9.445 | -0.59% (4/5) | +0.00% (0/5) | -0.75% (5/5) | 98.0 |
| 2×2 | catalog | mixed | 9.445 | -0.51% (4/5) | +0.02% (2/5) | -0.83% (5/5) | 93.5 |
| 2×2 | heldout | release | 9.523 | +0.27% (2/5) | +0.28% (0/5) | +0.00% (0/5) | 98.5 |
| 2×2 | heldout | ordinary | 9.503 | +0.00% (0/5) | +0.73% (2/5) | -0.27% (3/5) | 98.2 |
| 2×2 | heldout | previous | 9.459 | -0.72% (3/5) | +0.00% (0/5) | -0.28% (5/5) | 99.5 |
| 2×2 | heldout | mixed | 9.475 | -0.51% (3/5) | +0.63% (2/5) | -0.50% (4/5) | 94.2 |
| 4×2 | catalog | release | 13.644 | -0.11% (3/5) | +0.01% (2/5) | +0.00% (0/5) | 206.5 |
| 4×2 | catalog | ordinary | 13.779 | +0.00% (0/5) | +0.35% (1/5) | +0.11% (2/5) | 220.0 |
| 4×2 | catalog | previous | 13.726 | -0.35% (4/5) | +0.00% (0/5) | -0.01% (3/5) | 201.8 |
| 4×2 | catalog | mixed | 13.637 | -0.40% (4/5) | -0.23% (4/5) | -0.24% (4/5) | 209.8 |
| 4×2 | heldout | release | 13.734 | -0.35% (4/5) | -0.63% (3/5) | +0.00% (0/5) | 199.2 |
| 4×2 | heldout | ordinary | 13.796 | +0.00% (0/5) | +1.49% (2/5) | +0.35% (1/5) | 197.8 |
| 4×2 | heldout | previous | 13.612 | -1.47% (3/5) | +0.00% (0/5) | +0.63% (2/5) | 204.8 |
| 4×2 | heldout | mixed | 13.756 | -0.29% (3/5) | +0.88% (2/5) | +0.30% (1/5) | 197.8 |
| 8×2 | catalog | release | 22.355 | -0.02% (3/5) | +0.11% (2/5) | +0.00% (0/5) | 358.5 |
| 8×2 | catalog | ordinary | 22.322 | +0.00% (0/5) | +0.17% (2/5) | +0.02% (2/5) | 369.2 |
| 8×2 | catalog | previous | 22.237 | -0.17% (3/5) | +0.00% (0/5) | -0.11% (3/5) | 366.8 |
| 8×2 | catalog | mixed | 22.121 | +0.52% (2/5) | -0.23% (4/5) | -1.05% (4/5) | 383.2 |
| 8×2 | heldout | release | 22.260 | +0.27% (2/5) | -0.32% (4/5) | +0.00% (0/5) | 377.8 |
| 8×2 | heldout | ordinary | 22.323 | +0.00% (0/5) | -0.58% (3/5) | -0.27% (3/5) | 384.8 |
| 8×2 | heldout | previous | 22.456 | +0.59% (2/5) | +0.00% (0/5) | +0.32% (1/5) | 377.5 |
| 8×2 | heldout | mixed | 22.694 | +1.44% (1/5) | +0.56% (2/5) | +0.94% (0/5) | 374.8 |

## Quotient timing

Three matched blocks separately time the complete packed Euler scope in the same executable. Clocked proofs have unclocked companions. The effect columns describe the full proof after adding the clock.

| Shape | Input | Ordinary ms | Mixed with direct loads ms | Paired FFT change | Full-proof clock effect ordinary / mixed |
|---|---|---:|---:|---:|---:|
| 2×2 | catalog | 1.160 | 1.197 | +4.29% (1/3) | +0.45% / +0.21% |
| 2×2 | heldout | 1.223 | 1.208 | +0.37% (1/3) | -0.63% / -0.05% |
| 4×2 | catalog | 2.222 | 2.170 | -3.02% (3/3) | +0.70% / +0.09% |
| 4×2 | heldout | 2.234 | 2.174 | -1.95% (3/3) | +0.21% / -0.44% |
| 8×2 | catalog | 3.954 | 4.003 | +1.18% (1/3) | -0.56% / +0.00% |
| 8×2 | heldout | 3.974 | 4.005 | +4.12% (1/3) | +0.89% / -0.59% |

[Source](artifacts/source.patch) · [Design](artifacts/design.md) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

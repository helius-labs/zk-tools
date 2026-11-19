# Independent sums for the two G2 endomorphism components

The trial uses the prover's existing signed decomposition into two short components. It computes their scalar products independently and applies the endomorphism to the second complete sum once. The integrated Phi control combines the components while processing window buckets. The trial runs scalar splitting alongside point grouping, and its separate component workspace is charged before use. It builds each query from the current proof and retains no point or scalar table.

Ryzen uses 24 main and eight frontend logical CPUs. 8×2 enables Poseidon origins and serial scalar conversion in all arms. Four matched blocks per input compare four component layouts with the released SDK, a same-build ordinary control, and the existing Phi mode. Each process primes three requests and measures thirteen. The test matched independent generator outputs for signed coefficients, opposite points, zeros, identities, and four geometries at one and eight workers. All 120 exported proofs independently verified. Route probes checked the ordinary fallback with zero optional workspace.

The following trial modes beat both controls in at least three of four blocks on both inputs: 4×2 pc-12x1. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.582 | +0.94% (0/4) | +0.00% (0/4) | 200.8 |
| 4×2 | catalog | ordinary | 14.426 | +0.00% (0/4) | -0.93% (4/4) | 202.0 |
| 4×2 | catalog | integrated-phi | 13.987 | -3.03% (4/4) | -4.20% (4/4) | 200.0 |
| 4×2 | catalog | pc-11x2 | 14.750 | +2.07% (0/4) | +0.83% (1/4) | 204.0 |
| 4×2 | catalog | pc-12x1 | 14.201 | -1.54% (3/4) | -2.60% (4/4) | 201.2 |
| 4×2 | catalog | pc-12x2 | 14.444 | +0.54% (1/4) | -1.05% (3/4) | 206.0 |
| 4×2 | catalog | pc-12x3 | 14.537 | +0.90% (0/4) | -0.04% (2/4) | 210.8 |
| 4×2 | heldout | release | 14.978 | +3.21% (1/4) | +0.00% (0/4) | 216.5 |
| 4×2 | heldout | ordinary | 14.544 | +0.00% (0/4) | -3.11% (3/4) | 207.8 |
| 4×2 | heldout | integrated-phi | 14.015 | -3.64% (4/4) | -6.06% (4/4) | 198.8 |
| 4×2 | heldout | pc-11x2 | 14.712 | +0.93% (1/4) | -0.64% (2/4) | 195.8 |
| 4×2 | heldout | pc-12x1 | 14.278 | -2.14% (4/4) | -4.59% (4/4) | 207.0 |
| 4×2 | heldout | pc-12x2 | 14.390 | -0.57% (3/4) | -3.09% (3/4) | 205.8 |
| 4×2 | heldout | pc-12x3 | 14.723 | +0.95% (0/4) | -1.60% (3/4) | 203.5 |
| 8×2 | catalog | release | 22.361 | -0.89% (4/4) | +0.00% (0/4) | 391.8 |
| 8×2 | catalog | ordinary | 22.567 | +0.00% (0/4) | +0.90% (0/4) | 405.8 |
| 8×2 | catalog | integrated-phi | 21.856 | -3.35% (4/4) | -2.88% (3/4) | 401.2 |
| 8×2 | catalog | pc-11x2 | 22.903 | +1.12% (1/4) | +2.02% (1/4) | 394.2 |
| 8×2 | catalog | pc-12x1 | 22.239 | -1.43% (3/4) | -0.55% (3/4) | 401.5 |
| 8×2 | catalog | pc-12x2 | 22.562 | -0.24% (3/4) | +0.25% (1/4) | 396.8 |
| 8×2 | catalog | pc-12x3 | 22.556 | -0.03% (2/4) | +0.87% (1/4) | 398.2 |
| 8×2 | heldout | release | 22.846 | +0.52% (1/4) | +0.00% (0/4) | 402.8 |
| 8×2 | heldout | ordinary | 22.755 | +0.00% (0/4) | -0.52% (3/4) | 408.8 |
| 8×2 | heldout | integrated-phi | 22.181 | -2.40% (4/4) | -2.72% (4/4) | 405.5 |
| 8×2 | heldout | pc-11x2 | 23.146 | +1.72% (1/4) | +1.13% (1/4) | 401.0 |
| 8×2 | heldout | pc-12x1 | 22.696 | -0.15% (2/4) | -0.29% (2/4) | 399.2 |
| 8×2 | heldout | pc-12x2 | 22.472 | -0.83% (3/4) | -1.68% (3/4) | 408.5 |
| 8×2 | heldout | pc-12x3 | 22.699 | -0.13% (2/4) | -0.56% (4/4) | 408.5 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

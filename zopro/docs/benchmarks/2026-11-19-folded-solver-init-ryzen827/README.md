# Solver allocation and placement on Ryzen

The trial moves solver allocation and witness copying to the existing frontend pool alongside instruction execution. It also tests the existing witness buffer mode that skips initialization only when the solver certifies definite assignment. The conditional allocation mode preserves its initialized fallback. The quotient and curve paths are shared across runs.

Ryzen uses 24 main and eight frontend logical CPUs. Both starting configurations already execute solver instructions on the frontend. 8×2 enables recognized Poseidon origins and serial B2 conversion. Four matched blocks per input measure thirteen calls after three primes. All 96 exports independently verified. The traces record which allocator placement and initialization path the solver actually used.

The following trial modes beat both controls in at least all required paired comparisons across the tested inputs: 4×2 full, 8×2 uninit. The code remains experimental.

- 4×2 used the certified uninitialized path when requested.
- 8×2 used the certified uninitialized path when requested.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.592 | +0.36% (1/4) | +0.00% (0/4) | 196.8 |
| 4×2 | catalog | ordinary | 14.519 | +0.00% (0/4) | -0.36% (3/4) | 204.2 |
| 4×2 | catalog | uninit | 14.724 | +1.46% (1/4) | +0.71% (0/4) | 202.0 |
| 4×2 | catalog | full | 14.361 | -1.35% (4/4) | -1.27% (4/4) | 207.0 |
| 4×2 | catalog | full-uninit | 14.341 | -1.15% (4/4) | -1.63% (4/4) | 201.8 |
| 4×2 | heldout | release | 14.447 | +0.69% (1/4) | +0.00% (0/4) | 196.0 |
| 4×2 | heldout | ordinary | 14.407 | +0.00% (0/4) | -0.69% (3/4) | 208.2 |
| 4×2 | heldout | uninit | 14.558 | +1.31% (0/4) | +0.80% (1/4) | 200.0 |
| 4×2 | heldout | full | 14.385 | -0.23% (3/4) | -0.39% (3/4) | 219.8 |
| 4×2 | heldout | full-uninit | 14.336 | -0.18% (2/4) | -1.27% (4/4) | 204.8 |
| 8×2 | catalog | release | 22.777 | +0.78% (1/4) | +0.00% (0/4) | 376.8 |
| 8×2 | catalog | ordinary | 22.679 | +0.00% (0/4) | -0.77% (3/4) | 382.2 |
| 8×2 | catalog | uninit | 22.457 | -1.41% (3/4) | -1.40% (3/4) | 381.8 |
| 8×2 | catalog | full | 22.862 | +0.73% (0/4) | +0.51% (2/4) | 392.0 |
| 8×2 | catalog | full-uninit | 22.816 | -0.07% (3/4) | -0.17% (2/4) | 385.8 |
| 8×2 | heldout | release | 22.820 | -1.38% (3/4) | +0.00% (0/4) | 396.0 |
| 8×2 | heldout | ordinary | 23.140 | +0.00% (0/4) | +1.40% (1/4) | 370.5 |
| 8×2 | heldout | uninit | 22.500 | -2.60% (4/4) | -1.24% (4/4) | 385.8 |
| 8×2 | heldout | full | 23.103 | +0.31% (2/4) | +1.39% (0/4) | 405.0 |
| 8×2 | heldout | full-uninit | 22.582 | -2.30% (4/4) | -1.04% (4/4) | 396.8 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

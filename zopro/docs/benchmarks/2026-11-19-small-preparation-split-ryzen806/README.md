# Solver, gather and FFT pool placement for small CPU proofs

The ordinary small CPU route solves the witness, collects canonical coefficients and computes the quotient on its frontend pool before the main-pool curve queries. In this trial the frontend still solves the witness. The main mode then runs gather and FFT sequentially on the main pool. The parallel mode runs gather and FFT concurrently on that pool. Split keeps the gather on the frontend while the FFT takes the main pool. The operations use their existing inputs and outputs. The budget admits only proof-local overlap buffers prior to the split. Budget denial retains the existing preparation sequence.

Five matched blocks on Ryzen use 24 main and eight frontend logical CPUs. The 2×2 profiles use the medium fused K/H option. Each process primes three calls and measures thirteen. All 116 exports independently verified against the pinned original keys. Route checks confirm the selected placement and fallback under zero optional memory.

The following trial modes beat both controls in at least four of five blocks on both inputs: 1×2 split, 2×2 main, 2×2 parallel, 2×2 split. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.919 | +0.80% (2/5) | +0.00% (0/5) | 57.5 |
| 1×2 | catalog | ordinary | 6.867 | +0.00% (0/5) | -0.80% (3/5) | 57.2 |
| 1×2 | catalog | main | 7.048 | +1.18% (0/5) | +1.38% (2/5) | 57.8 |
| 1×2 | catalog | parallel | 6.888 | -0.50% (3/5) | -1.04% (3/5) | 58.8 |
| 1×2 | catalog | split | 6.832 | -0.80% (4/5) | -1.59% (5/5) | 59.2 |
| 1×2 | heldout | release | 6.803 | -0.01% (3/5) | +0.00% (0/5) | 61.2 |
| 1×2 | heldout | ordinary | 6.816 | +0.00% (0/5) | +0.01% (2/5) | 57.5 |
| 1×2 | heldout | main | 6.823 | +0.30% (2/5) | -0.57% (3/5) | 57.5 |
| 1×2 | heldout | parallel | 6.829 | -0.45% (3/5) | -0.94% (4/5) | 65.8 |
| 1×2 | heldout | split | 6.740 | -1.12% (4/5) | -1.66% (5/5) | 61.5 |
| 2×2 | catalog | release | 9.471 | +0.15% (1/5) | +0.00% (0/5) | 93.8 |
| 2×2 | catalog | ordinary | 9.457 | +0.00% (0/5) | -0.15% (4/5) | 98.5 |
| 2×2 | catalog | main | 9.348 | -1.28% (4/5) | -1.36% (5/5) | 99.0 |
| 2×2 | catalog | parallel | 9.231 | -1.90% (4/5) | -2.65% (5/5) | 96.2 |
| 2×2 | catalog | split | 9.327 | -1.47% (5/5) | -1.62% (5/5) | 102.0 |
| 2×2 | heldout | release | 9.499 | +0.66% (1/5) | +0.00% (0/5) | 97.8 |
| 2×2 | heldout | ordinary | 9.458 | +0.00% (0/5) | -0.66% (4/5) | 98.2 |
| 2×2 | heldout | main | 9.431 | -0.35% (4/5) | -0.63% (4/5) | 93.0 |
| 2×2 | heldout | parallel | 9.260 | -2.10% (4/5) | -2.40% (4/5) | 88.8 |
| 2×2 | heldout | split | 9.314 | -1.45% (5/5) | -1.73% (5/5) | 104.0 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

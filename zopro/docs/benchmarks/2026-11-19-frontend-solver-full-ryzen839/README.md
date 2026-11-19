# Full frontend solver setting on Ryzen

The SDK setting `ZOPRO_CPU_SOLVER_FRONTEND=full` uses the attached smaller pool for buffer allocation, instruction execution and witness copying in the general CPU solver. The ordinary setting continues to dispatch only instructions. The full setting uses the normal solver path when the pool is detached. Both use the existing memory policy.

Ryzen uses 24 main and eight frontend logical CPUs. 8×2 uses four epochs, packed tasks, certified buffers and recognized B2 origins. Five matched blocks per input measure thirteen calls after three primes. All 72 exports independently verified. Separate probes cover the full route, zero optional workspace and a detached pool.

The full-solver setting did not beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.526 | -1.76% (5/5) | +0.00% (0/5) | 221.2 |
| 4×2 | catalog | ordinary | 14.787 | +0.00% (0/5) | +1.79% (0/5) | 200.8 |
| 4×2 | catalog | full | 14.749 | -0.28% (4/5) | +1.53% (0/5) | 209.8 |
| 4×2 | heldout | release | 14.571 | -1.42% (4/5) | +0.00% (0/5) | 199.2 |
| 4×2 | heldout | ordinary | 14.780 | +0.00% (0/5) | +1.44% (1/5) | 201.5 |
| 4×2 | heldout | full | 14.663 | -0.77% (4/5) | +0.13% (2/5) | 220.5 |
| 8×2 | catalog | release | 22.319 | -1.89% (5/5) | +0.00% (0/5) | 365.2 |
| 8×2 | catalog | ordinary | 22.787 | +0.00% (0/5) | +1.93% (0/5) | 371.0 |
| 8×2 | catalog | full | 22.834 | +0.02% (2/5) | +0.92% (0/5) | 377.0 |
| 8×2 | heldout | release | 22.544 | -2.06% (5/5) | +0.00% (0/5) | 378.0 |
| 8×2 | heldout | ordinary | 23.000 | +0.00% (0/5) | +2.11% (0/5) | 370.8 |
| 8×2 | heldout | full | 23.029 | -1.16% (3/5) | +2.15% (1/5) | 386.0 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

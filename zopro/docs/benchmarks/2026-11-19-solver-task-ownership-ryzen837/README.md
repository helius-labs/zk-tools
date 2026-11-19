# Solver task ownership on Ryzen

The trial changes task ownership inside an existing solver dependency wave. The seed mode gives each logical claimant its first task by index and claims the remaining work dynamically. The stripe mode assigns round-robin tasks from the prepared longest-first ordering. Neither mode duplicates the solver plan or changes task dependencies. The focused test checks that every task runs once and that errors propagate under each mode and worker count.

Ryzen uses 24 main and eight frontend logical CPUs. All modes use the four-epoch profile with packed tasks, recognized B2 origins and serial B2 conversion. Five matched blocks per input measure thirteen proofs after three primes. All 46 exports independently verified, including stripe probes at zero optional workspace.

Neither task ownership mode beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 8×2 | catalog | release | 22.658 | +1.12% (2/5) | +0.00% (0/5) | 368.2 |
| 8×2 | catalog | ordinary | 22.431 | +0.00% (0/5) | -1.11% (3/5) | 376.8 |
| 8×2 | catalog | seed | 22.426 | +1.06% (2/5) | -1.21% (3/5) | 379.5 |
| 8×2 | catalog | stripe | 22.775 | +1.84% (1/5) | -0.17% (3/5) | 369.2 |
| 8×2 | heldout | release | 22.646 | +0.55% (2/5) | +0.00% (0/5) | 371.8 |
| 8×2 | heldout | ordinary | 22.940 | +0.00% (0/5) | -0.55% (3/5) | 405.5 |
| 8×2 | heldout | seed | 22.543 | -0.66% (3/5) | -0.46% (4/5) | 401.0 |
| 8×2 | heldout | stripe | 23.044 | +0.89% (1/5) | +1.24% (2/5) | 384.2 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

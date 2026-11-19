# Serial allocation for the certified solver on Ryzen

The certified solver can reserve buffer capacity without zeroing every field element. The trial reserves those buffers sequentially, removing the Rayon joins normally used to initialize buffers in parallel. A second setting runs the complete solver on the existing frontend pool. Both retain initialized buffers and ordinary allocation when definite assignment cannot be established.

Ryzen uses 24 main and eight frontend logical CPUs. All modes use four solver epochs, packed work, recognized B2 origins and serial B2 conversion. Five matched blocks per input measure thirteen calls after three primes. All 58 exports independently verified. The traces record which allocation path and placement each proof used.

No allocation mode beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 8×2 | catalog | release | 22.552 | +2.02% (1/5) | +0.00% (0/5) | 385.5 |
| 8×2 | catalog | ordinary | 22.338 | +0.00% (0/5) | -1.98% (4/5) | 364.8 |
| 8×2 | catalog | serial | 22.329 | +0.88% (2/5) | -1.08% (4/5) | 365.8 |
| 8×2 | catalog | full | 22.527 | +0.26% (2/5) | -0.41% (3/5) | 380.5 |
| 8×2 | catalog | full-serial | 22.295 | -0.10% (3/5) | -1.06% (5/5) | 352.2 |
| 8×2 | heldout | release | 22.705 | -0.04% (3/5) | +0.00% (0/5) | 404.8 |
| 8×2 | heldout | ordinary | 22.649 | +0.00% (0/5) | +0.04% (2/5) | 369.5 |
| 8×2 | heldout | serial | 22.747 | +0.83% (2/5) | +0.09% (2/5) | 373.2 |
| 8×2 | heldout | full | 22.372 | -0.39% (3/5) | -1.11% (4/5) | 382.2 |
| 8×2 | heldout | full-serial | 22.492 | -0.66% (4/5) | -1.04% (5/5) | 358.8 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

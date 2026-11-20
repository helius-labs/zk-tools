# Skipping inactive point runs in small G2 queries

The trial identifies runs of nonidentity G2 points after the existing scalar split. The component mode also skips runs with zero GLV component magnitude. It reuses the selected runs across ordinary windows and borrows the original points in their original order. The per-window signed digit and bucket operations still use the existing code. Indexed top windows use the original inputs. The candidate reserves proof-local space for range descriptors.

The two shapes use 24 main and eight frontend logical CPUs on Ryzen. Both have the original and first held-out inputs, with two additional 1×2 inputs. Each input has five matched blocks, three primes and thirteen timed calls per process. All 132 exports independently verified, including the fallback at zero optional workspace. The focused test compares every bucket under both run selections, including zero components and identities.

No selected-run mode improved both controls consistently across all tested inputs of either shape. The code remains experimental.




| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.903 | -0.23% (3/5) | +0.00% (0/5) | 58.0 |
| 1×2 | catalog | ordinary | 6.919 | +0.00% (0/5) | +0.23% (2/5) | 57.8 |
| 1×2 | catalog | identities | 6.890 | -0.41% (3/5) | -0.18% (3/5) | 56.5 |
| 1×2 | catalog | components | 6.829 | -0.91% (5/5) | -0.68% (4/5) | 57.2 |
| 1×2 | heldout | release | 6.785 | +0.56% (2/5) | +0.00% (0/5) | 58.2 |
| 1×2 | heldout | ordinary | 6.744 | +0.00% (0/5) | -0.56% (3/5) | 57.0 |
| 1×2 | heldout | identities | 6.782 | +0.32% (2/5) | -0.11% (3/5) | 58.0 |
| 1×2 | heldout | components | 6.769 | -0.22% (3/5) | -0.55% (4/5) | 57.0 |
| 1×2 | v25 | release | 6.776 | -0.24% (4/5) | +0.00% (0/5) | 58.2 |
| 1×2 | v25 | ordinary | 6.792 | +0.00% (0/5) | +0.24% (1/5) | 56.8 |
| 1×2 | v25 | identities | 6.730 | -0.78% (5/5) | -0.45% (5/5) | 57.2 |
| 1×2 | v25 | components | 6.719 | -1.39% (4/5) | -0.90% (4/5) | 57.2 |
| 1×2 | v26 | release | 6.793 | +0.31% (1/5) | +0.00% (0/5) | 57.2 |
| 1×2 | v26 | ordinary | 6.763 | +0.00% (0/5) | -0.30% (4/5) | 57.2 |
| 1×2 | v26 | identities | 6.752 | -0.63% (5/5) | -1.45% (5/5) | 57.0 |
| 1×2 | v26 | components | 6.779 | +0.03% (2/5) | -0.50% (3/5) | 56.5 |
| 2×2 | catalog | release | 9.512 | +0.26% (2/5) | +0.00% (0/5) | 91.5 |
| 2×2 | catalog | ordinary | 9.547 | +0.00% (0/5) | -0.26% (3/5) | 98.5 |
| 2×2 | catalog | identities | 9.487 | -0.37% (4/5) | -0.63% (4/5) | 94.0 |
| 2×2 | catalog | components | 9.608 | +0.51% (2/5) | +0.71% (1/5) | 93.5 |
| 2×2 | heldout | release | 9.499 | -0.19% (4/5) | +0.00% (0/5) | 97.5 |
| 2×2 | heldout | ordinary | 9.518 | +0.00% (0/5) | +0.19% (1/5) | 99.8 |
| 2×2 | heldout | identities | 9.470 | -0.08% (3/5) | -0.42% (3/5) | 94.5 |
| 2×2 | heldout | components | 9.500 | +0.08% (2/5) | +0.02% (2/5) | 92.0 |

## Observed run sizes

The final column reports the point-component entries excluded from each ordinary window, summed across both components and all point partitions.

| Shape | Input | Probe | Partitions | Runs | Entries skipped / total |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | identities | 2 | 4 | 0 / 60,178 |
| 1×2 | catalog | components | 2 | 633 | 5,921 / 60,178 |
| 1×2 | heldout | identities | 2 | 4 | 0 / 60,178 |
| 1×2 | heldout | components | 2 | 667 | 5,700 / 60,178 |
| 2×2 | catalog | identities | 2 | 4 | 0 / 54,436 |
| 2×2 | catalog | components | 2 | 10 | 9 / 54,436 |
| 2×2 | heldout | identities | 2 | 4 | 0 / 54,534 |
| 2×2 | heldout | components | 2 | 9 | 11 / 54,534 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

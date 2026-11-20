# In-place folded solver slot retention on Ryzen

The folded solver keeps every witness wire and selects the extra slots needed by the K query. The compact variant selects slots directly into the solver's existing buffer. The runs variant moves consecutive selected slots together. Increasing slot indices permit forward compaction because each source index is at or above its destination. Any other ordering uses the original temporary copy. The proof retains the same wire and slot order.

Zopro uses 24 main and eight frontend logical CPUs. Four matched blocks per input prime three proofs and measure thirteen. All 144 process exports independently verified, including zero optional memory probes. The focused native test checks dense and sparse selections, arbitrary order, repetitions and original wire indices against the copy.

No in-place variant improved both controls consistently across the two inputs of any tested shape. The code remains experimental.


| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.893 | -0.20% (2/4) | +0.00% (0/4) | 57.8 |
| 1×2 | catalog | ordinary | 6.899 | +0.00% (0/4) | +0.20% (2/4) | 58.0 |
| 1×2 | catalog | compact | 6.907 | +0.33% (1/4) | +0.25% (2/4) | 58.0 |
| 1×2 | catalog | runs | 6.926 | +0.63% (2/4) | +0.39% (0/4) | 57.5 |
| 1×2 | heldout | release | 6.826 | -0.81% (4/4) | +0.00% (0/4) | 57.0 |
| 1×2 | heldout | ordinary | 6.890 | +0.00% (0/4) | +0.81% (0/4) | 57.5 |
| 1×2 | heldout | compact | 6.784 | -1.68% (3/4) | -0.36% (3/4) | 58.0 |
| 1×2 | heldout | runs | 6.789 | -1.69% (3/4) | -0.55% (3/4) | 57.2 |
| 2×2 | catalog | release | 9.452 | -0.76% (3/4) | +0.00% (0/4) | 94.8 |
| 2×2 | catalog | ordinary | 9.499 | +0.00% (0/4) | +0.77% (1/4) | 93.8 |
| 2×2 | catalog | compact | 9.480 | -0.38% (3/4) | +0.43% (1/4) | 103.2 |
| 2×2 | catalog | runs | 9.497 | -0.04% (2/4) | +0.62% (1/4) | 93.0 |
| 2×2 | heldout | release | 9.505 | +0.61% (1/4) | +0.00% (0/4) | 93.8 |
| 2×2 | heldout | ordinary | 9.433 | +0.00% (0/4) | -0.61% (3/4) | 92.8 |
| 2×2 | heldout | compact | 9.494 | +0.50% (1/4) | -0.22% (3/4) | 90.2 |
| 2×2 | heldout | runs | 9.478 | +0.67% (0/4) | +0.21% (1/4) | 94.8 |
| 4×2 | catalog | release | 14.536 | +0.25% (2/4) | +0.00% (0/4) | 205.5 |
| 4×2 | catalog | ordinary | 14.505 | +0.00% (0/4) | -0.25% (2/4) | 206.0 |
| 4×2 | catalog | compact | 14.370 | -0.59% (2/4) | -0.83% (3/4) | 205.5 |
| 4×2 | catalog | runs | 14.456 | -0.08% (2/4) | -0.24% (3/4) | 196.0 |
| 4×2 | heldout | release | 14.454 | -0.01% (2/4) | +0.00% (0/4) | 197.5 |
| 4×2 | heldout | ordinary | 14.465 | +0.00% (0/4) | +0.01% (2/4) | 200.0 |
| 4×2 | heldout | compact | 14.479 | +0.02% (2/4) | +0.32% (2/4) | 198.8 |
| 4×2 | heldout | runs | 14.407 | -0.95% (3/4) | -0.29% (2/4) | 200.5 |
| 8×2 | catalog | release | 22.477 | +0.18% (1/4) | +0.00% (0/4) | 370.0 |
| 8×2 | catalog | ordinary | 22.437 | +0.00% (0/4) | -0.18% (3/4) | 379.0 |
| 8×2 | catalog | compact | 22.432 | -0.11% (2/4) | -0.39% (3/4) | 359.2 |
| 8×2 | catalog | runs | 22.579 | +0.39% (2/4) | +0.29% (2/4) | 371.5 |
| 8×2 | heldout | release | 22.772 | +1.13% (1/4) | +0.00% (0/4) | 396.8 |
| 8×2 | heldout | ordinary | 22.688 | +0.00% (0/4) | -1.10% (3/4) | 355.2 |
| 8×2 | heldout | compact | 22.748 | +0.92% (1/4) | -0.20% (2/4) | 379.5 |
| 8×2 | heldout | runs | 22.903 | +0.74% (0/4) | +0.41% (2/4) | 372.5 |

## Observed compaction paths

The final column is the size of the original temporary field array. It is independent of process peak RSS.

| Shape | Input | Probe | Selected slots | Temporary copy KiB |
|---|---|---|---:|---:|
| 1×2 | catalog | compact | 10 | 0.3 |
| 1×2 | catalog | runs-limited | 10 | 0.3 |
| 1×2 | heldout | compact | 10 | 0.3 |
| 1×2 | heldout | runs-limited | 10 | 0.3 |
| 2×2 | catalog | compact | 17 | 0.5 |
| 2×2 | catalog | runs-limited | 17 | 0.5 |
| 2×2 | heldout | compact | 17 | 0.5 |
| 2×2 | heldout | runs-limited | 17 | 0.5 |
| 4×2 | catalog | compact | 31 | 1.0 |
| 4×2 | catalog | runs-limited | 31 | 1.0 |
| 4×2 | heldout | compact | 31 | 1.0 |
| 4×2 | heldout | runs-limited | 31 | 1.0 |
| 8×2 | catalog | compact | 59 | 1.8 |
| 8×2 | catalog | runs-limited | 59 | 1.8 |
| 8×2 | heldout | compact | 59 | 1.8 |
| 8×2 | heldout | runs-limited | 59 | 1.8 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

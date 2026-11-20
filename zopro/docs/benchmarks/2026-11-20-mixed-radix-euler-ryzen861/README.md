# Mixed Montgomery radix across the fused Euler FFT on Ryzen

The inverse butterflies read solver values directly at R256. The existing R260 root multiplications preserve that scale. Scaling the Euler weights by sixteen moves the transformed coefficients to R260 before the forward pass. The final product combines each R260 derivative with an R256 input, and the existing R260 reduction yields an R256 result. This removes radix conversion at initial input, at the two final input imports and at final output. The path uses the original scratch arrays and checks the existing fused FFT layout and weight range before dispatch.

The native differential compares both fused variants and the separately materialized derivative path with and without cached roots, and covers unequal and padded input lengths. Route probes confirm that the mixed path is selected only with fused output. Small-shape release uses its recorded FFT path. The two small trial arms both enable the same packed, fused quotient settings. The two larger shapes already use those settings in the release control.

Each shape uses 24 main and eight frontend Ryzen logical CPUs. Five matched blocks per input prime three requests and time thirteen. All 144 proof exports independently verified.

The mixed representation did not improve both controls consistently on any tested shape.

| Shape | Input | Profile | Proof ms | Paired vs same-build ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.851 | -1.25% (5/5) | +0.00% (0/5) | 57.8 |
| 1×2 | catalog | ordinary | 6.961 | +0.00% (0/5) | +1.27% (0/5) | 57.2 |
| 1×2 | catalog | mixed | 6.995 | +0.39% (2/5) | +2.10% (1/5) | 59.5 |
| 1×2 | heldout | release | 6.817 | +0.11% (2/5) | +0.00% (0/5) | 58.0 |
| 1×2 | heldout | ordinary | 6.830 | +0.00% (0/5) | -0.11% (3/5) | 58.0 |
| 1×2 | heldout | mixed | 6.799 | -0.28% (3/5) | -0.40% (4/5) | 57.5 |
| 2×2 | catalog | release | 9.437 | -0.07% (3/5) | +0.00% (0/5) | 95.2 |
| 2×2 | catalog | ordinary | 9.463 | +0.00% (0/5) | +0.07% (2/5) | 93.5 |
| 2×2 | catalog | mixed | 9.405 | -0.63% (5/5) | -0.59% (4/5) | 94.8 |
| 2×2 | heldout | release | 9.495 | +0.45% (2/5) | +0.00% (0/5) | 97.5 |
| 2×2 | heldout | ordinary | 9.467 | +0.00% (0/5) | -0.45% (3/5) | 93.8 |
| 2×2 | heldout | mixed | 9.408 | +0.08% (2/5) | -1.00% (4/5) | 97.5 |
| 4×2 | catalog | release | 13.629 | -0.59% (3/5) | +0.00% (0/5) | 210.0 |
| 4×2 | catalog | ordinary | 13.710 | +0.00% (0/5) | +0.60% (2/5) | 201.0 |
| 4×2 | catalog | mixed | 13.677 | -0.41% (4/5) | +0.22% (1/5) | 194.8 |
| 4×2 | heldout | release | 13.822 | -0.32% (3/5) | +0.00% (0/5) | 220.2 |
| 4×2 | heldout | ordinary | 13.754 | +0.00% (0/5) | +0.32% (2/5) | 209.5 |
| 4×2 | heldout | mixed | 13.861 | -0.16% (3/5) | +0.61% (1/5) | 212.2 |
| 8×2 | catalog | release | 22.339 | +1.03% (1/5) | +0.00% (0/5) | 370.0 |
| 8×2 | catalog | ordinary | 22.268 | +0.00% (0/5) | -1.01% (4/5) | 367.5 |
| 8×2 | catalog | mixed | 22.436 | +0.39% (0/5) | +0.47% (2/5) | 380.5 |
| 8×2 | heldout | release | 22.378 | +0.08% (2/5) | +0.00% (0/5) | 372.0 |
| 8×2 | heldout | ordinary | 22.448 | +0.00% (0/5) | -0.08% (3/5) | 376.2 |
| 8×2 | heldout | mixed | 22.607 | +1.17% (2/5) | +0.52% (1/5) | 372.8 |

| Shape | Input | Observed domain in mixed-radix route probe |
|---|---|---:|
| 1×2 | catalog | 32,768 |
| 1×2 | heldout | 32,768 |
| 2×2 | catalog | 65,536 |
| 2×2 | heldout | 65,536 |
| 4×2 | catalog | 131,072 |
| 4×2 | heldout | 131,072 |
| 8×2 | catalog | 262,144 |
| 8×2 | heldout | 262,144 |

[Source](artifacts/source.patch) · [Design](artifacts/design.md) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

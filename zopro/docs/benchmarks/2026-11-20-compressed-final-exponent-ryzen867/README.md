# Signed and compressed BN254 final exponentiation on Ryzen

The signed arm uses 24 signed nonzero digits of the BN254 parameter. The compressed arm obtains the same exponent from a four-coordinate cyclotomic squaring ladder and reconstructs 23 powers with a single batch inverse. Its arrays use 7,360 bytes of fixed stack payload. If any reconstruction denominator is zero, the complete-coordinate signed method handles that element. All modes use the existing Miller loop and complete final exponentiation. [Karabina's squaring formulas](https://eprint.iacr.org/2010/542)

The native field suite passed 40 tests. Independent Arkworks comparisons cover signed and compressed powers, zero and identity inputs, and the complete final exponent. All 222 SDK exports independently verified.

The run uses 24 main and eight frontend Ryzen logical CPUs. Seven matched process blocks per input each prime three proofs and time thirteen. A profile must have a lower paired median and at least five wins against each control on both inputs to advance. Separate clocked processes measure the whole final exponent after key setup and priming. Peak RSS is the highest process reading in each group.

Neither power algorithm improved both controls consistently across both inputs of any tested shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.837 | +0.32% (3/7) | +0.00% (0/7) | 57.8 |
| 1×2 | catalog | ordinary | 6.847 | +0.00% (0/7) | -0.32% (4/7) | 59.2 |
| 1×2 | catalog | signed | 6.892 | +0.58% (2/7) | +0.30% (2/7) | 58.0 |
| 1×2 | catalog | compressed | 6.864 | +0.90% (3/7) | +0.46% (2/7) | 57.2 |
| 1×2 | heldout | release | 6.815 | -0.57% (6/7) | +0.00% (0/7) | 58.0 |
| 1×2 | heldout | ordinary | 6.854 | +0.00% (0/7) | +0.57% (1/7) | 58.2 |
| 1×2 | heldout | signed | 6.809 | -0.83% (5/7) | +0.31% (3/7) | 58.0 |
| 1×2 | heldout | compressed | 6.749 | -1.59% (7/7) | -0.79% (7/7) | 57.8 |
| 2×2 | catalog | release | 9.542 | +0.14% (3/7) | +0.00% (0/7) | 94.0 |
| 2×2 | catalog | ordinary | 9.490 | +0.00% (0/7) | -0.14% (4/7) | 98.2 |
| 2×2 | catalog | signed | 9.504 | -0.41% (4/7) | -0.17% (4/7) | 97.5 |
| 2×2 | catalog | compressed | 9.524 | +0.17% (1/7) | -0.12% (4/7) | 98.8 |
| 2×2 | heldout | release | 9.514 | +0.13% (3/7) | +0.00% (0/7) | 94.5 |
| 2×2 | heldout | ordinary | 9.517 | +0.00% (0/7) | -0.13% (4/7) | 95.5 |
| 2×2 | heldout | signed | 9.469 | -0.22% (5/7) | -0.61% (6/7) | 99.2 |
| 2×2 | heldout | compressed | 9.505 | +0.14% (3/7) | -0.01% (4/7) | 98.5 |
| 4×2 | catalog | release | 13.800 | +0.89% (1/7) | +0.00% (0/7) | 200.8 |
| 4×2 | catalog | ordinary | 13.715 | +0.00% (0/7) | -0.88% (6/7) | 202.8 |
| 4×2 | catalog | signed | 13.734 | +0.63% (3/7) | -0.31% (6/7) | 199.2 |
| 4×2 | catalog | compressed | 13.861 | +0.61% (2/7) | +0.58% (3/7) | 199.0 |
| 4×2 | heldout | release | 13.666 | -0.10% (4/7) | +0.00% (0/7) | 201.0 |
| 4×2 | heldout | ordinary | 13.773 | +0.00% (0/7) | +0.10% (3/7) | 205.8 |
| 4×2 | heldout | signed | 13.796 | +0.48% (2/7) | +0.61% (1/7) | 214.8 |
| 4×2 | heldout | compressed | 13.698 | -0.55% (6/7) | -0.27% (4/7) | 200.5 |

| Shape | Input | Mode | Final exponent ms | Full-proof clock effect |
|---|---|---|---:|---:|
| 1×2 | catalog | ordinary | 0.213 | +0.31% (1/3) |
| 1×2 | catalog | signed | 0.176 | +1.58% (0/3) |
| 1×2 | catalog | compressed | 0.191 | -0.14% (2/3) |
| 1×2 | heldout | ordinary | 0.191 | +0.13% (1/3) |
| 1×2 | heldout | signed | 0.176 | +1.08% (0/3) |
| 1×2 | heldout | compressed | 0.190 | +0.12% (1/3) |
| 2×2 | catalog | ordinary | 0.206 | +0.63% (0/3) |
| 2×2 | catalog | signed | 0.192 | +0.32% (1/3) |
| 2×2 | catalog | compressed | 0.210 | -0.03% (2/3) |
| 2×2 | heldout | ordinary | 0.209 | +0.20% (0/3) |
| 2×2 | heldout | signed | 0.191 | +0.54% (1/3) |
| 2×2 | heldout | compressed | 0.209 | +0.09% (1/3) |
| 4×2 | catalog | ordinary | 0.216 | +0.89% (1/3) |
| 4×2 | catalog | signed | 0.196 | +0.36% (1/3) |
| 4×2 | catalog | compressed | 0.219 | +0.56% (1/3) |
| 4×2 | heldout | ordinary | 0.216 | -0.06% (2/3) |
| 4×2 | heldout | signed | 0.198 | -0.33% (2/3) |
| 4×2 | heldout | compressed | 0.220 | +0.51% (1/3) |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

# Reusing B2 scalar classes at an observed exact offset

Six public training captures have a dominant repeat distance of 20,653 among nonzero B2 coefficients after excluding the most common value. In the three 8×2 captures that distance accounts for 125,695 to 125,790 adjacent occurrences within each coefficient's occurrence list. The mode uses the offset only as a candidate. All four limbs must match an already classified value before it reuses a class ID. Every other value follows the existing hash table path. Point order, class order and counts match the unmodified classifier. The same buffers and workspace budget apply.

The full proof test uses each original, second and v156 transaction on Ryzen, separate from the training captures. It compares the existing SDK, the same candidate disabled, the measured offset and its neighboring offset. The 24 main and eight frontend hardware threads have disjoint logical CPU IDs. Five paired process blocks warm three requests and time thirteen per arm. The native grouping tests and a new exact class-order comparison passed. All 120 exports independently verified.

On 8×2 the measured offset took **22.364 / 22.940 / 22.742 ms** on the three inputs, with paired changes of **+0.01% / -0.30% / -0.31%** against the disabled candidate. These measurements describe Ryzen. The C7a reference table is in the [solver pacing report](../2026-11-19-solver-grain-c7a764/README.md).

| Shape | Input | Mode | Proof ms | Paired vs disabled | Paired vs SDK | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 4x2 | catalog | release | 14.504 | -0.61% (4/5) | +0.00% (0/5) | 199.8 | 7.51× / 3.31× |
| 4x2 | catalog | off | 14.594 | +0.00% (0/5) | +0.61% (1/5) | 206.2 | 7.47× / 3.29× |
| 4x2 | catalog | period | 14.451 | -0.98% (4/5) | -0.37% (4/5) | 197.2 | 7.54× / 3.32× |
| 4x2 | catalog | nearby-period | 14.584 | +0.10% (2/5) | +0.08% (1/5) | 206.5 | 7.47× / 3.29× |
| 4x2 | heldout | release | 14.455 | +0.16% (2/5) | +0.00% (0/5) | 212.2 | — |
| 4x2 | heldout | off | 14.461 | +0.00% (0/5) | -0.16% (3/5) | 209.0 | — |
| 4x2 | heldout | period | 14.416 | -0.77% (3/5) | -0.25% (4/5) | 196.8 | — |
| 4x2 | heldout | nearby-period | 14.494 | +0.38% (2/5) | +0.69% (2/5) | 232.8 | — |
| 4x2 | v156 | release | 14.509 | -0.55% (3/5) | +0.00% (0/5) | 206.0 | — |
| 4x2 | v156 | off | 14.590 | +0.00% (0/5) | +0.56% (2/5) | 197.5 | — |
| 4x2 | v156 | period | 14.427 | -0.58% (4/5) | -0.56% (3/5) | 196.2 | — |
| 4x2 | v156 | nearby-period | 14.509 | -0.88% (4/5) | -0.42% (3/5) | 215.5 | — |
| 8x2 | catalog | release | 22.530 | +1.02% (2/5) | +0.00% (0/5) | 404.2 | 8.39× / 3.28× |
| 8x2 | catalog | off | 22.302 | +0.00% (0/5) | -1.01% (3/5) | 391.0 | 8.47× / 3.32× |
| 8x2 | catalog | period | 22.364 | +0.01% (2/5) | -0.88% (3/5) | 398.8 | 8.45× / 3.31× |
| 8x2 | catalog | nearby-period | 22.456 | +0.39% (2/5) | -0.63% (3/5) | 406.2 | 8.42× / 3.30× |
| 8x2 | heldout | release | 22.656 | -0.14% (3/5) | +0.00% (0/5) | 410.8 | — |
| 8x2 | heldout | off | 22.763 | +0.00% (0/5) | +0.14% (2/5) | 400.2 | — |
| 8x2 | heldout | period | 22.940 | -0.30% (3/5) | +0.89% (1/5) | 417.0 | — |
| 8x2 | heldout | nearby-period | 22.934 | -0.58% (4/5) | +0.19% (2/5) | 406.5 | — |
| 8x2 | v156 | release | 22.905 | +0.57% (2/5) | +0.00% (0/5) | 404.5 | — |
| 8x2 | v156 | off | 22.774 | +0.00% (0/5) | -0.57% (3/5) | 394.8 | — |
| 8x2 | v156 | period | 22.742 | -0.31% (4/5) | -0.48% (3/5) | 391.5 | — |
| 8x2 | v156 | nearby-period | 22.730 | +0.24% (2/5) | -0.33% (3/5) | 412.2 | — |

[Training distances](artifacts/training-distances.json) · [Proof samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

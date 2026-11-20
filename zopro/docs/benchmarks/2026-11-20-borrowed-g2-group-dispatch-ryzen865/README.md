# Borrowing first-layer operands across grouped G2 routes on Ryzen

The [first trial](../2026-11-20-borrowed-g2-group-head-ryzen864/README.md) reached 2×2 and exposed a coverage gap on larger inputs. This follow-up includes the generic grouped G2 branch and reserves first-layer buffers according to each tile's actual point counts. It borrows the second operand from the key, emits completed classes directly and uses the existing limb reducer for unfinished classes. Tiles above 2,048 input points take the original path.

Five matched blocks per input use 24 main and eight frontend Ryzen logical CPUs. Each process primes three proofs and times thirteen. All 144 proof exports independently verified. Separate probes cover the borrowed path on all three shapes and the fallback with zero optional workspace.

The borrowed-first-layer variant did not improve both controls consistently across both inputs on any tested shape.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 2×2 | catalog | release | 9.497 | +0.01% (2/5) | +0.00% (0/5) | 94.8 |
| 2×2 | catalog | ordinary | 9.491 | +0.00% (0/5) | -0.01% (3/5) | 98.2 |
| 2×2 | catalog | borrow | 9.471 | -0.22% (4/5) | -0.53% (3/5) | 95.2 |
| 2×2 | heldout | release | 9.508 | +0.17% (1/5) | +0.00% (0/5) | 93.8 |
| 2×2 | heldout | ordinary | 9.494 | +0.00% (0/5) | -0.17% (4/5) | 96.2 |
| 2×2 | heldout | borrow | 9.465 | -0.18% (5/5) | -0.38% (5/5) | 89.2 |
| 4×2 | catalog | release | 13.765 | -0.03% (3/5) | +0.00% (0/5) | 203.2 |
| 4×2 | catalog | ordinary | 13.813 | +0.00% (0/5) | +0.03% (2/5) | 201.0 |
| 4×2 | catalog | borrow | 13.730 | -0.51% (3/5) | -0.35% (4/5) | 203.0 |
| 4×2 | heldout | release | 13.729 | +0.39% (2/5) | +0.00% (0/5) | 201.0 |
| 4×2 | heldout | ordinary | 13.710 | +0.00% (0/5) | -0.39% (3/5) | 194.2 |
| 4×2 | heldout | borrow | 13.681 | -0.22% (3/5) | -0.04% (3/5) | 201.5 |
| 8×2 | catalog | release | 22.298 | -1.10% (3/5) | +0.00% (0/5) | 366.8 |
| 8×2 | catalog | ordinary | 22.382 | +0.00% (0/5) | +1.11% (2/5) | 375.8 |
| 8×2 | catalog | borrow | 22.410 | -0.60% (3/5) | +0.65% (1/5) | 376.8 |
| 8×2 | heldout | release | 22.583 | +0.80% (0/5) | +0.00% (0/5) | 377.2 |
| 8×2 | heldout | ordinary | 22.326 | +0.00% (0/5) | -0.80% (5/5) | 359.5 |
| 8×2 | heldout | borrow | 22.596 | +1.45% (0/5) | +0.04% (2/5) | 374.0 |

## Group point reduction timing

Three matched blocks separately time B2 scalar-class point reduction in the same executable. Clocked proofs have unclocked companions. The effect columns describe the full proof after adding the clock.

| Shape | Input | Ordinary ms | Borrowed first layer ms | Paired reduction change | Full-proof clock effect ordinary / borrowed |
|---|---|---:|---:|---:|---:|
| 2×2 | catalog | 0.850 | 0.746 | -14.54% (3/3) | +0.18% / -0.12% |
| 2×2 | heldout | 0.841 | 0.733 | -11.59% (3/3) | +0.10% / -0.01% |
| 4×2 | catalog | 1.981 | 2.138 | +7.90% (1/3) | -0.22% / -0.05% |
| 4×2 | heldout | 1.683 | 1.931 | -1.97% (2/3) | -0.42% / +0.83% |
| 8×2 | catalog | 7.149 | 7.141 | +0.19% (1/3) | -1.33% / +0.99% |
| 8×2 | heldout | 6.992 | 6.979 | -1.46% (2/3) | +0.47% / -0.03% |

## Group geometry

| Shape | Input | Terms | Classes | One point | Two points | Largest class | Terms in oversized tiles |
|---|---|---:|---:|---:|---:|---:|---:|
| 2×2 | catalog | 52,274 | 27,218 | 10,013 | 17,168 | 779 | 0 |
| 2×2 | heldout | 52,274 | 27,267 | 10,035 | 17,214 | 913 | 0 |
| 4×2 | catalog | 95,537 | 29,738 | 11,325 | 273 | 1,576 | 2,590 |
| 4×2 | heldout | 95,537 | 29,961 | 12,478 | 259 | 1,708 | 2,728 |
| 8×2 | catalog | 183,347 | 38,075 | 18,588 | 757 | 4,210 | 66,971 |
| 8×2 | heldout | 183,347 | 39,891 | 20,794 | 751 | 4,451 | 30,089 |

[Source](artifacts/source.patch) · [Design](artifacts/design.md) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

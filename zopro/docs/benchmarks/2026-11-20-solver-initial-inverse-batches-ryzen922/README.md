# Batching initially ready inversion hints in the CPU solver

Parallel batching cut the 1×2 initial inversion step from **40–41 µs to 3–4 µs** relative to doing the same work early with individual inversions. The parallel candidate's full-proof change against ordinary was **−0.43% / +0.43%** on the original and held-out inputs. Neither batch mode qualified on any of the three shapes.

The prepared plan selects hints whose entire input expression uses original, already initialized witness inputs. Definite assignment confirms independent outputs. The solver evaluates those hints before launching its usual bands. The scalar control uses the original inverse for every selected hint. Serial and parallel batch modes use zero-aware groups of 32 and fixed scratch per task. The runtime hint marker serializes in the original format. The prepared index list uses the existing optional workspace policy.

| Shape | Input | Initially ready hints | Nonzero values | Groups of up to 32 |
|---|---|---:|---:|---:|
| 1x2 | catalog | 105 | 62 | 4 |
| 1x2 | heldout | 105 | 63 | 4 |
| 2x2 | catalog | 152 | 92 | 5 |
| 2x2 | heldout | 152 | 91 | 5 |
| 4x2 | catalog | 258 | 158 | 9 |
| 4x2 | heldout | 258 | 153 | 9 |

The Ryzen run uses 24 main and eight frontend hardware threads. Eight counterbalanced blocks per input prime three proofs and time seventeen. The fresh ordinary controls all calibrated within 2% of the frozen release. Each batch mode needed lower paired medians and at least six wins against both controls on both inputs. Whole-process peak RSS includes setup.

| Shape | Input | Mode | Proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.927 | +0.00% (0/8) | +0.44% (3/8) | 58.2 |
| 1x2 | catalog | ordinary | 6.907 | -0.44% (5/8) | +0.00% (0/8) | 57.8 |
| 1x2 | catalog | batch | 6.931 | -0.58% (6/8) | +0.11% (4/8) | 57.8 |
| 1x2 | catalog | parallel | 6.896 | -0.59% (5/8) | -0.43% (5/8) | 57.8 |
| 1x2 | heldout | release | 6.810 | +0.00% (0/8) | +0.23% (4/8) | 58.8 |
| 1x2 | heldout | ordinary | 6.767 | -0.23% (4/8) | +0.00% (0/8) | 58.5 |
| 1x2 | heldout | batch | 6.804 | -0.44% (5/8) | +0.64% (2/8) | 57.5 |
| 1x2 | heldout | parallel | 6.812 | -0.24% (5/8) | +0.43% (2/8) | 58.5 |
| 2x2 | catalog | release | 9.523 | +0.00% (0/8) | +0.28% (3/8) | 97.8 |
| 2x2 | catalog | ordinary | 9.482 | -0.28% (5/8) | +0.00% (0/8) | 95.8 |
| 2x2 | catalog | batch | 9.536 | -0.16% (5/8) | +0.29% (2/8) | 100.2 |
| 2x2 | catalog | parallel | 9.459 | -0.36% (5/8) | +0.01% (4/8) | 94.8 |
| 2x2 | heldout | release | 9.533 | +0.00% (0/8) | +0.53% (2/8) | 94.5 |
| 2x2 | heldout | ordinary | 9.483 | -0.53% (6/8) | +0.00% (0/8) | 94.2 |
| 2x2 | heldout | batch | 9.561 | +0.05% (4/8) | +0.88% (0/8) | 94.0 |
| 2x2 | heldout | parallel | 9.499 | -0.56% (6/8) | -0.01% (4/8) | 98.0 |
| 4x2 | catalog | release | 13.576 | +0.00% (0/8) | -0.73% (7/8) | 210.0 |
| 4x2 | catalog | ordinary | 13.683 | +0.73% (1/8) | +0.00% (0/8) | 219.8 |
| 4x2 | catalog | batch | 13.672 | +0.48% (2/8) | -0.10% (6/8) | 214.0 |
| 4x2 | catalog | parallel | 13.576 | +0.10% (3/8) | -0.48% (6/8) | 214.8 |
| 4x2 | heldout | release | 13.674 | +0.00% (0/8) | +0.27% (3/8) | 206.8 |
| 4x2 | heldout | ordinary | 13.610 | -0.27% (5/8) | +0.00% (0/8) | 233.8 |
| 4x2 | heldout | batch | 13.642 | -0.01% (4/8) | -0.41% (5/8) | 211.0 |
| 4x2 | heldout | parallel | 13.664 | +0.39% (3/8) | -0.13% (4/8) | 213.2 |

The clocked panels use three separate blocks with seven timed proofs. The early time measures the same inputs in all three early modes. Its diagnostic log is emitted before recording the end of the whole solver, so the total solver clock includes that extra emission. The full-proof comparisons above use tracing disabled.

| Shape | Input | Mode | Early hints µs | Total solver ms |
|---|---|---|---:|---:|
| 1x2 | catalog | ordinary | — | 0.698 |
| 1x2 | catalog | scalar | 40.1 | 0.747 |
| 1x2 | catalog | batch | 8.2 | 0.714 |
| 1x2 | catalog | parallel | 3.5 | 0.712 |
| 1x2 | heldout | ordinary | — | 0.695 |
| 1x2 | heldout | scalar | 41.2 | 0.762 |
| 1x2 | heldout | batch | 8.9 | 0.717 |
| 1x2 | heldout | parallel | 3.4 | 0.713 |
| 2x2 | catalog | ordinary | — | 0.902 |
| 2x2 | catalog | scalar | 57.7 | 0.973 |
| 2x2 | catalog | batch | 13.4 | 0.929 |
| 2x2 | catalog | parallel | 4.9 | 0.928 |
| 2x2 | heldout | ordinary | — | 0.902 |
| 2x2 | heldout | scalar | 59.0 | 0.993 |
| 2x2 | heldout | batch | 13.2 | 0.951 |
| 2x2 | heldout | parallel | 5.1 | 0.946 |
| 4x2 | catalog | ordinary | — | 1.508 |
| 4x2 | catalog | scalar | 103.5 | 1.567 |
| 4x2 | catalog | batch | 23.5 | 1.506 |
| 4x2 | catalog | parallel | 10.0 | 1.470 |
| 4x2 | heldout | ordinary | — | 1.495 |
| 4x2 | heldout | scalar | 100.1 | 1.583 |
| 4x2 | heldout | batch | 24.2 | 1.544 |
| 4x2 | heldout | parallel | 10.3 | 1.512 |

The scalar arithmetic test covers all lengths through 32, all-zero and all-one chunks, and mixed field values. All 31 enabled native solver tests passed, and all 288 exported proofs independently verified. Route probes confirmed the expected initial hint counts and valid fallback when optional workspace was unavailable. An initial compile caught a raw-pointer capture and a missing test fixture field. Both were corrected before any capture.

The C7a scorecard remains five of eight at 10× M5 CPU, with all eight Metal comparisons. The pending 2×2 C7a confirmation still awaits AWS access. [CPU and Metal scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Samples and route probes](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)

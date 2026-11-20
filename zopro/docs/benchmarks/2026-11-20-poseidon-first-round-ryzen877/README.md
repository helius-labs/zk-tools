# Direct execution of the first Poseidon round on Ryzen

The solver already recognizes spans where the first round computes three consecutive powers per input, with a stored affine expression for each input. This variant evaluates those original sum chains in their original order, computes the three powers directly, and writes all original wires and A/B/C rows. The state enters the existing sparse Poseidon kernel directly. The two temporary field arrays use 832 bytes of stack payload across all supported widths.

The native solver suite runs with both modes. Its independent symbolic fixtures cover widths two through thirteen, both operand orientations, optional C rows, encoded emissions and interpreter fallback. Whole field outputs match the interpreter. All 138 SDK proof exports independently verified.

The worker pools use 24 main and eight frontend Ryzen logical CPUs. The 4×2 arms use seeded solver tasks and encoded Poseidon emissions. All three arms for each shape use the same other runtime options. Seven paired blocks per input prime three proofs and time thirteen. Separate probes count recognized first rounds and the constraint rows they write. Advancement requires lower paired medians and five wins against each control on both inputs.

The direct first round did not consistently improve both controls across both inputs of any tested shape.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.888 | -2.13% (7/7) | +0.00% (0/7) | 133.9 | 129 | 57.5 |
| 1×2 | catalog | ordinary | 7.037 | +0.00% (0/7) | +2.17% (0/7) | 134.6 | 114 | 57.2 |
| 1×2 | catalog | prefix | 7.036 | +0.08% (3/7) | +1.53% (0/7) | 134.5 | 121 | 57.2 |
| 1×2 | heldout | release | 6.763 | -2.19% (7/7) | +0.00% (0/7) | 129.5 | 118 | 58.2 |
| 1×2 | heldout | ordinary | 6.922 | +0.00% (0/7) | +2.24% (0/7) | 129.8 | 102 | 57.5 |
| 1×2 | heldout | prefix | 6.904 | +0.08% (3/7) | +2.39% (0/7) | 130.5 | 125 | 58.2 |
| 2×2 | catalog | release | 9.483 | -1.13% (7/7) | +0.00% (0/7) | 181.0 | 241 | 95.8 |
| 2×2 | catalog | ordinary | 9.635 | +0.00% (0/7) | +1.14% (0/7) | 182.3 | 306 | 94.0 |
| 2×2 | catalog | prefix | 9.654 | -0.02% (4/7) | +1.51% (1/7) | 182.3 | 230 | 99.2 |
| 2×2 | heldout | release | 9.541 | -1.54% (6/7) | +0.00% (0/7) | 182.2 | 311 | 99.8 |
| 2×2 | heldout | ordinary | 9.664 | +0.00% (0/7) | +1.57% (1/7) | 182.4 | 301 | 99.0 |
| 2×2 | heldout | prefix | 9.674 | +0.13% (3/7) | +1.32% (1/7) | 183.2 | 280 | 99.0 |
| 4×2 | catalog | release | 13.732 | -1.85% (7/7) | +0.00% (0/7) | 293.2 | 1055 | 219.5 |
| 4×2 | catalog | ordinary | 13.963 | +0.00% (0/7) | +1.88% (0/7) | 294.6 | 929 | 204.8 |
| 4×2 | catalog | prefix | 13.953 | -0.27% (4/7) | +1.60% (0/7) | 294.3 | 840 | 216.5 |
| 4×2 | heldout | release | 13.748 | -1.34% (6/7) | +0.00% (0/7) | 292.3 | 1012 | 205.2 |
| 4×2 | heldout | ordinary | 13.969 | +0.00% (0/7) | +1.36% (1/7) | 293.5 | 914 | 236.0 |
| 4×2 | heldout | prefix | 13.859 | -1.40% (5/7) | +0.98% (2/7) | 293.2 | 806 | 201.0 |

| Shape | Input | Direct first rounds per proof | Constraint rows written per proof |
|---|---|---:|---:|
| 1×2 | catalog | 83.0 | 801.0 |
| 1×2 | heldout | 83.0 | 801.0 |
| 2×2 | catalog | 150.0 | 1413.0 |
| 2×2 | heldout | 150.0 | 1413.0 |
| 4×2 | catalog | 264.0 | 2427.0 |
| 4×2 | heldout | 264.0 | 2427.0 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

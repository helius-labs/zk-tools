# Staging borrowed B2 window buckets during the solver on Ryzen

The solver's [wire readiness](../2026-11-20-curve-input-readiness881/README.md) allows a contiguous suffix to borrow key points directly. Before tagging the solver schedule, the trial checks each B2 wire's unique producer and chooses the first band with a complete suffix of adequate size. The early arm gathers those scalars and forms the suffix's affine buckets during the following band. The late arm runs that work after solving. Each arm forms the remaining prefix buckets in the ordinary curve phase, merges both ranges, and weights each window once. The two ranges use separate scalar grouping when enabled. Both use the existing signed Phi decomposition and 12-bit windows. The release and ordinary arms use the unsplit query.

The retained range data is proof-local and covered by optional workspace admission. The early canonical gather applies each public-wire offset to its copied suffix scalar. The ordinary post-solver gather handles the prefix and the existing curve correction covers both ranges. Input bases stay borrowed in the fill. The existing scalar grouping can form a temporary set of point sums for either range.

Ryzen uses 24 main and eight frontend logical CPUs. Both shapes run their recorded fused K/H configurations with a four-band packed solver order. Eight matched blocks on each primary input and four on each additional input prime three proofs and time thirteen. Advancement requires lower paired medians and six or three paired wins against release, ordinary and late on each input.

Early staging did not consistently improve all three controls on either shape.

Native tests check the two-range result against a generator sum with signed, zero and repeated scalars and identity bases. Solver and borrowed-window tests pass. All 176 final-panel proofs independently verified, including the ordinary path at zero optional workspace. A preliminary panel produced 176 verified proofs but stayed on the ordinary route because both keys used public-wire offsets. Those timings are excluded from the table.

| Shape | Completed solver bands at readiness | Borrowed suffix points |
|---|---:|---:|
| 1×2 | 2 | 8,238 |
| 2×2 | 2 | 9,416 |

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.900 | -0.23% (5/8) | +0.00% (0/8) | -21.88% (8/8) | 134.1 | 118 | 58.2 |
| 1×2 | catalog | ordinary | 6.893 | +0.00% (0/8) | +0.23% (3/8) | -21.78% (8/8) | 134.0 | 117 | 58.2 |
| 1×2 | catalog | late | 8.874 | +27.85% (0/8) | +28.00% (0/8) | +0.00% (0/8) | 132.1 | 143 | 58.5 |
| 1×2 | catalog | early | 8.647 | +25.07% (0/8) | +24.51% (0/8) | -2.10% (6/8) | 132.0 | 133 | 59.0 |
| 1×2 | heldout | release | 6.786 | +0.22% (4/8) | +0.00% (0/8) | -21.04% (8/8) | 129.4 | 126 | 57.5 |
| 1×2 | heldout | ordinary | 6.816 | +0.00% (0/8) | -0.22% (4/8) | -21.09% (8/8) | 129.6 | 127 | 58.5 |
| 1×2 | heldout | late | 8.623 | +26.73% (0/8) | +26.65% (0/8) | +0.00% (0/8) | 129.0 | 142 | 58.2 |
| 1×2 | heldout | early | 8.456 | +26.40% (0/8) | +25.48% (0/8) | -1.31% (5/8) | 128.1 | 138 | 59.8 |
| 1×2 | v27 | release | 6.811 | +0.90% (0/4) | +0.00% (0/4) | -20.94% (4/4) | 129.2 | 133 | 57.8 |
| 1×2 | v27 | ordinary | 6.768 | +0.00% (0/4) | -0.89% (4/4) | -21.31% (4/4) | 130.1 | 117 | 57.5 |
| 1×2 | v27 | late | 8.616 | +27.08% (0/4) | +26.49% (0/4) | +0.00% (0/4) | 128.3 | 153 | 58.5 |
| 1×2 | v27 | early | 8.566 | +26.90% (0/4) | +25.56% (0/4) | -0.36% (2/4) | 127.6 | 167 | 58.8 |
| 1×2 | v28 | release | 6.755 | +0.05% (2/4) | +0.00% (0/4) | -21.78% (4/4) | 129.2 | 138 | 58.0 |
| 1×2 | v28 | ordinary | 6.753 | +0.00% (0/4) | -0.05% (2/4) | -21.88% (4/4) | 128.2 | 120 | 57.5 |
| 1×2 | v28 | late | 8.648 | +28.04% (0/4) | +27.88% (0/4) | +0.00% (0/4) | 126.9 | 162 | 59.0 |
| 1×2 | v28 | early | 8.429 | +24.83% (0/4) | +24.28% (0/4) | -0.19% (3/4) | 128.1 | 144 | 58.8 |
| 2×2 | catalog | release | 9.482 | +0.44% (2/8) | +0.00% (0/8) | -20.95% (8/8) | 181.4 | 280 | 99.0 |
| 2×2 | catalog | ordinary | 9.448 | +0.00% (0/8) | -0.44% (6/8) | -20.96% (8/8) | 181.7 | 303 | 94.5 |
| 2×2 | catalog | late | 11.973 | +26.51% (0/8) | +26.50% (0/8) | +0.00% (0/8) | 180.5 | 199 | 90.5 |
| 2×2 | catalog | early | 11.840 | +25.48% (0/8) | +25.08% (0/8) | -0.82% (6/8) | 180.3 | 263 | 91.8 |
| 2×2 | heldout | release | 9.522 | +0.97% (3/8) | +0.00% (0/8) | -20.29% (8/8) | 182.1 | 240 | 98.8 |
| 2×2 | heldout | ordinary | 9.440 | +0.00% (0/8) | -0.96% (5/8) | -21.02% (8/8) | 182.0 | 204 | 94.8 |
| 2×2 | heldout | late | 11.987 | +26.62% (0/8) | +25.46% (0/8) | +0.00% (0/8) | 180.8 | 255 | 94.0 |
| 2×2 | heldout | early | 11.836 | +24.96% (0/8) | +24.22% (0/8) | -1.26% (8/8) | 180.4 | 187 | 92.8 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

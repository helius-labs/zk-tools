# Preparing the fresh B pairing term during curve queries on Ryzen

The fused solver can start rerandomizing B2 and preparing its Miller lines as soon as the G2 query finishes. The late arm does the same work after the curve joins. Both arms sample fresh blinding scalars before solving and keep the lines only for that proof. The SDK binds the lines to the parsed B point and checks the complete pairing equation. Proof parsing retains its existing validity policy. The lines use the same allocation as ordinary verification, with an earlier lifetime in the early arm.

Ryzen uses 24 main and eight frontend logical CPUs. Both shapes use fused K/H and the existing public-input verification overlap. Eight matched blocks per input prime three proofs and time thirteen. Advancement requires lower paired medians and six wins against each control on both inputs.

Early preparation failed to beat all three controls consistently on either shape.

Native checks cover the rerandomization equations, zero delta, the exceptional A branch, and verification of the intended public input. Instrumented runs confirm one early or late preparation and one consumption per call on 1×2 and 2×2. Additional 1×2 and 2×2 probes run the full G2 subgroup check. The 4×2 probes exercise ordinary verification through the larger-query fallback. All 150 exported proofs independently verified.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.934 | -0.01% (4/8) | +0.00% (0/8) | -1.11% (6/8) | 133.9 | 109 | 57.5 |
| 1×2 | catalog | ordinary | 6.891 | +0.00% (0/8) | +0.01% (4/8) | -1.01% (6/8) | 133.5 | 130 | 57.8 |
| 1×2 | catalog | late | 6.981 | +1.02% (2/8) | +1.12% (2/8) | +0.00% (0/8) | 133.6 | 114 | 57.8 |
| 1×2 | catalog | early | 6.988 | +1.13% (1/8) | +1.24% (2/8) | +0.18% (3/8) | 133.5 | 122 | 61.0 |
| 1×2 | heldout | release | 6.771 | -0.04% (4/8) | +0.00% (0/8) | -1.12% (8/8) | 129.3 | 133 | 58.2 |
| 1×2 | heldout | ordinary | 6.774 | +0.00% (0/8) | +0.04% (4/8) | -1.23% (6/8) | 129.0 | 136 | 57.8 |
| 1×2 | heldout | late | 6.841 | +1.24% (2/8) | +1.13% (0/8) | +0.00% (0/8) | 128.7 | 123 | 57.8 |
| 1×2 | heldout | early | 6.826 | +0.80% (2/8) | +1.64% (0/8) | -0.17% (5/8) | 128.9 | 121 | 58.0 |
| 2×2 | catalog | release | 9.462 | -0.21% (5/8) | +0.00% (0/8) | -1.25% (8/8) | 181.2 | 242 | 94.8 |
| 2×2 | catalog | ordinary | 9.515 | +0.00% (0/8) | +0.21% (3/8) | -0.78% (7/8) | 181.4 | 240 | 98.8 |
| 2×2 | catalog | late | 9.567 | +0.78% (1/8) | +1.26% (0/8) | +0.00% (0/8) | 181.0 | 269 | 96.2 |
| 2×2 | catalog | early | 9.574 | +0.75% (1/8) | +1.09% (0/8) | +0.01% (4/8) | 181.3 | 265 | 92.8 |
| 2×2 | heldout | release | 9.496 | -0.23% (5/8) | +0.00% (0/8) | -0.75% (6/8) | 182.0 | 211 | 94.0 |
| 2×2 | heldout | ordinary | 9.557 | +0.00% (0/8) | +0.23% (3/8) | -0.32% (5/8) | 182.1 | 247 | 94.0 |
| 2×2 | heldout | late | 9.570 | +0.32% (3/8) | +0.75% (2/8) | +0.00% (0/8) | 181.7 | 333 | 93.8 |
| 2×2 | heldout | early | 9.547 | +0.20% (4/8) | +0.69% (2/8) | -0.06% (4/8) | 181.5 | 237 | 99.0 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

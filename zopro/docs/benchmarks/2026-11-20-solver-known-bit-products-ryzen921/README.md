# Selecting products with known bit factors in the CPU solver

The 1×2 solver routed **2,656 products per proof** through the bit shortcut. Its measured solver span fell by **9–10 µs**. Full-proof latency changed **−0.05% / +0.05%** in paired medians against same-build ordinary. The fresh ordinary build ran **2.10% / 2.44%** slower than the frozen release, missing calibration on both 1×2 inputs.

The solver marks directly used bit-hint wires and bit products during plan construction. A marked factor equal to zero or one selects the exact product. An unexpected field value uses ordinary multiplication. The marker occupies an existing expression tag and serializes as the original wire. The solver format, expression size and scheduling dependencies are preserved. Temporary marking storage is admitted by the existing memory policy. Probes verified fallback when optional workspace was unavailable.

| Shape | Input | Tagged products | Observed shortcut calls | Ordinary solver ms | Candidate solver ms |
|---|---|---:|---:|---:|---:|
| 1x2 | catalog | 2,656 | 2,656 | 0.692 | 0.682 |
| 1x2 | heldout | 2,656 | 2,656 | 0.696 | 0.687 |
| 2x2 | catalog | 4,972 | 4,972 | 0.917 | 0.889 |
| 2x2 | heldout | 4,972 | 4,972 | 0.911 | 0.886 |
| 4x2 | catalog | 9,604 | 9,604 | 1.447 | 1.413 |
| 4x2 | heldout | 9,604 | 9,604 | 1.498 | 1.454 |

Proofs ran on Ryzen with 24 main and eight frontend hardware threads. Eight counterbalanced blocks per input prime three proofs and time seventeen. Three separate clocked blocks measure the prepared solver. The candidate must have lower paired medians and six wins out of eight against each control on both inputs. The fresh ordinary build must also calibrate within 2% of the release. That check failed on both 1×2 inputs and the original 4×2 input. The 2×2 control calibrated, and the candidate lost every paired comparison to the release. Peak RSS includes setup.

| Shape | Input | Mode | Proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.930 | +0.00% (0/8) | -2.05% (8/8) | 57.8 |
| 1x2 | catalog | ordinary | 7.058 | +2.10% (0/8) | +0.00% (0/8) | 58.2 |
| 1x2 | catalog | bit | 7.070 | +2.60% (0/8) | -0.05% (4/8) | 58.0 |
| 1x2 | heldout | release | 6.765 | +0.00% (0/8) | -2.38% (8/8) | 57.5 |
| 1x2 | heldout | ordinary | 6.945 | +2.44% (0/8) | +0.00% (0/8) | 57.8 |
| 1x2 | heldout | bit | 6.968 | +2.71% (0/8) | +0.05% (4/8) | 57.5 |
| 2x2 | catalog | release | 9.493 | +0.00% (0/8) | -1.83% (8/8) | 97.8 |
| 2x2 | catalog | ordinary | 9.663 | +1.87% (0/8) | +0.00% (0/8) | 96.0 |
| 2x2 | catalog | bit | 9.630 | +1.28% (0/8) | -0.72% (8/8) | 94.2 |
| 2x2 | heldout | release | 9.519 | +0.00% (0/8) | -1.31% (7/8) | 97.8 |
| 2x2 | heldout | ordinary | 9.676 | +1.33% (1/8) | +0.00% (0/8) | 94.5 |
| 2x2 | heldout | bit | 9.672 | +1.38% (0/8) | -0.26% (5/8) | 98.8 |
| 4x2 | catalog | release | 13.577 | +0.00% (0/8) | -1.99% (8/8) | 217.2 |
| 4x2 | catalog | ordinary | 13.875 | +2.04% (0/8) | +0.00% (0/8) | 208.8 |
| 4x2 | catalog | bit | 13.854 | +2.15% (0/8) | -0.01% (4/8) | 209.2 |
| 4x2 | heldout | release | 13.587 | +0.00% (0/8) | -1.65% (8/8) | 221.2 |
| 4x2 | heldout | ordinary | 13.846 | +1.68% (0/8) | +0.00% (0/8) | 218.8 |
| 4x2 | heldout | bit | 13.764 | +1.10% (0/8) | -0.26% (5/8) | 213.8 |

The targeted field test checks both operands, nonbit fallback and the all-bit case. The 31 enabled native solver tests passed. All 192 exported proofs independently verified. The candidate stayed outside the default prover.

The recorded C7a scorecard remains five of eight at 10× M5 CPU and includes all eight Metal comparisons. The pending 2×2 C7a test still awaits AWS access. [CPU and Metal scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Samples and route probes](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)

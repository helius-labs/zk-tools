# Eight-lane Montgomery decoding for proof scalars

The trial packs eight native field elements at a time in the mixed-radix AVX-512 IFMA representation. Multiplication by the raw integer one decodes each lane. The final partial batch uses the [standalone four-word reduction](../2026-11-19-canonical-reduction-ryzen809/README.md). The ordinary path preserves the original field backend. The trial uses existing proof buffers for gather and in-place MSM scalar conversion. It retains no key data or lookup table.

The generator-based field unit matched the existing backend on fifteen length and boundary patterns. A separate one-core check converts a fixed array of 65,536 scalars per repetition. Twelve matched blocks contain sixteen conversions per method. Input copies and output checks are outside the timer. All outputs match the original decoder.

| Decoder | ns per scalar | Paired change from generic |
|---|---:|---:|
| generic | 9.051 | +0.00% |
| reduced | 7.911 | -12.55% |
| vector | 1.989 | -78.05% |

The full Ryzen proofs use 24 main and eight frontend logical CPUs. Five matched blocks per input compare release, same-build ordinary, scalar reduction and vector reduction. Each process warms three requests and measures thirteen. All 176 exports passed independent verification, including sixteen normal and zero-budget dispatch captures.

None of the conversion modes beat both controls consistently on either shape. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.903 | -1.72% (3/5) | +0.00% (0/5) | 57.8 |
| 1×2 | catalog | ordinary | 6.984 | +0.00% (0/5) | +1.75% (2/5) | 58.0 |
| 1×2 | catalog | reduced | 7.055 | +1.23% (1/5) | +1.46% (0/5) | 57.8 |
| 1×2 | catalog | vector | 7.011 | -0.64% (3/5) | +0.57% (1/5) | 57.2 |
| 1×2 | heldout | release | 6.824 | -1.14% (5/5) | +0.00% (0/5) | 57.0 |
| 1×2 | heldout | ordinary | 6.880 | +0.00% (0/5) | +1.16% (0/5) | 58.5 |
| 1×2 | heldout | reduced | 6.882 | -0.47% (3/5) | +0.85% (0/5) | 58.0 |
| 1×2 | heldout | vector | 6.814 | -0.74% (4/5) | -0.06% (3/5) | 57.2 |
| 2×2 | catalog | release | 9.482 | -0.10% (3/5) | +0.00% (0/5) | 97.5 |
| 2×2 | catalog | ordinary | 9.480 | +0.00% (0/5) | +0.10% (2/5) | 93.2 |
| 2×2 | catalog | reduced | 9.512 | +0.33% (2/5) | +0.44% (2/5) | 90.5 |
| 2×2 | catalog | vector | 9.464 | -0.06% (3/5) | +0.08% (2/5) | 93.5 |
| 2×2 | heldout | release | 9.552 | -0.74% (3/5) | +0.00% (0/5) | 98.2 |
| 2×2 | heldout | ordinary | 9.513 | +0.00% (0/5) | +0.75% (2/5) | 93.2 |
| 2×2 | heldout | reduced | 9.532 | +0.00% (2/5) | +0.49% (2/5) | 92.8 |
| 2×2 | heldout | vector | 9.468 | -0.95% (5/5) | -0.49% (4/5) | 98.0 |
| 4×2 | catalog | release | 14.491 | +0.09% (2/5) | +0.00% (0/5) | 204.2 |
| 4×2 | catalog | ordinary | 14.477 | +0.00% (0/5) | -0.09% (3/5) | 201.2 |
| 4×2 | catalog | reduced | 14.570 | +0.55% (2/5) | +0.47% (1/5) | 207.0 |
| 4×2 | catalog | vector | 14.598 | -0.67% (3/5) | -0.67% (3/5) | 196.0 |
| 4×2 | heldout | release | 14.484 | -0.23% (3/5) | +0.00% (0/5) | 237.8 |
| 4×2 | heldout | ordinary | 14.519 | +0.00% (0/5) | +0.23% (2/5) | 222.8 |
| 4×2 | heldout | reduced | 14.546 | +0.20% (2/5) | -0.12% (3/5) | 202.5 |
| 4×2 | heldout | vector | 14.621 | -0.48% (3/5) | +0.79% (2/5) | 228.0 |
| 8×2 | catalog | release | 22.565 | +0.03% (2/5) | +0.00% (0/5) | 410.8 |
| 8×2 | catalog | ordinary | 22.662 | +0.00% (0/5) | -0.03% (3/5) | 401.8 |
| 8×2 | catalog | reduced | 23.010 | +1.33% (1/5) | +0.67% (2/5) | 409.8 |
| 8×2 | catalog | vector | 22.710 | +0.01% (2/5) | -0.14% (4/5) | 415.2 |
| 8×2 | heldout | release | 22.923 | -0.65% (3/5) | +0.00% (0/5) | 408.2 |
| 8×2 | heldout | ordinary | 22.967 | +0.00% (0/5) | +0.65% (2/5) | 401.5 |
| 8×2 | heldout | reduced | 22.761 | -0.63% (3/5) | +0.12% (2/5) | 400.0 |
| 8×2 | heldout | vector | 22.925 | -1.24% (3/5) | -0.12% (3/5) | 408.0 |

[Source](artifacts/source.patch) · [Standalone samples](artifacts/micro.json) · [Full-proof samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

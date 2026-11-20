# Reusing completed Poseidon spans from the existing solution on Ryzen

The proof-local cache stores only a full width-three state and the position of a completed span in the existing solver schedule. After checking the complete state, a match reads the powers already written into the source span's immutable solution cells, then writes every destination wire and row in its original orientation. The single-writer certificate and cache publication order make those reads safe. A lock conflict or occupied slot with a different state leaves the ordinary arithmetic path available.

The cache has 256 entries, with no duplicate trace storage or retained witness state. It uses the existing optional workspace reservation. The native test checks disjoint source and destination addresses, opposite operand orientations and optional C rows. The route probes also verify the uncached path at zero optional workspace.

The 4×2 and 8×2 proofs use the recorded four-epoch seeded solver and encoded Poseidon stores on 24 main and eight frontend Ryzen logical CPUs. Five matched blocks per input each prime three requests and time thirteen. All 72 proof exports independently verified.

Reuse did not beat both controls consistently on either shape. The candidate remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 13.718 | +0.60% (1/5) | +0.00% (0/5) | 201.0 |
| 4×2 | catalog | ordinary | 13.614 | +0.00% (0/5) | -0.59% (4/5) | 205.0 |
| 4×2 | catalog | reuse | 13.755 | -0.10% (3/5) | -0.86% (3/5) | 198.2 |
| 4×2 | heldout | release | 13.672 | -0.48% (4/5) | +0.00% (0/5) | 200.0 |
| 4×2 | heldout | ordinary | 13.726 | +0.00% (0/5) | +0.49% (1/5) | 205.5 |
| 4×2 | heldout | reuse | 13.683 | -0.03% (3/5) | +0.72% (2/5) | 205.5 |
| 8×2 | catalog | release | 22.287 | -0.93% (4/5) | +0.00% (0/5) | 369.0 |
| 8×2 | catalog | ordinary | 22.566 | +0.00% (0/5) | +0.94% (1/5) | 377.2 |
| 8×2 | catalog | reuse | 22.304 | +0.03% (2/5) | +1.13% (2/5) | 390.8 |
| 8×2 | heldout | release | 22.449 | -0.45% (3/5) | +0.00% (0/5) | 378.0 |
| 8×2 | heldout | ordinary | 22.304 | +0.00% (0/5) | +0.45% (2/5) | 387.5 |
| 8×2 | heldout | reuse | 22.437 | +0.81% (2/5) | +1.19% (1/5) | 374.0 |

| Shape | Input | Reused spans per proof in route probes |
|---|---|---:|
| 4×2 | catalog | 24–29 |
| 4×2 | heldout | 22–30 |
| 8×2 | catalog | 55–88 |
| 8×2 | heldout | 52–81 |

[Source](artifacts/source.patch) · [Design](artifacts/design.md) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

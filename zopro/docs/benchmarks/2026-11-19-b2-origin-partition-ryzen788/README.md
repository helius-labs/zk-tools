# Separate origin and residual scalar grouping

The released [Poseidon-origin option](../2026-11-19-b2-origin-sdk-c7a786/README.md) classifies a scalar at each position before collecting an ordered list of positions per class. This trial builds those position lists separately. Terms from matching full Poseidon states form a parallel set of tasks. Each task groups by output coordinate and verifies the full scalar before reusing a class. Unlabeled terms take a separate ordinary dictionary pass. Unexpected nonzero terms remain exact singleton classes. Independent class lists concatenate and may represent the same scalar more than once.

The optional key plan stores point indices and coordinate masks. It contains no witness scalars or curve points. Per-proof admission charges the separate grouping outputs and their assembly. A denied reservation leaves the ordinary origin or standard grouping route available. The route checks confirm the partition in ordinary conditions and fallback with zero optional memory.

Five matched process blocks on the primary inputs and four on the additional 8×2 inputs use 24 main and eight frontend logical threads on Ryzen. Each process discarded three priming requests and measured 13 full proofs. The released 8×2 control enables origins. The same-build origins comparison holds the scalar conversion setting fixed. All 124 exported proofs independently verified.

The partition did not lead both controls consistently across the measured Ryzen inputs.

| Shape | Input | Profile | Proof ms | Paired vs current origins | Paired vs released SDK | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.580 | +0.18% (2/5) | +0.00% (0/5) | +1.47% (1/5) | 214.5 |
| 4×2 | catalog | ordinary | 14.531 | -1.19% (4/5) | -1.45% (4/5) | +0.00% (0/5) | 201.2 |
| 4×2 | catalog | origins | 14.568 | +0.00% (0/5) | -0.18% (3/5) | +1.21% (1/5) | 227.5 |
| 4×2 | catalog | split | 14.479 | -0.51% (3/5) | -0.38% (3/5) | +0.91% (2/5) | 253.8 |
| 4×2 | heldout | release | 14.454 | +0.56% (1/5) | +0.00% (0/5) | +0.37% (1/5) | 203.8 |
| 4×2 | heldout | ordinary | 14.466 | +0.83% (2/5) | -0.37% (4/5) | +0.00% (0/5) | 210.8 |
| 4×2 | heldout | origins | 14.348 | +0.00% (0/5) | -0.56% (4/5) | -0.82% (3/5) | 205.0 |
| 4×2 | heldout | split | 14.481 | +0.53% (1/5) | +0.27% (2/5) | -0.07% (3/5) | 211.2 |
| 8×2 | catalog | release | 22.558 | +0.05% (2/5) | +0.00% (0/5) | +0.22% (2/5) | 408.0 |
| 8×2 | catalog | ordinary | 22.508 | +0.92% (2/5) | -0.22% (3/5) | +0.00% (0/5) | 421.5 |
| 8×2 | catalog | origins | 22.317 | +0.00% (0/5) | -0.05% (3/5) | -0.92% (3/5) | 414.8 |
| 8×2 | catalog | split | 22.211 | -0.37% (4/5) | -1.54% (5/5) | -1.32% (3/5) | 405.5 |
| 8×2 | heldout | release | 22.672 | +0.11% (2/5) | +0.00% (0/5) | +1.24% (0/5) | 410.8 |
| 8×2 | heldout | ordinary | 22.363 | -1.18% (5/5) | -1.22% (5/5) | +0.00% (0/5) | 407.5 |
| 8×2 | heldout | origins | 22.593 | +0.00% (0/5) | -0.11% (3/5) | +1.19% (0/5) | 410.8 |
| 8×2 | heldout | split | 22.391 | -0.90% (4/5) | -1.81% (4/5) | -0.20% (3/5) | 398.8 |
| 8×2 | v153 | release | 22.517 | -0.20% (3/4) | +0.00% (0/4) | +0.46% (1/4) | 395.0 |
| 8×2 | v153 | ordinary | 22.490 | -1.08% (3/4) | -0.46% (3/4) | +0.00% (0/4) | 400.0 |
| 8×2 | v153 | origins | 22.651 | +0.00% (0/4) | +0.20% (1/4) | +1.10% (1/4) | 406.8 |
| 8×2 | v153 | split | 22.562 | -1.17% (2/4) | +0.20% (2/4) | -0.96% (2/4) | 399.0 |
| 8×2 | v156 | release | 22.939 | -0.90% (3/4) | +0.00% (0/4) | +0.67% (2/4) | 400.8 |
| 8×2 | v156 | ordinary | 22.564 | -1.54% (3/4) | -0.65% (2/4) | +0.00% (0/4) | 403.8 |
| 8×2 | v156 | origins | 22.918 | +0.00% (0/4) | +0.91% (1/4) | +1.58% (1/4) | 421.0 |
| 8×2 | v156 | split | 22.305 | -2.71% (4/4) | -2.95% (4/4) | -1.18% (4/4) | 401.0 |

| Shape | Input | Original terms | Classes | Nonzero terms | Poseidon state groups |
|---|---|---:|---:|---:|---:|
| 4×2 | catalog | 95,537 | 29,738 | 86,163 | 75 |
| 4×2 | heldout | 95,537 | 29,961 | 83,409 | 71 |
| 8×2 | catalog | 183,347 | 38,315 | 173,466 | 89 |
| 8×2 | heldout | 183,347 | 39,891 | 173,850 | 91 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

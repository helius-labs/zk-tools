# Montgomery inputs for folded G1 queries

The trial feeds Montgomery scalar words directly to selected G1 MSMs and removes the common Montgomery factor from each resulting point. For A and K, the existing scalar grouping compares those same words and retains their representation. H uses the existing dense Montgomery route. The experiment changes neither FFT values nor proving-key points and uses the ordinary memory admission and fallback for grouping.

Ryzen uses 24 main and eight frontend logical CPUs. 4×2 uses the frontend solver. 8×2 uses recognized Poseidon origins and serial B2 scalar conversion. The two primary inputs use five matched blocks. Two additional 8×2 inputs use four. Each process primes three proofs and times thirteen. All 156 exported proofs independently verified. The native arithmetic test checks zero, repeated and full-width scalar classes against independent generator arithmetic. Route probes cover the zero optional-workspace case.

No Montgomery-input mode beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.590 | +1.24% (1/5) | +0.00% (0/5) | 206.0 |
| 4×2 | catalog | ordinary | 14.416 | +0.00% (0/5) | -1.22% (4/5) | 215.0 |
| 4×2 | catalog | mont-h | 14.379 | -0.26% (4/5) | -1.10% (5/5) | 209.2 |
| 4×2 | catalog | mont-ak | 14.320 | -0.18% (4/5) | -1.41% (4/5) | 202.8 |
| 4×2 | catalog | mont-all | 14.379 | -0.18% (4/5) | -1.48% (4/5) | 203.5 |
| 4×2 | heldout | release | 14.466 | +0.62% (1/5) | +0.00% (0/5) | 196.0 |
| 4×2 | heldout | ordinary | 14.370 | +0.00% (0/5) | -0.62% (4/5) | 227.0 |
| 4×2 | heldout | mont-h | 14.301 | -0.10% (3/5) | -1.03% (4/5) | 202.5 |
| 4×2 | heldout | mont-ak | 14.530 | -0.03% (3/5) | -1.18% (3/5) | 201.5 |
| 4×2 | heldout | mont-all | 14.610 | +0.55% (2/5) | +1.04% (2/5) | 209.8 |
| 8×2 | catalog | release | 22.648 | -0.18% (3/5) | +0.00% (0/5) | 397.2 |
| 8×2 | catalog | ordinary | 22.454 | +0.00% (0/5) | +0.18% (2/5) | 400.8 |
| 8×2 | catalog | mont-h | 22.383 | -1.17% (3/5) | -0.71% (4/5) | 397.8 |
| 8×2 | catalog | mont-ak | 22.456 | +0.08% (2/5) | -0.55% (3/5) | 400.8 |
| 8×2 | catalog | mont-all | 22.274 | -1.91% (4/5) | -0.95% (4/5) | 391.0 |
| 8×2 | heldout | release | 22.810 | +0.28% (2/5) | +0.00% (0/5) | 414.8 |
| 8×2 | heldout | ordinary | 22.736 | +0.00% (0/5) | -0.28% (3/5) | 407.8 |
| 8×2 | heldout | mont-h | 22.748 | +0.02% (2/5) | +0.51% (2/5) | 400.8 |
| 8×2 | heldout | mont-ak | 22.683 | +0.06% (2/5) | +1.09% (2/5) | 399.8 |
| 8×2 | heldout | mont-all | 22.633 | -0.47% (3/5) | -0.42% (3/5) | 403.5 |
| 8×2 | v153 | release | 22.900 | +0.29% (1/4) | +0.00% (0/4) | 392.0 |
| 8×2 | v153 | ordinary | 22.644 | +0.00% (0/4) | -0.29% (3/4) | 398.0 |
| 8×2 | v153 | mont-h | 22.550 | -0.79% (4/4) | -1.45% (4/4) | 399.5 |
| 8×2 | v153 | mont-ak | 22.944 | +0.52% (2/4) | +0.58% (1/4) | 413.0 |
| 8×2 | v153 | mont-all | 22.495 | -0.56% (3/4) | -1.80% (4/4) | 413.2 |
| 8×2 | v156 | release | 22.588 | +0.26% (2/4) | +0.00% (0/4) | 399.2 |
| 8×2 | v156 | ordinary | 22.423 | +0.00% (0/4) | -0.25% (2/4) | 406.0 |
| 8×2 | v156 | mont-h | 22.364 | -0.06% (2/4) | -0.47% (2/4) | 402.2 |
| 8×2 | v156 | mont-ak | 22.562 | +0.32% (1/4) | +0.17% (2/4) | 401.0 |
| 8×2 | v156 | mont-all | 22.490 | +0.30% (1/4) | -0.48% (3/4) | 400.0 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

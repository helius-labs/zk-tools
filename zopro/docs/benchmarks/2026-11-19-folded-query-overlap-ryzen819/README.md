# Overlap folded curve queries with the quotient FFT

The trial starts B2 while the frontend pool runs the folded Euler quotient transform. The second schedule also starts G1 A before the transform finishes. K and H become eligible after the frontend returns the quotient. Both schedules use the ordinary curve routines and proof inputs. Extra overlap memory is reserved before routing. No proof or point data is retained across requests.

Ryzen uses 24 main and eight frontend logical CPUs. 4×2 enables the frontend solver. 8×2 uses recognized Poseidon origins and serial scalar conversion. The two primary inputs use five matched blocks. Two additional 8×2 inputs use four. Each process primes three requests and measures thirteen. All 128 exported proofs independently verified. Route probes checked both schedules with usual admission and zero optional workspace.

Neither early-query schedule beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.586 | -0.01% (3/5) | +0.00% (0/5) | 210.5 |
| 4×2 | catalog | ordinary | 14.588 | +0.00% (0/5) | +0.01% (2/5) | 200.2 |
| 4×2 | catalog | early-b2 | 14.692 | +0.86% (1/5) | +0.35% (2/5) | 184.0 |
| 4×2 | catalog | early-ab2 | 14.970 | +2.73% (0/5) | +2.75% (0/5) | 184.0 |
| 4×2 | heldout | release | 14.482 | -0.14% (3/5) | +0.00% (0/5) | 193.2 |
| 4×2 | heldout | ordinary | 14.494 | +0.00% (0/5) | +0.14% (2/5) | 210.8 |
| 4×2 | heldout | early-b2 | 14.812 | +1.92% (0/5) | +2.54% (0/5) | 182.8 |
| 4×2 | heldout | early-ab2 | 15.025 | +3.70% (0/5) | +3.75% (0/5) | 179.0 |
| 8×2 | catalog | release | 22.501 | -0.69% (3/5) | +0.00% (0/5) | 391.5 |
| 8×2 | catalog | ordinary | 22.468 | +0.00% (0/5) | +0.70% (2/5) | 396.2 |
| 8×2 | catalog | early-b2 | 22.757 | -0.33% (3/5) | -0.23% (3/5) | 390.8 |
| 8×2 | catalog | early-ab2 | 22.924 | +2.13% (1/5) | +1.53% (1/5) | 372.0 |
| 8×2 | heldout | release | 22.667 | -1.17% (4/5) | +0.00% (0/5) | 404.0 |
| 8×2 | heldout | ordinary | 22.968 | +0.00% (0/5) | +1.18% (1/5) | 403.2 |
| 8×2 | heldout | early-b2 | 22.742 | -0.45% (4/5) | +0.33% (2/5) | 374.0 |
| 8×2 | heldout | early-ab2 | 23.158 | +2.29% (1/5) | +2.38% (0/5) | 366.5 |
| 8×2 | v153 | release | 22.668 | -0.00% (2/4) | +0.00% (0/4) | 395.2 |
| 8×2 | v153 | ordinary | 22.659 | +0.00% (0/4) | +0.03% (2/4) | 425.0 |
| 8×2 | v153 | early-b2 | 23.092 | +1.43% (1/4) | +2.30% (1/4) | 390.8 |
| 8×2 | v153 | early-ab2 | 23.128 | +1.67% (0/4) | +1.85% (1/4) | 373.5 |
| 8×2 | v156 | release | 22.634 | +0.20% (1/4) | +0.00% (0/4) | 396.2 |
| 8×2 | v156 | ordinary | 22.600 | +0.00% (0/4) | -0.20% (3/4) | 399.8 |
| 8×2 | v156 | early-b2 | 22.897 | +1.26% (0/4) | +0.85% (0/4) | 396.5 |
| 8×2 | v156 | early-ab2 | 23.345 | +3.21% (0/4) | +3.33% (0/4) | 375.5 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

# Overlap witness gather with the quotient FFT

The trial overlaps the existing witness-index gathers with the folded Euler quotient FFT. The FFT stays on the main pool. The two gathering modes use the main or frontend pool. All ordinary curve queries wait for both outputs as before. Extra overlap memory is charged before allocation. The ordinary schedule handles denied reservations.

Ryzen uses 24 main and eight frontend logical CPUs. 4×2 enables the frontend solver. 8×2 uses recognized Poseidon origins and serial B2 conversion. The two primary inputs use five matched blocks. Two additional 8×2 inputs use four. Each process primes three proofs and times thirteen. All 128 exported proofs independently verified. Route probes cover both placements and the zero optional-workspace fallback.

Neither gather schedule beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.451 | +0.57% (1/5) | +0.00% (0/5) | 212.8 |
| 4×2 | catalog | ordinary | 14.448 | +0.00% (0/5) | -0.57% (4/5) | 202.0 |
| 4×2 | catalog | gather-front | 14.442 | +0.77% (2/5) | +0.19% (2/5) | 203.2 |
| 4×2 | catalog | gather-main | 14.660 | +1.18% (0/5) | +0.34% (1/5) | 200.2 |
| 4×2 | heldout | release | 14.605 | +0.66% (2/5) | +0.00% (0/5) | 210.8 |
| 4×2 | heldout | ordinary | 14.463 | +0.00% (0/5) | -0.65% (3/5) | 216.8 |
| 4×2 | heldout | gather-front | 14.432 | -0.03% (3/5) | -0.76% (4/5) | 240.2 |
| 4×2 | heldout | gather-main | 14.534 | +0.15% (2/5) | -0.58% (5/5) | 199.2 |
| 8×2 | catalog | release | 22.717 | -0.44% (4/5) | +0.00% (0/5) | 410.5 |
| 8×2 | catalog | ordinary | 22.761 | +0.00% (0/5) | +0.44% (1/5) | 408.2 |
| 8×2 | catalog | gather-front | 22.768 | +0.85% (2/5) | +0.22% (2/5) | 401.0 |
| 8×2 | catalog | gather-main | 22.553 | -1.52% (3/5) | +0.58% (1/5) | 386.5 |
| 8×2 | heldout | release | 23.005 | +0.11% (2/5) | +0.00% (0/5) | 405.5 |
| 8×2 | heldout | ordinary | 22.676 | +0.00% (0/5) | -0.11% (3/5) | 405.0 |
| 8×2 | heldout | gather-front | 23.394 | +3.89% (0/5) | +3.54% (1/5) | 406.0 |
| 8×2 | heldout | gather-main | 23.032 | +0.58% (2/5) | +2.81% (0/5) | 417.0 |
| 8×2 | v153 | release | 22.607 | +0.34% (1/4) | +0.00% (0/4) | 412.0 |
| 8×2 | v153 | ordinary | 22.633 | +0.00% (0/4) | -0.34% (3/4) | 393.8 |
| 8×2 | v153 | gather-front | 22.771 | +0.46% (2/4) | +0.35% (1/4) | 399.2 |
| 8×2 | v153 | gather-main | 22.844 | +1.53% (1/4) | +1.13% (0/4) | 390.8 |
| 8×2 | v156 | release | 22.637 | -0.69% (3/4) | +0.00% (0/4) | 413.8 |
| 8×2 | v156 | ordinary | 22.905 | +0.00% (0/4) | +0.70% (1/4) | 401.0 |
| 8×2 | v156 | gather-front | 23.238 | +1.35% (2/4) | +2.66% (1/4) | 434.0 |
| 8×2 | v156 | gather-main | 23.003 | -0.04% (2/4) | +0.89% (0/4) | 370.8 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

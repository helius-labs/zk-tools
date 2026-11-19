# Grouped G2 windows on the frontend pool

The trial splits the unsigned windows for the grouped B2 query between the existing main and frontend Rayon pools. Point grouping stays on the main pool, and the two results are combined in their original window order. Extra scratch is charged to the optional workspace. Each proof builds its own groups from its input.

Ryzen uses 24 main and eight frontend logical CPUs. 4×2 enables the frontend solver. 8×2 uses recognized Poseidon origins and serial scalar conversion. Five matched blocks per input compare four window splits with the released SDK and a same-build ordinary control. Each process primes three requests and measures thirteen. The native test compared independent generator sums across three window widths and three range schedules using two pools. All 128 exported proofs independently verified. Route probes exercised both pools and checked the ordinary path with a zero-byte optional workspace budget.

None of the window splits beat both controls consistently on either shape. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.524 | +0.31% (1/5) | +0.00% (0/5) | 210.8 |
| 4×2 | catalog | ordinary | 14.479 | +0.00% (0/5) | -0.31% (4/5) | 218.2 |
| 4×2 | catalog | front-4 | 14.600 | -0.06% (3/5) | +0.73% (2/5) | 212.0 |
| 4×2 | catalog | front-8 | 15.204 | +5.41% (1/5) | +4.82% (0/5) | 198.8 |
| 4×2 | catalog | front-12 | 16.143 | +12.29% (1/5) | +11.03% (0/5) | 218.0 |
| 4×2 | catalog | front-16 | 16.913 | +16.65% (0/5) | +16.32% (0/5) | 210.2 |
| 4×2 | heldout | release | 14.482 | -0.01% (3/5) | +0.00% (0/5) | 202.2 |
| 4×2 | heldout | ordinary | 14.500 | +0.00% (0/5) | +0.01% (2/5) | 201.2 |
| 4×2 | heldout | front-4 | 14.390 | -0.76% (4/5) | -1.20% (4/5) | 207.5 |
| 4×2 | heldout | front-8 | 15.102 | +3.86% (1/5) | +3.93% (1/5) | 213.5 |
| 4×2 | heldout | front-12 | 16.114 | +11.55% (0/5) | +11.42% (0/5) | 204.5 |
| 4×2 | heldout | front-16 | 16.938 | +16.36% (0/5) | +16.95% (0/5) | 204.2 |
| 8×2 | catalog | release | 22.434 | -0.31% (3/5) | +0.00% (0/5) | 402.0 |
| 8×2 | catalog | ordinary | 22.520 | +0.00% (0/5) | +0.31% (2/5) | 404.0 |
| 8×2 | catalog | front-4 | 22.422 | -1.19% (5/5) | -0.65% (4/5) | 393.5 |
| 8×2 | catalog | front-8 | 22.516 | -0.02% (3/5) | +0.84% (2/5) | 414.5 |
| 8×2 | catalog | front-12 | 23.512 | +4.45% (0/5) | +4.66% (0/5) | 407.8 |
| 8×2 | catalog | front-16 | 24.613 | +9.29% (0/5) | +10.25% (0/5) | 403.0 |
| 8×2 | heldout | release | 22.647 | -0.08% (3/5) | +0.00% (0/5) | 387.0 |
| 8×2 | heldout | ordinary | 22.664 | +0.00% (0/5) | +0.08% (2/5) | 405.2 |
| 8×2 | heldout | front-4 | 22.454 | -1.01% (3/5) | -0.50% (3/5) | 397.8 |
| 8×2 | heldout | front-8 | 22.656 | +0.43% (2/5) | +0.39% (1/5) | 418.8 |
| 8×2 | heldout | front-12 | 24.043 | +6.02% (1/5) | +6.54% (0/5) | 413.5 |
| 8×2 | heldout | front-16 | 24.807 | +9.12% (1/5) | +9.73% (0/5) | 407.0 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

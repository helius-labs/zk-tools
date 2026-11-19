# G1 and G2 query replay on Ryzen

This fixed-request diagnostic measures the complete proof after replacing selected curve queries with results computed during warmup. The timed path still solves the witness, checks full coefficients for every reused curve query, verifies the proof and exports a fresh proof. A different witness and unavailable optional memory select the ordinary path. The benchmark retains replay state for one immutable key and one process. 1×2 uses its existing fused K/H route and 2×2 enables the medium fused K/H option.

Both shapes use 24 main and eight frontend logical CPUs on Ryzen. Five matched blocks per input include a released SDK control and a same-binary ordinary control. Each process warms three requests and measures thirteen. Routing checks confirm A and K/H reuse independently, all-curve reuse, mismatches with a different warmup request and fallback under zero optional memory. All 160 exports independently verified.

| Shape | Input | Ordinary ms | A replay ms | K/H replay ms | B2 replay ms | Both G1 replay ms | All curve queries replay ms |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 6.969 | 6.792 | 5.108 | 5.063 | 4.418 | 2.170 |
| 1×2 | heldout | 6.959 | 6.666 | 4.887 | 4.968 | 4.488 | 2.156 |
| 2×2 | catalog | 9.566 | 9.054 | 6.949 | 7.244 | 6.359 | 3.131 |
| 2×2 | heldout | 9.554 | 9.034 | 6.980 | 7.257 | 6.409 | 3.085 |

The net changes include altered task overlap and the coefficient comparisons. Per-query savings need not add. Replay values are experimental diagnostics and excluded from the performance scorecard.

| Shape | Input | Mode | Proof ms | Paired Δ vs ordinary ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.063 | +0.045 | +0.64% (2/5) | +0.00% (0/5) | 57.5 |
| 1×2 | catalog | ordinary | 6.969 | +0.000 | +0.00% (0/5) | -0.63% (3/5) | 57.5 |
| 1×2 | catalog | a | 6.792 | -0.177 | -2.55% (5/5) | -3.35% (5/5) | 57.8 |
| 1×2 | catalog | kh | 5.108 | -1.827 | -26.34% (5/5) | -25.03% (5/5) | 56.2 |
| 1×2 | catalog | b2 | 5.063 | -1.906 | -27.35% (5/5) | -26.31% (5/5) | 52.8 |
| 1×2 | catalog | both | 4.418 | -2.516 | -36.28% (5/5) | -35.64% (5/5) | 56.2 |
| 1×2 | catalog | all | 2.170 | -4.762 | -68.77% (5/5) | -69.57% (5/5) | 57.0 |
| 1×2 | heldout | release | 7.062 | -0.160 | -2.30% (3/5) | +0.00% (0/5) | 57.2 |
| 1×2 | heldout | ordinary | 6.959 | +0.000 | +0.00% (0/5) | +2.36% (2/5) | 57.8 |
| 1×2 | heldout | a | 6.666 | -0.378 | -5.43% (5/5) | -4.66% (5/5) | 58.0 |
| 1×2 | heldout | kh | 4.887 | -2.033 | -29.22% (5/5) | -28.89% (5/5) | 55.2 |
| 1×2 | heldout | b2 | 4.968 | -2.046 | -29.47% (5/5) | -29.42% (5/5) | 52.2 |
| 1×2 | heldout | both | 4.488 | -2.560 | -37.03% (5/5) | -36.15% (5/5) | 55.0 |
| 1×2 | heldout | all | 2.156 | -4.804 | -69.02% (5/5) | -68.80% (5/5) | 55.0 |
| 2×2 | catalog | release | 9.481 | +0.031 | +0.33% (2/5) | +0.00% (0/5) | 94.0 |
| 2×2 | catalog | ordinary | 9.566 | +0.000 | +0.00% (0/5) | -0.33% (3/5) | 94.0 |
| 2×2 | catalog | a | 9.054 | -0.544 | -5.61% (5/5) | -4.22% (5/5) | 94.2 |
| 2×2 | catalog | kh | 6.949 | -2.533 | -26.34% (5/5) | -26.71% (5/5) | 94.2 |
| 2×2 | catalog | b2 | 7.244 | -2.186 | -22.86% (5/5) | -23.10% (5/5) | 85.2 |
| 2×2 | catalog | both | 6.359 | -3.189 | -33.16% (5/5) | -33.37% (5/5) | 94.2 |
| 2×2 | catalog | all | 3.131 | -6.428 | -67.40% (5/5) | -67.39% (5/5) | 89.0 |
| 2×2 | heldout | release | 9.551 | -0.003 | -0.03% (3/5) | +0.00% (0/5) | 98.5 |
| 2×2 | heldout | ordinary | 9.554 | +0.000 | +0.00% (0/5) | +0.03% (2/5) | 94.2 |
| 2×2 | heldout | a | 9.034 | -0.517 | -5.46% (5/5) | -4.84% (5/5) | 94.0 |
| 2×2 | heldout | kh | 6.980 | -2.572 | -27.15% (5/5) | -26.81% (5/5) | 91.2 |
| 2×2 | heldout | b2 | 7.257 | -2.287 | -24.05% (5/5) | -23.92% (5/5) | 84.0 |
| 2×2 | heldout | both | 6.409 | -3.103 | -32.48% (5/5) | -32.46% (5/5) | 92.8 |
| 2×2 | heldout | all | 3.085 | -6.455 | -67.57% (5/5) | -67.56% (5/5) | 88.8 |

[Samples](artifacts/rows.json.gz) · [Routes](artifacts/routes.json) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured non-replay C7a profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 798 SDK medium K/H 64 main | 12.127 | 59 / 30 | 4.87× / 2.47× | +6.227 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

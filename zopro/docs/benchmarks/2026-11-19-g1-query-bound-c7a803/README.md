# G1 and G2 query replay on C7a

This fixed-request diagnostic measures the complete proof after replacing selected curve queries with results computed during warmup. The timed path still solves the witness, checks full coefficients for every reused curve query, verifies the proof and exports a fresh proof. A different witness and unavailable optional memory select the ordinary path. The benchmark retains replay state for one immutable key and one process. 1×2 uses its existing fused K/H route and 2×2 enables the medium fused K/H option.

1×2 uses 80 main and 16 frontend physical cores. 2×2 uses 64 main and 32 frontend physical cores. The single C7a instance and its temporary storage were removed. Five matched blocks per input include a released SDK control and a same-binary ordinary control. Each process warms three requests and measures thirteen. Routing checks confirm A and K/H reuse independently, all-curve reuse, mismatches with a different warmup request and fallback under zero optional memory. All 160 exports independently verified.

| Shape | Input | Ordinary ms | A replay ms | K/H replay ms | B2 replay ms | Both G1 replay ms | All curve queries replay ms |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.817 | 7.967 | 7.728 | 7.348 | 7.615 | 4.007 |
| 1×2 | heldout | 7.770 | 7.868 | 7.740 | 7.303 | 7.546 | 4.004 |
| 2×2 | catalog | 12.089 | 12.027 | 12.087 | 10.912 | 11.773 | 5.514 |
| 2×2 | heldout | 12.147 | 11.809 | 12.237 | 10.916 | 12.135 | 5.529 |

The net changes include altered task overlap and the coefficient comparisons. Per-query savings need not add. Replay values are experimental diagnostics and excluded from the performance scorecard.

| Shape | Input | Mode | Proof ms | Paired Δ vs ordinary ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.809 | -0.007 | -0.09% (3/5) | +0.00% (0/5) | 42.3 |
| 1×2 | catalog | ordinary | 7.817 | +0.000 | +0.00% (0/5) | +0.09% (2/5) | 41.6 |
| 1×2 | catalog | a | 7.967 | +0.152 | +1.94% (0/5) | +2.01% (0/5) | 37.8 |
| 1×2 | catalog | kh | 7.728 | -0.064 | -0.82% (5/5) | -1.05% (3/5) | 40.0 |
| 1×2 | catalog | b2 | 7.348 | -0.447 | -5.74% (5/5) | -5.90% (5/5) | 43.1 |
| 1×2 | catalog | both | 7.615 | -0.202 | -2.58% (5/5) | -2.50% (5/5) | 38.5 |
| 1×2 | catalog | all | 4.007 | -3.802 | -48.61% (5/5) | -48.57% (5/5) | 40.0 |
| 1×2 | heldout | release | 7.775 | -0.017 | -0.22% (3/5) | +0.00% (0/5) | 39.5 |
| 1×2 | heldout | ordinary | 7.770 | +0.000 | +0.00% (0/5) | +0.22% (2/5) | 43.0 |
| 1×2 | heldout | a | 7.868 | +0.095 | +1.23% (0/5) | +1.21% (0/5) | 36.3 |
| 1×2 | heldout | kh | 7.740 | -0.024 | -0.30% (3/5) | -0.37% (4/5) | 39.3 |
| 1×2 | heldout | b2 | 7.303 | -0.454 | -5.86% (5/5) | -6.18% (5/5) | 40.7 |
| 1×2 | heldout | both | 7.546 | -0.231 | -2.97% (5/5) | -2.94% (5/5) | 39.2 |
| 1×2 | heldout | all | 4.004 | -3.766 | -48.48% (5/5) | -48.58% (5/5) | 38.5 |
| 2×2 | catalog | release | 12.051 | -0.038 | -0.31% (3/5) | +0.00% (0/5) | 74.4 |
| 2×2 | catalog | ordinary | 12.089 | +0.000 | +0.00% (0/5) | +0.31% (2/5) | 72.9 |
| 2×2 | catalog | a | 12.027 | -0.146 | -1.21% (4/5) | -0.30% (3/5) | 71.5 |
| 2×2 | catalog | kh | 12.087 | -0.095 | -0.78% (3/5) | +0.21% (2/5) | 73.7 |
| 2×2 | catalog | b2 | 10.912 | -1.265 | -10.39% (5/5) | -8.68% (5/5) | 75.9 |
| 2×2 | catalog | both | 11.773 | -0.292 | -2.43% (5/5) | -2.73% (4/5) | 72.3 |
| 2×2 | catalog | all | 5.514 | -6.566 | -54.10% (5/5) | -54.14% (5/5) | 70.1 |
| 2×2 | heldout | release | 12.060 | +0.005 | +0.04% (2/5) | +0.00% (0/5) | 73.8 |
| 2×2 | heldout | ordinary | 12.147 | +0.000 | +0.00% (0/5) | -0.04% (3/5) | 74.5 |
| 2×2 | heldout | a | 11.809 | -0.242 | -1.99% (5/5) | -2.03% (4/5) | 72.9 |
| 2×2 | heldout | kh | 12.237 | +0.255 | +2.11% (1/5) | +2.05% (0/5) | 73.8 |
| 2×2 | heldout | b2 | 10.916 | -1.188 | -9.77% (5/5) | -9.49% (5/5) | 78.2 |
| 2×2 | heldout | both | 12.135 | -0.022 | -0.18% (3/5) | -0.22% (3/5) | 74.4 |
| 2×2 | heldout | all | 5.529 | -6.624 | -54.52% (5/5) | -54.04% (5/5) | 69.3 |

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

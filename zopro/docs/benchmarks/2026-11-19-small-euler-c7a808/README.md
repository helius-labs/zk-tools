# Small Euler FFT paths on C7a

The released SDK already supports packing Euler FFT input columns, applying local inverse and forward transform stages in the same leaf, and emitting the final quotient values from the last column pass. The trial enables those changes cumulatively. Ordinary uses the release settings on the two small shapes. Columns switches the initial packing. Center adds local fusion. Emit also removes an intermediate output pass. The solver, point queries, worker placement and keys are the same across the Zopro timing arms.

One c7a.24xlarge runs 1×2 with 80 main and 16 frontend physical cores, and 2×2 with 64 main and 32 frontend cores. Zopro 2×2 uses medium fused K/H in every arm. Prepared Aeglos uses all 96 cores. Six matched blocks per input measure thirteen proofs after three warmups. All 136 exports independently verified. The packed local transform was observed in 8 of eight applicable routing processes. The instance and temporary storage were removed.

- 1×2 `emit` took 7.982 / 7.935 ms across original and held-out, paired +1.91% / +2.02% from the same-build ordinary arm.
- 2×2 `emit` took 12.009 / 12.086 ms across original and held-out, paired -0.68% / +0.43% from the same-build ordinary arm.

None of the layouts improved both primary inputs consistently against the ordinary path.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | ordinary | 7.837 | +0.00% (0/6) | -4.05% (6/6) | 44.6 |
| 1×2 | catalog | columns | 8.087 | +3.50% (0/6) | -1.41% (5/6) | 40.7 |
| 1×2 | catalog | center | 8.003 | +2.07% (0/6) | -1.97% (4/6) | 40.7 |
| 1×2 | catalog | emit | 7.982 | +1.91% (1/6) | -2.92% (5/6) | 42.2 |
| 1×2 | catalog | aeglos | 8.141 | +4.22% (0/6) | +0.00% (0/6) | 456.8 |
| 1×2 | heldout | ordinary | 7.776 | +0.00% (0/6) | -5.38% (6/6) | 40.0 |
| 1×2 | heldout | columns | 8.016 | +3.10% (0/6) | -1.40% (4/6) | 38.5 |
| 1×2 | heldout | center | 7.985 | +2.61% (0/6) | -2.76% (6/6) | 42.3 |
| 1×2 | heldout | emit | 7.935 | +2.02% (0/6) | -3.11% (6/6) | 40.8 |
| 1×2 | heldout | aeglos | 8.203 | +5.68% (0/6) | +0.00% (0/6) | 459.5 |
| 2×2 | catalog | ordinary | 12.054 | +0.00% (0/6) | +4.91% (0/6) | 76.7 |
| 2×2 | catalog | columns | 12.357 | +2.79% (1/6) | +8.17% (0/6) | 77.5 |
| 2×2 | catalog | center | 12.210 | +0.88% (1/6) | +6.36% (0/6) | 78.1 |
| 2×2 | catalog | emit | 12.009 | -0.68% (4/6) | +4.46% (0/6) | 75.3 |
| 2×2 | catalog | aeglos | 11.511 | -4.68% (6/6) | +0.00% (0/6) | 803.7 |
| 2×2 | heldout | ordinary | 12.051 | +0.00% (0/6) | +3.87% (0/6) | 76.8 |
| 2×2 | heldout | columns | 12.434 | +3.46% (0/6) | +7.86% (0/6) | 75.3 |
| 2×2 | heldout | center | 12.285 | +2.07% (1/6) | +5.29% (0/6) | 77.4 |
| 2×2 | heldout | emit | 12.086 | +0.43% (2/6) | +4.15% (0/6) | 73.1 |
| 2×2 | heldout | aeglos | 11.589 | -3.73% (6/6) | +0.00% (0/6) | 804.3 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Routing](artifacts/routes.json) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |

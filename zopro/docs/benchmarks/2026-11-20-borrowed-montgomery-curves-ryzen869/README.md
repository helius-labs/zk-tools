# Borrowing Montgomery scalar words for the small curve queries

The experimental CPU route carries the solver's existing Montgomery words into A, K and B2 and passes the quotient's existing Montgomery words to H. It omits the conversions of the entire solved witness and quotient. It scales the curve results by the inverse of the common factor before applying public corrections. The fused K/H path needs one scaling for the joint result. No persistent table, key copy or proof-local scalar copy is added. Both query routes retain their original input gathers.

The native generator test independently checks the common factor for G1 and G2 with zero, unit, negative unit and full-width scalar values. Complete SDK runs check the original verifying key, all public outputs and the option with zero optional workspace. All 96 proof exports independently verified.

Both circuits use 24 main and eight frontend Ryzen logical CPUs. The 2×2 panel uses medium fused K/H when admitted. Seven matched blocks per input prime three calls and measure thirteen. A candidate advances with a lower paired median and at least five wins against both controls on both inputs.

The Montgomery-input route did not consistently improve both controls across both inputs of either shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.888 | +0.29% (1/7) | +0.00% (0/7) | 133.0 | 96 | 57.5 |
| 1×2 | catalog | ordinary | 6.856 | +0.00% (0/7) | -0.29% (6/7) | 133.9 | 116 | 57.8 |
| 1×2 | catalog | montgomery | 7.481 | +8.90% (0/7) | +8.84% (0/7) | 141.6 | 90 | 51.5 |
| 1×2 | heldout | release | 6.737 | -0.39% (6/7) | +0.00% (0/7) | 128.2 | 113 | 58.2 |
| 1×2 | heldout | ordinary | 6.767 | +0.00% (0/7) | +0.39% (1/7) | 129.7 | 107 | 58.0 |
| 1×2 | heldout | montgomery | 7.502 | +11.80% (0/7) | +11.38% (0/7) | 139.6 | 78 | 53.0 |
| 2×2 | catalog | release | 9.543 | +0.95% (3/7) | +0.00% (0/7) | 180.7 | 267 | 94.8 |
| 2×2 | catalog | ordinary | 9.520 | +0.00% (0/7) | -0.94% (4/7) | 182.0 | 219 | 97.5 |
| 2×2 | catalog | montgomery | 9.782 | +3.83% (0/7) | +2.34% (0/7) | 191.1 | 279 | 100.5 |
| 2×2 | heldout | release | 9.581 | +0.90% (3/7) | +0.00% (0/7) | 181.7 | 223 | 95.5 |
| 2×2 | heldout | ordinary | 9.507 | +0.00% (0/7) | -0.89% (4/7) | 182.8 | 297 | 93.5 |
| 2×2 | heldout | montgomery | 9.861 | +3.56% (0/7) | +3.98% (0/7) | 192.4 | 223 | 105.2 |

| Shape | Input | Probe | A terms | B2 terms | Solver wire words | H terms |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | montgomery | 21,006 | 30,089 | 30,531 | 30,922 |
| 1×2 | catalog | montgomery-limited | 21,006 | 30,089 | 30,531 | 30,922 |
| 1×2 | heldout | montgomery | 21,006 | 30,089 | 30,531 | 30,922 |
| 1×2 | heldout | montgomery-limited | 21,006 | 30,089 | 30,531 | 30,922 |
| 2×2 | catalog | montgomery | 36,637 | 52,274 | 53,081 | 53,874 |
| 2×2 | catalog | montgomery-limited | 36,637 | 52,274 | 53,081 | 53,874 |
| 2×2 | heldout | montgomery | 36,637 | 52,274 | 53,081 | 53,874 |
| 2×2 | heldout | montgomery-limited | 36,637 | 52,274 | 53,081 | 53,874 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

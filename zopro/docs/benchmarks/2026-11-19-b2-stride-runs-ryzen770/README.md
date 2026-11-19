# Exact strided scalar-run classification on Ryzen

The experiment collapses proof-local runs of equal B2 scalars at the candidate stride of 20,653. It validates equality using all four limbs and uses the exact hash table to combine representatives. A nearby stride provides a control. Parallel scans emit only representative records for serial hashing and output-list construction. First-occurrence class order is preserved. Members within a class are emitted in run order. The proof still includes the complete point sum. The scratch reservation covers temporary run buffers, and denial uses the ordinary calculation.

Matched timing on Ryzen used 24 main and eight frontend logical threads. The original and second requests used five process blocks and the v156 8×2 request used three. Each process warmed three requests and timed 13. Separate traces measured seven requests with phase clocks. The native test compared equality classes, counts and full coverage with the ordinary classifier. All 155 exported proofs independently verified.

The stride mode did not lead all three Zopro controls across the measured requests for either circuit.

| Shape | Input | Profile | Proof ms | Paired vs ordinary layout | Paired vs disabled trial | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.478 | +0.57% (1/5) | +0.42% (1/5) | +0.00% (0/5) | 200.2 |
| 4×2 | catalog | off | 14.446 | +0.15% (2/5) | +0.00% (0/5) | -0.42% (4/5) | 207.8 |
| 4×2 | catalog | b2-base | 14.457 | +0.00% (0/5) | -0.15% (3/5) | -0.57% (4/5) | 200.8 |
| 4×2 | catalog | stride-runs | 14.463 | -0.23% (3/5) | +0.42% (1/5) | -0.11% (4/5) | 206.2 |
| 4×2 | catalog | stride-near | 14.519 | -0.03% (3/5) | +0.82% (2/5) | -0.46% (3/5) | 218.0 |
| 4×2 | heldout | release | 14.587 | +1.76% (2/5) | +1.47% (1/5) | +0.00% (0/5) | 211.5 |
| 4×2 | heldout | off | 14.410 | +0.17% (1/5) | +0.00% (0/5) | -1.45% (4/5) | 204.8 |
| 4×2 | heldout | b2-base | 14.368 | +0.00% (0/5) | -0.17% (4/5) | -1.73% (3/5) | 207.2 |
| 4×2 | heldout | stride-runs | 14.419 | +1.13% (2/5) | +0.06% (2/5) | -0.23% (4/5) | 200.8 |
| 4×2 | heldout | stride-near | 14.350 | +0.33% (2/5) | -0.42% (4/5) | -1.39% (5/5) | 202.5 |
| 8×2 | catalog | release | 22.520 | -0.38% (3/5) | +0.12% (2/5) | +0.00% (0/5) | 393.2 |
| 8×2 | catalog | off | 22.294 | +0.00% (2/5) | +0.00% (0/5) | -0.12% (3/5) | 394.2 |
| 8×2 | catalog | b2-base | 22.513 | +0.00% (0/5) | -0.00% (3/5) | +0.38% (2/5) | 411.2 |
| 8×2 | catalog | stride-runs | 22.427 | -0.56% (3/5) | +0.14% (2/5) | -0.34% (4/5) | 425.2 |
| 8×2 | catalog | stride-near | 22.170 | -0.48% (4/5) | -1.16% (4/5) | -1.55% (3/5) | 406.0 |
| 8×2 | heldout | release | 22.674 | -0.35% (3/5) | +1.54% (2/5) | +0.00% (0/5) | 409.8 |
| 8×2 | heldout | off | 22.471 | -0.27% (3/5) | +0.00% (0/5) | -1.52% (3/5) | 394.5 |
| 8×2 | heldout | b2-base | 22.670 | +0.00% (0/5) | +0.27% (2/5) | +0.35% (2/5) | 413.8 |
| 8×2 | heldout | stride-runs | 22.753 | -0.20% (3/5) | +0.07% (2/5) | -0.93% (3/5) | 430.2 |
| 8×2 | heldout | stride-near | 22.365 | -1.35% (5/5) | -1.00% (4/5) | -2.16% (4/5) | 403.5 |
| 8×2 | v156 | release | 22.421 | +0.34% (1/3) | +0.10% (1/3) | +0.00% (0/3) | 399.0 |
| 8×2 | v156 | off | 22.364 | +0.23% (1/3) | +0.00% (0/3) | -0.10% (2/3) | 399.0 |
| 8×2 | v156 | b2-base | 22.506 | +0.00% (0/3) | -0.23% (2/3) | -0.33% (2/3) | 402.5 |
| 8×2 | v156 | stride-runs | 22.702 | +0.87% (1/3) | +1.06% (1/3) | +0.72% (1/3) | 416.8 |
| 8×2 | v156 | stride-near | 22.271 | -0.18% (2/3) | -0.56% (3/3) | -0.90% (3/3) | 426.0 |

## Separate phase traces

| Shape | Input | Profile | Representatives | Classification ms | Point reduction ms | Final MSM ms | Proof ms |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | b2-base | — | 3.328 | 1.802 | 3.311 | 14.558 |
| 4×2 | catalog | stride-runs | 32247 | 3.145 | 1.860 | 3.346 | 14.610 |
| 4×2 | heldout | b2-base | — | 3.042 | 1.789 | 3.412 | 14.293 |
| 4×2 | heldout | stride-runs | 31262 | 3.530 | 1.580 | 3.266 | 14.486 |
| 8×2 | catalog | b2-base | — | 3.696 | 6.046 | 4.353 | 22.385 |
| 8×2 | catalog | stride-runs | 44625 | 4.535 | 5.453 | 3.991 | 22.474 |
| 8×2 | heldout | b2-base | — | 5.441 | 4.597 | 4.188 | 23.145 |
| 8×2 | heldout | stride-runs | 45403 | 4.872 | 5.389 | 4.388 | 22.897 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json) · [C7a reference timings](../2026-11-19-grouped-b2-c7a768/README.md)

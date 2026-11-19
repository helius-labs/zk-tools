# Validating a compact B2 scalar template on Ryzen

The template contains only index groups and Boolean positions inferred from three synthetic requests per key. It stores no scalars or points. File sizes are 510,792 bytes for 4×2 and 892,096 bytes for 8×2. In five separate synthetic checks, the largest number of non-Boolean exceptions was 57 for 4×2 and 91 for 8×2.

At proof time, the prover checks each term against the actual class representative and checks the predicted Boolean positions against zero and one. It skips zeros, combines ones, and keeps each other mismatch as a singleton with its actual scalar. The plan rejects invalid dimensions and duplicate indices, and calls the ordinary classifier on optional-memory denial or excessive exceptions. A key with the same dimensions and a different valid partition still computes the exact MSM.

Matched proof timing on Ryzen used 24 main and eight frontend logical threads. The original and second requests used five process blocks. Two additional 8×2 requests used four. Timed processes warmed three proofs and measured 13. Separate traces measured seven requests with phase clocks and confirm actual template use. One-request checks cover wrong dimensions and a zero optional-memory allowance. The native test checks malformed layouts, scalar checks, complete membership and fallback. All 152 exports independently verified. The decoder runs in the first warm proof and its memory is included in whole-process RSS.

The template did not lead all three Zopro controls across the measured requests for either circuit.

The original 8×2 request fell back to ordinary classification in the separate route checks and timed phase traces. The other three 8×2 workloads appear in the full timing panel. Only requests with confirmed template activation can establish a template speedup.

| Shape | Input | Profile | Proof ms | Paired vs ordinary layout | Paired vs disabled trial | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.597 | +0.01% (2/5) | +0.08% (2/5) | +0.00% (0/5) | 206.2 |
| 4×2 | catalog | off | 14.536 | +0.14% (2/5) | +0.00% (0/5) | -0.08% (3/5) | 212.5 |
| 4×2 | catalog | b2-base | 14.534 | +0.00% (0/5) | -0.14% (3/5) | -0.01% (3/5) | 203.0 |
| 4×2 | catalog | template | 14.779 | +1.44% (1/5) | +1.40% (1/5) | +1.31% (0/5) | 218.2 |
| 4×2 | heldout | release | 14.506 | -0.10% (4/5) | -0.77% (4/5) | +0.00% (0/5) | 263.5 |
| 4×2 | heldout | off | 14.493 | +0.74% (1/5) | +0.00% (0/5) | +0.78% (1/5) | 222.0 |
| 4×2 | heldout | b2-base | 14.487 | +0.00% (0/5) | -0.73% (4/5) | +0.10% (1/5) | 246.2 |
| 4×2 | heldout | template | 14.537 | +0.47% (1/5) | -0.60% (4/5) | +0.76% (1/5) | 261.0 |
| 8×2 | catalog | release | 22.606 | -0.80% (5/5) | +0.65% (1/5) | +0.00% (0/5) | 408.0 |
| 8×2 | catalog | off | 22.508 | -2.21% (5/5) | +0.00% (0/5) | -0.64% (4/5) | 411.2 |
| 8×2 | catalog | b2-base | 22.865 | +0.00% (0/5) | +2.26% (0/5) | +0.80% (0/5) | 398.2 |
| 8×2 | catalog | template | 22.374 | -2.16% (5/5) | -0.37% (3/5) | -1.01% (4/5) | 421.8 |
| 8×2 | heldout | release | 22.549 | -1.47% (4/5) | -1.18% (4/5) | +0.00% (0/5) | 413.2 |
| 8×2 | heldout | off | 22.757 | -0.17% (3/5) | +0.00% (0/5) | +1.19% (1/5) | 413.2 |
| 8×2 | heldout | b2-base | 22.728 | +0.00% (0/5) | +0.17% (2/5) | +1.49% (1/5) | 411.0 |
| 8×2 | heldout | template | 23.008 | +1.37% (1/5) | +0.82% (0/5) | +2.03% (0/5) | 395.5 |
| 8×2 | v153 | release | 22.643 | -2.38% (3/4) | -0.39% (2/4) | +0.00% (0/4) | 404.0 |
| 8×2 | v153 | off | 22.733 | -1.86% (3/4) | +0.00% (0/4) | +0.40% (2/4) | 409.5 |
| 8×2 | v153 | b2-base | 23.033 | +0.00% (0/4) | +1.90% (1/4) | +2.45% (1/4) | 417.5 |
| 8×2 | v153 | template | 22.932 | +0.48% (2/4) | +1.03% (0/4) | +1.89% (0/4) | 391.5 |
| 8×2 | v156 | release | 22.648 | -2.83% (4/4) | -13.58% (4/4) | +0.00% (0/4) | 406.5 |
| 8×2 | v156 | off | 26.685 | +0.80% (2/4) | +0.00% (0/4) | +17.87% (0/4) | 409.0 |
| 8×2 | v156 | b2-base | 23.336 | +0.00% (0/4) | -0.79% (2/4) | +2.93% (0/4) | 407.2 |
| 8×2 | v156 | template | 22.917 | -1.66% (4/4) | -12.66% (3/4) | +1.20% (0/4) | 384.5 |

## Separate phase traces

| Shape | Input | Profile | Non-Boolean exceptions | Classification ms | Point reduction ms | Final MSM ms | Proof ms |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | b2-base | — | 2.799 | 2.029 | 3.520 | 14.699 |
| 4×2 | catalog | template | 378 | 2.398 | 2.288 | 4.053 | 14.713 |
| 4×2 | heldout | b2-base | — | 2.981 | 2.104 | 3.577 | 14.696 |
| 4×2 | heldout | template | 78 | 2.517 | 2.266 | 3.890 | 14.402 |
| 8×2 | catalog | b2-base | — | 5.400 | 4.474 | 4.455 | 22.375 |
| 8×2 | catalog | template | — | 7.004 | 2.540 | 4.148 | 22.554 |
| 8×2 | heldout | b2-base | — | 3.965 | 5.941 | 4.212 | 23.191 |
| 8×2 | heldout | template | 111 | 3.596 | 6.100 | 4.712 | 23.693 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Template census](artifacts/template-census.json) · [Verification](artifacts/verification.json) · [C7a reference timings](../2026-11-19-b2-class-phases-c7a771/README.md)

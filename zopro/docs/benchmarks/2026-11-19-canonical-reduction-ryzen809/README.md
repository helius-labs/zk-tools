# Reduction-only Montgomery decoding for proof scalars

The native field backend ordinarily decodes a Montgomery scalar by multiplying it by the integer one. The experiment uses four word cancellations directly. At each step the low word cancels modulo 2⁶⁴. Its carry into the next word is the high half of the modulus product plus one when the current low word is nonzero. After four rounds the final modulus correction is unchanged. The candidate uses this arithmetic in canonical gather and the in-place MSM conversion. Point arithmetic, field encoding, memory allocations and key format are the same.

Both BN254 moduli passed 65,546 comparisons with the native multiplier and original portable multiplier, including randomized full-width words and boundaries. The Ryzen panel uses 24 main and eight frontend logical CPUs, with five matched process blocks per input. Each process primes three requests and times thirteen. All 136 proofs independently verified. The panel includes one reduced-path and one zero optional-memory proof per shape and input.

None of the conversion modes beat both controls consistently on either shape. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | release | 6.908 | +0.46% (1/5) | +0.00% (0/5) | 58.0 |
| 1×2 | catalog | ordinary | 6.843 | +0.00% (0/5) | -0.45% (4/5) | 57.8 |
| 1×2 | catalog | reduced | 6.874 | +0.16% (2/5) | -0.54% (3/5) | 57.5 |
| 1×2 | heldout | release | 6.813 | +0.09% (2/5) | +0.00% (0/5) | 58.0 |
| 1×2 | heldout | ordinary | 6.813 | +0.00% (0/5) | -0.09% (3/5) | 58.2 |
| 1×2 | heldout | reduced | 6.755 | -0.81% (5/5) | -0.92% (5/5) | 58.2 |
| 2×2 | catalog | release | 9.501 | -0.48% (3/5) | +0.00% (0/5) | 99.2 |
| 2×2 | catalog | ordinary | 9.480 | +0.00% (0/5) | +0.49% (2/5) | 95.8 |
| 2×2 | catalog | reduced | 9.486 | -0.34% (3/5) | -0.16% (3/5) | 93.2 |
| 2×2 | heldout | release | 9.519 | +0.35% (2/5) | +0.00% (0/5) | 94.0 |
| 2×2 | heldout | ordinary | 9.471 | +0.00% (0/5) | -0.34% (3/5) | 94.2 |
| 2×2 | heldout | reduced | 9.510 | -0.10% (4/5) | -0.27% (5/5) | 98.5 |
| 4×2 | catalog | release | 14.432 | +0.32% (2/5) | +0.00% (0/5) | 204.8 |
| 4×2 | catalog | ordinary | 14.525 | +0.00% (0/5) | -0.32% (3/5) | 210.8 |
| 4×2 | catalog | reduced | 14.630 | +0.25% (1/5) | +0.91% (2/5) | 204.2 |
| 4×2 | heldout | release | 14.462 | -0.34% (3/5) | +0.00% (0/5) | 202.0 |
| 4×2 | heldout | ordinary | 14.461 | +0.00% (0/5) | +0.34% (2/5) | 196.5 |
| 4×2 | heldout | reduced | 14.532 | +1.13% (2/5) | +0.34% (1/5) | 206.0 |
| 8×2 | catalog | release | 22.526 | -0.35% (3/5) | +0.00% (0/5) | 397.0 |
| 8×2 | catalog | ordinary | 22.649 | +0.00% (0/5) | +0.35% (2/5) | 399.8 |
| 8×2 | catalog | reduced | 22.451 | -0.41% (4/5) | -0.08% (3/5) | 411.5 |
| 8×2 | heldout | release | 22.654 | -0.48% (4/5) | +0.00% (0/5) | 414.5 |
| 8×2 | heldout | ordinary | 22.726 | +0.00% (0/5) | +0.48% (1/5) | 403.8 |
| 8×2 | heldout | reduced | 22.925 | +0.88% (1/5) | +1.70% (0/5) | 409.0 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

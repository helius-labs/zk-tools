# Fused Euler products when all eight rows agree

The [row-support survey](../2026-11-19-fft-pair-support783/README.md) found that A and B are equal and nonzero in about 60% of the constraint rows. The trial leaves both derivative transforms unchanged. When all eight A values match their corresponding B values, the fused output multiplies their common value by the sum of the derivatives. The ordinary fused operation handles the remaining vectors. The change uses no new buffers.

The native oracle compared the two kernels with equal, unequal, zero and partial vectors. Separate trace checks confirmed that the proofs used the packed Euler output. Complete proofs were measured with 24 main and eight frontend logical threads on Ryzen, three primes and thirteen timed proofs. There are five matched blocks on the primary inputs and four on the additional 8×2 inputs. All 88 exports independently verified.

Neither shape saw consistent gains against both controls across its inputs.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs released SDK | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.396 | -0.49% (3/5) | +0.00% (0/5) | 205.5 |
| 4×2 | catalog | base | 14.468 | +0.00% (0/5) | +0.49% (2/5) | 212.8 |
| 4×2 | catalog | equal | 14.388 | -0.32% (3/5) | -0.06% (3/5) | 218.5 |
| 4×2 | heldout | release | 14.324 | -1.26% (4/5) | +0.00% (0/5) | 201.5 |
| 4×2 | heldout | base | 14.402 | +0.00% (0/5) | +1.28% (1/5) | 202.8 |
| 4×2 | heldout | equal | 14.376 | -0.55% (3/5) | +0.73% (1/5) | 200.5 |
| 8×2 | catalog | release | 22.039 | +0.34% (2/5) | +0.00% (0/5) | 406.8 |
| 8×2 | catalog | base | 22.171 | +0.00% (0/5) | -0.33% (3/5) | 410.8 |
| 8×2 | catalog | equal | 22.186 | -0.18% (3/5) | +0.10% (2/5) | 395.2 |
| 8×2 | heldout | release | 22.369 | +0.57% (2/5) | +0.00% (0/5) | 414.0 |
| 8×2 | heldout | base | 22.466 | +0.00% (0/5) | -0.57% (3/5) | 406.8 |
| 8×2 | heldout | equal | 22.445 | +0.18% (2/5) | +0.29% (2/5) | 414.2 |
| 8×2 | v153 | release | 22.418 | +1.12% (1/4) | +0.00% (0/4) | 418.8 |
| 8×2 | v153 | base | 22.142 | +0.00% (0/4) | -1.10% (3/4) | 405.0 |
| 8×2 | v153 | equal | 22.290 | +0.77% (1/4) | -0.51% (2/4) | 417.0 |
| 8×2 | v156 | release | 22.394 | +0.47% (1/4) | +0.00% (0/4) | 406.0 |
| 8×2 | v156 | base | 22.362 | +0.00% (0/4) | -0.46% (3/4) | 415.0 |
| 8×2 | v156 | equal | 22.408 | -0.10% (2/4) | -0.07% (2/4) | 409.5 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

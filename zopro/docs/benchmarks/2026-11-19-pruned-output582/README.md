# Pruning unneeded outer forward butterflies in verified merge proofs

The candidate checks the residual input pairs in every four-row SIMD pack at the final radix-32 transform. A 32-bit mask per column identifies which output packs can contribute to the pointwise Euler product. In the five outer forward layers it propagates demand backward, skips butterflies with no needed outputs, and computes only the requested sums or differences in single-output butterflies. A zero-output pack retains initialized intermediate field elements. Its subsequent product vanishes because A and B vanish in each lane. All-live columns use the regular radix-32 kernel. The implementation uses bounded per-worker scratch and reads the existing row arrays.

The native oracle checks every necessary derivative pack and the complete pointwise product against the unpruned path, including zero, dense, sparse, padded and unequal input lengths with both root layouts. Separate traced processes measure actual support on the original merge and four withheld public inputs, each independently checked against the earlier zero-row census. Five randomized blocks use uninstrumented complete requests on 32 Ryzen workers.

| Key | Request | Outer butterflies skipped / one output only | Prior / disabled / on ms | Paired vs disabled | Paired vs prior | Current vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| ordinary | Original | 0.23% / 20.18% | 64.257 / 63.573 / 63.104 | -0.82% (3/5) | -0.83% (4/5) | 8.03× / 2.77× |
| mapped | 112 | 15.68% / 29.92% | 67.148 / 66.972 / 66.180 | -1.96% (3/5) | -1.34% (4/5) | — |
| mapped | 124 | 6.71% / 25.62% | 68.944 / 69.011 / 69.155 | +0.39% (2/5) | +0.31% (1/5) | — |
| mapped | 122 | 8.77% / 26.94% | 66.592 / 66.247 / 67.040 | +1.20% (1/5) | +0.60% (2/5) | — |
| mapped | Original | 15.69% / 29.92% | 62.275 / 61.465 / 61.658 | +0.08% (2/5) | -0.88% (4/5) | 8.22× / 2.84× |
| mapped | 109 | 27.04% / 33.89% | 64.219 / 63.837 / 62.968 | -1.36% (5/5) | -1.46% (5/5) | — |

All 96 exports verified with the original key. [Native kernel](artifacts/output_mask.rs) · [Integration](artifacts/integration.patch) · [Process samples and support](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)

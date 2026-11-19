# Fused K/H pool placement on Ryzen

Both trial modes run the existing fused private K and quotient H curve query on the frontend pool when its scratch reservation is admitted. Split keeps G1 A on the main pool. Shared leaves the G1 A placement unchanged, so A and K/H can use the frontend together. G2 B2 stays on the main pool. The ordinary proof path handles denied reservations.

Ryzen uses 24 main and eight frontend logical CPUs. Both shapes use fused K/H, with the existing medium option on 2×2. Five matched blocks per input measure thirteen calls after three primes. All 92 exported proofs independently verified. Separate probes cover both placements and zero optional workspace.

Neither placement mode beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.939 | +0.01% (2/5) | +0.00% (0/5) | 57.5 |
| 1×2 | catalog | ordinary | 6.882 | +0.00% (0/5) | -0.01% (3/5) | 57.5 |
| 1×2 | catalog | split | 8.505 | +22.73% (0/5) | +22.43% (0/5) | 53.2 |
| 1×2 | catalog | shared | 10.147 | +47.45% (0/5) | +48.37% (0/5) | 51.8 |
| 1×2 | heldout | release | 6.767 | +0.74% (2/5) | +0.00% (0/5) | 57.8 |
| 1×2 | heldout | ordinary | 6.736 | +0.00% (0/5) | -0.73% (3/5) | 58.0 |
| 1×2 | heldout | split | 8.220 | +22.43% (0/5) | +21.31% (0/5) | 53.0 |
| 1×2 | heldout | shared | 9.734 | +43.56% (0/5) | +43.92% (0/5) | 51.0 |
| 2×2 | catalog | release | 9.498 | +0.49% (2/5) | +0.00% (0/5) | 93.2 |
| 2×2 | catalog | ordinary | 9.501 | +0.00% (0/5) | -0.48% (3/5) | 95.2 |
| 2×2 | catalog | split | 12.849 | +35.45% (0/5) | +34.70% (0/5) | 89.8 |
| 2×2 | catalog | shared | 14.935 | +57.18% (0/5) | +57.64% (0/5) | 93.2 |
| 2×2 | heldout | release | 9.484 | +0.88% (2/5) | +0.00% (0/5) | 98.8 |
| 2×2 | heldout | ordinary | 9.503 | +0.00% (0/5) | -0.87% (3/5) | 97.2 |
| 2×2 | heldout | split | 12.916 | +35.58% (0/5) | +36.59% (1/5) | 92.5 |
| 2×2 | heldout | shared | 15.117 | +59.28% (0/5) | +57.89% (0/5) | 92.5 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

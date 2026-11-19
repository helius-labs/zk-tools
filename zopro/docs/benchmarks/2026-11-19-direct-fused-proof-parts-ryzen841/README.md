# Direct fused proof assembly on Ryzen

The trial passes the fused solver's projective A, K/H and B2 results directly to Groth16 rerandomization. It saves the intermediate point normalization, byte encoding and decode. The public five-part API and ordinary proof fallback retain their formats. The native exponent oracle checks the rerandomization equations and identity fallback.

Ryzen uses 24 main and eight frontend logical CPUs. Both shapes use fused K/H, with the existing medium option on 2×2. Five matched blocks per input measure thirteen calls after three primes. All 68 exported proofs independently verified. Separate probes check the projective route and zero optional workspace.

Direct assembly did not beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.918 | +0.13% (2/5) | +0.00% (0/5) | 57.5 |
| 1×2 | catalog | ordinary | 7.110 | +0.00% (0/5) | -0.13% (3/5) | 57.2 |
| 1×2 | catalog | direct | 7.047 | +2.01% (2/5) | +1.87% (0/5) | 57.0 |
| 1×2 | heldout | release | 6.771 | -1.83% (3/5) | +0.00% (0/5) | 57.2 |
| 1×2 | heldout | ordinary | 6.895 | +0.00% (0/5) | +1.87% (2/5) | 57.2 |
| 1×2 | heldout | direct | 6.821 | -0.94% (3/5) | +0.41% (0/5) | 61.2 |
| 2×2 | catalog | release | 9.539 | -0.05% (3/5) | +0.00% (0/5) | 106.2 |
| 2×2 | catalog | ordinary | 12.014 | +0.00% (0/5) | +0.05% (2/5) | 100.0 |
| 2×2 | catalog | direct | 13.040 | +0.86% (0/5) | +5.43% (0/5) | 100.5 |
| 2×2 | heldout | release | 9.480 | -0.74% (5/5) | +0.00% (0/5) | 94.0 |
| 2×2 | heldout | ordinary | 11.024 | +0.00% (0/5) | +0.75% (0/5) | 102.0 |
| 2×2 | heldout | direct | 9.527 | -0.87% (4/5) | +0.19% (2/5) | 101.2 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

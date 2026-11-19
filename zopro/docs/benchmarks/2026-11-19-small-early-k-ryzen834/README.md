# Early fused K windows on Ryzen

The trial starts the lowest two, four, or eight private K windows on the main pool while the frontend runs the quotient FFT. The selected windows retain their proof-local bucket state and consume H in the same buckets when its coefficients are ready. The remaining windows use the ordinary fused query. The route charges scratch before overlapping the work. The native generator oracle checks the selected window counts with zero, equal, opposite and identity point inputs.

Ryzen uses 24 main and eight frontend logical CPUs. Both shapes use fused K/H, with the existing medium option on 2×2. Five matched blocks per input measure thirteen calls after three primes. All 112 exported proofs independently verified. Separate probes cover ordinary admission and zero optional workspace.

No early-window mode beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.889 | +0.07% (2/5) | +0.00% (0/5) | 57.2 |
| 1×2 | catalog | ordinary | 6.862 | +0.00% (0/5) | -0.07% (3/5) | 57.5 |
| 1×2 | catalog | early-2 | 7.112 | +3.40% (0/5) | +3.37% (0/5) | 66.2 |
| 1×2 | catalog | early-4 | 7.098 | +3.19% (0/5) | +3.03% (0/5) | 67.2 |
| 1×2 | catalog | early-8 | 7.030 | +2.44% (1/5) | +2.11% (0/5) | 68.5 |
| 1×2 | heldout | release | 6.757 | -0.13% (3/5) | +0.00% (0/5) | 57.5 |
| 1×2 | heldout | ordinary | 6.767 | +0.00% (0/5) | +0.13% (2/5) | 57.2 |
| 1×2 | heldout | early-2 | 6.986 | +3.67% (0/5) | +3.85% (0/5) | 67.0 |
| 1×2 | heldout | early-4 | 7.005 | +4.18% (0/5) | +4.66% (0/5) | 68.5 |
| 1×2 | heldout | early-8 | 7.016 | +3.18% (0/5) | +3.25% (0/5) | 68.2 |
| 2×2 | catalog | release | 9.478 | +0.65% (1/5) | +0.00% (0/5) | 95.0 |
| 2×2 | catalog | ordinary | 9.447 | +0.00% (0/5) | -0.65% (4/5) | 95.5 |
| 2×2 | catalog | early-2 | 9.493 | +0.48% (1/5) | -0.16% (3/5) | 119.0 |
| 2×2 | catalog | early-4 | 9.424 | -0.25% (3/5) | -0.26% (4/5) | 111.0 |
| 2×2 | catalog | early-8 | 9.474 | -0.13% (3/5) | -0.13% (4/5) | 125.8 |
| 2×2 | heldout | release | 9.462 | -0.06% (4/5) | +0.00% (0/5) | 94.8 |
| 2×2 | heldout | ordinary | 9.506 | +0.00% (0/5) | +0.06% (1/5) | 97.2 |
| 2×2 | heldout | early-2 | 9.665 | +1.59% (1/5) | +1.88% (1/5) | 115.0 |
| 2×2 | heldout | early-4 | 9.436 | -0.31% (3/5) | -0.25% (3/5) | 114.0 |
| 2×2 | heldout | early-8 | 9.443 | -0.75% (4/5) | -0.25% (4/5) | 126.2 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

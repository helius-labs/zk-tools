# Window geometry for fused K/H

The 1×2 native prover uses a combined MSM for private K and quotient H. 2×2 uses the same kernel when its existing medium option is enabled. The experimental kernel retains balanced signed digits and varies the number of contiguous windows. Other modes keep twenty windows and process the two input segments using separate bucket buffers. The merge mode combines corresponding buckets before weighting. The separate mode combines the two complete weighted sums. All terms are evaluated for each proof. Extra transient storage is charged to the usual workspace account.

Five matched blocks on Ryzen use 24 main and eight frontend logical CPUs. The release and same-build ordinary controls use the existing twenty shared windows, with the medium option enabled for 2×2 in every trial. Each process primes three calls and measures thirteen. The generator oracle covers the two independent layouts and the edge counts, with signed and zero scalars, identity and opposite points at one and eight workers. All 172 exports independently verified. Separate checks cover budget denial and normal dispatch.

None of the component layouts beat both controls consistently on either shape. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.968 | -5.61% (4/5) | +0.00% (0/5) | 64.8 |
| 1×2 | catalog | ordinary | 7.241 | +0.00% (0/5) | +5.94% (1/5) | 58.2 |
| 1×2 | catalog | 18-shared | 7.185 | -0.99% (3/5) | +4.89% (1/5) | 78.2 |
| 1×2 | catalog | 19-shared | 6.960 | -4.18% (3/5) | -0.12% (3/5) | 64.0 |
| 1×2 | catalog | 21-shared | 6.907 | -7.06% (4/5) | -1.53% (3/5) | 55.5 |
| 1×2 | catalog | 22-shared | 6.918 | -0.48% (3/5) | -0.28% (3/5) | 53.0 |
| 1×2 | catalog | 20-merge | 7.005 | -3.97% (3/5) | +1.74% (1/5) | 63.8 |
| 1×2 | catalog | 20-separate | 7.085 | +1.53% (2/5) | +3.19% (1/5) | 62.8 |
| 1×2 | heldout | release | 6.848 | +1.15% (1/5) | +0.00% (0/5) | 57.5 |
| 1×2 | heldout | ordinary | 6.776 | +0.00% (0/5) | -1.13% (4/5) | 57.5 |
| 1×2 | heldout | 18-shared | 7.097 | +4.53% (0/5) | +3.51% (0/5) | 75.5 |
| 1×2 | heldout | 19-shared | 6.840 | +1.22% (1/5) | +0.52% (2/5) | 65.0 |
| 1×2 | heldout | 21-shared | 6.771 | +0.06% (2/5) | -0.63% (4/5) | 56.2 |
| 1×2 | heldout | 22-shared | 6.891 | +1.15% (1/5) | +0.47% (1/5) | 52.2 |
| 1×2 | heldout | 20-merge | 6.922 | +1.82% (1/5) | +0.89% (0/5) | 64.8 |
| 1×2 | heldout | 20-separate | 6.990 | +3.15% (0/5) | +2.40% (0/5) | 62.0 |
| 2×2 | catalog | release | 9.553 | +0.58% (2/5) | +0.00% (0/5) | 95.0 |
| 2×2 | catalog | ordinary | 9.538 | +0.00% (0/5) | -0.58% (3/5) | 92.5 |
| 2×2 | catalog | 18-shared | 10.041 | +5.80% (0/5) | +5.84% (0/5) | 112.0 |
| 2×2 | catalog | 19-shared | 9.712 | +2.23% (0/5) | +1.67% (0/5) | 98.8 |
| 2×2 | catalog | 21-shared | 9.586 | +0.99% (1/5) | +0.36% (1/5) | 93.2 |
| 2×2 | catalog | 22-shared | 9.756 | +2.29% (0/5) | +2.14% (0/5) | 90.8 |
| 2×2 | catalog | 20-merge | 10.029 | +5.75% (0/5) | +5.13% (0/5) | 96.2 |
| 2×2 | catalog | 20-separate | 10.057 | +7.01% (0/5) | +6.33% (0/5) | 93.8 |
| 2×2 | heldout | release | 9.490 | -0.50% (4/5) | +0.00% (0/5) | 94.0 |
| 2×2 | heldout | ordinary | 9.479 | +0.00% (0/5) | +0.50% (1/5) | 98.5 |
| 2×2 | heldout | 18-shared | 10.074 | +5.82% (1/5) | +6.26% (0/5) | 111.8 |
| 2×2 | heldout | 19-shared | 9.749 | +2.19% (1/5) | +2.49% (0/5) | 98.8 |
| 2×2 | heldout | 21-shared | 9.642 | +1.40% (1/5) | +1.91% (0/5) | 98.0 |
| 2×2 | heldout | 22-shared | 9.745 | +1.82% (1/5) | +3.16% (0/5) | 91.5 |
| 2×2 | heldout | 20-merge | 10.152 | +5.73% (0/5) | +6.53% (0/5) | 98.5 |
| 2×2 | heldout | 20-separate | 10.110 | +5.90% (1/5) | +6.53% (0/5) | 93.5 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

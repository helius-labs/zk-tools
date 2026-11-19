# Restricting 2×2 point pairs to varying wide scalars

The same canonical-word executable used its existing offsets, key and fixed workspace configuration. Eight training fixtures determine groups of at least two members with at least three distinct observed scalar values and at least one 160-bit or wider value. The original all-scalar plan and groups disabled serve as controls. Five randomized blocks each use three primes and 31 timed full proofs. The 32-worker main pool and twelve-worker frontend use the same affinity within each block.

| Request | Off / all-scalar plan / wide B2 only / all wide, ms | All wide vs off | All wide vs all-scalar plan |
|---|---:|---:|---:|
| Original | 9.626 / 10.431 / 9.788 / 9.806 | +0.33% (2/5) | -4.28% (5/5) |
| 41 | 9.766 / 10.422 / 9.768 / 9.686 | +0.07% (2/5) | -4.73% (4/5) |
| 42 | 9.672 / 10.187 / 9.691 / 9.715 | +0.15% (2/5) | -4.02% (3/5) |
| 43 | 9.854 / 10.232 / 9.759 / 9.688 | -0.44% (3/5) | -4.16% (5/5) |
| 44 | 9.845 / 10.255 / 10.298 / 10.157 | +2.94% (1/5) | -1.26% (4/5) |
| 47 | 9.680 / 10.252 / 9.891 / 9.821 | +0.76% (0/5) | -4.62% (5/5) |
| 48 | 9.625 / 10.367 / 9.695 / 9.929 | +3.16% (1/5) | -4.63% (4/5) |

All 143 exported proofs passed independent verification. [Samples](artifacts/processes.json.gz) · [Group sizes](artifacts/layouts.json) · [Routes](artifacts/probes.json) · [Verification](artifacts/verification.json)

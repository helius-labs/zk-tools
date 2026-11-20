# Prepared K and B2 quartet sums on C7a

The prepared B2 and joint routes took **14.917 ms** and **14.914 ms** on the original request. Same-build ordinary took **14.858 ms**. Both prepared routes fell short of the paired requirement across the two inputs. Original input factors for the joint mode are **7.31× M5 CPU** and **3.22× Metal**.

The exact [Ryzen-qualified build](../2026-11-20-joint-prepared-quartets-ryzen936/README.md) checks the full equality-class membership before applying each stored quartet sum. C7a uses 64 main and 32 frontend physical cores. The separate route probes check both reuse counts, zero optional workspace and the invalid K plan with B2 still active.

Each input uses twelve counterbalanced timing blocks. All 108 exports verified on the runner and again after retrieval. The shared c7a.24xlarge terminated after the last panel, its bucket was deleted, and the final check found zero tagged volumes or active benchmark instances.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs B2 | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | frozen | 14.989 | +0.00% (0/12) | +0.49% (5/12) | -0.08% (7/12) | 213.2 |
| catalog | ordinary | 14.858 | -0.49% (7/12) | +0.00% (0/12) | +0.51% (5/12) | 225.8 |
| catalog | b2 | 14.917 | +0.08% (5/12) | -0.50% (7/12) | +0.00% (0/12) | 225.2 |
| catalog | both | 14.914 | +0.31% (5/12) | +0.35% (5/12) | -0.01% (6/12) | 226.8 |
| heldout | frozen | 14.870 | +0.00% (0/12) | -0.89% (10/12) | -0.37% (8/12) | 217.0 |
| heldout | ordinary | 14.930 | +0.89% (2/12) | +0.00% (0/12) | +0.04% (6/12) | 207.9 |
| heldout | b2 | 14.878 | +0.38% (4/12) | -0.04% (6/12) | +0.00% (0/12) | 218.4 |
| heldout | both | 14.794 | +0.22% (4/12) | -0.63% (7/12) | -0.16% (8/12) | 222.9 |



[Timing and proof verification](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup receipt](artifacts/cleanup.json)

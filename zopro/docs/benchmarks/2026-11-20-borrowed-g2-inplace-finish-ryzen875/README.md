# In-place completion of borrowed G2 buckets on Ryzen

The borrowed G2 window previously allocated combined projective points, normalization prefixes, and affine outputs before writing the buckets. The variant combines each projective contribution in its own slot, stores the prefix in the destination X, and normalizes directly into that destination. Each contribution has a unique bucket. Identity and unit Z finish during the first pass. The experiment adds no allocation or key table.

The native test checks zero through 25 contributions with sparse destinations visited in descending order, cancellations, identities, unit Z and general projective inputs against individual affine conversions. The native borrowed G2 suite also passes with the route enabled. All 138 SDK proof exports independently verified.

The worker pools use 24 main and eight frontend Ryzen logical CPUs. The 1×2 and 2×2 trials use the ordinary exclusive G2 query. The 4×2 same-build control and trial enable grouped Phi to exercise this kernel. The 4×2 release uses its ordinary grouped B2 path. Seven paired blocks per input prime three proofs and time thirteen. Separate probes count completion calls and the points sent through normalization. Advancement requires lower paired medians and five wins against each control on both inputs.

In-place completion did not consistently improve both controls across both inputs of any tested shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.852 | -0.68% (6/7) | +0.00% (0/7) | 133.5 | 117 | 58.2 |
| 1×2 | catalog | ordinary | 6.920 | +0.00% (0/7) | +0.68% (1/7) | 133.3 | 121 | 58.0 |
| 1×2 | catalog | inplace | 6.928 | +0.20% (3/7) | +1.28% (1/7) | 134.2 | 129 | 58.5 |
| 1×2 | heldout | release | 6.776 | -0.35% (4/7) | +0.00% (0/7) | 129.5 | 125 | 59.0 |
| 1×2 | heldout | ordinary | 6.774 | +0.00% (0/7) | +0.35% (3/7) | 129.5 | 106 | 58.2 |
| 1×2 | heldout | inplace | 6.805 | +0.14% (3/7) | +0.70% (1/7) | 128.9 | 128 | 58.2 |
| 2×2 | catalog | release | 9.464 | -0.77% (5/7) | +0.00% (0/7) | 181.6 | 282 | 96.0 |
| 2×2 | catalog | ordinary | 9.487 | +0.00% (0/7) | +0.77% (2/7) | 180.9 | 291 | 95.2 |
| 2×2 | catalog | inplace | 9.554 | +0.33% (3/7) | +0.98% (2/7) | 181.6 | 223 | 98.0 |
| 2×2 | heldout | release | 9.493 | -0.45% (6/7) | +0.00% (0/7) | 181.9 | 245 | 98.0 |
| 2×2 | heldout | ordinary | 9.540 | +0.00% (0/7) | +0.45% (1/7) | 181.8 | 261 | 93.8 |
| 2×2 | heldout | inplace | 9.571 | +0.22% (2/7) | +0.96% (1/7) | 181.4 | 281 | 94.2 |
| 4×2 | catalog | release | 13.754 | +3.16% (0/7) | +0.00% (0/7) | 293.2 | 855 | 206.0 |
| 4×2 | catalog | ordinary | 13.336 | +0.00% (0/7) | -3.06% (7/7) | 282.5 | 971 | 220.5 |
| 4×2 | catalog | inplace | 13.281 | -0.53% (5/7) | -3.28% (6/7) | 283.2 | 920 | 215.8 |
| 4×2 | heldout | release | 13.736 | +2.31% (0/7) | +0.00% (0/7) | 293.1 | 950 | 214.0 |
| 4×2 | heldout | ordinary | 13.434 | +0.00% (0/7) | -2.26% (7/7) | 283.6 | 876 | 210.2 |
| 4×2 | heldout | inplace | 13.431 | +0.20% (2/7) | -1.94% (7/7) | 283.8 | 996 | 202.8 |

| Shape | Input | Completion calls per proof | Projective contributions per proof | Nonunit points normalized per proof |
|---|---|---:|---:|---:|
| 1×2 | catalog | 20.0 | 526.0 | 526.0 |
| 1×2 | heldout | 20.0 | 299.0 | 299.0 |
| 2×2 | catalog | 20.0 | 365.0 | 365.0 |
| 2×2 | heldout | 20.0 | 290.0 | 290.0 |
| 4×2 | catalog | 20.0 | 124.0 | 124.0 |
| 4×2 | heldout | 20.0 | 286.0 | 286.0 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

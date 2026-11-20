# Joint indexed leading window for the B2 query on Ryzen

The full-query scalar grouping is shared by all Phi modes. The new leading window counts and scatters both signed components in one pass each, reducing the four component-and-sign planes in a single indexed call. It then merges signs, applies the endomorphism to the high component, and weights the combined leading window once. The remaining windows use two or three balanced fills. The new temporary index array contains at most two entries per input point and is covered by the optional workspace reservation.

The [earlier geometry study](../2026-11-18-g2-geometry297/README.md) tested 11-bit partitions with the original separate leading reductions. Here, `12x3` keeps those separate reductions with three 12-bit fills. `11x4` supplies the four-range geometry at this worker placement. The 4×2 same-build modes enable grouped Phi, and its release uses the ordinary grouped query. The main and frontend pools use 24 and eight Ryzen logical CPUs.

The native oracle checks the combined leading buckets against the existing leading window and compares both complete layouts with a generator sum. All 285 proof exports independently verified, including the zero-allowance fallbacks. Seven paired blocks per input prime three proofs and time thirteen. Advancement requires lower paired medians and five wins against both ordinary and release on both inputs. The three-range combined mode must also clear the separate three-range control.

The combined leading window cleared its matched comparisons on 1×2 12x2-joint, 1×2 12x3-joint.

| Shape | Input | B2 points reaching the paired geometry |
|---|---|---:|
| 1×2 | catalog | 30,089 |
| 1×2 | heldout | 30,089 |
| 2×2 | catalog | 27,218 |
| 2×2 | heldout | 27,267 |
| 4×2 | catalog | 29,738 |
| 4×2 | heldout | 29,961 |

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs 12x3 | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.942 | +0.49% (1/7) | +0.00% (0/7) | -1.31% (6/7) | 134.0 | 112 | 57.2 |
| 1×2 | catalog | ordinary | 6.909 | +0.00% (0/7) | -0.49% (6/7) | -1.49% (7/7) | 134.1 | 101 | 59.0 |
| 1×2 | catalog | 11x4 | 7.069 | +2.09% (0/7) | +1.29% (1/7) | +0.80% (3/7) | 139.3 | 108 | 57.2 |
| 1×2 | catalog | 12x3 | 7.013 | +1.51% (0/7) | +1.32% (1/7) | +0.00% (0/7) | 137.9 | 168 | 61.8 |
| 1×2 | catalog | 12x2-joint | 6.780 | -1.69% (7/7) | -2.33% (7/7) | -3.56% (7/7) | 132.2 | 123 | 57.2 |
| 1×2 | catalog | 12x3-joint | 6.717 | -2.81% (7/7) | -3.18% (7/7) | -4.16% (7/7) | 135.4 | 146 | 62.0 |
| 1×2 | heldout | release | 6.797 | -0.52% (4/7) | +0.00% (0/7) | -2.72% (6/7) | 129.7 | 117 | 57.8 |
| 1×2 | heldout | ordinary | 6.799 | +0.00% (0/7) | +0.52% (3/7) | -2.21% (6/7) | 129.4 | 105 | 58.0 |
| 1×2 | heldout | 11x4 | 6.969 | +2.29% (1/7) | +3.02% (1/7) | +0.19% (3/7) | 134.5 | 125 | 57.0 |
| 1×2 | heldout | 12x3 | 6.978 | +2.26% (1/7) | +2.80% (1/7) | +0.00% (0/7) | 133.8 | 172 | 62.5 |
| 1×2 | heldout | 12x2-joint | 6.757 | -1.01% (7/7) | -0.76% (7/7) | -3.23% (7/7) | 127.8 | 115 | 57.8 |
| 1×2 | heldout | 12x3-joint | 6.651 | -2.73% (7/7) | -2.22% (7/7) | -4.69% (7/7) | 131.4 | 168 | 61.0 |
| 2×2 | catalog | release | 9.534 | +0.14% (3/7) | +0.00% (0/7) | -2.16% (6/7) | 182.2 | 286 | 99.5 |
| 2×2 | catalog | ordinary | 9.520 | +0.00% (0/7) | -0.14% (4/7) | -2.27% (7/7) | 182.1 | 255 | 93.5 |
| 2×2 | catalog | 11x4 | 9.753 | +2.33% (0/7) | +2.04% (1/7) | +0.02% (3/7) | 186.6 | 231 | 93.0 |
| 2×2 | catalog | 12x3 | 9.750 | +2.32% (0/7) | +2.21% (1/7) | +0.00% (0/7) | 186.3 | 256 | 99.8 |
| 2×2 | catalog | 12x2-joint | 9.503 | -0.05% (4/7) | -0.20% (6/7) | -2.44% (7/7) | 181.1 | 232 | 95.2 |
| 2×2 | catalog | 12x3-joint | 9.585 | +1.30% (1/7) | +0.62% (2/7) | -1.27% (6/7) | 183.9 | 302 | 98.0 |
| 2×2 | heldout | release | 9.528 | -0.75% (6/7) | +0.00% (0/7) | -3.23% (7/7) | 182.6 | 211 | 96.0 |
| 2×2 | heldout | ordinary | 9.583 | +0.00% (0/7) | +0.76% (1/7) | -2.17% (7/7) | 181.9 | 265 | 94.0 |
| 2×2 | heldout | 11x4 | 9.721 | +1.15% (0/7) | +2.05% (0/7) | -0.92% (6/7) | 186.5 | 237 | 94.8 |
| 2×2 | heldout | 12x3 | 9.805 | +2.22% (0/7) | +3.34% (0/7) | +0.00% (0/7) | 186.1 | 321 | 98.5 |
| 2×2 | heldout | 12x2-joint | 9.530 | +0.23% (3/7) | -0.12% (4/7) | -2.70% (6/7) | 181.6 | 255 | 94.5 |
| 2×2 | heldout | 12x3-joint | 9.685 | +1.27% (1/7) | +1.84% (1/7) | -0.99% (6/7) | 184.6 | 243 | 97.8 |
| 4×2 | catalog | release | 13.700 | +2.56% (0/7) | +0.00% (0/7) | +0.30% (3/7) | 293.5 | 935 | 195.5 |
| 4×2 | catalog | ordinary | 13.430 | +0.00% (0/7) | -2.49% (7/7) | -2.69% (6/7) | 285.6 | 953 | 209.8 |
| 4×2 | catalog | 11x4 | 13.727 | +2.52% (1/7) | -0.88% (4/7) | -1.68% (6/7) | 289.3 | 926 | 196.0 |
| 4×2 | catalog | 12x3 | 13.841 | +2.77% (1/7) | -0.29% (4/7) | +0.00% (0/7) | 289.7 | 936 | 202.0 |
| 4×2 | catalog | 12x2-joint | 13.339 | -0.51% (5/7) | -3.47% (6/7) | -3.01% (6/7) | 283.5 | 854 | 206.2 |
| 4×2 | catalog | 12x3-joint | 13.822 | +2.03% (1/7) | +1.05% (3/7) | -0.53% (5/7) | 286.9 | 878 | 212.2 |
| 4×2 | heldout | release | 13.729 | +2.45% (0/7) | +0.00% (0/7) | -0.83% (5/7) | 293.0 | 971 | 197.2 |
| 4×2 | heldout | ordinary | 13.423 | +0.00% (0/7) | -2.40% (7/7) | -2.90% (7/7) | 283.8 | 918 | 195.2 |
| 4×2 | heldout | 11x4 | 13.754 | +3.21% (0/7) | +0.44% (3/7) | +0.42% (3/7) | 290.1 | 894 | 202.0 |
| 4×2 | heldout | 12x3 | 13.845 | +2.99% (0/7) | +0.83% (2/7) | +0.00% (0/7) | 288.9 | 943 | 199.8 |
| 4×2 | heldout | 12x2-joint | 13.391 | -0.24% (4/7) | -2.39% (7/7) | -2.97% (7/7) | 283.5 | 887 | 203.8 |
| 4×2 | heldout | 12x3-joint | 13.873 | +3.47% (0/7) | +1.26% (1/7) | +0.18% (1/7) | 288.4 | 940 | 206.8 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-solver-b2-prefix-parts-c7a885/README.md#eight-supplied-m5-reference-timings)

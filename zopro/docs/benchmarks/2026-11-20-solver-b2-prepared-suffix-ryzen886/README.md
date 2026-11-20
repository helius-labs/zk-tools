# Staging B2 coefficients ahead of the window fills on Ryzen

The [C7a split trial](../2026-11-20-solver-b2-prefix-parts-c7a885/README.md) spent roughly 3.1 ms in the staged suffix bucket fill. This variant ends the staged job after scalar grouping and signed decomposition. When grouping is available it retains only the proof-local group sums and short coefficients. Without grouping the fill borrows the key points. The group reservation stays live through the fill. Both prefix parts and the suffix then fill their windows during the usual curve phase, followed by one weighting of their merged buckets.

The same-build and pinned early split controls stage the full suffix buckets. The two preparation arms use the same arithmetic and differ in when the frontend submits the preparation job. Zopro keeps its public-wire offsets and uses the same 24 main and eight frontend Ryzen logical CPUs. Route probes check the prepared payload and the zero-workspace fallback.

The native oracle checks grouped and ungrouped inputs, including zero and repeated scalars and identity bases. All 264 exports independently verified. Eight paired blocks on each primary input and four on each alternate prime three proofs and time thirteen. Advancement requires lower paired medians and six or three wins against release, ordinary, early bucket staging and late preparation.

Early suffix preparation failed to beat all four controls consistently on either shape.

| Shape | Completed solver bands at readiness | Borrowed suffix points |
|---|---:|---:|
| 1×2 | 2 | 8,238 |
| 2×2 | 2 | 9,416 |

| Shape | Input | Mode | Suffix job ms | Suffix started before FFT by ms | Suffix and FFT overlap ms |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | split-early | 1.292 | 0.413 | 0.860 |
| 1×2 | catalog | prep-late | 0.037 | 0.107 | 0.000 |
| 1×2 | catalog | prep-early | 0.035 | 0.346 | 0.000 |
| 1×2 | heldout | split-early | 1.233 | 0.399 | 0.827 |
| 1×2 | heldout | prep-late | 0.033 | 0.103 | 0.000 |
| 1×2 | heldout | prep-early | 0.035 | 0.336 | 0.000 |
| 2×2 | catalog | split-early | 1.663 | 0.443 | 1.224 |
| 2×2 | catalog | prep-late | 0.265 | 0.123 | 0.143 |
| 2×2 | catalog | prep-early | 0.253 | 0.429 | 0.000 |
| 2×2 | heldout | split-early | 1.737 | 0.458 | 1.279 |
| 2×2 | heldout | prep-late | 0.280 | 0.148 | 0.115 |
| 2×2 | heldout | prep-early | 0.260 | 0.429 | 0.000 |

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs pinned | Paired vs early buckets | Paired vs prep late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.938 | +0.50% (3/8) | +0.00% (0/8) | -0.59% (5/8) | -0.60% (5/8) | -5.48% (8/8) | 133.9 | 102 | 57.0 |
| 1×2 | catalog | ordinary | 6.938 | +0.00% (0/8) | -0.49% (5/8) | -0.79% (6/8) | -0.22% (6/8) | -5.37% (8/8) | 134.5 | 106 | 58.0 |
| 1×2 | catalog | pinned | 6.962 | +0.79% (2/8) | +0.59% (3/8) | +0.00% (0/8) | +0.23% (3/8) | -4.87% (8/8) | 140.4 | 154 | 63.0 |
| 1×2 | catalog | split-early | 6.950 | +0.22% (2/8) | +0.60% (3/8) | -0.23% (5/8) | +0.00% (0/8) | -5.02% (8/8) | 141.2 | 146 | 62.0 |
| 1×2 | catalog | prep-late | 7.345 | +5.68% (0/8) | +5.80% (0/8) | +5.12% (0/8) | +5.28% (0/8) | +0.00% (0/8) | 139.9 | 139 | 61.5 |
| 1×2 | catalog | prep-early | 7.300 | +5.72% (0/8) | +5.06% (0/8) | +4.85% (0/8) | +5.06% (0/8) | -0.31% (6/8) | 140.5 | 141 | 61.5 |
| 1×2 | heldout | release | 6.804 | -0.11% (5/8) | +0.00% (0/8) | -0.49% (5/8) | -0.52% (5/8) | -6.22% (8/8) | 129.0 | 124 | 57.5 |
| 1×2 | heldout | ordinary | 6.818 | +0.00% (0/8) | +0.11% (3/8) | -0.73% (6/8) | -0.83% (5/8) | -5.94% (8/8) | 130.5 | 134 | 57.8 |
| 1×2 | heldout | pinned | 6.833 | +0.74% (2/8) | +0.49% (3/8) | +0.00% (0/8) | -0.54% (6/8) | -5.99% (8/8) | 135.5 | 164 | 62.5 |
| 1×2 | heldout | split-early | 6.855 | +0.84% (3/8) | +0.52% (3/8) | +0.54% (2/8) | +0.00% (0/8) | -5.15% (8/8) | 135.9 | 157 | 63.0 |
| 1×2 | heldout | prep-late | 7.271 | +6.32% (0/8) | +6.63% (0/8) | +6.38% (0/8) | +5.43% (0/8) | +0.00% (0/8) | 136.0 | 148 | 61.2 |
| 1×2 | heldout | prep-early | 7.252 | +7.13% (0/8) | +6.61% (0/8) | +5.91% (0/8) | +5.24% (0/8) | +0.11% (3/8) | 136.1 | 159 | 62.0 |
| 1×2 | v27 | release | 6.827 | +0.36% (1/4) | +0.00% (0/4) | +0.01% (2/4) | -0.96% (3/4) | -5.30% (4/4) | 129.7 | 100 | 58.0 |
| 1×2 | v27 | ordinary | 6.811 | +0.00% (0/4) | -0.36% (3/4) | +0.03% (2/4) | -1.34% (3/4) | -5.41% (4/4) | 130.2 | 135 | 58.0 |
| 1×2 | v27 | pinned | 6.829 | -0.03% (2/4) | -0.01% (2/4) | +0.00% (0/4) | -1.20% (3/4) | -5.22% (4/4) | 135.6 | 171 | 62.2 |
| 1×2 | v27 | split-early | 6.893 | +1.36% (1/4) | +0.97% (1/4) | +1.21% (1/4) | +0.00% (0/4) | -4.79% (4/4) | 136.5 | 166 | 61.8 |
| 1×2 | v27 | prep-late | 7.206 | +5.72% (0/4) | +5.60% (0/4) | +5.51% (0/4) | +5.03% (0/4) | +0.00% (0/4) | 136.4 | 166 | 61.5 |
| 1×2 | v27 | prep-early | 7.189 | +5.67% (0/4) | +5.39% (0/4) | +5.49% (0/4) | +4.29% (0/4) | -0.02% (2/4) | 136.4 | 168 | 62.5 |
| 1×2 | v28 | release | 6.780 | +0.20% (2/4) | +0.00% (0/4) | -1.48% (3/4) | -1.66% (3/4) | -5.97% (4/4) | 128.8 | 116 | 57.0 |
| 1×2 | v28 | ordinary | 6.777 | +0.00% (0/4) | -0.20% (2/4) | -0.91% (3/4) | -0.88% (4/4) | -5.67% (4/4) | 129.4 | 129 | 57.2 |
| 1×2 | v28 | pinned | 6.861 | +0.91% (1/4) | +1.50% (1/4) | +0.00% (0/4) | -0.55% (3/4) | -4.81% (4/4) | 135.5 | 168 | 62.0 |
| 1×2 | v28 | split-early | 6.848 | +0.88% (0/4) | +1.69% (1/4) | +0.55% (1/4) | +0.00% (0/4) | -4.34% (4/4) | 135.5 | 174 | 61.8 |
| 1×2 | v28 | prep-late | 7.204 | +6.01% (0/4) | +6.36% (0/4) | +5.05% (0/4) | +4.54% (0/4) | +0.00% (0/4) | 135.2 | 157 | 61.2 |
| 1×2 | v28 | prep-early | 7.179 | +6.10% (0/4) | +5.89% (0/4) | +5.49% (0/4) | +5.42% (1/4) | +0.16% (2/4) | 135.1 | 170 | 62.0 |
| 2×2 | catalog | release | 9.547 | -0.04% (4/8) | +0.00% (0/8) | +2.05% (0/8) | +1.26% (0/8) | -3.07% (8/8) | 182.1 | 236 | 97.8 |
| 2×2 | catalog | ordinary | 9.520 | +0.00% (0/8) | +0.04% (4/8) | +2.00% (0/8) | +1.43% (0/8) | -3.55% (8/8) | 183.0 | 241 | 94.8 |
| 2×2 | catalog | pinned | 9.359 | -1.96% (8/8) | -2.00% (8/8) | +0.00% (0/8) | -0.53% (5/8) | -5.44% (8/8) | 190.0 | 263 | 95.2 |
| 2×2 | catalog | split-early | 9.405 | -1.41% (8/8) | -1.25% (8/8) | +0.53% (3/8) | +0.00% (0/8) | -4.57% (8/8) | 191.1 | 226 | 95.5 |
| 2×2 | catalog | prep-late | 9.870 | +3.68% (0/8) | +3.17% (0/8) | +5.76% (0/8) | +4.79% (0/8) | +0.00% (0/8) | 189.0 | 232 | 95.0 |
| 2×2 | catalog | prep-early | 9.788 | +2.88% (0/8) | +2.57% (1/8) | +4.61% (0/8) | +4.07% (0/8) | -0.72% (8/8) | 188.8 | 299 | 96.5 |
| 2×2 | heldout | release | 9.540 | +0.34% (1/8) | +0.00% (0/8) | +1.92% (1/8) | +1.46% (0/8) | -3.53% (8/8) | 182.3 | 285 | 94.2 |
| 2×2 | heldout | ordinary | 9.510 | +0.00% (0/8) | -0.33% (7/8) | +1.62% (1/8) | +1.41% (2/8) | -3.86% (8/8) | 182.8 | 253 | 96.0 |
| 2×2 | heldout | pinned | 9.352 | -1.60% (7/8) | -1.88% (7/8) | +0.00% (0/8) | -0.44% (6/8) | -5.35% (8/8) | 190.7 | 234 | 96.2 |
| 2×2 | heldout | split-early | 9.414 | -1.39% (6/8) | -1.44% (8/8) | +0.44% (2/8) | +0.00% (0/8) | -4.78% (8/8) | 191.3 | 266 | 97.0 |
| 2×2 | heldout | prep-late | 9.886 | +4.01% (0/8) | +3.66% (0/8) | +5.65% (0/8) | +5.02% (0/8) | +0.00% (0/8) | 189.4 | 280 | 96.5 |
| 2×2 | heldout | prep-early | 9.820 | +2.79% (0/8) | +3.00% (0/8) | +5.26% (0/8) | +4.22% (0/8) | -0.60% (5/8) | 189.3 | 254 | 94.8 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-solver-b2-prefix-parts-c7a885/README.md#eight-supplied-m5-reference-timings)

# Parallel prefix fills after B2 suffix staging on Ryzen

The [main-pool trial](../2026-11-20-solver-b2-main-scope-ryzen883/README.md) formed one prefix bucket set per window after staging the suffix. The new route groups the prefix once and splits it into two balanced fills within each window. It merges both fills with the staged suffix and weights the merged buckets once. An optional reservation accounts for the extra fill. The pinned earlier executable and same-build `main-early` use one prefix fill. The main-pool suffix and frontend retain their placement.

The native suffix oracle checks both geometries against the complete generator sum, with grouping enabled and disabled. Route probes check the full prefix and suffix coverage, pool widths, public-wire offsets and the ordinary path at zero optional workspace. All 264 proof exports independently verified.

Ryzen uses 24 main and eight frontend logical CPUs. Each paired block primes three proofs and times thirteen. The original and held-out inputs have eight blocks, and the other inputs have four. Advancement requires a lower paired median and at least six or three wins against release, ordinary, main-early and split-late for the corresponding inputs.

The split early route did not consistently improve all four required controls on either shape.

| Shape | Completed solver bands at readiness | Borrowed suffix points |
|---|---:|---:|
| 1×2 | 2 | 8,238 |
| 2×2 | 2 | 9,416 |

| Shape | Input | Mode | Suffix started before FFT by ms | Suffix and FFT overlap ms |
|---|---|---|---:|---:|
| 1×2 | catalog | main-early | 0.412 | 0.816 |
| 1×2 | catalog | split-late | 0.105 | 0.736 |
| 1×2 | catalog | split-early | 0.389 | 0.761 |
| 1×2 | heldout | main-early | 0.386 | 0.717 |
| 1×2 | heldout | split-late | 0.103 | 0.670 |
| 1×2 | heldout | split-early | 0.389 | 0.719 |
| 2×2 | catalog | main-early | 0.449 | 1.259 |
| 2×2 | catalog | split-late | 0.153 | 1.347 |
| 2×2 | catalog | split-early | 0.433 | 1.143 |
| 2×2 | heldout | main-early | 0.438 | 1.250 |
| 2×2 | heldout | split-late | 0.154 | 1.622 |
| 2×2 | heldout | split-early | 0.466 | 1.236 |

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs pinned | Paired vs main-early | Paired vs split-late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.934 | +0.21% (4/8) | +0.00% (0/8) | -6.12% (8/8) | -7.42% (8/8) | -3.65% (8/8) | 134.7 | 119 | 58.0 |
| 1×2 | catalog | ordinary | 6.931 | +0.00% (0/8) | -0.21% (4/8) | -6.99% (8/8) | -7.69% (8/8) | -4.47% (8/8) | 134.0 | 111 | 58.5 |
| 1×2 | catalog | pinned | 7.451 | +7.52% (0/8) | +6.52% (0/8) | +0.00% (0/8) | -0.78% (5/8) | +2.54% (1/8) | 136.1 | 157 | 60.8 |
| 1×2 | catalog | main-early | 7.489 | +8.33% (0/8) | +8.02% (0/8) | +0.78% (3/8) | +0.00% (0/8) | +3.52% (1/8) | 135.7 | 148 | 60.2 |
| 1×2 | catalog | split-late | 7.225 | +4.68% (0/8) | +3.79% (0/8) | -2.48% (7/8) | -3.40% (7/8) | +0.00% (0/8) | 140.6 | 132 | 61.8 |
| 1×2 | catalog | split-early | 6.933 | +0.06% (4/8) | -0.28% (5/8) | -6.39% (8/8) | -7.34% (8/8) | -4.08% (8/8) | 139.9 | 142 | 62.2 |
| 1×2 | heldout | release | 6.810 | +0.69% (2/8) | +0.00% (0/8) | -5.78% (8/8) | -6.89% (8/8) | -3.82% (8/8) | 129.7 | 125 | 57.8 |
| 1×2 | heldout | ordinary | 6.738 | +0.00% (0/8) | -0.69% (6/8) | -7.85% (8/8) | -7.53% (8/8) | -5.29% (8/8) | 129.2 | 125 | 57.5 |
| 1×2 | heldout | pinned | 7.270 | +8.51% (0/8) | +6.14% (0/8) | +0.00% (0/8) | -0.53% (4/8) | +2.82% (2/8) | 132.1 | 145 | 60.0 |
| 1×2 | heldout | main-early | 7.299 | +8.15% (0/8) | +7.40% (0/8) | +0.54% (4/8) | +0.00% (0/8) | +2.26% (0/8) | 132.3 | 140 | 59.0 |
| 1×2 | heldout | split-late | 7.103 | +5.59% (0/8) | +3.97% (0/8) | -2.74% (6/8) | -2.21% (8/8) | +0.00% (0/8) | 135.4 | 153 | 61.5 |
| 1×2 | heldout | split-early | 6.839 | +1.31% (1/8) | +0.80% (1/8) | -6.32% (8/8) | -6.71% (8/8) | -3.68% (8/8) | 136.0 | 159 | 62.5 |
| 1×2 | v27 | release | 6.784 | +0.04% (2/4) | +0.00% (0/4) | -6.61% (4/4) | -7.62% (4/4) | -5.74% (4/4) | 129.3 | 136 | 57.2 |
| 1×2 | v27 | ordinary | 6.763 | +0.00% (0/4) | -0.03% (2/4) | -6.80% (4/4) | -8.33% (4/4) | -5.88% (4/4) | 129.1 | 99 | 58.8 |
| 1×2 | v27 | pinned | 7.239 | +7.29% (0/4) | +7.08% (0/4) | +0.00% (0/4) | -1.07% (4/4) | +0.07% (2/4) | 132.9 | 158 | 58.5 |
| 1×2 | v27 | main-early | 7.368 | +9.09% (0/4) | +8.25% (0/4) | +1.09% (0/4) | +0.00% (0/4) | +1.98% (1/4) | 131.4 | 133 | 58.8 |
| 1×2 | v27 | split-late | 7.185 | +6.25% (0/4) | +6.10% (0/4) | -0.06% (2/4) | -1.94% (3/4) | +0.00% (0/4) | 136.6 | 138 | 62.0 |
| 1×2 | v27 | split-early | 6.833 | +0.51% (1/4) | +0.47% (1/4) | -5.72% (4/4) | -6.75% (4/4) | -4.91% (4/4) | 135.4 | 158 | 62.0 |
| 1×2 | v28 | release | 6.813 | +0.29% (2/4) | +0.00% (0/4) | -5.63% (4/4) | -6.32% (4/4) | -3.34% (4/4) | 128.7 | 137 | 57.5 |
| 1×2 | v28 | ordinary | 6.763 | +0.00% (0/4) | -0.29% (2/4) | -6.27% (4/4) | -5.99% (4/4) | -4.20% (4/4) | 127.9 | 111 | 55.8 |
| 1×2 | v28 | pinned | 7.234 | +6.69% (0/4) | +5.96% (0/4) | +0.00% (0/4) | -0.09% (2/4) | +2.73% (0/4) | 130.2 | 136 | 59.8 |
| 1×2 | v28 | main-early | 7.251 | +6.37% (0/4) | +6.75% (0/4) | +0.10% (2/4) | +0.00% (0/4) | +2.53% (0/4) | 132.3 | 146 | 58.0 |
| 1×2 | v28 | split-late | 7.056 | +4.39% (0/4) | +3.46% (0/4) | -2.66% (4/4) | -2.45% (4/4) | +0.00% (0/4) | 135.2 | 151 | 61.5 |
| 1×2 | v28 | split-early | 6.793 | -0.19% (3/4) | -0.02% (2/4) | -6.49% (4/4) | -6.16% (4/4) | -3.92% (4/4) | 135.1 | 156 | 62.0 |
| 2×2 | catalog | release | 9.528 | +0.47% (2/8) | +0.00% (0/8) | -7.28% (8/8) | -7.51% (8/8) | +0.80% (2/8) | 181.6 | 264 | 93.8 |
| 2×2 | catalog | ordinary | 9.474 | +0.00% (0/8) | -0.47% (6/8) | -7.64% (8/8) | -7.97% (8/8) | +0.37% (3/8) | 181.6 | 280 | 95.8 |
| 2×2 | catalog | pinned | 10.214 | +8.28% (0/8) | +7.85% (0/8) | +0.00% (0/8) | -0.57% (5/8) | +8.64% (1/8) | 185.9 | 279 | 95.5 |
| 2×2 | catalog | main-early | 10.267 | +8.67% (0/8) | +8.12% (0/8) | +0.57% (3/8) | +0.00% (0/8) | +9.10% (1/8) | 186.1 | 250 | 94.2 |
| 2×2 | catalog | split-late | 9.436 | -0.37% (5/8) | -0.79% (6/8) | -7.95% (7/8) | -8.34% (7/8) | +0.00% (0/8) | 191.1 | 257 | 97.5 |
| 2×2 | catalog | split-early | 9.384 | -1.04% (6/8) | -1.21% (8/8) | -8.12% (8/8) | -8.93% (8/8) | -0.95% (5/8) | 189.9 | 253 | 97.5 |
| 2×2 | heldout | release | 9.553 | +0.73% (3/8) | +0.00% (0/8) | -7.01% (8/8) | -7.73% (8/8) | +0.87% (2/8) | 182.2 | 307 | 97.0 |
| 2×2 | heldout | ordinary | 9.499 | +0.00% (0/8) | -0.73% (5/8) | -7.75% (8/8) | -7.92% (8/8) | +0.50% (3/8) | 181.9 | 203 | 99.0 |
| 2×2 | heldout | pinned | 10.284 | +8.41% (0/8) | +7.53% (0/8) | +0.00% (0/8) | -0.58% (5/8) | +9.08% (0/8) | 187.2 | 241 | 91.2 |
| 2×2 | heldout | main-early | 10.317 | +8.61% (0/8) | +8.38% (0/8) | +0.59% (3/8) | +0.00% (0/8) | +9.25% (0/8) | 186.5 | 265 | 97.0 |
| 2×2 | heldout | split-late | 9.436 | -0.50% (5/8) | -0.87% (6/8) | -8.32% (8/8) | -8.47% (8/8) | +0.00% (0/8) | 190.8 | 254 | 96.0 |
| 2×2 | heldout | split-early | 9.374 | -1.25% (7/8) | -1.40% (8/8) | -8.47% (8/8) | -9.14% (8/8) | -0.22% (5/8) | 190.0 | 262 | 97.0 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

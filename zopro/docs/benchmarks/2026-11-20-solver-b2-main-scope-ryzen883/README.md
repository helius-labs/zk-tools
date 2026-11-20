# Staging the B2 suffix on the main worker pool on Ryzen

The [shared-pool trial](../2026-11-20-solver-b2-window-stage-ryzen882/README.md) formed suffix buckets in the solver pool. This experiment sends an owned copy of the ready canonical coefficients to the existing main pool. The main worker job borrows the original suffix points. The frontend continues with the remaining solver bands, gather and FFT. Its work and the suffix job join before the curve phase. The usual public-wire offsets apply to both ranges.

The early arm submits the job after its wires become available. The late arm submits it after solving and still overlaps the remaining frontend work. Both use the same split, grouping, bucket merge and weighting. The previous shared-pool executable supplies a pinned comparison. Separate probes record the pool width and elapsed intervals of the suffix fill and FFT.

The Ryzen worker pools use 24 main and eight frontend logical CPUs. The native suffix oracle and solver tests pass. All 220 proof exports independently verified, including the usual proof route at zero optional workspace. Matched blocks prime three proofs and time thirteen. Advancement requires lower paired medians and at least six wins out of eight or three out of four against release, ordinary and main-late on each input.

Early main-pool staging did not consistently improve all three required controls on either shape.

| Shape | Completed solver bands at readiness | Borrowed suffix points |
|---|---:|---:|
| 1×2 | 2 | 8,238 |
| 2×2 | 2 | 9,416 |

| Shape | Input | Mode | Suffix started before FFT by ms | Suffix and FFT overlap ms |
|---|---|---|---:|---:|
| 1×2 | catalog | main-late | 0.114 | 0.833 |
| 1×2 | catalog | main-early | 0.397 | 0.783 |
| 1×2 | heldout | main-late | 0.103 | 0.753 |
| 1×2 | heldout | main-early | 0.391 | 0.830 |
| 2×2 | catalog | main-late | 0.150 | 1.671 |
| 2×2 | catalog | main-early | 0.461 | 1.156 |
| 2×2 | heldout | main-late | 0.152 | 1.555 |
| 2×2 | heldout | main-early | 0.489 | 1.052 |

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs shared | Paired vs main-late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.873 | -1.03% (6/8) | +0.00% (0/8) | -20.72% (8/8) | -11.76% (8/8) | 134.0 | 120 | 57.8 |
| 1×2 | catalog | ordinary | 6.948 | +0.00% (0/8) | +1.04% (2/8) | -20.22% (8/8) | -10.79% (8/8) | 134.9 | 107 | 58.5 |
| 1×2 | catalog | shared | 8.739 | +25.35% (0/8) | +26.15% (0/8) | +0.00% (0/8) | +12.03% (0/8) | 130.8 | 156 | 59.2 |
| 1×2 | catalog | main-late | 7.812 | +12.09% (0/8) | +13.33% (0/8) | -10.74% (8/8) | +0.00% (0/8) | 136.6 | 128 | 59.2 |
| 1×2 | catalog | main-early | 7.469 | +7.03% (0/8) | +8.03% (0/8) | -14.61% (8/8) | -4.34% (8/8) | 136.8 | 149 | 60.0 |
| 1×2 | heldout | release | 6.812 | -1.41% (6/8) | +0.00% (0/8) | -19.65% (8/8) | -10.81% (8/8) | 129.3 | 129 | 58.0 |
| 1×2 | heldout | ordinary | 6.865 | +0.00% (0/8) | +1.43% (2/8) | -19.26% (8/8) | -9.48% (8/8) | 130.7 | 127 | 58.8 |
| 1×2 | heldout | shared | 8.460 | +23.85% (0/8) | +24.47% (0/8) | +0.00% (0/8) | +11.81% (0/8) | 128.8 | 142 | 57.8 |
| 1×2 | heldout | main-late | 7.624 | +10.49% (0/8) | +12.13% (0/8) | -10.55% (8/8) | +0.00% (0/8) | 133.5 | 140 | 58.5 |
| 1×2 | heldout | main-early | 7.384 | +7.03% (0/8) | +8.64% (0/8) | -13.44% (8/8) | -3.01% (8/8) | 133.0 | 151 | 59.2 |
| 1×2 | v27 | release | 6.765 | -0.50% (3/4) | +0.00% (0/4) | -20.27% (4/4) | -11.80% (4/4) | 129.2 | 112 | 58.0 |
| 1×2 | v27 | ordinary | 6.803 | +0.00% (0/4) | +0.50% (1/4) | -20.33% (4/4) | -11.28% (4/4) | 130.4 | 111 | 56.8 |
| 1×2 | v27 | shared | 8.540 | +25.53% (0/4) | +25.43% (0/4) | +0.00% (0/4) | +11.69% (0/4) | 127.4 | 151 | 59.2 |
| 1×2 | v27 | main-late | 7.670 | +12.73% (0/4) | +13.39% (0/4) | -10.41% (4/4) | +0.00% (0/4) | 133.6 | 134 | 59.2 |
| 1×2 | v27 | main-early | 7.199 | +6.01% (0/4) | +5.82% (0/4) | -16.28% (4/4) | -4.98% (4/4) | 133.3 | 151 | 59.0 |
| 1×2 | v28 | release | 6.772 | -0.90% (3/4) | +0.00% (0/4) | -20.33% (4/4) | -11.21% (4/4) | 128.7 | 117 | 57.2 |
| 1×2 | v28 | ordinary | 6.847 | +0.00% (0/4) | +0.91% (1/4) | -19.61% (4/4) | -10.40% (4/4) | 130.1 | 144 | 57.2 |
| 1×2 | v28 | shared | 8.500 | +24.39% (0/4) | +25.53% (0/4) | +0.00% (0/4) | +11.46% (0/4) | 126.3 | 145 | 58.5 |
| 1×2 | v28 | main-late | 7.639 | +11.61% (0/4) | +12.62% (0/4) | -10.28% (4/4) | +0.00% (0/4) | 132.6 | 137 | 58.5 |
| 1×2 | v28 | main-early | 7.310 | +5.98% (0/4) | +6.15% (0/4) | -13.55% (4/4) | -3.93% (4/4) | 132.2 | 125 | 58.0 |
| 2×2 | catalog | release | 9.512 | -0.76% (7/8) | +0.00% (0/8) | -19.44% (8/8) | -8.68% (8/8) | 181.7 | 271 | 94.8 |
| 2×2 | catalog | ordinary | 9.560 | +0.00% (0/8) | +0.77% (1/8) | -19.31% (8/8) | -7.98% (8/8) | 183.2 | 247 | 93.5 |
| 2×2 | catalog | shared | 11.816 | +23.93% (0/8) | +24.14% (0/8) | +0.00% (0/8) | +13.57% (0/8) | 180.8 | 229 | 90.2 |
| 2×2 | catalog | main-late | 10.390 | +8.67% (0/8) | +9.51% (0/8) | -11.95% (8/8) | +0.00% (0/8) | 188.3 | 231 | 90.8 |
| 2×2 | catalog | main-early | 10.322 | +8.41% (0/8) | +8.62% (0/8) | -12.71% (8/8) | -0.67% (5/8) | 187.8 | 217 | 94.2 |
| 2×2 | heldout | release | 9.501 | -0.59% (7/8) | +0.00% (0/8) | -20.54% (8/8) | -8.93% (8/8) | 181.6 | 204 | 97.2 |
| 2×2 | heldout | ordinary | 9.544 | +0.00% (0/8) | +0.60% (1/8) | -19.72% (8/8) | -8.39% (8/8) | 183.8 | 267 | 95.0 |
| 2×2 | heldout | shared | 11.899 | +24.57% (0/8) | +25.85% (0/8) | +0.00% (0/8) | +14.01% (0/8) | 181.1 | 278 | 94.5 |
| 2×2 | heldout | main-late | 10.423 | +9.16% (0/8) | +9.80% (0/8) | -12.28% (8/8) | +0.00% (0/8) | 189.0 | 277 | 96.8 |
| 2×2 | heldout | main-early | 10.331 | +8.24% (0/8) | +8.84% (0/8) | -13.54% (8/8) | -0.84% (7/8) | 188.5 | 204 | 93.2 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

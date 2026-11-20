# Radix-4 FFT input staging across solver bands on Ryzen

The [dependency scan](../2026-11-20-solver-fft-readiness878/README.md) identified groups of complete columns at solver band boundaries. This trial uses the actual prepared schedule to give each 16-column tile a unique band and verifies that every real input row has one writer. The early arm runs the two inverse radix-4 layers for those tiles alongside the following solver band. The late arm runs the same tiled work after solving. Both arms finish the transform and the quotient products with the same kernel. The ordinary and release arms use the existing radix-32 first layer.

Tiles read only rows whose writers have joined. Each FFT tile owns disjoint positions in the ordinary packed workspace, which is allocated earlier. Setup retains only the per-band tile indices under an optional workspace reservation. Another reservation covers the packed workspace and bounded expanded FFT leaves. At zero optional preparation allowance the full proving path falls back to the existing solver and FFT.

The eight frontend Ryzen logical CPUs share the staged work and the four-band seeded solver. The remaining FFT uses the ordinary placement. The main pool uses 24 logical CPUs. Each block primes three proofs and measures thirteen. The original and held-out requests have eight paired blocks, and the other two requests have four each. Advancement requires six or three wins and a lower paired median against all three controls for each respective input.

Early radix-4 staging did not consistently improve all three controls across all four inputs.

The original FFT oracle covers partial and full row prefixes. The native FFT and solver tests pass. The packed solver completes (89, 124, 131, 168) tiles in the four bands, and its unpacked probe completes (141, 116, 115, 140). Each sequence totals 512 tiles per proof. All 112 exports independently verified, including the zero-allowance runs.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 13.633 | +0.17% (3/8) | +0.00% (0/8) | -0.64% (6/8) | 292.1 | 990 | 208.5 |
| catalog | ordinary | 13.618 | +0.00% (0/8) | -0.17% (5/8) | -0.42% (6/8) | 292.8 | 966 | 202.5 |
| catalog | late | 13.803 | +0.42% (2/8) | +0.65% (2/8) | +0.00% (0/8) | 294.1 | 984 | 205.5 |
| catalog | early | 13.633 | -0.41% (6/8) | -0.21% (5/8) | -1.12% (7/8) | 291.9 | 1004 | 203.5 |
| heldout | release | 13.620 | +0.41% (3/8) | +0.00% (0/8) | -0.55% (5/8) | 291.9 | 1026 | 214.0 |
| heldout | ordinary | 13.653 | +0.00% (0/8) | -0.41% (5/8) | -0.30% (4/8) | 292.2 | 929 | 197.2 |
| heldout | late | 13.689 | +0.30% (4/8) | +0.56% (3/8) | +0.00% (0/8) | 293.5 | 980 | 202.8 |
| heldout | early | 13.627 | -0.24% (5/8) | -0.55% (5/8) | -0.40% (6/8) | 291.9 | 858 | 202.2 |
| v153 | release | 13.527 | -0.22% (2/4) | +0.00% (0/4) | -0.45% (3/4) | 291.5 | 1023 | 199.2 |
| v153 | ordinary | 13.573 | +0.00% (0/4) | +0.23% (2/4) | -0.03% (2/4) | 291.5 | 1099 | 208.0 |
| v153 | late | 13.588 | +0.04% (2/4) | +0.45% (1/4) | +0.00% (0/4) | 292.7 | 987 | 201.2 |
| v153 | early | 13.574 | -0.22% (3/4) | +0.19% (1/4) | -0.01% (2/4) | 292.1 | 950 | 193.0 |
| v156 | release | 13.474 | -0.48% (4/4) | +0.00% (0/4) | -1.38% (4/4) | 290.5 | 890 | 206.5 |
| v156 | ordinary | 13.508 | +0.00% (0/4) | +0.48% (0/4) | -0.49% (2/4) | 291.0 | 866 | 208.0 |
| v156 | late | 13.701 | +0.50% (2/4) | +1.40% (0/4) | +0.00% (0/4) | 292.2 | 997 | 198.5 |
| v156 | early | 13.466 | -0.52% (3/4) | -0.04% (2/4) | -1.49% (4/4) | 289.4 | 923 | 193.2 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

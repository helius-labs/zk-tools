# Resuming the radix-32 FFT after solver staging on Ryzen

The [first staging trial](../2026-11-20-solver-radix4-stage-ryzen879/README.md) reorganized the rest of the FFT around radix-4 stripes. This trial retains the ordinary radix-32 leaves and fused forward product. Early bands run the first two inverse layers in proof-local workspace alongside the following solver band. The remaining three inverse layers then load those partial columns in their original stripe order. Tiles needing the final solver band enter from A and B directly at that point and complete all five layers with one packed store. The late arm performs the same staged operations after solving. The pinned first trial is included as `pilot`.

The native FFT test covers four combinations of staged and directly loaded tiles on five partial or full row prefixes. All outputs match the existing Euler kernel. The rest of the native FFT and solver tests pass. The packed solver stages 344 tiles in the first three bands with counts (89, 124, 131). The unpacked probe stages 372 with counts (141, 116, 115). At zero optional workspace the prover takes its usual route. All 137 exports independently verified.

The Ryzen pools use 24 main and eight frontend logical CPUs. Timed arms prime three proofs and measure thirteen per block. The original and held-out inputs have eight blocks each. The other inputs have four. Advancement requires a lower paired median against release, ordinary and late on each input with at least six wins out of eight or three out of four.

Early staging did not consistently improve all three required controls across all four inputs.

The same-build ordinary control was 4.70–6.27% slower than release in the paired comparisons. Early versus late estimates the placement effect within the trial binary. The comparison against release combines the placement effect with the shift in the ordinary control.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs pilot | Paired vs late | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 13.565 | -5.90% (8/8) | +0.00% (0/8) | -2.16% (8/8) | -5.91% (8/8) | 292.1 | 920 | 201.8 |
| catalog | ordinary | 14.400 | +0.00% (0/8) | +6.27% (0/8) | +3.34% (0/8) | +0.30% (3/8) | 308.5 | 914 | 199.5 |
| catalog | pilot | 13.903 | -3.23% (8/8) | +2.20% (0/8) | +0.00% (0/8) | -3.49% (8/8) | 298.3 | 1035 | 202.8 |
| catalog | late | 14.383 | -0.30% (5/8) | +6.28% (0/8) | +3.61% (0/8) | +0.00% (0/8) | 309.7 | 886 | 198.2 |
| catalog | early | 14.305 | -0.99% (6/8) | +5.60% (0/8) | +2.94% (0/8) | -0.22% (4/8) | 308.1 | 949 | 198.5 |
| heldout | release | 13.608 | -4.49% (8/8) | +0.00% (0/8) | -1.32% (6/8) | -5.00% (8/8) | 292.2 | 948 | 202.5 |
| heldout | ordinary | 14.288 | +0.00% (0/8) | +4.70% (0/8) | +3.31% (0/8) | -0.68% (5/8) | 306.7 | 902 | 211.0 |
| heldout | pilot | 13.864 | -3.21% (8/8) | +1.34% (2/8) | +0.00% (0/8) | -3.74% (8/8) | 296.8 | 940 | 199.2 |
| heldout | late | 14.344 | +0.68% (3/8) | +5.26% (0/8) | +3.88% (0/8) | +0.00% (0/8) | 307.8 | 878 | 192.2 |
| heldout | early | 14.271 | +0.18% (3/8) | +4.60% (0/8) | +3.47% (0/8) | -1.09% (6/8) | 308.2 | 836 | 198.2 |
| v153 | release | 13.524 | -5.22% (4/4) | +0.00% (0/4) | -2.65% (4/4) | -5.48% (4/4) | 292.2 | 952 | 195.2 |
| v153 | ordinary | 14.250 | +0.00% (0/4) | +5.51% (0/4) | +2.87% (0/4) | +0.28% (1/4) | 307.1 | 950 | 200.0 |
| v153 | pilot | 13.893 | -2.79% (4/4) | +2.72% (0/4) | +0.00% (0/4) | -2.90% (4/4) | 296.5 | 980 | 193.8 |
| v153 | late | 14.292 | -0.28% (3/4) | +5.80% (0/4) | +2.98% (0/4) | +0.00% (0/4) | 308.1 | 1023 | 209.8 |
| v153 | early | 14.211 | -0.48% (3/4) | +4.82% (0/4) | +2.38% (0/4) | -0.47% (3/4) | 305.7 | 866 | 197.8 |
| v156 | release | 13.617 | -5.64% (4/4) | +0.00% (0/4) | -1.73% (4/4) | -5.49% (4/4) | 293.3 | 964 | 199.0 |
| v156 | ordinary | 14.405 | +0.00% (0/4) | +5.98% (0/4) | +4.06% (0/4) | -0.07% (2/4) | 307.3 | 869 | 203.2 |
| v156 | pilot | 13.896 | -3.90% (4/4) | +1.77% (0/4) | +0.00% (0/4) | -3.65% (4/4) | 297.9 | 782 | 189.2 |
| v156 | late | 14.402 | +0.08% (2/4) | +5.81% (0/4) | +3.79% (0/4) | +0.00% (0/4) | 308.9 | 888 | 194.8 |
| v156 | early | 14.392 | -0.03% (2/4) | +5.75% (0/4) | +3.60% (0/4) | -0.18% (3/4) | 308.1 | 1015 | 203.5 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-small-b2-prep-overlap-c7a872/README.md#eight-supplied-m5-reference-timings)

# Destination-owned ordinary B2 buckets on Ryzen

The existing B2 kernel partitions points and merges partial buckets from the point ranges. The trial assigns each ordinary digit bucket to one worker. Each window builds a packed index for both signed components. Within each 256-bucket tile the worker feeds one point per bucket in each layer, applies the same endomorphism between components and locally weights the final buckets. The window folds their weighted sums with tile offsets. The leading window uses the existing point partitions. An ordinary window with an unusually concentrated histogram also uses point partitions. The proof-local indices and worker scratch are charged to the optional workspace.

The 11-bit owner uses the same four-range fallback as the `point11` control. The 12-bit owner matches ordinary two-range geometry. The released and same-build controls use 24 main and eight frontend Ryzen logical CPUs. On 4×2, the same-build profiles use grouped Phi and the released arm uses the ordinary grouped G2 query.

Native checks compare complete MSMs with a generator sum on one and four workers, with identity and exceptional additions included. All 237 proof exports independently verified, including zero optional-workspace fallbacks. Seven paired blocks per input prime three proofs and time thirteen. A mode advances with lower paired medians and five wins against its controls on both inputs.

Neither ownership mode consistently improved its matched controls on any tested shape.

| Shape | Input | B2 points | Owned ordinary windows, 11-bit | Owned ordinary windows, 12-bit |
|---|---|---:|---:|---:|
| 1×2 | catalog | 30,089 | 10/11 | 9/10 |
| 1×2 | heldout | 30,089 | 10/11 | 9/10 |
| 2×2 | catalog | 27,218 | 11/11 | 10/10 |
| 2×2 | heldout | 27,267 | 11/11 | 10/10 |
| 4×2 | catalog | 29,738 | 11/11 | 10/10 |
| 4×2 | heldout | 29,961 | 11/11 | 10/10 |

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs point11 | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.886 | -0.38% (5/7) | +0.00% (0/7) | -4.06% (7/7) | 133.4 | 116 | 58.2 |
| 1×2 | catalog | ordinary | 6.911 | +0.00% (0/7) | +0.38% (2/7) | -2.65% (7/7) | 133.3 | 110 | 58.0 |
| 1×2 | catalog | point11 | 7.124 | +2.72% (0/7) | +4.23% (0/7) | +0.00% (0/7) | 138.6 | 133 | 57.0 |
| 1×2 | catalog | owner11 | 7.192 | +3.49% (0/7) | +4.52% (0/7) | +0.59% (2/7) | 138.7 | 113 | 55.2 |
| 1×2 | catalog | owner12 | 6.991 | +1.42% (2/7) | +1.40% (0/7) | -1.87% (7/7) | 139.9 | 129 | 56.2 |
| 1×2 | heldout | release | 6.763 | +0.04% (3/7) | +0.00% (0/7) | -3.07% (7/7) | 129.2 | 105 | 58.0 |
| 1×2 | heldout | ordinary | 6.772 | +0.00% (0/7) | -0.04% (4/7) | -3.17% (7/7) | 129.4 | 125 | 57.5 |
| 1×2 | heldout | point11 | 6.991 | +3.27% (0/7) | +3.17% (0/7) | +0.00% (0/7) | 133.5 | 132 | 56.8 |
| 1×2 | heldout | owner11 | 7.093 | +4.75% (0/7) | +4.40% (0/7) | +1.16% (0/7) | 134.2 | 123 | 55.5 |
| 1×2 | heldout | owner12 | 6.922 | +2.60% (0/7) | +2.07% (0/7) | -0.65% (6/7) | 135.3 | 139 | 56.0 |
| 2×2 | catalog | release | 9.530 | -0.42% (4/7) | +0.00% (0/7) | -1.98% (7/7) | 181.5 | 258 | 94.8 |
| 2×2 | catalog | ordinary | 9.524 | +0.00% (0/7) | +0.42% (3/7) | -2.55% (7/7) | 182.0 | 243 | 93.8 |
| 2×2 | catalog | point11 | 9.755 | +2.62% (0/7) | +2.02% (0/7) | +0.00% (0/7) | 186.3 | 237 | 96.2 |
| 2×2 | catalog | owner11 | 9.955 | +4.31% (0/7) | +4.65% (0/7) | +1.93% (0/7) | 188.3 | 295 | 92.0 |
| 2×2 | catalog | owner12 | 9.806 | +2.91% (0/7) | +3.34% (0/7) | +0.52% (0/7) | 190.0 | 236 | 94.5 |
| 2×2 | heldout | release | 9.542 | -0.00% (4/7) | +0.00% (0/7) | -2.97% (7/7) | 182.6 | 277 | 96.5 |
| 2×2 | heldout | ordinary | 9.508 | +0.00% (0/7) | +0.00% (3/7) | -2.97% (7/7) | 181.6 | 221 | 94.5 |
| 2×2 | heldout | point11 | 9.801 | +3.06% (0/7) | +3.06% (0/7) | +0.00% (0/7) | 186.3 | 276 | 93.8 |
| 2×2 | heldout | owner11 | 9.932 | +4.45% (0/7) | +4.56% (0/7) | +1.40% (0/7) | 188.3 | 243 | 92.5 |
| 2×2 | heldout | owner12 | 9.841 | +3.40% (0/7) | +3.09% (0/7) | +0.02% (3/7) | 189.9 | 209 | 93.0 |
| 4×2 | catalog | release | 13.700 | +3.68% (0/7) | +0.00% (0/7) | +0.75% (2/7) | 294.2 | 893 | 204.2 |
| 4×2 | catalog | ordinary | 13.315 | +0.00% (0/7) | -3.55% (7/7) | -3.06% (7/7) | 283.7 | 893 | 194.2 |
| 4×2 | catalog | point11 | 13.718 | +3.16% (0/7) | -0.75% (5/7) | +0.00% (0/7) | 288.9 | 960 | 197.8 |
| 4×2 | catalog | owner11 | 14.094 | +5.85% (0/7) | +2.22% (1/7) | +2.61% (0/7) | 292.1 | 979 | 196.5 |
| 4×2 | catalog | owner12 | 13.767 | +3.53% (0/7) | +0.58% (3/7) | +0.84% (3/7) | 291.4 | 871 | 192.0 |
| 4×2 | heldout | release | 13.748 | +2.91% (0/7) | +0.00% (0/7) | +0.67% (2/7) | 293.3 | 960 | 205.2 |
| 4×2 | heldout | ordinary | 13.336 | +0.00% (0/7) | -2.82% (7/7) | -2.63% (7/7) | 283.9 | 1020 | 202.0 |
| 4×2 | heldout | point11 | 13.659 | +2.70% (0/7) | -0.67% (5/7) | +0.00% (0/7) | 288.9 | 976 | 207.5 |
| 4×2 | heldout | owner11 | 14.123 | +6.28% (0/7) | +2.83% (0/7) | +2.71% (0/7) | 293.4 | 1030 | 201.8 |
| 4×2 | heldout | owner12 | 13.883 | +4.13% (0/7) | +0.88% (1/7) | +1.38% (1/7) | 292.8 | 984 | 204.0 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-20-b2-joint-top-c7a889/README.md#eight-supplied-m5-reference-timings)

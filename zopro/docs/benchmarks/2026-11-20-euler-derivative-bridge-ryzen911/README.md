# Fusing the boundary between sequential Euler derivatives on Ryzen

The [single-workspace quotient](../2026-11-20-euler-sequential-ryzen910/README.md) computes D(B), stores A·D(B) in the output, and reuses the derivative buffer for D(A). Its outer transform originally assigned up to 32 columns to each task. The `adaptive` mode chooses a power-of-two task count from the active worker pool, bounded by the number of columns and the original maximum tile size. It changes only the column split. The `bridge` mode uses the same task policy and combines the two outer passes at the derivative boundary.

Each fused task loads one column from the unfinished D(B) buffer, completes the forward butterflies, and emits A·D(B) while keeping the original A values in the column tile. Those values immediately start the inverse butterflies for D(A). This removes two full writes of the compact workspace, one full read, and the read of its active prefix. The workspace remains N/8 compact packs. A and B still use the same scalar field, roots, row order, output and verification as the earlier route.

Native checks compare all three schedules with independently evaluated polynomials on eight domain sizes and both root orientations. Another check compares the fused boundary with the unchanged path for unequal nonzero prefixes under one, five and 24 workers. The route logs confirm the active pool and actual tile width for all tested profiles. All 312 exported proofs independently verified.

On 1×2, the complete quotient took 0.688 / 0.703 ms with fixed tasks, 0.520 / 0.556 ms with adaptive tasks and 0.507 / 0.503 ms with the fused boundary. The fused boundary changed full-proof latency by -1.04% / -0.23% against adaptive and -0.85% / -0.46% against ordinary. Both modes fell short of the paired requirement.
On 2×2, the complete quotient took 1.219 / 1.212 ms with fixed tasks, 1.170 / 1.191 ms with adaptive tasks and 1.159 / 1.146 ms with the fused boundary. The fused boundary changed full-proof latency by +0.09% / -0.44% against adaptive and -0.25% / -0.35% against ordinary. Both modes fell short of the paired requirement.
On 4×2, the complete quotient took 2.480 / 2.464 ms with fixed tasks, 2.446 / 2.379 ms with adaptive tasks and 2.228 / 2.218 ms with the fused boundary. The fused boundary changed full-proof latency by -1.58% / -1.13% against adaptive and -0.49% / +0.40% against ordinary. Both modes fell short of the paired requirement.

The proof panel runs on 24 main and eight frontend Ryzen logical CPUs. The ordinary arm uses the original quotient on 1×2 and 4×2 and the qualified shift quotient on 2×2. The frozen prior and same-build fixed arms use the earlier single-workspace route. Advancement needs lower paired medians and six wins against both ordinary and prior on both inputs. Eight paired blocks per shape and input prime three proofs and time nineteen. Peak RSS covers the entire process. The separate clocks measure complete Euler quotient calls in two processes with seven timed requests each. The main latency runs have no observer clocks or tracing.

| Shape | Input | Mode | Full proof ms | Paired vs prior | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | prior | 7.048 | +0.00% (0/8) | +2.17% (0/8) | 56.8 |
| 1x2 | catalog | ordinary | 6.889 | -2.12% (8/8) | +0.00% (0/8) | 58.0 |
| 1x2 | catalog | fixed | 7.046 | +0.19% (3/8) | +1.89% (0/8) | 56.8 |
| 1x2 | catalog | adaptive | 6.925 | -1.79% (6/8) | +0.27% (4/8) | 56.8 |
| 1x2 | catalog | bridge | 6.831 | -3.28% (8/8) | -0.85% (7/8) | 57.0 |
| 1x2 | heldout | prior | 6.955 | +0.00% (0/8) | +2.64% (0/8) | 56.8 |
| 1x2 | heldout | ordinary | 6.780 | -2.58% (8/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | fixed | 6.946 | +0.24% (3/8) | +2.37% (0/8) | 56.8 |
| 1x2 | heldout | adaptive | 6.771 | -2.47% (8/8) | +0.14% (3/8) | 57.0 |
| 1x2 | heldout | bridge | 6.750 | -2.74% (8/8) | -0.46% (5/8) | 57.2 |
| 2x2 | catalog | prior | 9.374 | +0.00% (0/8) | -0.01% (4/8) | 91.8 |
| 2x2 | catalog | ordinary | 9.375 | +0.01% (4/8) | +0.00% (0/8) | 98.5 |
| 2x2 | catalog | fixed | 9.388 | +0.03% (4/8) | +0.04% (4/8) | 97.5 |
| 2x2 | catalog | adaptive | 9.359 | -0.08% (4/8) | -0.33% (6/8) | 92.0 |
| 2x2 | catalog | bridge | 9.341 | -0.22% (5/8) | -0.25% (7/8) | 91.2 |
| 2x2 | heldout | prior | 9.386 | +0.00% (0/8) | -0.19% (5/8) | 91.5 |
| 2x2 | heldout | ordinary | 9.391 | +0.19% (3/8) | +0.00% (0/8) | 95.0 |
| 2x2 | heldout | fixed | 9.387 | -0.14% (5/8) | -0.08% (4/8) | 91.5 |
| 2x2 | heldout | adaptive | 9.373 | -0.14% (6/8) | -0.13% (5/8) | 95.5 |
| 2x2 | heldout | bridge | 9.361 | -0.22% (4/8) | -0.35% (7/8) | 91.8 |
| 4x2 | catalog | prior | 13.918 | +0.00% (0/8) | +1.83% (0/8) | 215.2 |
| 4x2 | catalog | ordinary | 13.644 | -1.80% (8/8) | +0.00% (0/8) | 217.8 |
| 4x2 | catalog | fixed | 13.929 | +0.04% (4/8) | +1.88% (0/8) | 218.0 |
| 4x2 | catalog | adaptive | 13.865 | -0.87% (6/8) | +1.57% (2/8) | 224.0 |
| 4x2 | catalog | bridge | 13.604 | -1.88% (8/8) | -0.49% (5/8) | 209.8 |
| 4x2 | heldout | prior | 13.861 | +0.00% (0/8) | +1.86% (0/8) | 211.2 |
| 4x2 | heldout | ordinary | 13.616 | -1.83% (8/8) | +0.00% (0/8) | 211.8 |
| 4x2 | heldout | fixed | 13.913 | +0.31% (3/8) | +2.58% (0/8) | 205.5 |
| 4x2 | heldout | adaptive | 13.756 | -0.67% (7/8) | +1.36% (0/8) | 211.0 |
| 4x2 | heldout | bridge | 13.643 | -1.42% (8/8) | +0.40% (2/8) | 207.5 |

| Shape | Input | Mode | Quotient FFT ms |
|---|---|---|---:|
| 1x2 | catalog | ordinary | 0.545 |
| 1x2 | catalog | fixed | 0.688 |
| 1x2 | catalog | adaptive | 0.520 |
| 1x2 | catalog | bridge | 0.507 |
| 1x2 | heldout | ordinary | 0.536 |
| 1x2 | heldout | fixed | 0.703 |
| 1x2 | heldout | adaptive | 0.556 |
| 1x2 | heldout | bridge | 0.503 |
| 2x2 | catalog | ordinary | 1.192 |
| 2x2 | catalog | fixed | 1.219 |
| 2x2 | catalog | adaptive | 1.170 |
| 2x2 | catalog | bridge | 1.159 |
| 2x2 | heldout | ordinary | 1.203 |
| 2x2 | heldout | fixed | 1.212 |
| 2x2 | heldout | adaptive | 1.191 |
| 2x2 | heldout | bridge | 1.146 |
| 4x2 | catalog | ordinary | 2.238 |
| 4x2 | catalog | fixed | 2.480 |
| 4x2 | catalog | adaptive | 2.446 |
| 4x2 | catalog | bridge | 2.228 |
| 4x2 | heldout | ordinary | 2.204 |
| 4x2 | heldout | fixed | 2.464 |
| 4x2 | heldout | adaptive | 2.379 |
| 4x2 | heldout | bridge | 2.218 |

| Shape | Input | Same-build fixed vs frozen prior | Adaptive vs fixed | Bridge vs adaptive |
|---|---|---:|---:|---:|
| 1x2 | catalog | +0.19% (3/8) | -1.48% (6/8) | -1.04% (7/8) |
| 1x2 | heldout | +0.24% (3/8) | -2.54% (8/8) | -0.23% (5/8) |
| 2x2 | catalog | +0.03% (4/8) | -0.19% (5/8) | +0.09% (3/8) |
| 2x2 | heldout | -0.14% (5/8) | -0.06% (5/8) | -0.44% (5/8) |
| 4x2 | catalog | +0.04% (4/8) | -0.14% (4/8) | -1.58% (8/8) |
| 4x2 | heldout | +0.31% (3/8) | -1.01% (7/8) | -1.13% (8/8) |

| Shape | Input | Mode | FFT workers | Columns per task | Outer tasks |
|---|---|---|---:|---:|---:|
| 1x2 | catalog | fixed | 8 | 32 | 4 |
| 1x2 | catalog | adaptive | 8 | 16 | 8 |
| 1x2 | catalog | bridge | 8 | 16 | 8 |
| 1x2 | heldout | fixed | 8 | 32 | 4 |
| 1x2 | heldout | adaptive | 8 | 16 | 8 |
| 1x2 | heldout | bridge | 8 | 16 | 8 |
| 2x2 | catalog | fixed | 8 | 32 | 8 |
| 2x2 | catalog | adaptive | 8 | 32 | 8 |
| 2x2 | catalog | bridge | 8 | 32 | 8 |
| 2x2 | heldout | fixed | 8 | 32 | 8 |
| 2x2 | heldout | adaptive | 8 | 32 | 8 |
| 2x2 | heldout | bridge | 8 | 32 | 8 |
| 4x2 | catalog | fixed | 24 | 32 | 16 |
| 4x2 | catalog | adaptive | 24 | 16 | 32 |
| 4x2 | catalog | bridge | 24 | 16 | 32 |
| 4x2 | heldout | fixed | 24 | 32 | 16 |
| 4x2 | heldout | adaptive | 24 | 16 | 32 |
| 4x2 | heldout | bridge | 24 | 16 | 32 |

[Source](artifacts/source.patch) · [Samples and traces](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 references

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md) still reaches 10× M5 CPU on **5/8** original requests. AWS authentication was expired before this experiment.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

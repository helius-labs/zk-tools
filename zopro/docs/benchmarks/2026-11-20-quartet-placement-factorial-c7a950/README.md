# Direct quartet compaction and solver placement on C7a 4×2

Direct compaction and the 16-core solver each cleared both controls on both inputs. The joint trial fell short of the qualification requirement. Joint proof latency was **14.493 / 14.610 ms** on the original and held-out inputs. The original-input target is **10.9 ms**.

The experiment combines the [direct quartet checks](../2026-11-20-quartet-origin-runtime-ryzen948/README.md) with the [16-core witness interpreter](../2026-11-20-four-by-two-prep-placement-c7a946/README.md) in one executable. The four main arms independently turn those two changes on and off. The FFT and curve pools use the same 64+32 placement. The byte-exact source builds and frozen SDK supply three controls.

Each input ran twelve counterbalanced blocks. Each process primed three proofs and timed seventeen. The table reports medians of per-process means, paired changes, and maximum whole-process RSS.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs frozen | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | frozen | 14.952 | -0.41% (8/12) | +0.00% (0/12) | 222.2 |
| catalog | ordinary | 15.040 | +0.00% (0/12) | +0.41% (4/12) | 216.2 |
| catalog | narrow | 14.723 | -1.83% (12/12) | -2.33% (11/12) | 213.9 |
| catalog | direct | 14.760 | -2.17% (9/12) | -1.17% (11/12) | 226.1 |
| catalog | joint | 14.493 | -4.13% (12/12) | -3.24% (12/12) | 219.9 |
| catalog | prior-narrow | 14.702 | -2.46% (12/12) | -2.12% (11/12) | 208.7 |
| catalog | prior-direct | 14.799 | -1.58% (12/12) | -0.81% (10/12) | 214.7 |
| heldout | frozen | 15.096 | -0.73% (7/12) | +0.00% (0/12) | 207.9 |
| heldout | ordinary | 15.096 | +0.00% (0/12) | +0.74% (5/12) | 207.9 |
| heldout | narrow | 14.732 | -2.91% (11/12) | -2.20% (11/12) | 217.6 |
| heldout | direct | 14.991 | -1.47% (9/12) | -0.75% (9/12) | 221.4 |
| heldout | joint | 14.610 | -3.45% (12/12) | -2.80% (12/12) | 217.0 |
| heldout | prior-narrow | 14.766 | -2.38% (12/12) | -1.93% (11/12) | 216.3 |
| heldout | prior-direct | 14.884 | -1.53% (10/12) | -1.12% (9/12) | 215.4 |

| Input | Ordinary vs frozen | Narrow vs prior solver | Direct vs prior compaction | Joint vs narrow | Joint vs direct | Joint vs prior solver | Interaction ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | +0.41% (4/12) | -0.08% (6/12) | -0.33% (7/12) | -0.88% (9/12) | -1.77% (8/12) | -0.82% (8/12) | +0.201 |
| heldout | +0.74% (5/12) | -0.63% (9/12) | +0.01% (6/12) | -0.30% (6/12) | -1.86% (10/12) | -0.85% (9/12) | +0.123 |

The joint mode beat frozen in every block on both inputs. Its extra saving fell short of the requirement against the best isolated arms. The positive interaction means the combined saving was smaller than the sum of the isolated savings. The interaction is calculated within each block as joint latency minus both isolated latencies plus ordinary latency. The SDK source stays experimental.

## Separate stage clocks

| Input | Mode | Solve ms | Gather ms | FFT ms | Remaining proof ms | Clock effect | Within ±3% |
|---|---|---:|---:|---:|---:|---:|---|
| catalog | ordinary | 3.040 | 1.041 | 2.048 | 8.977 | +0.00% (1/3) | yes |
| catalog | narrow | 2.728 | 0.984 | 2.037 | 9.217 | +1.71% (1/3) | yes |
| catalog | direct | 3.028 | 1.011 | 2.045 | 8.814 | +0.28% (0/3) | yes |
| catalog | joint | 2.645 | 1.047 | 2.042 | 8.758 | +1.37% (0/3) | yes |
| heldout | ordinary | 3.046 | 1.033 | 2.029 | 9.112 | -0.86% (2/3) | yes |
| heldout | narrow | 2.568 | 1.000 | 2.052 | 8.912 | -1.26% (3/3) | yes |
| heldout | direct | 2.936 | 1.043 | 2.067 | 8.983 | +0.35% (1/3) | yes |
| heldout | joint | 2.604 | 1.024 | 2.069 | 8.700 | -0.90% (2/3) | yes |

The remaining proof time includes the curve queries, verification, and other work outside the measured stages. All six native tests passed. The twelve local route checks verified solver and FFT widths, direct quartet use, zero optional workspace, and invalid K metadata. All 204 exported proofs independently verified. The temporary C7a instance and its storage were removed.

## Eight supplied M5 references

Five of eight shapes reach 10× against M5 CPU. The other seven shapes use their [previously verified C7a measurements](../2026-11-20-instruction-pool-sdk-c7a949/README.md#eight-supplied-m5-references).

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.457 | 109 / 48 | 7.54× / 3.32× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz) · [Cleanup](artifacts/cleanup.json)

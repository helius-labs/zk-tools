# Shard-local K scalar classification output on C7a 4×2

Shard-local output shortened the full K hash interval by 0.26–0.39 ms against the matching atomic layout. Most of the saving came from the final class-order scan. Both widths fell short of the full-proof requirement across the two inputs.

The runner used the [exact Ryzen executable and route checks](../2026-11-20-k-shard-output-ryzen966/README.md), with 64 main and 32 frontend physical cores on c7a.24xlarge. The solver uses 16 frontend cores and K uses all 32. Both layouts preserve exact class order, charge about 3.53 MiB of optional scratch, and keep all other query routes and pools unchanged.

| Input | Shards | Partition | Setup | Parallel hashing | Scatter | Remap | Full hash interval |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 8 | +0.021 | +0.029 | −0.090 | +0.056 | −0.349 | −0.287 |
| catalog | 32 | +0.026 | +0.019 | −0.157 | +0.088 | −0.410 | −0.394 |
| heldout | 8 | +0.009 | +0.014 | −0.062 | +0.053 | −0.271 | −0.260 |
| heldout | 32 | +0.029 | +0.025 | −0.141 | +0.077 | −0.392 | −0.388 |

The table reports local minus atomic paired medians in milliseconds from six blocks of thirteen timed proofs per input. Individual phase medians need not sum to the median change for the full interval. The timing suggests that output layout affects both the parallel pass and the caller's later scan. The benchmark does not measure cache-line transfers directly.

| Input | Shards | Atomic ms | Local ms | Paired vs atomic | Paired vs ordinary | Paired vs frozen | Local peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 8 | 14.679 | 14.632 | −0.42% (7/12) | −1.51% (9/12) | +0.07% (6/12) | 231.1 |
| catalog | 32 | 14.653 | 14.604 | −0.39% (9/12) | −1.05% (9/12) | +0.50% (5/12) | 213.2 |
| heldout | 8 | 14.678 | 14.768 | +0.37% (4/12) | +0.96% (4/12) | +0.89% (5/12) | 220.7 |
| heldout | 32 | 14.955 | 14.629 | −2.04% (9/12) | +0.71% (5/12) | −0.37% (9/12) | 210.2 |

The unclocked full-proof arms use twelve blocks of seventeen timed proofs. Ordinary took 14.735 and 14.628 ms, with paired calibration within 1.39% of frozen. Clock drift stayed within 2.99%. With 32 local-output shards, B2 finished last in 54 of 78 original traces and 55 of 78 held-out traces, up from 40 and 37 in ordinary execution.

The route probes verified exact equality of the 97,075 K class assignments and ordinary behavior when experimental scratch was disallowed. All 216 proof exports independently verified after retrieval. The temporary instance and storage were removed. [Eight-shape M5 CPU and Metal scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)

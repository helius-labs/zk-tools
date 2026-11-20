# Scalar preparation and workspace forecasting on C7a 4×2

The reduced forecast saved 0.11 ms of calculation, while the admission mutex wait increased by 0.12–0.13 ms. The same pattern appeared on Ryzen. On C7a, scalar hashing is the largest measured preparation step at 2.79–2.86 ms for K and 1.85–2.07 ms for B2.

The c7a.24xlarge uses 64 main and 32 frontend workers on disjoint physical cores. The solver uses 16 frontend cores, K uses the frontend pool and B2 uses main. The trial uses the [exact Ryzen executable](../2026-11-20-curve-preparation-ryzen962/README.md). Each clocked arm has six paired blocks of thirteen timed proofs after three warm-ups.

| Input | K frontend interval | Ordinary ms | Paired change with reduced forecast, ms | Paired change with B2 replay, ms |
|---|---|---:|---:|---:|
| catalog | Capacity calculation | 0.148 | −0.115 | +0.002 |
| catalog | Admission mutex wait | 0.375 | +0.132 | −0.295 |
| catalog | Dispatch through reservation | 0.777 | +0.022 | −0.291 |
| heldout | Capacity calculation | 0.147 | −0.114 | +0.002 |
| heldout | Admission mutex wait | 0.385 | +0.118 | −0.302 |
| heldout | Dispatch through reservation | 0.785 | +0.005 | −0.293 |

The B2 replay probes use the ordinary forecast and require exact equality of the base identity and complete input scalar array. The ordinary K frontend clocks measured about 0.025 ms for dispatch and 0.225–0.228 ms for the fresh memory checks under the lock. B2 replay also removes that query's admission and curve work. Its measured drop in K mutex wait isolates where the delay occurs. It does not identify how much a combined reservation would save.

| Input | Query | Scalar preparation ms | Mutex wait ms | System memory checks ms | Hash classification ms | Index assembly ms | Conversion wall ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | K | 3.712 | <0.001 | 0.179 | 2.790 | 0.356 | 0.344 |
| heldout | K | 3.721 | <0.001 | 0.187 | 2.862 | 0.293 | 0.352 |
| catalog | B2 | 3.432 | 0.177 | 0.280 | 1.847 | 0.295 | 0.783 |
| heldout | B2 | 3.469 | 0.135 | 0.290 | 2.073 | 0.300 | 0.682 |

The hash interval includes temporary allocation and initialization. The conversion interval can include work that the pool executes before the parallel conversion returns.

| Input | Frozen ms | Ordinary ms | Reduced forecast ms | Paired vs ordinary | Paired vs frozen |
|---|---:|---:|---:|---:|---:|
| catalog | 14.612 | 14.710 | 14.726 | +0.42% (5/12) | +0.62% (5/12) |
| heldout | 14.823 | 14.715 | 14.729 | +0.21% (4/12) | +0.58% (5/12) |

The unclocked arms use twelve blocks of seventeen timed proofs after three warm-ups. The ordinary arm calibrated within 0.56% of frozen. On ordinary and reduced-forecast paths the clocked medians stayed within 2.19% of their unclocked controls. The reduced forecast remains experimental. All 128 proof exports independently verified after retrieval. The temporary instance and storage were removed. [Eight-shape M5 CPU and Metal scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)

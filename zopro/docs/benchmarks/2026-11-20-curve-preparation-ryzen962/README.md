# Scalar preparation and workspace forecasting on Ryzen 4×2

Evaluating each reachable small MSM layout once cut the K workspace forecast by about 0.14 ms. K reached the admission mutex earlier and waited about 0.13 ms longer. Full-proof latency stayed near parity and the candidate remained experimental.

The observer preserves the [existing query schedule](../2026-11-20-curve-query-timing-ryzen960/README.md). It separates frontend dispatch, requested-capacity calculation, mutex wait and fresh system memory checks for K. Inside both scalar grouping routines it records sample, admission, hash classification, index assembly and coefficient conversion. Each clocked arm has four paired blocks of thirteen measured proofs after three warm-ups.

| Input | K frontend interval | Ordinary ms | Reduced forecast ms | Paired change ms |
|---|---|---:|---:|---:|
| catalog | Capacity calculation | 0.169 | 0.031 | −0.138 |
| catalog | Admission mutex wait | 0.252 | 0.384 | +0.131 |
| catalog | Dispatch through reservation | 0.637 | 0.622 | −0.005 |
| heldout | Capacity calculation | 0.171 | 0.031 | −0.139 |
| heldout | Admission mutex wait | 0.241 | 0.374 | +0.131 |
| heldout | Dispatch through reservation | 0.624 | 0.620 | −0.006 |

| Input | Query | Scalar preparation ms | Mutex wait ms | System memory checks ms | Hash classification ms | Index assembly ms | Conversion wall ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | K | 1.898 | <0.001 | 0.102 | 1.514 | 0.151 | 0.084 |
| heldout | K | 1.808 | <0.001 | 0.102 | 1.436 | 0.145 | 0.087 |
| catalog | B2 | 3.159 | 0.180 | 0.179 | 1.269 | 0.194 | 1.329 |
| heldout | B2 | 3.403 | 0.150 | 0.189 | 1.522 | 0.175 | 1.303 |

Conversion wall time includes any other work the pool performs before the parallel conversion returns. In the replay probes, removing B2 reduced K's frontend mutex wait by 0.11 ms on both inputs with the original forecast. Both query widths and point counts matched the ordinary route.

| Input | Frozen ms | Ordinary ms | Reduced forecast ms | Paired vs ordinary | Paired vs frozen |
|---|---:|---:|---:|---:|---:|
| catalog | 14.472 | 14.507 | 14.489 | −0.31% (5/8) | −0.40% (4/8) |
| heldout | 14.478 | 14.488 | 14.506 | +0.25% (2/8) | +0.05% (4/8) |

The native oracle matched held and leaf capacity estimates for both curves across 128 worker counts, 256 small input bounds and seven width configurations. Each full-proof arm has eight paired blocks of seventeen timed proofs. Clocked and unclocked ordinary and reduced-forecast medians were within 1.72%. Gnark independently verified all 88 exports. [C7a scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)

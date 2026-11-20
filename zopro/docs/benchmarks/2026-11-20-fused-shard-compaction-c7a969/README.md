# Fused shard classification and point compaction on C7a 4×2

Fusing K saved 1.34–1.49 ms in preparation and point compaction. With the longer window phase, the whole K query saved 0.77–0.79 ms. The full proof improved by 0.40–0.51% in paired medians against ordinary, with six to eight wins out of twelve. No path qualified on both inputs.

The c7a.24xlarge used the [exact Ryzen executable and sixteen-shard layout](../2026-11-20-fused-shard-compaction-ryzen968/README.md), with 64 main, 32 frontend and 16 solver cores. K uses the frontend, and B2 uses main. Each shard classifies scalars and reduces points independently. The existing window MSM consumes compact points in shard order. The extra construction reservations are 40.94 MiB for K and 65.08 MiB for B2.

| Input | Mode | Full proof ms | Paired vs ordinary | Paired vs frozen | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | frozen | 14.795 | — | — | — |
| catalog | ordinary | 14.749 | — | −0.26% (7/12) | 223.7 |
| catalog | K | 14.678 | −0.40% (8/12) | −0.61% (8/12) | 215.5 |
| catalog | B2 | 15.195 | +3.36% (1/12) | +3.16% (0/12) | 204.2 |
| catalog | K and B2 | 15.299 | +3.64% (0/12) | +3.70% (0/12) | 218.5 |
| heldout | frozen | 14.736 | — | — | — |
| heldout | ordinary | 14.740 | — | −0.03% (6/12) | 216.2 |
| heldout | K | 14.670 | −0.51% (6/12) | −0.30% (7/12) | 222.3 |
| heldout | B2 | 15.324 | +3.76% (0/12) | +3.47% (0/12) | 212.5 |
| heldout | K and B2 | 15.297 | +4.24% (0/12) | +3.48% (0/12) | 223.0 |

| Input | Single fused query | Paired preparation and compaction change, ms | Paired window change, ms | Paired total query change, ms |
|---|---|---:|---:|---:|
| catalog | K | −1.492 | +0.663 | −0.786 |
| heldout | K | −1.341 | +0.567 | −0.775 |
| catalog | B2 | +0.460 | −0.046 | +0.448 |
| heldout | B2 | +0.600 | +0.058 | +0.712 |

The unclocked profiles use twelve paired blocks of seventeen proofs. Ordinary calibrated within 0.26% of frozen. The phase probes use six paired blocks of thirteen timed proofs. The original ordinary trace showed +3.10% full-proof drift, slightly exceeding the 3% tolerance. All other traced modes stayed within that tolerance. The paired changes for each phase are descriptive. With fused K, B2 finished last in 65 of 78 traces on the original input and 60 of 78 on the held-out input.

The preflight compared every scalar class's membership and the complete curve point against the ordinary path. Zero experimental scratch selected the ordinary path. All 178 proof exports independently verified after retrieval. The temporary C7a instance and storage were removed. [Eight-shape M5 CPU and Metal scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)

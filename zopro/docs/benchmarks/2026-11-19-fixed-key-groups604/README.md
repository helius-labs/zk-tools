# Do distinct scalar counts track proof latency when only the 12×2 request changes?

All arms use 32 Ryzen workers, the same ordinary verification key and five requests. The two historical arms share the exact executable previously used on C7a. Tracing uses a separate process for each request. The two G1 counts are sorted by original base count. An absent grouped call means that route did not run. Numbers in milliseconds are medians of five warmed process means.

| Input | Distinct grouped G1 | Distinct grouped G2 | Historical / dedup off / current / Aeglos ms | Historical vs dedup off | Current vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 30,704, 46,216 | 46,062 | 33.668 / 63.108 / 28.380 / 184.035 | -47.05% (5/5) | 10.01× / 3.56× |
| 57 | 30,920, 46,524 | 46,362 | 33.672 / 62.524 / 28.546 / 183.203 | -46.15% (5/5) | — |
| 61 | 30,924, 46,528 | 46,366 | 33.698 / 63.283 / 28.420 / 183.381 | -46.64% (5/5) | — |
| 139 | 87,570, 131,116 | 130,945 | 50.081 / 62.873 / 44.773 / 182.607 | -20.35% (5/5) | — |
| 143 | 87,832, 131,542 | 131,370 | 50.565 / 63.130 / 45.479 / 183.551 | -19.86% (5/5) | — |

| Engine | Scattered minus adjacent, median paired ms | Five block differences, ms |
|---|---:|---:|
| historical | +16.650 | +16.235, +16.650, +23.373, +16.948, +16.208 |
| historical-off | +0.616 | -0.105, +0.616, +1.570, +0.982, -0.028 |
| current | +16.901 | +16.134, +16.703, +17.164, +16.901, +16.909 |
| aeglos | +0.173 | -1.589, +0.679, -1.151, +0.823, +0.173 |

The contrast gives equal weight to the two adjacent and two scattered held-out requests and excludes the reference from both. All 105 exports passed independent verification. [Protocol](protocol.md) · [Counts](artifacts/counts.json) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)


Within each block, subtracting the scattered-minus-adjacent difference with grouping disabled from the difference with grouping enabled gives +16.340, +16.033, +21.803, +15.966, +16.237 ms. The median is **+16.237 ms**. These paired measurements isolate how changing the grouping option changes the latency gap between these requests. Other work inside that option, including point combination and memory allocation, contributes alongside the number of distinct scalars. Of the five current-build reference processes, 3 met the 28.400 ms M5 CPU target.

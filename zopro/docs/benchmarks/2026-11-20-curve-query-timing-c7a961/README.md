# K and B2 query timing on C7a 4×2

Replacing B2 with an exact replay shortened the still-active K query by 0.54–0.65 ms on C7a, compared with 1.64–1.66 ms on Ryzen. Roughly 0.30 ms came before K began scalar preparation and 0.32–0.33 ms came from its window phase. With both queries active, scalar preparation took 3.66–3.71 ms for K and 3.41–3.53 ms for B2. These intervals include sampling, workspace admission, classification, metadata emission and coefficient conversion.

This uses the [same observer and replay checks as the Ryzen trial](../2026-11-20-curve-query-timing-ryzen960/README.md). The c7a.24xlarge has 96 physical workers, with 64 main and 32 frontend. Solving uses 16 frontend cores, K uses the frontend and B2 uses main. A and H stay active in every diagnostic arm. Each query clock records six paired blocks of thirteen timed proofs on both inputs.

| Input | Active query | All active ms | Peer replayed ms | Paired change ms | Before preparation | Scalar preparation | Point compaction | Windows and return |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | K | 7.080 | 6.502 | −0.538 | −0.300 | +0.018 | −0.009 | −0.317 |
| heldout | K | 7.177 | 6.507 | −0.648 | −0.304 | −0.044 | −0.005 | −0.334 |
| catalog | B2 | 7.304 | 6.919 | −0.419 | ≈0 | −0.092 | −0.216 | −0.134 |
| heldout | B2 | 7.202 | 7.145 | −0.005 | ≈0 | +0.054 | +0.041 | −0.138 |

The change columns are paired medians in milliseconds. They can differ from the difference of the displayed medians. The interval before preparation includes K's frontend dispatch and workspace check.

| Replayed queries | Predicted saving with retained query times fixed, ms | Measured curve-stage saving, ms |
|---|---:|---:|
| K | 0.048 / 0.124 | 0.403 / 0.233 |
| B2 | 0.214 / 0.102 | 1.153 / 0.973 |
| K and B2 | 0.941 / 1.007 | 3.341 / 3.151 |

Each pair is catalog and heldout. The prediction substitutes the measured replay durations at the ordinary starts and keeps the remaining query times and join overhead fixed. The observed timings show that the retained work runs differently when K or B2 is removed. They do not distinguish scheduling, workspace admission and memory-system effects.

In twelve unclocked blocks, the frozen catalog and held-out medians were 14.425 and 14.704 ms. The ordinary observer build took 14.688 and 14.453 ms, within 0.46% of frozen in paired medians. Tracing moved all paired full-proof medians by less than 2.34% in magnitude except held-out joint replay, which shifted +4.45%. Treat that clocked joint result as exploratory. The two single-replay comparisons and the fully active clocks stayed within the 3% observer tolerance. All 176 proofs independently verified after retrieval. The temporary instance and bucket were removed, with zero active tagged benchmark instances and zero associated volumes in the region.

## Eight supplied M5 examples

| Shape | Recorded C7a baseline ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.457 | 109 / 48 | 7.54× / 3.32× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Earlier scorecard](../2026-11-20-poseidon-handoff-c7a959/README.md#eight-supplied-m5-examples) · [Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup](artifacts/cleanup.json)

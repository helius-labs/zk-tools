# Curve query ranges ready across solver bands

For 2×2, the recorded solver order makes 28 of 52 contiguous 1,024-point B2 ranges available before the third band. With 4,096-point ranges, 3 of 13 are available at that point and 7 before the last band. On 4×2 the corresponding 4,096-point counts are 8 and 12 out of 24. Contiguous ranges can borrow the original bases directly. The [C7a curve replay](../2026-11-19-g1-query-bound-c7a803/README.md) measured a larger latency effect for removing B2 than for removing A on 1×2 and 2×2.

The reader uses the pinned solver images and four-band configurations from the [FFT readiness study](../2026-11-20-solver-fft-readiness878/README.md). It maps each query index to the band of its wire's producer. An entire range is available once its last writer finishes. The counts use original key order and include the final partial range. Each column gives the cumulative count after each of four bands.

| Shape | Query | Terms | Ranges of 1,024 | Ranges of 4,096 |
|---|---|---:|---:|---:|
| 1×2 | A | 21,006 | 5 / 10 / 14 / 21 | 0 / 2 / 3 / 6 |
| 1×2 | B2 | 30,089 | 8 / 16 / 22 / 30 | 0 / 3 / 4 / 8 |
| 2×2 | A | 36,637 | 8 / 18 / 26 / 36 | 0 / 2 / 4 / 9 |
| 2×2 | B2 | 52,274 | 14 / 28 / 40 / 52 | 1 / 3 / 7 / 13 |
| 4×2 | A | 67,154 | 23 / 36 / 48 / 66 | 2 / 4 / 6 / 17 |
| 4×2 | B2 | 95,537 | 34 / 54 / 73 / 94 | 3 / 8 / 12 / 24 |

These are readiness counts. Separate MSMs can add bucket weighting, duplicate scalar grouping across range boundaries and compete with the solver. The scan does not measure those effects. All counts use the existing solver images and require no witness values.

[Counts](artifacts/counts.json) · [Reader](artifacts/reader.py) · [Pinned workflow](../2026-11-20-poseidon-first-round-ryzen877/artifacts/workflow.json.gz) · [Image pins](../2026-11-20-poseidon-first-round-ryzen877/artifacts/pins.json)

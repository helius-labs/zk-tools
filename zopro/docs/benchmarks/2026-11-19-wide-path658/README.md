# Elapsed proof phases on the two wide-output keys

Two byte-identical copies of the current ordinary SDK used the same key, request and 32 main Ryzen workers. One copy ran with Linux entry/return probes. In six paired blocks, each process warmed three proofs and timed thirteen. CPU affinity and exported proof validity were checked for every process. The first columns give medians of process means and the paired change. Phase columns give arithmetic mean elapsed milliseconds across 78 traced requests per key. Within each function, overlapping intervals are counted once. Columns for different functions may overlap. Zero means the named function was not observed.

| Shape | Plain / traced ms | Paired change | Full traced span | Solver | FFT product | General curve query driver | Exclusive G2 | Grouped G2 | Grouped G1 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x16 | 12.794 / 13.045 | +1.82% | 13.023 | 1.628 | 1.369 | 0.000 | 8.740 | 0.000 | 5.884 |
| 5x16 | 23.737 / 23.949 | +1.79% | 25.669 | 3.249 | 5.199 | 15.251 | 0.000 | 9.188 | 9.376 |

Using the median of six per-process means limits the effect of one 34.49 ms traced 5×16 process. By this measure the FFT took 1.34 ms on 1×16 and 4.41 ms on 5×16. Subtracting the entire FFT from each trace still left 11.65 and 19.54 ms, respectively. The observed ordinary half-latencies were 6.40 and 11.87 ms. This subtraction holds the rest of the execution fixed.

On 1×16 the exclusive G2 query took 8.77 ms and overlapped grouped G1 for 5.78 ms. On 5×16, grouped G1 and G2 overlapped for 7.47 ms. The first grouped operation began 3.97 ms after FFT return. The interval includes preparation and other concurrent work. Speeding up just one overlapping query can leave the other on the critical path. The 5×16 key has 145,951 B2 points and the catalog witness has 51,707 distinct nonzero B2 scalars. This motivates testing a grouped operation that exploits the smaller size and exclusive ownership of the coefficients. [Measured class counts](../2026-11-19-crossover-groups603/README.md)

[Symbols and executable digest](artifacts/symbols.json) · [Per-proof intervals](artifacts/rows.json.gz) · [Timing and call counts](artifacts/summary.json) · [Independent verification](artifacts/verification.json)

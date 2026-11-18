# Where 24×2 spends time on the Ryzen 9950X

On the exact accepted SDK, each of 102 traced requests finished its FFT before starting any of the three scalar-grouping calls. All three calls handled witness values available before the FFT. At 16 workers the FFT averaged 23.51 ms and the subsequent MSM stage averaged 41.60 ms. Rescheduling independent work across this boundary is a candidate for full-proof measurement.

| 24×2 elapsed time | 16 workers | 32 workers |
|---|---:|---:|
| Plain verified request, median of three process means | 76.60 ms | 98.02 ms |
| Witness solve, traced mean | 7.43 ms | 10.78 ms |
| Solver return to FFT entry | 1.95 ms | 6.11 ms |
| FFT | 23.51 ms | 44.62 ms |
| MSM | 41.60 ms | 34.47 ms |
| Other request time | 1.79 ms | 5.33 ms |
| Supplied M5 CPU / plain Zopro | 6.89× | 5.39× |
| Supplied M5 Metal / plain Zopro | 2.30× | 1.80× |

The FFT and MSM rows are disjoint elapsed intervals. Within MSM the reader observed two grouped G1 calls, one grouped G2 call, three raw G1 calls and one raw G2 call per request. Grouped calls contain raw calls. Parallel and nested callback durations cannot be added to obtain request latency. The CPU target is 52.8 ms against the supplied 528 ms M5 CPU reference. The M5 Metal reference is 176 ms.

The panel used 17 timed sequential requests after three primes per process, three independent executable backings, and the same image and request. At both widths the median trace/plain ratio passed the prespecified 5% limit (1.003 and 1.034, respectively). One 32-worker block had 8.3% observer overhead. These are local results. The C7a result needs a separate matched test.

An exploratory first process also traced the solver's frequently called instruction evaluator, producing nearly ten million events. We verified that export separately and removed the hot probe from the complete panel. The table uses only the sparse observer and its controls. All 13 process exports passed Gnark and were fresh against the prior ledger, bringing it to 65,247. [Protocol](artifacts/protocol.md) · [Per-request intervals](artifacts/rows.json.gz) · [Reader](artifacts/read.rs)

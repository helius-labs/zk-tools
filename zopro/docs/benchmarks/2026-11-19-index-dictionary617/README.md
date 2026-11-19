# Store a class index in each proof-local hash-table entry

On 4×2, an indexed entry reduced paired latency by 4.96% against the ordinary dictionary and 1.68% against the frozen SDK. It also lowered observed peak RSS against both. The 5×16 and 6×2 requests had lower paired latency against both controls. Each had higher peak RSS than the frozen binary. The 1×2 and 2×2 results against the frozen binary were mixed. These timings use one input per shape.

The indexed dictionary borrows the existing class values for equality checks. Its table storage column sums the final allocation of each table constructed during one diagnostic proof. Process peak RSS is measured separately in the timing arms.

| Transfer | Indexed table bytes/proof | Prior / ordinary / indexed ms | Indexed vs ordinary / prior | Peak prior / ordinary / indexed MiB | Indexed vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2 | 163,856 | 7.033 / 7.172 / 7.076 | -0.83% (6/7) / +0.46% (2/7) | 66.8 / 66.2 / 61.8 | 5.23× / 3.39× |
| 2x2 | 491,568 | 9.480 / 9.527 / 9.497 | -0.37% (5/7) / -0.02% (4/7) | 114.5 / 119.0 / 115.0 | 6.21× / 3.16× |
| 4x2 | 819,248 | 15.378 / 15.732 / 15.019 | -4.96% (7/7) / -1.68% (6/7) | 324.2 / 325.5 / 295.2 | 7.26× / 3.20× |
| 5x16 | 983,088 | 24.186 / 24.615 / 23.962 | -2.68% (6/7) / -1.04% (6/7) | 425.2 / 458.2 / 436.5 | — |
| 6x2 | 819,248 | 21.629 / 21.569 / 21.019 | -2.98% (6/7) / -2.82% (7/7) | 382.2 / 399.8 / 389.5 | — |

All 110 exports verified independently. [Protocol](protocol.md) · [Source](artifacts/source.patch) · [Raw samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)

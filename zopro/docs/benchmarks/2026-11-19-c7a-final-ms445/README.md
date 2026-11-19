# Arithmetic samples just before the C7a curve queries return

This reanalysis uses the archived 64- and 96-core traces from the same C7a. For each proof, the sampling window ends when the enclosing curve-query function returns and starts after H returns. Each cell counts requests with at least one directly named arithmetic sample on any worker. Rayon iterator symbols with a curve type in their generic parameters are excluded from the arithmetic counts. Cases with no named arithmetic sample remain in the denominator. All twelve archived exports passed independent Gnark verification again.

| Shape | Cores | Window µs | Requests with full window / 33 | G1 only | G2 only | Both | Neither | G1 / G2 samples |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 1000 | 20 | 14 | 7 | 2 | 10 | 76 / 41 |
| 16x2 | 64 | 250 | 21 | 7 | 0 | 0 | 26 | 14 / 0 |
| 16x2 | 96 | 1000 | 22 | 11 | 11 | 2 | 9 | 38 / 63 |
| 16x2 | 96 | 250 | 23 | 2 | 0 | 0 | 31 | 2 / 0 |
| merge-24x1 | 64 | 1000 | 16 | 14 | 7 | 1 | 11 | 62 / 21 |
| merge-24x1 | 64 | 250 | 21 | 7 | 1 | 0 | 25 | 9 / 1 |
| merge-24x1 | 96 | 1000 | 21 | 7 | 14 | 1 | 11 | 33 / 80 |
| merge-24x1 | 96 | 250 | 24 | 1 | 2 | 0 | 30 | 1 / 2 |

On merge at 96 cores, 15 of 33 requests had a named G2 sample in the final millisecond, compared with eight at 64. In the final 250 µs, 30 of 33 requests at 96 cores had no named G1 or G2 sample. The measurements motivate tracing the individual query returns before choosing a kernel to optimize.

CPU-clock samples represent approximately two CPU-ms each. Short windows can contain samples from multiple simultaneously running workers or none at all. Samples show which arithmetic was executing close to the return. Exact critical-path attribution requires recording the A, K and B2 completion times for each request.

[Request counts](artifacts/requests.json.gz) · [Symbol classification](artifacts/symbol-classes.json.gz) · [Trace and proof identities](artifacts/inputs.json) · [Whole post-H interval](../2026-11-19-c7a-curve-tail424/README.md)

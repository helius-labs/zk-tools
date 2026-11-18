# Frontend worker limits on an independent C7a host

The total worker count remained fixed throughout each proof. On the experimental build the frontend limit applied to the witness solve, scalar preparation and FFT. The remaining cores joined for the curve queries. A frontend value equal to total cores ran without restriction.

With 32 frontend workers, limiting the proof to **64 total cores** beat 96 in all seven paired blocks, with a median paired latency reduction of **7.29%**. The earlier host measured 9.471 ms at 64 cores and this host measured 9.482 ms. The lowest exploratory median here was 9.399 ms at 64 total / 24 frontend, but it beat 64 / 32 in just three pairs. The evidence supports the total core limit. It gives little reason to change the frontend setting from 32 to 24. [First host](../2026-11-18-proof-c7a291/README.md)

| Shape | Engine | Total | Frontend | Median ms | Paired vs reference | CPU ms | Peak MiB | M5 CPU / Metal | Blocks ≤9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | Accepted | 64 | — | 10.979 | +15.67% | 400 | 121 | 3.37× / 2.19× | 0/7 |
| 1x2 | Aeglos | 96 | — | 7.902 | -16.23% | 515 | 471 | 4.68× / 3.04× | 7/7 |
| 1x2 | Zopro | 56 | 16 | 9.418 | +0.94% | 269 | 64 | 3.93× / 2.55× | 0/7 |
| 1x2 | Zopro | 56 | 24 | 9.502 | +0.51% | 280 | 71 | 3.89× / 2.53× | 0/7 |
| 1x2 | Zopro | 56 | 32 | 9.671 | +1.99% | 293 | 77 | 3.83× / 2.48× | 0/7 |
| 1x2 | Zopro | 64 | 8 | 9.828 | +4.22% | 285 | 57 | 3.76× / 2.44× | 0/7 |
| 1x2 | Zopro | 64 | 16 | 9.623 | +2.91% | 297 | 60 | 3.84× / 2.49× | 0/7 |
| 1x2 | Zopro | 64 | 24 | 9.399 | +0.32% | 301 | 68 | 3.94× / 2.55× | 0/7 |
| 1x2 | Zopro | 64 | 32 | 9.482 | +0.00% | 313 | 69 | 3.90× / 2.53× | 0/7 |
| 1x2 | Zopro | 64 | 40 | 9.485 | +0.51% | 325 | 67 | 3.90× / 2.53× | 0/7 |
| 1x2 | Zopro | 64 | 48 | 9.590 | +2.37% | 339 | 70 | 3.86× / 2.50× | 0/7 |
| 1x2 | Zopro | 64 | 64 | 9.703 | +2.45% | 367 | 62 | 3.81× / 2.47× | 0/7 |
| 1x2 | Zopro | 72 | 16 | 9.585 | +1.72% | 324 | 58 | 3.86× / 2.50× | 0/7 |
| 1x2 | Zopro | 72 | 24 | 9.585 | +1.62% | 333 | 69 | 3.86× / 2.50× | 0/7 |
| 1x2 | Zopro | 72 | 32 | 9.668 | +0.07% | 342 | 69 | 3.83× / 2.48× | 0/7 |
| 1x2 | Zopro | 96 | 32 | 10.044 | +7.87% | 454 | 72 | 3.68× / 2.39× | 0/7 |
| 2x2 | Accepted | 48 | — | 14.086 | +0.60% | 412 | 276 | 4.19× / 2.13× | — |
| 2x2 | Aeglos | 96 | — | 11.560 | -17.14% | 812 | 821 | 5.10× / 2.60× | — |
| 2x2 | Zopro | 48 | 16 | 14.167 | +0.82% | 356 | 287 | 4.16× / 2.12× | — |
| 2x2 | Zopro | 48 | 24 | 14.015 | -0.25% | 368 | 295 | 4.21× / 2.14× | — |
| 2x2 | Zopro | 48 | 32 | 14.003 | +0.00% | 380 | 294 | 4.21× / 2.14× | — |
| 2x2 | Zopro | 48 | 48 | 14.045 | +0.30% | 406 | 289 | 4.20× / 2.14× | — |

For 1×2 the reference was the prespecified 64 total / 32 frontend setting. For 2×2 it was 48 / 32. Each cell covers seven rotated processes, each timing 71 sequential requests after three primes. Relative comparisons use within-block ratios. CPU is summed user and system time per proof; RSS is the maximum whole-process high-water mark. Accepted denotes the production Zopro build. Zopro denotes the exact experimental 257 executable. It certified all 1×2 requests and used the original fallback on 2×2. The accepted build and retained Aeglos used the same host, fixture and block rotation.

All 154 exports passed independent Gnark verification on AWS and after retrieval. The instance terminated and the temporary volume and bucket were removed. [Paired block data](artifacts/cells.json) · [Process samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

## Individual 1×2 proof latency

Each row pools the 497 timed requests across seven processes. Quantiles are order statistics at zero-based index ⌊p(n−1)⌋. The table above reports the median of the seven process means in each configuration.

| Engine | Total / frontend | P50 ms | P90 ms | Under 9 ms | At least 11 ms |
|---|---:|---:|---:|---:|---:|
| Accepted | 64 / — | 10.996 | 11.736 | 0 | 248 |
| Zopro | 96 / 32 | 9.324 | 12.469 | 56 | 133 |
| Zopro | 64 / 32 | 8.531 | 11.583 | 326 | 154 |
| Zopro | 64 / 24 | 8.481 | 11.523 | 328 | 149 |
| Aeglos | 96 / — | 7.888 | 8.308 | 492 | 0 |

At 64 / 32, 326 individual proofs finished under 9 ms. Another 154 took at least 11 ms, enough to keep all seven process means above the milestone. Aeglos stayed under 11 ms for every request. Even the pooled median Zopro request at 64 / 32 was 0.64 ms behind Aeglos. Closing the gap calls for measuring both the common path and the slower requests.

Using each block’s median sub-10 ms proof as a reference, the excess in ≥11 ms requests averaged 1.05 ms per request in the median block. Removing half that excess with all other requests unchanged would put the median block mean at 8.955 ms. Removing all of it would yield 8.474 ms. This arithmetic estimates the size of the opportunity. Recoverable time requires a measured intervention. [Per-block calculation](artifacts/tail-counterfactual.json)

# G2 bucket parallelism on C7a and K/H replication

Each G2 mode specifies digit bits and number of contiguous point ranges per window. The original uses 12 bits, two ranges and 22 fill tasks. At 11 bits each ordinary bucket array is half the size; with three or four ranges there are 36 or 48 fill tasks. Ten bits and five ranges yield 65 tasks. The final top window uses a specialized reduction. The candidate uses the selected geometry in the requested-capacity forecast. All runs measure a whole, sequential, warm SDK proof.

The K/H continuation replicated at 64 cores. **Split measured 9.239 ms**, the same-binary disabled control 9.482 ms, and exact 257 9.391 ms. Split beat disabled in all seven paired blocks (2.27% median reduction), and reduced requests ≥11 ms from 234 to 117 of 707. The first host gave 9.390 ms and a 2.37% paired reduction. Peak RSS on this host was 81 MiB for split and 75 MiB disabled. All seven split process means remained above 9 ms. Aeglos at 96 cores measured 7.919 ms and 479 MiB.

Smaller G2 buckets gave a width-dependent result. The prespecified 11-bit/three-range setting at 64 cores lost five of seven pairs. At 96 cores, 11-bit/four-range cut paired 1×2 latency by 4.15% in seven wins, with 9.907 ms against 10.183 ms disabled and 10.082 ms for the exact parent. On 2×2 at 48 cores the same geometry won seven pairs, 13.517 ms against 14.012 ms disabled and 13.927 ms on the exact parent. Both four-range results were exploratory and come from one C7a host. The split K/H result has two independent hosts.

| Shape | Cores / threads | Mode | Median ms | Paired vs disabled | Wins | CPU ms | Peak MiB | Requests <9 / ≥11 ms | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 10x5 | 10.047 | +5.51% | 0/7 | 325 | 83 | 112 / 115 | 3.68× / 2.39× |
| 1x2 | 64 | 11x3 | 9.595 | +0.32% | 2/7 | 323 | 77 | 361 / 53 | 3.86× / 2.50× |
| 1x2 | 64 | 11x4 | 9.875 | +4.72% | 0/7 | 328 | 75 | 196 / 61 | 3.75× / 2.43× |
| 1x2 | 64 | geom-off | 9.492 | — | — | 318 | 73 | 454 / 236 | 3.90× / 2.53× |
| 1x2 | 64 | off | 9.482 | — | — | 316 | 75 | 457 / 234 | 3.90× / 2.53× |
| 1x2 | 64 | ref | 9.391 | -1.43% | 7/7 | 311 | 80 | 489 / 199 | 3.94× / 2.56× |
| 1x2 | 64 | split | 9.239 | -2.27% | 7/7 | 313 | 81 | 478 / 117 | 4.00× / 2.60× |
| 1x2 | 96 | 10x5 | 9.990 | -2.27% | 6/7 | 466 | 67 | 240 / 195 | 3.70× / 2.40× |
| 1x2 | 96 | 11x3 | 9.975 | -2.62% | 6/7 | 471 | 66 | 273 / 248 | 3.71× / 2.41× |
| 1x2 | 96 | 11x4 | 9.907 | -4.15% | 7/7 | 469 | 69 | 265 / 196 | 3.73× / 2.42× |
| 1x2 | 96 | aeglos | 7.919 | — | — | 511 | 479 | 705 / 0 | 4.67× / 3.03× |
| 1x2 | 96 | geom-off | 10.183 | — | — | 479 | 73 | 30 / 205 | 3.63× / 2.36× |
| 1x2 | 96 | ref | 10.082 | -1.88% | 6/7 | 453 | 69 | 41 / 173 | 3.67× / 2.38× |
| 2x2 | 48 | 10x5 | 14.081 | +0.06% | 2/7 | 385 | 306 | 0 / 707 | 4.19× / 2.13× |
| 2x2 | 48 | 11x3 | 13.549 | -3.11% | 7/7 | 382 | 308 | 0 / 707 | 4.35× / 2.21× |
| 2x2 | 48 | 11x4 | 13.517 | -3.39% | 7/7 | 386 | 312 | 0 / 707 | 4.36× / 2.22× |
| 2x2 | 48 | geom-off | 14.012 | — | — | 381 | 299 | 0 / 707 | 4.21× / 2.14× |
| 2x2 | 48 | ref | 13.927 | -1.00% | 6/7 | 379 | 306 | 0 / 707 | 4.24× / 2.15× |
| 2x2 | 96 | aeglos | 11.609 | — | — | 827 | 824 | 0 / 696 | 5.08× / 2.58× |

Seven rotated processes per cell, 101 requests after three primes. Ref is the exact experimental 257 SDK. The G2 variants use geom-off as the disabled control. Split and off use the separate K/H executable from the previous host. Aeglos retained its warm cache at 96 cores. No configuration combines the two code changes. All 133 distinct process exports verified on the instance and again after retrieval. The instance terminated and temporary storage was removed. [Local source and qualification](../2026-11-18-g2-geometry297/README.md) · [First K/H comparison](../2026-11-18-kh-c7a296/README.md)

[Paired blocks](artifacts/pairs.json) · [All requests](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

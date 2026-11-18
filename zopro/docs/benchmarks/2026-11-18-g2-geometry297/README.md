# G2 bucket parallelism at smaller window widths

Each G2 mode specifies digit bits and number of contiguous point ranges per window. The original uses 12 bits, two ranges and 22 fill tasks. At 11 bits each ordinary bucket array is half the size; with three or four ranges there are 36 or 48 fill tasks. Ten bits and five ranges yield 65 tasks. The final top window uses a specialized reduction. The candidate uses the selected geometry in the requested-capacity forecast. All runs measure a whole, sequential, warm SDK proof.

At 32 logical Ryzen threads the primary three-range configuration took 8.056 ms, compared with 7.563 ms disabled and 7.541 ms on the exact parent. All three new geometries were slower on 1×2. On 2×2, four ranges at 11 bits measured 10.461 ms against 10.533 ms disabled. Separate route probes verified the expected callback counts on both shapes. The C7a panel tests whether a larger physical core count changes the balance between extra parallelism and additional bucket merging.

| Shape | Cores / threads | Mode | Median ms | Paired vs disabled | Wins | CPU ms | Peak MiB | Requests <9 / ≥11 ms | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 16 | 10x5 | 9.284 | +5.14% | 1/3 | 107 | 65 | 14 / 0 | 3.99× / 2.59× |
| 1x2 | 16 | 11x3 | 9.031 | +0.39% | 1/3 | 105 | 65 | 37 / 0 | 4.10× / 2.66× |
| 1x2 | 16 | 11x4 | 9.223 | +2.97% | 1/3 | 106 | 65 | 25 / 0 | 4.01× / 2.60× |
| 1x2 | 16 | off | 8.957 | — | — | 103 | 65 | 37 / 2 | 4.13× / 2.68× |
| 1x2 | 32 | 10x5 | 7.914 | +3.04% | 2/7 | 155 | 102 | 293 / 54 | 4.68× / 3.03× |
| 1x2 | 32 | 11x3 | 8.056 | +4.94% | 1/7 | 154 | 94 | 262 / 74 | 4.59× / 2.98× |
| 1x2 | 32 | 11x4 | 7.867 | +2.96% | 1/7 | 157 | 93 | 280 / 57 | 4.70× / 3.05× |
| 1x2 | 32 | off | 7.563 | — | — | 149 | 88 | 305 / 43 | 4.89× / 3.17× |
| 1x2 | 32 | ref | 7.541 | -0.13% | 4/7 | 149 | 103 | 313 / 38 | 4.91× / 3.18× |
| 2x2 | 32 | 10x5 | 10.809 | +2.61% | 1/3 | 211 | 251 | 0 / 14 | 5.46× / 2.78× |
| 2x2 | 32 | 11x3 | 10.575 | +0.40% | 1/3 | 207 | 254 | 0 / 7 | 5.58× / 2.84× |
| 2x2 | 32 | 11x4 | 10.461 | -0.75% | 2/3 | 209 | 252 | 0 / 7 | 5.64× / 2.87× |
| 2x2 | 32 | off | 10.533 | — | — | 201 | 256 | 0 / 18 | 5.60× / 2.85× |

Seven randomized process blocks at 32 local Ryzen threads, three primes and 51 timed 1×2 requests per process. Secondary cells have three blocks of 21 requests each. Eight separate route probes counted calls into the actual fill callback. All 145 native tests passed, including generator-point full-curve checks for the three new geometries. The generic build and native resource-scheduler-only configuration compiled. The attached patch includes a conditional default for the latter configuration; that branch is excluded in the timed native build. [Source patch](artifacts/source.patch.gz) · [Route counts](artifacts/routes.json)

[Paired blocks](artifacts/pairs.json) · [All requests](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

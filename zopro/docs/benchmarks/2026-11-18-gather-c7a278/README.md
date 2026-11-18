# Splitting G2 scalars in place during witness gather on C7a

The 96-core gather variant did not meet the positive performance screen. At 96 cores moving the split into the gather changed paired latency by +6.23% and CPU time by +4.17%, with 0/7 latency wins. The paired OFF/parent calibration ratio was 0.999.

| Cores | Engine / split timing | Wall ms | CPU ms | Peak MiB | Fast / slow out of 1,057 | M5 CPU | M5 Metal |
|---:|---|---:|---:|---:|---:|---:|---:|
| 96 | Accepted Zopro | 12.473 | 696 | 117 | 0 / 1057 | 2.97× | 1.92× |
| 96 | Exact 257 | 10.126 | 457 | 69 | 760 / 293 | 3.65× | 2.37× |
| 96 | Original, same binary | 10.136 | 455 | 76 | 767 / 288 | 3.65× | 2.37× |
| 96 | During B gather | 10.823 | 473 | 74 | 487 / 549 | 3.42× | 2.22× |
| 96 | During FFT | 10.818 | 474 | 67 | 477 / 564 | 3.42× | 2.22× |
| 96 | Aeglos | 7.935 | 508 | 486 | 1056 / 0 | 4.66× | 3.02× |
| 32 | Exact 257 | 10.700 | 230 | 75 | 17 / 270 | 3.46× | 2.24× |
| 32 | Original, same binary | 10.645 | 231 | 69 | 18 / 175 | 3.48× | 2.25× |
| 32 | During B gather | 11.196 | 232 | 74 | 0 / 909 | 3.30× | 2.14× |

At 32 cores moving the split into the gather changed paired latency by +5.05% and CPU time by +0.09%, with 0/7 latency wins. The paired OFF/parent calibration ratio was 0.995. The fast class is under 10 ms and the slow class is at least 11 ms. Every ON proof used the split gathered scalars and none required restoration on this input. Every experimental proof certified. The earlier FFT placement used the same executable. These are full-proof measurements with three primes and 151 timed requests in each of seven main processes. All 72 fresh exported proofs verified in Gnark on AWS and after retrieval. The instance terminated, tagged volumes were absent and the temporary bucket was deleted. [Source and forced fallback](../2026-11-18-gather-split277/README.md) · [Block data](artifacts/cells.json) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

## Accepted build against the eight supplied M5 references

The 24×2 row comes from the independent fixed 48-core confirmation. Other rows use the faster accepted-build median from the earlier matched 32/96-core catalog panel. This table gives comparable reference coverage for the production build. Only 1×2 was remeasured in the present experiment. [Full catalog methodology](../2026-11-18-full-catalog-c7a244/README.md) · [24×2 confirmation](../2026-11-18-confirm-48c271/README.md)

| Shape | Cores | Zopro ms | M5 CPU ms | M5 Metal ms | CPU speedup | Metal speedup | 10× CPU target ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 10.846 | 37 | 24 | 3.411× | 2.213× | 3.7 |
| 2x2 | 32 | 14.665 | 59 | 30 | 4.023× | 2.046× | 5.9 |
| 4x2 | 32 | 19.658 | 109 | 48 | 5.545× | 2.442× | 10.9 |
| 8x2 | 32 | 26.891 | 189 | 74 | 7.028× | 2.752× | 18.9 |
| 12x2 | 32 | 36.019 | 284 | 101 | 7.885× | 2.804× | 28.4 |
| 16x2 | 32 | 41.253 | 335 | 124 | 8.121× | 3.006× | 33.5 |
| 24x2 | 48 | 52.731 | 528 | 176 | 10.013× | 3.338× | 52.8 |
| merge-24x1 | 96 | 62.182 | 507 | 175 | 8.153× | 2.814× | 50.7 |

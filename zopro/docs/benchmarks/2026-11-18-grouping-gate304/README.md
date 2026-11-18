# Testing scalar diversity before allocating the G2 grouping map

The candidate performs the ordinary grouping sample first. If that sample permits grouping, the exclusive G2 path can count occupied bits in a proof-local bitmap and bypass the map when the grouping policy would decline. Equal scalars set the same bit. Collisions reduce the bound and preserve the ordinary fallback. The bitmap holds at most 32 KiB plus an admission charge for fixed overhead. This variant runs inline in the accepted-source prover and uses its existing pool.

Each pass uses five randomized blocks. A fresh 32-worker process runs three primes and 51 timed requests. The first 1×2 pass had individual process means from 7.27 to 13.11 ms. A second randomized pass checks sensitivity to that spread. The last two columns compare enabled with disabled, then disabled with the archived executable by block.

| Pass | Shape | On / disabled / accepted ms | Peak MiB on / disabled / accepted | M5 CPU / Metal speedup on | Paired on change | Wins | Control calibration |
|---|---|---:|---:|---:|---:|---:|---:|
| First | 1x2 | 7.301 / 10.409 / 7.500 | 98 / 137 / 131 | 5.07× / 3.29× | -3.52% | 4/5 | +1.33% |
| First | 2x2 | 10.505 / 10.550 / 10.411 | 285 / 296 / 284 | 5.62× / 2.86× | -0.59% | 3/5 | +1.64% |
| Repeat | 1x2 | 7.321 / 7.393 / 7.395 | 89 / 116 / 115 | 5.05× / 3.28× | -1.21% | 3/5 | +0.12% |
| Repeat | 2x2 | 10.544 / 10.418 / 10.478 | 281 / 290 / 284 | 5.60× / 2.85× | +1.20% | 2/5 | -0.66% |

All 60 timed processes exported independently verified proofs. Two additional verified proofs exercised zero optional preparation allowance and four logical workers. The conservative-bound test passed, and the generic core library compiled. The four-worker run used x86 hardware. Production keeps its existing default pending a target-host comparison.

[Source patch](artifacts/source.patch) · [Raw timing](artifacts/rows.json.gz) · [Fallback proofs](artifacts/fallback.json) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md)

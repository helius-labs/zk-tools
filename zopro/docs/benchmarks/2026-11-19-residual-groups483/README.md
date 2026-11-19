# Prepared B2 point sums with sparse scalar corrections

Eight public training requests identified 17,976 sets of equal B2 scalars. Each set contains at least three positions and varied across at least three distinct values during training. The input index plan occupies 897,920 bytes. Preparation adds the actual key points in each set. The original request and eight different witnesses were evaluated without changing the groups. All arms retain the qualified public offsets and FFT settings. Each observation used 32 Ryzen workers, three primes and eleven timed SDK requests. Three randomized blocks compared the prototype enabled, the identical executable disabled, and the production executable.

The prototype keeps the compact B2 basis. Every call computes exact scalar differences for members whose prediction fails and runs a separate sparse query for them. A high mismatch count or insufficient optional memory selects the ordinary query. The original route probe found 8 nonzero residuals.

| Request | Release / off / on ms | Paired vs release (wins) | Paired vs off (wins) | Peak MiB release / on |
|---|---:|---:|---:|---:|
| Original | 32.490 / 31.548 / 30.467 | -4.70% (3/3) | -1.91% (3/3) | 551.7 / 625.9 |
| 57 | 31.121 / 31.386 / 30.397 | -4.68% (3/3) | -5.49% (3/3) | 541.2 / 590.5 |
| 58 | 31.409 / 31.567 / 30.289 | -3.19% (3/3) | -4.05% (2/3) | 559.1 / 610.7 |
| 59 | 31.469 / 31.345 / 30.540 | -1.81% (3/3) | -3.01% (2/3) | 567.8 / 603.5 |
| 60 | 31.939 / 30.969 / 30.051 | -5.84% (3/3) | -3.01% (3/3) | 546.9 / 571.7 |
| 61 | 31.009 / 31.433 / 30.566 | -1.79% (3/3) | -2.78% (3/3) | 535.8 / 603.1 |
| 62 | 31.384 / 31.474 / 30.300 | -3.23% (3/3) | -3.55% (3/3) | 541.8 / 596.4 |
| 63 | 31.691 / 31.692 / 30.059 | -4.62% (3/3) | -4.96% (3/3) | 573.5 / 613.0 |
| 64 | 31.917 / 31.590 / 30.621 | -5.26% (3/3) | -2.54% (3/3) | 566.5 / 601.8 |

The held-out median change was -3.93% vs release (8/8 faster) and -3.28% vs disabled (8/8 faster). The candidate passed the initial full-proof screen. A zero optional allowance exercised ordinary proving. All 84 exports verified independently with Gnark. The [qualified scorecard](../2026-11-19-merge-deep479/README.md) is the reference pending independent confirmation.

[Results and route probes](artifacts/results.json) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Experimental patch](artifacts/source.patch)

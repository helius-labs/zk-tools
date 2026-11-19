# Verified B2 point sums on 12×2

Eight public training requests identified 17,976 sets of equal B2 scalars. Each set contains at least three positions and varied across at least three distinct values during training. The input index plan occupies 897,920 bytes. Preparation adds the actual key points in each set. The original request and eight different witnesses were evaluated without changing the groups. All arms retain the qualified public offsets and FFT settings. Each observation used 32 Ryzen workers, three primes and eleven timed SDK requests. Three randomized blocks compared the prototype enabled, the identical executable disabled, and the production executable.

Each proof checks equality across all members. The prototype emits a prepared sum for matching sets and copies the original points and scalars for every member of a mismatching set. The original route probe found 4 mismatching sets. Insufficient optional memory selects the ordinary query.

| Request | Release / off / on ms | Paired vs release (wins) | Paired vs off (wins) | Peak MiB release / on |
|---|---:|---:|---:|---:|
| Original | 31.530 / 31.188 / 30.778 | -1.98% (3/3) | -1.16% (3/3) | 557.9 / 742.6 |
| 57 | 31.467 / 31.146 / 30.905 | -2.00% (3/3) | -1.94% (2/3) | 542.3 / 687.9 |
| 58 | 31.266 / 31.000 / 30.888 | -1.21% (2/3) | -1.13% (3/3) | 568.2 / 709.6 |
| 59 | 31.775 / 31.377 / 30.493 | -3.18% (3/3) | -1.71% (3/3) | 569.7 / 673.9 |
| 60 | 31.232 / 30.856 / 30.755 | -2.79% (2/3) | +0.12% (1/3) | 555.6 / 694.8 |
| 61 | 31.646 / 31.218 / 30.583 | -3.34% (3/3) | -2.03% (2/3) | 578.7 / 771.2 |
| 62 | 31.087 / 31.339 / 30.796 | -1.07% (2/3) | -0.86% (3/3) | 544.6 / 713.3 |
| 63 | 31.440 / 31.254 / 30.438 | -3.02% (3/3) | -1.46% (3/3) | 544.9 / 701.6 |
| 64 | 31.297 / 31.279 / 31.220 | +0.64% (1/3) | -0.34% (2/3) | 572.7 / 673.5 |

The held-out median change was -2.39% vs release (7/8 faster) and -1.30% vs disabled (7/8 faster). The candidate passed the initial full-proof screen. A zero optional allowance exercised ordinary proving. All 84 exports verified independently with Gnark. The [qualified scorecard](../2026-11-19-merge-deep479/README.md) is the reference pending independent confirmation.

[Results and route probes](artifacts/results.json) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Experimental patch](artifacts/source.patch)

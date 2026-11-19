# Reusing point sums in three 12×2 curve queries

The 12×2 B2 groups from the [verified point-sum trial](../2026-11-19-verified-groups482/README.md) were mapped to the original witness wires and intersected with the A and folded private K queries. Each query prepares its own compact basis. Every proof computes the residual contributions for any member whose scalar differs from its representative. The compact queries contain 64,990 of 190,734 A positions, 86,903 of 275,427 K positions, and 82,233 of 270,757 B2 positions before sparse residuals. The existing H computation and qualified wire offsets remain in use.

The production binary, same-build disabled, B2 alone and all three queries each ran three randomized blocks of three primes and eleven timed proofs per request, at 32 workers on Ryzen. Peak RSS covers the whole process.

| Input | Release / off / B2 / all ms | All vs release | All vs off | All vs B2 | Peak MiB release / all |
|---|---:|---:|---:|---:|---:|
| Original | 31.387 / 31.515 / 30.178 / 29.701 | -5.37% (3/3) | -5.72% (3/3) | -1.63% (2/3) | 561.2 / 606.2 |
| 57 | 31.408 / 31.343 / 30.492 / 29.852 | -5.57% (3/3) | -4.76% (3/3) | -2.74% (3/3) | 568.0 / 631.1 |
| 58 | 31.728 / 31.545 / 30.567 / 30.153 | -5.42% (3/3) | -4.59% (3/3) | -0.94% (3/3) | 562.5 / 645.1 |
| 59 | 31.540 / 31.368 / 30.335 / 29.835 | -5.95% (3/3) | -4.20% (3/3) | -1.32% (2/3) | 566.6 / 621.5 |
| 60 | 31.281 / 31.199 / 30.793 / 30.372 | -2.25% (2/3) | -2.85% (2/3) | +0.39% (1/3) | 525.5 / 614.1 |
| 61 | 31.771 / 31.577 / 30.533 / 30.538 | -3.21% (3/3) | -3.57% (3/3) | +0.03% (1/3) | 544.8 / 622.0 |
| 62 | 31.564 / 31.639 / 30.438 / 29.651 | -6.06% (3/3) | -6.44% (3/3) | -2.59% (3/3) | 553.5 / 615.2 |
| 63 | 31.907 / 31.504 / 30.488 / 30.320 | -5.38% (3/3) | -5.25% (3/3) | -1.58% (2/3) | 560.5 / 605.0 |
| 64 | 31.580 / 31.302 / 30.819 / 29.706 | -5.64% (3/3) | -4.43% (3/3) | -3.98% (3/3) | 560.9 / 595.9 |

On the eight held-out requests the median changes were -5.50% vs release, -4.51% vs disabled and -1.45% vs B2 alone. The prototype passed the latency screen. The two instrumented witnesses exercised all three compact queries, and zero optional preparation allowance selected the ordinary route. All 111 exported proofs passed Gnark verification. This is a prototype measurement.

[Full measurements](artifacts/results.json) · [Raw processes](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Patch](artifacts/source.patch)

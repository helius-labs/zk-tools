# Extended 12×2 groups across worker counts and witness populations

The prior correction experiment selected both using median full-proof latency across the original request and adjacent request 59. A four-block screen then compared 24, 26, 28, 30 and 32 Ryzen workers on those two requests. It selected 30 workers. Five new randomized blocks compare this setting with the production release at 32 workers and the prior wide-scalar groups at 28. Requests 57–64 use adjacent leaves and 141–144 use scattered leaves. Each process primes three times and times eleven proofs. Whole-process peak RSS includes preparation.

| Request | Release / wide / extended, ms | Extended vs wide | Extended vs release | Peak MiB release / wide / extended | Original blocks ≤28.4 ms |
|---|---:|---:|---:|---:|---:|
| Original | 31.496 / 29.567 / 29.714 | +0.45% (1/5) | -5.53% (5/5) | 561.5 / 603.3 / 521.3 | 0 |
| 57 | 31.938 / 29.961 / 29.500 | -1.42% (4/5) | -7.76% (5/5) | 566.0 / 622.7 / 509.8 | — |
| 58 | 31.398 / 29.551 / 29.472 | -1.06% (4/5) | -6.14% (5/5) | 555.4 / 592.9 / 511.8 | — |
| 59 | 31.590 / 29.578 / 29.585 | -0.09% (4/5) | -5.91% (5/5) | 586.9 / 620.4 / 529.6 | — |
| 60 | 31.700 / 29.763 / 29.458 | -1.03% (4/5) | -6.47% (5/5) | 568.0 / 604.3 / 519.8 | — |
| 61 | 31.337 / 29.448 / 29.500 | -0.02% (3/5) | -6.54% (5/5) | 590.0 / 621.3 / 511.3 | — |
| 62 | 32.095 / 29.675 / 29.680 | +0.99% (2/5) | -8.53% (5/5) | 590.6 / 607.3 / 533.4 | — |
| 63 | 31.446 / 29.930 / 29.374 | -2.46% (4/5) | -6.59% (5/5) | 566.4 / 616.8 / 522.3 | — |
| 64 | 31.170 / 29.515 / 29.500 | -0.29% (3/5) | -6.24% (5/5) | 573.0 / 619.4 / 522.8 | — |
| 141 | 48.102 / 47.119 / 49.785 | +5.96% (0/5) | +3.91% (0/5) | 991.4 / 988.8 / 982.0 | — |
| 142 | 48.257 / 46.881 / 49.913 | +4.31% (0/5) | +3.27% (1/5) | 971.9 / 1009.4 / 1019.1 | — |
| 143 | 47.522 / 47.530 / 49.360 | +4.12% (0/5) | +4.06% (0/5) | 975.5 / 950.9 / 974.3 | — |
| 144 | 47.661 / 47.812 / 49.971 | +3.57% (0/5) | +3.63% (0/5) | 942.6 / 961.8 / 1028.3 | — |

All 235 process exports verified independently. [Selection](artifacts/selection.json) · [Worker sweep](artifacts/screen.json.gz) · [Confirmation](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

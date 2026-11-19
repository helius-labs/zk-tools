# Adaptive representatives for prepared curve queries

Each group first counts the terms differing from its first scalar. When more than half differ, Boyer–Moore voting proposes a new scalar and an exact recount determines whether it reduces the exceptions. Each group stores its chosen index and count together. Both the main and residual calculations use that choice. At least 256 exceptions enable the duplicate-aware residual query. These thresholds affect scheduling alone. The full ordinary query remains available when reservations or input checks decline the compact route.

Five fresh randomized blocks compare the release, the preceding correction policy, the same build with adaptation disabled and the adaptive policy. The 12×2 candidate uses the previously selected worker count, larger shapes and the release use 32. All cells use three primes and eleven timed proofs. The larger-shape heldouts differ from those used to screen the prior correction policy. Peak RSS includes setup.

| Shape | Request | Release / previous / off / adaptive ms | Adaptive vs off | Adaptive vs previous | Adaptive vs release | Peak MiB release / adaptive |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | Original | 31.912 / 29.862 / 32.084 / 29.152 | -9.14% (5/5) | -2.36% (4/5) | -8.43% (5/5) | 592.2 / 523.7 |
| 12x2 | 59 | 31.560 / 29.741 / 32.173 / 29.796 | -7.92% (5/5) | -1.03% (4/5) | -5.56% (5/5) | 553.7 / 535.5 |
| 12x2 | 141 | 47.929 / 49.295 / 48.065 / 46.907 | -1.89% (5/5) | -4.73% (5/5) | -0.59% (4/5) | 938.4 / 1200.0 |
| 16x2 | Original | 39.681 / 38.289 / 39.190 / 37.639 | -3.96% (5/5) | -1.82% (3/5) | -5.15% (5/5) | 651.6 / 734.2 |
| 16x2 | 74 | 39.512 / 38.297 / 37.190 / 36.712 | -1.53% (3/5) | -4.64% (5/5) | -7.54% (5/5) | 713.7 / 736.7 |
| 16x2 | 79 | 40.194 / 37.546 / 37.653 / 37.094 | -2.40% (5/5) | -1.20% (5/5) | -6.63% (5/5) | 700.9 / 740.0 |
| 24x2 | Original | 64.054 / 60.482 / 63.312 / 59.349 | -6.26% (5/5) | -0.90% (4/5) | -8.04% (5/5) | 1030.9 / 1136.3 |
| 24x2 | 99 | 65.083 / 59.671 / 59.539 / 58.115 | -2.15% (5/5) | -2.20% (5/5) | -11.20% (5/5) | 1056.6 / 1107.1 |
| 24x2 | 103 | 64.570 / 59.764 / 58.089 / 58.453 | -0.18% (3/5) | -1.56% (4/5) | -9.55% (5/5) | 1015.9 / 1133.3 |

All 198 exported proofs verified independently. [Samples](artifacts/processes.json.gz) · [Correction counts](artifacts/probes.json) · [Experimental change](artifacts/incremental.patch) · [Verification](artifacts/verification.json)

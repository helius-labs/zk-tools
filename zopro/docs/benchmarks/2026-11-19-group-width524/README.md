# Worker count and FFT radix on the grouped 12×2 prover

On a Ryzen 9950X, three randomized blocks screened 24, 28, 30 and 32 workers with ordinary and derivative-only radix conversion. All configurations used 16 physical cores and rotated additional SMT siblings equally between the two CCDs. Each used the same nested B2 and broad A/K plans. Selection minimized the equally weighted average of the median paired percentage changes on the original, an adjacent-leaf request and a scattered-leaf request, relative to 32 grouped workers with ordinary conversion.

The worker screen with ordinary radix conversion gave the following three-block medians. Peak RSS includes preparation.

| Workers | Original ms | Original CPU-ms | Original peak MiB | Adjacent 58 ms | Scattered 142 ms |
|---:|---:|---:|---:|---:|---:|
| 24 | 30.451 | 648.9 | 565.0 | 30.104 | 47.320 |
| 28 | 29.249 | 722.8 | 552.5 | 29.569 | 45.387 |
| 30 | 29.340 | 766.0 | 564.0 | 29.345 | 45.575 |
| 32 | 29.379 | 807.8 | 568.5 | 29.118 | 45.634 |

The selected configuration used 32 workers and the derivative radix setting. Five independent confirmation blocks tested the original, two additional adjacent requests and two additional scattered requests. Each process warmed three proofs and timed eleven. CPU time is user plus system time per proof. Peak process RSS includes key setup. The group plans were disabled for the release control.

| Request | Release 32 ms | Grouped 32 ms | Selected ms | Selected vs grouped 32 | CPU-ms grouped 32 / selected | Peak MiB grouped 32 / selected |
|---|---:|---:|---:|---:|---:|---:|
| Original | 31.360 | 29.343 | 29.812 | +0.59% (0/5) | 809.3 / 817.2 | 577.1 / 579.0 |
| 59 | 31.479 | 29.469 | 29.312 | -0.53% (3/5) | 811.6 / 810.8 | 583.8 / 585.7 |
| 63 | 31.573 | 30.144 | 29.401 | -1.65% (4/5) | 822.6 / 815.6 | 601.6 / 574.7 |
| 141 | 47.335 | 45.214 | 46.285 | +3.36% (0/5) | 1241.2 / 1253.4 | 981.3 / 1008.6 |
| 144 | 47.751 | 45.552 | 46.029 | -0.25% (3/5) | 1227.1 / 1232.4 | 1028.3 / 1035.2 |

On the original, 0/5 selected process means reached 28.4 ms. The speedups against the supplied M5 CPU and Metal references were 9.53× and 3.39×. All 147 process exports verified independently.

[Selection](artifacts/selection.json) · [Screen](artifacts/screen.json.gz) · [Confirmation](artifacts/confirmation.json.gz) · [Verification](artifacts/verification.json)

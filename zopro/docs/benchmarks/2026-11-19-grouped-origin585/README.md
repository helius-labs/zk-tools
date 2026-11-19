# Grouping curve queries on preconverted 12×2 Euler keys

The candidate prepares the existing A, K and two-level B2 group indices against the points in each loaded key. The mapped K points include the Euler cross terms. Rebuilding the K sums from those points allows the same group indices to work on either key. The configuration is fixed across the original request, two adjacent transfers and two scattered transfers. Four setup probes checked both shapes of transaction with ordinary preparation and a zero optional-preparation budget. The converted key still has its own decoded row dictionary at zero group budget.

Seven shuffled blocks compare a previously measured grouped executable with four settings of one new executable on the same 30 pinned Ryzen workers. Each process loads one key, primes three full proofs and measures fifteen. Every exported proof was checked with the ordinary key. The paired comparison columns give combined versus ordinary, mapped alone, grouped alone and the previous grouped executable, respectively. Peak process RSS includes setup.

| Request | Prior grouped / ordinary / mapped / grouped / combined ms | Paired changes and wins | Max process MiB grouped / combined | Combined vs M5 CPU / Metal and blocks ≤28.4 ms |
|---|---:|---:|---:|---:|
| Original | 29.286 / 31.385 / 30.546 / 29.521 / 28.624 | -8.80% (7/7) / -5.86% (7/7) / -3.50% (7/7) / -2.86% (6/7) | 596.1 / 604.4 | 9.92× / 3.53× (2/7) |
| 59 | 29.351 / 31.069 / 30.579 / 29.540 / 28.680 | -7.78% (7/7) / -5.60% (7/7) / -2.98% (7/7) / -2.37% (7/7) | 600.4 / 602.5 | — |
| 64 | 29.487 / 31.121 / 30.464 / 29.695 / 29.098 | -6.58% (7/7) / -4.67% (7/7) / -1.83% (6/7) / -1.62% (6/7) | 602.0 / 616.7 | — |
| 141 | 45.489 / 47.071 / 46.779 / 45.647 / 45.635 | -3.96% (7/7) / -2.44% (7/7) / -0.24% (4/7) / +0.32% (2/7) | 1093.1 / 1036.8 | — |
| 142 | 45.194 / 47.067 / 46.657 / 45.707 / 45.303 | -3.95% (7/7) / -2.96% (7/7) / -1.00% (4/7) / +0.04% (3/7) | 1096.0 / 1049.9 | — |

All 179 exports passed verification. [Samples and probe events](artifacts/processes.json.gz) · [Integration](artifacts/integration.patch) · [Binary and key identities](artifacts/pins.json) · [Verification](artifacts/verification.json)

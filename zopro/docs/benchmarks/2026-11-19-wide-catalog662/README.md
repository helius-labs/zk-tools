# Current grouped G2 option across the full Ryzen catalog

The sample covers all 41 catalog shapes, including all eight supplied M5 reference shapes and the 20 shapes that favored Aeglos in the older 96-core C7a comparison. These fresh results use 32 Ryzen hardware threads. All three arms use the catalog's existing per-shape configuration. On 5×16 this experiment isolates G2 in the original FFT configuration. The release-check report separately measured G2 with the packed FFT. The release arm uses the prior executable, while off and on use the new executable without and with the optional G2 decomposition. The range check may leave a shape on its ordinary path. Each case has six balanced timing blocks and a separate sampled profile for each arm. Every timing process warmed three single proofs and timed seven. All 861 independent exports verified.

| Shape | Release / off / on ms | Paired on vs off | Paired on vs release | Peak release / off / on MiB |
|---|---:|---:|---:|---:|
| 4x16 | 20.478 / 20.624 / 20.197 | -2.02% (6/6) | -2.01% (5/6) | 321.8 / 321.2 / 335.0 |
| 5x16 | 24.367 / 24.235 / 23.422 | -3.44% (5/6) | -2.35% (6/6) | 385.8 / 381.5 / 390.2 |
| 16x4 | 40.260 / 40.570 / 39.884 | -1.13% (5/6) | -0.64% (4/6) | 698.5 / 692.5 / 707.8 |
| 12x2 | 31.116 / 30.988 / 30.625 | -1.04% (5/6) | -2.09% (5/6) | 503.7 / 483.5 / 485.9 |
| 40x2 | 91.163 / 92.199 / 92.471 | +0.18% (3/6) | +0.78% (1/6) | 1484.5 / 1550.5 / 1508.5 |
| 1x16 | 13.023 / 13.150 / 13.150 | -0.50% (3/6) | +1.16% (3/6) | 225.2 / 231.0 / 215.0 |
| merge-24x1 | 63.332 / 63.270 / 62.645 | -0.96% (5/6) | -1.09% (6/6) | 966.2 / 965.5 / 990.0 |
| 49x2 | 148.471 / 149.559 / 150.081 | -0.00% (3/6) | +0.69% (1/6) | 1468.6 / 1534.6 / 1495.0 |
| 2x2 | 9.717 / 9.844 / 9.896 | +0.53% (2/6) | +2.03% (1/6) | 103.2 / 106.8 / 104.5 |
| 4x4 | 16.427 / 16.840 / 16.345 | -3.85% (6/6) | -0.76% (4/6) | 254.2 / 259.0 / 262.0 |
| 16x8 | 42.378 / 42.061 / 41.994 | -0.58% (4/6) | -0.34% (3/6) | 699.0 / 700.8 / 699.0 |
| 48x2 | 145.669 / 148.473 / 147.102 | -0.94% (5/6) | +0.78% (1/6) | 1474.2 / 1459.2 / 1443.7 |
| merge-54x1 | 155.135 / 150.956 / 151.659 | -0.35% (5/6) | -2.24% (6/6) | 1661.7 / 1620.6 / 1604.8 |
| 24x2 | 64.455 / 63.138 / 62.028 | -0.94% (4/6) | -3.55% (5/6) | 943.4 / 970.9 / 967.3 |
| 5x8 | 21.544 / 21.859 / 21.491 | -0.73% (4/6) | +0.52% (3/6) | 337.8 / 351.2 / 333.8 |
| 6x8 | 23.107 / 23.537 / 22.912 | -3.22% (5/6) | -0.76% (4/6) | 365.2 / 387.2 / 384.2 |
| 1x8 | 10.757 / 10.704 / 10.681 | -0.10% (3/6) | -1.30% (4/6) | 166.0 / 170.5 / 171.0 |
| 12x8 | 35.538 / 35.479 / 34.841 | -1.80% (4/6) | -1.14% (6/6) | 592.8 / 580.2 / 609.2 |
| 1x4 | 9.517 / 9.605 / 9.618 | -0.30% (4/6) | +1.25% (2/6) | 132.5 / 131.2 / 134.2 |
| 24x4 | 66.649 / 66.761 / 66.631 | -0.48% (4/6) | +0.27% (2/6) | 1039.0 / 1024.0 / 1082.2 |
| 2x16 | 17.214 / 17.196 / 17.103 | -0.20% (3/6) | -1.90% (4/6) | 266.0 / 243.0 / 259.0 |
| 4x2 | 15.438 / 15.203 / 15.013 | -1.07% (5/6) | -3.17% (5/6) | 240.8 / 224.2 / 241.5 |
| 6x2 | 20.915 / 20.937 / 20.946 | +0.28% (3/6) | -0.56% (4/6) | 332.5 / 337.8 / 347.5 |
| 5x2 | 17.718 / 17.978 / 17.359 | -3.43% (5/6) | -2.40% (6/6) | 287.5 / 267.8 / 283.5 |
| 2x4 | 11.218 / 11.259 / 11.285 | +0.79% (2/6) | +0.40% (1/6) | 167.5 / 175.2 / 172.5 |
| 8x16 | 29.580 / 29.114 / 29.029 | -0.44% (4/6) | -1.13% (6/6) | 474.0 / 485.0 / 492.0 |
| 8x8 | 26.191 / 26.607 / 26.401 | -0.28% (4/6) | +1.24% (1/6) | 423.2 / 439.8 / 436.0 |
| 8x4 | 25.081 / 25.430 / 24.827 | -2.47% (5/6) | -1.03% (4/6) | 405.0 / 403.2 / 420.2 |
| 8x2 | 24.611 / 24.166 / 23.689 | -2.32% (5/6) | -2.47% (6/6) | 405.5 / 415.8 / 402.2 |
| 4x8 | 17.740 / 17.770 / 17.307 | -2.30% (5/6) | -2.74% (5/6) | 289.2 / 277.0 / 285.2 |
| 3x8 | 16.083 / 16.104 / 16.042 | -0.44% (4/6) | -0.40% (3/6) | 242.8 / 246.8 / 255.2 |
| 16x2 | 39.809 / 39.506 / 38.760 | -2.13% (5/6) | -2.95% (5/6) | 594.6 / 607.7 / 624.6 |
| 6x4 | 21.928 / 21.980 / 21.581 | -1.21% (4/6) | -1.11% (4/6) | 346.0 / 353.0 / 358.8 |
| 5x4 | 18.379 / 18.502 / 18.173 | -1.37% (4/6) | -0.79% (5/6) | 277.8 / 279.8 / 285.0 |
| 3x2 | 14.128 / 14.199 / 13.901 | -1.90% (6/6) | -1.23% (4/6) | 207.0 / 204.0 / 201.0 |
| 32x2 | 79.636 / 79.879 / 79.237 | -0.60% (5/6) | -0.10% (4/6) | 1285.0 / 1257.2 / 1285.4 |
| 3x4 | 14.955 / 14.932 / 14.646 | -2.30% (4/6) | -2.30% (5/6) | 215.0 / 215.8 / 220.8 |
| 2x8 | 13.910 / 14.248 / 14.251 | -1.08% (4/6) | +1.54% (2/6) | 209.8 / 220.0 / 209.5 |
| 12x4 | 34.919 / 34.512 / 34.241 | -0.47% (4/6) | -2.03% (5/6) | 551.2 / 561.8 / 571.0 |
| 1x2 | 7.123 / 7.199 / 7.200 | +0.01% (3/6) | +0.23% (3/6) | 58.0 / 59.0 / 59.2 |
| merge-8x1 | 24.029 / 23.609 / 23.633 | -0.00% (3/6) | -1.47% (5/6) | 377.2 / 367.0 / 371.8 |

Counts below use the direction of each paired median for latency. CPU time uses the median process CPU-ms per proof. Memory uses the highest whole-process RSS observed in each arm. Six paired blocks and a single request per shape make this a screen for transferability. They do not establish a benefit on every witness or under a different core topology.

| Set | Shapes | On faster than off | On faster than release | Off faster than release | On uses less CPU than off | On has lower peak RSS than off |
|---|---:|---:|---:|---:|---:|---:|
| full catalog | 41 | 36 | 29 | 19 | 37 | 15 |
| historical 96-core losses | 20 | 17 | 13 | 7 | 19 | 6 |

The dedicated CPU profiles sampled the owned G2 function or its closures on 20/41 shapes. Sampling is independent of the latency measurements. An absent sample does not establish that a function was never called.

The largest paired reduction versus the same executable with the option disabled occurred on 4x4 (-3.85%, 6/6 wins). The largest paired increase occurred on 2x4 (+0.79%, 2/6 wins). [Disabled build versus reference](artifacts/default-comparison.json)

## Supplied M5 references

Ratios use the supplied M5 full-proof timing divided by the measured Ryzen median. Merge 54×1 has no supplied M5 reference.

| Circuit | M5 CPU / Metal ms | Release speedup over CPU / Metal | Opt-in speedup over CPU / Metal | 10× CPU target ms |
|---|---:|---:|---:|---:|
| 1x2 | 37 / 24 | 5.19× / 3.37× | 5.14× / 3.33× | 3.7 |
| 2x2 | 59 / 30 | 6.07× / 3.09× | 5.96× / 3.03× | 5.9 |
| 4x2 | 109 / 48 | 7.06× / 3.11× | 7.26× / 3.20× | 10.9 |
| 8x2 | 189 / 74 | 7.68× / 3.01× | 7.98× / 3.12× | 18.9 |
| 12x2 | 284 / 101 | 9.13× / 3.25× | 9.27× / 3.30× | 28.4 |
| 16x2 | 335 / 124 | 8.42× / 3.11× | 8.64× / 3.20× | 33.5 |
| 24x2 | 528 / 176 | 8.19× / 2.73× | 8.51× / 2.84× | 52.8 |
| merge-24x1 | 507 / 175 | 8.01× / 2.76× | 8.09× / 2.79× | 50.7 |

[Source release check](../2026-11-19-owned-phi661/README.md) · [Historic C7a loss definitions](../2026-11-19-c7a-work655/README.md) · [Config and pins](artifacts/configs.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

On an independent 12-block check of the three largest disabled-build regressions, 2×8 remained slower by 1.23% paired. The 6×8 and 4×4 disabled regressions did not repeat. [Independent repeat](../2026-11-19-default-repeat666/README.md)

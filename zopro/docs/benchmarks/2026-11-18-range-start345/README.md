# Releasing G2 windows from independent scalar ranges on C7a

All 1×2 measurements use 16 frontend workers on the same physical core mask as the main pool. Seven randomized fresh processes per setting each time 101 requests on 1×2 or 21 on 2×2. The table reports medians of process means and the paired change versus the same executable using ordinary parallel decomposition with four ranges. The base rows use the integrated release.

At 96 cores the sequential split won all seven pairs, reduced median process latency from 9.017 to 8.485 ms and reduced request p95 from 11.019 to 8.812 ms. Requests taking at least 10 ms fell from 164 to 9 out of 707. Peak RSS was 54 MiB for the sequential split and 477 MiB for Aeglos, which took 7.925 ms. At 32 cores the sequential split lost all seven pairs.

[Stage timing](workers.md) · [G1 feed experiment](g1-feeds.md) · [Previous eight-shape M5 scorecard](../2026-11-18-worker-width340/README.md)

| Shape | Cores | Mode | Wall ms | Paired vs four-range off | Faster blocks | Request p50 / p95 ms | CPU-ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | base2 | 10.794 | -4.16% | 7/7 | 10.750 / 11.405 | 221.4 | 64 |
| 1x2 | 32 | test2 | 10.802 | -4.01% | 7/7 | 10.751 / 11.345 | 221.5 | 67 |
| 1x2 | 32 | base4 | 11.140 | -0.74% | 7/7 | 11.132 / 11.574 | 231.5 | 69 |
| 1x2 | 32 | off | 11.247 | +0.00% | 0/7 | 11.239 / 11.617 | 233.7 | 67 |
| 1x2 | 32 | serial | 11.353 | +0.95% | 0/7 | 11.312 / 11.807 | 231.9 | 66 |
| 1x2 | 32 | range | 11.524 | +2.54% | 0/7 | 11.438 / 12.062 | 235.9 | 69 |
| 1x2 | 64 | base2 | 8.409 | -8.31% | 7/7 | 8.316 / 8.620 | 273.2 | 55 |
| 1x2 | 64 | test2 | 8.409 | -7.91% | 7/7 | 8.324 / 8.643 | 274.4 | 53 |
| 1x2 | 64 | base4 | 9.021 | -1.05% | 5/7 | 8.844 / 9.858 | 294.9 | 54 |
| 1x2 | 64 | off | 9.154 | +0.00% | 0/7 | 8.904 / 9.908 | 296.7 | 60 |
| 1x2 | 64 | serial | 9.018 | -1.49% | 6/7 | 9.000 / 9.361 | 293.6 | 54 |
| 1x2 | 64 | range | 9.361 | +2.32% | 0/7 | 9.330 / 9.757 | 305.4 | 55 |
| 1x2 | 96 | base2 | 10.137 | +12.25% | 0/7 | 8.905 / 12.249 | 406.3 | 54 |
| 1x2 | 96 | test2 | 9.842 | +8.60% | 0/7 | 8.823 / 12.246 | 398.4 | 56 |
| 1x2 | 96 | base4 | 9.027 | -0.50% | 4/7 | 8.504 / 11.053 | 408.1 | 52 |
| 1x2 | 96 | off | 9.017 | +0.00% | 0/7 | 8.515 / 11.019 | 408.7 | 51 |
| 1x2 | 96 | serial | 8.485 | -5.59% | 7/7 | 8.467 / 8.812 | 386.0 | 54 |
| 1x2 | 96 | range | 9.092 | +0.03% | 3/7 | 8.768 / 11.399 | 417.0 | 51 |
| 1x2 | 96 | aeg96 | 7.925 | -12.19% | 7/7 | 7.889 / 8.506 | 508.9 | 477 |
| 2x2 | 48 | base4 | 13.631 | -0.18% | 4/7 | 13.558 / 14.360 | 351.9 | 110 |
| 2x2 | 48 | off | 13.641 | +0.00% | 0/7 | 13.573 / 14.460 | 352.1 | 100 |
| 2x2 | 48 | serial | 13.745 | +0.54% | 1/7 | 13.620 / 14.859 | 348.0 | 101 |
| 2x2 | 48 | range | 14.210 | +3.85% | 0/7 | 14.055 / 15.231 | 359.7 | 105 |

[Protocol](artifacts/protocol.md) · [Primary samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

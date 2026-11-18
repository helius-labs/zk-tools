# Full-shape C7a comparison of K/H scheduling and G2 point ranges

Each latency is the median of seven fresh process means for warm sequential SDK requests on one c7a.24xlarge. Worker counts are physical cores. Aeglos used its prepared key at 96 cores throughout. Zopro used the widths shown. Each process produced a distinct proof verified on the instance and again after retrieval.

On 96 cores, accepted Zopro took 12.479 ms on 1×2 versus 7.895 ms for Aeglos. The experimental G2 geometry reached 9.857 ms and replicated its gain over the same binary with the geometry disabled. Accepted Zopro took 123.721 ms on merge54 versus 171.052 ms for Aeglos, at 1,899 versus 13,023 MiB peak RSS. The separate K/H scheduling change regressed on accepted-source 1×2 and stays disabled. The fastest measured 1×2 Zopro configuration was experimental K/H scheduling at 64 cores, 9.216 ms with zero of seven block means at or below 9 ms.

None of the eight accepted-build medians reached the corresponding 10× M5 CPU target on this host. The closest was 24×2, 53.781 ms versus 52.8 ms. This shows the margin on the two earlier qualifying runs is sensitive to the host.

| Shape | Zopro cores | Accepted ms | Candidate off ms | Candidate on ms | Aeglos ms | Peak MiB accepted / on / Aeglos | Accepted M5 CPU / Metal speedup | On M5 CPU / Metal speedup | 10× CPU target ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 11.124 | 11.032 | 11.364 | 7.895 | 120 / 118 / 478 | 3.33× / 2.16× | 3.26× / 2.11× | 3.7 |
| 1x2 | 96 | 12.479 | 12.279 | 12.675 | 7.895 | 112 / 113 / 478 | 2.96× / 1.92× | 2.92× / 1.89× | 3.7 |
| 2x2 | 48 | 14.149 | 14.150 | 14.143 | 11.496 | 300 / 294 / 829 | 4.17× / 2.12× | 4.17× / 2.12× | 5.9 |
| 4x2 | 48 | 18.055 | 17.930 | 18.070 | 18.376 | 262 / 273 / 1351 | 6.04× / 2.66× | 6.03× / 2.66× | 10.9 |
| 8x2 | 48 | 24.532 | 24.494 | 24.411 | 30.815 | 446 / 449 / 2486 | 7.70× / 3.02× | 7.74× / 3.03× | 18.9 |
| 12x2 | 48 | 32.805 | 32.704 | 32.451 | 48.299 | 640 / 643 / 3698 | 8.66× / 3.08× | 8.75× / 3.11× | 28.4 |
| 16x2 | 48 | 37.003 | 37.172 | 36.580 | 56.387 | 787 / 747 / 4588 | 9.05× / 3.35× | 9.16× / 3.39× | 33.5 |
| 24x2 | 48 | 53.781 | 54.648 | 53.826 | 87.126 | 1187 / 1178 / 6765 | 9.82× / 3.27× | 9.81× / 3.27× | 52.8 |
| merge-24x1 | 48 | 55.934 | 55.645 | 55.434 | 83.989 | 1093 / 1105 / 6600 | 9.06× / 3.13× | 9.15× / 3.16× | 50.7 |
| merge-54x1 | 96 | 123.721 | 122.977 | 121.985 | 171.052 | 1899 / 1805 / 13023 | — | — | — |

## Prespecified decisions

Each test requires a ≥2% median paired improvement over disabled, at least five paired wins, a lower median than the retained reference, and disabled/reference calibration within 5%. The production-source K/H candidate uses its own rebuilt control and the accepted SDK. The two G2 tests use the exact experimental binaries from the previous host.

| Shape / cores | Candidate | Candidate / disabled / reference ms | Paired change | Wins | Control calibration | Passed |
|---|---|---:|---:|---:|---:|---|
| 1x2 / 64 | prod-on | 11.364 / 11.032 / 11.124 | +4.87% | 1/7 | -1.66% | False |
| 1x2 / 96 | 11x4 | 9.857 / 10.110 / 10.028 | -2.50% | 7/7 | +0.81% | True |
| 2x2 / 48 | 11x4 | 13.529 / 13.959 / 13.899 | -3.32% | 7/7 | +0.43% | True |

## Small-shape diagnostics

| Shape | Width | Mode | Median ms | Median CPU ms/proof | Max RSS MiB | Block means ≤9 ms |
|---|---:|---|---:|---:|---:|---:|
| 1x2 | 64 | accepted | 11.124 | 402 | 120 | 0/7 |
| 1x2 | 64 | prod-off | 11.032 | 384 | 119 | 0/7 |
| 1x2 | 64 | prod-on | 11.364 | 397 | 118 | 0/7 |
| 1x2 | 64 | ref | 9.589 | 315 | 73 | 0/7 |
| 1x2 | 64 | split | 9.216 | 313 | 88 | 0/7 |
| 1x2 | 96 | 11x4 | 9.857 | 467 | 69 | 0/7 |
| 1x2 | 96 | accepted | 12.479 | 690 | 112 | 0/7 |
| 1x2 | 96 | aeglos | 7.895 | 511 | 478 | 7/7 |
| 1x2 | 96 | geom-off | 10.110 | 476 | 69 | 0/7 |
| 1x2 | 96 | prod-off | 12.279 | 626 | 117 | 0/7 |
| 1x2 | 96 | prod-on | 12.675 | 658 | 113 | 0/7 |
| 1x2 | 96 | ref | 10.028 | 452 | 77 | 0/7 |
| 1x2 | 96 | split | 9.989 | 475 | 72 | 0/7 |
| 2x2 | 48 | 11x4 | 13.529 | 386 | 304 | 0/7 |
| 2x2 | 48 | accepted | 14.149 | 412 | 300 | 0/7 |
| 2x2 | 48 | geom-off | 13.959 | 378 | 313 | 0/7 |
| 2x2 | 48 | prod-off | 14.150 | 410 | 303 | 0/7 |
| 2x2 | 48 | prod-on | 14.143 | 410 | 294 | 0/7 |
| 2x2 | 48 | ref | 13.899 | 378 | 301 | 0/7 |
| 2x2 | 96 | aeglos | 11.496 | 825 | 829 | 0/7 |

For the six larger supplied shapes, each process timed 17 requests. The two small shapes timed 101 each and merge54 timed seven. All processes performed three warmups. Each peak includes setup. Comparisons between engines describe these specific worker counts.

[Block means and request times](artifacts/rows.json.gz) · [Decision statistics](artifacts/checks.json) · [Protocol](artifacts/protocol.md) · [Verification and cleanup](artifacts/verification.json)

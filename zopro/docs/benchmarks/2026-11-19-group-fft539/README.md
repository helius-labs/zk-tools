# Computing the prepared B2 query while the Euler FFT runs

The FFT and prepared B2 query consume different scalar inputs after solving. Two schedules run them on the same Rayon pool with opposite join order, then consume the computed B2 point alongside the other queries. A declined group plan uses the existing exact fallback. An optional reservation covers the additional FFT buffer lifetime during overlap. Zero preparation allowance selects the sequential route. The separate frozen control uses the preceding executable, with the same scalar view and group plans throughout.

Five randomized blocks use 32 Ryzen workers, three warmups and eleven timed calls per process. Independent route probes confirm both overlap orders and the low-budget path. Peak RSS includes setup.

| Shape | Input | Frozen / ordinary / FFT-first / B2-first ms | Paired FFT-first and B2-first vs ordinary, then vs frozen | Peak MiB in mode order |
|---|---|---:|---|---:|
| 12x2 | Original | 29.434 / 29.113 / 31.470 / 31.719 | +7.75% (0/5) / +8.82% (0/5) / +7.90% (0/5) / +7.91% (0/5) | 532 / 544 / 700 / 570 |
| 24x2 | Original | 59.762 / 59.749 / 66.145 / 67.652 | +10.73% (0/5) / +13.23% (0/5) / +10.70% (0/5) / +13.20% (0/5) | 1126 / 1132 / 972 / 1015 |
| 12x2 | 142 | 45.873 / 45.874 / 50.979 / 52.809 | +10.94% (0/5) / +14.52% (0/5) / +11.19% (0/5) / +14.16% (0/5) | 1026 / 998 / 876 / 799 |
| 24x2 | 103 | 58.574 / 59.700 / 64.662 / 64.886 | +10.08% (0/5) / +10.22% (0/5) / +8.89% (1/5) / +10.78% (1/5) | 1108 / 1091 / 1019 / 1014 |

Original FFT-first speedup against M5 CPU and Metal was 12x2 9.02× / 3.21×, 24x2 7.98× / 2.66×. All 92 exports verified. [Samples](artifacts/samples.json.gz) · [Route probes](artifacts/routes.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## CPU use during the timed proof loop

User plus system CPU time divided by elapsed loop time gives the average number of busy logical CPUs for each process. The table takes medians over the five processes in each cell. Setup, priming and export are outside this interval.

| Shape | Input | CPU-ms per proof ordinary / FFT-first / B2-first | Average busy logical CPUs in the same order |
|---|---|---:|---:|
| 12x2 | Original | 808.7 / 835.5 / 834.5 | 27.76 / 26.55 / 26.31 |
| 12x2 | 142 | 1247.0 / 1240.7 / 1241.1 | 27.08 / 24.37 / 23.52 |
| 24x2 | Original | 1675.8 / 1701.7 / 1707.4 | 28.05 / 25.72 / 25.24 |
| 24x2 | 103 | 1662.8 / 1658.4 / 1657.8 | 27.88 / 25.65 / 25.54 |

Both overlap orders lost all 20 matched blocks against ordinary execution. Average CPU occupancy also fell in every request and order. These counters locate a scheduling tradeoff without identifying a particular wait or hardware bottleneck. [CPU and wall-time summary](artifacts/occupancy.json)

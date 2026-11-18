# Fresh comparison of interpreter placement at 96 cores

The 48-worker interpreter was selected after early primary results on merge24. These are seven subsequently randomized blocks from the same C7a, each measuring 21 full SDK requests after three primes. An idle pool lets us separate the effect of routing work to it from the cost of creating its threads. The ordinary release uses the main pool only.

| Shape | Main cores | Interpreter setup | Full proof ms | Aggregate CPU-ms/proof | Peak MiB | M5 CPU / Metal speedup |
|---|---:|---|---:|---:|---:|---:|
| 16x2 | 64 | Integrated release | 34.090 | 1671.3 | 795 | 9.83× / 3.64× |
| 16x2 | 96 | Integrated release | 36.841 | 2500.0 | 821 | 9.09× / 3.37× |
| 16x2 | 96 | Experimental, no auxiliary pool | 36.467 | 2508.5 | 789 | 9.19× / 3.40× |
| 16x2 | 96 | Experimental, 48 auxiliary workers idle | 36.639 | 2479.6 | 803 | 9.14× / 3.38× |
| 16x2 | 96 | Experimental, 48 workers executing instructions | 35.914 | 2167.9 | 734 | 9.33× / 3.45× |
| merge-24x1 | 64 | Integrated release | 48.541 | 2292.5 | 1143 | 10.44× / 3.61× |
| merge-24x1 | 96 | Integrated release | 52.571 | 3414.5 | 1156 | 9.64× / 3.33× |
| merge-24x1 | 96 | Experimental, no auxiliary pool | 52.365 | 3394.2 | 1157 | 9.68× / 3.34× |
| merge-24x1 | 96 | Experimental, 48 auxiliary workers idle | 53.453 | 3465.2 | 1174 | 9.49× / 3.27× |
| merge-24x1 | 96 | Experimental, 48 workers executing instructions | 50.440 | 2976.3 | 1046 | 10.05× / 3.47× |


| Shape | Active 96 vs control | Paired latency change | Faster blocks |
|---|---|---:|---:|
| 16x2 | 96 cores, idle48 | -1.98% | 6/7 |
| 16x2 | 96 cores, off | -1.37% | 5/7 |
| 16x2 | 96 cores, e8 | -2.39% | 6/7 |
| 16x2 | 64 cores, e8 | +5.74% | 0/7 |
| merge-24x1 | 96 cores, idle48 | -5.64% | 7/7 |
| merge-24x1 | 96 cores, off | -3.68% | 7/7 |
| merge-24x1 | 96 cores, e8 | -3.96% | 7/7 |
| merge-24x1 | 64 cores, e8 | +3.30% | 1/7 |

[Confirmation protocol](artifacts/confirm-protocol.md) · [Primary comparison](team.md)

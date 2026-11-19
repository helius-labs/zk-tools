# Warm scheduler and hardware profiles with shorter idle searches

Three randomized blocks each collected hardware counters and sampled user cycles in separate processes. The 32-worker Ryzen pool used 32, five or seventeen idle rounds for ordinary, 128 and 512 respectively. The eight-worker frontend on 1×2 retained its default sleep policy. Perf began after three warmups and stopped immediately after 61 small or 21 large verified SDK calls. The sample buckets identify named Crossbeam and Rayon-core symbols and exclude other symbols that may contain inlined scheduler work.

| Shape | Nominal probe budget | Stat SDK ms | CPU-ms | Million user cycles / instructions | Switches / minor faults | Crossbeam / other Rayon-core samples |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | off | 7.075 | 145.1 | 648.5 / 1212.9 | 551 / 46 | 7.1% / 4.7% |
| 1x2 | 128 | 7.162 | 138.6 | 618.1 / 1192.2 | 606 / 51 | 2.7% / 5.1% |
| 1x2 | 512 | 7.108 | 141.9 | 634.1 / 1203.0 | 544 / 50 | 4.2% / 4.7% |
| 16x2 | off | 39.460 | 1125.0 | 5168.6 / 5394.9 | 963 / 2402 | 4.9% / 4.6% |
| 16x2 | 128 | 40.517 | 1112.3 | 5096.8 / 5353.2 | 3887 / 1048 | 2.9% / 4.8% |
| 16x2 | 512 | 40.548 | 1116.5 | 5138.3 / 5378.6 | 1595 / 2253 | 4.1% / 5.0% |
| merge-24x1 | off | 63.602 | 1810.2 | 8375.0 / 7895.2 | 1249 / 3847 | 4.0% / 3.2% |
| merge-24x1 | 128 | 65.463 | 1794.3 | 8258.3 / 7842.1 | 5110 / 2056 | 2.3% / 3.7% |
| merge-24x1 | 512 | 64.633 | 1799.4 | 8323.0 / 7880.1 | 2155 / 4499 | 3.6% / 3.9% |

All 54 process exports verified. [Counters and leading symbols](artifacts/summary.json) · [Process records](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

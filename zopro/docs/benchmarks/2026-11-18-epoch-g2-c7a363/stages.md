# Where large proofs spend time at 64 and 96 cores

Three traced and three plain processes per setting each time eleven SDK requests. The probes are attached to a byte-identical diagnostic copy of the integrated release. Merge24 uses eight epochs and 32 or 48 interpreter workers at 64 or 96 cores. For 16×2, six epochs use the main pool. The solver timer includes allocation, instruction execution and completion. Values are medians across 33 traced requests. Intervals from different stages can overlap. Each stage uses the union of its calls inside each request. A dash means that the linked symbol had no measured calls.

| Shape | Cores | Traced vs plain | SDK ms | General solver call ms | FFT ms | H query ms | Through FFT ms | After FFT ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | +8.64% | 35.914 | 5.308 | 6.391 | 17.876 | 14.329 | 21.762 |
| 16x2 | 96 | +4.85% | 37.800 | 6.319 | 6.527 | 17.013 | 15.946 | 21.902 |
| merge-24x1 | 64 | +4.68% | 50.321 | 10.067 | 10.231 | 23.294 | 22.864 | 27.384 |
| merge-24x1 | 96 | +4.96% | 52.795 | 10.920 | 9.596 | 22.289 | 23.705 | 29.490 |

CPU-clock sampling covers userspace instructions across all threads during the indicated wall interval. Each sample represents approximately two CPU-ms. These estimates include tasks running concurrently with the observed stage.

| Shape | Cores | Stage | User CPU-ms/proof | Crossbeam CPU-ms | Rayon coordination CPU-ms |
|---|---:|---|---:|---:|---:|
| 16x2 | 64 | sdk | 1665.0 | 428.1 | 36.8 |
| 16x2 | 64 | denseh | 845.9 | 100.1 | 7.9 |
| 16x2 | 64 | throughfft | 684.7 | 283.4 | 26.4 |
| 16x2 | 64 | afterfft | 980.4 | 144.7 | 10.4 |
| 16x2 | 96 | sdk | 2498.4 | 1026.0 | 75.8 |
| 16x2 | 96 | denseh | 1074.2 | 255.9 | 18.0 |
| 16x2 | 96 | throughfft | 1162.1 | 638.5 | 54.4 |
| 16x2 | 96 | afterfft | 1336.3 | 387.5 | 21.5 |
| merge-24x1 | 64 | sdk | 2047.6 | 271.7 | 39.0 |
| merge-24x1 | 64 | denseh | 1077.4 | 98.8 | 9.6 |
| merge-24x1 | 64 | throughfft | 819.9 | 142.6 | 26.9 |
| merge-24x1 | 64 | afterfft | 1227.6 | 129.1 | 12.1 |
| merge-24x1 | 96 | sdk | 2875.2 | 735.2 | 99.6 |
| merge-24x1 | 96 | denseh | 1359.6 | 272.1 | 27.2 |
| merge-24x1 | 96 | throughfft | 1207.6 | 347.0 | 63.4 |
| merge-24x1 | 96 | afterfft | 1667.6 | 388.1 | 36.2 |

[Timing samples](artifacts/stages.json.gz) · [CPU samples and symbols](artifacts/stage-samples.json) · [Protocol](artifacts/stage-protocol.md) · [Verification](artifacts/stage-verification.json)

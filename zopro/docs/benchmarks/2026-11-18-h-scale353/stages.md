# Where large proofs spend time at 64 and 96 cores

Three traced and three plain processes per setting each time eleven SDK requests. The probes are attached only to a diagnostic copy of the integrated release. Values are medians across 33 traced requests. Intervals from different stages can overlap. Each stage uses the union of its calls inside each request. A dash means that the linked symbol had no measured calls.

| Shape | Cores | Traced vs plain | SDK ms | Solver interpreter ms | FFT ms | H query ms | Through FFT ms | After FFT ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | +3.92% | 38.447 | 7.228 | 5.993 | 19.659 | 15.716 | 22.866 |
| 16x2 | 96 | +3.33% | 41.210 | 9.474 | 5.682 | 20.886 | 18.088 | 23.097 |
| merge-24x1 | 64 | +1.78% | 55.724 | 12.603 | 10.295 | 27.628 | 25.854 | 29.428 |
| merge-24x1 | 96 | +2.98% | 60.530 | 16.626 | 9.569 | 29.177 | 29.418 | 30.958 |

On merge24, the interpreter alone grew from 12.603 to 16.626 ms while the FFT fell from 10.295 to 9.569 ms. On 16×2 the interpreter also grew by 2.246 ms. These same-executable profiles isolate a larger source of the wide-pool latency increase than the FFT.

CPU-clock sampling covers userspace instructions across all threads during the indicated wall interval. Each sample represents approximately two CPU-ms. These estimates include tasks running concurrently with the observed stage.

| Shape | Cores | Stage | User CPU-ms/proof | Crossbeam CPU-ms | Rayon coordination CPU-ms |
|---|---:|---|---:|---:|---:|
| 16x2 | 64 | sdk | 1667.2 | 444.2 | 40.2 |
| 16x2 | 64 | denseh | 873.6 | 94.2 | 9.5 |
| 16x2 | 64 | throughfft | 761.8 | 331.6 | 30.1 |
| 16x2 | 64 | afterfft | 905.3 | 112.7 | 10.1 |
| 16x2 | 96 | sdk | 2531.9 | 1060.0 | 95.0 |
| 16x2 | 96 | denseh | 1171.3 | 275.9 | 26.1 |
| 16x2 | 96 | throughfft | 1309.2 | 747.9 | 67.7 |
| 16x2 | 96 | afterfft | 1222.7 | 312.1 | 27.3 |
| merge-24x1 | 64 | sdk | 2294.5 | 529.2 | 51.4 |
| merge-24x1 | 64 | denseh | 1158.9 | 119.4 | 13.3 |
| merge-24x1 | 64 | throughfft | 1121.5 | 400.4 | 37.9 |
| merge-24x1 | 64 | afterfft | 1173.1 | 128.8 | 13.5 |
| merge-24x1 | 96 | sdk | 3459.6 | 1351.2 | 116.6 |
| merge-24x1 | 96 | denseh | 1539.7 | 347.5 | 31.8 |
| merge-24x1 | 96 | throughfft | 1897.9 | 985.6 | 83.9 |
| merge-24x1 | 96 | afterfft | 1561.7 | 365.6 | 32.7 |

[Timing samples](artifacts/stages.json.gz) · [CPU samples and symbols](artifacts/stage-samples.json) · [Protocol](artifacts/stage-protocol.md) · [Verification](artifacts/stage-verification.json)

# Worker-trace calibration for G2 scaling

Local Ryzen calibration used 32 logical workers and eight frontend workers. Three fresh processes per geometry ran 11 timed proofs each for both the plain and instrumented paths. The instrumented process records entries and returns for G2 windows and their range tasks, plus 500 Hz userspace CPU-clock samples.

| Geometry | Plain ms | Traced ms | Paired observer shift | Median distinct range workers per proof |
|---|---:|---:|---:|---:|
| 12x2 | 7.617 | 7.674 | +0.63% | 14 |
| 11x4 | 7.878 | 7.925 | +1.12% | 25 |

All 66 traced requests contained the exact number of G2 window and range entry/exit events, and all 12 process exports verified. A global recording frequency in the first prototype sampled away probe events. Applying the sampling period only to CPU-clock events preserved complete task counts. Instrumented latency is reported separately because the observer changes execution time.

[Measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

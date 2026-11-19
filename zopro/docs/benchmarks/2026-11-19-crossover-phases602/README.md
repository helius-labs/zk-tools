# Crossover phase measurements on Ryzen with the archived C7a executables

The local process medians include five independently verified processes per arm on 32 Ryzen hardware threads. These historical SDK profiles run with the same configuration as the original C7a catalog at 32 cores. The two extra shapes have no supplied M5 reference.

| Shape | Zopro / Aeglos ms | Traced Zopro ms | Paired trace effect | Max Zopro / Aeglos MiB | Zopro vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 5x16 | 24.329 / 70.412 | 24.650 | +0.32% | 409.5 / 1890.9 | — |
| 6x2 | 21.622 / 69.189 | 21.483 | +0.94% | 370.5 / 1856.3 | — |
| 4x2 | 15.758 / 38.099 | 15.866 | +0.73% | 252.0 / 1267.9 | 6.92× / 3.05× |
| 8x2 | 24.001 / 85.648 | 24.461 | +2.30% | 447.2 / 2292.9 | 7.87× / 3.08× |

The intervals below are arithmetic means of 55 traced requests for each shape. Calibration compares full traced and untraced requests. The last interval contains the remaining curve queries, proof assembly and verification. The trace times several work units that may run concurrently inside that interval.

| Shape | SDK entry to solve ms | Solve ms | Solve to FFT ms | FFT ms | FFT return to SDK return ms | Within 5% calibration |
|---|---:|---:|---:|---:|---:|---|
| 5x16 | 0.478 | 3.129 | 0.607 | 4.451 | 16.084 | yes |
| 6x2 | 0.437 | 2.864 | 0.581 | 4.490 | 13.148 | yes |
| 4x2 | 0.355 | 2.383 | 0.384 | 2.661 | 10.095 | yes |
| 8x2 | 0.501 | 3.425 | 0.776 | 4.606 | 15.303 | yes |

All 60 process exports and the preceding smoke export verified independently. [Run protocol](protocol.md) · [Intervals and samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)

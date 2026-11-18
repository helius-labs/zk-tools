# Where time goes as core count increases

The exact integrated 1×2 SDK was sampled in three blocks of 51 requests per process. Each trace has a paired plain run. Times show median inclusive durations over the 153 traced requests. Each request made three Miller loop calls. The Miller column sums their durations, which can overlap. The interpreter, FFT and G2 intervals may be nested inside the canonical core. A dash means the symbol had no completed call inside a measured SDK request. The primary timings in the main report were uninstrumented.

| Cores | Ranges | Traced vs plain | SDK ms | Witness ms | Interpreter ms | FFT ms | G2 ms | Miller loop ms | Final exponentiation ms |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | two | +4.74% | 8.776 | 0.125 | 1.836 | 0.811 | 4.067 | 0.512 | 0.302 |
| 80 | serial | +3.59% | 9.136 | 0.132 | 1.845 | 0.820 | 4.335 | 0.525 | 0.303 |
| 88 | serial | +3.51% | 8.852 | 0.137 | 1.894 | 0.821 | 3.946 | 0.549 | 0.308 |
| 96 | serial | +4.58% | 8.838 | 0.137 | 1.933 | 0.830 | 3.822 | 0.557 | 0.312 |
| 96 | two | +5.75% | 10.315 | 0.130 | 1.873 | 0.829 | 4.338 | 0.532 | 0.300 |

A 500 Hz userspace CPU-clock sample represents roughly two aggregate CPU-ms. The following estimates cover all threads during the SDK call. Worker scheduling often overlaps arithmetic on other cores. Sampling does not establish the recoverable wall time.

| Cores | Ranges | Estimated user CPU-ms / proof | In Crossbeam | In Rayon worker coordination | Samples |
|---:|---|---:|---:|---:|---:|
| 64 | two | 263.9 | 55.7 | 5.9 | 20187 |
| 80 | serial | 315.4 | 91.1 | 7.0 | 24125 |
| 88 | serial | 335.6 | 106.1 | 7.6 | 25672 |
| 96 | serial | 366.5 | 134.6 | 8.8 | 28037 |
| 96 | two | 390.0 | 161.4 | 11.8 | 29836 |

On the tuned 96-core run, 74.1 of the 78.9 extra sampled Crossbeam CPU-ms per request occurred between FFT return and canonical core return. Over that same interval median elapsed time fell from 4.605 to 4.470 ms. The busiest sampled Crossbeam instructions included epoch pinning and advancement. This locates the CPU cost while preserving the separate wall-time comparison.

[Early samples versus remaining latency](early-samples.md) · [Trace and sampling protocol](artifacts/prefix-protocol.md) · [Per-request marker timings](artifacts/prefix-traces.json.gz) · [CPU samples by stage](artifacts/prefix-samples.json) · [Proof verification](artifacts/prefix-verification.json)

# Curve completion order with adaptive group representatives

Four matched blocks compare the uninstrumented adaptive prover from experiment 504 with a build that records each query return. The same public input, group plan, CPU mask, three warmups and eleven timed proofs are used within each pair. The 12×2 processes use 30 Ryzen workers, and 16×2 and 24×2 use 32. The timestamp for B2 includes its group and residual handling. The interval between the last and penultimate query returns measures the exposed end of their parallel execution. Instrumentation can change scheduling.

| Shape | Request | Plain / traced ms | Paired tracing effect | Last A / K / H / B2 of 44 | Median gap ms |
|---|---|---:|---:|---:|---:|
| 12x2 | Original | 29.495 / 29.243 | -1.42% | 4 / 5 / 30 / 5 | 0.081 |
| 12x2 | 59 | 29.308 / 29.169 | -0.26% | 2 / 2 / 34 / 6 | 0.070 |
| 12x2 | 141 | 46.876 / 46.790 | +0.34% | 6 / 13 / 0 / 25 | 0.098 |
| 16x2 | Original | 37.144 / 36.722 | -1.45% | 0 / 4 / 34 / 6 | 0.171 |
| 24x2 | Original | 58.721 / 57.958 | -1.49% | 0 / 0 / 42 / 2 | 0.094 |

All 40 process exports verified independently. [Raw timings](artifacts/processes.json.gz) · [Instrumentation](artifacts/incremental.patch) · [Verification](artifacts/verification.json)

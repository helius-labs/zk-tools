# Request order on a warm 12×2 prover

Each process loads one key and proves three distinct warmup requests followed by 25 measured calls. Each of five requests occurs five times. The mixed order changes request every call. The consecutive order presents five identical requests in a row. Both use the same three warmups within each block. Five blocks rotate CPU placement and shuffle the four processes. Timing encloses the full verified SDK call. Each process keeps its 25 outputs in memory until timing ends. All 500 proofs passed independent verification against the original key and their respective public outputs.

The five request identities test order and reuse within a process. Scattered input 139 was one of the four public requests used earlier to refine the second grouping plan; scattered input 143 was held out from that training. The speed ratios below use the separate original M5 reference request. [Training and evaluation inputs](../2026-11-19-nested-groups517/README.md)

Medians below summarize five process means. The last column gives the paired change in latency with mixed order and the count of blocks in which mixed order was faster. The all-input row weights the five inputs equally.

| Key and solver | Request | Mixed ms | Consecutive ms | Paired mixed vs consecutive |
|---|---|---:|---:|---:|
| grouped-flat | all | 35.417 | 35.129 | +0.88% (1/5) |
| grouped-flat | 0 | 28.859 | 28.210 | +2.37% (1/5) |
| grouped-flat | 57 | 28.836 | 28.281 | +1.53% (2/5) |
| grouped-flat | 61 | 28.497 | 28.232 | +0.49% (2/5) |
| grouped-flat | 139 | 45.431 | 45.475 | +0.80% (2/5) |
| grouped-flat | 143 | 45.876 | 45.478 | +0.29% (2/5) |
| grouped-early | all | 34.889 | 35.009 | -0.55% (3/5) |
| grouped-early | 0 | 27.916 | 28.084 | -0.49% (3/5) |
| grouped-early | 57 | 28.264 | 28.117 | +0.30% (2/5) |
| grouped-early | 61 | 28.060 | 28.447 | -0.94% (4/5) |
| grouped-early | 139 | 44.191 | 44.894 | -0.74% (3/5) |
| grouped-early | 143 | 44.833 | 45.734 | -2.28% (4/5) |

[Samples](artifacts/samples.json.gz) · [Schedule](artifacts/schedule.json) · [SDK harness](artifacts/cpu-stream.rs) · [Identities](artifacts/pins.json) · [Verification](artifacts/verification.json)

## Effect of changing request on the next proof

The mixed sequence permits a comparison at the same destination request after two different kinds of predecessor. Each comparison below uses only blocks with observations in both groups. Within each eligible block it subtracts the mean after an original or adjacent request from the mean after a scattered request. Positive values indicate a slower next proof following a scattered input. The predecessor classes reflect these five fixtures. The design does not isolate a particular hardware cache or solver mechanism.

| Configuration | Destination | Samples after original or adjacent / scattered | Eligible blocks | Median difference ms | Within-block range ms |
|---|---|---:|---:|---:|---:|
| grouped-flat | Original | 13 / 12 | 5/5 | +0.205 | -0.395 to +0.583 |
| grouped-flat | 57 | 12 / 13 | 4/5 | -0.080 | -1.378 to +0.372 |
| grouped-flat | 61 | 13 / 12 | 5/5 | -0.357 | -0.794 to +0.519 |
| grouped-flat | 139 | 18 / 7 | 4/5 | +0.311 | -1.315 to +3.696 |
| grouped-flat | 143 | 19 / 6 | 4/5 | +0.567 | -2.293 to +2.342 |
| grouped-early | Original | 13 / 12 | 5/5 | +0.001 | -0.362 to +0.315 |
| grouped-early | 57 | 12 / 13 | 4/5 | +0.390 | +0.308 to +0.667 |
| grouped-early | 61 | 13 / 12 | 5/5 | +0.073 | -0.731 to +0.874 |
| grouped-early | 139 | 18 / 7 | 4/5 | +0.043 | -2.601 to +1.669 |
| grouped-early | 143 | 19 / 6 | 4/5 | -0.622 | -1.258 to +0.432 |

[Per-call predecessor and duration](artifacts/transitions.json.gz)

## M5 reference result

The original has a 284 ms M5 CPU reference and a 101 ms Metal reference. Each process contributes the mean of all five timed occurrences of the original. The five process means have equal weight. Peak memory covers the whole five-input process.

| Configuration | Order | Original means per block, ms | CPU / Metal speedup | Processes at 28.4 ms | Individual calls at 28.4 ms | Maximum peak MiB |
|---|---|---|---:|---:|---:|---:|
| grouped-flat | mixed | 28.402 / 28.859 / 28.904 / 29.417 / 28.722 | 9.84× / 3.50× | 0/5 | 9/25 | 983.6 |
| grouped-flat | consecutive | 28.541 / 28.192 / 28.210 / 28.342 / 28.087 | 10.07× / 3.58× | 4/5 | 17/25 | 993.0 |
| grouped-early | mixed | 28.523 / 27.916 / 27.740 / 27.862 / 27.945 | 10.17× / 3.62× | 4/5 | 23/25 | 934.4 |
| grouped-early | consecutive | 28.228 / 28.055 / 28.360 / 28.084 / 27.754 | 10.11× / 3.60× | 5/5 | 19/25 | 994.8 |

The early setting crossed the 10× CPU threshold on this reference in both request orders. The preceding original-only SDK run measured 28.495 ms across seven processes. These workloads differ in request history, calls per process and output retention. [Original-only comparison](../2026-11-19-early-blocks588/README.md) · [Independent confirmation protocol](../2026-11-19-reference-recheck592/protocol.md)

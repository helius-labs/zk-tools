# Request order on a warm 12×2 prover

Each process loads one key and proves three distinct warmup requests followed by 25 measured calls. Each of five requests occurs five times. The mixed order changes request every call. The consecutive order presents five identical requests in a row. Both use the same three warmups within each block. Five blocks rotate CPU placement and shuffle the four processes. Timing encloses the full verified SDK call. Each process keeps its 25 outputs in memory until timing ends. All 500 proofs passed independent verification against the original key and their respective public outputs.

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

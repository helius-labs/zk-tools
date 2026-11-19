# Unequal G2 ranges and paired Euler arithmetic in the same proof

The trial combines the earlier 9/16 first point range in G2 with the paired Euler transform in the integrated SDK. The ordinary arm uses the staged release. Frozen uses the earlier paired-transform executable with the packed FFT settings and its ordinary arithmetic. Four arms of the new executable share the packed settings and independently toggle the G2 split and the paired transform. The second G2 range receives the remaining points. No mode adds a proving-key table.

Each of three witnesses runs at sixteen physical cores and on the same cores with thirty-two hardware threads. Six balanced blocks per placement warm three requests and time twenty-one. One additional sampled process per arm and one dispatch check per input and placement also export proofs. The observed dispatch checks exercise both options. All 258 exports verified independently against the original key and expected public input. Each CPU profile contains samples with no reported loss.

Times are medians of warm process means. The interaction column takes the within-block combined time minus the two individual times plus the disabled time, then reports the median. Under additive latency changes that difference is zero. M5 ratios apply to the supplied original witness.

| Witness and workers | Ordinary / frozen / off / range / FFT / both ms | Both vs off / range / FFT / ordinary (wins) | Interaction ms | Both peak MiB | Both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2-0-w16 | 8.232 / 8.262 / 8.243 / 8.124 / 8.210 / 8.102 | -1.55% (5/6) / -0.56% (3/6) / -1.10% (5/6) / -1.62% (5/6) | +0.060 | 56.8 | 4.57× / 2.96× |
| 1x2-29-w32 | 6.946 / 7.203 / 6.998 / 7.022 / 6.987 / 6.993 | +0.01% (3/6) / -0.67% (4/6) / +0.26% (1/6) / +0.58% (1/6) | -0.015 | 58.8 | — |
| 1x2-32-w16 | 8.075 / 8.069 / 8.046 / 7.972 / 8.056 / 7.950 | -0.99% (6/6) / -0.56% (3/6) / -1.50% (6/6) / -1.59% (5/6) | -0.042 | 55.2 | — |
| 1x2-0-w32 | 7.146 / 7.128 / 7.138 / 7.084 / 7.098 / 7.112 | -1.19% (4/6) / +1.04% (2/6) / -0.15% (3/6) / -0.64% (4/6) | +0.056 | 62.0 | 5.20× / 3.37× |
| 1x2-29-w16 | 8.063 / 8.096 / 8.054 / 8.032 / 8.103 / 8.070 | +0.00% (3/6) / +0.28% (3/6) / +0.07% (3/6) / +0.31% (1/6) | -0.004 | 55.5 | — |
| 1x2-32-w32 | 7.019 / 7.064 / 7.027 / 6.993 / 6.991 / 6.930 | -0.84% (5/6) / -0.89% (5/6) / -0.48% (4/6) / +0.03% (3/6) | -0.002 | 58.5 | — |

On the original at sixteen physical cores, both options gave 8.102 ms against 8.232 ms ordinary, with a 1.62% paired reduction in five of six blocks. The range change alone took 8.124 ms and split the six pairs with the combined option. On witness 29 the combination lost five of six pairs against ordinary at both widths. The 32-worker original took 7.112 ms with both options. These results support retaining the prior runtime settings. All 158 native core tests passed. During an initial attempt, staging the cloud payload changed a pinned file’s hard-link metadata. Its bytes rechecked against the original digest. All timing rows were then collected afresh after staging, with the superseded attempt excluded.

[Range change against the paired source](artifacts/range.patch) · [Paired-transform source](../2026-11-19-paired-circulant680/README.md) · [Build](artifacts/build.json) · [Configurations](artifacts/configs.json.gz) · [Dispatch checks](artifacts/markers.json) · [Timings and profiles](artifacts/rows.json.gz) · [All comparisons](artifacts/summary.json) · [Verification](artifacts/verification.json)

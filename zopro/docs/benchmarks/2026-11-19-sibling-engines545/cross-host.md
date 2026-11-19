# Physical cores and worker count in two Ryzen experiments

The first pair uses sixteen workers throughout. It compares one hardware thread on each of sixteen physical cores with both hardware threads on eight cores. The second pair keeps sixteen physical cores and changes from sixteen workers to thirty-two workers. Both experiments use the same engine binaries and requests. Each pair has its own control and randomized process blocks. The reported changes compare arms within an experiment. The last column gives archived Aeglos latency at thirty-two physical C7a cores on the identical binary, with the same native byte counts at 32 workers.

| Shape | Engine | 16 workers on 16 / 8 cores, ms | Paired change | 16 cores with 16 / 32 workers, ms | Paired change | C7a 32 physical, ms |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | zopro | 8.34 / 10.54 | +26.43% | 8.35 / 7.22 | -13.13% | — |
| 1x2 | aeglos | 13.09 / 15.99 | +22.95% | 12.88 / 12.91 | +0.55% | 12.07 |
| 16x2 | zopro | 43.82 / 52.49 | +19.29% | 43.56 / 39.67 | -8.40% | — |
| 16x2 | aeglos | 231.87 / 260.07 | +11.85% | 232.12 / 240.31 | +3.56% | 102.72 |
| merge-54x1 | zopro | 155.01 / 181.63 | +17.36% | 155.94 / 155.05 | -0.56% | — |
| merge-54x1 | aeglos | 1002.97 / 1056.73 | +5.34% | 1011.66 / 1002.33 | -0.74% | 347.58 |

The C7a column also changes processor, cache hierarchy and memory system. The local placement and worker-count measurements cannot apportion the cross-host difference among these factors.

[New placement comparison](README.md) · [Earlier Ryzen and C7a comparison](../2026-11-19-matched-engines531/README.md) · [Source hashes and process medians](artifacts/topology-comparisons.json)

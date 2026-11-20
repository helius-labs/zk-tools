# Stable parallel K scalar classification on Ryzen 4×2

Parallel classification cut roughly 0.54 ms from the K hash interval. Sixteen shards improved the original full proof by 0.90% against the ordinary path, with the held-out proof near parity. Neither width met the full-proof requirement on both inputs.

The prototype partitions scalar indices by bits derived from the coefficient, then classifies each shard using all four limbs for equality. Each shard visits its original indices in order and records the earliest member of every class. A final input-order scan assigns the same class numbers as the serial implementation. Class values, class membership, counts and point order are preserved. Only K on the existing frontend pool uses this route. [Original scalar-preparation profile](../2026-11-20-curve-preparation-ryzen962/README.md)

The experimental scratch charge is about 3.53 MiB for 97,075 inputs. The charge covers the temporary partition, representatives and hash tables. After classification, these buffers are released and the ordinary group reservation remains through its consumers. Rejection of the extra scratch takes the ordinary classification path.

| Input | Hash layout | Full proof ms | Paired vs ordinary | Paired vs frozen | Paired K hash change, ms | Paired K query change, ms |
|---|---|---:|---:|---:|---:|---:|
| catalog | frozen | 14.551 | +0.14% (4/8) | — | — | — |
| catalog | ordinary | 14.487 | — | −0.14% (4/8) | — | — |
| catalog | 8 shards | 14.542 | +0.37% (2/8) | −0.16% (4/8) | −0.553 | −0.237 |
| catalog | 16 shards | 14.398 | −0.90% (6/8) | −0.82% (6/8) | −0.540 | −0.389 |
| heldout | frozen | 14.500 | +0.32% (2/8) | — | — | — |
| heldout | ordinary | 14.409 | — | −0.32% (6/8) | — | — |
| heldout | 8 shards | 14.527 | +0.48% (1/8) | −0.10% (4/8) | −0.531 | −0.287 |
| heldout | 16 shards | 14.432 | +0.05% (4/8) | −0.06% (4/8) | −0.536 | −0.192 |

Each full-proof arm uses eight blocks of seventeen timed proofs. Four separate clocked blocks per input measure phases while keeping 24 main, eight frontend and four solver workers. The ordinary path calibrated within 0.33% of frozen. Full-proof clock drift stayed within 2.24%.

The native oracle compared exact serial counts, class values and point labels on varied inputs, including skew and zeros. The route probes reproduced the full class data for both real requests and verified the ordinary path at zero experimental scratch. All 96 proof exports independently verified. [C7a scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)

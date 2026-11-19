# SMT width for the mapped 12×2 prover on Ryzen

The 30-worker control on the original request measured **28.335 ms**, or **10.02×** against the supplied M5 CPU reference and **3.56×** against the supplied Metal reference. 5 of eight process means were at or below 28.4 ms. Full SMT at 32 workers had a 28.456 ms median on that request. The 30-worker control used less peak RSS on both transactions.

The same mapped proof image and experimental SDK ran at six widths with all 16 physical cores present. The workers added equal numbers of SMT siblings on the two CCDs. Placement rotated by block. Three blocks on two inputs screened 22, 24, 26, 28, 30 and 32 workers. The strongest alternate to 30 by the mean relative median across the two inputs advanced to eight new paired blocks. Neither input showed a consistent paired gain from the wider pool.

| Input | 30 workers ms | 32 workers ms | Paired change (wins) | Peak MiB at 30 / 32 |
|---|---:|---:|---:|---:|
| catalog | 28.335 | 28.456 | +0.66% (3/8) | 557.3 / 604.0 |
| heldout | 44.994 | 44.673 | +0.40% (4/8) | 1076.8 / 1097.3 |

Each process warmed three calls and measured fifteen single proofs. The Rust benchmark workflow checked all 68 distinct exports against the original key and expected public values. The historical Apple measurements refer to the original request. The additional transaction tests workload sensitivity on Ryzen. The timing statistic is the median of process means.

[Measurements](artifacts/rows.json.gz) · [Exact workflow](artifacts/workflow.json.gz) · [Proof verification](artifacts/verification.json) · [Earlier 30-worker placements](../2026-11-19-rotated-reference652/README.md)

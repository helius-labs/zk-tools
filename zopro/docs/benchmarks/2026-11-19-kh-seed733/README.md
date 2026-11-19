# Running part of K/H before publishing G1 A

The seeded route finishes one, two or four balanced K/H windows before submitting G1 A and the other K/H windows. The outer join schedules B2 alongside the G1 branch. The route uses the original field operations and point data, and places the twenty window points in a fixed array. It needs no new tables or worker pool. Trial dispatch requires the borrowed affine route and at least 32 workers. The two additional arms combine a two-window seed with disabled A grouping, or with both A and exclusive B2 grouping disabled.

Five randomized blocks ran the full SDK on two requests using 32 logical threads on 16 Ryzen cores. Each process warmed three calls and timed fifteen. The control and the seeded arms share a binary. The separate frozen executable comes from the G2-first trial. Gnark verified all 70 exported proofs against the original key. The supplied M5 CPU and Metal measurements refer to the catalog request.

| Input | Configuration | Median ms | Paired vs off | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| catalog | frozen | 7.172 | -0.50% (4/5) | 60.0 | 5.16× / 3.35× |
| catalog | off | 7.188 | +0.00% (0/5) | 61.0 | 5.15× / 3.34× |
| catalog | seed1 | 7.407 | +3.38% (0/5) | 62.2 | 4.99× / 3.24× |
| catalog | seed2 | 7.374 | +2.59% (1/5) | 62.0 | 5.02× / 3.25× |
| catalog | seed4 | 7.402 | +2.98% (1/5) | 61.0 | 5.00× / 3.24× |
| catalog | seed2-skip-a | 7.861 | +8.94% (0/5) | 61.8 | 4.71× / 3.05× |
| catalog | seed2-skip-both | 7.642 | +5.94% (0/5) | 61.5 | 4.84× / 3.14× |
| heldout | frozen | 6.920 | -1.19% (3/5) | 57.5 | — |
| heldout | off | 6.941 | +0.00% (0/5) | 59.5 | — |
| heldout | seed1 | 7.323 | +5.26% (0/5) | 61.8 | — |
| heldout | seed2 | 7.409 | +5.97% (0/5) | 60.8 | — |
| heldout | seed4 | 7.262 | +4.65% (0/5) | 62.2 | — |
| heldout | seed2-skip-a | 7.602 | +9.20% (0/5) | 61.0 | — |
| heldout | seed2-skip-both | 7.524 | +7.41% (0/5) | 61.5 | — |

The region between the G1 and K/H submissions contains only work consumed by that proof. The AWS start-time result motivates this change. Every seeded arm had a higher paired median than the disabled control on both Ryzen inputs. The 96-core C7a scheduling hypothesis requires a separate measurement.

[Implementation](artifacts/source.patch) · [Complete samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [C7a timing intervention](../2026-11-19-solver-order728/README.md)

# Full-proof sensitivity to removing individual curve queries

This experiment measures how much full-request latency changes when specific branches of the parallel proof computation finish immediately. In an isolated benchmark executable, each process loads one public fixture. Its first warmup computes the three original query results. For the selected branches only, subsequent requests reuse the corresponding projective point. Each request checks the fixed witness and key identity. Randomized proof assembly and verification still run on every request. The shipped prover handles arbitrary requests normally.

The machine is a 16-core Ryzen with 32 logical workers and a 16-worker frontend. Five blocks for each of two distinct inputs cover all eight subsets of canonical G1 A, combined G1 K/H and exclusive G2 B2. A separate frozen release measures diagnostic overhead. Each process warmed three requests and timed fifteen. The independent Gnark verifier checked the 90 exported proofs. The table gives medians of process means. Milliseconds saved are within-block differences from the diagnostic control.

| Input | Reused query results | Fixture ms | Paired ms saved | CPU-ms per timed request |
|---|---|---:|---:|---:|
| catalog | Frozen release | 7.246 | -0.026 | 155.6 |
| catalog | Diagnostic control | 7.205 | +0.000 | 155.2 |
| catalog | A | 6.486 | +0.792 | 133.5 |
| catalog | K/H | 5.729 | +1.476 | 103.8 |
| catalog | B2 | 4.786 | +2.475 | 93.6 |
| catalog | A and K/H | 4.991 | +2.219 | 82.4 |
| catalog | A and B2 | 4.390 | +2.824 | 67.6 |
| catalog | K/H and B2 | 3.202 | +3.983 | 39.6 |
| catalog | A, K/H and B2 | 1.895 | +5.310 | 14.3 |
| heldout | Frozen release | 7.036 | +0.047 | 151.7 |
| heldout | Diagnostic control | 7.060 | +0.000 | 152.5 |
| heldout | A | 6.337 | +0.717 | 131.2 |
| heldout | K/H | 5.398 | +1.573 | 101.1 |
| heldout | B2 | 4.583 | +2.371 | 89.9 |
| heldout | A and K/H | 4.929 | +2.081 | 82.9 |
| heldout | A and B2 | 4.331 | +2.705 | 66.2 |
| heldout | K/H and B2 | 2.904 | +4.133 | 37.1 |
| heldout | A, K/H and B2 | 1.894 | +5.128 | 14.2 |

These are counterfactual timings on repeated identical benchmark fixtures. Each cell includes changes in contention and worker availability for the remaining branches. A deployable arithmetic optimization would need to compute the selected result for each new witness. The all-three cell measures the residual in this experiment on this machine. These simulations do not change the release leaderboard.

[Per-process samples](artifacts/rows.json.gz) · [Benchmark-only source change](artifacts/diagnostic.patch) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Measured AWS timeline](../2026-11-19-tail-c7a726/README.md)

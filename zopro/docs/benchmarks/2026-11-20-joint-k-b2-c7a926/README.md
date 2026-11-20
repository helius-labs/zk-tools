# Joint K and B2 replay on C7a 4×2 proofs

Joint K and B2 replay took **11.781 / 11.822 ms** across the two 4×2 inputs. The extra saving over the sum of individual removals was **+2.120 / +1.907 ms**. The 10× M5 CPU threshold for this shape is 10.9 ms. The all-active original input is **7.38× M5 CPU** and **3.25× Metal**.

This is the [Ryzen interaction diagnostic](../2026-11-20-four-by-two-query-factorial-ryzen916/README.md) on C7a with 64 main and 32 frontend physical cores. Replay compares the entire coefficient list and the original bases before using the existing query result. A and H still run in the joint K/B2 test. Replay results are diagnostic.

Each input uses twelve counterbalanced timing blocks. All 182 exports verified on the runner and again after retrieval. The shared c7a.24xlarge terminated after the last panel, its bucket was deleted, and the final check found zero tagged volumes or active benchmark instances.

| Input | Mode | Proof ms | Paired vs frozen | Peak MiB |
|---|---|---:|---:|---:|
| catalog | frozen | 14.864 | +0.00% (0/12) | 199.1 |
| catalog | ordinary | 14.874 | -0.63% (8/12) | 202.7 |
| catalog | all active | 14.777 | -0.57% (7/12) | 200.4 |
| catalog | K replay | 14.861 | -0.54% (7/12) | 200.6 |
| catalog | B2 replay | 13.750 | -7.48% (12/12) | 139.8 |
| catalog | K and B2 replay | 11.781 | -20.51% (12/12) | 147.3 |
| catalog | all replay | 7.537 | -49.31% (12/12) | 141.9 |
| heldout | frozen | 14.959 | +0.00% (0/12) | 204.2 |
| heldout | ordinary | 14.838 | -0.11% (6/12) | 210.2 |
| heldout | all active | 14.941 | -0.09% (7/12) | 206.4 |
| heldout | K replay | 14.628 | -1.70% (12/12) | 207.1 |
| heldout | B2 replay | 13.898 | -7.08% (12/12) | 144.9 |
| heldout | K and B2 replay | 11.822 | -20.66% (12/12) | 141.2 |
| heldout | all replay | 7.557 | -49.45% (12/12) | 141.2 |

Both observer calibrations pass on both inputs.

[Timing and proof verification](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Cleanup receipt](artifacts/cleanup.json)

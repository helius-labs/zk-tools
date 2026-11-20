# Splitting long scalar classes in 4×2 curve queries

Splitting long scalar classes on both curves changed paired proof latency by **−0.39% on the original input and +0.48% on the held-out input** against the same-build control. Full proofs took **13.615 / 13.649 ms**. The joint mode and both single-curve modes fell short of the timing criteria.

When at least 1,024 points have the same scalar, the trial reduces independent 512-point chunks in the existing worker pool and combines their roots into the original class sum. Other classes use the existing reduction. Optional scratch is charged to the existing memory policy. The trial adds no retained table or workers.

Each monitored query has one long class on each input. The query assignments match the term and class counts from the independent C7a census. The largest G1 class used five chunks. The largest B2 class used four. The corresponding optional workspace forecasts were about 0.62 and 0.77 MiB. With zero optional workspace, the proof takes the ordinary fallback and none of the observed queries admits the new route. [C7a query census](../2026-11-20-four-by-two-curve-work-c7a896/README.md)

| Input | Query | Grouped terms | Classes | Largest class | Long classes | Chunks |
|---|---|---:|---:|---:|---:|---:|
| catalog | A | 58,424 | 19,851 | 1,972 | 1 | 4 |
| catalog | K | 86,732 | 29,828 | 1,976 | 1 | 4 |
| catalog | B2 | 86,163 | 29,738 | 1,576 | 1 | 4 |
| heldout | A | 56,690 | 20,014 | 2,107 | 1 | 5 |
| heldout | K | 83,995 | 30,052 | 2,108 | 1 | 5 |
| heldout | B2 | 83,409 | 29,961 | 1,708 | 1 | 4 |

The Ryzen run uses 24 main and eight frontend hardware threads. Eight counterbalanced blocks per input warm three proofs and time seventeen. The setup requires the fresh ordinary controls to calibrate within 2% of the frozen reference. Candidate modes require lower paired medians and six wins out of eight against both controls on both inputs. The control differences were +0.54% and +0.85%. RSS is whole-process peak and includes key loading.

| Input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | frozen | 13.572 | +0.00% (0/8) | -0.53% (5/8) | 215.8 |
| catalog | ordinary | 13.643 | +0.54% (3/8) | +0.00% (0/8) | 207.5 |
| catalog | g1 | 13.619 | +0.01% (4/8) | +0.05% (4/8) | 211.8 |
| catalog | g2 | 13.633 | +0.25% (4/8) | -0.14% (4/8) | 207.2 |
| catalog | both | 13.615 | +0.15% (3/8) | -0.39% (5/8) | 209.0 |
| heldout | frozen | 13.504 | +0.00% (0/8) | -0.84% (6/8) | 221.5 |
| heldout | ordinary | 13.614 | +0.85% (2/8) | +0.00% (0/8) | 216.2 |
| heldout | g1 | 13.616 | +0.35% (4/8) | -0.14% (4/8) | 217.2 |
| heldout | g2 | 13.608 | +0.88% (2/8) | -0.03% (5/8) | 223.5 |
| heldout | both | 13.649 | +0.47% (2/8) | +0.48% (4/8) | 224.8 |

In three separate clocked blocks, G1-only splitting shortened the measured A span by about 0.38–0.43 ms. K and B2 still took 7.95–8.59 ms in those runs. Query spans include waiting and nested worker execution. The operation redistributes group additions without eliminating them. [Earlier K and B2 replay diagnostic](../2026-11-20-four-by-two-query-factorial-ryzen916/README.md)

The native test compared G1 and G2 with complete group arithmetic on one and four workers, including cancellation, doubling, identity, empty groups and uneven chunk boundaries. All 114 SDK proof exports independently verified. This is a local experiment. The C7a scorecard remains five of eight at 10× M5 CPU and includes all eight Metal comparisons. [CPU and Metal scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Proof samples and query clocks](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Trial source](artifacts/source.patch) · [Verification](artifacts/verification.json)

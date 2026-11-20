# Repeated Poseidon work in the 4×2 solver schedule

The 64 tuples of four matching Poseidon spans fall into seven disjoint groups of four independent solver tasks. Within each group, all four tasks encounter the matching spans in the same order. Each span follows a prefix of ordinary solver work. The spans in a tuple share an epoch, so the existing schedule gives none of them a guaranteed earlier result to reuse.

The observer locates the spans behind the [14,394 certified K and B2 quartets](../2026-11-20-quartet-origin-census-ryzen947/README.md) before the solver applies wire offsets. It checks the actual solved coefficients and their K projection, then records the epoch, task and span offsets. Two independent processes per input reproduced identical coordinates on the original and held-out requests. Gnark independently verified all four exported proofs.

| Solver epoch | Independent groups of four tasks | Matching span tuples |
|---|---:|---:|
| 0 | 1 | 5 |
| 1 | 2 | 20 |
| 2 | 2 | 21 |
| 3 | 2 | 18 |
| Total | 7 | 64 |

A grouped executor could run each task's prefix before comparing all three first-round state elements. When all four states match, it could compute the remaining rounds once and emit the results to each task's own wire and constraint rows. On the two observed inputs this would remove 14,400 S-box evaluations, while preserving all original writes. That count excludes the impact of task placement, shared execution and memory traffic. The observer measures no latency and adds no speedup to the [C7a scorecard](../2026-11-20-eager-h-canonical-c7a954/README.md#eight-supplied-m5-references).

[Counts and proof binding](artifacts/counts.json) · [Schedule coordinates](artifacts/sites.json.gz) · [Protocol](artifacts/protocol.json) · [Experimental observer](artifacts/source.patch.gz)

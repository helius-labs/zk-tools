# Witness-solver continuations across dependency bands

Each row describes the existing schedule built from the key image and used for a verified SDK proof. For each pair of adjacent bands, the diagnostic identifies following tasks with exactly one producer task in the previous band. It models immediately continuing those tasks on the producer, provided their serial spans fit in its spare time before the previous band would finish. Tasks with more than one immediate producer, or with no immediate producer, keep their original position. The schedule actually executed is unchanged. Operations can read intermediate values they write during the same operation. The census counts these reads separately and uses only distinct producer and consumer operations to build inter-task dependencies.

Span uses one unit per operation, includes existing inner parallelism, and ignores worker limits, scheduling cost and contention. The two gain columns sum savings for disjoint pairs starting with band 0 or band 1, divided by the full original span. Consumers and moves are counts across all boundaries, including adjacent boundaries. An unavailable row means a physical wire had multiple writers and the diagnostic declined to infer a unique producer.

| Shape | Bands | Span | Following tasks | Single immediate parent | No immediate parent | Moved tasks | Moved ops | Even-pair span saving | Odd-pair span saving |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 4 | 11500 | 7 | 7 | 0 | 1 | 236 | 0.00% | 0.00% |
| 1x2 | 6 | 11439 | 10 | 9 | 0 | 0 | 0 | 0.00% | 0.00% |
| 1x2 | 8 | 11668 | 15 | 14 | 0 | 1 | 1234 | 0.00% | 0.00% |
| 2x2 | 4 | 12931 | 13 | 13 | 0 | 3 | 5174 | 0.00% | 0.00% |
| 2x2 | 6 | 11920 | 19 | 18 | 0 | 2 | 3322 | 0.00% | 0.00% |
| 2x2 | 8 | 12144 | 28 | 27 | 0 | 2 | 2468 | 0.00% | 0.00% |
| 4x2 | 4 | 14417 | 25 | 25 | 0 | 5 | 10112 | 0.00% | 0.00% |
| 4x2 | 6 | 11985 | 37 | 36 | 0 | 4 | 6644 | 0.00% | 0.00% |
| 4x2 | 8 | 12145 | 55 | 53 | 0 | 4 | 4936 | 0.00% | 0.00% |
| 8x2 | 4 | 17270 | 49 | 49 | 0 | 9 | 19988 | 0.00% | 0.00% |
| 8x2 | 6 | 12352 | 74 | 72 | 0 | 8 | 13288 | 0.00% | 0.00% |
| 8x2 | 8 | 13196 | 107 | 105 | 0 | 8 | 9872 | 0.00% | 0.00% |
| 12x2 | 4 | 20098 | 74 | 72 | 0 | 12 | 29628 | 0.00% | 0.00% |
| 12x2 | 6 | 13769 | 112 | 110 | 0 | 13 | 20050 | 0.00% | 0.00% |
| 12x2 | 8 | 12727 | 162 | 158 | 0 | 12 | 14808 | 0.00% | 0.00% |
| 16x2 | 4 | 23559 | 98 | 96 | 0 | 16 | 39504 | 0.00% | 0.00% |
| 16x2 | 6 | 13096 | 150 | 146 | 0 | 16 | 26576 | 0.00% | 0.00% |
| 16x2 | 8 | 13099 | 218 | 215 | 0 | 17 | 19808 | 0.00% | 0.00% |
| 24x2 | 4 | 28594 | 150 | 146 | 0 | 24 | 59256 | 0.00% | 0.00% |
| 24x2 | 6 | 14230 | 226 | 222 | 0 | 24 | 39864 | 0.00% | 0.00% |
| 24x2 | 8 | 14661 | 328 | 324 | 0 | 25 | 29730 | 0.00% | 0.00% |
| merge-24x1 | 4 | 45932 | 147 | 146 | 0 | 24 | 59256 | 0.00% | 0.00% |
| merge-24x1 | 6 | 41891 | 245 | 244 | 0 | 24 | 39864 | 0.00% | 0.00% |
| merge-24x1 | 8 | 34834 | 321 | 318 | 0 | 74 | 90494 | 0.00% | 3.55% |

The greedy moves left the longest modeled task unchanged at every boundary for all seven transfer shapes. Only merge-24×1 with eight bands had a positive saving, at 3.55% of solver span for the odd pairing. That shape already meets the CPU target on C7a. This rules out this particular continuation policy as a promising route for the six remaining shapes. Other policies can choose different tasks or allow work to extend past the previous boundary.

The first diagnostic asserted that every dependency has distinct producer and consumer operation indices. Solver steps can read intermediate slots that they have written themselves. The corrected diagnostic counts these internal reads separately, checks every remaining dependency, and uses the original executable schedule for verification.

The motivating question was whether an early-finishing solver task can start its successor before a global synchronization point. Work on sparse triangular solves studies how arrival and waiting differ across workers. The producer census above is specific to the Zopro solver. [Elasticity in Parallel Sparse Triangular Solve](https://arxiv.org/html/2607.02324v1).

[Per-boundary counts and proof hashes](artifacts/census.json.gz) · [Diagnostic source](artifacts/source.patch.gz)

# Bounded digit grouping across nine G2 witnesses

Five different 1×2 witnesses select a partition of the eleven signed Booth windows for each endomorphism component. Four further witnesses vary the coin, amounts and Merkle trees. Only the window boundaries are fixed from training. The model finds equal signed patterns afresh on each input, within each tile, and includes the additions required to construct the signed point sums. Two contiguous halves are processed separately. The denominator is the same full-query nominal within-component count used in the original census.

| Maximum rows per tile | Patterns per window block and tile | Training median saved | Holdout median saved | Holdout range |
|---:|---|---:|---:|---:|
| 128 | all | 0.205% | 0.212% | 0.202%–0.219% |
| 128 | top16 | 0.205% | 0.212% | 0.202%–0.219% |
| 1,024 | all | 0.632% | 0.639% | 0.629%–0.646% |
| 1,024 | top16 | 0.323% | 0.330% | 0.319%–0.337% |
| 4,096 | all | 1.220% | 1.220% | 1.216%–1.226% |
| 4,096 | top16 | 0.379% | 0.383% | 0.380%–0.386% |
| 16,384 | all | 3.010% | 3.010% | 3.006%–3.017% |
| 16,384 | top16 | 0.409% | 0.409% | 0.409%–0.409% |

With at most 1,024 rows in a tile, even the unrestricted set of patterns removed a median 0.639% of nominal additions on the four held-out requests. Selecting the sixteen best patterns in a tile reduced that to 0.330%.

On the original input the unrestricted model reproduced 35,442 of 572,819 nominal additions saved. Top16 grants perfect knowledge of the sixteen most useful patterns in each tile and block. Searching for them, moving points, and scheduling work cost time in an implementation. These are operation counts and supply no measured latency or RAM figure. No data-dependent plans are kept by the prover. Related work studies operation counts from sharing digit patterns across windows. [Hrabar, University of Waterloo](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403)

[Counts](artifacts/counts.json) · [Fixed window partitions](artifacts/partitions.json) · [Rust reader](artifacts/count.rs) · [Earlier unrestricted model](../2026-11-19-g2-digit-blocks414/README.md)

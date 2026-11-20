# Poseidon result availability under the ordinary 4×2 schedule

With independent tasks, a designated source had finished its tail by follower readiness for a median of 55 of 192 possible reuses with four solver workers and 36 with seven. A two-microsecond wait did not raise those medians. Across all 256 spans, the median wait for another tail to finish was 10–11 µs.

The probe instruments the [64 tuples of matching spans](../2026-11-20-poseidon-frontier-ryzen955/README.md) while keeping the existing 28 tasks and their epoch barriers. It timestamps first-round state readiness, the 25th and 50th of 75 subsequent S-box writes, and tail completion. The designated source is the first member in each original task group. Each row summarizes 85 timed traces over five processes on Ryzen. The 24 main and eight frontend workers use the same placement in all arms.

| Solver workers | Input | Source tail already ready | Source ready within 2 µs | Earliest of three other tails ready | Median individual tail µs |
|---:|---|---:|---:|---:|---:|
| 4 | catalog | 55 | 55 | 55 | 11.70 |
| 4 | heldout | 55 | 55 | 55 | 11.74 |
| 7 | catalog | 36 | 36 | 36 | 11.83 |
| 7 | heldout | 36 | 36 | 36 | 11.79 |

The median number of followers whose designated source had already written the first 25 S-boxes was 63–67 with four workers and 40–41 with seven. In a replay model holding the observed task starts fixed, the sum of improvements in selected task completion at the epoch barriers was 182–183 µs with four workers and 300–303 µs with seven. The model assumes zero-cost reuse of already completed tails, advances each follower after a reuse, and excludes unobserved tasks. These are modeled times. Actual replay pays the full cost of copying values and writing the original destinations.

| Solver workers | Input | Observer proof drift vs ordinary | Observer solve drift vs ordinary | Ordinary proof drift vs frozen |
|---:|---|---:|---:|---:|
| 4 | catalog | +0.27% | +3.10% | +2.13% |
| 4 | heldout | −0.81% | +2.20% | +1.76% |
| 7 | catalog | −0.10% | +1.49% | +2.62% |
| 7 | heldout | −0.44% | −0.56% | +3.10% |

Changes are paired medians of process means. The ordinary build fell outside a 2% full-proof calibration band in three of four comparisons. The timing results support a local chronology estimate, with the observer drift shown above. The probe executes ordinary arithmetic throughout. Five native tests passed and Gnark independently verified all 60 exported proofs. The [C7a scorecard](../2026-11-20-eager-h-canonical-c7a954/README.md#eight-supplied-m5-references) is unchanged.

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Per-proof summaries](artifacts/rows.json.gz) · [Capture examples](artifacts/raw-examples.json.gz) · [Experimental source](artifacts/source.patch.gz)

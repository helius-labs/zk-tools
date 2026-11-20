# Reusing completed Poseidon tails in the 4×2 solver on Ryzen

The seven-worker replay path passed the matched full-proof comparison on both inputs. It saved 1.20–1.46% against the same build with replay disabled. The four-worker path missed the frozen control on the original input.

The experiment uses the [seven sets of matching spans](../2026-11-20-poseidon-timeline-ryzen957/README.md) under the original independent tasks and epoch barriers. Each designated source publishes its latest completed span ordinal. A follower runs its own first round, checks publication once and compares all three state words. A match replays the completed tail from the source's existing solution cells into the follower's original destinations. A late source or different state runs the ordinary arithmetic. The optional schedule accounts for 3,904 bytes under a 16 KiB admission charge. Each proof has seven cache-line-separated counters totaling 448 bytes.

All arms use 24 main and eight frontend workers on Ryzen. Observe performs the same completion and equality checks and then runs the ordinary tail. Each arm has eight counterbalanced process blocks per input with three warm-up and seventeen timed proofs. The changes are paired medians of the process means.

| Solver workers | Input | Frozen ms | Ordinary ms | Observe ms | Replay ms | Replay vs ordinary | Replay vs frozen |
|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 4 | catalog | 14.434 | 14.562 | 14.571 | 14.529 | −0.79% (5/8) | +0.52% (3/8) |
| 4 | heldout | 14.484 | 14.573 | 14.592 | 14.430 | −0.77% (7/8) | −0.11% (6/8) |
| 7 | catalog | 13.891 | 13.897 | 13.946 | 13.742 | −1.20% (7/8) | −0.80% (6/8) |
| 7 | heldout | 13.837 | 13.899 | 13.887 | 13.767 | −1.46% (8/8) | −0.83% (7/8) |

The seven-worker phase probes replayed a median of 36 tails per proof on both inputs and reduced the solve stage from 1.813/1.870 to 1.634/1.629 ms. Whole-process peak RSS for replay was 231/211 MiB. All four ordinary proof comparisons calibrated within 2% of frozen.

Five native tests passed. Ten route probes compared all 129,130 wire and retained-slot entries and 98,674 entries in each materialized A, B and C array. They covered late sources, equal-state replay, deliberate mismatches, and both materialized and omitted C. Gnark independently verified all 170 exported proofs. These results are local Ryzen measurements. [C7a scorecard](../2026-11-20-eager-h-canonical-c7a954/README.md#eight-supplied-m5-references)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)

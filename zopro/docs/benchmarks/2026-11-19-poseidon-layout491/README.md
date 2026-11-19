# Physical layout of repeated Poseidon spans

This diagnostic requires two recognized spans to have the same full post-first-round field state and identical output locations up to three independent translations for stored slots, constraint rows and output wires. It compares the normalized address vectors exactly, including operand orientation, in S-box execution order. The three run counts come from sorted unique positions in each vector. A single run in all three vectors would allow three contiguous address ranges to cover the complete remaining output of a span; A, B and optional C each use the row range.

| Shape | Input | Repeated states | Translation-compatible repeats | One run per address kind | At most 12 total address runs |
|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 1 | 0 | 0 | 0 |
| 2x2 | Original | 68 | 0 | 0 | 0 |
| 4x2 | Original | 175 | 0 | 0 | 0 |
| 8x2 | Original | 407 | 0 | 0 | 0 |
| 12x2 | Original | 644 | 0 | 0 | 0 |
| 16x2 | Original | 922 | 0 | 0 | 0 |
| 24x2 | Original | 1347 | 0 | 0 | 0 |
| merge-24x1 | Original | 1333 | 0 | 0 | 0 |
| 12x2 | 137 | 379 | 0 | 0 | 0 |
| 12x2 | 138 | 381 | 0 | 0 | 0 |
| 12x2 | 139 | 380 | 0 | 0 | 0 |
| 12x2 | 140 | 383 | 0 | 0 | 0 |
| 12x2 | 141 | 379 | 0 | 0 | 0 |
| 12x2 | 142 | 381 | 0 | 0 | 0 |
| 12x2 | 143 | 379 | 0 | 0 | 0 |
| 12x2 | 144 | 378 | 0 | 0 | 0 |
| merge-24x1 | 129 | 1237 | 0 | 0 | 0 |
| merge-24x1 | 130 | 1177 | 0 | 0 | 0 |
| merge-24x1 | 131 | 1175 | 0 | 0 | 0 |
| merge-24x1 | 132 | 1225 | 0 | 0 | 0 |
| merge-24x1 | 133 | 1177 | 0 | 0 | 0 |
| merge-24x1 | 134 | 1159 | 0 | 0 | 0 |
| merge-24x1 | 135 | 1212 | 0 | 0 | 0 |
| merge-24x1 | 136 | 1185 | 0 | 0 | 0 |

The counts exclude the first ordinary operations in each span and describe the 24 public requests from the preceding reuse study. Repeated states differ from the number available to an asynchronous consumer. This diagnostic does not copy any solved rows. All 24 exported proofs verified independently. [Results by width and run count](artifacts/counts.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

# Poseidon recognition after changing solver bands

The sparse Poseidon implementation recognizes complete spans inside a solver leaf. Moving a band boundary can cut such a span, which changes the amount of arithmetic handled by the general interpreter. Each cell counts original solver operations covered by recognized spans for the exact packing candidate. Counts are collected at key preparation. Four schedules on each of the eight images exported independently checked proofs. The diagnostic uses the same binary as experiment 377.

| Shape | Total ops | Release bands | Blended outer bands | Blended outer packed at 128 | Count-balanced packed at 128 |
|---|---:|---:|---:|---:|---:|
| 1x2 | 31061 | 17559 | 19866 | 19866 | 18231 |
| 2x2 | 54083 | 32637 | 35628 | 35628 | 32355 |
| 4x2 | 99035 | 59127 | 60618 | 60618 | 62568 |
| 8x2 | 190331 | 113649 | 120441 | 120441 | 124716 |
| 12x2 | 281371 | 172119 | 179511 | 179511 | 187194 |
| 16x2 | 372731 | 239277 | 248340 | 248340 | 246303 |
| 24x2 | 556987 | 344568 | 357240 | 357240 | 349281 |
| merge-24x1 | 526435 | 333852 | 339360 | 339360 | 333552 |

On 1×2, changing the outer band boundaries increased specialized coverage from 17,559 to 19,866 operations. Packing within those same bands left every per-width span count unchanged on all eight shapes. The additional 1×2 packing gain in the matched timing comparison therefore cannot be explained by recognizing more hashes. Conversely, the outer-band comparison changed both scheduling and arithmetic coverage. This makes keeping whole candidate spans together during band formation worth testing.

A recognized width-t span covers three times the sum of seven times t and its partial-round count. The per-width counts and original schedule metrics are included for each run. This diagnostic establishes which operations changed execution path. It does not assign a latency cost to an individual operation.

[Per-width spans and hashes](artifacts/counts.json) · [Matched latency study](../2026-11-18-packed-waves377/README.md)

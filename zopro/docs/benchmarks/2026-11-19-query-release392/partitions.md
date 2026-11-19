# Contiguous B-query ranges available at each solver barrier

A prospective worker can read up to four disjoint slices of the original point array. For each barrier the reader orders maximal ready runs by length, takes at most four, and stops when they fill one nominal input partition. This maximizes the number of ready entries under that four-slice limit. Existing partitions use consecutive equal-sized chunks with a possible shorter final chunk. The artifact records all half-open ranges. The calculation uses key topology alone. Equal-scalar grouping can change the effective query order on a given proof.

| Shape | Barrier | All ready runs | Original halves ready | Ready rows in up to four runs / half | Original quarters ready | Ready rows in up to four runs / quarter |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 0 | 1 | 0 | 142 / 15,045 | 0 | 142 / 7,523 |
| 1x2 | 1 | 3 | 0 | 15,045 / 15,045 | 0 | 7,523 / 7,523 |
| 1x2 | 2 | 3 | 0 | 15,045 / 15,045 | 1 | 7,523 / 7,523 |
| 1x2 | 3 | 3 | 0 | 15,045 / 15,045 | 2 | 7,523 / 7,523 |
| 2x2 | 0 | 1 | 0 | 231 / 26,137 | 0 | 231 / 13,069 |
| 2x2 | 1 | 5 | 0 | 21,965 / 26,137 | 0 | 13,069 / 13,069 |
| 2x2 | 2 | 5 | 0 | 26,137 / 26,137 | 0 | 13,069 / 13,069 |
| 2x2 | 3 | 5 | 0 | 26,137 / 26,137 | 0 | 13,069 / 13,069 |
| 16x2 | 0 | 1 | 0 | 1,477 / 179,172 | 0 | 1,477 / 89,586 |
| 16x2 | 1 | 89 | 0 | 20,459 / 179,172 | 0 | 20,459 / 89,586 |
| 16x2 | 2 | 52 | 0 | 26,186 / 179,172 | 0 | 26,186 / 89,586 |
| 16x2 | 3 | 33 | 0 | 46,529 / 179,172 | 0 | 46,529 / 89,586 |
| 16x2 | 4 | 33 | 0 | 51,515 / 179,172 | 0 | 51,515 / 89,586 |
| 16x2 | 5 | 17 | 0 | 87,730 / 179,172 | 0 | 87,730 / 89,586 |

At the first 1×2 barrier, none of the current two or four contiguous partitions is complete. Three ready slices can fill one half-size worker assignment. A single ready slice can fill one quarter-size assignment. Starting it there could overlap at most 0.347 ms of the observed remaining solver work. Altering the point order or introducing partial reductions would have its own cost. The simplest full-query split can repeat bucket reductions, which motivates preserving their existing count when testing these ranges. [EdMSM, section 5.1](https://eprint.iacr.org/2022/1400.pdf)

[Exact ranges and map hashes](artifacts/partitions.json) · [Rust reader](artifacts/partitions.rs)
